# 规格：Qwen3-TTS 语音适配（契约冻结）

版本：v1（2026-11-20）｜ 状态：**已冻结**，变更需回退到本文件重新评审

## 1. 能力与架构核实（全部标注出处）

| 事实 | 结论 | 出处 |
| --- | --- | --- |
| 发布信息 | 2026-01-22 发布 0.6B/1.7B 系列，基于 Qwen3-TTS-Tokenizer-12Hz | README `News` |
| 架构 | 离散多码本语言模型（discrete multi-codebook LM）端到端；tokenizer 为 12Hz、16 码本、2048 词表、非 DiT 轻量重建 | README `Introduction` / `Model Architecture` / tokenizer 基准表 |
| 接口形态 | Python 包 `qwen-tts`，`Qwen3TTSModel.from_pretrained(...)`，`generate_custom_voice / generate_voice_design / generate_voice_clone`，返回 `(wavs, sr)` | README `Python Package Usage` |
| 语言 | 中/英/日/韩/德/法/俄/葡/西/意 共 10 种，另支持 `Auto` | README `Introduction`、模型表、`language="Auto"` 说明 |
| 音色 | CustomVoice 固定 9 音色：Vivian、Serena、Uncle_Fu、Dylan、Eric、Ryan、Aiden、Ono_Anna、Sohee | README `Custom Voice Generate` speaker 表 |
| 指令控制 | `instruct` 自然语言控制音色、情绪、韵律、语速 | README `Introduction`、`generate_custom_voice` 示例 |
| 流式 | 单模型同支持流式/非流式；首包延迟低至 97ms（模型文档指标，非本项目实测） | README `Introduction` |
| 采样率 | **README 未标注**，由模型运行时返回 | 未标注 → 服务端 `/health` 回传实际值，不做假设 |
| 授权 | Apache-2.0 | HF 模型元数据 `license:apache-2.0`；技术报告“release both tokenizers and models under the Apache 2.0 license” |
| 体积 | 1.7B-Base 4.54GB、0.6B-Base 2.52GB（HF 权重体积） | HF 模型页/公开评测记录 |
| 部署方式 | `pip install -U qwen-tts`；推荐 Python 3.12 独立环境；可选 FlashAttention-2 | README `Environment Setup` |
| vLLM | vLLM-Omni day-0 支持，**当前仅离线推理**，在线 serving 待支持 | README `vLLM Usage` |
| 官方 API | DashScope 实时 API（CustomVoice / Clone / VoiceDesign） | README `DashScope API Usage` |

## 2. 部署形态选型

**选型：独立进程的本地 HTTP 语音服务（Python `qwen-tts`），桌面端通过 HTTP 调用。**

理由：

1. 模型是 Python + PyTorch 生态，Rust 侧无法内联推理；进程隔离同时规避依赖冲突（README 明确建议独立环境）。
2. 桌面应用体积与显卡差异大，服务可独立部署、独立升级、独立降级。
3. 与既有架构一致：项目已有 OpenAI 兼容 LLM 外部依赖，HTTP 适配是既定模式。
4. 失败可观测、可回退：服务未启动时客户端立即回退系统语音。

备选与取舍：

| 方案 | 采纳 | 原因 |
| --- | --- | --- |
| 本地 HTTP 服务（本项目实现） | ✅ | 可离线、可自托管、Apache-2.0、无按次计费 |
| DashScope 实时 API | ⏸ 预留 | 需密钥与联网；`baseUrl` 可指向兼容网关，接口层无需改动 |
| vLLM-Omni | ❌ 一期不用 | 目前仅离线推理，无在线 serving |
| 进程内 Rust FFI/ONNX | ❌ | 上游无 Rust 绑定，量化方案非官方 |

### 资源预算（量化）

| 项 | 1.7B（默认） | 0.6B（低配） | 依据 |
| --- | --- | --- | --- |
| 权重磁盘 | ≈4.5GB | ≈2.5GB | HF 权重体积 |
| 显存（bf16 推理） | ≈4–6GB | ≈2–3GB | 权重 + KV/激活估算（**估算值**，需实测确认） |
| 系统内存 | ≥16GB | ≥8GB | 加载峰值估算 |
| 首字延迟（文档指标） | 97ms（流式首包，官方指标） | 同左 | README，**非本项目实测** |
| 传输层开销（本项目实测） | p50 3.9–9.7ms / p95 22.9–30.5ms | 同左 | 见 `verification/verification.md` 第 4 节 |
| 建议并发 | 1（单卡串行，服务端互斥） | 1 | 本实现用全局锁串行推理，避免显存抖动 |

## 3. 冻结契约

### 3.1 服务端 HTTP 契约（`services/qwen3-tts/server.py`）

```http
GET /health
200 {"status":"ok|loading|degraded","ready":bool,"engine":"qwen3-tts|mock",
     "model":str,"sample_rate":int,"languages":[str],
     "voices":[{voice_id,display_name,language}],"error":str|null,
     "uptime_seconds":num,"requests":int,"failures":int}

GET /v1/voices
200 {"voices":[{voice_id,display_name,language}],"languages":[str]}

POST /v1/audio/speech
{"text":str,"voice":str|null,"language":str|null,"instruct":str|null,"response_format":"wav"}
200 audio/wav（RIFF，单声道 16-bit PCM，采样率由模型决定）
4xx/5xx {"error":{"code":str,"message":str,"retryable":bool}}
```

### 3.1.1 契约扩展 v1.1（模型下载）

后续请求 `qwen3-tts-model-download` 增加了两个端点，原端点语义不变：

```http
GET /v1/models
200 {"models":[{"model_id","display_name","task","is_default","is_installed"}],
     "download":{"model_id"|null,"status":"idle|downloading|ready|error",
                 "progress_percent"|null,"bytes_downloaded","bytes_total"|null,
                 "path"|null,"error"|null}}

POST /v1/models/download
{"model": str|null}          # 缺省为 Qwen/Qwen3-TTS-12Hz-1.7B-CustomVoice
202 {"model_id","status":"downloading","progress_percent":0,...}
400 tts_model_unknown ｜ 409 tts_download_busy ｜ 503 tts_download_unavailable
```

客户端对应命令：`tts_qwen3_models`、`tts_qwen3_download_model`；错误码 `qwen3_tts_model_unknown` /
`qwen3_tts_download_busy` / `qwen3_tts_download_unavailable`。

错误码（服务端）：`tts_invalid_request`、`tts_unsupported_format`、`tts_unsupported_language`、`tts_instruct_required`、`tts_model_loading`、`tts_synthesis_failed`、`tts_not_found`。

### 3.2 Rust 侧抽象（`application/media/ports.rs`）

```rust
pub struct SynthesisRequest { text, voice_id, language, instruct, rate }
pub struct SynthesisVoice { voice_id, display_name, language }
pub struct SynthesisHealth { ready, engine, model, sample_rate, error }

#[async_trait]
pub trait SpeechSynthesisPort: Send + Sync {
    async fn health(&self) -> Result<SynthesisHealth, MediaIoError>;
    async fn voices(&self) -> Result<Vec<SynthesisVoice>, MediaIoError>;
    async fn synthesize_wav(&self, request: SynthesisRequest) -> Result<Vec<u8>, MediaIoError>;
}
```

调用方只依赖该 trait 与 `SpeechEnginePort` / `AiSpeechOutput`，**不感知模型**。

### 3.3 错误码（客户端）

| 码 | 含义 | 可重试 | 触发回退 |
| --- | --- | --- | --- |
| `qwen3_tts_disabled` | 开关关闭 | 否 | 是 |
| `qwen3_tts_unreachable` | 连接失败/被拒 | 是 | 是 |
| `qwen3_tts_timeout` | 超过 `timeoutMs` | 是 | 是 |
| `qwen3_tts_loading` | 模型加载中（HTTP 503） | 是 | 是 |
| `qwen3_tts_invalid_request` | 文本/语言/格式非法（4xx） | 否 | 是（AI 路径） |
| `qwen3_tts_invalid_response` | 返回非 RIFF/WAVE | 否 | 是 |
| `qwen3_tts_invalid_config` | 配置校验失败 | 否 | 否（保存即失败） |
| `qwen3_tts_synthesis_failed` | 服务端 5xx | 是 | 是 |

## 4. 适配器设计

| 适配器 | 位置 | 实现方式 |
| --- | --- | --- |
| `Qwen3TtsService` | `infrastructure/media/qwen_tts.rs` | 配置读写（`t_app_setting` 键 `media.qwen3_tts`）+ reqwest 传输 + `SpeechSynthesisPort` |
| `Qwen3TtsSpeechEngine` | 同上 | 实现 `SpeechEnginePort`：合成 → 临时 WAV → `RodioAudioOutput` 播放 → 轮询结束（上限 120s） |
| `Qwen3TtsAiSpeechOutput` | `infrastructure/member/qwen_ai_tts.rs` | 实现 `AiSpeechOutput`：合成 → 临时 WAV → SAPI `ISpeechFileStream` + `ISpeechVoice::SpeakStream`（保持设备路由）→ 失败回退 `SystemAiSpeechOutput` |

### 旧实现兼容策略

1. **播报**：`TtsRuntime` 启动时原本传入 `(System, Unavailable, Unavailable)`；现将 Cloud 槽位替换为 `Qwen3TtsSpeechEngine`。回退链 `System → Cloud → LocalFallback` 不变，`selectedEngine=system` 时行为与改造前完全一致。
2. **AI 回复**：用 `Qwen3TtsAiSpeechOutput` 包裹 `SystemAiSpeechOutput`。当 `voice_id` 不以 `qwen3:` 前缀开头时**直接透传**给系统引擎，因此历史配置（如 `gentle-peach`）零改动继续工作。
3. 未启用/不可达时：`voices()` 不返回 Qwen 音色（前端“云端”选项自动置灰），`speak()` 回退系统引擎。

## 5. 配置项

存储：`t_app_setting`，`key = media.qwen3_tts`，`scope = global`（JSON，**无需数据库迁移**）。

| 字段 | 类型 | 默认 | 约束 |
| --- | --- | --- | --- |
| `isEnabled` | bool | `false` | — |
| `baseUrl` | string | `http://127.0.0.1:8020` | 必须 http/https 且有 host |
| `voiceId` | string? | `Serena` | 空则用默认 |
| `language` | enum | `Chinese` | 11 个合法值 |
| `instruct` | string | 空 | ≤400 字符 |
| `timeoutMs` | u64 | `8000` | 500–60000 |
| `fallbackToSystem` | bool | `true` | — |
| `aiVoiceId` | string? | `null` | 空则复用 `voiceId` |
| `aiInstruct` | string? | `null` | ≤400 字符；空则复用 `instruct` |

## 6. 降级 / 超时 / 取消 / 缓存语义

| 语义 | 规则 |
| --- | --- |
| 超时 | 单请求 `tokio::time::timeout(timeoutMs)`；取音频体额外 +10s |
| 重试 | 不自动重试（避免直播排队放大延迟），由上层回退链处理；`retryable` 供 UI 提示 |
| 取消 | 播报：`stop()` 停止 rodio 播放并丢弃当前项；AI 回复：`cancel_epoch` 自增 + SAPI `SVSFPurgeBeforeSpeak` 清空 |
| 缓存 | 一期不做音频缓存（文本模板化但含用户名，命中率低）；服务端保留 `requests/failures` 计数 |
| 限流 | 服务端单锁串行；客户端队列容量沿用既有（播报 128 / AI 可配） |
| 静音降级 | 全部失败且 `fallbackToSystem=false` 时返回错误，不播放任何声音（绝不播放半截音频） |
| 临时文件 | `%TEMP%/bili-tts-<uuid>.wav`、`bili-ai-tts-<uuid>.wav`，播放结束/Cancel 即删除 |
