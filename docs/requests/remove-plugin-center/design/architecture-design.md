# Architecture Design: Remove Plugin Center

## Delivery Unit And Objective

`remove-plugin-center`. Retire the business plugin capability as a closed dependency graph while preserving the existing architecture and behavior of update, about, resilience, shell configuration, and Tauri framework integrations.

## Scope, Inputs, And Assumptions

Inputs are the approved spec, request, source snapshot, and scoped code-context inventory. This is an existing application deletion, so scaffold/bootstrap design is not applicable. The architecture may remove plugin-owned nodes and branches but may not redesign surviving capabilities.

## Architecture Decision Records

### ADR-1: Dependency-Closure Removal

- Problem: hiding the page would leave callable commands, state, events, and storage behind.
- Choice/owner: remove the plugin branch at every existing owner: shell/router, release frontend, desktop contract, Rust IPC/application/domain/provider, configuration, and persistence.
- Alternatives: menu-only removal, compatibility facade, feature flag, dormant endpoint retention.
- Rationale: full closure is the smallest structure satisfying deletion; compatibility layers contradict the request.
- Dependency direction: unchanged for surviving code; removed edges disappear from UI -> store -> contract -> IPC -> service -> provider/storage.
- State/data/effect owners: plugin owners are deleted; update owners remain unchanged.
- Extension seam: none created.
- Verification: residual symbol/command search plus frontend and Rust tests/checks.

### ADR-2: Append-Only Database Cleanup

- Problem: `t_plugin` exists in a published SQLx migration and installed databases.
- Choice/owner: migration `0007_remove_plugin_center.sql` owns `DROP TABLE IF EXISTS t_plugin` and schema marker 7.
- Alternatives: edit `0001_initial.sql`, retain orphan table, runtime cleanup.
- Rationale: preserves migration checksums and upgrades existing installations deterministically.
- Compatibility: one-way schema retirement; no dual runtime path or rollback compatibility layer.
- Verification: database migration tests assert version 7 and table absence.

### ADR-3: Preserve Shared Release And Shell Owners

- Problem: plugin and update/config branches share files.
- Choice/owner: use narrow branch deletion; keep filenames, public entrypoints, state owners, and patterns for surviving behavior.
- Alternatives: split/rename release modules, reorganize shell, introduce new facade.
- Rationale: avoids unrelated architectural churn in a dirty worktree.
- Verification: focused tests for update runtime/store/pages and shell contracts.

### ADR-4: Relocate Surviving Update Tests

- Problem: `views/plugin-manager/release-pages.test.ts` contains both plugin and update coverage.
- Choice/owner: move the surviving update-dialog tests to `views/update-log/release-pages.test.ts`, then delete `views/plugin-manager`.
- Alternative: delete the combined test or retain a misleading plugin-owned directory.
- Rationale: preserves coverage and assigns it to the remaining feature owner.
- Verification: Vitest discovers and passes the relocated file.

## Capability And Ownership Map

| Capability | Current owner | Post-change owner |
| --- | --- | --- |
| Plugin navigation/page | application shell / plugin-manager | removed |
| Plugin frontend state/contracts/events | release store/runtime/adapters/contracts | removed |
| Plugin commands and lifecycle | Rust release IPC/application/domain/provider | removed |
| Plugin package path/permission | paths/build-channel/foundation permission | removed |
| Plugin configuration persistence | shell infrastructure / `t_plugin` | removed by code cleanup and migration 0007 |
| Update/about/resilience | existing release owners | unchanged |
| Non-plugin shell configuration | existing shell owners | unchanged |

## Module And File Design

- Delete feature-owned directories/files: `views/plugin-manager`, dedicated plugin docs, homepage plugin feature page, and historical plugin-system request artifacts.
- Modify shared frontend files only to remove plugin imports, branches, commands, event registrations, types, locale keys, and test expectations.
- Modify shared Rust files only to remove plugin types, branches, providers, commands, errors, permissions, and table access.
- Add exactly one production artifact: migration `0007_remove_plugin_center.sql`.
- Add no new production module, helper, type, facade, manager, registry, or compatibility layer.

## Code Relationship And Dependency Direction

Surviving flow remains:

`update UI -> release Pinia store -> desktop release contract -> Rust release IPC -> ReleaseState -> UpdateProvider`

Removed flow is deleted end to end:

`/plugins -> plugin page -> release store -> plugin contract/events -> Rust IPC -> PluginProvider -> t_plugin/package path`

Configuration flow remains `settings -> shell contract -> Rust shell infrastructure`, but its `plugins` scope/table branch is removed.

## Dependency Rule Matrix

| Owner | Allowed | Forbidden after change |
| --- | --- | --- |
| Application shell | surviving route/navigation descriptors | plugin route/icon/scope descriptors |
| Release frontend | update/about/resilience contracts and actions | plugin DTOs, adapters, actions, listeners |
| Desktop contracts | exact surviving Rust command/event shapes | compatibility plugin unions or optional plugin members |
| Rust release | update provider/state/event sink | PluginProvider, plugin command/event branches |
| Shell/config | surviving scope variants and tables | `plugins` scope or `t_plugin` access |
| Database | ordered immutable migrations | editing old migration checksums or runtime table fallback |

No new public API is introduced. Existing private plugin symbols are removed. The deletion reduces, rather than creates, cycle risk.

## State Topology And Data Lifecycle

- Writable plugin snapshots and per-plugin generation tracking are deleted from Pinia and Rust `ReleaseState`.
- Plugin-derived UI rows and status mapping are deleted with their adapter/page.
- Plugin event subscriptions and cleanup callbacks are deleted; the update listener remains the sole release runtime subscription.
- Plugin DTO/request/response lifecycle is deleted rather than normalized into a replacement shape.
- Surviving update DTO -> adapter -> Pinia -> dialog flow remains unchanged.

## Function And Side-Effect Design

- Delete plugin validators/mappers/command methods rather than retaining unused pure helpers.
- Keep update mappers and validation as their existing deterministic functions.
- Side effects continue only in Tauri IPC, providers, Pinia actions, and lifecycle listeners.
- Migration owns the single destructive schema side effect; configuration reset must stop issuing SQL against `t_plugin`.

## Data Structures And Type Strategy

Remove plugin enums, snapshots, requests, results, permissions, resources, provider traits, and shell/path variants on both TypeScript and Rust sides. Do not replace them with loose records, deprecated aliases, optional members, or assertions. Compiler errors are used to reveal missed consumers.

## Contract And Adapter Boundaries

The Rust invoke/event surface and TypeScript declarations are removed atomically. `adaptPluginSnapshot` is deleted with all consumers. Surviving update adapter semantics and field names remain unchanged.

## Reuse Candidate Inventory And Anti-DRY Matrix

| Candidate | Semantics/layer | Stable use sites | Variation | Decision |
| --- | --- | ---:| --- | --- |
| Plugin/update status mapping | different domain semantics; adapter layer | one each | status enums and lifecycle differ | delete plugin mapper; keep update mapper separate |
| Plugin/update event subscriptions | same transport mechanism; different events | one runtime | payload and lifecycle differ | remove plugin subscriptions; retain local update listener |
| Plugin/update provider traits | distinct application ports | one implementation each | commands/resources differ | delete PluginProvider; keep UpdateProvider |
| Plugin/config table reset branches | shared shell transaction, distinct tables | one branch per scope | schemas differ | remove plugin branch; no abstraction |

Commonality classification: the remaining shared transport and shell mechanisms are infrastructure conventions already owned by existing modules. No new extraction or JSDoc-promoted shared API is justified.

## Locality And File Extraction

No helper is added or extracted. Relocated update tests stay beside the update-log view because that is their single production owner. The new SQL migration is necessarily a separate ordered migration artifact.

## Pattern Decision

Use the existing Level 2 layered pattern and remove one vertical capability slice. Existing adapter, store-action, event-sink, provider-port, and repository/migration shapes remain for surviving capabilities. Reject Facade/Proxy/Strategy/Registry/feature-flag layers because there is no remaining variation axis or compatibility window.

## Extension Seams And Rejected Future-Proofing

No extension seam for third-party business plugins remains. Re-adding a plugin platform in the future would require a new approved capability and contracts; dormant hooks are forbidden. Tauri's framework plugin mechanism is unrelated and retained only where currently consumed.

## Complexity Budget

- New production files: one SQL migration.
- New public APIs: zero.
- New indirection layers: zero.
- Reactive fan-out: reduced.
- Migration scope: one table and schema marker.
- Compatibility paths/feature flags: zero.

## Migration, Compatibility, And Cleanup

Run `0007` after existing migrations; `DROP TABLE IF EXISTS` supports both databases that contain the table and defensive initialization states. All runtime queries must be removed in the same release. Rollback is database-backup/reinstall territory; no product-level rollback path is retained because the feature is intentionally retired.

## Readability And Maintenance Guardrails

Use narrow patches, preserve unrelated dirty hunks, remove unused imports/exports/tests/mocks, and avoid wholesale formatting. A final residual search must classify legitimate `tauri-plugin-*`, Vue, or build-tool references separately from business plugin-center remnants.

## Risks

- Shared files have unrelated edits; hunk-level review is mandatory.
- Combined update/plugin tests require careful relocation.
- Migration tests must prove both version advancement and table absence.
- Documentation searches may find generic framework “plugin” terminology that should not be removed.

## Open Architecture Questions

None.

