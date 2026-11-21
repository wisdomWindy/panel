# 请求：首页一键下载 Qwen3-TTS 1.7B 模型

- 请求 ID：`qwen3-tts-model-download`
- 提出时间：2026-11-20
- 前置请求：`qwen3-tts-voice`（语音服务与适配层）、`remove-whisper-asr`

## 原始诉求

软件中添加一个下载 Qwen3-TTS 1.7B 模型的按钮，点击之后下载 1.7B 模型。这个按钮应该放在首页。

## 设计决策

**模型归语音服务所有，下载经由服务完成，首页按钮只做触发与进度展示。**

理由：

1. 权重由 `qwen-tts` 通过 Hugging Face / ModelScope 缓存加载，下载到同一缓存目录后续 `from_pretrained` 直接命中，不会重复下载。
2. 由 Rust 自己实现 Hub 分片下载需要处理 LFS 重定向、断点续传、镜像切换，与上游工具链重复且更脆弱。
3. 服务已具备 `--mock` 能力，接入后整条 UI 流程可无网络演示与测试。

按钮位置：首页（数据看板）底部卡片。

## 接口扩展（服务端）

| 方法 | 路径 | 说明 |
| --- | --- | --- |
| GET | `/v1/models` | 已发布模型清单（含 `is_default`、`is_installed`）+ 当前下载状态 |
| POST | `/v1/models/download` | `{"model": string?}`，缺省下载 1.7B CustomVoice；返回 `202` + 快照 |

新增错误码：`tts_model_unknown`(400)、`tts_download_busy`(409)、`tts_download_unavailable`(503)。
客户端映射为 `qwen3_tts_model_unknown` / `qwen3_tts_download_busy` / `qwen3_tts_download_unavailable`。

## 实现清单

| 层 | 变更 |
| --- | --- |
| 服务端 | `MODEL_CATALOG`（5 个已发布模型）、`HubModelDownloader`（hub → modelscope 回退、目录增量轮询进度）、`MockModelDownloader`、`ServiceState.start_download` 与模型状态机、两个新路由 |
| Rust | `Qwen3ModelEntry/Qwen3DownloadState/Qwen3ModelCatalog`、`model_catalog()`、`start_model_download()`、服务端错误码翻译 |
| IPC | `tts_qwen3_models`、`tts_qwen3_download_model`（命令面 38 → 40）、DTO、权限 |
| 前端 | 契约类型与命令映射、`adaptQwen3ModelCatalog/adaptQwen3DownloadState`、store 的 `qwen3Catalog/loadQwen3Models/downloadQwen3Model`（含 2s 轮询）、首页卡片、双语 12 个文案 |

## 语义与边界

- **幂等**：同一模型重复点击不重复下载；并发下载不同模型返回 409。
- **失败可见**：下载失败时状态为 `error`，卡片显示服务端错误原因，服务本身继续可用。
- **服务未启动**：按钮禁用并提示先启动 `services/qwen3-tts/server.py`，不假装成功。
- **不做**：不支持暂停/取消下载（一期）；不校验磁盘空间；不随包分发权重。

## 验收

见 `verification.md`。
