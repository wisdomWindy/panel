# Task Board

| ID | Task | Status | Ready | Mode | Target | Source | Test |
| --- | --- | --- | --- | --- | --- | --- | --- |
| T01 | Red contract tests | done | yes | serial | shell-components-contract.test.ts | AC01-AC06, P01-P06 | targeted Vitest failed before implementation, passed after |
| T02 | Web startup mode | done | yes | serial after T01 | package.json dev:web | B01/P06 | dev server + browser console passed |
| T03 | Shell redesign | done | yes | serial after T01 | App + toolbar/nav/status | B02/P01-P03/P05 | contract + wide/narrow browser passed |
| T04 | Connection redesign | done | yes | serial after T01 | connection/index.vue | B03/P04-P05 | contract + wide/narrow browser passed |
| T05 | Verify and cleanup | done | yes | serial | scoped closure | AC01-AC07 | test/type/lint/web build/Tauri build/UI passed |

## Execute-readiness Notes

- All user-visible items are existing allowlisted content; no missing product decision.
- State/data/API owners do not change.
- No API contract work and no external data transmission.
- Every task uses S2 scoped context and preserves the existing event/store/router chain.
- Tailwind utilities only; no hidden or generated class strings.
