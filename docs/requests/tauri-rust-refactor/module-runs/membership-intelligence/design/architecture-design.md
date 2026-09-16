# Membership Intelligence Architecture Design

## Delivery Unit Identifier

- request: `tauri-rust-refactor`
- module: `membership-intelligence`
- approved spec: `module-runs/membership-intelligence/spec/spec.md`
- stage result: ready for `plan`

## Architecture Objective

Add a member-only vertical feature slice that keeps entitlement and side effects native, keeps Vue focused on normalized view state, excludes member capabilities from the free binary/bundle, consumes only accepted live events, and exposes honest unavailable states for missing production integrations. The design must replace legacy browser/localStorage owners without changing the approved product contract.

## Architecture Scope And Triggers

Architecture is required because this unit adds build-time capability selection, 28 native operations, long-running login/download/ASR/generation tasks, shared shell status, accepted-event fan-out, repositories, three route families, and removal of legacy stores/services. State ownership, dependency direction, cancellation, and external adapter isolation cannot safely be decided inside page code.

## Upstream Inputs And Assumptions

- `spec/spec.md` and `spec/clarifications.md` are immutable behavior boundaries.
- `design/page-design.md` fixes page hierarchy/component boundaries.
- The external membership/payment/key/manifest, moderation transport, LLM, and Whisper artifacts are unavailable in production; injectable test ports do not create production success.
- QR endpoints and status semantics are sourced by PRD; network failure remains recoverable through UID fallback.
- Scoped `rg` caller/import scans substitute for a Vue-aware code graph.

## Scaffold Alignment And Bootstrap Boundaries

The request-level Tauri v2 backend attached to the existing Vue/Vite package is already approved and implemented. Keep the existing `src-tauri/{domain,application,infrastructure,ipc}` layering, Pinia setup stores, typed desktop gateway, router, Vitest, SQLx migrations, and free/member Cargo/Vite scripts. Do not create another package, app entry, router, state library, request client, or CSS system.

## Expert Architecture Decision Records

### ADR-MI-01: Compile-Time Member Slice

- Problem: free output must contain no member route, command, Rust logic, or product copy.
- Choice: Rust modules/commands/dependencies use `#[cfg(feature = "member")]`; frontend router/runtime/locale consume a Vite channel entry resolved to an empty free implementation or real member implementation at build time.
- Alternatives: runtime-only checks leak code; duplicating App/router per channel forks the shell.
- Minimum fit: one build adapter preserves a single shell while making the free import graph empty of real member code.
- Owners: Cargo/Vite own selection; member entry owns route/runtime/message contribution; shell owns placement.
- Cleanup/verification: no compatibility flag; dual registry/bundle scans and free route tests are mandatory.

### ADR-MI-02: Three Native Capability Owners

- Problem: account/entitlement/modules, AI/ASR, and moderation have different lifecycles and should not form a god state.
- Choice: `MembershipState`, `AiState`, and `ModerationState` are separate application owners held under member cfg in `AppState`; AI/moderation read an immutable capability projection from `MembershipState` before effects.
- Alternatives: one `MemberManager` creates broad locking and mixed responsibilities; independent entitlement copies create drift.
- Minimum fit: three cohesive owners plus one read-only projection.
- Data/state/effects: each state owns its runtime lock/tasks; repositories/ports own I/O; Vue never writes native state directly.
- Evidence: transition/unit tests and no duplicate writable entitlement field.

### ADR-MI-03: Accepted Event Fan-Out

- Problem: the live pipeline currently accepts one consumer (`MediaState`); AI context and moderation need the same post-filter events without importing member policy into live-data.
- Choice: add `AcceptedLiveEventFanout` beside the existing live-data consumer contract. It owns an immutable consumer list, calls every consumer, continues after one fails, and returns/logs aggregate failure after fan-out. In free it contains media only; member adds one member consumer that feeds AI context and moderation.
- Alternatives: member subscribes to frontend events (wrong side/security); live-data imports member types (reverse dependency); generic bus (hidden lifecycle).
- Minimum fit: a small Composite/Observer-shaped adapter at the existing seam.
- Evidence: both consumers receive once; one failure does not suppress the other or frontend publish; free composition contains one consumer.

### ADR-MI-04: Explicit Ports And Fail-Closed Adapters

- Problem: external contracts vary and most production endpoints/artifacts are absent.
- Choice: application traits define auth, membership, token verification, artifact install, LLM, moderation transport, clock, repository, and event sink boundaries. Production provides Bilibili auth plus unavailable adapters for missing services; tests inject deterministic adapters.
- Alternatives: page/service branching leaks transport; fixture-backed production is false success; a universal service interface becomes a mega contract.
- Minimum fit: ports are grouped by capability and injected into the state that uses them.
- Extension seam: only real providers listed above; no provider registry/factory until a second production implementation exists.
- Evidence: unavailable integration tests and test adapters cannot be constructed by production bootstrap.

### ADR-MI-05: Typed State Machines And Generation IDs

- Problem: QR, order/install, ASR, AI streaming, and mute actions have races/stale results.
- Choice: Rust discriminated enums plus opaque operation IDs represent terminal/non-terminal states; transition functions are pure where possible. Vue stores accept only matching IDs and normalized terminal transitions.
- Alternatives: boolean clusters permit impossible combinations; page-local timers/polling duplicate owners.
- Minimum fit: enums/state tables, not a class hierarchy or workflow engine.
- Evidence: exhaustive invalid/stale/cancel/race tests.

### ADR-MI-06: Fixed Signed Resource Contributions

- Problem: PRD requests downloaded frontend resources, but arbitrary script execution violates the security/source boundary.
- Choice: module packages contain a signed manifest and fixed resource/config/Sidecar slots. Compiled Vue pages remain authoritative; installed resources can supply versioned static assets/config consumed through native resource URLs, never executable arbitrary JavaScript.
- Alternatives: dynamic JS plugin loader is out of scope; ignoring module readiness violates activation flow.
- Minimum fit: manifest allowlist and opaque module records.
- Evidence: traversal/hash/signature/resource-kind tests and CSP/source scan.

### ADR-MI-07: Feature Stores And One Runtime Listener

- Problem: current legacy stores perform browser I/O and App hardcodes unavailable membership.
- Choice: `useMembershipStore`, `useAiStore`, `useModerationStore` contain normalized snapshots/actions; one `useMemberRuntime` initializes them and owns all Tauri listeners. The channel entry gives App a reactive `MemberShellStatus` projection and lifecycle callbacks.
- Alternatives: pages listen directly (duplicate teardown); application-shell store owns member domain (reversed ownership); one giant member store (fan-out/review burden).
- Evidence: one-listener tests, teardown tests, shell projection tests, no localStorage/browser-engine callers.

## Business Capability And Ownership Map

| Capability | Owner | Owns | Forbidden owner |
| --- | --- | --- | --- |
| build availability | Cargo/Vite channel entry | compile/import graph | runtime store flag alone |
| identity session | Rust `MembershipState` auth runtime | client, poll task, public identity | dialog/store timer |
| entitlement/order/modules | Rust `MembershipState` | signature/time, order/install tasks, inventory | shell/page |
| AI/ASR | Rust `AiState` | config, context, generation/Sidecar task, logs | template/browser engine |
| moderation | Rust `ModerationState` | rule windows, alerts, action retry/log | page/localStorage service |
| persistence | member repository | SQL/account/secret-safe records | Pinia/template |
| external transport | capability ports/adapters | HTTP/serialization/credentials | components/stores |
| frontend server state | three Pinia stores | normalized snapshots/pending/errors | duplicated component refs |
| dialog form/focus | extracted dialog SFC | draft/validation display/focus | global store unless saved |
| route/nav | router + shell navigation | route record/availability/redirect | member pages |
| event lifecycle | `useMemberRuntime` | register/dispatch/unlisten | pages |

## Module Boundary Design

- `domain/member`: opaque identifiers, immutable models, entitlement/capability rules, module transitions, AI filtering/truncation/input rules, moderation detection/similarity/state rules.
- `application/member`: three state owners, task cancellation/dedupe, ports, orchestration, accepted-event consumer.
- `infrastructure/member`: SQL/account repository, Bilibili auth, Ed25519 verifier construction, artifact confinement/integrity, unavailable external adapters.
- `ipc/member`: request/response/event DTOs, commands, safe error mapping, event sink.
- `renderer/platform/desktop/contracts/member.ts`: exact frontend command/event types; no UI labels.
- `renderer/modules/membership-intelligence`: DTO adapters, build-channel entry, view-model/status mappings.
- stores/runtime: server-state snapshots and side-effect dispatch.
- views/components: orchestration and presentational/lifecycle boundaries from page design.

## File And Directory Structure

```text
src-tauri/src/
  domain/member/{mod.rs,models.rs,rules.rs}
  application/member/{mod.rs,ports.rs,state.rs,tests.rs}
  infrastructure/member/{mod.rs,repository.rs,external.rs,artifacts.rs,tests.rs}
  ipc/{member.rs,member_contract_test.rs}
src-tauri/migrations/0002_membership_intelligence.sql

src/renderer/
  platform/desktop/contracts/member.ts
  modules/membership-intelligence/{adapters.ts,channel.free.ts,channel.member.ts}
  composables/useMemberRuntime.ts
  stores/{membership.ts,ai.ts,moderation.ts}
  views/membership/{index.vue,components/*}
  views/ai/{index.vue,log.vue,components/*}
  views/moderation/{index.vue,log.vue,components/*}
```

`asr.ts` is removed as an independent store because ASR is a sub-lifecycle of AI. Existing feature-private types/services/composables/engines are deleted only after caller scans prove replacement.

## Code Relationship And Dependency Direction

```text
App/router/i18n -> channel entry -> member runtime/routes/messages
member pages -> feature stores -> desktop gateway -> member IPC
member IPC -> application states -> domain rules
application states -> injected ports -> infrastructure adapters/repository/events
accepted live pipeline -> fanout -> media + member accepted consumer
member accepted consumer -> AiState context + ModerationState detection
MembershipState capability snapshot -> AiState / ModerationState (read-only dependency)
```

No inward layer imports an outward layer. `domain` imports no Tauri/SQLx/reqwest. `application` imports no Vue/Tauri command types/SQLx. `infrastructure` implements application ports. IPC maps types and never owns workflow rules.

## State Topology And Data Lifecycle

### Writable Native State

- Membership: account, entitlement, module inventory, current QR/order/install operations and cancellation handles.
- AI: saved config, ASR/model state, bounded context deque, active/last generation and cancellation handle.
- Moderation: saved rules/risk/auto flag, per-user bounded windows, pending alerts, counts, active action IDs.

### Derived State

- `MemberCapabilities` derives from build + verified entitlement + offline decision + module readiness; never persisted as an independent truth.
- shell status derives from account/entitlement; route availability derives from shell status + module capability.
- total progress derives from verified item bytes; UI pending derives from operation states; counts derive from logs/runtime events.

### Data Lifecycle

```text
external/native raw DTO
  -> infrastructure validation/adapter
  -> domain/application model
  -> IPC DTO (safe fields only)
  -> TS contract validator/adapter
  -> immutable Pinia snapshot/view model
  -> precise component props / local form draft
  -> typed request payload
  -> native validation and effect
  -> response/event and snapshot replacement
```

Null/enum/default/error normalization occurs at infrastructure/domain and frontend adapter boundaries. Templates do not repair malformed data. Async IDs own latest/stale/cancel semantics; state owners own cleanup.

## Dependency Rule Matrix

| From | May import | Must not import | Public API |
| --- | --- | --- | --- |
| domain/member | std + domain-stable crates | application/infrastructure/ipc/Tauri | models/rule functions |
| application/member | domain + port traits | infrastructure/ipc/Tauri | three states + accepted consumer |
| infrastructure/member | domain/application ports | ipc/UI | port implementations |
| ipc/member | domain/application + shared IPC error | infrastructure internals/UI | commands/DTO/event sink |
| channel entry | route/runtime/message contracts | page-private helpers/DTO raw shapes | routes/messages/runtime bridge |
| stores | desktop contracts + adapters | components/router/localStorage/fetch | setup-store actions/snapshots |
| runtime | stores + desktop listen | pages/domain normalization | start/stop/status bridge |
| pages | stores + normalized components | invoke/listen/raw DTO/infrastructure | route component only |
| components | precise view props/types | stores/gateway/router/raw DTO | props/emits |

Cycle risk centers on shell/member status and accepted events. Resolve with read-only projection and application-level consumer trait, never shell-to-store or live-data-to-member imports.

## Extension Seams And Rejected Future-Proofing

- Real seams: external auth/membership/LLM/moderation/artifact ports; clock/verifier for deterministic tests; signed resource-kind mapping.
- Provider selection uses a typed config plus one unavailable production adapter today, not a registry/factory. Add a strategy map only when two approved production providers have different transport behavior.
- Fixed module resources are not plugins. No arbitrary JS, dynamic route registry, dependency injection container, event bus, `*Manager`, or abstract class hierarchy.
- Remove a port if it never gains a second implementation and ceases to isolate an external/test boundary; add a new seam only for a source-approved provider/resource kind.

## Complexity Budget

- Rust: at most 14 new production files including migration/contract test; frontend: at most 24 production files excluding focused tests and locale message resources.
- New cross-feature public APIs: channel entry, member desktop contracts, three stores, one runtime, one accepted-event fanout; all other symbols feature-private.
- Indirection: page -> store -> gateway -> IPC -> state -> port is the maximum effect path; pure rule calls remove the port hop.
- One root native listener group, no page listeners; no watcher for semantic normalization; each store exposes no more than one immutable snapshot per capability plus operation state.
- If budget is exceeded, combine cohesive domain/application files or keep page-only markup inline; never collapse layers or hide logic in catch-all utils.

## Migration, Compatibility, And Cleanup Design

- No dual production path is allowed. Implement native contract/tests, migrate pages/stores, then remove legacy member services/types/engines/composables and caller imports in the same delivery unit.
- The only fallback is a typed unavailable external port, owned by native bootstrap. Cleanup trigger is receipt of an approved production contract/artifact in a later request; until then it stays visibly unavailable.
- Existing DB schema is advanced by migration, never edited in place. On migration/bootstrap failure, member capability is unavailable while free capabilities continue.
- Rollback surface is member feature files and additive migration; free build remains independently buildable throughout.
- Stale-code scans cover localStorage keys, browser microphone/LLM/moderation service imports, old stores, Electron API, direct listen/invoke, and send-danmaku terms.

## Locality And File-Extraction Decisions

| Unit | Callers | Decision/reason |
| --- | --- | --- |
| adapter validators/status mappers | 3 stores/pages | feature-local file; contract isolation and independent tests |
| QR/entitlement/module/AI/moderation pure rules | multiple native states/tests | domain files; stable business owner |
| external port implementations | one bootstrap each | separate infrastructure files; I/O/security/test boundary |
| page-only formatting/guards | 1 page | local in owning SFC unless page readability becomes unacceptable |
| dialog components | 1 each | separate only under page-design lifecycle/focus/test exceptions |
| paused notice | 2 same-semantics pages | shared inside feature component directory |
| generic audit/empty/status utilities | hypothetical | reject; different semantics or no stable callers |

## Reuse Candidate Inventory And Anti-DRY Matrix

| Candidate | Sites/layer | Same semantics | Stability / variation axes | Decision |
| --- | --- | --- | --- | --- |
| entitlement capability projection | shell, AI, moderation, membership; domain/UI | yes | stable; account, token, module | extract domain projection + TS mapper |
| accepted-event delivery | media/member; application seam | yes technical delivery | stable; consumer failure only | `AcceptedLiveEventFanout` despite 2 consumers because existing seam requires composition |
| module and ASR download integrity | 2 native workflows | same hash/resume/confinement, different manifest/license/readiness | stable mechanism; resource kind/policy | extract artifact mechanism, inject policy |
| QR/order/download/generation IDs | 4 workflows | same stale-identity mechanism, different state semantics | stable opaque ID; payload differs | reuse UUID/newtype convention, keep state machines separate |
| AI/moderation pause UI | 2 pages/UI | same entitlement pause/recovery | stable; title/body only | reuse `MembershipPausedNotice` |
| AI/moderation audit tables | 2 pages/UI | no, different fields/actions | unstable DTO, permissions, filters | keep separate |
| AI/moderation config drafts | 2 dialogs/UI | no, different validation/fields | >3 variation axes | keep separate |
| legacy service configuration mergers | 3 browser stores | not authoritative and different semantics | unstable/obsolete | delete, do not abstract |

## Commonality Classification

- Technical/infrastructure: accepted-event fanout, artifact integrity/confinement, opaque operation IDs, desktop error contract.
- Business/domain: entitlement-to-capability projection only.
- UI/design-system: existing Element Plus; feature-level paused notice.
- Configuration/constants: QR polling constants, seven-day offline duration, sourced moderation defaults, and default timeout live under their narrow domain owner.

## Shared Abstraction Decisions

- `AcceptedLiveEventFanout::new(Vec<Arc<dyn AcceptedLiveEventConsumer>>)` is application infrastructure-neutral, continues all calls, has no environment access, and documents media/member scenarios with `@see`-equivalent Rust docs/tests.
- `MemberCapabilities::derive(entitlement, modules, now)` is pure domain API used by membership state and projected through IPC; AI/moderation receive a getter/Arc reference rather than import storage or shell.
- Artifact installer accepts validated `ArtifactDescriptor`, destination root, byte stream/downloader port, and resource policy. It owns confinement/hash/atomic promotion; capability states own readiness semantics.
- `MembershipPausedNotice` accepts normalized text/action props and emits `renew`; it imports no store/router and owns no entitlement calculation.
- No broad shared utility index is introduced.

## Dependency Injection And Strategy Boundaries

- `MembershipState`: repository, auth gateway, membership gateway, token verifier, artifact installer, clock, event sink.
- `AiState`: membership capability getter, repository, LLM gateway, ASR runtime/artifact port, clock, event sink.
- `ModerationState`: membership capability getter, repository, moderation gateway, clock, event sink.
- Stable mechanism lives in application/domain; endpoint fields, filesystem/process/network effects and provider policy remain injected infrastructure.
- Tests construct states with fixed clock, in-memory repositories and deterministic ports; shared pure APIs never access browser/network/router/storage/store globals.

## Responsibility Split

- Pages orchestrate sections/routes and call store intentions.
- Extracted SFCs own cohesive UI/focus/draft behavior only.
- Stores own request dedupe, normalized snapshots and display error state.
- Runtime owns initialization/listener lifecycle.
- Desktop contracts/adapters own frontend type/runtime normalization.
- IPC owns serialization/safe errors.
- Application owns use cases/tasks; domain owns deterministic rules; infrastructure owns I/O.

## Function Design And Public Entrypoints

- Native public entrypoints are the exact command/event groups in approved spec. Commands are thin: validate DTO, call one state method, map DTO/error.
- Each state exposes `status` plus capability actions; task-start methods return operation snapshots immediately and publish updates; cancel methods are idempotent.
- Pure functions include `map_qr_status`, `validate_entitlement`, `derive_capabilities`, `transition_module`, `validate_generation_input`, `filter_and_truncate`, `evaluate_moderation`, and DTO mappers.
- Stores expose user intentions named `bootstrap/start/cancel/save/generate/regenerate/ignore/mute` with pending/error sets; components do not invoke commands.

## Functional Boundaries And Side-Effect Model

- Pure inputs are immutable DTO/model snapshots; rules return new states/results.
- Side effects are limited to infrastructure ports, application task methods, IPC event sink, Pinia actions, runtime lifecycle and component event handlers.
- No helper that sounds like `get/map/derive/validate` may perform I/O or mutation.
- Derived capability/progress/count/navigation states remain computed, not separately writable.
- Reject a new FP library, workflow engine, point-free composition, or reducer framework.

## State Ownership And Data Flow

- Membership is the sole authority for account/entitlement/modules and exposes a read-only capability snapshot.
- AI and moderation cache no writable entitlement; they check the snapshot at command boundaries and respond to expiry event for task cancellation.
- Stores replace snapshots atomically after validation. Dialog drafts are cloned and discarded/saved explicitly.
- Root runtime dispatches event by typed name to exactly one store action; operation ID/sequence protects latest-wins semantics.

## Data Structures And Type Strategy

- Rust newtypes: `UserUid`, `AuthSessionId`, `OrderId`, `ModuleId`, `OperationId`, `GenerationId`; secret token/key/Cookie wrappers redact `Debug`/serialization.
- Rust enums: `AuthStatus`, `EntitlementStatus`, `ModuleStatus`, `OrderStatus`, `AsrStatus`, `GenerationStatus`, `ModerationActionStatus`, `MemberErrorCode`; exhaustive matches required.
- TypeScript mirrors safe IPC shapes with readonly discriminated unions and branded string IDs. Raw input drafts remain strings until validated payload creation.
- Amounts use integer minor units plus explicit currency when contract exists; no `REAL` amount becomes frontend truth. Timestamps are integer UTC milliseconds; expiry formatter is membership-specific.
- Unknown external enums fail adapter validation. `null` is explicit absence; `undefined` only before initialization.

## Contract And Adapter Boundaries

- Rust DTOs are authoritative for desktop IPC; `contracts/member.ts` matches serialization exactly.
- `adapters.ts` validates unknown runtime payloads and maps to safe view models; it does not invent default success/content.
- Bilibili, membership, LLM, moderation, and artifact upstream shapes remain private to infrastructure adapters.
- SQL rows remain private to repository. Pages/components receive no SQL/upstream DTO or secret/path.

## Pattern Decisions And Rejected Alternatives

- Overall: reuse existing project layering/Command/gateway/store pattern at Level 2.
- Adapt lightweight State via enums/transition functions for QR/order/module/AI/ASR/mute, Adapter for external/IPC shapes, and Composite/Observer for accepted-event fan-out.
- Use Strategy only as injected port objects for real external/test variants; no runtime strategy registry.
- Reject Facade/Manager around all member features, generic Command objects, frontend Pub-Sub, provider Factory, dynamic plugin registry, and proxy/cache layers. They add indirection without an approved variant or obscure cleanup/security.
- Simplify if any pattern has one variant without boundary/test value, adds an extra effect hop, or forces broad optional types.

## Readability And Maintenance Guardrails

- Keep commands under roughly one screen and state actions single-workflow. Split by cohesive capability, not arbitrary file length.
- Explicit transition matches and guard clauses beat boolean clusters/clever combinators.
- Document security-sensitive path/signature/redaction decisions; avoid narration comments.
- Every new shared API names callers/evidence. Every background task has generation/cancel/shutdown owner.
- No stale compatibility exports, barrel dumping ground, hidden constants, long class strings, console/debug code, or unrelated formatting.

## Architecture Risks

- Adding external crypto/archive/network crates can expand build surface; keep them member-gated where Cargo permits and verify free dependency/binary scans.
- SQL initial schema stores amount as `REAL` and credential-like account fields; migration must introduce safe shapes without destructive rewrite.
- App-shell status integration can create cycles; channel runtime projection is the only frontend bridge.
- Fan-out must isolate a failing member consumer so media and frontend live events continue.
- Large UI/component count risks review fragmentation; page-design exceptions are the cap, not a target for extra extraction.

## Open Architecture Questions

- Production secret storage implementation remains pending; use an unavailable secret/provider path rather than plaintext.
- Production membership verifier key, artifact host/archive format, provider schemas, moderation credential lifecycle, and Whisper manifest remain external pending.
- Exact archive hard limits and resume protocol depend on signed manifest/transport contract; architecture fixes confinement/hash/atomicity now and leaves provider-specific limits unavailable.
- These do not change approved UI/business semantics and do not block planning the local/fail-closed implementation.

## Architecture Exit Check

- Ownership, topology, dependency rules, files, public APIs, state/data lifecycle, Anti-DRY decisions, injection seams, cleanup, patterns, and complexity budget are fixed.
- Spec/page-design semantics are unchanged.
- Architecture is sufficiently concrete for task decomposition; advance `state.json.stage` to `plan` and preserve `loop`.
