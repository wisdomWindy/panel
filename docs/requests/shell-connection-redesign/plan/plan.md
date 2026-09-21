# Execution Plan: shell-connection-redesign

## Preconditions

- spec approved：yes
- architecture design：not required; no owner, state topology, dependency or public API change
- speed profile：S2 scoped
- execution：serial, because tests and implementation touch the same component closure

## T01 Red Contract Tests

- source：spec AC01-AC06；P01-P06。
- product-content delta：none。
- file：`components/application-shell/shell-components-contract.test.ts`。
- add assertions for app icon, root `aria-busy`, absence of root `v-loading`, responsive navigation width/text treatment, connection page dark-mode/readability structure and package `dev:web` free mode.
- run targeted Vitest and capture expected failure before implementation.

## T02 Web Startup Fix

- source：B01 / AC01 / P06。
- product-content delta：none。
- file/symbol：`package.json -> scripts.dev:web`。
- set command to `vite --mode free --host 127.0.0.1`; keep dev:free/member/Tauri scripts unchanged.
- verify direct start produces mounted page and no build-channel error.

## T03 Application Shell Redesign

- source：B02 / AC02-AC06 / P01-P03/P05。
- files：`App.vue`, three `AppShell*.vue` components。
- `App.vue`：keep all state/functions/runtime calls; replace root overlay with `aria-busy`; use stable 64/body/36 layout, neutral surfaces, responsive navigation/content sizing.
- Toolbar：import existing Tauri icon; render brand, preserve select/button/popover/settings events; improve grouping and dark surfaces. Do not change emitted event contract.
- Navigation：preserve input props/emits/menu semantics; use 232px desktop / 72px compact rail, active row and icon sizing; hide text only at compact breakpoint. Preserve groups and disabled states.
- Status：preserve seven items and values; refine separators/surfaces and connection tone without changing projection.
- product-content delta：only re-presentation of allowlisted existing controls and labels, no new copy/behavior.

## T04 Connection Page Redesign

- source：B03 / field table / AC02-AC06 / P04-P05。
- file：`views/connection/index.vue`。
- preserve computed status mapping, store owner, error alert, field values and `formatCount`.
- use existing icons for title/status/field groups; create un-nested status band and three repeated summary cards with <=8px radius; add dark/readable states.
- status color binding may only toggle small approved tone classes; no helper/utility/component extraction.
- product-content delta：re-presentation of existing labels/status/values only.

## T05 Verification And Cleanup

- source：AC01-AC07。
- targeted tests：application shell component + shell contract.
- static checks：type-check, lint:check, build:web:free.
- browser：1280x720 and 900x720 screenshots/DOM; inspect errors/warnings; click dashboard, connection and settings navigation; exercise room validation without transmitting external data.
- chain recheck：all original events, router calls, stores and projections remain referenced; no unused imports or stale contract assertions.

## Code Review Checklist Contract

- robustness：retain room input normalization, null output, caught command failure and button pending/disabled boundaries.
- maintainability：components keep current ownership; no new helper/constant/SFC; no mixed orchestration moved into views.
- performance/memory：no new subscriptions/watchers/listeners/timers; existing runtime cleanup unchanged; no optimization required.
- stack anchoring：Vue 3 script setup, Pinia, Vue Router, Element Plus icons/components and Tailwind utilities only.
- human review：no unrelated formatting; each production hunk maps to T02-T04; no debug code/dead code/stale comments/unused symbol.

## Pattern / Component / Functional Decisions

- Level 0 direct code; no new pattern layer.
- Existing shell component boundaries remain; connection subregions stay inline (one caller, no separate lifecycle).
- No new business transform. Existing status computed and formatter remain pure/derived; effects remain App/store/lifecycle owners.

## Rollback / Stop Conditions

- If required behavior needs new copy/field/action, stop and return to intake.
- If component owner/public contract must change, return to spec/architecture routing.
- If visual solution requires scoped CSS or new dependency, return to plan rather than bypassing styling policy.

## Verification Commands

```text
rtk pnpm --dir packages/bilibili-live-tool exec vitest run --mode free src/renderer/components/application-shell/shell-components-contract.test.ts src/renderer/modules/application-shell/shell-contract.test.ts
rtk pnpm --dir packages/bilibili-live-tool type-check
rtk pnpm --dir packages/bilibili-live-tool lint:check
rtk pnpm --dir packages/bilibili-live-tool build:web:free
rtk pnpm --dir packages/bilibili-live-tool dev:web
```
