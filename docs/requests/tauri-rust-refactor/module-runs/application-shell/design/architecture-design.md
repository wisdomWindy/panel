# Application Shell Architecture Design

## Delivery Unit Identifier

- request: `tauri-rust-refactor`
- module: `application-shell`
- stage: `architecture-design`
- self-healing iteration: `7`
- input contract: framework-approved `spec/spec.md` and `design/page-design.md`

## Architecture Objective

Create one durable application-shell boundary that composes already-owned room/media state with Rust-owned UI/config/system state. The architecture must remove root/settings browser authority, keep free/member frontend and native surfaces separable at build time, and give later membership/notification modules narrow typed contribution points without a generic registry.

## Architecture Scope And Triggers

- Root component and lifecycle currently mix room connection, update behavior, hard-coded navigation, state display and route layout.
- UI/locale configuration has duplicate browser and Rust persistence owners.
- Configuration import/export/reset crosses file grants, versioned serialization and SQLite transaction boundaries.
- Dynamic navigation varies by build/member state and affects route access.
- CPU/memory are high-frequency OS data requiring one lifecycle owner.
- These ownership, state/data topology and cross-module seams require architecture design; spec behavior remains immutable.

## Upstream Inputs And Assumptions

- PRD 8/9.17/9.20/9.21 and the spec define all visible fields, states, defaults and acceptance.
- Existing typed desktop transport remains the only frontend Tauri dependency.
- Existing room/feed/recording stores remain authoritative for their state.
- `t_app_setting`, SQLite pool and PathGrantStore are reusable infrastructure.
- `sysinfo 0.30.13` is selected for process metrics: official docs state Windows/macOS/Linux support, MSRV 1.69, differential CPU sampling and the need to retain one instance and refresh only required data. It is compatible with project Rust 1.77.2 and will use `default-features = false`.
- Real member state and notification contents are downstream; this design cannot fabricate them.

## Scaffold Alignment And Bootstrap Boundaries

Not a new scaffold. Preserve the approved Vue/Vite/Tauri v2 bootstrap, root `main.ts`, Hash router, Pinia instance, strict TypeScript context and free/member Tauri configs. Only extend `AppState`, IPC registries/capabilities and the existing frontend bootstrap. Do not add another app root, router, store container, Electron bridge or UI framework.

## Expert Architecture Decision Records

### AS-ADR-01: One Shell State Topology

- Problem: `App.vue`, localStorage config, route strings and feature stores currently overlap state responsibilities.
- Choice: Rust `ApplicationShellState` owns persisted UI/locale/config metadata and metrics runtime; Pinia `useApplicationShellStore` owns only latest normalized snapshots/async state; room/feed/recording remain in their existing stores; pages own drafts.
- Alternatives: keep state inline in App; create one global mega store; copy room/recording into shell state. Rejected due lifecycle mixing or duplicate writable truth.
- Dependency: components -> shell store/view models -> desktop adapter -> Rust; shell store reads completed stores only through root composition helpers.
- Side effects: Rust/application, desktop transport, Pinia actions and root lifecycle only.
- Extension: member projection is a typed optional input, not shell-authored member truth.
- Cleanup/evidence: delete localStorage/UI writes and duplicate status refs; store/ownership tests and scans.

### AS-ADR-02: Typed Closed Shell IPC Surface

- Problem: generic `config_get/config_set` cannot express atomic UI/config workflows or validate field semantics.
- Choice: add the exact spec shell command/event map in `contracts/shell.ts` and `ipc/shell.rs`; Rust serde is authoritative; frontend adapter maps DTOs to immutable view models.
- Alternatives: expose raw key/value config to pages; direct Tauri APIs; one generic `shell_action`. Rejected for weak typing, path leakage and unclear permissions.
- Minimum structure: one contract file, one IPC module, one adapter module; no command classes or per-command wrappers.
- Cleanup/evidence: pages import store only; exact 13/3 registry/capability tests.

### AS-ADR-03: Fixed Versioned Config Archive Transaction

- Problem: export/import/reset spans multiple current configuration owners and must be atomic without a plugin registry.
- Choice: `ShellRepository` implements a versioned, closed `ConfigArchiveV1` over current common scopes using explicit match/queries inside one SQL transaction; member variants compile only under member feature and may be extended by its owning module.
- Alternatives: serialize frontend stores; dynamic exporter registry; arbitrary DB dump. Rejected due secrets, browser authority, speculative extensibility and schema leakage.
- File I/O: grants resolve only inside Rust `ConfigArchiveFilePort`; display paths never become authority.
- Rollback: parse/validate complete archive before transaction; any write failure rolls back; staged output is removed on failure.
- Evolution: a later version adds explicit enum variants/migration, not arbitrary section handlers.

### AS-ADR-04: Unified UI Projection With Saved/Draft Separation

- Problem: theme, locale, opacity and layout can drift between localStorage, Vue and native state.
- Choice: Rust stores/validates `UiPreferences` and `LocalePreferences`; one frontend `applyUiProjection` function maps validated snapshots to root data attributes/CSS variables and vue-i18n/Element Plus locale. Settings components own drafts only.
- Transparency decision: use UI surface alpha through one CSS variable; do not enable OS transparent windows or macOS private APIs. Tauri documents platform caveats and Windows alpha behavior, so native transparency would add distribution risk not required by the source wording.
- `ui_preview` validates through Rust and returns a normalized non-persisted snapshot; Vue applies it. Save commits then emits; failure reapplies persisted snapshot.
- Alternatives: component-level styles; direct document mutation in each control; native transparent window. Rejected for fan-out, rollback ambiguity and platform risk.
- Evidence: projection/rollback/high-DPI/free+member tests.

### AS-ADR-05: Build-Time Free Surface Plus Injected Member Projection

- Problem: runtime `v-if` alone can leave member strings/routes/chunks in the free bundle.
- Choice: common shell contains only free descriptors and neutral contribution types. Member-only descriptors/locales/routes live in the later member feature module and are imported through a Vite-eliminable direct `import.meta.env.VITE_BUILD_CHANNEL` branch. Rust member commands remain `#[cfg(feature="member")]`.
- Runtime: member module supplies `MemberShellProjection`; shell never maps `unavailable` to logged-out/active.
- Alternatives: shared locale file with hidden strings; generic RBAC; route meta booleans scattered across router/templates. Rejected for leakage or overengineering.
- Cleanup/evidence: bundle/source/route/registry scans for member labels/commands in free artifacts and four-state projection tests in member channel.

### AS-ADR-06: Single Process Metrics Runtime

- Problem: CPU/memory sampling requires differential state and can leak timers/tasks or cause reactive churn.
- Choice: one Rust `SystemMetricsRuntime` retains a `sysinfo::System`, refreshes only current-process CPU/memory on a bounded 1-second cadence, stores/emits latest value and stops via cancellation token/join handle on main-window destruction.
- Alternatives: browser Performance API; recreate `System` per request; renderer polling timers; all-system refresh. Rejected for incomplete metrics, incorrect CPU samples or excess cost.
- Port: `SystemMetricsPort` supports fake/unavailable adapters; production `SysinfoMetricsPort` returns nullable metrics on unsupported systems without fake zero.
- Evidence: paused-time runtime tests, one-owner test, cleanup and bounded-event tests.

### AS-ADR-07: Explicit Shell Region Components

- Problem: root template would otherwise mix permission, high-frequency state, form flows and layout classes.
- Choice: approved toolbar/navigation/status and settings panels/dialog become focused SFCs; App/settings remain orchestration containers.
- Components receive view models and intention emits; no component imports transport or calculates permission.
- Alternatives: one monolithic App; broad configurable `ShellSection`; card wrappers. Rejected for review cost, optional-prop interfaces and page-design conflict.
- Evidence: focused component tests and stable screenshot geometry.

### AS-ADR-08: Direct Migration With No Dual Runtime

- Problem: retaining localStorage/Blob/static permission paths would allow split truth.
- Choice: implement native contracts and store, switch root/settings callers, then delete owned old config store/browser utilities/action code in the same module.
- Temporary compatibility: none. Existing persisted localStorage may be read once by a Rust/front-end migration input only if an explicit source format is proven during plan; otherwise it is not authoritative and is not silently imported.
- Rollback: source-control rollback of the delivery; persisted database remains versioned/backed up.
- Evidence: before/after caller scans, migration tests and dual builds.

## Business Capability And Ownership Map

| Capability | Writable/data owner | Derived/UI owner | Forbidden owner |
| --- | --- | --- | --- |
| UI/locale persistence | Rust `ApplicationShellState` + `ShellRepository` | shell store snapshot; settings draft | localStorage, component, locale JSON |
| config archive/reset | Rust application + repository/file port | config panel draft/pending | browser Blob, page JSON parser |
| system metrics | Rust metrics runtime | shell store latest snapshot/status formatter | App timer, component history |
| room/connect | existing live room store/Rust live data | toolbar/status view model | shell duplicated refs |
| danmaku rate/audience | existing feed/room snapshots | status projection | status component counters |
| recording | existing recording store/Rust runtime | status projection | shell writable recording state |
| route | Vue Router | navigation active state | store duplicate current route |
| free/member visibility | compile channel + downstream member projection | pure navigation builder | template `v-if`, generic RBAC |
| settings drafts | owning panel | field validation display | Rust persisted state mutation before Save |
| UI visual projection | validated shell store snapshot | root projection lifecycle | individual pages/styles |
| notification open/focus | toolbar/root local UI | deferred content slot | shell notification history |

## Module Boundary Design

- Rust domain shell: values, defaults, validation, archive schema, locale/layout rules. No Tauri/sqlx/sysinfo.
- Rust application shell: use cases, state locks, transaction orchestration, event/metrics lifecycle through ports.
- Rust infrastructure shell: SQLite repository, archive files, sysinfo process metrics, OS directory opening.
- Rust IPC shell: request/result DTOs, serde mapping, command/event sink only.
- Frontend platform contracts: exact DTO maps and public command/event names.
- Frontend shell module: validation mirror, adapters, navigation/status/projection pure functions.
- Shell store/runtime: async ownership, latest generation, event listeners and projection cleanup.
- Vue root/components/settings: orchestration, drafts and rendering only.

## File And Directory Structure

```text
packages/bilibili-live-tool/
  src-tauri/src/
    domain/shell/{mod.rs,models.rs,rules.rs}
    application/shell/{mod.rs,ports.rs,state.rs,tests.rs}
    infrastructure/shell/{mod.rs,repository.rs,files.rs,metrics.rs,directory.rs,tests.rs}
    ipc/{shell.rs,shell_contract_test.rs,dto.rs}
    lib.rs
  src-tauri/capabilities/{main.json,effect.json}
  src-tauri/tauri.conf.json
  src/renderer/
    platform/desktop/contracts/shell.ts
    modules/shell/{adapters.ts,navigation.ts,projection.ts,formatters.ts,validation.ts,*.test.ts}
    stores/application-shell.ts
    composables/useApplicationShellRuntime.ts
    components/shell/{AppShellToolbar.vue,AppShellNavigation.vue,AppShellStatusBar.vue}
    views/settings/
      index.vue
      components/{ConfigManagementPanel.vue,LanguageSettings.vue,InterfaceSettingsPanel.vue,LayoutEditorDialog.vue}
    App.vue
    router/index.ts
    i18n-runtime.ts
    locales/{zh-CN.json,en-US.json}
```

Names are design targets, not permission to create every file mechanically. Co-locate tests with their owner and merge files if the complexity budget can be reduced without mixing responsibilities.

## Code Relationship And Dependency Direction

```text
Vue SFCs
  -> Pinia shell/room/feed/recording stores + pure shell view models
  -> platform/desktop adapters/contracts
  -> Tauri invoke/listen
  -> ipc/shell DTO + command sink
  -> application/shell state/use cases/ports
  -> domain/shell rules/models
  -> infrastructure shell repository/files/metrics/directory
```

- Domain imports standard library/serde-compatible value semantics only.
- Infrastructure implements application ports and may depend on sqlx/sysinfo/Tauri handle where adapter-specific.
- IPC depends inward on application/domain and maps errors outward.
- Frontend components depend inward on normalized props/store actions; transport never depends on stores/components.
- Completed feature stores do not import shell; root composes their snapshots into shell display.

## State Topology And Data Lifecycle

### Writable State

- Rust: persisted `UiPreferences`, `LocalePreferences`, archive metadata; metrics runtime latest sample.
- Shell store: `bootstrapStatus`, persisted normalized preferences, system metrics, operation pending/error, member projection input.
- Existing stores: room/feed/recording.
- Settings panels: local draft and dialog/checkbox focus state.
- Router: current route.

### Derived State

- navigation tree = build channel + member projection + localized labels + registered routes.
- status bar = room/feed/recording/system snapshots formatted through pure functions.
- form validity/dirty = draft vs persisted snapshot.
- Element Plus locale/root theme variables = validated locale/UI projection.

Derived values are computed, never written back as competing state.

### Lifecycle

1. Rust bootstrap creates shell repository, metrics port/runtime and loads defaults/saved values before UI commands.
2. Frontend runtime uses a generation token, calls initial reads, subscribes once to three shell events, applies projection.
3. Events map/validate before store replacement; invalid payload preserves last valid state and records structured error.
4. Settings clone snapshots into drafts; preview returns normalized value, save replaces persisted snapshot.
5. Root unmount stops listeners and projection observers; Rust window destroy cancels and joins metrics task.

## Dependency Rule Matrix

| From | Allowed | Forbidden | Public API |
| --- | --- | --- | --- |
| shell/components | Vue/Element Plus/icons, shell view types | transport, Rust DTO, stores, router permission logic | props/emits only |
| settings panels | view types, local validators, Element Plus | Tauri, sqlx, other feature stores | draft/action emits |
| App/settings containers | stores, runtime, router, components | raw DTO parsing, filesystem, member business flow | root composition |
| shell store | adapters, desktop invoke/listen, pure module | SFCs, localStorage, raw Tauri, feature page internals | explicit actions/snapshots |
| shell module | contract types and pure inputs | stores, router globals, browser storage/network | pure functions |
| desktop shell contract | base branded ids/errors | store/view imports | 13 commands/3 events |
| Rust domain shell | std/value types | Tauri/sqlx/sysinfo | models/rules/defaults |
| Rust application shell | domain + ports | Vue/IPC DTO/sqlx concrete | use cases/state |
| Rust infrastructure shell | application ports/domain | frontend/IPC | concrete adapters |
| Rust IPC shell | application/domain/DTO/error | SQL/business rules in command | commands/events |

Cycle risks: shell store must not import App/router; navigation pure module accepts route availability as input; completed room/media stores never import shell. Contract tests and scoped import scan enforce these directions.

## Extension Seams And Rejected Future-Proofing

- Real seams: `MemberShellProjection` input, notification content slot, `SystemMetricsPort`, `ConfigArchiveFilePort`, `DirectoryOpenerPort`, `ShellEventSink`.
- Member seam exists because build/state variation is source-backed; it is not a generic permission system.
- Notification slot exists because source owns the trigger while later module owns content; no registry/history API is added now.
- Config archive uses a closed enum/version, not provider registration. A new scope requires schema/enum code and tests.
- Reject: shell manager class, generic dashboard/status registry, route plugin registry, RBAC policy engine, dynamic settings panel registry, abstract config provider, service locator/DI container and event bus.
- Add a more general seam only after 3+ stable same-semantic production providers/callers or a new source-backed variation axis.

## Complexity Budget

- New production files: <=11 Rust and <=11 frontend; reuse/edit existing files where ownership remains clear.
- New public common commands/events: exactly <=13/<=3 from spec; no extra convenience surface.
- New frontend public store/composable entries: one store and one root runtime.
- Indirection: component -> store -> desktop -> IPC -> application -> port/repository; pure view mapping may add one adapter step, never a manager/facade pair.
- Root subscriptions: at most 3 shell listeners plus existing live/media runtimes; metrics has one Rust task, one latest snapshot, no history.
- Vue watchers: zero cross-store deep watchers; projection uses narrow `watch`/computed over validated preferences only.
- Mixed orchestration function limit: <=150 lines; use short named phases if exceeded.
- If budget exceeds, merge narrow domain files/tests or keep single-use helpers local before adding abstraction.

## Migration, Compatibility, And Cleanup Design

- Sequence: contracts/tests -> Rust domain/application/infrastructure -> IPC/capabilities -> frontend adapter/store/runtime -> shell components/root -> settings panels -> route/i18n -> delete old owners -> full verification.
- No dual runtime flag or Electron fallback.
- LocalStorage is not a lasting compatibility store. If its exact current shape passes typed validation, one migration read may be performed once and immediately removed/marked; otherwise defaults are used and the old value is left untouched for rollback.
- Browser Blob/object URL/anchor/pasted JSON config paths are deleted after native config tests pass.
- Static App menu/status/hard-coded strings and old broad config store are removed only after caller/import/generated scans.
- Rollback is source control plus versioned SQLite backup/transaction; no hidden compatibility facade.

## Locality And File-Extraction Decisions

| Candidate | Count | Decision | Boundary reason |
| --- | --- | --- | --- |
| room input handler | 1 | local in root/container | one workflow, no independent boundary |
| settings category selection | 1 | local in settings page | simple route UI state |
| shell runtime | 1 | separate composable | listener/projection lifecycle + focused tests |
| DTO adapters | 1 store boundary | separate feature adapter | native contract isolation + malformed-input tests |
| navigation builder | toolbar/router tests/store | feature-local pure file | permission semantics and independent matrix tests |
| formatters | 3+ shell fields/callers | feature-local pure file | same field semantics, multiple components/tests |
| UI validation | Rust mirror + form/store | feature-local TS file and Rust rules | cross-language contract-equivalence tests |
| config panel field handlers | 1 | local in panel | no reuse/lifecycle beyond component |
| layout reducer | dialog + tests | local to dialog unless independent test readability requires feature-local function | one interaction owner |

No `utils.ts`, `helpers.ts`, one-function module or file split solely for line count.

## Reuse Candidate Inventory

| Candidate | Locations/callers | Semantics/layer | Variations | In scope |
| --- | --- | --- | --- | --- |
| desktop error normalization | all existing stores | same transport infrastructure | none | reuse |
| root listener generation | live/media runtimes, new shell | same lifecycle technique, distinct events/stores | event count/start payload | evaluate reuse shape |
| config key/value repository | base config and media records | same SQLite table, different typed schemas | validation/schema/transaction | reuse repository pool, keep typed rules separate |
| room/recording status | existing stores and shell display | same source truth, display projection differs | units/labels/null | consume existing, no extraction of domain owner |
| locale projection | vue-i18n, Element Plus, route titles | same locale value | target adapter | extract one UI-domain projection owner |
| settings errors/pending | current panels | syntactically similar but different operations | rollback/grants/preview | keep separate |
| navigation/route capability | App menu/router | same access semantics | build/member/current route | one feature-local builder |
| number formatting | audience/rate/memory/CPU | different business units/precision | unit/null/precision | keep domain formatters separate |

## Anti-DRY Abstraction Matrix

| Candidate | Sites | Stability | Variation count | Decision | Rationale |
| --- | ---: | --- | ---: | --- | --- |
| desktop error normalization | 7+ | high | 1 | reuse existing | exact shared infrastructure semantics |
| listener generation pattern | 3 roots | high | 3 | reuse local lifecycle pattern, no generic hook | same cleanup shape but distinct stores/events; generic listener registry adds weaker typing |
| locale projection | 3 targets | high | 2 | extract `applyLocaleProjection` | one value/source with stable target adapters |
| UI projection | 4+ root consumers | high | 3 | extract `applyUiProjection` | same validated preferences and rollback owner |
| config typed validation | Rust/TS | high | 2 languages | Rust authority + TS mirror, not shared runtime code | cross-language contract, not same executable layer |
| operation pending/error | 3 panels | medium | >3 | keep separate | distinct grant/transaction/preview semantics |
| status number formatting | 4 fields | high | >3 | separate named formatters | meaning/units/precision/null differ despite numeric input |
| member/free route filtering | 2 consumers + tests | high | 3 | one navigation builder | same permission semantics and route tree |

## Commonality Classification

- Technical/infrastructure: desktop errors, lifecycle generation technique, metrics port, archive file/directory ports.
- Business/domain: navigation capability projection, UI/locale validation, config scope/archive schema.
- UI/design system: root locale/UI projection, shell region components.
- Configuration/constants: PRD defaults and safe layout bounds under domain shell owner.
- No broad shared-utils directory is introduced.

## Shared Abstraction Decisions

- Reuse `normalizeDesktopError`, `invokeDesktop/listenDesktop`, branded grant/room ids and existing Pinia/root-runtime conventions unchanged.
- Add feature-local `buildNavigationTree(input)` returning readonly descriptors; inputs contain build channel, optional validated member projection and registered routes. No globals/side effects. Used by store/root/router tests. JSDoc `@see` points to PRD 8 matrix and one `@example` covers free/expired states.
- Add `applyLocaleProjection`/`applyUiProjection` in one projection module because 3+ root targets share the exact value and rollback lifecycle. Environment setters are injected, so tests use fakes without document/i18n globals.
- Keep number/status formatters separately named in one shell formatter module; do not create `formatValue`.
- Keep config/locale/interface pending and drafts in their panels/store actions; no generic form action hook.

## Dependency Injection And Strategy Boundaries

- `SystemMetricsPort.sample()` is injected into Rust runtime; fake/unavailable/sysinfo adapters contain environment variation.
- `ConfigArchiveFilePort` receives resolved grant/path internally; application never sees browser files.
- `DirectoryOpenerPort.open_config_directory()` owns platform-specific `explorer/open/xdg-open` invocation with no shell interpolation.
- `ShellEventSink` receives validated snapshots; Tauri adapter owns target emission.
- Frontend projection functions receive setters for locale, Element Plus locale, root attributes and CSS variables; pure mapping is testable without browser globals.
- Build/member variation is typed data passed to navigation builder, not a strategy class.

## Responsibility Split

- Rust domain: validation/default/archive semantics.
- Rust application: atomic use cases, saved/preview state, metrics lifecycle, event order.
- Rust infrastructure: SQLite/files/sysinfo/OS process.
- Rust IPC: boundary validation/mapping and targeted events.
- Desktop adapter: runtime DTO shape validation and view-model conversion.
- Pinia: frontend request/event/operation state.
- Root composable: listener/projection lifecycle.
- App/settings: composition and intent routing.
- Components: accessible presentation and local UI lifecycle only.

## Function Design And Public Entrypoints

- Rust use cases: `status`, `config_overview`, `open_config_directory`, `export_config`, `import_config`, `reset_config`, `locale`, `set_locale_auto`, `save_locale`, `ui`, `preview_ui`, `save_ui`, `reset_ui`, `shutdown`.
- Domain functions: `default_ui_preferences`, `validate_ui_preferences`, `resolve_locale`, `validate_layout`, `validate_config_archive`.
- Frontend store actions mirror use cases and always normalize errors/finally states.
- Pure frontend entries: `adaptShellStatus`, `adaptUiPreferences`, `adaptLocalePreferences`, `buildNavigationTree`, `buildStatusBarView`, `apply*Projection` mapping phase.
- Components emit user intentions; root/settings call store actions. No exported per-button helper.

## Functional Boundaries And Side-Effect Model

- Pure: defaults/validation/serde mapping/navigation/status formatting/layout reducer/view payload building.
- Immutable: DTOs, store snapshots, props, route descriptors and validators' drafts.
- Effects: repository/file/OS/system metrics/window command in Rust; invoke/listen in desktop transport; store writes/actions; DOM/i18n application in root projection lifecycle; focus/dialog in component.
- Composition uses short named functions and explicit async steps. No functional library, currying or generic reducer framework.

## State Ownership And Data Flow

```text
Rust saved prefs/archive/metrics
  -> serde DTO
  -> TS adapter validation
  -> shell store immutable snapshot
  -> root pure projection / component view props

settings snapshot
  -> panel draft
  -> pure validator/payload
  -> store action
  -> Rust validate + transaction
  -> saved DTO/event
  -> store replacement + root projection
```

- Grant flow: dialog -> grant id -> config action -> file port -> revoke in frontend `finally` and server cleanup on failure.
- Member flow: member feature owner -> validated projection -> navigation builder; free build has no producer/import.
- Route flow: navigation intent -> router -> active descriptor; invalidated route -> replace connection.

## Data Structures And Type Strategy

- Rust newtypes/enums for config scope, locale, theme/font/layout mode and version; floats validated finite/in range.
- TS readonly interfaces mirror camelCase DTOs; view types distinguish formatted labels from raw bytes/percent/timestamps.
- `null` is explicit unavailable/absence; missing fields are invalid except optional base contract values.
- `FourRegionLayoutV1` is versioned and validates all fields together. Technical bounds: navigation 176-280px, toolbar 48-64px, status 28-40px; these preserve fixed region semantics/minimum viewport and are not new user-visible presets.
- Font mapping is fixed under domain constants: small 12px, medium 14px, large 16px; no arbitrary number survives the adapter.
- Opacity validates finite `[0,1]` exactly as the approved spec contract; default remains 0.95. Projection tests cover the zero endpoint without allowing controls or content to overlap.
- Archive uses tagged/versioned typed sections and never serializes database schema or secrets directly.

## Contract And Adapter Boundaries

- Add `ShellCommandRequestMap/ResultMap/EventPayloadMap` and merge into desktop contract index/transport generics.
- Rust `SHELL_COMMAND_NAMES/SHELL_EVENT_NAMES` are mirrored by TS constants and exact count/order tests.
- DTO adapter validates every enum, finite number, bounds, date and nullable semantic before commit.
- API dates are epoch milliseconds; display adapter formats backup time to locale seconds. Metrics memory is bytes, CPU percent 0-100.
- Path DTO contains grant id/display path; config commands accept only grant id.
- Error adapter retains code/details/retryable and maps to i18n keys outside page templates.

## Pattern Decisions And Rejected Alternatives

- Decision: reuse Level 2 ports/adapters/facade/observer/state-owner patterns and Level 1 typed records/pure reducers.
- Adapter/facade isolates Rust serde/Tauri. Observer is justified for bounded system/preference updates with one root owner. Explicit state unions model bootstrap/operations/member projection.
- Direct match/record handles closed navigation/config variants; no Strategy/factory required.
- Reject manager classes, DI container, provider registries, command objects, global event bus, RBAC engine, generic settings schema renderer and service locator.
- Remove a port if it ends with no fake/unavailable/production boundary or only forwards one method without isolating environment effects.

## Readability And Maintenance Guardrails

- No mixed function over 150 lines; command handlers validate/map/delegate only.
- No `ShellManager`, `SettingsService` mega-owner, catch-all utility, optional mega DTO or route-string scattering.
- Domain defaults/bounds have one narrow owner and tests; template values stay local.
- Comments explain transaction/order/security/platform constraints only.
- Every new public helper documents representative use with `@see`/`@example`; private single-use helpers stay local.
- Generated declarations are reviewed; no deleted component/import remains.

## Architecture Risks

- Config archive scope can expand into final-module work. Keep fixed common V1 and explicit downstream handoff.
- Free frontend tree shaking can fail if build branching is indirect. Use direct compile-time expression and inspect emitted chunks/text.
- `sysinfo` CPU requires a prior sample/interval. Runtime owns warm-up and never emits fabricated zero as ready data.
- Theme/opacity projection can flash during bootstrap. Apply validated defaults/saved data before revealing shell, without a new splash product surface.
- Layout bounds may conflict with browser zoom. Test logical pixels at min size/high zoom; fallback default rather than clamp silently after corruption.
- OS open-directory command varies by platform; arguments are passed directly and failure is structured/retryable as appropriate.

## Open Architecture Questions

- No blocking architecture question remains.
- Plan must confirm `sysinfo 0.30.13` compiles with locked targets and disabled default features; incompatibility keeps metrics nullable/unavailable rather than switching libraries ad hoc.
- Notification content and member state producer remain downstream seams with explicit owners.
- Full cross-scope archive encryption/migration remains a final resilience extension; common current scopes must be safe and transactionally complete now.

## Architecture Exit Decision

- The design preserves every spec behavior/acceptance criterion and page-design boundary.
- Ownership, state/data topology, dependency matrix, contracts, reuse decisions, component boundaries, budgets, cleanup and rollback are fixed for planning.
- No speculative manager/registry/dual path is approved.
- Ready for `plan`; preserve `state.json.loop` unchanged.
