# 请求：移除 whisper-rs-sys 及本地语音识别

- 请求 ID：`remove-whisper-asr`
- 提出时间：2026-11-20
- 前置请求：`qwen3-tts-voice`（其验证记录中该依赖被标记为构建阻塞项）

## 原始诉求

移除 whisper-rs-sys。

## 决策

经确认，采取**彻底删除**方案：依赖 + ASR 后端 + 前端入口一并移除，不留可选 feature。

理由：

1. `whisper-rs-sys` 需要 libclang + CMake + MSVC 完整工具链，在本机与多数开发机上直接导致 `cargo check` / `cargo test` 失败，是实际的交付阻塞。
2. 该依赖在项目内只被 `infrastructure/member/asr_runtime.rs` 引用，服务的是**本地进程内 Whisper 推理**这一条链路。
3. 语音识别此前并未随应用发布（`whisper_artifact_available` 恒为 `false`），属于未上线能力。

## 范围

已移除：

| 层 | 内容 |
| --- | --- |
| 依赖 | `whisper-rs = "0.16.0"` |
| 基础设施 | `infrastructure/member/asr.rs`、`asr_runtime.rs`（推理 + 麦克风采集）、`UnavailableAsrProvider` |
| 应用层 | `AsrProvider` 端口、`AsrStatusRecord` / `AsrModelRecord` / `AsrState`、`AiState` 的 ASR 字段与 `asr_models/download_asr_model/start_asr/stop_asr`、`MemberEventSink` 的 `publish_asr_*` |
| IPC | `asr_status` / `asr_model_list` / `asr_model_download` / `asr_start` / `asr_stop`、`AsrStateDto` / `AsrStatusDto` / `AsrModelManifestDto` / `AsrDownloadRequestDto`、`asr_transcript` / `asr_status_changed` 事件、`asr_*` 错误文案 |
| 配置 | `AiConfigRecord.asr_context_count`（含仓储读写与前端「语音上下文条数」）、`AiContextSourceDto::Asr` |
| 发布状态 | `whisper_artifact_available`（Rust 结构体 + 前端契约） |
| 前端 | ASR 契约类型与命令映射、`adaptAsrStatus/adaptAsrModelManifest/adaptAsrModels`、AI store 的 ASR 状态与动作、AI 页面 ASR 开关与识别结果区、`AsrModelDownloadDialog.vue`、`useAiRuntime` 的两个 ASR 事件监听、`typings/whisper-node.d.ts`、双语 `ai.asr` / `settings.asr` 文案 |

## 刻意保留（非 ASR 引擎）

| 项 | 原因 |
| --- | --- |
| 数据库表 `t_asr_config`、列 `t_ai_config.asr_context_count` | 需要破坏性迁移才能删除；保留列并停止读写即可，写入时走列默认值。**不新增迁移、不改变 `SCHEMA_VERSION`** |
| `speech` / `tts` 相关能力 | 与 ASR 无关，属于 Qwen3-TTS 交付 |

## 影响面

- 用户可见：AI 页面不再出现语音识别开关、模型下载弹窗与识别结果面板；人物配置里不再有「语音上下文条数」。
- 命令面：AI 命令从 24 条降至 19 条；事件从 4 个降至 2 个。
- 构建：`cargo check` / `cargo test` 不再依赖 libclang 与 CMake。

## 验收

见 `verification.md`。
