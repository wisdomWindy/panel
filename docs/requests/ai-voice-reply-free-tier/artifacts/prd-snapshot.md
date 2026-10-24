# PRD Snapshot: AI Voice Reply Free Tier

## Verbatim Source Evidence Index

| Ref | Source anchor | Verbatim evidence |
| --- | --- | --- |
| S1 | User instruction, 2026-10-23 | `需求文档已修改，根据修改后的功能模块进行开发` |
| S2 | PRD 0.3 | `AI 语音回复属于免费功能` |
| S3 | PRD 0.11 | `AI 语音回复包含“按规则生成回复文本 + 软件内显示 + 本地 TTS 播报到指定音频设备”` |
| S4 | PRD 0.11 | `不调用 B 站发送弹幕接口，不提供发送按钮或发送 Command` |
| S5 | PRD 9.14.2 | `软件不内置模型，未配置模型时 AI 语音回复不可开启` |
| S6 | PRD 9.14.3 | `Rust 侧统一以 OpenAI 兼容协议发起请求` |
| S7 | PRD 9.14.6 | `Rust 枚举系统音频设备，前端只传设备 ID，Rust 校验白名单` |
| S8 | PRD 9.14.8 | `首次开启需确认；未配置模型时置灰` |
| S9 | PRD 9.14.10.15 | `免费版安装包与会员版安装包均包含 AI 语音回复、AI 模型配置、TTS 音频路由命令` |
| S10 | PRD 9.14.10.16 | `会员到期不影响 AI 语音回复，AI 语音回复继续可用` |
| S11 | PRD 11.7 | `只保留一个 AI 语音回复总开关，不设独立“语音播报”开关` |
| S12 | PRD 11.10 | `API Key 加密存储，导出配置时默认脱敏` |
| S13 | User clarification, 2026-10-23 | Remove the manual input/generate block; when ASR is started without model resources, show a dialog for selecting and downloading an ASR model. |
| S14 | User clarification, 2026-10-23 | Remove the proxy URL field from the AI model profile add/edit surface. |
| S15 | User clarification, 2026-10-23 | Replace the temperature control with a reasoning-effort selector using common AI agent values such as low, medium, and high. |
| S16 | Official OpenAI API and Agents SDK references, checked 2026-10-23 | `reasoning_effort` supports `none`, `minimal`, `low`, `medium`, `high`, and `xhigh`; the current Agents SDK also exposes `max` for supported models. |

## Source Summary

The revised PRD reclassifies AI voice reply as a shared free capability. The implementation must use user-configured external or local OpenAI-compatible model endpoints, keep model credentials in the Rust boundary, generate short conversational replies, display them locally, and play the same text through a user-selected local audio output. Membership controls only automatic moderation.

## Key Business Goals

- `source-backed` Make AI voice reply available in both installers and all membership states.
- `source-backed` Keep AI output local to the application and audio device; never send danmaku.
- `source-backed` Support external cloud providers, aggregators, local model services, and custom OpenAI-compatible endpoints without bundling weights.
- `source-backed` Make activation deliberate: model configured, audio route selected/tested, explicit confirmation accepted.
- `source-backed` Keep replies short, conversational, plain-text, and suitable for immediate TTS playback.

## Greenfield Scaffold Sensitivity

Not applicable: this is an incremental change to the existing Vue/Tauri application.

## Explicit Behavior Constraints

### Packaging and entitlement

- AI route, resources, Rust logic, model configuration, and TTS routing are shared by free and member builds.
- Membership expiry must not hide, disable, or interrupt AI voice reply.
- Member-only scope remains login, membership center, and automatic moderation/spam protection.

### Model profiles

- Profiles support provider type, display name, Base URL, API key, model name, temperature, maximum tokens, timeout, optional proxy, enabled status, connection status, and one default profile.
- Supported categories are domestic/international cloud models, third-party aggregators, local services, and custom OpenAI-compatible endpoints.
- Requests originate in Rust; API keys are encrypted at rest, masked in logs/UI, and redacted from configuration export.
- Deleting the default profile returns AI voice reply to an unconfigured and paused state.

### AI voice reply

- One master switch controls both generation and automatic TTS playback.
- Default is off. Enabling requires a configured default model and first-time audio-route confirmation.
- Trigger/rate settings include trigger scope, global cooldown, per-user cooldown, queue limit, style tier, maximum reply length, maximum sentence count, filtering, deduplication, and priority.
- Replies are a single plain-text conversational paragraph with no headings, lists, Markdown, stock courtesy opening, or AI self-reference.
- Default style target is at most 50 Chinese characters and two sentences; configurable hard limits remain within PRD ranges.
- Displayed reply text and TTS text must be identical.

### TTS route and queue

- Rust enumerates audio output devices; the frontend sends only a selected device ID.
- Test playback is available for route confirmation.
- Device loss pauses AI playback and requires reselection; it must not silently switch to the default speaker.
- AI playback has an independent volume and queue, with pause/resume, skip-current, and clear controls.

### Boundary and failure states

- Missing model: master switch disabled and model configuration path offered.
- Endpoint/LLM failure: show failure, retain input, do not enqueue or report success, apply retry/backoff where specified.
- Invalid/overlong model output: normalize or retry once, then truncate/drop according to hard limits; invalid content is not played.
- Queue overflow: discard lower-priority work first.
- Missing whisper component/model or microphone failure affects ASR only, not text generation or TTS.
- No outgoing Bilibili danmaku UI, IPC command, or automatic task may be added.

## Closed-World Product Scope Allowlist

- Move existing AI Assistant navigation and routes into the shared/free surface.
- Move AI generation/configuration IPC registration into commands shared by both installers.
- Decouple AI state from membership status and expiry.
- Add the PRD-defined model-profile management fields and default-profile/test/delete interactions.
- Add the PRD-defined single AI voice reply switch, activation confirmation, rules, output-device selection/test, queue display, pause/skip/clear, and completed-reply copy interactions.
- Remove the manual prompt/generate/regenerate surface per the user's later clarification.
- Make missing ASR resources recoverable through a selectable model-download dialog.
- Remove the proxy URL field from model-profile add/edit UI and normalize profile saves to no proxy.
- Replace model-profile temperature with a closed reasoning-effort enum and migrate existing profiles to `medium`.
- Add reply text normalization, queue limits/cooldowns, failure states, and tests required by PRD 9.14.
- Adjust configuration import/export redaction and build/capability contracts where the changed ownership requires it.
- Preserve member-only moderation, login, and membership center behavior.

Anything outside this list, including sending messages to Bilibili, analytics expansion, unrelated page redesign, provider-specific proprietary protocols, or automatic OBS configuration, is out of scope.

## Forms, Tables, Displays, And Interactions Extracted From Source

- AI Assistant header: model configuration, persona configuration, logs.
- Runtime form: master switch, route guide, current/default model, trigger scope, cooldowns, output device, test playback.
- Queue display: currently playing, pending count/items, pause/resume, skip, clear.
- Automatic reply display: latest generated result and copy.
- Optional ASR: status/switch; a missing resource opens model selection and download progress instead of a terminal unavailable label.
- Model profile table: profile name, provider, model, status, test/edit/delete/default actions; add and import controls.
- Model profile editor: provider type, profile name, Base URL, API key, model name, temperature, maximum tokens, timeout, proxy, enabled/default state.
- First-enable dialog: route explanation, selected device, test playback, capture-confirmation checkbox, cancel/confirm.
- Persona dialog: identity, style tier, reply length, sentence count, fixed safety constraints, system prompt.

## Workflow And State Rules Extracted From Source

1. Configure and test at least one model profile, then select a default.
2. Configure persona/rules and select an output device.
3. Test playback and explicitly confirm OBS/Live Studio capture before first enable.
4. Filter incoming danmaku before trigger evaluation; eligible events enter the AI queue.
5. Rust assembles context and calls the configured endpoint; frontend receives streaming deltas.
6. Normalize and validate the result; successful text is displayed and enqueued for TTS.
7. Membership state changes do not participate in this state machine.

## Relevant Modules Or Pages

- `/ai`, `/ai/log`, AI Assistant navigation.
- `stores/ai.ts`, desktop contracts/adapters, Tauri invoke registration.
- Rust member-domain AI types/services that require relocation or entitlement removal.
- Existing media/TTS audio device and playback services.
- Configuration management redaction.

## Open Questions

- `code-fact-backed`: the repository currently has no production LLM HTTP adapter; repository implementation can provide validated profile persistence and a clear unavailable/transport boundary, while real endpoint success needs deployment credentials.
- `code-fact-backed`: actual OBS/Live Studio meter detection is not exposed by the current desktop contracts. The repository can implement test playback and explicit user confirmation, but cannot prove third-party capture without external integration.
- No product decision is needed to proceed because the PRD defines safe failure behavior for unavailable external dependencies.

