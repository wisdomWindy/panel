# Task Board

| ID | Task | Source | Files | Status |
| --- | --- | --- | --- | --- |
| T1 | Configure real SFC SSR coverage and obtain a valid RED | A1-A4 / U1-U5 | `vite.config.ts`, `views/dark-theme-page-surfaces.test.ts` | completed |
| T2 | Fix dark utilities for the danmaku panel | A1/A2/A4 | `views/danmaku-list/index.vue` | completed |
| T3 | Fix dark utilities for dashboard and TrendChart | A1-A4 | `views/dashboard/index.vue`, `components/live-data/TrendChart.vue` | completed |
| T4 | Fix dark utilities for sensitive words and blacklist | A1/A2/A4 | both owning `index.vue` files | completed |
| T5 | Complete GREEN, chain audit, and full static verification | A5 | scoped diff/tests/type/lint/build | completed |
| T6 | Verify all four routes in dark mode at narrow and desktop viewports | A6 | browser evidence | completed |

Execution was sequential because T2-T4 depended on T1's valid RED baseline. No sub-agent execution was used.

