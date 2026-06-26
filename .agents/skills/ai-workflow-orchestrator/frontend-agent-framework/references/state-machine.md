# State Machine

The lifecycle stage is the delivery state. The loop is the control mechanism that decides how the workflow advances, recovers gates, or re-enters a prior stage.
The goal contract defines completion. Workflow-style parallelization, when used, only affects execution scale and does not add new states.

## Stages

- intake
- page-design
- spec
- plan
- execute
- verify
- review
- complete

## Loop States

- running
- blocked
- complete

## Required Gates

- page design artifact before `spec` for page-oriented requests
- `spec_approved` before `plan`
- `plan_approved` before `execute`
- verification artifact before `review`
- review artifact with no blockers before `complete`

## Loop Decision Table

| Current condition | Loop action | Result |
| --- | --- | --- |
| Stage exit criteria satisfied and next gate satisfied | advance stage | enter the next lifecycle stage |
| Gate missing but recoverable from repository state, artifacts, or evidence | recover gate | stay in the same lifecycle position, repair the prerequisite, then re-check the gate |
| Gate missing and depends on user approval or external resource | block | set `loop.state=blocked`, set `loop.pending_gate`, wait for recovery |
| Verification fails but implementation can continue under the approved plan | re-enter execute | route to `execute` immediately and continue the loop |
| Review finds blockers but implementation can continue under the approved plan | re-enter execute | route to `execute` immediately and continue the loop |
| Request reaches completion conditions | complete | set `stage=complete` and `loop.state=complete` |

## Goal and Workflow Constraints

- The request may reach `complete` only when the goal contract is satisfied and verified.
- Loop iterations do not themselves prove completion.
- Workflow-style parallel execution does not create additional lifecycle states or bypass gates.
- A proactive workflow must declare its trigger, minimum context, and handoff path before the team treats it as reliable automation.

## Re-entry Rules

- `resume_request`: re-enter the loop for an in-flight request that already has `state.json`
- `gate_recovery`: re-enter after a previously blocking gate was satisfied
- `requirement_changed`: route back to `spec` or `plan`
- `verification_failed`: route back to `execute`
- `review_blocked`: route back to `execute`
- `manual_reopen`: re-enter a previously completed request after an explicit reopen decision
- Every re-entry above must increment `loop.iteration` when the main orchestrator starts the next decision cycle.

## Rollback Rules

- Requirement change during execute: back to `spec` or `plan` with `reentry_reason=requirement_changed`
- Verification failure: back to `execute` with `reentry_reason=verification_failed`
- Blocking review finding: back to `execute` with `reentry_reason=review_blocked`
- `execute -> verify -> execute -> verify` repeats until verification passes or a real blocking gate is reached
- These rollback paths are same-run loop continuations unless a real blocking gate requires waiting.
