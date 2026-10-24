# 应用自动更新与检查更新 - 实施计划

## 目标与范围

按已审批 spec 和 architecture design，在现有 release runtime/store/Tauri updater 链路上补齐启动自动检查、全局更新对话框、强制更新不可关闭与设置页手动检查。仅修正原生 updater 的检查失败映射，不改 IPC/DTO、发布策略、签名发布脚本、导航结构或更新通道配置。

## 执行前合同

- Governing TypeScript: `packages/bilibili-live-tool/tsconfig.json` + `tsconfig.eslint.json`; strict, bundler resolution, DOM libs and `@/*` alias apply.
- Reused contract: `UpdateSnapshot` and release store actions; no new DTO/type/API.
- Writable state remains `useReleaseStore.update/pending/error`; component refs only own presentation.
- Side effects remain in runtime lifecycle, store actions and explicit event handlers.
- Product-content allowlist is limited to startup check, settings check control/outcomes, global existing dialog and localized factual messages.
- Preserve all unrelated dirty-tree hunks, especially settings/application-shell/i18n work.

## 业务流程

```text
App mounted
  -> releaseRuntime.start
  -> releaseStore.bootstrap
  -> register release listeners
  -> active=true
  -> releaseStore.checkUpdate once
      -> current/offline: no startup popup/error interruption
      -> candidate: shared store transition -> App opens dialog
      -> download -> verified native install -> restart
```

```text
Settings/About -> click Check Update
  -> checkUpdate pending/loading + dedupe
  -> current: localized latest message
  -> candidate: localized available message + root dialog opens
  -> failure: normalized error message, pending clears, retry remains available
```

## 任务

### T1 - 先补失败测试

- Grounding: `source-backed` SRC-01..05; spec AC-01..AC-05.
- Product-content delta: none (tests only).
- Edit `src/renderer/composables/useReleaseRuntime.test.ts`:
  - mock `store.checkUpdate`;
  - assert it runs once only after all three listeners resolve;
  - assert rejection does not reject `runtime.start()` and runtime remains active;
  - preserve listener rollback/dispose tests.
- Edit `src/renderer/components/application-shell/shell-components-contract.test.ts`:
  - assert `App.vue` mounts `UpdateAvailableDialog` and calls store download/install/skip actions;
  - assert no direct `invokeDesktop` is introduced.
- Edit `src/renderer/views/plugin-manager/release-pages.test.ts`:
  - replace the weak forced-footer-only check with assertions for `show-close`, modal click and Escape bindings in addition to Later/Skip hiding.
- Extend `src/renderer/views/settings/settings-shell-contract.test.ts` without removing the user's current `theme: 'system'` assertion:
  - assert settings uses `release.checkUpdate()`, a loading key, localized current/available feedback, and no direct transport.
- Run the four focused test files and record expected failures before production edits.

### T2 - 接线启动自动检查

- Grounding: `source-backed` SRC-01..03; spec Startup Check and AC-01/AC-05.
- Product-content delta: startup invokes the approved automatic check; no new visible copy.
- Edit `src/renderer/composables/useReleaseRuntime.ts`, symbol `startGeneration`:
  - keep existing bootstrap and generation guard;
  - keep transactional listener registration/rollback;
  - after listeners are committed and `active=true`, await one `store.checkUpdate()` inside a startup-only rejection boundary;
  - do not clear store error, add timers, retry/poll, or throw auto-check failure;
  - `start()` idempotency and `stop()` cleanup remain unchanged.
- Run `useReleaseRuntime.test.ts` to green.

### T3 - 挂载全局更新对话框

- Grounding: `source-backed` SRC-02, SRC-04, SRC-05; spec AC-02/AC-05.
- Product-content delta: existing approved update dialog becomes route-independent.
- Edit `src/renderer/App.vue`:
  - import `watch`, `useReleaseStore`, and existing `UpdateAvailableDialog`;
  - add one local dialog ref and one watch over store update status/candidate transitions;
  - open only when entering `available|downloading|downloaded` with non-null candidate; do not continuously reopen after Later;
  - add local `handleDownloadUpdate`, `handleInstallUpdate`, `handleSkipUpdate` commands with candidate guard for skip and intentional local catch because store owns normalized errors;
  - mount dialog as a sibling inside the root provider, outside route content; pass shared snapshot and aggregate release pending state.
- No new component/style file; no direct desktop IPC; no duplicate candidate payload.
- Run shell contract test and TypeScript check.

### T4 - 关闭强制更新隐式退出路径

- Grounding: `source-backed` PRD forced-update rule; spec AC-03.
- Product-content delta: forced candidate cannot use ordinary dismissal paths.
- Edit `src/renderer/views/update-log/UpdateAvailableDialog.vue`:
  - derive local `isForced` from the immutable snapshot;
  - bind `:show-close="!isForced"`, `:close-on-click-modal="!isForced"`, `:close-on-press-escape="!isForced"`;
  - reuse `isForced` for Later/Skip visibility;
  - preserve existing download/install/progress behavior.
- Run release page contract test.

### T5 - 增加设置页手动检查与本地化

- Grounding: `source-backed` SRC-01; `source-derived` factual check outcomes; spec AC-04/AC-05.
- Product-content delta: one Check Update button plus current/available/failure feedback in both supported languages.
- Edit `src/renderer/views/settings/index.vue`:
  - import `Refresh` and `ElMessage`;
  - add local `handleCheckUpdate` which awaits `release.checkUpdate()`;
  - on `current`, show `update.noUpdate`; on actionable candidate show `update.hasUpdate`; on error show `release.error?.message ?? t('settings.feedback.operationFailed')`;
  - add button in About header/metadata area with `:loading="release.pending.has('checkUpdate')"` and existing `update.checkUpdate` label;
  - use only short inline Tailwind utilities and existing Element Plus controls.
- Reuse existing `update.noUpdate`, `update.hasUpdate`, and `update.checkUpdate` keys; only add locale keys if tests prove a required factual state is missing.
- Run settings contract test and TypeScript check.

### T6 - 链路复查、验证与人工评审准备

- Grounding: `code-fact-backed`; spec AC-01..AC-07.
- Product-content delta: none.
- Re-run focused Vitest suite.
- Run `pnpm type-check`, scoped ESLint check and `pnpm build:renderer`.
- Run `pnpm test:release`.
- Run scoped Rust tests for release/foundation contracts; if platform toolchain blocks, record the exact command/error and retain Node/TS evidence.
- Search changed chain for direct updater/IPC bypass, duplicate check calls, stale imports and orphan handlers.
- Inspect `git diff` only for approved files; preserve unrelated user changes and remove debug/dead/formatting churn.
- Record execution, verification and remaining risks.

### T5B - 分离原生检查错误与启动静默策略（架构回流新增）

- Grounding: `source-backed` offline startup silence (SRC-02) + `source-derived` truthful manual outcome; revised ADR-05.
- Product-content delta: manual check no longer reports `current` when the updater request failed.
- Edit the existing unit test in `src-tauri/src/infrastructure/release.rs` first:
  - assert `ReleaseNotFound` maps to `ReleaseServiceError { code: "update_check_failed", retryable: true }`;
  - assert `EmptyEndpoints` maps to invalid, non-retryable `invalid_update_manifest`.
- Observe the Rust test fail because the mapper does not exist.
- Replace `is_transient_check_error` with a local pure `map_update_check_error` beside `TauriUpdateProvider`:
  - network/request/release-not-found -> retryable unavailable error;
  - all other updater errors -> non-retryable invalid manifest;
  - `check()` returns this error after generation validation instead of fabricating `current`.
- Update `ReleaseServiceError::unavailable` only if needed so retryability is truthful for unavailable transport failures; verify all current unavailable callers remain semantically correct before changing a shared constructor. Prefer adding a narrow retryable constructor if existing callers must stay non-retryable.
- Add `update_check_failed` to `DesktopError` user message mapping without changing DTO shape.
- Re-run Rust release tests and renderer runtime/settings tests to prove startup remains silent while manual errors remain visible.

## TDD And Verification Matrix

| Acceptance | Red evidence | Green evidence |
| --- | --- | --- |
| AC-01 | runtime expectations fail before auto-check code | runtime focused tests |
| AC-02 | App contract fails before root dialog/store wiring | shell contract + type check |
| AC-03 | forced-dismiss props absent | release-pages test |
| AC-04 | settings control/handler absent | settings contract + type check |
| AC-05 | call-chain/reference assertions | focused suite + review |
| AC-06 | existing release/foundation contracts | Node/Rust release tests |
| AC-07 | n/a | type, lint, build and focused tests |
| Manual offline truthfulness | native mapper test fails before implementation | Rust mapper + runtime/settings tests |

## Code Review Checklist Contract

- Robustness: candidate null guards; startup rejection contained; manual rejection visible; native/adapted external data only; no unhandled promise.
- Maintainability: no new service/helper file, no mixed function over 150 lines, explicit handler names, existing Composition API pattern.
- Performance/memory: one startup request, store dedupe, one root watch, no polling/cache/debounce, runtime listener cleanup retained.
- Stack anchoring: Vue 3 Composition API, Pinia, Element Plus, Tailwind utilities, existing desktop transport and Tauri updater only.
- Review readiness: every hunk maps to T1-T6/T5B; no unrelated format churn, debug code, unused imports, stale tests or hidden side effects.

## Rollback And Stop Conditions

- If implementation requires changing `UpdateSnapshot`, native commands, signing configuration or product semantics, stop and return to spec/architecture instead of guessing.
- If root visibility cannot avoid reopening after Later without a new persisted preference, keep state local and define transition semantics; do not add localStorage.
- The renderer-only integration can be reverted without database/config migration.
