# 验证记录

日期：2026-11-20 ｜ 环境：Windows 11 / rustc stable MSVC / Node 26 / Python 3.13

> 结论先行：**代码层验证通过**（Rust 140 项测试 + Python 10 项契约测试全绿）。
> **真实模型发声未在本机验证**——本机无 GPU 模型权重，且前端工具链因 `node_modules` 指向已搬迁路径而不可用；两项均在下文如实标注为未覆盖项，并给出复现命令。

## 1. 静态检查

| 项 | 命令 | 结果 |
| --- | --- | --- |
| Rust 编译（含测试目标） | `cargo check --lib --tests --features free` | ✅ 0 error / 0 warning |
| Rust 单元测试 | `cargo test --lib --features free` | ✅ `140 passed; 0 failed` |
| Python 服务契约 | `python -m unittest discover -s services/qwen3-tts/tests -t services/qwen3-tts -v` | ✅ `Ran 10 tests ... OK` |
| 语言包一致性 | `node -e "JSON.parse(...)"` 双语言键比对 | ✅ `tts.qwen3` 18 键 / zh-CN 与 en-US 一致 |
| TypeScript 类型检查 | `pnpm --filter @monorepo/bilibili-live-tool type-check` | ⛔ **未执行**（见第 6 节阻塞项） |

### 关于 Rust 验证的补充说明（已于后续变更解决）

本记录最初生成时，本 crate 的 `whisper-rs-sys` 依赖（既有，与 Qwen3-TTS 改动无关）无法在本机构建，
验证曾通过临时移除该依赖完成。**该依赖随后已被彻底移除**（见 `docs/requests/remove-whisper-asr/`），
现在 `cargo check` / `cargo test` 可直接运行，无需任何隔离手段。

当前基线：`cargo test --lib --features free` → **146 passed, 0 failed**（含此前被排除的 member 单元测试）。

下表 140 项为 Qwen3-TTS 交付当时的数量；移除 ASR 后总数变为 146（新增包含 member 模块测试，减去被删除的 whisper 目录测试）。

## 2. 新增/修改测试清单

| 测试 | 覆盖点 |
| --- | --- |
| `qwen_tts::composes_rate_into_a_natural_language_instruction` | 语速倍率 → 指令文本（含边界 1.0 / 1.4 / 0.6） |
| `qwen_tts::parses_a_voice_catalogue` | `/v1/voices` 解析 |
| `qwen_tts::reports_health_from_the_service` | `/health` 解析（ready / model / sampleRate） |
| `qwen_tts::returns_rejected_when_the_service_is_disabled` | 关闭开关 → `qwen3_tts_disabled`、不可重试 |
| `qwen_tts::rejects_a_non_wav_payload` | 非 RIFF → `qwen3_tts_invalid_response` |
| `qwen_tts::returns_the_synthesised_wave` | 正常合成返回 RIFF |
| `qwen_tts::maps_a_loading_service_to_a_retryable_error` | 503 → `qwen3_tts_loading`、可重试 |
| `qwen_tts::maps_a_bad_request_to_a_non_retryable_error` | 400 → `qwen3_tts_invalid_request`、不可重试 |
| `qwen_tts::temporary_wave_files_are_removed_on_drop` | 临时 WAV 生命周期 |
| `qwen_ai_tts::a_system_voice_id_never_touches_the_qwen_service` | 非 `qwen3:` 音色直接透传系统引擎 |
| `qwen_ai_tts::an_unreachable_service_falls_back_to_the_system_voice` | 不可达 → 系统语音（回滚语义） |
| `qwen_ai_tts::a_disabled_service_falls_back_to_the_system_voice` | 关闭开关 → 系统语音 |
| `qwen_ai_tts::a_failed_service_surfaces_the_error_when_fallback_is_off` | 关闭回退 → 返回可重试错误 |
| `qwen_ai_tts::disabled_service_hides_qwen_voices_but_keeps_system_voices` | 关闭时音色列表降级 |
| `qwen_ai_tts::an_explicit_qwen3_speaker_overrides_the_configured_default` | 选中 `qwen3:Vivian` → 请求体确实使用 Vivian |
| `qwen_ai_tts::a_blank_speaker_falls_back_to_the_configured_reply_voice` | 未指定音色 → 使用配置默认（Serena） |
| `media::an_unavailable_cloud_engine_falls_back_to_the_system_voice` | **播报**回退链 + 回滚到 system 后不再触碰 cloud |
| `ipc::qwen3_tts_config_round_trips_through_the_media_dto` | 配置 DTO 序列化/反序列化 |
| `ipc::exposes_the_approved_media_surface_only` | 命令面 35→38 |

Python 侧 10 项：健康、音色目录、WAV 可播放性（RIFF/24k/单声道/16bit）、空文本 400、
超长 413、非法语言 400、非法格式 400、未知路径 404、请求计数、加载中 503 可重试。

## 3. 契约冻结证据

- 服务端响应头 `Content-Type: audio/wav`，body 以 `RIFF` 开头，`wave` 模块解析出 24000Hz / 1ch / 16bit（mock 常量；真实采样率由模型返回并上报 `/health`）。
- 客户端对非 RIFF 响应直接拒绝（不播放未知格式），避免“半截音频”。
- 命令面从 35 增至 38，`permissions/foundation.toml` 与 `lib.rs` 两个通道的 `generate_handler!` 同步注册，契约测试 `registers_the_same_media_commands_for_both_channels_and_main_only` 通过。

## 4. 延迟基线（实测）

`python services/qwen3-tts/bench.py --iterations 20`，进程内 mock 引擎，
**仅含 HTTP + 服务开销，不含模型推理**：

| 场景 | 样本 | 均值(ms) | p50(ms) | p95(ms) | 最大(ms) |
| --- | --- | --- | --- | --- | --- |
| `/health` 探测 | 20 | 1.9 | 0.6 | 1.0 | 24.9 |
| `/v1/voices` 目录 | 20 | 1.2 | 0.7 | 1.0 | 10.8 |
| 短弹幕（10 字） | 20 | 6.6 | 3.9 | 22.9 | 26.7 |
| 常见弹幕（24 字） | 20 | 13.4 | 6.4 | 31.3 | 31.4 |
| 长播报（96 字） | 20 | 14.8 | 9.7 | 30.5 | 31.8 |

解读：传输层 p50 均 < 10ms，p95 < 32ms（`urllib` 每次新建连接，未复用连接池，属于上界）。
**模型推理耗时未测**：上游文档给出流式首包 97ms 的官方指标，本项目未在 GPU 上复测，
因此“首字延迟”一项**标记为未验证**，不得当作已达标。

真实环境复测命令：

```bash
python services/qwen3-tts/server.py --model Qwen/Qwen3-TTS-12Hz-1.7B-CustomVoice
python services/qwen3-tts/bench.py --base-url http://127.0.0.1:8020 --iterations 20
```

## 5. 回滚演练（代码层）

| 演练 | 手段 | 结果 |
| --- | --- | --- |
| 播报：云端不可用 | `an_unavailable_cloud_engine_falls_back_to_the_system_voice` | ✅ Cloud 失败 → 系统引擎发声，`fallbackReasonCode=tts_engine_fallback` |
| 播报：切回系统引擎 | 同上测试后半段 | ✅ 切 `system` 后不再调用云端（cloud 调用计数保持 1），且不标记为降级 |
| AI 回复：服务不可达 | `an_unreachable_service_falls_back_to_the_system_voice` | ✅ 回退系统语音 |
| AI 回复：关闭开关 | `a_disabled_service_falls_back_to_the_system_voice` | ✅ 回退系统语音 |
| AI 回复：选择系统音色 | `a_system_voice_id_never_touches_the_qwen_service` | ✅ 完全不走 Qwen3-TTS |

配置写入路径为单条 SQLite `upsert`（`t_app_setting`），无迁移、无重启依赖；
“关闭开关 → 下一次合成生效”由 `Qwen3TtsService::is_enabled()` 每次调用实时读取配置保证。

## 6. 未覆盖项（必须如实告知）

| 项 | 状态 | 原因 | 复现方式 |
| --- | --- | --- | --- |
| 真实模型合成音质 | ⛔ 未验证 | 本机无 GPU 权重（1.7B 约 4.5GB） | 按 `services/qwen3-tts/README.md` 2.2 启动后试听 |
| 首字延迟 / 总耗时达标 | ⛔ 未验证 | 同上，仅有传输层数据 | `bench.py --base-url` |
| 播报端到端发声 | ⛔ 未验证 | 需桌面应用 + 音频设备 + 真实服务 | 应用内“试听”按钮 |
| AI 回复端到端发声（SAPI `SpeakStream`） | ⛔ 未验证 | 同上，且需 Windows 音频设备 | AI 页面选 `qwen3:Serena` 触发回复 |
| 前端 tsc / vitest | ⛔ 未执行 | `node_modules` 的 junction 指向已搬迁旧路径（`...\业务实战项目\panel\...`），`pnpm exec` 因缺包失败 | `pnpm install --force` 后 `pnpm -F @monorepo/bilibili-live-tool type-check` + `vitest run` |
| ASR 模块回归 | ✅ 已不适用 | `whisper-rs` 依赖与其 ASR 后端已整体移除，不再存在待回归模块 | — |

前 4 项属于“必须由有 GPU + 音频设备的环境完成”的实测项，建议作为发布前置门槛。
