# 应用自动更新与检查更新 - 执行记录

## Delivery Unit

`application-auto-update`

## Framework Inputs

- `spec/spec.md`
- `design/architecture-design.md`
- `plan/plan.md`

## Self-Healing Execution Notes

- Initial execution iteration: 4.
- T1-T5 implemented through red -> green cycles.
- During T6 behavior review, found `architecture-gap` `native_transient_check_semantics`: `TauriUpdateProvider.check()` converts network/release-not-found failures into a `current` snapshot. That satisfies silent startup but makes an explicit manual check falsely report that the app is current. The architecture file list had also incorrectly excluded the native provider from the change.
- Same-stage attempts: 1; total attempts: 1.
- Re-entry target: architecture-design, then plan. The design must assign transient error propagation to the native provider and startup silence to `useReleaseRuntime`; the plan must add a tested native error mapping.
- No native production code was changed before this re-entry.
- The architecture/plan re-entry completed in iteration 6. `TauriUpdateProvider` now reports transient checks as retryable `update_check_failed`; only `useReleaseRuntime` suppresses that failure during startup. Exact native mapper and IPC-message tests pass.
- Final execute self-review found `runtime-error` `disposed_check_stale_write`: a startup check resolving after `release.dispose()` could still replace `release.update`. Red evidence: `release.test.ts` failed with a non-null late snapshot. Repair: `checkUpdate()` captures the current store generation and commits only when still current. Same-stage attempts: 1; total attempts: 1; re-entry target: execute; focused store/runtime tests rechecked 7/7 green.
- Final execute self-review found `logic-error` `global_update_failure_feedback`: root dialog download/install/skip failures were normalized in the store but not shown, and a removed candidate could leave the dialog open. Red evidence: the root shell contract failed before error feedback and candidate-gated rendering were implemented. Repair: candidate guards, actionable-state close behavior, Element Plus normalized error feedback, and candidate-gated dialog mounting. Same-stage attempts: 1; total attempts: 1; re-entry target: execute; shell contract rechecked 4/4 green.
- Independent review after verification found `runtime-error` `disposed_operation_corrupts_new_state`: after `dispose()` clears the operation registry, an older same-key promise can still clear a newer request's pending flag in `finally`, and an older rejection can repopulate stale error state. Same-stage attempts: 1; total attempts: 1. Re-entry target: execute in iteration 7; add red concurrency/error tests before changing the shared operation boundary.
- Iteration 7 repair: `run()` now captures its generation, writes normalized errors only while that generation is current, and clears pending only when the registry still points to that exact promise. Red evidence contained two expected failures; green evidence is release store/runtime 9/9 plus TypeScript pass. Remaining blocker for this identity: none.

## Implemented Chain

- `useReleaseRuntime.startGeneration()` performs one check after bootstrap and listener registration and contains startup-only failures.
- `useReleaseStore.checkUpdate()` retains dedupe/error ownership and ignores results that resolve after disposal.
- `App.vue` presents the existing update dialog globally, routes actions through the release store, closes on non-actionable snapshots, and shows normalized action failures.
- `UpdateAvailableDialog.vue` disables close icon, backdrop, Escape, Later, and Skip for forced candidates.
- Settings About exposes a loading manual check button with current/available/failure feedback.
- Native updater failures remain truthful for manual callers while signed manifest/download/install ownership remains unchanged.

## TDD Evidence

- Initial renderer contract run: 4 expected failures before production wiring.
- Runtime auto-check suite: 4 passed after implementation.
- Store stale-response test: failed before generation guard, then store/runtime suites passed 7/7.
- Root failure-feedback contract: failed before feedback/candidate guard, then passed 4/4.
- Forced update and settings contracts passed after their production changes.

## Changed Files Owned By This Delivery

- Renderer production: `App.vue`, `useReleaseRuntime.ts`, `stores/release.ts`, `views/settings/index.vue`, `views/update-log/UpdateAvailableDialog.vue`.
- Native production: `src-tauri/src/infrastructure/release.rs`, `src-tauri/src/ipc/error.rs`.
- Tests: focused runtime/store/shell/settings/release-page contract files.
- Existing unrelated dirty hunks in the shared files and worktree were preserved.

## Current Task Status

- T1-T5B: implemented with recorded red -> green evidence.
- Execute handoff is ready for T6 verification. Prior checks show zero lint errors (two pre-existing logger warnings), successful renderer build and release Node tests, and passing exact native updater tests.
