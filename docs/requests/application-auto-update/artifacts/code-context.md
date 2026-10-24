# 更新链路代码上下文

## Scope

仅分析应用自动检查和手动检查的生产调用链、类型、页面消费者与发布配置。本次跨 runtime/store/root UI/settings，code graph 属于优先能力。检测结果：仓库无 graph 配置或团队指定 bootstrap，环境中也未找到 `codegraph`、`ckg` 或 `codemap` 可执行文件，因此无可安全推断的自动安装源。回退到定向 `rg`、直接 import/caller 阅读和 Git 历史，并已确认完整结构闭包。

## Current Ownership And Flow

```text
App.vue mounted
  -> useReleaseRuntime.start()
  -> releaseStore.bootstrap()
  -> update_status + plugin_list + about_info + resilience_status
  -> listen update://status, plugin://status, plugin://crashed

/update-log or future settings action
  -> releaseStore.checkUpdate()
  -> invokeDesktop('update_check')
  -> ipc::release::update_check
  -> ReleaseState.check_update()
  -> TauriUpdateProvider.check()
  -> tauri-plugin-updater
```

## Owners

| Concern | Current owner | Constraint |
| --- | --- | --- |
| Native candidate/check/download/install | `src-tauri/src/infrastructure/release.rs` | Must remain behind Tauri updater signature verification. |
| Update policy and latest-operation protection | `src-tauri/src/application/release.rs` | Rollout, pause, blacklist and forced rules remain native-owned. |
| Desktop contract | `src/renderer/platform/desktop/contracts/release.ts` | Existing `UpdateSnapshot` is authoritative for renderer code. |
| DTO normalization | `src/renderer/modules/release/adapters.ts` | UI must not bypass runtime validation. |
| Renderer mutable release state | `src/renderer/stores/release.ts` | Single writable owner; `run()` deduplicates operation keys. |
| Root lifecycle/event subscription | `src/renderer/composables/useReleaseRuntime.ts` | Correct owner for one startup check and listener cleanup. |
| Update interaction UI | `src/renderer/views/update-log/UpdateAvailableDialog.vue` | Reuse globally; do not create a parallel modal. |
| Manual entry | `src/renderer/views/settings/index.vue` | Add to the existing About section. |

## Existing Gaps

1. `useReleaseRuntime.startGeneration()` bootstraps state and registers listeners but never calls `store.checkUpdate()`.
2. `UpdateAvailableDialog` is mounted only by `/update-log`; that route is not exposed by the application navigation tree, so startup checks cannot surface a candidate globally.
3. The settings About section displays version/channel but has no check action or result feedback.
4. Forced updates hide footer defer/skip actions, but the default Element Plus close button, modal click and Escape can still dismiss the dialog.

## TypeScript Context

- Governing config: `packages/bilibili-live-tool/tsconfig.json`, with `tsconfig.eslint.json` for the explicit type-check script.
- Material settings: `strict`, `noUnusedLocals`, `noUnusedParameters`, `moduleResolution=bundler`, `target=ES2020`, DOM libs, and `@/* -> ./src/renderer/*`.
- Relevant declarations/contracts: `src/vue-shims.d.ts` and `src/renderer/platform/desktop/contracts/release.ts`.
- No generated backend DTO or new ambient global is required.

## Change-Chain Boundary

Expected production callers after the change:

- auto check: `App.vue -> useReleaseRuntime.start -> releaseStore.checkUpdate`
- manual check: `settings/index.vue -> releaseStore.checkUpdate`
- candidate dialog: root `App.vue -> UpdateAvailableDialog -> releaseStore download/install/skip`
- existing `/update-log` remains compatible and continues to reuse the same store/dialog.

No native command, DTO shape, updater endpoint, signing key, or release workflow change is required.
