# Architecture Design: AI Voice Tone Selection

## Delivery Unit Identifier

`ai-voice-tone-selection`

## Architecture Objective and Triggers

Carry one stable voice ID through an existing vertical feature chain while preserving the current UI/store/IPC/application/repository/platform ownership. The trigger is cross-layer type and data-lifecycle change, not a need for new module boundaries.

## Upstream Inputs and Assumptions

- Framework-approved `spec/spec.md` is the behavior contract.
- `design/page-design.md` fixes the selector location and interaction order.
- `artifacts/code-context.md` records the bounded call chain and graph fallback.
- Scaffold alignment: not applicable.

## Expert Architecture Decision Records

### ADR-1: Stable Built-In Preset ID

- Problem: `温柔桃子` needs a durable default across varying host voice catalogs.
- Decision: persist `gentle-peach`; resolve it in the platform adapter to preferred/default host speech.
- Alternative rejected: persist a system display name, which is localized and unstable.
- Owner: application contract defines the constant; platform adapter resolves it.
- Verification: migration/repository round trip and adapter/application tests.

### ADR-2: Extend Existing Speech Strategy Port

- Problem: voice enumeration and selection vary by OS.
- Decision: add `voices()` and a `voice_id` argument to `AiSpeechOutput`.
- Alternatives rejected: frontend-owned catalog, a new factory/registry, or direct OS calls from application state.
- Dependency direction: application depends on the port; infrastructure implements it.
- Verification: compile all implementations/test doubles and exercise selected ID propagation.

### ADR-3: Snapshot-Owned Catalog

- Problem: the dialog needs catalog and persisted selection atomically.
- Decision: return voices with `AiVoiceReplySnapshot`, matching existing device ownership.
- Alternative rejected: a new command/store loading state for a small catalog, which adds race and pending-state complexity.
- Owner: IPC snapshot assembler.

## Business Capability and Ownership Map

| Capability | Owner |
| --- | --- |
| Draft selection | `AiVoiceReplyControl.vue` |
| Async command/snapshot | `stores/ai.ts` |
| DTO normalization | `modules/ai/adapters.ts` |
| Contract serialization | `ipc/member.rs` |
| Validation and state transition | `application/member/state.rs` |
| Stable persisted ID | `SqliteMemberRepository` / migration |
| Host catalog and token resolution | `SystemAiSpeechOutput` |

## Module and File Boundary Design

No new frontend module or component. Add the migration and extend existing files in their current owners. The only new public data structure is a voice catalog record/interface adjacent to output-device types.

## Code Relationship and Dependency Direction

`Vue -> store -> desktop contract -> IPC DTO -> application port/state -> repository + infrastructure adapter`. Infrastructure must not leak Windows token types upward. Frontend must not know the `gentle-peach` resolution algorithm.

## State Topology and Data Lifecycle

- Writable server state: `t_ai_voice_config.voice_id` through repository.
- Writable UI draft: `route.voiceId` while dialog is open.
- Derived state: selected display label and option availability from immutable snapshot.
- Async owner: existing Pinia operation map; no new parallel request.
- Lifecycle: DB row -> Rust record -> queue/snapshot -> DTO -> adapter -> store -> dialog; submit returns through the same path.

## Dependency Rule Matrix

| Layer | May depend on | Must not depend on |
| --- | --- | --- |
| Vue component | frontend contract props/events | IPC transport, OS APIs |
| Store | desktop contract and adapter | Rust/storage details |
| IPC | application records/state | Vue/UI concepts |
| Application | ports/domain records | SAPI/`tts` concrete APIs |
| Repository/platform | application ports | frontend types |

## Extension Seams and Rejected Future-Proofing

The only real seam is `AiSpeechOutput`. Reject provider registries, plugin catalogs, cloud voice metadata, separate voice stores, and generic selection frameworks until a real second provider exists.

## Complexity Budget

- New files: one SQL migration plus workflow artifacts; no new production TS/Rust module.
- Public APIs: one record/interface and one port method/argument expansion.
- Reactive fan-out: one additional draft field and snapshot array.
- Migration scope: one additive non-null column.

## Migration, Compatibility, and Cleanup Design

Migration assigns `gentle-peach` to all existing rows. No dual schema path or feature flag is retained. Rollback requires reverting code and database snapshot; SQLite down migration is not introduced by current project convention.

## Locality and File-Extraction Decisions

Voice mapping functions stay in `infrastructure/member/ai.rs`, their single concrete owner. UI field stays in `AiVoiceReplyControl.vue`. No new helper/SFC file is justified.

## Reuse Candidate Inventory and Anti-DRY Matrix

| Candidate | Semantics/layer | Production uses | Stability/variation | Decision |
| --- | --- | --- | --- | --- |
| General TTS `SystemSpeechEngine::voices` | Similar mechanism, different media domain and no device routing | 1 per subsystem | OS adapter details differ | Keep separate; extracting would reverse/couple domain dependencies |
| Existing AI output-device DTO flow | Same AI route snapshot semantics | Existing chain | Stable, one new sibling field | Reuse pattern in place |
| Preferred HuiHui selection helper | General TTS frontend vs AI Rust platform | 2, cross language/layer | Same preference, different runtime catalogs | Keep separate; no shared executable owner exists |

Commonality classification: platform voice access is infrastructure; `gentle-peach` is the narrow AI voice configuration constant; selector markup is feature UI.

## Function Design and Public Entrypoints

- `AiSpeechOutput::voices() -> Vec<AiVoiceRecord>`.
- `AiSpeechOutput::speak(text, device_id, voice_id, volume)`.
- `AiState::set_output_device(device_id, voice_id, volume, confirmed)` validates both catalogs and persists atomically.
- `AiState::test_output_device(device_id, voice_id, volume)` validates through the speech adapter.
- Private Windows helpers enumerate/select SAPI tokens; non-Windows implementation uses `tts` voices.

## Functional Boundaries and Side Effects

Catalog mapping and preferred-token matching are deterministic transformations. SQLite updates, desktop invokes, and playback remain explicit side effects. Inputs from snapshots/DTOs remain immutable on the frontend.

## Data Structures and Type Strategy

- TypeScript uses branded `AiVoiceId` and readonly interfaces.
- Rust uses owned strings at IPC/storage boundaries and borrowed `&str` in playback calls.
- Persisted `voice_id` and queue `selected_voice_id` are non-optional after migration.
- The default ID/display constants live in the narrow application port owner and are reused by infrastructure/tests.

## Contract and Adapter Boundaries

Rust serde remains authoritative for wire fields; TypeScript mirrors camelCase. `adaptAiVoiceReplySnapshot` validates catalog arrays and non-empty selected voice values before UI consumption.

## Pattern Decision

Reuse the existing Strategy/Port pattern at Level 2. No new pattern layer is introduced. The port already isolates the only material variation axis, so a factory, facade, manager, or registry would add indirection without a second lifecycle or ownership problem.

## Readability and Maintenance Guardrails

Keep SQL column order aligned across select/map/update, avoid unrelated formatting, update every port implementation/test double, and re-run reference search for stale payload/signature shapes.

## Architecture Risks

- Windows COM voice APIs are compile-sensitive.
- System voice IDs can disappear; a saved concrete ID must fail explicitly while `gentle-peach` retains fallback semantics.
- The working tree is heavily modified, so changed-hunk review must distinguish this request from existing user work.

## Open Architecture Questions

None.

