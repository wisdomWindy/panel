# State Machine

The lifecycle stage is the delivery state. The loop is the control mechanism that decides how the workflow advances, recovers gates, or re-enters a prior stage.
The goal contract defines completion. Workflow-style parallelization, when used, only affects execution scale and does not add new states.

## Stages

- intake
- requirement-splitting
- architecture-design
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

- architecture design artifact before `spec` for backend-structural requests
- requirement-splitting artifacts before `architecture-design` or `spec` for PRD-driven requests
- raw snapshot plus Markdown-normalized snapshot before `spec` for every requirement-splitting module that required Markdown normalization
- `spec_approved` before `plan` (implicit by default inside the same active run; explicit only when a manual checkpoint is required)
- `plan_approved` before `execute` (implicit by default inside the same active run; explicit only when a manual checkpoint is required)
- verification artifact before `review`
- review artifact with no blockers before `complete`
- release readiness artifact with `result=pass` before `complete`

## Loop Decision Table

| Current condition | Loop action | Result |
| --- | --- | --- |
| Stage exit criteria satisfied and next gate satisfied | advance stage | enter the next lifecycle stage |
| Stage exit criteria satisfied and the only missing approval is a default implicit stage approval | recover gate | set the approval flag, advance immediately, and continue the same run |
| PRD-driven request has no requirement-splitting artifacts yet | dispatch stage | enter `requirement-splitting` before `architecture-design` or `spec` |
| PRD-driven request has a normalized module missing either raw or normalized companion snapshot | recover gate | stay before `spec`, restore the missing companion artifact, then re-check the gate |
| Gate missing but recoverable from repository state, artifacts, or evidence | recover gate | stay in the same lifecycle position, repair the prerequisite, then re-check the gate |
| Gate missing and depends on an explicit user checkpoint, unresolved human decision, or external resource | block | set `loop.state=blocked`, set `loop.pending_gate`, wait for recovery |
| Verification fails but implementation can continue under the approved plan | re-enter execute | route to `execute` immediately and continue the loop |
| Review finds blockers but implementation can continue under the approved plan | re-enter execute | route to `execute` immediately and continue the loop |
| Review passes but release readiness artifact is missing or failed | recover gate | stay before `complete`, write or repair release readiness evidence, then re-check the gate |
| Request reaches completion conditions | complete | set `stage=complete` and `loop.state=complete` |

## Goal and Workflow Constraints

- The request may reach `complete` only when the goal contract is satisfied and verified.
- Loop iterations do not themselves prove completion.
- Workflow-style parallel execution does not create additional lifecycle states or bypass gates.

## Re-entry Rules

- `resume_request`: re-enter the loop for an in-flight request that already has `state.json`
- `gate_recovery`: re-enter after a previously blocking gate was satisfied
- `requirement_changed`: route back to `spec` or `plan`
- `verification_failed`: route back to `execute`
- `review_blocked`: route back to `execute`
- `manual_reopen`: re-enter a previously completed request after an explicit reopen decision

## Rollback Rules

- Requirement change during execute: back to `spec` or `plan` with `reentry_reason=requirement_changed`
- If upstream requirement content changes enough to alter module boundaries, back to `requirement-splitting` with `reentry_reason=requirement_changed`
- Verification failure: back to `execute` with `reentry_reason=verification_failed`
- Blocking review finding: back to `execute` with `reentry_reason=review_blocked`
- `execute -> verify -> execute -> verify` repeats until verification passes or a real blocking gate is reached
