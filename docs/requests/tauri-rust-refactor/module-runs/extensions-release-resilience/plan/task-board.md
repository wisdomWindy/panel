# Extensions Release And Resilience Task Board

## 2026-09-16 Re-entry

| Task | Status | Contract |
| --- | --- | --- |
| T06 Release script contracts and red tests | completed | ER-S13, ER-S14, ER-S17, ER-S18, ER-S20 |
| T07 Native Tauri updater provider | completed | ER-S15, ER-S16 |
| T08 Signed dual-channel GitHub release | completed | ER-S13, ER-S14, ER-S16, ER-S20 |
| T09 Official runtime artifact catalog | completed | ER-S17, ER-S18, ER-S19 |
| T10 Verification and review | completed | amendment acceptance 1-5 |

## T01 Contract And Red Tests

- status: completed
- covers: ER-S03..11
- outputs: Rust/TS contracts, rule/adapter/registry red tests

## T02 Rust State, IPC, And Resilience

- status: completed
- prerequisite: T01
- covers: ER-S02..04, ER-S06..11
- outputs: domain/application/infrastructure/IPC, migration backup retention

## T03 Frontend Runtime And Pages

- status: completed
- prerequisite: T02
- covers: ER-S01/05/08/10
- outputs: adapters/store/runtime/update/plugin/About UI

## T04 Legacy Cleanup And Compliance

- status: completed
- prerequisite: T03
- covers: ER-S06/10/12
- outputs: Electron/browser plugin/update dependency closure removed

## T05 Verification And Review

- status: completed
- prerequisite: T04
- covers: all acceptance criteria
- outputs: verification/review/changelog and request completion

## Global Constraints

- source-grounded, type-first, fail-fast, strict null, immutable DTO/snapshot, no `any`, no fabricated success.
- Level 2 existing architecture; pure rules/adapters, explicit side-effect owners, no manager/factory/event bus.
- Tailwind utility styling only; approved dialog extraction boundaries only.
- Existing PRD working-tree change is preserved.
