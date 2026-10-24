# Execution Changelog

## Delivery Unit

`theme-switch-bilibili-accent`

## Changes

- Added a Tailwind 4 class-based `dark` variant so Tailwind and Element Plus use the root `.dark` state.
- Updated the primary palette and renderer/Rust defaults to bilibili pink `#FB7299`.
- Projected the Element Plus base, RGB, light, and dark primary variables from the active accent.
- Added a lifecycle-owned system color-scheme listener with system-only reprojection and stop cleanup.
- Migrated only the exact legacy default `#FF6699`; preserved other custom colors.
- Replaced `structuredClone` calls on Vue proxies with a typed, detached UI preference clone. This was required for the existing preview/save chain to execute in WebView2.

## TDD Evidence

- Initial focused RED: 4 failures covering the missing class variant, derived Element Plus colors, legacy migration, and system listener.
- Rust RED: the default accent assertion observed `#FF6699` instead of `#FB7299`.
- Installed runtime diagnosis: `structuredClone(shell.savedUi)` reproduced `DataCloneError`; the new reactive-clone contract failed before implementation and passed afterward.
- Focused GREEN: 4 files, 18 tests passed after the final repair.

## Scope

All product-facing changes map to U1-U4. The proxy-clone repair is a technical prerequisite for the already-approved preview/save interaction and has `product-content delta: none`.
