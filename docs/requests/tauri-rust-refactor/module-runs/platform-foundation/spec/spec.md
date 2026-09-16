# Platform Foundation Engineering Spec

## Delivery Unit Identifier

- Request: `tauri-rust-refactor`
- Module: `platform-foundation`
- Sequence: 1 of 6
- Source module: `requirements/modules/platform-foundation.md`
- Architecture route: `architecture-design_required: true`
- Page design: not required; this unit adds no product page.

## Background And Goals

This unit replaces the Electron desktop boundary with a Tauri v2 runtime while preserving the existing Vue 3, TypeScript, Vite, Pinia, Vue Router and Element Plus application. It establishes the Rust ownership, typed command/event boundary, SQLite schema, configuration foundation, path grants, capabilities and compile-time free/member separation required by every later module.

The practical goal is not merely to add a `src-tauri` folder. Completion requires one executable desktop boundary whose free build cannot register member commands, whose WebView cannot access arbitrary files, databases or secrets, and whose downstream business modules can be migrated without reintroducing Electron.

## In Scope

- Official Tauri v2 backend initialization inside `packages/bilibili-live-tool` while retaining the existing Vue/Vite frontend.
- Rust crate, Tauri configuration, application state and five-layer module skeleton.
- A typed TypeScript desktop gateway backed by `@tauri-apps/api`; no renderer caller may invoke Electron directly after its owning module migrates.
- Rust command guard and stable serialized error envelope.
- SQLite initialization with WAL, the PRD's explicit 21-table schema, seven indexes, schema state and repository-only SQL access.
- Configuration repository primitives and sensitive-value storage boundary; user-facing configuration flows remain module 6.
- Dialog-originated path grant representation and Rust-owned path validation.
- Main-window and effect-window capability files, with the effect window granted no commands.
- Compile-time `free` and `member` Cargo features plus matching Tauri/Vite build selection.
- Migration entrypoint, backup/transaction/rollback contract and non-destructive Electron data discovery. Detailed field migration is completed with module 6.
- Build, type, Rust, schema, capability and isolation tests required to prove the foundation.

## Out Of Scope

- Product pages or changes to visible navigation and layout.
- Live room connection, event parsing, moderation, media, AI, membership purchase, plugins and updater behavior.
- Production OAuth, payment, Bilibili Cookie moderation, module CDN or update service implementations.
- Bundling real FFmpeg/whisper binaries before approved artifacts and license files exist.
- Any manual, automated or indirect danmaku sending capability.
- Editing the source PRD or deleting old Electron files before all direct callers have migrated.

## Trigger And Start Conditions

- The current PRD is the product authority and remains read-only.
- The repository contains an Electron/Vue application but no Tauri crate or Cargo workspace.
- Node and pnpm are available. Rust stable and application dependencies must be installed before implementation verification.
- Existing failed root tests caused by stale `src/` imports are baseline defects; touched/migrated tests must be repaired, but unrelated failures cannot be represented as new regressions.

## Requirement Split Summary

This module owns only the platform mechanisms required by modules 2-6. Business behavior stays in the respective module. Shared data/statistics tables remain present in both builds. Member-only Rust modules and commands exist only under the member build feature; runtime membership gating is added in module 5.

## Source Grounding And Traceability

| Spec item | Grounding | Product semantic | Evidence | Downstream obligation |
| --- | --- | --- | --- | --- |
| Tauri + Rust + Vue WebView + Sidecar architecture | source-backed | yes | PRD title, 2.2, 3.1-3.3 | Scaffold and layer boundaries must match. |
| Vue only crosses desktop boundary through invoke/listen | source-backed | yes | PRD 2.2, 10.1 | Add typed gateway; prohibit direct I/O. |
| SQLite with WAL and Rust repository ownership | source-backed | yes | PRD 3.1, 12 | Create migrations and repository boundary. |
| Explicit 21-table schema and seven indexes | source-backed | yes | PRD 12.1-12.2 explicit SQL | Migration test must enumerate exact objects. |
| Two installers share code, identifier and data directory | source-backed | yes | PRD 4.1, 4.5 | Build profiles cannot fork persistence. |
| Free binary has no membership auth/download/member commands | source-backed | yes | PRD 4.2, 5.1, 10.4 | Compile-time exclusion and registry test. |
| Member binary initially contains free base plus minimal membership shell capability | source-backed + source-derived | yes | PRD 4.3 requires pre-payment login/member center; PRD 5.2 separately describes downloadable member resources | Compile base member shell, defer downloadable resources to module 5. |
| Rust validates types, ranges and path grants | source-backed | yes | PRD 10.1-10.3 | Central validator and path-grant tests. |
| Effect window receives events and has no command permission | source-backed | yes | PRD 10.5 | Dedicated capability with empty permissions. |
| Preserve Vue/Vite frontend and initialize only Tauri backend | code-fact-backed + confirmed technical decision | no | Existing app plus official Tauri v2 existing-frontend workflow | Use `tauri init`, not a second Vue starter. |
| Do not delete Electron path until caller closure is migrated | code-fact-backed | no | `artifacts/code-context.md` | Each later module owns its compatibility cleanup. |

### Closed-World Product Content Allowlist

This unit may expose no new product-visible page, copy, field, control, navigation target or analytics event. It may establish only these technical states for downstream consumption: `ready`, `degraded`, `migration_required`, `migration_failed`, `storage_unavailable`, `invalid_input`, `permission_denied`, `unsupported_build`, and `external_service_unavailable`. Any other visible behavior requires a later source-backed module spec.

Permitted desktop capabilities are: application bootstrap/status, typed error reporting, repository-backed configuration primitives, user-authorized path grants, and build-channel introspection needed to render later source-backed variants. There is no send-danmaku command, payload, event or transport method.

## Workflow Efficiency Profile

- Profile: `S3 broad`.
- Reason: cross-framework, cross-language migration with persistence, security and build isolation has repository-wide blast radius.
- Artifact density: full.
- Downstream context: full dependency closure for package/build/runtime/gateway/database/capabilities and direct tests; business modules remain out of the implementation closure.
- Upgrade condition: none currently; any newly discovered production contract that changes product semantics returns to requirement analysis/spec.

## Self-Healing Completion Bar

- Requirement consistency: every implemented command, event, table, permission and feature maps to this spec or the PRD; no product behavior is invented.
- Business rules: free/member compile separation, shared data directory, path grant validation and repository-only persistence are verified.
- Static checks: TypeScript type-check/lint/build and Rust fmt/clippy/test/build pass for both feature sets.
- Runtime safety: initialization, database open/migration failure, invalid input, poisoned state, event cleanup and unavailable external resources return explicit errors without panic or silent success.
- Organization: imports, commands, state, repositories, migrations and capability files have one owner; no renderer-side filesystem/database/secret access is introduced.
- Maintainability: precise names, small command adapters, pure validators, immutable DTO handling and explicit side effects.
- Debt blocker: Cargo/Rust and package dependencies must be installable; generated scaffold placeholder commands, debug logging and unused Electron bridge calls in migrated foundation code must be removed.
- Completion requires `self-healing loop compliance: pass`, `self-healing loop assessment: pass`, and remaining blockers `none` for this unit.

## User Intent Contract

- Literal request: refactor the project according to the named PRD.
- Practical goal: obtain a runnable Tauri/Rust product base, not documentation or a cosmetic Electron wrapper.
- Success: later modules can implement product functions exclusively through typed Tauri contracts and Rust-owned I/O; both build channels compile and enforce their capability boundary.
- Forbidden interpretations: keeping Electron as the production host; hiding member UI while leaving member commands in a free binary; exposing arbitrary file APIs; adding danmaku sending; pretending missing external services work.
- Acceptable approaches: incremental caller migration behind one typed gateway, schema-first SQLite migrations, compile features and deterministic unavailable adapters for missing production contracts.
- Review checks: inspect production dependencies/source, generated binaries/command registry, capabilities, migrations and all gateway call sites rather than relying on UI appearance.

## User Flow

There is no new direct user flow. The platform lifecycle is:

1. Tauri starts and resolves the shared application data directory.
2. Rust opens SQLite, applies PRD migrations in a transaction, enables WAL and creates managed directories.
3. Rust reports a typed bootstrap state to the main WebView.
4. The main WebView subscribes through the typed gateway; the effect window may only subscribe to its event stream.
5. On recoverable initialization failure, the application exposes a degraded typed state and preserves existing data for retry/recovery.

## Page And Module Design

- No page or visual layout is created in this unit.
- `src-tauri/src/lib.rs` composes plugins, managed state, commands and build-feature registries.
- IPC adapters validate/deserialize and delegate; application services orchestrate; domain modules own pure types/rules; infrastructure owns SQLite, filesystem, keyring, sidecar and network adapters.
- The TypeScript gateway is the only renderer import point for Tauri invoke/listen. It contains transport typing/error mapping, not business rules.

## UI / UX Design Contract

Not applicable: no user-facing layout or interaction is introduced. Downstream pages may render only the allowlisted bootstrap/build states and must define their own page-design contract.

## Frontend Styling Constraints

Not applicable: no authored style change is allowed in this unit.

## Field Display Semantics And Visual States

Not applicable: no business-visible field is added. Technical DTO fields are not authorized for direct template rendering by this spec.

## Function-Complete Behavior Breakdown

### Bootstrap

- Resolve Tauri-managed app data/config/cache/log/resource paths; callers cannot supply them.
- Initialize directories and database once through managed application state.
- Return build channel, schema version and readiness/degraded status in a typed response.
- Concurrent bootstrap calls are idempotent and share one initialization result.
- Failure retains the original database/config data and returns a stable error code with non-sensitive context.

### Command And Event Gateway

- TypeScript defines exact request/result/event maps and generic `invokeDesktop`/`listenDesktop` functions constrained to those maps.
- Rust command names and serde DTO field names are the authority; TypeScript mirrors them without ad hoc renaming.
- Unknown commands are unavailable by construction; errors normalize to `{ code, message, details?, retryable }`.
- Listener registration returns an unsubscribe function; Vue lifecycle owners must call it.
- No raw `window.electronAPI`, generic arbitrary IPC channel or arbitrary filesystem method is added.

### Validation

- `RoomId` and `UserId`: ASCII numeric strings, length 1-20; whitespace is not retained and normalized-empty is invalid.
- Username: remove control characters, then trim; normalized length 1-50; internal spaces remain.
- Sensitive word: remove control characters, then trim; normalized length 1-100; internal spaces remain.
- Volume: finite numeric value in inclusive range 0-1.
- Threshold: integer greater than or equal to 0.
- Time range, plan id, module name and template variables: exhaustive allowlists owned by their domain modules; unknown values fail closed.
- Validation occurs at every Rust command boundary even when TypeScript types already constrain input.

### SQLite And Configuration Foundation

- Open one application database under the shared app data directory and set journal mode to WAL.
- Apply ordered, idempotent SQL migrations; `t_schema_state` records completion.
- Create all 21 PRD tables and seven PRD indexes using the explicit SQL list as authority.
- SQL is callable only through infrastructure repositories; no SQL string crosses IPC.
- Sensitive account/member/API token values must pass through the approved secure-store adapter before later modules persist them; plaintext reads are never exposed to the WebView.
- Configuration primitives use explicit scope/key/value DTOs internally; the product-facing import/export/reset workflow is deferred.

### Path Grants And Capabilities

- A user path becomes usable only after selection through the Tauri dialog flow and Rust validation.
- Business commands accept a stored grant identifier or domain-owned configured path; they never accept an arbitrary renderer-composed path.
- Grants are constrained to their purpose and revoked/invalid paths fail with `permission_denied` or `invalid_input`.
- Rust exclusively owns `modules/` and other application-managed directories.
- Main window capability contains only explicitly used permissions. Effect-window capability matches `effect-*` and has an empty permission list.

### Build Channels

- Cargo features are mutually intentional: default/free build compiles shared/free foundation only; member build compiles member registry/ports in addition.
- Free command registration excludes exactly `membership_purchase`, `ai_generate`, `moderate_mute`, `asr_start`, `membership_orders`, `module_update` and any future member-only command.
- Free frontend build tree-shakes/excludes member routes/resources and has no member download/auth client.
- Member build uses the same application identifier/data directory and adds only the minimal pre-activation membership shell contract plus compile-time member core; downloaded feature resources remain absent initially.
- CI/build tests inspect command registries and compiled feature behavior, not only visual route filtering.

### Migration Boundary

- Discover legacy Electron data without mutating it.
- Before an actual conversion, verify free space and create a backup outside the active transaction target.
- Apply schema/data migration transactionally and idempotently; failure rolls back and leaves the source intact.
- This unit implements the migration runner and schema state; module 6 supplies full legacy-field mappings, retention and backup pruning.

## Expert Frontend Engineering Constraints

- State owner: Rust managed state owns bootstrap/storage/build capability; Pinia or a composable may hold a readonly projection, never a second writable authority.
- Data lifecycle: serde DTO -> typed gateway -> readonly view model; reverse requests validate in TypeScript for UX and again in Rust for authority.
- Async correctness: one bootstrap promise, explicit retry, no overlapping migrations, stale listener callbacks removed on unmount.
- Interaction resilience: later UI must expose actionable degraded errors; this unit does not invent copy.
- Performance: no polling for immutable build/schema state; avoid broad reactive event fan-out. Foundation must not start Sidecars during cold start.
- Evolution safety: Electron and Tauri cannot remain two production runtimes after caller migration; compatibility mappings live only in the single gateway with a removal trigger.
- Testability: pure validators/schema inspection and feature registry are deterministic; database and Tauri boundary use focused integration tests.

## Frontend Architecture Design Routing Inputs

- `architecture-design_required: true`.
- Immutable behavior: Rust owns I/O; Vue uses typed invoke/listen; SQLite/WAL/21 tables; path grants; empty effect capability; shared identifier/data; free compile exclusion; no danmaku send.
- Architecture must decide concrete module ownership, state topology, command/event DTO locations, repository traits, migration runner, feature registry, gateway public API, compatibility removal sequence and test seams.
- It must consider existing Electron callers and logger cycles from `artifacts/code-context.md`.
- It must not change product scope, validation semantics, build visibility or persistence rules. Any such need rolls back to spec.
- Later verification must record `frontend architecture quality compliance: pass`.

## Production Code Quality Constraints

- Type first: Rust newtypes for confusable identifiers and validated paths; serde enums for closed sets; TypeScript request/result/event maps. No unapproved `any`, broad channel strings or unchecked casts.
- Fail fast: invalid input, unsupported feature, unavailable external contract, database failure and permission denial are distinct errors. No panic/unwrap on runtime input and no swallowed promise.
- Nulls: optional values are optional only when the PRD permits absence; `null`/`Option::None` cannot stand in for invalid required data.
- Maintainability: no memoization/cache beyond one-time bootstrap/database handles without measured need.
- Pure rules: validators, error mapping and feature registry derivation are pure functions where possible; lifecycle resources may use structs managed by Tauri.
- Naming follows domain verbs and boolean prefixes; command functions describe the action rather than use vague `process`/`manage` names.
- Constants live in the narrowest owner: validation limits in validation/domain, schema in migrations, command allowlists in feature registry.

## Human Review Readiness Constraints

- Allowed production scope: application package manifest/scripts/Vite config, new `src-tauri`, one typed renderer gateway and directly affected declarations/tests.
- Electron business modules and unrelated Vue pages are not rewritten in this unit.
- Generated Tauri scaffold churn must be separated conceptually from manual domain code in the changelog.
- Review-sensitive risks: lockfile size, Cargo dependency surface, shared identifier, destructive migration, capability breadth, feature leakage and dual-runtime ambiguity.
- Every changed hunk must map to a plan task, scaffold output, compatibility requirement or test adaptation. Remove scaffold placeholders, debug logs, dead imports and unrelated formatting.
- Evidence includes both builds, Rust fmt/clippy/test, schema enumeration, TypeScript checks, capability inspection and a runtime smoke check.

## Functional-Programming Constraints

- Pure: input normalization/validation, error-code mapping, build capability derivation, DTO transformation and migration preflight decisions.
- Immutable inputs: serde request DTOs, TypeScript request objects, configuration snapshots and repository results.
- Side effects only in Tauri lifecycle setup, command/application service entrypoints and infrastructure adapters.
- Semantic normalization belongs in Rust validators or the TypeScript gateway mapper, never in Vue templates/watchers.
- Derived readiness/capability flags stay derived. No new functional utility library is permitted.

## Architecture Reuse And Shared Ownership Constraints

| Candidate | Existing facts | Anti-DRY result | Required design decision |
| --- | --- | --- | --- |
| Desktop transport | Many services use `window.electronAPI`; one IPC helper exists | Same infrastructure semantic, 10+ production callers, stable boundary, transport varies | Replace with one shared typed Tauri gateway; do not copy invoke wrappers. |
| Validation | Room/user/sensitive/volume rules appear across domains | Same domain constraints, multiple future commands, stable PRD limits | Rust validation owner with domain-specific functions/newtypes; TypeScript UX mirrors only. |
| Persistence | Renderer service plus Electron store and module-specific state | Same infrastructure concern but incompatible semantics | New Rust repository boundary; do not adapt electron-store as target. |
| Error mapping | Current IPC modules return inconsistent errors | Same cross-cutting boundary, all commands | One stable error envelope and code mapping. |
| Business repositories | Future modules have distinct aggregates | Different domain semantics despite common SQL mechanism | Shared connection/migration mechanism, separate domain repositories. |

Shared APIs require doc comments with a representative example or `@see`/Rust intra-doc reference. Architecture must avoid making shared foundation depend on feature-private entities or creating new import cycles.

## Locality And File-Extraction Constraints

- The desktop gateway is a separate file because it is a contract/transport boundary with many real callers and an independent test surface.
- Rust validators may be a cohesive domain module because multiple commands share them and they require focused tests.
- Migrations, database connection, path grants and feature registry each warrant separate infrastructure/boundary modules because they own independent side effects or compile contracts.
- One-call bootstrap formatting or local conversion remains inside its owner; no catch-all `utils.ts`, `helpers.ts`, `Manager` wrapper or one-function module.

## Vue Component Extraction Constraints

Not applicable: no Vue component is added or split.

## Design Constraints

- Dependency direction: UI -> typed gateway -> IPC adapter -> application/domain ports -> infrastructure implementations.
- Rust infrastructure may implement domain/application ports; domain cannot import Tauri, SQLx, filesystem or HTTP.
- Commands are thin validation/error adapters, not repositories or workflows.
- No public generic filesystem, database, shell or arbitrary IPC command.
- No duplicate writable source of build channel, schema state or granted path state.
- Compatibility code must have an explicit caller-migrated cleanup trigger.

## Project Bootstrap And Scaffold Decision

- Source signal: PRD requires a Tauri framework and Rust core; repository facts show an existing Vue/Vite frontend and no Tauri crate.
- Discovery checked: package scripts, pnpm workspace, sibling packages, existing `src-tauri`/Cargo files and official Tauri v2 existing-frontend initialization.
- Selected: Tauri v2 CLI backend initialization in `packages/bilibili-live-tool`, configured to use existing `pnpm dev`/Vite URL and dist output.
- Rejected: `create-tauri-app` full Vue starter because it duplicates the current frontend and would discard/multiply existing routes, stores and components. No repository-local Tauri starter or sibling app exists.
- Approved deviations: replace scaffold demo command/UI; add required plugins, capabilities, Cargo features, database migrations and shared identifier; preserve monorepo scripts/aliases.
- Fixed choices: one frontend, one Tauri backend, Rust stable, Tauri v2, existing Vue toolchain. Plan/execute may select exact compatible crate patch versions from the generated lockfile but cannot re-decide the architecture.

## Change Axes And Pattern Decision

- Signals: legacy/new contract adaptation, build-feature selection, command/event dispatch, side-effect coordination, state-dependent readiness and infrastructure replacement.
- Depth: Level 3 structural migration, with Level 2 reuse of Tauri/Vue conventions.
- Decision: introduce a narrow Ports-and-Adapters layering and typed Facade/Adapter at the WebView boundary; use a typed record/discriminated union for command/event/error contracts and Cargo features for build variants.
- Lightest fit: one gateway prevents dozens of renderer callers from knowing transport details; thin Rust command adapters keep Tauri out of domain/application rules.
- Rejected: direct replacement in every service would duplicate error/listener logic; a generic arbitrary channel bridge recreates unsafe Electron IPC; class-heavy manager/factory hierarchies add ceremony without a runtime polymorphism need.
- Overbuilt signal: one-method traits with one stable implementation, files with a single trivial caller, or abstractions that merely rename Tauri/SQLx APIs must be collapsed during review.

## Code Context And Impact Assumptions

- Governed TypeScript config: `packages/bilibili-live-tool/tsconfig.json`, strict mode, ESNext modules, bundler resolution and `@/* -> src/renderer/*`.
- Existing Electron production boundary: `src/main`, `src/preload`, renderer `utils/ipc.ts` and direct `window.electronAPI` callers enumerated in `artifacts/code-context.md`.
- Existing persistence uses `electron-store`; it is a legacy source, not a SQLite contract.
- Logger circular dependencies are recorded baseline and expected to disappear or be independently resolved when Electron logging is migrated; this unit must not expand them.
- Rust/Cargo was absent at discovery; toolchain installation is an implementation prerequisite, not a product decision.

## API And Data Contracts

### Desktop Error

```ts
interface DesktopError {
  code: string
  message: string
  details?: Record<string, string>
  retryable: boolean
}
```

`message` is diagnostic fallback text, not final localized product copy. Secrets, tokens and unrestricted paths are forbidden in `details`.

### Foundation Commands

| Command | Request | Result | Ownership |
| --- | --- | --- | --- |
| `app_bootstrap` | no payload | build channel, schema version, readiness/degraded code | Rust application bootstrap |
| `app_status` | no payload | current non-secret foundation status | Rust managed state |
| `config_get` | validated key/scope | optional non-secret value and update time | Rust config repository |
| `config_set` | validated key/scope/value | persisted revision | Rust config repository |
| `path_select` | closed purpose enum | opaque grant id plus non-sensitive display path | Tauri dialog + Rust path guard |
| `path_revoke` | grant id | success | Rust path guard |

These are technical foundation contracts. Product-facing settings behavior is not approved until the relevant module spec. Command names may be narrowed during architecture design only if semantics and a one-to-one trace remain unchanged.

### Foundation Events

| Event | Payload | Semantics |
| --- | --- | --- |
| `app_status_changed` | readiness/degraded code, retryability | Foundation state changed. |
| `migration_progress` | stage enum and bounded percentage | Migration runner progress; no raw path/data. |

### Database

The authoritative tables and indexes are the exact SQL in PRD 12.1-12.2. The narrative count of 17 is treated as a documentation defect because the explicit schema contains 21 tables. Migrations must not silently rename fields, relax nullability or replace PRD defaults. Secrets in account/member/config records require an encrypted/secure-store reference strategy before real values are written.

## Frontend / Server Responsibility Boundary

- Vue owns typed requests, subscriptions and later visual state; it owns no filesystem, SQL, key, migration or feature authorization decision.
- Rust owns validation, application state, repositories, path grants, secure storage, capability/command registration and build features.
- External services own OAuth/payment/token issuance/manifests/update/Cookie APIs. This module provides no fake success path.
- Missing external contracts do not block local foundation; their ports return `external_service_unavailable` until module-specific integration is configured.

## Context And Dependency Sources

- PRD sections 2-5, 10 and 12.
- `request.md`, `artifacts/prd-snapshot.md`, `artifacts/code-context.md`, `requirements/requirement-map.md`, `requirements/modules/platform-foundation.md`.
- Official Tauri v2 existing-frontend scaffold and capability conventions.
- Current package manifests, Vite config, TypeScript configs and Electron IPC caller graph.

## Edge Cases

- Database missing, locked, corrupt, newer than supported or migration interrupted.
- Shared data directory already contains legacy Electron data or a member-created database.
- Two bootstrap calls or processes contend for migration.
- Dialog result is cancelled, path disappears, symlink escapes scope or a stored grant is stale.
- Free frontend accidentally requests a member command; the command must be absent/unsupported, never silently allowed.
- Member build starts without external service configuration or downloaded modules; free base remains ready.
- Capability wildcard accidentally grants effect/main window more access than declared.
- Rust emits an event after a Vue owner unmounts; unsubscribe prevents stale mutation.

## Acceptance Criteria

1. `pnpm tauri dev` launches the retained Vue application through Tauri with default app initialization and without starting Electron.
2. Production package dependencies/scripts/source for the migrated foundation no longer require Electron; no new direct `window.electronAPI` call is introduced.
3. Rust fmt, clippy with warnings denied, tests and builds pass for free and member feature sets.
4. TypeScript type-check, lint, focused tests and Vite production build pass after dependencies are installed.
5. A clean database enables WAL and contains exactly the PRD's 21 tables and seven named indexes.
6. Migration tests prove idempotency and rollback, and original legacy input remains intact on failure.
7. Invalid IDs, username/word values, volume, threshold, enum values and stale/escaped path grants fail with typed errors.
8. The renderer has no arbitrary filesystem/database/secret API. Capability inspection proves `effect-*` has no command permission.
9. Free build command registry excludes all six PRD member commands and contains no membership authentication/module-download implementation; member build can compile its gated registry without enabling unavailable integrations.
10. Both build channels use the same application identifier and data-directory contract.
11. No command, event, type, button or transport capable of sending danmaku exists.
12. Cold bootstrap does not start Sidecars; runtime smoke evidence records bootstrap success or an actionable degraded state without panic.

## Human Review And Handoff

- Changelog maps generated scaffold, manual foundation code, dependency changes and tests to plan task IDs.
- Reviewer receives command outputs for both feature builds, schema/capability tests, TypeScript checks and smoke startup.
- Any unrun check records the exact blocker and substitute evidence; external production integrations are explicitly marked unavailable.
- Module may hand off only after verify/review record architecture, code quality, expert frontend and source-grounding assessments as pass.

## Risks

- Rust toolchain/native prerequisites can delay local builds.
- Tauri plugin versions and Windows WebView/runtime requirements may affect scaffold integration.
- The PRD's shared identifier for two installers makes packaging metadata/channel distinction sensitive.
- A full 21-table first migration creates breadth; schema tests must prevent omission or accidental reinterpretation.
- Incremental migration temporarily leaves Electron code in the repository. The gateway and per-module cleanup triggers must prevent permanent dual-runtime drift.
- Production Sidecars, signatures and service endpoints remain externally blocked and cannot be certified in this unit.
