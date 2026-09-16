# Platform Foundation Execution Changelog

## Delivery Unit Identifier

- Request: `tauri-rust-refactor`
- Module: `platform-foundation`
- Loop iteration: 7
- Pass: initial implementation

## Framework-Approved Inputs

- `spec/spec.md`
- `spec/clarifications.md`
- `design/architecture-design.md`
- `plan/plan.md`
- `plan/task-board.md`

## Execution Summary

PF-01 through PF-07 completed. The Tauri v2 host, typed WebView facade, single Rust `AppState`, SQLite/configuration boundary, purpose-bound path grants, capability policy and compile-time free/member command registries are implemented and green.

## Self-Healing Execution Notes

- Re-entry source: none.
- Defect class: none.
- Current focus: verification handoff.
- New evidence: both Tauri channel builds pass; member-only commands are actually registered and fail closed when external contracts are absent; free compilation excludes their handlers.
- Repaired during execution: five TypeScript errors, 48 ESLint errors, a renderer-to-Node logger dependency, duplicate bootstrap initialization, non-regular path acceptance, and a Tauri watcher loop caused by the local target junction.
- Remaining baseline signals: 106 non-blocking ESLint warnings in later functional modules and stale root-level tests owned outside this delivery unit.
- Verification focus: exact database objects, WAL, idempotency, rollback, repository-only config access, channel isolation, runtime startup and static security boundaries.

## Task-Board Updates

- PF-01: done.
- PF-02: done.
- PF-03: done.
- PF-04: done.
- PF-05: done.
- PF-06: done.
- PF-07: done.

## Introduced-Content Provenance Ledger

- Current artifact-only hunks: `product-content delta: none`; authorized by workflow documentation requirements.
- Production diff ledger will be derived from actual code changes after each task.

## Implementation Decisions

- Official existing-frontend scaffold retained `src-tauri` and existing Vue renderer; demo frontend was not generated.
- Vite no longer starts Electron compilation and listens on fixed `127.0.0.1:5173` for Tauri.
- Tauri config uses the shared identifier and PRD window dimensions/CSP.
- The desktop facade defines exact command/event maps, normalizes rejected invokes and returns the real unsubscribe function; direct Tauri imports stay in `transport.ts`.
- Rust newtypes enforce PRD ID/text/volume/threshold boundaries; bootstrap uses an explicit state machine; build channel and member command names are compile-time feature derived.
- Application bootstrap depends on a database port; the SQLx adapter owns the shared initialization cell, exact 21-table/7-index migration, WAL, pre-migration backup and config repository.
- Tauri registers bootstrap/status/config commands through thin IPC adapters; error serialization keeps stable codes and removes native failure details from WebView responses.
- Rust-native dialogs feed an opaque, purpose-bound grant store. Capabilities expose only the six foundation commands to `main`; `effect-*` has an empty permission list and the WebView receives no generic dialog/filesystem permission.
- One composition-root `AppState` owns application and path state. Concurrent bootstrap callers wait for the active attempt and do not invoke the database port twice.
- Member builds register the six PRD command names behind `cfg(feature = "member")`; each returns `external_service_unavailable` until its source contract exists. Free builds compile no such handler.
- The renderer gift service now uses the browser-safe logger; Node `fs/path` no longer enters the WebView build.
- `.taurignore` excludes generated target/gen paths so the Tauri development watcher does not rebuild on its own outputs.

## TDD Evidence Or Exceptions

- PF-01 scaffold/config generation used the skill-authorized configuration/generated-code exception. `cargo check` and `pnpm build:web` passed after integration.
- PF-02..PF-05 require observed red -> green -> refactor evidence.
- PF-02 red: module resolution failed because `transport.ts` did not exist. Green: 3 Vitest cases pass; focused ESLint passes; package type-check reports only the five baseline errors.
- PF-03 red: Rust integration tests could not resolve the approved application/domain/ipc/build modules; the member-only compile separately failed without its command set; DTO test failed without `ipc::dto`.
- PF-03 green: 7 integration tests pass under default free and `--no-default-features --features member`.
- PF-04 red: schema/config/bootstrap symbols and use cases were introduced through failing tests before implementation; the backup case failed against a missing backup before copy-on-preflight was added.
- PF-04 green: 5 database/repository unit tests and 12 foundation integration tests pass; concurrent bootstrap shares initialization, failure remains retryable, and ordinary config rejects sensitive keys.
- PF-05 red: path-store/application APIs and capability files each failed focused tests before implementation.
- PF-05 green: 17 foundation integration tests pass, covering cancel, purpose mismatch, revoke, stale roots, traversal/symlink escape, closed purpose deserialization and exact capability assignment.
- PF-06 red: member registry initially existed only as metadata, and the WebView bundle contained Node-only logger imports. Green: member integration has 19 tests including six fail-closed commands; both Tauri builds pass; Web builds no longer report Node externalization errors.
- PF-07 audit repair: the composition root initially managed two states and bootstrap relied on SQLx to deduplicate. Green: one `AppState`, a counting fake proves one initialization, path-kind tests cover file/directory enforcement, and `dev:free` remains stable after startup.

## Deviations, Blockers, And Rollback Triggers

- No remaining platform-foundation blocker.
- Full installer bundling is intentionally not claimed; debug no-bundle integration was used because signed release assets and production Sidecars are unavailable and belong to later modules.
- Existing Electron business callers remain under their assigned later-module cleanup triggers; deleting them here would violate the approved migration order.
- Vite reports an existing large-chunk advisory. It is not caused by the foundation gateway and is owned by application-shell/performance work.

## Verification Handoff Notes

- Rust: free 23/23 and member 24/24 tests pass; fmt passes; Clippy passes for both features with warnings denied.
- Frontend: type-check passes; lint exits 0 with 106 later-module warnings; platform Vitest 5/5 passes.
- Integration: free/member Tauri debug builds pass; `dev:free` starts Vite and the Tauri process without Electron or watcher churn.
- Runtime: main window observed in 647 ms with about 51.1 MB working set; no Sidecar is started.
- Static: only `platform/desktop/transport.ts` imports Tauri APIs; no send-danmaku production symbol; capabilities and five JSON configs parse.
