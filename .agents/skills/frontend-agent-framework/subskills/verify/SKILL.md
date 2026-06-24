---
name: frontend-agent-framework-verify
description: Stage subskill for verification. Validate the implementation against acceptance criteria and record durable evidence.
---

# Verify Subskill

## Purpose

Validate the implementation against the approved spec and produce durable evidence.

## Use With

Invoke from the main `frontend-agent-framework` skill after routing a request with `stage=verify`.

## Read First

- `docs/requests/<request-id>/state.json`
- `docs/requests/<request-id>/spec/spec.md`
- `docs/requests/<request-id>/plan/plan.md`
- `docs/requests/<request-id>/execution/changelog.md`
- `../../references/state-machine.md`
- `../../references/templates/verify.md`
- `../../references/policies/testing.md`
- `../../references/policies/doc-writing.md`

## Hard Rules

- Write `verification/verification.md`.
- Map each acceptance criterion to verification method, result, and evidence.
- Verify that implemented behavior matches the approved spec rather than undocumented intent.
- Confirm that testable behavior changes were covered by tests or explicitly justified exceptions.
- Verify that the workflow used the intended trigger/context/handoff assumptions when those assumptions are part of the approved spec.
- Write an explicit `spec constraint compliance` conclusion in `verification/verification.md` with a pass or fail judgment.
- Preserve the existing `state.json.loop` object; stage-local state updates must not reset or reinterpret it.
- If verification fails, record the failure clearly and hand control back to the main skill so it can route the request back to `execute` with the correct loop re-entry metadata and continue the workflow immediately.
- Do not claim completion without verification artifacts.

## Required Inputs

- `docs/requests/<request-id>/spec/spec.md`
- `docs/requests/<request-id>/plan/plan.md`
- `docs/requests/<request-id>/execution/changelog.md`
- `docs/requests/<request-id>/state.json`

## Required Artifacts

- `docs/requests/<request-id>/verification/verification.md`
- Optional evidence files under `docs/requests/<request-id>/verification/evidence/`

## Allowed Outputs

- Acceptance coverage mapping
- Verification evidence
- Spec-constraint compliance judgment with pass/fail and supporting evidence
- Failure records that route work back to execution
- TDD exception checks and fallback verification evidence where needed
- Handoff readiness evidence for workflows that produce outputs for people or downstream systems

## Forbidden Actions

- Marking the request complete without evidence
- Ignoring failed acceptance criteria
- Advancing as if spec-constraint compliance is implied without writing an explicit pass/fail conclusion
- Accepting implementation behavior that cannot be traced back to the approved spec

## Exit Criteria

- Every acceptance criterion is checked.
- Evidence is attached or referenced for each verified item.
- `verification/verification.md` explicitly states whether spec constraints pass or fail.
- Any TDD exception for testable behavior is explicitly justified and backed by alternative evidence.
- Any failures are fixed or explicitly routed back to execution.
- Any required handoff or downstream-ready result is explicitly verified when part of the approved workflow contract.
- `state.json` is updated with `stage` set to `review` while preserving the existing `loop` object when verification passes.
- A failed verification result is documented precisely enough that `execute` can continue without waiting for a new user restatement.
