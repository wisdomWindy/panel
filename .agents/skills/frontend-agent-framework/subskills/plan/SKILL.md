---
name: frontend-agent-framework-plan
description: Stage subskill for planning. Convert the approved spec into execution tasks, dependencies, and verification strategy.
---

# Plan Subskill

## Purpose

Break the approved spec into concrete execution tasks, dependencies, and verification strategy.

## Use With

Invoke from the main `frontend-agent-framework` skill after routing a request with `stage=plan`.

## Read First

- `docs/requests/<request-id>/state.json`
- `docs/requests/<request-id>/spec/spec.md`
- `docs/requests/<request-id>/spec/clarifications.md`
- `../../references/state-machine.md`
- `../../references/templates/plan.md`
- `../../references/policies/doc-writing.md`
- `../../references/policies/testing.md`

## Hard Rules

- Require `approvals.spec_approved=true` before starting this stage.
- Write `plan/plan.md` and `plan/task-board.md`.
- Unless the user explicitly requests another language, write `plan/plan.md` and `plan/task-board.md` in Chinese.
- Use Chinese for titles, section names, task labels, execution notes, dependency descriptions, testing strategy, and rollback notes; code ids, paths, field names, and API identifiers may remain in their original language where needed for accuracy.
- Map every task back to the approved spec.
- Decompose testable behavior changes into TDD-oriented execution units.
- For each behavior change, define the intended verification path before implementation begins.
- Distinguish serial work from independently parallelizable work.
- Recommend workflow-style parallel execution only when the task set is large enough, independent enough, and valuable enough to justify the extra coordination and token cost.
- Make trigger handling, context loading, observation points, and human intervention points explicit when the request describes a proactive workflow.
- Preserve the existing `state.json.loop` object; stage-local state updates must not reset or reinterpret it.
- Do not write implementation code.
- Require user approval before setting `approvals.plan_approved` to `true`.
- When the environment supports structured approval UI, present plan approval through explicit options such as approve / reject / revise instead of asking the user to type approval text manually.
- Only use plain-text approval prompts when structured approval UI is unavailable.

## Sample Usage

- Use `docs/requests/prd-material-delivery-config-center/plan/plan.md` and `plan/task-board.md` only as examples of decomposition quality and task-board style.
- Build the new plan from the approved current spec, not from the sample request's final outcome.
- Reuse numbering and follow-up recording patterns only when they fit the current request.
- Do not assume the sample's task count, dependency order, or module split applies to another request.

## Required Inputs

- `docs/requests/<request-id>/spec/spec.md`
- `docs/requests/<request-id>/spec/clarifications.md`
- `docs/requests/<request-id>/state.json`

## Required Artifacts

- `docs/requests/<request-id>/plan/plan.md`
- `docs/requests/<request-id>/plan/task-board.md`

## Allowed Outputs

- Task breakdown
- Dependency ordering
- Parallelization guidance
- Test and validation strategy
- Risk and rollback notes
- Test-first execution sequencing for behavior changes
- Trigger/context/handoff planning for proactive workflows

## Forbidden Actions

- Writing code
- Creating tasks not justified by the spec
- Entering execution without user approval
- Reusing the sample plan as a shortcut instead of planning from the current approved spec
- Deferring test design until after implementation for behavior that is reasonably testable
- Recommending workflow-style parallel execution for trivial, tightly coupled, or single-thread work
- Leaving trigger conditions, required context, or human intervention points implicit for a workflow that depends on them

## Exit Criteria

- `plan/plan.md` exists and covers tasks, dependencies, affected areas, testing, and rollback notes.
- `plan/task-board.md` exists and reflects the planned execution units.
- The plan artifacts are written in Chinese unless the user explicitly requested another language.
- The plan maps acceptance criteria to concrete execution and verification tasks, including test-first steps where behavior is testable.
- The plan states which tasks stay serial and which tasks are safe candidates for workflow-style parallel execution.
- The plan defines the workflow's trigger handling, minimum context, observation points, and human handoff path when those concerns apply.
- The user approves the plan, preferably through structured approval options when available.
- `state.json` is updated with `approvals.plan_approved=true` and `stage` set to `execute` while preserving the existing `loop` object.
