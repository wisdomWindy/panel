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
- If verification fails, move the request back to `execute`.
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
- Failure records that route work back to execution
- TDD exception checks and fallback verification evidence where needed

## Forbidden Actions

- Marking the request complete without evidence
- Ignoring failed acceptance criteria
- Accepting implementation behavior that cannot be traced back to the approved spec

## Exit Criteria

- Every acceptance criterion is checked.
- Evidence is attached or referenced for each verified item.
- Any TDD exception for testable behavior is explicitly justified and backed by alternative evidence.
- Any failures are fixed or explicitly routed back to execution.
- `state.json` is updated with `stage` set to `review`.
