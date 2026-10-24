# Code Context

## Scope

- Workspace: `packages/bilibili-live-tool`
- Frontend: Vue 3, Pinia, Vue Router, TypeScript, Vitest
- Desktop backend: Tauri 2, Rust, SQLx/SQLite
- Governing TypeScript config: `packages/bilibili-live-tool/tsconfig.json`
- Relevant compiler constraints: `strict`, `noUnusedLocals`, `noUnusedParameters`, bundler module resolution, `@/* -> src/renderer/*`

## Dependency Chain

1. `router/index.ts` and application-shell navigation expose `/plugins`.
2. `views/plugin-manager` consumes the release store.
3. The release store and runtime consume plugin adapters and desktop command/event contracts.
4. Tauri IPC forwards commands/events to `ReleaseState` and `PluginProvider`.
5. Paths, permissions, configuration import/export, and SQLite `t_plugin` support the same capability.

## Frontend Ownership

- Entry and navigation: `router/index.ts`, `modules/application-shell/navigation.ts`, `AppShellNavigation.vue`.
- State and event lifecycle: `stores/release.ts`, `composables/useReleaseRuntime.ts`.
- Contract normalization: `modules/release/adapters.ts`, `platform/desktop/contracts/release.ts`.
- Shared contract branches: `platform/desktop/contracts/base.ts`, `platform/desktop/contracts/shell.ts`, `modules/application-shell/adapters.ts`.
- Feature-owned UI: `views/plugin-manager/*`.
- Product copy: `locales/zh-CN.json`, `locales/en-US.json`.

## Backend Ownership

- Domain validation: `src-tauri/src/domain/release.rs`.
- Application state/ports: `application/release.rs`, `application/release_ports.rs`.
- Provider and IPC: `infrastructure/release.rs`, `ipc/release.rs`, `ipc/error.rs`, `lib.rs`.
- Shared contracts: `application/paths.rs`, `build_channel.rs`, `domain/shell.rs`, `permissions/foundation.toml`.
- Persistence/config: `infrastructure/database.rs`, `infrastructure/shell.rs`, migrations.

## Persistence Fact

`0001_initial.sql` creates `t_plugin`. SQLx migration history is checksum-sensitive, so the published migration must remain immutable. A new migration must drop `t_plugin` for both upgraded and newly initialized databases.

## Reuse And Dependency Scan

- Plugin adapter/validator/state/provider code is feature-specific and has no valid consumer after removal.
- Update/about/resilience code shares release files but not plugin semantics; it must remain.
- Tauri updater, dialog, notification, global-shortcut, and log crates are framework integrations for other features and must remain.
- No new shared abstraction is warranted; deletion should preserve the existing layer boundaries for surviving release behavior.

## Dirty Worktree Constraint

The repository contains unrelated modified and untracked files. Implementation must use targeted hunks, delete only feature-owned plugin files, and avoid formatting or regenerating unrelated files.

