# Spec: Remove Plugin Center

## Delivery Unit

`remove-plugin-center`, direct-change feature retirement.

## Background And Goal

The user explicitly requested: “把插件中心功能及其代码删除”. The application must no longer expose, compile, register, persist, document, or advertise the business plugin-center capability. Tauri framework plugins used by other capabilities are excluded.

## In Scope

- Remove plugin-center navigation, route, page, install dialog, copy, state, adapters, commands, events, and tests.
- Remove Rust plugin domain types, validation, application state, provider port/implementation, IPC handlers/events, command registration, permissions, error mappings, and tests.
- Remove plugin-specific path and configuration import/export/reset branches.
- Add an append-only migration that drops `t_plugin`; update schema assertions/version marker.
- Remove dedicated plugin-center documentation and product feature claims.
- Adapt shared release/update and shell tests to assert the surviving behavior.

## Out Of Scope

- Removing Tauri framework plugins for updater, dialog, notification, logging, or global shortcuts.
- Refactoring surviving update, about, resilience, shell, or configuration behavior.
- Adding a replacement page, redirect, notice, compatibility endpoint, or feature flag.

## Trigger And Start Conditions

The direct user instruction is sufficient authorization. Repository inspection confirms a complete cross-layer capability chain and no unresolved product decision.

## Requirement Split Summary

Not split: this is one deletion closure. Requirement-analysis, requirement-splitting, and page-design are not applicable. Architecture design is required because the removal crosses public contracts, state ownership, IPC, and persistence.

## Source Grounding And Traceability

| Spec item | Label | Source | Downstream obligation |
| --- | --- | --- | --- |
| Remove plugin center and its code | source-backed, product-facing | User: “把插件中心功能及其代码删除” | Delete every reachable feature layer |
| Preserve non-plugin release/shell behavior | code-fact-backed | Shared release and shell owners contain unrelated branches | Use targeted edits and regression tests |
| Preserve Tauri framework plugins | source-derived implementation boundary | These crates power non-plugin-center capabilities | Keep dependencies/config unless separately unused |
| Drop persisted plugin records by new migration | code-fact-backed | SQLx migration history and existing `t_plugin` | Add migration; do not rewrite `0001_initial.sql` |

Closed-world allowlist: removal of the existing plugin-center entry, page, install/enable/disable/configure/uninstall behavior, status/crash events, contracts, permissions, storage, tests, and dedicated documentation. No new user-visible content or behavior is authorized.

## Workflow Efficiency Profile

`S2 scoped`, full artifacts, scoped context. This is the lowest safe profile because one feature spans Vue, Tauri, Rust contracts, SQLite, and docs. Upgrade only if implementation reveals an externally consumed plugin API or migration incompatibility.

## Self-Healing Completion Bar

- Search must find no business plugin-center route, page, commands, events, types, table access, permissions, or product claims.
- Surviving update/about/resilience and configuration branches must type-check and retain focused tests.
- TypeScript tests/type-check/build and Rust format/tests/checks must pass, or a concrete environment blocker must be recorded.
- Imports, exports, mocks, stale tests, and dead code created by the deletion must be removed.
- Completion requires self-healing loop compliance: pass; assessment: pass; remaining blockers: none.

## User Intent Contract

- Literal request: delete plugin-center functionality and code.
- Practical goal: the product no longer contains a dormant or reachable business plugin system.
- Success: no entrypoint or implementation chain remains, and adjacent behavior is intact.
- Forbidden interpretations: hiding only the menu, retaining dormant IPC/storage, or deleting unrelated Tauri framework plugins.
- Acceptable approach: dependency-closure removal with append-only schema cleanup.
- Review checks: scoped diff, residual search, contract parity, migration test, frontend/Rust verification.

## User Flow And Page Design

After removal there is no plugin-center user journey. `/plugins` is no longer a registered named route. No replacement or placeholder is added. Page/UI design and field display semantics are otherwise not applicable because the surface is deleted.

## Function-Complete Behavior Breakdown

- Application shell omits the plugin navigation item and icon mapping.
- Router omits the `/plugins` route and lazy import.
- Release runtime listens only to surviving update events.
- Release store exposes only update/about/resilience state and actions.
- Desktop contract exposes no plugin DTO, command, event, path purpose, or shell scope.
- Rust command list, permissions, invoke handler, event sink, service, provider, validation, and errors contain no plugin branch.
- Configuration export/import/reset no longer reads or writes `t_plugin`.
- Migration `0007` drops `t_plugin` and advances the schema marker.
- Dedicated plugin docs/pages/request history and current product claims are removed.

## Engineering Constraints

- State ownership: surviving release state stays in `ReleaseState`/Pinia; no compatibility plugin state remains.
- Data lifecycle: delete raw plugin DTO -> adapter -> store -> view and IPC -> service -> provider paths atomically.
- Async correctness: remove plugin listeners and their cleanup; preserve update listener cleanup and update race semantics.
- Type-first: remove types before/with consumers; do not introduce `any`, broad unions, or compatibility shims.
- Fail fast: migration and configuration code must not access the dropped table after migration.
- Side effects remain at current IPC/store/lifecycle boundaries; no new manager, registry, or event bus.
- Performance: no new runtime work; removal should reduce bundle and command surface.
- Human review: shared dirty files receive only plugin-related hunks; unrelated changes are preserved.

## Architecture Reuse And Locality

Plugin-specific logic has one business owner and no valid post-removal use site, so it is deleted rather than extracted. Surviving update code remains in its current release owners. No helper, component, mapper, or shared API is added or moved.

Anti-DRY result: apparent similarities between update and plugin status/event handling are different domain semantics and only coexist in shared files; after plugin deletion there is no duplication candidate.

## Pattern Decision

- Concrete change: remove one existing cross-layer capability and close its dependencies.
- Depth: Level 2, reuse existing layered project boundaries during deletion.
- Signal: contract adaptation, observer lifecycle, command surface, and persistence ownership are already formalized.
- Decision: reuse existing pattern; remove plugin branches within router/store/adapter/IPC/service/provider/repository boundaries.
- Rejected: compatibility facade, feature flag, registry, manager, or replacement route because they preserve unnecessary surface and contradict full deletion.
- Overbuilt signal: any newly introduced abstraction or dual path.

## Frontend Architecture Routing Inputs

`architecture-design_required: true`. Architecture design must preserve the approved deletion scope and decide the exact ownership cleanup, dependency order, command/event removal, migration cleanup, and test relocation. It must roll back to spec if it changes surviving behavior or API semantics.

## API And Data Contracts

Authoritative contracts are the Rust Tauri commands/events and their mirrored TypeScript declarations. Remove plugin list/install/enable/disable/configure/uninstall commands, plugin status/crash events, request/result/snapshot/config/resource/permission types, and their IPC error mappings on both sides in the same change. Surviving update/about/resilience contracts remain direct and unchanged.

SQLite migration contract: `0001_initial.sql` remains unchanged; `0007_remove_plugin_center.sql` drops `t_plugin` with `IF EXISTS` and records schema version 7. Configuration export/import no longer includes a `plugins` scope.

## Frontend / Server Responsibility Boundary

Both frontend and Rust backend are in scope. The frontend must not retain mocks for removed server commands. Rust must not retain dormant endpoints for removed frontend callers. No external interface is pending.

## Edge Cases

- Existing installations with plugin rows upgrade without migration failure and lose the retired table.
- New installations run all migrations and end without `t_plugin`.
- Free and member command/scope matrices both omit plugin entries.
- The combined release-page test file is split or relocated so update-dialog coverage is not deleted with the plugin page.
- Residual uses of “plugin” that refer to Tauri/Vue/tooling dependencies are allowed only when they are demonstrably unrelated to the business plugin center.

## Acceptance Criteria

1. Navigation, router, localization, and feature-owned files expose no plugin center.
2. TypeScript release/shell/base contracts, runtime, store, adapters, and tests contain no business plugin symbol.
3. Rust release/path/shell/database/IPC/permission code contains no business plugin symbol or command.
4. Database tests prove schema version 7 and absence of `t_plugin`.
5. Dedicated plugin documentation and current product claims are removed.
6. Residual search distinguishes and retains only unrelated framework/tooling plugin references.
7. Focused frontend and Rust verification passes without regressions in surviving release/shell behavior.

## Human Review And Handoff

Map changed hunks to navigation/UI, frontend contracts/state, backend contracts/state, persistence/config, tests, and docs. Report deleted files, migration behavior, commands run, and any remaining non-business uses of the word `plugin`.

## Risks

- Dirty shared files could lose unrelated edits; mitigate with narrow patches and diff review.
- Editing old migrations would break checksum validation; prohibited.
- Removing the combined plugin/update test file could silently reduce update coverage; relocate the surviving tests.
- Broad residual search can produce legitimate Tauri/Vue tooling references; classify rather than delete them mechanically.

