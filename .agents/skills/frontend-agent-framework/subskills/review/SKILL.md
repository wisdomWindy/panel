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
- `../../references/state-machine.md`
- `../../references/templates/review.md`
- `../../references/policies/api-contracts.md`
- `../../references/policies/frontend-architecture.md`
- `../../references/policies/frontend-components.md`
- `../../references/policies/testing.md`

## Hard Rules

- Read the changed code before reviewing.
- Write `review/review.md`.
- Separate findings into blocking issues, non-blocking issues, accepted risks, and follow-up items.
- Preserve the existing `state.json.loop` object; stage-local state updates must not reset or reinterpret it.
- If blocking issues remain, record them and hand control back to the main skill so it can route the request back to `execute` with the correct loop re-entry metadata and continue the workflow immediately.
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
- Follow-up items that do not block completion
- Blocking findings that can drive the next `execute` pass without extra user restatement

## Forbidden Actions

- Collapsing review into a one-line approval note
- Declaring merge readiness when blockers remain

## Exit Criteria

- `review/review.md` exists and records findings by severity.
- No blocking issues remain unresolved.
- `state.json` is updated with `stage` set to `complete` while preserving the existing `loop` object.
