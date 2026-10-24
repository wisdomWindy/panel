# Module: Shared AI Voice Reply

## Module Boundary

Own the complete AI voice reply capability shared by both installers: navigation, page/runtime state, model profiles, persona/rules, generation, normalization, local display, dedicated TTS route/queue, ASR optionality, persistence, IPC, and removal of member entitlement coupling.

Member login, purchase, module delivery, and automatic moderation remain outside this module except for dependency-closure edits that remove AI from their capability/runtime ownership.

## Source Snapshot And References

- `source-backed` PRD 9.14.10.15: both installers include AI voice reply, model configuration, and TTS routing commands.
- `source-backed` PRD 9.14.10.16: membership expiry does not affect AI.
- `source-backed` PRD 9.14.1-9.14.10: full behavior, UI, state, failure, and acceptance contract.
- `source-backed` PRD 10.1-10.4: Rust-originated requests, validated device IDs, and shared command registration.
- `source-backed` PRD 11: revision invariants and paid/free boundary.
- `code-fact-backed` `artifacts/code-context.md`: current AI code is coupled to member route/runtime/state/IPC ownership and requires a closed-chain migration.

## Business Goal

Provide free, local-first AI voice replies that use a user-configured model, generate short safe spoken responses, display them in the tool, and play the same text to an explicitly confirmed local audio route without any Bilibili send capability.

## User-Visible Behaviors

- AI Assistant is visible and accessible in free and member builds regardless of member status.
- Model profiles can be added, edited, tested, enabled/disabled, selected as default, and deleted.
- The page shows current model status, voice reply rules, output device, test playback, and queue state.
- The single master switch is disabled until a default model and output route exist; first enable requires route confirmation.
- Users can pause/resume playback, skip current, clear the queue, and copy the latest automatic reply result. Manual prompt/generate/regenerate controls are removed.
- Starting ASR without a ready model opens a model-selection download dialog and exposes download progress/errors.
- Errors identify missing model, provider failure/rate limit, invalid output, device loss, queue overflow, and ASR-only degradation.

## Forms, Tables, Displays, And Interactions

### Model profile editor

Fields: provider category, profile name, Base URL, API key replacement, model name, temperature, maximum tokens, timeout, optional proxy, enabled, default.

Table/display: name, provider, model, test status, default state; actions test, edit, delete, select default, add/import.

### AI Assistant runtime

Fields/controls: master switch, current model, trigger scope, global/per-user cooldown, queue limit, style tier, maximum length/sentences, sensitive filter, deduplication, priority, output device, volume, test playback, route guide.

Displays/actions: current playback, pending queue, pause/resume, skip, clear, latest automatic result, copy, logs, and optional ASR model selection/download.

### First-enable confirmation

Explain local display/TTS/OBS capture behavior, show selected device, provide test playback, require capture-confirmation checkbox, then allow confirm enable.

## Workflow And State Rules

1. No configured default model -> AI unconfigured; switch disabled.
2. Default model configured and device selected -> ready to test, still disabled by default.
3. Successful/observable test is user-confirmed -> first enable permitted.
4. Enabled -> eligible filtered events enter cooldown/priority/queue evaluation.
5. Generation streams, normalizes, completes, displays, and enqueues identical text for TTS.
6. Disable pauses new generation and playback; queue controls remain available as defined by runtime state.
7. Default profile deletion or device loss disables/pauses the capability without membership interaction.

## Frontend Responsibilities

- Typed forms, table/actions, runtime controls, confirmation, loading/disabled/error/empty state presentation.
- Shared AI runtime event subscription independent of member runtime.
- Safe adapters for IPC snapshots and ordered streaming events.

## Rust Responsibilities

- Persist profiles/persona/rules/activation route state with secret redaction.
- Validate profiles/ranges/device IDs/activation prerequisites.
- Execute OpenAI-compatible requests through an adapter boundary.
- Normalize output and own queue/cooldown/TTS side effects.
- Register shared commands/events in both build variants.

## Shared Contracts And External Interfaces

- Shared DTOs/enums/errors for profiles, settings, queue, devices, generation, and ASR.
- External-interface pending: live provider credentials/models, audio devices, OBS capture, signed whisper manifests. These do not block repository delivery and must fail explicitly.

## Closed-World Scope Allowlist

- Shared AI route/runtime/commands.
- Model profile management defined by PRD.
- Persona and reply-rule settings defined by PRD.
- Single master switch and first-enable route confirmation.
- Dedicated output-device/test and queue controls.
- Text normalization, cooldown/queue failure handling, ASR independence.
- Necessary member-boundary, migration, config-redaction, permission, and test cleanup.

No Bilibili send action, proprietary provider protocol, OBS automation, or unrelated shell redesign is authorized.

## Upstream Constraints Spec Must Preserve

- AI is free and membership-independent.
- No bundled model weights.
- No plaintext API key output/log/export.
- No outgoing danmaku UI or command.
- One switch controls generation and AI TTS together.
- Displayed and spoken successful text is identical.
- Device loss does not silently fall back.
- Missing whisper/ASR assets do not disable text/TTS.

## Routing And Order

- Page design: required.
- Architecture design: required after approved spec.
- Execution order: first and only module.

## Open Questions

None blocking; see request-level external-interface notes.

