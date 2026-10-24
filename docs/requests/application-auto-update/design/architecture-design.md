# 应用自动更新与检查更新 - 架构设计

## Delivery Unit And Objective

- Delivery unit: `application-auto-update`
- Objective: 在不新增平行更新链路的前提下，让现有 Tauri release capability 具备启动检查、全局弹窗与设置页手动检查入口。
- Upstream: framework-approved `spec/spec.md`, `artifacts/code-context.md`, PRD 9.18.
- Scaffold: 不适用，不新建应用/包/前端面。

## Architecture Decision Records

### ADR-01: Startup Check Lives In Release Runtime

- Problem: startup check needs ordering with bootstrap/listener registration and lifecycle cleanup.
- Decision: extend `createReleaseRuntime.startGeneration()` after successful listener registration; the runtime invokes the existing store action once and treats rejection as startup-silent.
- Alternatives: invoke from `App.vue`; add a new auto-update composable/service; native background task.
- Smallest sufficient structure: reuse the existing lifecycle owner; no file or public API added.
- Dependency direction: `App -> release runtime -> release store -> desktop transport`; unchanged.
- State/data owner: store/native state unchanged.
- Side-effect owner: startup orchestration remains the release runtime.
- Rejected seams: no timer, scheduler, manager, registry or configurable policy layer.
- Verification: runtime test proves sequencing, one invocation, error isolation, and cleanup.

### ADR-02: Global Dialog Lives At Main App Root

- Problem: a candidate can arrive while any route is active.
- Decision: mount the existing `UpdateAvailableDialog` once in `App.vue`, driven by the shared store. Keep `/update-log` as a compatible explicit page; both consume the same store and component.
- Alternatives: route redirect to `/update-log`; per-page dialogs; toolbar-specific notification state.
- Smallest sufficient structure: one existing component instance at the only route-independent UI owner.
- State owner: candidate stays in store; only open/closed presentation state is local to root.
- Side effects: root handlers call store actions; no direct desktop IPC.
- Compatibility: existing update-log route remains unchanged and valid.
- Verification: root contract/component evidence and reference scan.

### ADR-03: Manual Check Lives In Settings About Section

- Problem: hidden `/update-log` does not provide an discoverable manual entry.
- Decision: add a check button beside existing version/channel information; handler remains local and calls the store.
- Alternatives: add a new navigation item; create a new settings category; move the entire update-log page.
- Smallest sufficient structure: no navigation or page IA change, no new component.
- State owner: settings owns only toast feedback, not update state.
- Verification: settings contract test and UI inspection.

### ADR-04: Forced Dialog Closure Is Component-Owned

- Problem: hiding Later/Skip does not disable close icon, backdrop, or Escape.
- Decision: `UpdateAvailableDialog` derives `isForced` from its immutable snapshot and binds Element Plus dismissal props.
- Alternatives: root intercepts `v-model`; native window lock; duplicate forced dialog.
- Smallest sufficient structure: local computed state in the component that owns dismissal affordances.
- Verification: dialog contract/component test covers every dismiss path.

### ADR-05: Native Check Errors Propagate; Startup Silence Is A Renderer Policy

- Problem: `TauriUpdateProvider.check()` currently turns network and release-not-found failures into `UpdateStatus::Current`, so an explicit manual check cannot distinguish offline from a verified no-update result.
- Decision: native provider maps transient updater failures to a retryable `ReleaseServiceError` and keeps malformed/configuration failures as invalid manifest errors. `useReleaseRuntime` alone suppresses the error during startup; explicit store callers continue receiving normalized errors.
- Alternatives: keep returning `current`; infer offline in Settings; add a second manual-check IPC.
- Smallest sufficient structure: one pure native error-mapping helper plus existing renderer lifecycle catch; no command or DTO change.
- Dependency/state direction: unchanged. Native provider owns transport truth; runtime owns startup presentation policy.
- Migration/cleanup: replace `is_transient_check_error` with the mapping owner and update its unit test; no compatibility path remains.
- Verification: Rust unit test proves transient/configuration mapping, runtime test proves startup suppression, settings contract proves manual failure feedback.

## Capability And Ownership Map

| Capability | Writable owner | Derived/presentation owner | Side-effect boundary |
| --- | --- | --- | --- |
| Candidate/status/progress/error | `useReleaseStore` from native snapshots | App, Settings, Update Log | Store actions -> desktop transport |
| Rollout/blacklist/forced/signature | Rust `ReleaseState` / `TauriUpdateProvider` | Renderer reads normalized flags only | Tauri updater |
| Startup lifecycle | `useReleaseRuntime` local lifecycle state | None | runtime `start/stop` |
| Global dialog visibility | `App.vue` local ref/watch | `UpdateAvailableDialog` | root event handlers |
| Manual outcome message | `settings/index.vue` transient UI | Element Plus message | settings event handler |

There is no duplicated writable candidate state, no DTO normalization in templates, and no direct IPC call from a page.

## Module Boundaries And File Structure

Only existing files are edited:

```text
src/renderer/
  App.vue                                      # global presentation + action orchestration
  composables/useReleaseRuntime.ts             # startup auto-check lifecycle
  composables/useReleaseRuntime.test.ts        # sequencing/error isolation
  views/settings/index.vue                     # manual action + feedback
  views/settings/settings-shell-contract.test.ts
  views/update-log/UpdateAvailableDialog.vue   # forced dismissal policy
  views/plugin-manager/release-pages.test.ts   # release UI contract
  locales/zh-CN.json
  locales/en-US.json
src-tauri/src/infrastructure/release.rs             # truthful updater error mapping
```

No new runtime, service, helper, store, contract, adapter or SFC is created.

## Dependency Rule Matrix

| Source | Allowed dependencies | Forbidden dependencies/private knowledge |
| --- | --- | --- |
| `useReleaseRuntime` | release store, typed desktop listener | Vue components, router, Element Plus, Tauri API directly |
| `useReleaseStore` | desktop transport/contracts, release adapters | route/component state, Element Plus messages |
| `App.vue` | runtime, store, existing dialog | direct `invokeDesktop`, native updater SDK |
| Settings page | release store, i18n, Element Plus UI | native IPC/updater SDK, writeable duplicate candidate |
| Update dialog | `UpdateSnapshot`, props/emits, i18n | store, desktop transport, route |
| Native update provider | Tauri updater SDK, release ports/domain | renderer UI concerns, settings-page policy |

Public API remains the existing runtime `start/stop/isActive`, store actions/refs and dialog props/emits. All new handlers/derived values are file-private. Cycles are not introduced.

## State Topology And Data Lifecycle

```text
Tauri updater metadata
  -> Rust candidate validation/policy
  -> IPC UpdateSnapshot
  -> adaptUpdateSnapshot (immutable renderer contract)
  -> useReleaseStore.update (single writable renderer owner)
  -> App/settings/update-log derive presentation
```

- Async state: `pending` Set owned by store; operation map deduplicates by key.
- Stale response: native generation policy and runtime generation guard remain authoritative.
- Cleanup: runtime `stop()` removes listeners and calls store `dispose()`; root local dialog state dies with component.
- Root watch derives only visibility from `candidate + status`; it does not copy snapshot fields.
- Manual feedback is an immediate command result, not persistent domain state.
- Transport failure remains an error until it reaches its caller. Only the startup runtime elects to suppress presentation; a manual store call observes the normalized error.

## Reuse Candidate Inventory And Anti-DRY Matrix

| Candidate | Locations/use sites | Semantic/layer result | Stability/variants | Decision |
| --- | --- | --- | --- | --- |
| Candidate action handlers | root and update-log page (2) | Same store commands, but route-local vs global presentation lifecycles | Few sites; handlers trivial | Keep local; extracting adds navigation without stable abstraction value. |
| Actionable status predicate | root/update-log (2) | Same update UI semantic | Stable, but one expression | Keep local now; fewer than three sites and no complexity. |
| Update dialog | update-log and root (2) | Same cohesive UI/business meaning | Stable props/emits | Reuse existing SFC. |
| Manual outcome message | settings only (1) | Page-local user feedback | Single owner | Colocate in settings page. |
| Startup side-effect orchestration | release runtime only (1) | Independent lifecycle boundary | Existing owner | Extend existing runtime, no extraction. |
| Updater error classification | native provider only (1) | Infrastructure truth used by both startup/manual callers | Stable transport variants | Keep a pure mapper beside provider; do not duplicate in renderer. |

Commonality classification: existing dialog is UI commonality already owned by the update feature; runtime/store are release-domain infrastructure. No new commonality is promoted, so JSDoc/shared API requirements do not trigger.

## Function And Side-Effect Design

- `startGeneration(localGeneration)`: command/lifecycle function; bootstrap, validate generation, subscribe, activate, invoke one auto check, contain startup-only failure.
- Native `map_update_check_error`: pure mapper from Tauri updater error to `ReleaseServiceError`; no state mutation or I/O.
- `handleDownloadUpdate`, `handleInstallUpdate`, `handleSkipUpdate`: root-local commands; candidate guard only where required, then store action.
- `handleCheckUpdate`: settings-local command; await store action and map final snapshot to factual localized feedback.
- `isForced` and dialog visibility conditions: view-local derived state; no mutation or I/O.
- Props/snapshots are immutable inputs. Only Pinia refs and component-local refs are mutated at explicit owners.
- No functional utility library or generic pipeline is justified.

## Type Strategy And Adapter Boundary

- Reuse `UpdateSnapshot`, `UpdateStatus`, `UpdateCandidate`, and `DesktopError` from the desktop contract.
- Reuse `adaptUpdateSnapshot`; no template-level validation or fallback normalization.
- No new branded IDs/tokens are needed because no new structurally ambiguous identifier crosses the boundary.
- Strict null rule: `candidate === null` is a valid no-update state; action handlers must not assert it non-null.

## Pattern Decision

- Level 2, reuse existing lifecycle composable + store command + observer bridge.
- This preserves current dependency direction and avoids a second facade/service.
- A new Command/Strategy/Facade/Proxy/manager layer is rejected because there is one fixed update transport and existing store actions already form the command surface.
- Existing native observer events remain lifecycle-safe under the runtime; no new event bus is introduced.

## Extension Seams And Rejected Future-Proofing

- Real seams retained: free/member release channel, native `UpdateSnapshot`, runtime lifecycle, dialog props/emits.
- Rejected: polling interval, selectable channels, alternate update providers, background-download strategies, registries/factories, feature flags, and duplicate localStorage preferences. None are source-backed.

## Complexity Budget

- New production files: 0.
- New public APIs/types: 0.
- New reactive watches: at most 1 root visibility watcher.
- New cross-module imports: App imports store/dialog; Settings imports Refresh/ElMessage only.
- New indirection depth: 0.
- Native files changed: at most 1; IPC/DTO migrations: 0.
- Tests may edit up to three existing focused test files; add a new test file only if existing ownership cannot express the contract.

## Migration, Compatibility, Cleanup And Rollback

- Migration is additive: existing `/update-log` route and store/native commands remain.
- No dual data path or compatibility flag is introduced.
- Rollback surface is limited to renderer edits; removing root/runtime/settings integration restores previous behavior without data migration.
- Existing base Tauri config remains updater-disabled; generated signed release configs remain the production activation mechanism.
- Post-change cleanup: reference scan must show no stale imports, duplicate updater invocation, orphan dialog implementation or direct page IPC.

## Readability And Review Guardrails

- Preserve local Composition API and store conventions.
- Keep handlers explicit and short; no one-function helper files.
- Avoid formatting unrelated dirty files or overwriting current user edits.
- Every hunk must map to AC-01..AC-07.
- Required evidence: focused tests, type check, scoped lint, renderer build, release contract tests, Rust release/foundation tests, and final diff review.

## Architecture Risks And Open Questions

- Risk: Element Plus dialog dismissal behavior must be fully bound for forced candidates, not inferred from footer visibility.
- Risk: auto-check failure suppression must be limited to the startup boundary; manual checks must still surface errors.
- Risk: a root watch can reopen a dialog after the user chooses Later if it reacts to unrelated changes. The implementation must react only to a transition into actionable candidate state, not continuously force open.
- Open architecture questions: none.
