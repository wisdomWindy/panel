# Page Design: Shared AI Voice Reply

## Delivery Unit

`shared-ai-voice-reply`

## Page Objective

Let a streamer configure a model and audio route, deliberately enable AI voice reply, monitor generated/spoken replies, and recover from provider/device failures without confusing the feature with membership or outgoing danmaku.

## Target Users And Scenarios

- Free or member-build streamer configuring AI for the first time.
- Returning streamer monitoring the live reply queue and adjusting runtime rules.
- Streamer recovering from model endpoint, device, or queue failure.

## Source And Upstream Constraints

- PRD 9.14 supplies all fields, controls, state rules, and failure semantics.
- Requirement analysis fixes AI as membership-independent and forbids outgoing Bilibili sends.
- Existing page uses Element Plus and Tailwind utility classes; preserve these conventions.
- Existing generic TTS remains a separate feature.

## Product-Content Allowlist Mapping

| Proposed UI content | Source |
| --- | --- |
| AI voice reply master switch | PRD 9.14.5, 9.14.8 |
| Default model status and model configuration | PRD 9.14.3, 9.14.7 |
| Audio route, device selection, test playback, confirmation | PRD 9.14.6-9.14.8 |
| Trigger/cooldown/limit/style settings | PRD 9.14.5 |
| Queue current/pending and pause/skip/clear | PRD 9.14.6-9.14.8 |
| Latest automatic reply result and copy | PRD 9.14.2, 9.14.7-9.14.8 plus user clarification removing manual generation |

## 2026-10-23 Reasoning Effort Control Amendment

- Replace the temperature stepper in the model-profile drawer with a full-width select labelled `推理强度`.
- Display the raw interoperable values `none`, `minimal`, `low`, `medium`, `high`, `xhigh`, and `max`; default to `medium` and preserve the selected value while editing.
- Keep the control in the existing two-column settings grid next to `最大 Token`; keyboard selection, focus, disabled state, and drawer responsiveness continue to come from the existing Element Plus select behavior.
| Model profile table/editor fields/actions | PRD 9.14.3, 9.14.7 |
| Persona settings | PRD 9.14.4, 9.14.7 |
| Missing-model/provider/device/ASR errors | PRD 9.14.9 |

No membership renewal notice, upgrade action, send button, automatic OBS configuration, or provider-specific marketing content is permitted.

## User Journey And Task Success

1. User opens AI Assistant from shared navigation.
2. The status band shows disabled/unconfigured, default model, selected device, and the master switch.
3. If unconfigured, the model action opens profile management; save/test/select-default provides a visible ready state.
4. The user selects an output device and uses test playback.
5. On first enable, a modal explains local TTS routing and requires explicit capture confirmation.
6. Enabled runtime shows current/pending queue and the latest automatically generated text. Users can pause/resume, skip, clear, or copy.
7. If the user starts ASR without local model resources, a dialog opens with selectable model sizes, download state, and retryable error feedback.
8. Provider/device failures preserve configuration and identify the next allowed action.

Primary action: enable/disable AI voice reply after prerequisites. Secondary actions: configure model/persona/rules/route, test, queue controls, and download an ASR model. Destructive action: delete model profile, with confirmation because deleting the default disables reply.

## Information Architecture And Content Strategy

1. Header: title plus model configuration, persona, logs.
2. Readiness band: master switch, compact status chips for model and route, route guide/test action.
3. Runtime queue: current playback and pending items; controls adjacent to the affected queue.
4. Latest reply output: read-only automatic result plus copy action; no manual prompt controls.
5. Optional ASR: compact state row; missing resources branch into a focused model-download dialog.
6. Advanced rules: progressively disclosed settings rather than competing with runtime monitoring.

Readiness and failure information stays above the queue so users do not attempt generation before prerequisites. ASR is secondary and labeled optional because its absence does not block text/TTS.

## Layout Structure

- Use an unframed constrained page (`max-w-6xl`) rather than nested cards.
- Header and readiness band are full-width page sections separated by borders.
- Runtime sections flow vertically: readiness, queue, ASR, then the read-only latest reply result. Removing the manual editor avoids a large empty workspace.
- Model profiles open in a wide dialog/drawer with table first and editor as a focused secondary dialog.
- First-enable confirmation uses a modal with device select/test and confirmation checkbox in one sequence.
- Advanced rule settings use a drawer or collapsible section with two-column field grid on desktop and one column on mobile.

## Visual Hierarchy And Direction

- Quiet operational console: compact controls, neutral surfaces, restrained borders, existing Bilibili accent only for primary state/action.
- Page title is compact; section headings remain `text-base` or equivalent.
- Status uses icon + text/tag, never color alone.
- Active/paused/error states have distinct labels and icons; no decorative gradients or oversized cards.
- Queue rows prioritize speaker/user, generated reply, and status; metadata is secondary.

## Section Breakdown

- `AI Assistant Header`: route-level actions, no local state.
- `Voice Reply Readiness`: master switch, model/device status, test/guide; owns activation interaction lifecycle.
- `Reply Queue`: current and pending items, pause/skip/clear; focused runtime test surface.
- `Latest Reply Result`: read-only automatic result and copy; no prompt, generate, or regenerate controls.
- `Model Profiles`: profile table plus editor lifecycle.
- `Persona And Rules`: persona and hard format limits.
- `Optional ASR`: status/start/stop isolated from main readiness; missing resources open the reusable selectable download dialog.

## Styling Direction

- Author only Tailwind CSS-style utility classes.
- No new scoped CSS, CSS modules, Sass/Less, inline style objects, semantic CSS classes, gradients, or decorative backgrounds.
- Keep utilities inline and reviewable; split semantic regions before class values exceed formatter width.
- Continue using Element Plus controls for accessible form, dialog, table, switch, slider/input-number, alerts, and confirmation behavior.

## Interaction Skeleton

- Switch focus/click: disabled with visible reason when no default model/device; first valid enable opens confirmation.
- Test model: pessimistic pending state on the affected row/editor; success/failure status from Rust.
- Save profile/rules: disabled while pending; server validation shown next to/form-level as supported by existing patterns.
- Delete profile: confirmation; deleting default transitions to unconfigured and pauses runtime.
- Test playback: device-specific pending state; failure keeps selection and provides reselection path.
- Queue controls: idempotent/deduplicated store actions; skip/clear disabled when no applicable item.
- Dialog cancellation restores focus to its trigger.
- ASR enable while `missing` or `unavailable` opens model selection instead of invoking start repeatedly.
- Model download disables duplicate submissions, shows determinate progress when supplied, refreshes ASR status on completion, and starts ASR only after the model becomes ready.

## State And Feedback Model

- Loading: section-level bootstrap skeleton/spinner; runtime actions retain stable layout.
- Unconfigured: master switch disabled; model action is primary recovery path.
- Ready-disabled: model/device shown; switch available; no success claim before confirmation.
- Enabled-idle: queue empty state indicates no pending replies without adding an action outside PRD.
- Generating/playing: current item and progress/status visible; conflicting action deduplicated.
- Provider failure/rate limit: no queue success; automatic processing recovers on the next eligible event after cooldown.
- Device missing: AI playback paused, device status error, no silent fallback.
- ASR missing/error: isolated warning with a clear “下载模型” recovery action; model generation and TTS controls remain enabled.
- Membership expired: no AI-specific visual change.

## UX Writing And Feedback Copy Stance

Use PRD terminology verbatim: “AI 语音回复”, “AI 模型配置”, “音频路由向导”, “测试播报”, “暂停播报”, “跳过当前”, “清空队列”. Error messages identify the failed boundary without exposing API keys or raw provider payloads. Do not introduce membership or upgrade copy on this page.

## Responsive Behavior

- Header actions wrap without overlapping title.
- Two-column runtime collapses to queue then generation in task order.
- Profile table becomes horizontally scrollable or row-detail presentation; actions remain reachable.
- Dialogs use viewport-constrained width/height with internal scrolling; footers remain visible.
- Fixed controls keep stable height and do not resize on status-label changes.

## Accessibility And Keyboard Considerations

- Use native/Element Plus semantic controls and explicit labels.
- Status has text plus icon/color; validation links to fields.
- Dialogs trap focus and restore it on close through component-library behavior.
- All queue/model actions are keyboard reachable; icon-only actions include tooltips/accessible labels.
- Confirmation checkbox is required and focusable before confirm enable.

## Usability Heuristics And Risk Controls

- Cognitive load: progressive disclosure keeps model/rules separate from live monitoring.
- Accidental activation: explicit prerequisites plus first-enable confirmation.
- Secret exposure: API key is write-only replacement; configured state is textual, never echoed.
- Hidden dependency: model and device status are colocated with the switch.
- Queue mistakes: controls are adjacent to current queue state and disabled when inapplicable.
- Responsive overflow: stable grid tracks and constrained dialogs.

## Component Reuse And Split Strategy

| Candidate | Decision | Reason / boundary |
| --- | --- | --- |
| Existing `AiPersonaConfigDialog` | Adapt/reuse | Cohesive single-caller form lifecycle already owns persona editing. |
| Existing `AsrModelDownloadDialog` | Adapt/reuse | Owns model selection, progress, error, retry, and completion; opened by the page when ASR lacks resources. |
| `AiModelProfilesDialog` | Extract | One caller, allowed cohesive business section + interaction lifecycle + focused tests; owns temporary profile selection/editor visibility, emits semantic profile actions. |
| `AiVoiceReplyControl` | Extract | One caller, allowed cohesive runtime section and focused test boundary; receives normalized readiness/queue props and emits enable/test/pause/skip/clear intentions. |
| Latest result area | Keep inline | Read-only automatic reply feedback with one copy action does not justify another component. |

State owner remains the AI Pinia store; extracted components own only temporary UI state. Components receive explicit normalized props, never raw backend DTOs. No slots are required.

## Design Risks

- Model/profile density may overwhelm live controls: keep it in a dialog/drawer.
- A generic TTS control could imply two switches: label every runtime audio action as AI voice reply and keep generic TTS out of this page.
- Existing mojibake text in the page can undermine comprehension: touched copy must be corrected to source-backed UTF-8 wording.

## Open UI Questions

None blocking. Exact provider logos and OBS meter automation are not source-authorized and are excluded.

