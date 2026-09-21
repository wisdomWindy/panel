# Scoped Code Context

## Feature Chain

- bootstrap：`package.json dev:web` -> Vite mode -> `build-channel.ts` -> `main.ts` mount。
- shell：`App.vue` owns local input/error/popover state, composes toolbar/navigation/router-view/status and starts runtimes.
- connection：Toolbar emits `toggleConnection` -> `App.toggleConnection` -> `normalizeRoomInput` -> room store `connect/disconnect` -> desktop command -> snapshot -> toolbar/connection page/status projections.
- navigation：`navigation.ts` tree -> App computed -> `AppShellNavigation` emit -> router push -> router-view.
- status：stores -> `buildStatusBarView` -> `AppShellStatusBar`.

## Direct References / Tests

- `components/application-shell/shell-components-contract.test.ts` reads App and shell SFC sources.
- `modules/application-shell/shell-contract.test.ts` covers navigation tree, room normalization and status projection.
- No code graph needed: scoped import/caller chain is explicit and no owner/dependency change is planned.

## Governing Types / Configuration

- `tsconfig.eslint.json -> tsconfig.json`; strict/noUnused/bundler/ES2020/DOM/Vue plugin.
- `RoomSnapshot`, `ShellNavigationItem`, `StatusBarItem` remain authoritative local types.
- Vite `vite/client` declares imported image modules.

## Root-cause Evidence

- Browser console under current `pnpm dev:web`: `VITE_BUILD_CHANNEL must be either free or member`; application does not mount.
- When started with `vite --mode free`, page mounts but dark theme shows low-contrast connection title/status and the shell is visually flat.
- Root `v-loading` is a full-shell overlay risk during bootstrap; existing behavior does not require hiding the shell.

## Change Boundary

Only startup mode, app-shell presentation, connection-page presentation, and their direct contract checks. Stores, DTOs, commands, routes, navigation tree, i18n content and business formatters remain unchanged.
