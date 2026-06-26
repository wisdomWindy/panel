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
- `docs/requests/<request-id>/artifacts/code-context.md` when present
- `../../references/state-machine.md`
- `../../references/templates/plan.md`
- `../../references/templates/code-context.md`
- `../../references/policies/design-patterns.md`
- `../../references/policies/code-graph.md`
- `../../references/policies/doc-writing.md`
- `../../references/policies/clean-code.md`
- `../../references/policies/spec-constraints.md`
- `../../references/policies/testing.md`

## Hard Rules

- Require `approvals.spec_approved=true` before starting this stage.
- Write `plan/plan.md` and `plan/task-board.md`.
- Unless the user explicitly requests another language, write `plan/plan.md` and `plan/task-board.md` in Chinese.
- Use Chinese for titles, section names, task labels, execution notes, dependency descriptions, testing strategy, and rollback notes; code ids, paths, field names, and API identifiers may remain in their original language where needed for accuracy.
- Map every task back to the approved spec.
- Preserve spec granularity: `plan` must decompose work at the same function-complete behavior granularity already defined in the approved spec, rather than inventing new product behavior or silently refining an under-specified spec.
- Decompose tasks to a function-complete level instead of stopping at module or page names.
- For each task, write enough delivery detail that `execute` can complete the requested functionality without guessing omitted fields, views, interactions, or states.
- When a task covers a module, page, panel, drawer, modal, card, table, form, detail view, or display block, explicitly break down the subfunctions inside that unit instead of treating the container as the whole task.
- For form-bearing work, record the form purpose, field list, field order when material, required vs optional status, control type, value semantics, default value, validation rules, disabled/editable conditions, visibility conditions, option sources, submission trigger, failure handling, and loading lifecycle.
- For table-bearing work, record the table purpose, column list, column order, column meaning, value source, formatting rule, empty-state behavior, row-level actions, batch actions, pagination or scrolling mode when material, and loading lifecycle.
- For pure display content, record the displayed fields, grouping or layout structure, formatting rule, hidden/empty-state behavior, and whether the content is read-only, collapsible, progressive, or conditionally visible.
- For interactions, record the user trigger, preconditions, resulting UI change, side effect, confirmation requirement, loading behavior, success feedback, failure feedback, and exact condition that ends loading or disables retry.
- For workflow or multi-step behavior, record the end-to-end path, branch conditions, state transitions, retry or rollback path, and cross-page or cross-module handoff points.
- If any material product detail is still unknown, record it as a blocking clarification or approved assumption in the plan instead of leaving `execute` to infer it silently.
- If the approved spec is too coarse to support function-complete planning, stop and route the request back to `spec` instead of compensating by inventing missing behavior inside the plan.
- Decompose testable behavior changes into TDD-oriented execution units.
- For each behavior change, define the intended verification path before implementation begins.
- Plan the change so module boundaries, responsibilities, duplication control, and side-effect containment stay explicit instead of being deferred to ad hoc cleanup during execution.
- When the spec shows a real pattern-level pressure, choose the simplest fitting pattern family and record why it is better than direct code or another pattern option.
- Treat declared spec constraints as mandatory planning inputs, not as optional commentary.
- For existing-code changes with non-trivial impact scope, prefer code graph to identify affected symbols, callers, consumers, and neighboring modules before finalizing the plan.
- Reuse and extend `artifacts/code-context.md` when the plan stage confirms impact scope, neighboring modules, or verification surface.
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
- `docs/requests/<request-id>/artifacts/code-context.md` when structural analysis is needed

## Allowed Outputs

- Task breakdown
- Function-complete task decomposition with subfunction coverage
- Dependency ordering
- Parallelization guidance
- Test and validation strategy
- Risk and rollback notes
- Test-first execution sequencing for behavior changes
- Clean-code guardrails for naming, responsibilities, dependency boundaries, and likely refactor points
- Pattern decisions, their problem statements, and the code areas where those abstractions should live
- Code-context impact scope, including affected symbols, modules, and likely verification surface when graph-based analysis is available
- Structured code-context updates that preserve graph detection, bootstrap, and fallback records in one durable artifact
- Trigger/context/handoff planning for proactive workflows

## Forbidden Actions

- Writing code
- Creating tasks not justified by the spec
- Entering execution without user approval
- Reusing the sample plan as a shortcut instead of planning from the current approved spec
- Deferring test design until after implementation for behavior that is reasonably testable
- Leaving obvious complexity, duplication, or mixed-responsibility hotspots unplanned when they are clearly part of the approved change
- Introducing a pattern in the plan without naming the concrete change axis, selection pressure, or rejected simpler alternative
- Recommending workflow-style parallel execution for trivial, tightly coupled, or single-thread work
- Leaving trigger conditions, required context, or human intervention points implicit for a workflow that depends on them
- Leaving page structure, field definitions, column definitions, interaction outcomes, or loading-state rules unspecified when the approved scope depends on them
- Letting `execute` infer missing product behavior that should have been defined or bounded in the plan
- Refining the approved spec into materially new behavior detail without recording that the request must first return to `spec`

## Exit Criteria

- `plan/plan.md` exists and covers tasks, dependencies, affected areas, testing, and rollback notes.
- `plan/plan.md` decomposes each planned unit to a function-complete level, including the required UI structure, data fields, interactions, states, and flow details for the scoped work.
- `plan/task-board.md` exists and reflects the planned execution units.
- The plan artifacts are written in Chinese unless the user explicitly requested another language.
- The plan maps acceptance criteria to concrete execution and verification tasks, including test-first steps where behavior is testable.
- The plan's task granularity matches the approved spec's function-complete behavior granularity instead of drifting coarser or finer in a way that changes product meaning.
- The plan does not rely on `execute` to guess omitted form fields, table columns, display fields, interaction effects, or loading-state completion conditions.
- The plan records any required clean-code guardrails for the touched modules, especially around naming, responsibility splits, duplication risks, and side-effect boundaries.
- The plan records any approved pattern decisions, why they are needed, and what simpler alternative was rejected.
- The plan states which tasks stay serial and which tasks are safe candidates for workflow-style parallel execution.
- The plan defines the workflow's trigger handling, minimum context, observation points, and human handoff path when those concerns apply.
- The user approves the plan, preferably through structured approval options when available.
- `state.json` is updated with `approvals.plan_approved=true` and `stage` set to `execute` while preserving the existing `loop` object.
