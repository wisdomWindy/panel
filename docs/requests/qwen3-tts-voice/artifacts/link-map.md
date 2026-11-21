# 语音链路现状图与 Qwen3-TTS 对照表

请求：`qwen3-tts-voice` ｜ 日期：2026-11-20 ｜ 代码基线：`packages/bilibili-live-tool` @ 改造前

## 1. 语音播报链路（事件驱动）

```
B 站开放平台 WebSocket
  └─ infrastructure/live_data/open_live/transport.rs      连接与心跳
      └─ open_live/protocol.rs + open_live/event_mapper.rs  帧解析 → LiveEvent
          └─ application/live_data/*  AcceptedLiveEventFanout  分流到各 consumer
              └─ MediaState::consume()            application/media/state.rs:190
                  └─ TtsRuntime::enqueue_event()  application/media/tts.rs:107
                      ├─ render_event()           application/media/tts.rs:305   模板渲染 + 身份/事件过滤
                      └─ 优先队列 (cap 128)        application/media/tts.rs:16
                          └─ TtsWorker::run()     application/media/tts.rs:236
                              └─ SpeechEnginePort::speak()   application/media/ports.rs:200
                                  ├─ SystemSpeechEngine        infrastructure/media/speech.rs:6   (tts crate)
                                  ├─ Cloud                    基础设施原为 UnavailableSpeechEngine
                                  └─ LocalFallback            基础设施原为 UnavailableSpeechEngine
                                      └─ 系统默认音频输出设备（无显式设备选择）
```

- 配置读写：`tts_get` / `tts_save` / `tts_set_enabled` / `tts_test` / `tts_pause` / `tts_clear`（`ipc/media.rs`），持久化表 `t_tts_config`（`infrastructure/media/repository.rs:103`）。
- 前端入口：`renderer/views/tts/index.vue` + `renderer/stores/media/tts.ts` + `renderer/platform/desktop/contracts/media.ts`（已存在 `system | cloud` 引擎选择与“云端不可用”提示）。
- 状态出口：`TtsRuntime::snapshot()` → `TtsStatusDto` → 事件 `tts_status_changed`。

## 2. AI 语音回复链路

```
弹幕/互动事件 → AiState 生成流程（application/member/state.rs）
  ├─ LlmProvider::generate()            application/member/ports.rs:452
  │   └─ OpenAiCompatibleLlmProvider    infrastructure/member/ai.rs:164   OpenAI 兼容 Chat Completions
  └─ AiVoiceRuntime::enqueue()          application/member/state.rs:817   文本入队（优先级 + 冷却）
      └─ AiVoiceRuntime::run()          application/member/state.rs:306   工作线程
          └─ AiSpeechOutput::speak()    application/member/ports.rs:471
              └─ SystemAiSpeechOutput   infrastructure/member/ai.rs:296
                  └─ Windows SAPI SpVoice（putref_AudioOutput 设备路由 + Speak 同步等待）
                      └─ 运维选定的输出设备（AI 回复要求音频路由已确认）
```

- 配置：`t_ai_voice_config`（`voice_id` / `output_device_id` / `is_audio_route_confirmed`），命令 `ai_tts_list_output_devices` / `ai_tts_set_output_device` / `ai_tts_test` / `ai_tts_pause` / `ai_tts_skip` / `ai_tts_clear_queue`（`ipc/member.rs`）。
- 前端入口：`renderer/views/ai/components/AiVoiceReplyControl.vue`。

## 3. 两条链路的差异与共性

| 维度 | 语音播报 | AI 语音回复 |
| --- | --- | --- |
| 文本来源 | 固定模板渲染（`{用户}/{内容}/{礼物}/{数量}`） | LLM 生成（口语短句系统提示词） |
| 抽象接口 | `SpeechEnginePort`（voices/speak/stop） | `AiSpeechOutput`（devices/voices/speak/stop） |
| 播放出口 | `tts` crate，系统默认设备 | SAPI `SpVoice`，显式设备路由 |
| 队列 | `TtsRuntime` 优先队列，容量 128 | `AiVoiceRuntime` 优先队列，容量可配 |
| 失败语义 | 逐级回退 System→Cloud→Local | 丢设备即自动降级并禁用 |

**结论**：两条链路的播放出口机制不同（默认设备 vs 显式设备），因此 Qwen3-TTS 必须分别适配：播报用“合成 + rodio 播放”，AI 回复用“合成 + SAPI `SpeakStream`”，否则会破坏 AI 回复的音频路由能力。

## 4. 现状能力与 Qwen3-TTS 能力对照

| 能力 | 改造前现状 | Qwen3-TTS（出处） | 差异处置 |
| --- | --- | --- | --- |
| 合成引擎 | Windows SAPI / `tts` crate | `qwen_tts.Qwen3TTSModel`（README Quickstart） | 新增 HTTP 服务 + Rust 适配器 |
| 音色 | 系统音色（SAPI token） | 9 个 CustomVoice 音色；VoiceDesign 自然语言造音；Base 3 秒克隆（README 模型表 + speaker 表） | 采用 CustomVoice 9 音色，克隆/造音列为后续 |
| 语言 | 依赖系统音色 | 10 语言 + Auto（README Introduction / 模型表） | 配置项 `language`，默认 `Chinese` |
| 指令控制 | 仅语速/音量 | `instruct` 自然语言控制音色/情绪/韵律（README Introduction） | 新增 `instruct`，语速倍率折算为指令 |
| 流式 | 无 | 单模型同时支持流式与非流式，首包延迟低至 97ms（README Introduction） | 一期用非流式整段 WAV，流式列入遗留 |
| 采样率 | 系统决定 | README 未标注；由模型运行时返回 | 服务端 `/health` 上报实际 `sample_rate`，不臆测 |
| 音色清单来源 | SAPI `GetVoices` | `/v1/voices` 由服务端枚举 | 播报“云端”音色下拉即可直接复用 |
| 授权 | — | Apache-2.0（HF 模型元数据 `license:apache-2.0`；技术报告“tokenizers and models under the Apache 2.0 license”） | 可商用自托管；发布时需随包声明 |
| 许可与体积 | — | 1.7B-Base 4.54GB / 0.6B-Base 2.52GB（HF） | 见 `spec/spec.md` 资源预算 |

## 5. 风险点清单

| 风险 | 影响 | 处置 |
| --- | --- | --- |
| 模型需 GPU 与数 GB 权重，无法随桌面应用打包 | 首启不可用 | 独立 Python 服务，默认关闭（`isEnabled=false`），未就绪时自动回退系统引擎 |
| 服务不可达/超时 | 播报堆积、AI 回复中断 | 超时 500–60000ms 可配；失败回退系统引擎（`fallbackToSystem`） |
| AI 回复要求设备路由 | 语音进错设备会泄漏到观众端 | 复用 SAPI `SpeakStream`，保持原有 `output_device_id` 语义 |
| `rate` 是倍率，Qwen3-TTS 无倍率参数 | 语速设置失效 | 折算为指令（≥1.15“语速稍快”，≤0.85“语速稍慢”），不做重采样 |
| 首字延迟未知（未实测真实模型） | 直播场景体验 | 以 `/health` 就绪探测 + 超时回退兜底；真实延迟待 GPU 环境实测（见 verification 遗留项） |
| `whisper-rs-sys` 构建需要 libclang/CMake/MSVC | 本机 `cargo check` 被阻断（既有问题） | 验证脚本临时隔离该依赖；已记录在 verification 限制项 |
| 前端 `node_modules` junction 指向已搬迁的旧路径 | 无法运行 tsc/vitest | 需 `pnpm install` 重建；已记录为环境遗留 |
