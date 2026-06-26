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
- `docs/requests/<request-id>/artifacts/code-context.md` when present
- `../../references/state-machine.md`
- `../../references/templates/code-context.md`
- `../../references/policies/api-contracts.md`
- `../../references/policies/clean-code.md`
- `../../references/policies/code-graph.md`
- `../../references/policies/design-patterns.md`
- `../../references/policies/spec-constraints.md`
- `../../references/policies/frontend-architecture.md`
- `../../references/policies/frontend-components.md`
- `../../references/policies/testing.md`

## Hard Rules

- Require `approvals.plan_approved=true` before writing code.
- Update `execution/changelog.md` and `plan/task-board.md` as work progresses.
- Implement only approved tasks.
- Treat the approved spec and plan as the authoritative implementation contract.
- If the approved plan lacks material field definitions, column definitions, interaction outcomes, state transitions, or loading completion conditions needed for correct implementation, stop execution and route the request back to `plan` instead of guessing.
- If the approved spec and approved plan disagree on behavior granularity or product meaning, stop execution and route the request back to `spec` or `plan` instead of choosing one interpretation ad hoc.
- Treat the spec's declared design constraints as part of that contract, especially for boundaries, side effects, duplication ownership, readability guardrails, and pattern decisions.
- For testable behavior changes, execute in red -> green -> refactor order.
- Write or update the failing test first, then implement the minimal passing change, then refactor while keeping tests green.
- Apply the clean-code policy while implementing: use intention-revealing names, keep units focused, reduce rule-level duplication, and keep side effects explicit.
- Apply approved pattern decisions deliberately; if the plan chose Strategy, State, Adapter, Observer, Command, or another pattern family, implement that intent clearly instead of drifting into ad hoc branching or accidental abstractions.
- Before or during non-trivial existing-code modification, prefer code graph to confirm impact scope, call sites, or ownership boundaries when that reduces the risk of incomplete or structurally inconsistent edits.
- Update `artifacts/code-context.md` instead of ad hoc notes when execution discovers new impact scope, call sites, or structural blind spots.
- Default to serial execution unless the approved plan explicitly marks independent work units as parallel-safe.
- If workflow-style parallel execution is used, keep every parallel unit mapped back to the same request artifacts, approvals, and task board.
- Keep execution observable enough that a human can inspect progress, ask follow-up questions, or redirect the work without losing artifact continuity.
- Preserve the existing `state.json.loop` object; stage-local state updates must not reset or reinterpret it.
- If requirements shift or a plan gap appears, stop and move the request back to `spec` or `plan`.
- When re-entered from failed verification or blocking review, continue from the approved plan and the latest recorded findings in the same active workflow run instead of treating the request as a fresh task.
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
- Targeted clean-code refactors that are necessary to keep the touched area maintainable and aligned with the approved plan
- Pattern-conforming implementation changes that reflect approved abstraction choices and boundary decisions
- Graph-supported impact analysis notes or fallback analysis notes when structural context is material to the change
- Durable code-context artifact updates when execution materially changes the understanding of impact scope
- Workflow-style parallel execution records when the approved plan explicitly calls for them
- Evidence of progress and intervention-friendly execution notes

## Forbidden Actions

- Inventing new requirements during coding
- Guessing omitted product behavior that the plan should have defined explicitly
- Choosing between conflicting spec and plan behavior interpretations without first repairing the artifacts
- Skipping task tracking
- Declaring the request done
- Writing behavior-first implementation for a testable change without recording why TDD was not applicable
- Treating the spec or plan as optional guidance instead of the implementation contract
- Leaving behind known misleading names, hidden side effects, or duplicated business rules in the touched area when the current change already depends on understanding or editing that area
- Inventing a new pattern layer during coding without updating the approved design artifacts
- Spawning workflow-style parallel execution for trivial or unplanned work
- Hiding material progress, context switches, or human intervention points outside repository artifacts

## Exit Criteria

- Planned implementation work is complete.
- Major execution decisions and deviations are recorded in `execution/changelog.md`.
- Test-first execution steps or justified exceptions are recorded for behavior changes.
- The changed area remains readable enough for review, with any necessary clean-code refactors completed or explicitly tracked in the plan and changelog.
- Any introduced pattern remains traceable to an approved problem statement and is expressed clearly enough for review to judge whether it earned its complexity.
- `plan/task-board.md` reflects the completed execution state.
- `state.json` is updated with `stage` set to `verify` while preserving the existing `loop` object.
- The stage leaves enough updated evidence that `verify` can re-run immediately in the same workflow loop.
