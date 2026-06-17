---
name: frontend-agent-framework-execute
description: Stage subskill for execution. Implement only approved tasks and keep execution records aligned with the plan.
---

# Execute Subskill

## Purpose

Implement the approved request while keeping execution records aligned with the spec and plan.

## Use With

Invoke from the main `frontend-agent-framework` skill after routing a request with `stage=execute`.

## Read First

- `docs/requests/<request-id>/state.json`
- `docs/requests/<request-id>/spec/spec.md`
- `docs/requests/<request-id>/plan/plan.md`
- `docs/requests/<request-id>/plan/task-board.md`
- `../../references/state-machine.md`
- `../../references/policies/api-contracts.md`
- `../../references/policies/frontend-architecture.md`
- `../../references/policies/frontend-components.md`
- `../../references/policies/testing.md`

## Hard Rules

- Require `approvals.plan_approved=true` before writing code.
- Update `execution/changelog.md` and `plan/task-board.md` as work progresses.
- Implement only approved tasks.
- Treat the approved spec and plan as the authoritative implementation contract.
- For testable behavior changes, execute in red -> green -> refactor order.
- Write or update the failing test first, then implement the minimal passing change, then refactor while keeping tests green.
- Default to serial execution unless the approved plan explicitly marks independent work units as parallel-safe.
- If workflow-style parallel execution is used, keep every parallel unit mapped back to the same request artifacts, approvals, and task board.
- Keep execution observable enough that a human can inspect progress, ask follow-up questions, or redirect the work without losing artifact continuity.
- Preserve the existing `state.json.loop` object; stage-local state updates must not reset or reinterpret it.
- If requirements shift or a plan gap appears, stop and move the request back to `spec` or `plan`.
- When re-entered from failed verification or blocking review, continue from the approved plan and the latest recorded findings instead of treating the request as a fresh task.
- Do not claim completion in this stage.

## Required Inputs

- `docs/requests/<request-id>/spec/spec.md`
- `docs/requests/<request-id>/plan/plan.md`
- `docs/requests/<request-id>/plan/task-board.md`
- `docs/requests/<request-id>/state.json`

## Required Artifacts

- `docs/requests/<request-id>/execution/changelog.md`
- Code changes that map to planned tasks

## Allowed Outputs

- Planned implementation changes
- Execution notes
- Task status updates
- Test-first implementation progress and deviations that are explicitly recorded
- Workflow-style parallel execution records when the approved plan explicitly calls for them
- Evidence of progress and intervention-friendly execution notes

## Forbidden Actions

- Inventing new requirements during coding
- Skipping task tracking
- Declaring the request done
- Writing behavior-first implementation for a testable change without recording why TDD was not applicable
- Treating the spec or plan as optional guidance instead of the implementation contract
- Spawning workflow-style parallel execution for trivial or unplanned work
- Hiding material progress, context switches, or human intervention points outside repository artifacts

## Exit Criteria

- Planned implementation work is complete.
- Major execution decisions and deviations are recorded in `execution/changelog.md`.
- Test-first execution steps or justified exceptions are recorded for behavior changes.
- `plan/task-board.md` reflects the completed execution state.
- `state.json` is updated with `stage` set to `verify` while preserving the existing `loop` object.
- The stage leaves enough updated evidence that `verify` can re-run immediately in the same workflow loop.
