# Platform Foundation Architecture Design

## Delivery Unit Identifier

- Request: `tauri-rust-refactor`
- Module: `platform-foundation`
- Upstream contract: `module-runs/platform-foundation/spec/spec.md`
- Decision status: approved architecture input for planning

## Architecture Objective

Create the smallest durable Tauri/Rust platform that can absorb the five remaining functional modules without leaking desktop I/O into Vue, retaining Electron as a second production runtime, or relying on UI hiding for security. The architecture fixes ownership and dependency direction while keeping domain repositories and adapters local to the modules that actually need them.

## Architecture Scope And Triggers

This design owns Tauri bootstrap, cross-boundary types, command/event transport, managed state, validation primitives, SQLite connection/migrations/config repository, path grants, capabilities and free/member compilation. It is architecture-sensitive because it introduces a Rust build surface, reverses current renderer-owned I/O, establishes shared APIs for many callers and controls temporary Electron compatibility.

It does not design live data, media, UI shell, membership behavior or extensions beyond the ports and build gates explicitly required by the foundation spec.

## Upstream Inputs And Assumptions

- The current PRD and the approved module spec are immutable behavior contracts.
- Existing Vue/Vite/Pinia/Router code remains the sole WebView application.
- Rust stable is installed as an implementation prerequisite.
- `sqlx` with SQLite and Tokio is selected from the PRD's allowed `sqlx / rusqlite` choices because downstream services are async and migrations need one transactional owner.
- External services and Sidecar artifacts are unavailable; no placeholder success behavior is permitted.
- The Tauri v2 CLI generated structure is a baseline, not an authority for product behavior.

## Scaffold Alignment And Bootstrap Boundaries

- Use the official Tauri v2 existing-frontend initialization under `packages/bilibili-live-tool/src-tauri`.
- Preserve generated `Cargo.toml`, `build.rs`, `tauri.conf.json`, icons/resources and `src/{main,lib}.rs` layout where it matches current Tauri conventions.
- Remove the demo greet command, demo frontend markup and unused sample assets/configuration.
- Configure `beforeDevCommand`/`devUrl` and `beforeBuildCommand`/`frontendDist` for the existing package's Vite scripts and `dist/renderer`.
- Vite owns WebView bundling; Cargo/Tauri owns desktop runtime, resources, capabilities and native packaging.
- Package scripts own developer entrypoints. No custom JavaScript process supervisor is retained as the Tauri launcher.
- Deviations fixed by spec: custom five-layer Rust modules, SQL migrations, custom permissions/capabilities, two build configs and one shared application identifier.

## Expert Architecture Decision Records

### ADR-PF-001: One Typed WebView Gateway

- Problem: current renderer callers use generic channel strings and direct `window.electronAPI`, including unrestricted filesystem functions.
- Choice/owner: `src/renderer/platform/desktop` owns a typed Tauri facade with command/event maps and error normalization.
- Alternatives: per-service `invoke`; a generic Electron-compatible bridge; retaining preload.
- Smallest sufficient structure: one infrastructure boundary is justified by 10+ callers and one transport variation axis.
- Direction: feature store/service -> desktop facade -> `@tauri-apps/api`; the facade imports no feature-private type.
- State/data: no mutable global state; listener lifecycle remains with the caller.
- Side effects: invoke/listen exist only in this boundary.
- Cleanup: delete legacy `utils/ipc.ts` and preload when all mapped callers migrate.
- Evidence: static search, type-level command tests and lifecycle listener tests.

### ADR-PF-002: Thin Commands, Application-Owned Workflows

- Problem: Electron IPC handlers currently mix transport, validation and implementation.
- Choice/owner: `ipc` deserializes/validates/maps errors; `application` orchestrates; `domain` owns pure contracts; `infrastructure` owns I/O.
- Alternative: commands directly query SQL/filesystem; class-heavy service container.
- Smallest sufficient structure: module functions plus Tauri managed state; traits only at real replaceable external/I/O boundaries.
- Direction: ipc -> application/domain ports <- infrastructure.
- State owner: one `AppState` composed at bootstrap.
- Extension seam: repository/external adapters only; no speculative manager/factory registry.
- Cleanup/evidence: no Tauri/SQLx import in domain, enforced by module review and tests.

### ADR-PF-003: SQLx Connection And Ordered Migrations

- Problem: PRD requires SQLite WAL, explicit schema, backup and transactional migration while current target uses electron-store.
- Choice/owner: infrastructure `database` owns `SqlitePool`, connection pragmas and embedded migrations; repositories receive a pool handle.
- Alternatives: rusqlite blocking connection; one repository per future table now.
- Smallest sufficient structure: one pool, one migration runner and only the foundation config repository. Future repositories stay with their domain modules.
- State/data: database and schema version are Rust-owned; WebView receives only status DTOs.
- Side effects: open/migrate/backup only in database/migration infrastructure.
- Cleanup: legacy store reader is temporary and removed after module 6 migration verification.
- Evidence: clean schema enumeration, WAL, idempotency and rollback tests.

### ADR-PF-004: Compile-Time Channel Isolation

- Problem: free installer must not contain membership logic or command registration.
- Choice/owner: Cargo `free` default and `member` features conditionally compile modules/commands; Vite receives the same channel at build time and cannot import member entrypoints in free builds.
- Alternatives: runtime boolean; route hiding; separate forks.
- Smallest sufficient structure: compile features plus two configuration overlays share all common code and identifier.
- State owner: compile constant derives a readonly `BuildChannel`; it is never writable.
- Cleanup: no temporary dual feature flag. Any command added later must declare its channel owner.
- Evidence: both builds plus command-symbol/registry tests and free bundle inspection.

### ADR-PF-005: Opaque Purpose-Bound Path Grants

- Problem: current preload accepts arbitrary renderer paths; PRD requires dialog-authorized paths and Rust validation.
- Choice/owner: `PathGrantStore` in Rust maps opaque grant ids to canonical paths and a closed purpose enum. Commands accept grants, not paths.
- Alternatives: frontend-held raw paths; broad filesystem plugin scope; global directory allowlist.
- Smallest sufficient structure: one in-memory/persisted grant owner and pure containment/purpose validation.
- State owner: Rust; display path is a non-authoritative DTO.
- Side effects: dialog/canonicalization/persistence at infrastructure boundary.
- Cleanup: delete preload filesystem API when owning callers migrate.
- Evidence: cancel, stale, symlink escape, wrong-purpose and revoke tests.

### ADR-PF-006: Stable Error Envelope

- Problem: current IPC errors have inconsistent success wrappers/string errors.
- Choice/owner: application error enum maps once to serializable `DesktopError`; TypeScript maps rejected invokes into the same shape.
- Alternatives: raw strings; per-command response wrapper; broad unknown details.
- Smallest sufficient structure: discriminated codes with optional string details and retryability.
- State/side effects: none; mapping is pure and redacts secrets/paths.
- Evidence: serialization snapshots and representative error-path tests.

## Business Capability And Ownership Map

| Capability | Writable owner | Public consumer | Forbidden owner |
| --- | --- | --- | --- |
| Build channel/capabilities | Rust compile configuration | bootstrap DTO, router later | Pinia mutation, localStorage, query string |
| Bootstrap readiness | Rust `AppState` | desktop gateway, later shell store | Vue component-local duplicate authority |
| Database pool/schema | Rust database infrastructure | repositories/application bootstrap | WebView, Tauri command body |
| Configuration values | Rust config repository | config application service | template, arbitrary renderer storage |
| Path grants | Rust path grant store | application services by grant id | raw Vue strings, preload filesystem API |
| Input constraints | Rust domain validation | command adapters; TS UX mirror | templates/watchers as authority |
| Transport contract | Rust serde DTOs + TS mirror | feature services/stores | individual pages with raw channels |
| Listener cleanup | subscribing Vue owner | component/store lifecycle | global hidden event bus |
| Migration | Rust migration runner | bootstrap status | renderer, independent table repositories |

## Module Boundary Design

### Rust

- `lib.rs`: composition root only; plugins, state, command registration and setup.
- `build_channel.rs`: compile-derived `BuildChannel` and command capability metadata.
- `ipc`: command DTOs, handlers, event DTOs and error serialization.
- `application`: bootstrap/config/path use cases and application errors/ports.
- `domain`: value types and pure validation; no framework or I/O dependencies.
- `infrastructure`: database/migrations/config repository, filesystem/path grants, secure-store adapter and external unavailable adapters.
- `member` modules are behind `#[cfg(feature = "member")]`; foundation only exposes the gated registration seam.

### TypeScript

- `platform/desktop/contracts.ts`: exact foundation request/result/event/error types.
- `platform/desktop/transport.ts`: constrained invoke/listen implementation.
- `platform/desktop/index.ts`: deliberate public exports.
- Feature services import the public gateway only. Existing business types remain in feature modules until their migration.

## File And Directory Structure

```text
packages/bilibili-live-tool/
  package.json
  vite.config.ts
  src/
    renderer/
      platform/
        desktop/
          contracts.ts
          transport.ts
          index.ts
      utils/ipc.ts                 # temporary, then removed by caller cleanup
    main/                          # legacy, module-by-module deletion only
    preload/                       # legacy, removed after no callers remain
  src-tauri/
    Cargo.toml
    build.rs
    tauri.conf.json
    tauri.free.conf.json
    tauri.member.conf.json
    capabilities/
      main.json
      effect.json
    permissions/
      foundation.toml
    migrations/
      0001_initial.sql
    src/
      main.rs
      lib.rs
      build_channel.rs
      ipc/
        mod.rs
        commands.rs
        dto.rs
        error.rs
        events.rs
      application/
        mod.rs
        bootstrap.rs
        config.rs
        paths.rs
        ports.rs
      domain/
        mod.rs
        validation.rs
      infrastructure/
        mod.rs
        database.rs
        config_repository.rs
        migrations.rs
        path_grants.rs
        secure_store.rs
```

Separate Rust files are justified by framework entrypoints, public contracts, independent side-effect ownership or focused test surfaces. Architecture may combine adjacent small files when implementation shows they are trivial, but it may not introduce catch-all utilities.

## Code Relationship And Dependency Direction

```text
Vue page/store/service
  -> renderer/platform/desktop public facade
    -> Tauri invoke/listen
      -> ipc command adapter
        -> application use case -> domain rules/ports
                              -> infrastructure adapter
```

- `lib.rs` may depend on every Rust layer only to compose them.
- Infrastructure implements application/domain ports; application/domain never import infrastructure.
- IPC never contains SQL, filesystem traversal or business workflows.
- Renderer platform code cannot import stores/views; feature code cannot import `@tauri-apps/api` directly.

## State Topology And Data Lifecycle

### Writable State

- `AppState`: immutable handles plus an async initialization state machine guarded for one-time initialization.
- `SqlitePool`: owned by database infrastructure and shared by cloneable handle.
- `PathGrantStore`: synchronized Rust map/repository, keyed by opaque id.
- Config rows: SQLite is authoritative; any Vue store snapshot is readonly cache and invalidated by command result/event.
- Listener lifecycle: local subscriber owns the unsubscribe handle.

### Derived State

- Build channel derives from Cargo feature, never persisted.
- Supported command/capability set derives from build channel and registration, never a writable list.
- Readiness derives from bootstrap/migration result.
- Schema version derives from applied migration state.

### Data Lifecycle

```text
TS immutable request
  -> Tauri serializer
  -> Rust serde DTO
  -> pure validation/newtype
  -> application input
  -> repository/adapter side effect
  -> application result
  -> response DTO
  -> TS typed result / later view-model adapter
```

Invalid, empty and enum semantics normalize in Rust validation. The TypeScript facade may prevalidate for fast feedback but cannot weaken authority. Secrets never enter result DTOs. Cancellation/stale response handling belongs to each caller; bootstrap itself deduplicates concurrent calls and has explicit retry state.

## Dependency Rule Matrix

| From | May import | Must not import | Public API |
| --- | --- | --- | --- |
| Vue views/components | feature store/service, UI components | Tauri API, Rust DTO internals, filesystem | none |
| Feature store/service | desktop public facade, feature types | transport internals, other feature private state | feature methods |
| Desktop facade | Tauri core/event API, contracts | views, stores, feature-private types | typed invoke/listen |
| Rust IPC | application, domain DTO/value constructors | SQLx, filesystem internals, Vue concerns | Tauri commands/events |
| Rust application | domain ports/types | Tauri/WebView, concrete infrastructure | use-case functions |
| Rust domain | Rust std/serde only where contract serialization is necessary | Tauri, SQLx, reqwest, filesystem | value types/rules/ports |
| Rust infrastructure | domain/application ports, approved crates | IPC/webview modules, feature UI | adapter implementations |
| Composition root | all Rust modules for wiring | business logic in setup | `run()` |

Cycle risks: existing logger cycles remain outside new Rust/desktop facade. `contracts.ts` cannot import feature types; feature modules own adapters to prevent a platform-to-feature reverse edge.

## Extension Seams And Rejected Future-Proofing

### Approved Real Seams

- Build channel: Cargo feature and conditional module/command registration.
- Storage/external dependencies: traits only where tests or member/external adapters require replacement.
- Desktop transport: one typed facade because the transport is changing now.
- Domain repository ownership: connection mechanism shared; concrete repositories added by owning modules.

### Rejected

- No generic plugin/service locator, dependency-injection framework, command registry DSL or runtime module factory.
- No repository trait for each of 21 tables before a real use case.
- No universal DTO mega-interface or generic CRUD command.
- No Electron/Tauri runtime auto-detection in views.
- No frontend event bus layered on top of Tauri without multiple real consumers and lifecycle ownership.

Add a seam later only when a second real implementation/provider or three stable same-semantic callers appear. Remove an approved seam if it becomes a one-method pass-through with no dependency or test benefit.

## Complexity Budget

- Foundation target: at most 24 new hand-authored Rust/TypeScript/config/migration files, excluding generated icons and lockfiles.
- Public frontend API: three generic typed operations at most (`invokeDesktop`, `listenDesktop`, error guard) plus contract types.
- Public Rust command surface: only six foundation commands and two events from spec; internal helper APIs remain crate-private.
- Indirection: page/service -> facade -> IPC -> application -> infrastructure, maximum five intentional layers across process boundary; pure domain validation may be composed within application.
- Reactive fan-out: no new watcher; each event subscription has an explicit owner/unsubscribe.
- Migration scope: package/build/gateway/foundation storage only. Business service rewrites stay outside this module.
- If budget is exceeded, collapse trivial module files and remove pass-through traits before weakening ownership boundaries.

## Migration, Compatibility And Cleanup Design

| Temporary path | Owner | Cleanup trigger | Rollback | Evidence |
| --- | --- | --- | --- | --- |
| Electron `src/main` | owning later functional module | its Tauri replacement and tests pass | retain source until replacement verified | per-module caller search |
| Electron preload | platform migration | zero `window.electronAPI` callers | restore only from VCS/user changes, never runtime fallback | `rg` zero-result + build |
| `utils/ipc.ts` compatibility | renderer desktop facade | all callers use typed facade | explicit mapping rollback within feature branch | import graph/type-check |
| Legacy store reader | module 6 migration | migration verified across supported legacy fixtures | source data untouched; DB transaction rolled back | fixture hashes/schema state |
| External unavailable adapter | owning integration module | authoritative contract/config/artifact supplied and tested | revert to unavailable without affecting free base | adapter contract tests |

There is no automated runtime fallback to Electron. Stale-code detection uses dependency search plus production dependency/bundle inspection. Shared data migration writes only after backup/preflight; failure leaves legacy input intact.

## Locality And File-Extraction Decisions

| Unit | Callers | Decision | Reason |
| --- | --- | --- | --- |
| TS desktop contracts/transport | 10+ projected | separate platform boundary | contract isolation and independent tests |
| Rust validation | many commands | separate domain module | stable PRD rules, deterministic tests |
| Error mapper | all commands | IPC/application boundary file | cross-cutting serialization and redaction |
| SQL migration | one runner | separate migration file/module | generated/contract isolation and transactional test surface |
| Config repository | bootstrap + later settings | separate infrastructure module | persistence side effect owner |
| Path grants | many file workflows | separate infrastructure module | security lifecycle and focused tests |
| Bootstrap-only small conversion | one | colocate in bootstrap owner | no separate semantic boundary |

No `utils.ts`, `helpers.ts`, one-function class or page-owned helper extraction is approved.

## Reuse Candidate Inventory And Anti-DRY Matrix

| Candidate | Locations/use sites | Semantics/layer | Stability/variation | Decision |
| --- | --- | --- | --- | --- |
| Generic IPC invocation | `utils/ipc.ts` + many direct Electron callers, 10+ | same transport boundary | stable mechanism; command DTOs vary | replace with typed facade |
| Electron result `{success,error}` mapping | several main/renderer modules | same error boundary but inconsistent payloads | stable need; codes/details vary | one DesktopError mapper, migrate callers |
| ID/string/number validation | room, blacklist, sensitive, audio/settings, 3+ | same PRD domain constraints at Rust boundary | stable limits; distinct value type | shared validation primitives with named newtypes |
| Database access | renderer database service + Electron store + future domains | same I/O mechanism, different domain semantics | pool stable; queries vary by aggregate | share pool/migration only, keep repositories separate |
| Path checks/file methods | preload, plugin, recording, sound, 3+ | same security mechanism; purposes differ | purpose enum is stable, operations vary | shared path-grant mechanism; domain operation stays local |
| Feature visibility | future router/nav/member commands | same build capability source across Rust/TS | two real build variants | compile-derived channel contract, not mutable store |
| Business DTO adapters | future live/media/member modules | different fields, permissions and lifecycle | more than three variation axes | keep separate in owning modules |

The approved extractions are infrastructure/domain commonalities with multiple stable use sites. Repository and DTO similarities are intentionally not abstracted because their business semantics and variation axes differ.

## Commonality Classification And Shared Abstraction Decisions

- Technical/infrastructure: typed desktop transport, database pool/migrations, path-grant mechanism, error serialization.
- Business/domain: validated room/user identifiers and PRD value constraints; names remain domain-specific rather than a generic string validator API.
- Configuration/constants: command channel ownership, validation limits and build channel enums stay in the narrowest module.
- UI/design system: none in this unit.

Promoted TypeScript APIs include `@see` references to representative service callers once migrated. Rust public/crate APIs use doc comments with examples for validation and feature registry. Tests prove error/validation behavior equivalence with PRD rules, not with unsafe Electron behavior.

## Dependency Injection And Strategy Boundaries

- Application bootstrap receives repository/path/secure-store ports only where fake implementations are required for deterministic failure/rollback tests.
- Stable mechanism stays in the foundation; feature-specific policy is passed as typed input or implemented in the feature module.
- Pure helpers accept all data as parameters and can run without Tauri, WebView, network, filesystem or global store.
- `AppState` wires concrete adapters; no container library or ambient mutable singleton.

## Responsibility Split

- Vue: call/subscribe, own presentation lifecycle, never authorize or persist native state.
- IPC: validate boundary DTOs, call one application entrypoint, map errors/events.
- Application: coordinate bootstrap/config/path workflows and transaction boundaries.
- Domain: express valid values, build channel and error semantics.
- Infrastructure: native I/O and external dependency implementations.
- Composition root: construct handles and register only commands compiled for the active feature.

## Function Design And Public Entrypoints

### TypeScript

```ts
invokeDesktop<K extends CommandName>(name: K, payload: CommandRequestMap[K]): Promise<CommandResultMap[K]>
listenDesktop<K extends EventName>(name: K, listener: (payload: EventPayloadMap[K]) => void): Promise<UnlistenFn>
isDesktopError(value: unknown): value is DesktopError
```

### Rust

- `run()` is the desktop composition entrypoint.
- IPC exports only spec command functions.
- `bootstrap_application(&AppState)`, config operations and path operations are application entrypoints.
- Validators return validated newtypes/results; repositories expose intent-revealing operations, not generic SQL/CRUD.
- Command feature lists are produced by compile-conditional functions used by registration/tests.

## Functional Boundaries And Side-Effect Model

- Pure: normalize/validate values, derive build capabilities, map errors, convert DTOs and decide migration preconditions.
- Effects: Tauri setup/commands, SQL repositories/migrations, dialog/filesystem canonicalization, secure storage and event emission.
- Inputs are immutable; transformations return new values.
- Query functions do not mutate; commands have effect-revealing names.
- No functional library, point-free pipeline or hidden module cache. Straight Rust/TypeScript functions are sufficient.

## Data Structures And Type Strategy

- Rust validated newtypes: `RoomId`, `UserId`, `Username`, `SensitiveWord`, `Volume`, `Threshold`, `PathGrantId`.
- Rust enums: `BuildChannel`, `BootstrapStatus`, `PathPurpose`, `DesktopErrorCode`, migration stage.
- Serde DTOs use `camelCase` at the boundary; domain types remain Rust-idiomatic.
- TypeScript mirrors command/event maps and discriminated states. No `any`; `unknown` is accepted only at rejection boundary before runtime narrowing.
- `Option`/optional fields mean source-authorized absence. Required invalid values are rejected instead of defaulted.
- Configuration values begin as a closed internal enum/value representation, not arbitrary SQL or unrestricted secret access.

## Contract And Adapter Boundaries

- Rust serde DTO names are canonical. TypeScript contracts mirror them one-to-one.
- The TS transport adapter handles only invocation/listening/error normalization.
- Feature adapters later translate foundation DTOs into view models; templates never consume backend/storage records directly.
- SQL rows are private to repositories.
- Secure values are represented by presence/reference metadata, not returned plaintext.
- External unavailable adapters satisfy no fake production contract and return the stable unavailable error.

## Pattern Decisions And Rejected Alternatives

- Overall: Level 3 `introduce pattern`, Ports-and-Adapters at the native boundary, justified by an actual runtime replacement and multiple I/O providers.
- WebView: Level 2 typed Facade/Adapter using functions and mapped types.
- Build variants: Level 1 compile-time strategy through Cargo/Vite constants; no runtime strategy objects.
- State: explicit small enum state machines for bootstrap/migration; no State classes.
- Rejected generic Command pattern registry: Tauri commands already provide invocation; an extra object layer adds no queue/undo/retry behavior.
- Rejected factories/managers/service locator: one composition root and concrete lifecycle owners are clearer.
- Simplification trigger: collapse any trait/facade that has one pass-through implementation and no test/dependency isolation value.

## Readability And Maintenance Guardrails

- One workflow entrypoint per foundation capability and guard clauses for invalid preconditions.
- Commands stay thin; SQL/path logic cannot appear in handlers.
- Exhaustive enum matching; no wildcard that silently accepts future variants.
- Domain names replace vague `process`, `manager`, `handler` except framework-required handler terminology.
- No shared index dumping ground or cross-feature type union.
- Every temporary compatibility mapping includes owner and deletion condition in the changelog.
- Comments explain security/transaction reasoning only; generated/sample narration is removed.

## Architecture Risks

- Tauri custom-command ACL behavior must be verified against the installed v2 version; capability tests are mandatory.
- Two build overlays must not change the shared identifier or data path.
- SQLx/Tauri async setup can deadlock if migrations hold state locks while emitting events; initialization state and pool work must not share a blocking mutex.
- Canonical-path checks vary across platforms and missing paths; grant tests must cover Windows now and portable semantics in CI.
- Incremental Electron cleanup can drift if caller searches are not repeated per module.

## Open Architecture Questions

No architecture-blocking question remains. Exact compatible crate/plugin patch versions and generated Tauri config schema are resolved by the official scaffold/lockfile during execution. If the installed Tauri command ACL model cannot express an effect window with no app-command access, implementation must return to this design/spec rather than ship a UI-only restriction.
