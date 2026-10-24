# Architecture Design: Shared AI Voice Reply

## Delivery Unit And Objective

`shared-ai-voice-reply`: establish one shared AI capability boundary used by both build channels while keeping membership/moderation isolated and preserving current persisted AI data.

## Scope, Inputs, And Assumptions

- Approved spec and page design are authoritative.
- Existing TypeScript contract/store/adapter and Rust state/port/IPC patterns are reused.
- The repository has no code graph setup; scoped import/caller evidence is recorded in `artifacts/code-context.md`.
- Existing SQLite AI rows must remain readable.
- Not greenfield; no scaffold/bootstrap changes.

## Expert Architecture Decision Records

### ADR-AI-01: Shared frontend capability boundary

- Problem: AI routes, runtime subscriptions, contracts, and adapters are currently owned by the member channel.
- Decision: create shared AI contracts/adapters/runtime ownership; register AI routes directly in the common router and navigation tree; remove AI from `channel.member.ts` and `useMemberRuntime`.
- Alternatives: duplicate AI into free channel, or make free channel import member runtime. Both retain wrong ownership and risk drift.
- Dependency after: page/store -> shared AI contracts/adapters -> desktop transport. Member channel may not import AI stores/pages.
- State owner: AI Pinia store; side-effect owner: `useAiRuntime` and store actions.
- Cleanup: remove AI members from member contract maps/capability routing and update boundary tests.
- Evidence: free/member web builds and runtime-listener tests.

### ADR-AI-02: Shared Rust state with compatibility-preserving persistence

- Problem: `AiState` and SQL methods live under the existing member source tree and are feature-gated.
- Decision: compile the existing AI application/persistence primitives for both builds and expose a dedicated `AppState.ai()` field/accessor initialized from the shared SQLite pool. Member `MemberStates` continues to own membership/moderation only at runtime. AI IPC calls `state.ai()` and never requests membership status.
- Smallest-sufficient rationale: moving every AI type/SQL mapper into new files in the same change would create a large mechanical migration across an already dirty worktree without changing runtime behavior. Dedicated state ownership and command registration remove the behavioral boundary immediately; source-tree renaming is not required by the PRD.
- Dependency after: IPC AI commands -> `AppState.ai()` -> `AiState` -> repository/provider ports. No AI -> MembershipState edge.
- Compatibility: existing `MemberRepository` AI methods remain temporarily because the SQLite implementation already owns the rows. Cleanup trigger is a later schema-level multi-profile migration that introduces a dedicated `AiRepository`; no dual runtime path is allowed now.
- Rollback: revert shared initialization/registration while preserving existing tables.
- Evidence: Rust tests under both `free` and `member`, plus an assertion that AI methods accept no entitlement parameter.

### ADR-AI-03: TTS mechanism reuse, state separation

- Problem: existing TTS runtime reads live events; PRD requires a separate AI queue.
- Decision: reuse speech-engine/media mechanisms only through an explicit AI-facing port; do not reuse generic TTS enable/config state. For the current increment, shared AI UI exposes route/test/queue states backed by dedicated AI contracts; unavailable device-specific routing remains an honest error rather than falling back to generic TTS.
- Alternatives: bind AI switch to `tts_set_enabled` or use generic TTS queue. Rejected because it creates two meanings for one state and violates independent queue semantics.
- Evidence: tests show generic TTS state is unchanged by AI enable/pause/clear operations.

### ADR-AI-04: Model profile evolution

- Problem: current database stores one AI config, while PRD supports multiple profiles.
- Decision: treat the existing single configuration as the migrated default profile and introduce typed profile collection at the shared AI contract/state boundary. Persistence extension remains in the narrow AI repository owner and must preserve the legacy row.
- Alternative: frontend-only profiles/local storage. Rejected because Rust must own secrets and requests.
- Evidence: migration/repository round-trip tests and no plaintext key in DTO/export.

## Business Capability And Ownership Map

| Capability | Owner | Forbidden owner |
| --- | --- | --- |
| Page orchestration | `views/ai/index.vue` | member channel |
| Model/profile and queue state | AI Pinia store / Rust `AiState` | membership store |
| DTO normalization | AI adapter module | templates/computed fallbacks |
| Transport | desktop transport + Rust IPC | Vue components |
| Provider/audio I/O | Rust injected ports | shared pure helpers |
| Membership entitlement | membership/moderation | AI state |
| Form drafts/dialog focus | extracted AI components | global store unless persisted |

## Module And File Structure

Frontend target:

```text
platform/desktop/contracts/ai.ts
modules/ai/adapters.ts
composables/useAiRuntime.ts
stores/ai.ts
views/ai/index.vue
views/ai/components/AiVoiceReplyControl.vue
views/ai/components/AiModelProfilesDialog.vue
```

Existing AI files may be adapted in place. Member files lose AI imports/listeners/routes/contracts. Rust keeps current physical AI primitives for migration safety but publishes them through `AppState.ai()` and shared command registration.

## Dependency Rule Matrix

| From | Allowed | Forbidden |
| --- | --- | --- |
| AI views/components | AI store, shared AI types, Element Plus | membership store/channel, raw Tauri API |
| AI store/runtime | shared AI adapters/contracts, desktop transport | member store/runtime |
| Member channel/runtime | membership/moderation | AI views/store/runtime |
| AI IPC | `AppState.ai()`, AI DTO mappers | `MembershipState::status` |
| AI state | AI repository/provider/event ports | UI/member entitlement |
| Pure normalizers | immutable strings/config values | network, store, persistence, router |

## State Topology And Data Lifecycle

- Persisted config/profile -> Rust snapshot DTO -> desktop transport -> adapter -> immutable Pinia snapshot -> derived readiness -> component props.
- Component draft -> trimmed/validated request -> Pinia action -> Tauri command -> Rust validation/persistence -> authoritative snapshot response.
- Generation request -> Rust provider -> pure normalization -> event/result -> adapter -> store; deltas are accepted only for current ID/next sequence.
- Writable owners: Rust for persisted/runtime truth; Pinia for current client snapshot; dialog components for unsaved drafts.
- Derived only: `hasDefaultModel`, `hasAudioRoute`, `canEnable`, queue action availability.

## Extension Seams And Rejected Future-Proofing

- Real seam: `LlmProvider` for user-selected OpenAI-compatible endpoint and `AsrProvider` for optional whisper component.
- Real seam: AI audio output port for device enumeration/playback.
- Rejected: provider plugin registry, provider-specific class hierarchy, model factory, AI manager, second event bus, automatic OBS controller.
- Add a provider-specific adapter only when a source-backed non-compatible protocol exists.

## Complexity Budget

- Frontend: at most five new AI-owned files (contract, adapter, runtime, two cohesive components), plus scoped edits/tests.
- Rust: prefer scoped state/IPC/build registration changes and one migration/adapter boundary; avoid parallel old/new runtime paths.
- Public command additions limited to PRD actions; no generic command registry.
- One AI event subscription owner; no watchers for synchronization.
- No repository-wide format or unrelated member refactor.

## Migration, Compatibility, And Cleanup

- Legacy single config becomes initial default profile; preserve persona/rule values.
- AI command names remain stable where behavior matches; add new names only for activation/profile/queue actions.
- Member capability payload may retain `ai` temporarily for wire compatibility, but navigation/access ignores it. Cleanup trigger: all consumers migrated to shared AI readiness.
- No dual event runtime. AI listeners are removed from member runtime before shared runtime starts.
- Stale tests asserting free builds exclude AI must be replaced with assertions that only membership/moderation remain excluded.

## Locality And Component Decisions

- Pure reply normalization belongs in the Rust AI domain owner because automatic and manual generation share it and it needs independent tests.
- Frontend readiness derivation stays in the AI store/computed owner; do not create a one-function helper file.
- `AiModelProfilesDialog` and `AiVoiceReplyControl` are single-caller SFC exceptions justified by cohesive business lifecycle and focused tests.
- Manual generation helpers remain in the page because they are page-private orchestration.

## Reuse Candidate And Anti-DRY Matrix

| Candidate | Locations / semantics | Decision |
| --- | --- | --- |
| AI/member runtime listener orchestration | one generic lifecycle shape, different event owners | Reuse `create*Runtime` shape, keep separate runtimes because lifecycle and failure domains differ. |
| Member/AI DTO adapters | currently one file, different capability owners | Extract AI adapters; do not create generic adapter utilities. |
| Generic TTS vs AI voice queue | similar playback verbs, different trigger/config/queue semantics | Reuse speech mechanism through a port; keep state/rules separate. |
| Text filter/truncate | one AI domain rule used by manual/automatic generation | Keep one pure AI-domain normalizer and extend it; no duplicate frontend normalization. |
| Async Pinia operation dedupe | repeated local shape across stores | Keep local; fewer than three stable same-semantics callers in scoped change and extraction would broaden ownership. |

## Commonality Classification And Shared Decisions

- Desktop contracts/adapters: technical contract commonality, narrow AI owner.
- Reply normalization and readiness: AI domain commonality, AI owner.
- Element Plus controls: UI/design-system reuse.
- PRD numeric limits: configuration/domain constants under AI owner.

## Functional And Side-Effect Boundaries

- Pure: DTO adaptation, validation predicates, readiness derivation, output normalization, queue priority/eviction.
- Effects: invoke/listen in transport/store/runtime; persistence/provider/audio in Rust ports/state; UI messages/navigation in page handlers.
- Inputs/DTOs remain immutable; store updates replace snapshots.
- No functional utility library is introduced.

## Pattern Decision

- Level 2, reuse/formalize project pattern: typed Adapter boundary + Pinia state owner + injected Rust ports + Tauri observer events.
- This is the lightest structure that removes entitlement coupling while preserving testable external boundaries.
- No new factory/manager/registry. If only one OpenAI-compatible provider shape remains, additional pattern layers are overbuilt.

## Readability And Maintenance Guardrails

- Member names must not appear in shared AI navigation/readiness logic.
- No API key in display snapshots, debug output, or test fixtures using realistic secrets.
- Keep command validation near IPC/domain boundary and user-visible state mapping in adapters.
- Every changed old AI caller must point to shared ownership or be removed.

## Architecture Risks

- Temporary physical placement of Rust AI code under `member` is misleading; imports must use the dedicated `AppState.ai()` boundary and the cleanup trigger is recorded.
- Device-specific output is not supported by the current speech engine. Commands must return an explicit unavailable state until a real device adapter exists; never silently claim route confirmation.
- The current provider is unavailable. Connection/generation tests use fakes; production failure remains explicit until a configured HTTP provider is wired.

## Open Architecture Questions

None blocking. External provider/audio implementations are variation points already represented by ports and explicit failure states.

## 2026-10-23 ASR Recovery Amendment

- Remove `AiContextList`, prompt draft, selected-context state, and manual generate/regenerate/clear handlers from `views/ai/index.vue`. The page continues to render the latest automatic generation and copy action from the existing store state.
- Replace the nullable page-owned `manifest` placeholder with an authoritative Rust-owned ASR model catalog. Add `asr_model_list`; change download input to an allowlisted `modelId` plus license acceptance. The frontend must never provide a download URL or filesystem path.
- Extend the ASR provider boundary with catalog and model-download operations. The infrastructure implementation owns approved model metadata, destination directory, partial-file cleanup, HTTP status/size validation, and atomic rename.
- `AsrModelDownloadDialog` owns only selected model ID and license-checkbox draft state. It receives typed manifests and status from the AI store and emits a semantic download request.
- The AI store owns catalog loading and the current ASR status. Bootstrap loads status and catalog together; a successful download updates status to `ready` and closes the dialog.
- Starting ASR in `modelMissing` opens the dialog. Starting in `unavailable` first loads the catalog and opens the dialog when downloadable models exist; engine/sidecar absence after a successful model download remains a distinct explicit error and must not be mislabeled as a missing model.
- Reuse the existing typed command/adapter/Pinia/component boundaries. No downloader manager, frontend URL map, second event bus, or new state library is introduced.
- Tests must prove the manual editor is absent, selectable models render, request payloads contain no URL/path, unknown model IDs fail, and a successful model download reaches `ready` without writing directly to the final path.

## 2026-10-23 Reasoning Effort Contract Amendment

- Replace `temperature` with one shared closed enum across the TypeScript contract, adapter, Vue draft, Rust DTO, application record, repository validation, and provider payload. TypeScript owns the UI-facing union; Rust owns transport/persistence validation.
- Add migration `0005` with `reasoning_effort TEXT NOT NULL DEFAULT 'medium'` and a closed-value check. Existing profile rows therefore migrate deterministically to `medium`.
- Retain the legacy SQLite `temperature` column because migration `0004` made it non-null. New inserts write the legacy constant `0.7`, updates stop changing it, and no runtime DTO/domain/provider code reads it. This compatibility column has a cleanup trigger only when the profile table is rebuilt in a future schema migration.
- The OpenAI-compatible provider sends `reasoning_effort` and omits `temperature`. Provider/model incompatibility remains an explicit existing provider error; no fallback value or retry mutation is introduced.
- Reuse the existing typed adapter, DTO mapping, repository, and provider boundaries. A strategy/factory layer would add no value for one closed scalar setting.

