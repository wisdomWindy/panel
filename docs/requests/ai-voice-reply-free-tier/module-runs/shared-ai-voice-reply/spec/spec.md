# Specification: Shared AI Voice Reply

## Scope And Source Mapping

This spec covers only `shared-ai-voice-reply` and carries PRD 9.14 plus the free/member ownership constraints in PRD 0, 5, 8, 10, and 11. The closed-world allowlist is the module snapshot; outgoing Bilibili messages, OBS automation, and unrelated membership/UI redesign are forbidden.

## User Intent Contract

- Literal: develop the modified PRD modules.
- Practical goal: ship AI voice reply as a complete free/shared capability, not a renamed member-only preview.
- Required evidence: both build variants expose the same AI route/runtime/IPC; AI never reads entitlement; configuration/activation/queue/error flows work; no outgoing danmaku path exists.

## Functional Contract

### Shared availability

- `/ai` and `/ai/log` are common routes and the `ai` navigation item exists in the shared navigation tree.
- Free mode continues to omit membership and moderation routes/navigation/runtime.
- Member mode continues to gate moderation by entitlement; AI is absent from member capability checks.
- Shared AI runtime starts once from application bootstrap and owns AI/ASR event subscriptions. Member runtime must not subscribe to or dispose AI state.

### Model configuration

- A model profile has `profileId`, `name`, `providerKind`, `baseUrl`, `model`, `temperature`, `maxTokens`, `timeoutSeconds`, `proxyUrl`, `isEnabled`, `isDefault`, `connectionState`, `isApiKeyConfigured`, and `updatedAt`.
- `providerKind` supports `cloud`, `aggregator`, `local`, and `customOpenAi` categories. All use OpenAI-compatible HTTP semantics at the Rust boundary.
- API keys are replacement-only input. Read DTOs contain only `isApiKeyConfigured`; logs, errors, and export must not expose plaintext.
- Profile name/base URL/model are trimmed. Empty name/model/base URL, invalid HTTP(S) URL, temperature outside 0-2, max tokens outside 1-8192, timeout outside 1-120 seconds, or malformed optional proxy fail fast.
- Exactly zero or one default profile exists. Selecting default requires an enabled profile. Deleting the default pauses/disables AI voice reply.
- Connection test has explicit pending, success, and failure state; an unavailable external endpoint is never reported as success.

### Persona and reply rules

- Persona fields: identity, system prompt, style tier (`concise`, `natural`, `detailed`), maximum reply characters (20-80), maximum sentences (1-3).
- Runtime rules: trigger scope, global cooldown (1-60 seconds), per-user cooldown (5-300 seconds), queue capacity (5-100), sensitive filter, duplicate/similar suppression, and priority order.
- Fixed constraints cannot be disabled: single plain-text paragraph, no list/heading/Markdown, no stock courtesy opening, no AI self-reference.
- Text inputs are trimmed on submit; whitespace-only required values fail. Length checks use normalized text.

### Voice reply activation

- The only generation/playback enablement is `isVoiceReplyEnabled`.
- Initial value is false. It cannot become true without an enabled default model profile and selected output device.
- First enable, and enable after device change/loss, requires `isAudioRouteConfirmed=true` supplied by the confirmation dialog after test playback opportunity.
- Disabling stops new automatic generation and AI playback; it does not delete profiles, settings, logs, or completed results.
- Membership state and events never change AI enablement/readiness.

### Generation and normalization

- The AI page exposes no manual prompt, context picker, generate, regenerate, or clear-result controls. Generation remains automatic from eligible live events.
- Automatic generation accepts only already-filtered eligible events from the live-data boundary.
- One generation may be active at a time. Concurrent requests deduplicate/reject with `ai_generation_pending`; stale deltas are ignored by generation ID and monotonically increasing sequence.
- Rust builds the provider request and applies a timeout from the selected profile.
- Returned text is normalized: remove Markdown/list/title markers, collapse to one paragraph, remove stock courtesy/AI self-reference prefixes, enforce sentence and character limits, then re-check sensitive terms.
- A completed generation contains the exact normalized text used for display and AI TTS. Failed/rejected output is not enqueued.
- Provider timeout, transport failure, rate limit, invalid output, and sensitive output are distinct error codes and never produce success.

### Audio routing and queue

- Rust owns output-device enumeration and returns opaque device IDs plus display names and availability.
- Frontend sends only a device ID; Rust rejects unknown/unavailable IDs.
- AI voice reply owns a separate `AiVoiceQueueSnapshot`: `state`, `current`, `pending`, `capacity`, `volume`, `selectedDeviceId`, `isAudioRouteConfirmed`, `lastErrorCode`.
- Queue actions: `pause(isPaused)`, `skipCurrent`, `clear`, `test(deviceId, volume)`. Actions are idempotent where repeat calls are possible.
- Queue capacity eviction is deterministic and drops the lowest-priority pending item first.
- Device loss pauses the queue, clears route confirmation, and returns `ai_audio_device_missing`; it never switches speakers silently.
- Missing whisper/model/microphone changes ASR state only.

### ASR model acquisition

- Rust owns a closed model catalog. Each entry exposes only `modelId`, display name, byte size, license, source label, and installed state; download URL, checksum, and destination path never cross into the frontend.
- The initial selectable catalog contains official whisper.cpp-compatible Whisper `tiny`, `base`, and `small` models. Unknown model IDs fail with `asr_model_unknown`.
- Turning ASR on while the model is missing or unavailable opens the model dialog instead of showing a terminal unavailable message.
- The dialog requires one model selection and explicit license acceptance before download. It shows size/source, section-owned pending/progress state, retryable failure copy, and installed state.
- Rust downloads to an application-data temporary file, verifies the catalog checksum, and atomically renames the completed artifact. Failed downloads do not leave a ready model.
- After a successful download the catalog and ASR status refresh. Runtime engine absence is reported separately as `asr_engine_unavailable`; it must not be presented as a missing model.

## UI Contract

- Page order follows page design: header, readiness band, queue, optional ASR, latest automatic result.
- Readiness shows switch, default model name/state, output device/state, test/route actions, and precise disabled reason.
- Model profiles dialog provides list/table, add/edit/test/delete/set-default actions and a write-only API-key field. It does not expose a proxy URL control; profile saves normalize the compatibility-only `proxyUrl` field to `null`.
- Model profiles use `reasoningEffort` instead of numeric `temperature`. The UI is a required select with the closed values `none | minimal | low | medium | high | xhigh | max`; new and migrated profiles use `medium`, while unknown values fail validation.
- OpenAI-compatible requests send the selected value as `reasoning_effort` and do not send `temperature`. Provider rejection remains an explicit connection/generation failure with no silent fallback.
- First-enable dialog shows route explanation, device selection, test playback, required confirmation checkbox, cancel, and confirm enable.
- Queue shows current spoken text and pending items with pause/resume, skip, clear.
- The latest automatic result is read-only and keeps its copy action. The obsolete context and manual input blocks are absent.
- The ASR model dialog presents a single-select model list, source/size/license metadata, a required license checkbox, and download/cancel actions.
- Loading remains section-owned; profile, route, generation, and queue operations do not block unrelated controls.
- All touched copy uses valid UTF-8 Chinese/English source terminology. No membership renewal notice appears on AI pages.
- Tailwind utility classes only; no new scoped CSS or hidden class constants.

## Type And Contract Ownership

- Create a shared frontend AI contract module and merge its maps into `CommandRequestMap`, `CommandResultMap`, and `EventPayloadMap` independently of member contracts.
- Branded IDs distinguish model profile, generation, queue item, user, and device identifiers.
- Rust AI DTO/domain/application types are compiled in both feature variants and accessed through `AppState.ai()`.
- Member DTOs retain login/order/module/moderation only. Temporary re-exports are allowed only if tests prove compatibility and carry a cleanup trigger.
- Governing TypeScript config: `packages/bilibili-live-tool/tsconfig.json`; relevant declarations: `src/vue-shims.d.ts` and desktop contract modules.

## State Ownership

- Rust persists configuration and owns authoritative runtime/generation/queue state.
- Pinia AI store owns the latest adapted snapshot and pending-operation keys.
- Vue components own only draft/form/dialog/focus state. Readiness and queue status are derived, not copied into writable component state.
- Member store does not own or derive AI status.

## Async And Failure Contract

- Store operations deduplicate by semantic operation key and ignore results after disposal/revision change.
- Streaming deltas must be contiguous; mismatched generation or sequence is ignored and a terminal snapshot remains authoritative.
- Profile save/test, activation, route test, generation, and queue commands use pessimistic success updates.
- Errors are normalized once at transport/store boundaries and retain retryability without secret payloads.

## Pattern-Fit Decision

- Problem: one capability crosses build variants, adapts external model/audio APIs, and coordinates event-driven state.
- Depth: Level 2, reuse/formalize existing project patterns.
- Decision: reuse the typed desktop contract + runtime adapter + Pinia store + Rust state/port pattern already used by live-data/media; extract AI from member ownership.
- Syntax: TypeScript interfaces/discriminated unions, pure adapter functions, Pinia actions, Vue SFC boundaries, Rust traits/state structs/commands.
- Adapter signal: OpenAI-compatible transport and audio output remain behind existing Rust trait-style boundaries. A new factory/manager hierarchy is rejected as heavier than the current single protocol and state owner.
- Observer signal: existing Tauri events are reused with one lifecycle owner; a second event bus is rejected.
- Command signal: typed invoke maps remain the command surface; bespoke command objects are rejected.

## Component Extraction Decisions

- Reuse `AiPersonaConfigDialog` and update `AsrModelDownloadDialog` for the selectable Rust-owned catalog. `AiContextList` is no longer rendered by this page.
- Extract `AiModelProfilesDialog`: one caller, cohesive profile-management lifecycle and focused test surface; explicit profile props and semantic emits, no raw DTO passthrough.
- Extract `AiVoiceReplyControl`: one caller, cohesive runtime control and focused state/interaction tests; store remains parent owner.
- Keep the read-only latest-result section inline because extraction would be pass-through orchestration.

## Production Code Quality Constraints

- Define contract types before implementation; no `any`, secret-bearing read DTO, or broad optional mega-shape.
- Pure functions own normalization, validation, adapter mapping, readiness derivation, and queue eviction.
- Side effects remain in Rust ports/commands, Pinia actions, and lifecycle handlers.
- Use guard clauses for missing prerequisites and invalid ranges; do not default invalid required data.
- No memoization, debounce, or virtualization without measured need; queue/profile sizes are explicitly bounded.
- Boolean names use `is/has/can/should`; handlers use `handle*`; data/update functions use precise verbs.

## Architecture Reuse Signals

- Existing member adapters contain AI and member normalization mixed together: architecture design must decide extraction into AI ownership and migrate all in-scope callers.
- Existing TTS is semantically different (reads incoming events) and must remain separate; reuse infrastructure mechanism, not business state/config.
- Existing `MemberRepository` mixes AI persistence with membership. Architecture design must assign a shared AI repository port without duplicating SQL rules.
- Existing event sink mixes AI/member/moderation. Architecture design must split or provide a narrow shared AI event sink.

## TDD And Verification Contract

- Red tests first for free-route/command availability, membership-independent generation, removal of manual controls, selectable ASR catalog/download, model readiness/activation, normalization, device-loss behavior, and member-runtime listener cleanup.
- Focused TypeScript tests cover contracts/adapters/store/components/build channels.
- Focused Rust tests cover AI state rules, repository persistence/migration, IPC registration/validation, and both feature sets.
- Run type-check, scoped Vitest, Cargo tests for `free` and `member`, and both web builds.
- Browser screenshots must verify desktop/mobile layout and no overlap after UI implementation.

## Acceptance Criteria

1. Free and member builds expose AI navigation, routes, shared runtime, and shared commands.
2. Member expiry or logged-out state does not hide/disable AI or stop its runtime.
3. Model configuration is explicit, secret-safe, validated, and supports the source-defined provider categories/default/test flow.
4. Master switch cannot enable before model/device prerequisites and confirmation.
5. Generated successful text satisfies format/length constraints and display/TTS text equality.
6. Queue pause/skip/clear/device-loss behaviors are explicit and independent of generic TTS.
7. ASR failure does not disable text generation/TTS.
8. No outgoing Bilibili send UI, command, or task exists.
9. Both build variants compile and targeted tests/type checks pass.
10. The AI page contains no manual context/input/generate/regenerate/clear-result surface and still shows/copies the latest automatic result.
11. Enabling ASR without a model opens a selectable download dialog; a verified catalog model becomes installed, while unknown IDs and failed verification fail closed.
12. The model profile add/edit surface contains no proxy URL field, and every frontend profile save submits `proxyUrl: null`.
13. The model profile surface contains no temperature control or profile contract field; it displays a reasoning-effort selector with the approved values and defaults migrated profiles to `medium`.
14. Saving, loading, connection testing, and generation preserve the selected effort end to end, and the provider payload contains `reasoning_effort` without `temperature`.

