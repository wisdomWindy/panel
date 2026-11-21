# 实施计划与灰度方案

依赖：`spec/spec.md`（已冻结）

## 1. 变更清单

### 新增

| 文件 | 作用 |
| --- | --- |
| `services/qwen3-tts/server.py` | Qwen3-TTS HTTP 语音服务（标准库 HTTP 层 + 惰性加载模型 + `--mock` 引擎） |
| `services/qwen3-tts/tests/test_server.py` | 服务端契约测试（10 项） |
| `services/qwen3-tts/bench.py` | 延迟基线测量（传输层/真实服务通用） |
| `services/qwen3-tts/requirements.txt` | 模型运行时依赖 |
| `services/qwen3-tts/README.md` | 部署与运维说明 |
| `src-tauri/src/infrastructure/media/qwen_tts.rs` | `Qwen3TtsService` + `Qwen3TtsSpeechEngine`（9 项单测） |
| `src-tauri/src/infrastructure/member/qwen_ai_tts.rs` | `Qwen3TtsAiSpeechOutput`（SAPI SpeakStream + 回退，5 项单测） |

### 修改

| 文件 | 变更 |
| --- | --- |
| `domain/media/models.rs` | 新增 `Qwen3TtsConfig` 与其默认值、语言白名单、`event_voice/reply_voice/reply_instruct` |
| `domain/media/rules.rs` | 新增 `validate_qwen3_tts_config` |
| `application/media/ports.rs` | 新增 `SpeechSynthesisPort`、`SynthesisRequest/Voice/Health`、`MediaIoError::coded` |
| `application/media/mod.rs` | 导出上述类型（`pub use ports::*` 已有，无需改动） |
| `infrastructure/media/mod.rs` | 注册并导出 `qwen_tts` |
| `infrastructure/member/mod.rs` | 注册并导出 `qwen_ai_tts` |
| `lib.rs` | 启动时构建 `Qwen3TtsService`；播报 Cloud 槽位接入；AI 回复包裹系统输出；`AppState` 暴露服务 |
| `ipc/dto.rs` | `Qwen3TtsConfigDto/HealthDto/SnapshotDto/RequestDto`；命令名常量 |
| `ipc/media.rs` | `tts_qwen3_get/save/test` 三个命令 + 有界探测快照 |
| `ipc/error.rs` | 8 个 `qwen3_tts_*` 中文提示 |
| `ipc/media_contract_test.rs` | 契约命令数 35→38、新增配置往返测试 |
| `permissions/foundation.toml` | 三个新命令的 main-only 权限 |
| `renderer/platform/desktop/contracts/media.ts` | 新类型与命令映射 |
| `renderer/modules/media/adapters.ts` | `adaptQwen3TtsSnapshot`（严格字段校验） |
| `renderer/stores/media/tts.ts` | `qwen3` 快照 + `loadQwen3/saveQwen3/testQwen3` |
| `renderer/views/tts/index.vue` | Qwen3-TTS 设置区（开关/地址/语言/音色/指令/超时/回退/探测/试听/保存） |
| `locales/zh-CN.json`、`locales/en-US.json` | `tts.qwen3.*` 18 个键（双语一致） |

## 2. 实施顺序

1. 服务端契约 + `--mock` 引擎 + 契约测试（可独立验证，不依赖 GPU）。
2. Rust 抽象与适配器 + 单测（用进程内 HTTP stub，不依赖真实服务）。
3. 两条链路接线（`lib.rs`）+ `cargo check`/`cargo test`。
4. IPC 与前端设置界面。
5. 延迟基线与参数基线。
6. 灰度与回滚演练。

## 3. 灰度方案

### 开关层级（三级，逐级收窄）

| 级别 | 开关 | 作用范围 | 回滚动作 |
| --- | --- | --- | --- |
| L1 全局 | `qwen3.isEnabled` | 关闭后无任何 Qwen 合成请求 | 设置页关闭开关 |
| L2 链路 | 播报 `selectedEngine` | `system` 走系统 TTS，`cloud` 走 Qwen3-TTS | 引擎切换为 `system` |
| L3 单条回复 | AI 回复 `voice_id` | 非 `qwen3:` 前缀即走系统 SAPI | 选择系统音色 |

### 放量步骤

1. **内测（1 台）**：`isEnabled=true`，播报仍 `system`，仅用“试听”验证合成与播放。
2. **小流量**：播报切 `cloud`，只勾选“舰长 + 礼物”事件，观察 1 小时 `tts_status_changed` 与日志。
3. **全量播报**：放开全部事件类型；AI 回复音色选 `qwen3:Serena`。
4. **稳定观察 24h** 后视为达成。

### 熔断条件（任一命中即回滚）

- 合成失败率 > 5%（连续 20 条中 ≥2 条回退）
- 单条播报从入队到出声 > 6s（超过直播对话节奏）
- 应用日志出现 `qwen3_tts_invalid_response`（可能音频损坏）
- AI 回复音频进入非预期设备

### 一键回滚耗时

设置页关闭 `isEnabled` → 下一次合成立即回退系统引擎，**无需重启**。
配置写入为单条 SQLite upsert，实测路径 < 1s；回滚演练见 `verification/verification.md` 第 5 节。
