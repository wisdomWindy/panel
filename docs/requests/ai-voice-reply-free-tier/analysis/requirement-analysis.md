# Requirement Analysis

## Source Inventory And Reading Stance

| Source | Authority | Use |
| --- | --- | --- |
| User instruction dated 2026-10-23 | Primary trigger | Authorizes development against the modified PRD. |
| Working-tree PRD, especially 9.14 and 11 | Primary product source | Defines AI ownership, fields, interactions, state rules, boundaries, and acceptance criteria. |
| PRD working-tree diff | Scope detector | Identifies the changed capability ownership and new AI voice reply details. |
| Existing Vue/Tauri code and tests | Technical constraint only | Establishes the current member coupling and required dependency closure; does not add product behavior. |

The source is sufficiently detailed for downstream automatic approval. Repository facts constrain migration and compatibility but do not authorize adjacent redesign.

## Source Grounding Matrix

| Item | Classification | Source / consequence |
| --- | --- | --- |
| AI available in free and member builds | source-backed | PRD 9.14.10.15 |
| Membership expiry does not affect AI | source-backed | PRD 9.14.10.16 |
| External/local OpenAI-compatible endpoints | source-backed | PRD 9.14.3 |
| Multi-profile/default/test/delete model management | source-backed | PRD 9.14.3 and 9.14.7 |
| Single master switch with first-enable route confirmation | source-backed | PRD 9.14.2, 9.14.7, 9.14.8 |
| Short conversational plain-text normalization | source-backed | PRD 9.14.4 |
| Dedicated output-device selection and queue controls | source-backed | PRD 9.14.6-9.14.8 |
| Shared AI route/runtime/IPC extraction | code-fact-backed | Current AI chain is behind `@member-channel` and `#[cfg(feature = "member")]`; extraction preserves the requested availability. |
| Reuse existing SQLite AI tables while migrating ownership | code-fact-backed | Tables are already installed in the shared database and contain AI configuration/log data. |
| Real provider/OBS success requires deployment credentials/devices | external-interface-pending | Product behavior and failure rules are stable; adapters and explicit unavailable states permit implementation. |
| Bilibili danmaku send support | out-of-scope | Explicitly forbidden by PRD 0.11 and 9.14.10.14. |
| Automatic OBS configuration or meter API | out-of-scope | PRD requires user confirmation and observable test playback, not OBS control. |

## User Intent Contract

- Stated request: develop according to the modified functional modules.
- Practical goal: make the revised AI voice reply module genuinely usable as a free shared feature instead of merely relabeling the existing member-only UI.
- Success criteria: both builds contain the complete route/runtime/IPC chain; membership state is absent from AI eligibility; the new model, activation, routing, normalization, and queue requirements are represented and tested.
- Forbidden interpretations: changing only labels; exposing the page while commands remain member-only; returning mock success from an unavailable provider; reusing generic danmaku TTS enablement as a second AI switch; adding any outgoing Bilibili message path.
- Acceptable approaches: extract AI into shared modules, reuse stable persistence/media primitives, and retain explicit unavailable errors for external dependencies.

## Confirmed Scope

1. Re-own AI route, store startup, TypeScript contracts, Rust state, persistence access, events, IPC commands, and permissions as shared capability.
2. Add model-profile configuration and default-profile state without bundling model weights.
3. Add AI voice reply settings, deliberate activation confirmation, audio-device selection/test, and independent reply queue controls.
4. Apply chat-style normalization and hard limits before result completion/TTS enqueueing.
5. Update free/member boundary and regression tests.

## Out Of Scope / Non-Goals

- Membership purchase/login redesign.
- Automatic moderation behavior beyond removing AI from its capability map.
- Provider-specific SDKs or non-OpenAI-compatible protocols.
- Shipping LLM or whisper weights.
- Automating OBS/Bilibili Live Studio configuration.
- Sending generated text to Bilibili.
- Unrelated visual redesign of the application shell.

## Confirmed Decisions / Invariants

- AI eligibility depends on local configuration and runtime health, not member entitlement.
- Moderation remains the only paid runtime capability.
- AI and generic danmaku TTS remain separate queues and settings.
- API keys cross the frontend boundary only as replacement input and configured/masked status; plaintext values are never returned.
- External failures remain explicit and retryable, never reported as successful generation or route verification.
- Missing ASR assets cannot disable text generation or TTS.

## System Boundaries / Ownership

| Owner | Responsibilities |
| --- | --- |
| Vue page/components | Forms, tables, activation confirmation, disabled/loading/error/empty states, queue controls, copy/regenerate interactions. |
| Pinia AI store/runtime | Async deduplication, event ordering, AI configuration/generation/queue state, bootstrap and disposal independent of member runtime. |
| Desktop contract/adapters | Stable request/result/event types, runtime validation, separation from member contracts. |
| Rust AI application state | Model-profile/rule validation, configuration persistence, generation lifecycle, normalization, cooldown/queue state, ASR isolation. |
| Rust infrastructure | SQLite persistence, secret storage boundary, OpenAI-compatible HTTP adapter, audio-device/playback adapter. |
| Member subsystem | Login, payment, module delivery, entitlement, and moderation only. |
| External provider/OBS | Provider responses and audio capture; treated as external interfaces with observable failures. |

## Frontend / Service Responsibility Split

### Frontend-owned

- Present all PRD fields and actions.
- Validate immediate form shape/ranges and render backend validation errors.
- Keep streaming deltas ordered and ignore stale generations.
- Require explicit checkbox confirmation before first enable.
- Never expose API-key plaintext after save.

### Rust-owned

- Validate Base URL, profile identity, model parameters, rule ranges, output device ID, and activation prerequisites.
- Persist profiles/rules and secret material at the desktop boundary.
- Execute OpenAI-compatible requests and normalize outputs.
- Own cooldowns, queue limits, device-loss behavior, and generation/TTS side effects.
- Register shared commands in both build variants.

### Shared contracts

- Profile/config/queue DTOs, enums, command names, event payloads, stable error codes.
- `ai_generation_delta` ordering and terminal generation snapshot.
- API key semantics: replacement input, boolean configured output, masked display only.

### External-interface pending, non-blocking

- Real endpoint authentication/model availability: handled by test-connection/generation errors.
- Physical/virtual audio device and OBS capture: handled by enumeration/test playback plus user confirmation.
- Whisper download manifest: existing controlled unavailable response remains valid until a signed manifest source is configured; text generation/TTS remain usable.

## Source-Backed Behavior Inventory

- Model profile list/editor with provider category, profile name, Base URL, API key, model, temperature, token limit, timeout, proxy, enabled/default, status, test/edit/delete.
- AI Assistant runtime surface with one master switch, route guide, current model, trigger rules, output device/test, queue status and controls, context/manual generation and result controls.
- Persona settings with identity, style tier, maximum length/sentences, fixed format/self-reference constraints, and system prompt.
- Failure states for unconfigured model, default deletion, endpoint/LLM errors, rate limit, malformed/overlong output, device loss, queue overflow, ASR asset absence, and microphone error.

## Developer-Facing Behavior Details

- Disabled activation must clearly point to missing default model or missing audio route.
- The first-enable confirmation records route acknowledgement; later enable operations may reuse it until the selected device changes or disappears.
- Deleting the default profile disables AI reply and pauses the queue.
- A terminal generation may only be `completed` after normalization succeeds; rejected/failed output is never enqueued.
- Queue overflow eviction is deterministic and favors the PRD priority order.
- Member expiry events must not mutate AI store state.

## Hidden Assumptions Exposure

| Assumption | Classification | Handling |
| --- | --- | --- |
| Shared DB migrations run in free builds | code-fact-backed, confirmed by `SqliteDatabase` bootstrap | Reuse schema; add migration only if profile multiplicity requires it. |
| Current single-row AI config cannot represent multiple profiles | code-fact-backed | Architecture/spec must define a migration or compatible profile table. |
| Generic TTS state does not expose output device enumeration/AI queue | code-fact-backed | Add an explicit AI audio boundary; do not overload the danmaku TTS switch. |
| No repository-approved graph tool exists | non-blocking | Scoped caller/import analysis is recorded in `artifacts/code-context.md`. |

## Workflow / Dependency Assessment

The frontend, IPC, application state, repository, and build command registration must change as one delivery unit. Splitting by layer would create invalid intermediate package variants. Page design and architecture design are both required before implementation.

## Ambiguity / Clarification Register

| Item | Classification | Resolution |
| --- | --- | --- |
| Exact production provider credentials | external-interface-pending | Configurable at runtime; not needed for repository implementation. |
| Exact OBS meter detection API | non-blocking | PRD permits explicit user confirmation after test playback; no automatic detection contract is required. |
| Secret-store implementation availability | non-blocking technical risk | Preserve no-plaintext-return contract; implementation may reject replacement until a repository-owned encryption adapter exists, but must not report it saved. |

No unresolved item changes product scope, data semantics, permissions, or state flow, so downstream automatic approval is permitted.

## Risk And Constraint Assessment

| Risk | Trigger | Affected surface | Required handling |
| --- | --- | --- | --- |
| Free build compiles without member modules | moving route only | Free runtime/IPC | Extract or compile AI dependencies independently; test both feature sets. |
| Double event listeners | starting shared AI and member runtime | Streaming/ASR state | Remove AI listeners/bootstrap from member runtime before starting shared runtime. |
| Member DTO drift | partial contract extraction | Frontend invoke typing | Create one shared AI contract source and update contract tests. |
| Secret leakage | profile save/export/log | Security/config | Replacement-only input, redacted output/export, no debug logging. |
| TTS coupling | reusing generic TTS enable switch | Existing TTS feature | Separate AI route/queue state and regression test generic TTS. |
| Migration data loss | replacing single config | Existing users | Migration preserves current config as a default profile/persona baseline. |

## Blocker Classification And Handling Stance

- Business blockers: none.
- External-interface pending: provider credentials, real models, audio devices, and OBS capture; all have stable adapter/failure boundaries.
- Non-blocking: code graph unavailable; scoped call-chain reconstruction provides sufficient evidence.

## Confirmation-Reduction Notes

- Downstream stages must not reopen whether AI is free, whether expiry affects it, whether one master switch is required, or whether outgoing danmaku is forbidden.
- External endpoint and audio capture success must be verified as integration/deployment evidence, not simulated in repository tests.
- Frontend and Rust ownership is explicit; neither layer may silently absorb the other's validation or persistence responsibilities.
- Model, activation, normalization, and queue rules must become executable acceptance tests.

## Splitting Strategy

Use one split module: `shared-ai-voice-reply`. It stays whole because build-channel ownership, route/runtime availability, IPC registration, persisted configuration, generation state, and TTS queue must remain consistent across both installers. Membership/moderation cleanup is part of this module's dependency closure, not a second product module.

## Downstream Stage Signals

- Page design required: AI Assistant information architecture, first-enable confirmation, model table/editor, queue controls, and responsive/error states materially change.
- Architecture design required: capability ownership, type boundaries, persistence migration, runtime/event ownership, and side-effect orchestration materially change.
- Sequence: page design -> spec -> architecture design -> plan -> execute -> verify -> review.

