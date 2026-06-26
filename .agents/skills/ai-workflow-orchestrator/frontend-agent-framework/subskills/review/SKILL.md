---
name: frontend-agent-framework-review
description: Stage subskill for review. Review correctness, standards compliance, and residual risk before completion.
---

# Review Subskill

## Purpose

Perform the final review pass against the code, spec, plan, and verification output before completion.

## Use With

Invoke from the main `frontend-agent-framework` skill after routing a request with `stage=review`.

## Read First

- `docs/requests/<request-id>/state.json`
- `docs/requests/<request-id>/spec/spec.md`
- `docs/requests/<request-id>/plan/plan.md`
- `docs/requests/<request-id>/verification/verification.md`
- `docs/requests/<request-id>/artifacts/code-context.md` when present
- `../../references/state-machine.md`
- `../../references/templates/review.md`
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

- Read the changed code before reviewing.
- Write `review/review.md`.
- Separate findings into blocking issues, non-blocking issues, accepted risks, and follow-up items.
- Review changed code against the clean-code policy, not just against functional correctness.
- Review any introduced or modified pattern use against the design-pattern policy, including whether the abstraction was necessary and correctly fitted to the problem.
- Review the implementation against the spec's declared design constraints instead of treating maintainability and abstraction judgments as free-floating preferences.
- Review whether the approved spec and approved plan stayed aligned at the same function-complete behavior granularity, and treat material drift as a blocking workflow issue rather than a documentation nit.
- Write explicit pass or fail conclusions for `clean-code assessment` and `design-pattern assessment`.
- When the review depends on dependency direction, ownership boundaries, side-effect spread, or actual abstraction fan-out, prefer code graph evidence over impressionistic reasoning.
- If review extends or contradicts earlier structural understanding, update `artifacts/code-context.md` so the final artifact reflects the current best view of the code structure.
- Preserve the existing `state.json.loop` object; stage-local state updates must not reset or reinterpret it.
- If blocking issues remain, record them and hand control back to the main skill so it can route the request back to `execute` with the correct loop re-entry metadata and continue the workflow immediately in the same active run.
- Do not mark the request complete while blockers remain.

## Required Inputs

- `docs/requests/<request-id>/spec/spec.md`
- `docs/requests/<request-id>/plan/plan.md`
- `docs/requests/<request-id>/verification/verification.md`
- `docs/requests/<request-id>/state.json`

## Required Artifacts

- `docs/requests/<request-id>/review/review.md`

## Allowed Outputs

- Final review findings
- Merge readiness judgment
- Spec-plan alignment findings where behavior granularity drift changed or risked changing implementation meaning
- Follow-up items that do not block completion
- Clean-code findings covering naming, responsibility boundaries, duplication, side effects, and control-flow readability
- Design-pattern findings covering fit, overengineering risk, dependency direction, and whether the abstraction simplified future change
- Code-graph-backed structural review findings when graph analysis is available
- Final structural assessment aligned with the durable code-context artifact
- Explicit pass/fail compliance conclusions for clean-code and design-pattern assessments
- Blocking findings that can drive the next `execute` pass without extra user restatement

## Forbidden Actions

- Collapsing review into a one-line approval note
- Ignoring maintainability problems that materially affect readability or safe future change on the grounds that the feature already works
- Accepting unnecessary pattern layers, fake extensibility, or framework-shaped indirection without checking whether they solve a real problem
- Treating clean-code or design-pattern compliance as implicit without a written pass/fail judgment
- Treating material spec-plan granularity drift as harmless when it changes, obscures, or weakens the approved behavior contract
- Declaring merge readiness when blockers remain

## Exit Criteria

- `review/review.md` exists and records findings by severity.
- The review explicitly states whether clean-code blockers remain in the changed area.
- The review explicitly states whether the introduced pattern choices were justified, overbuilt, or unnecessary.
- `review/review.md` explicitly states `clean-code assessment: pass|fail` and `design-pattern assessment: pass|fail`.
- The review explicitly states whether spec and plan remained aligned at the required behavior granularity for the changed scope.
- No blocking issues remain unresolved.
- `state.json` is updated with `stage` set to `complete` while preserving the existing `loop` object.
