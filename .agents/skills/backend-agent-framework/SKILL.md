---
name: backend-agent-framework
description: Use when working in this repository's local `.agents` workflow for backend development. This is the default entry skill that orchestrates the full request lifecycle, enforces lifecycle gates, and routes work to the matching backend stage subskill.
---

# Backend Agent Framework

Use this as the default entry skill for repository-local backend workflow.

This main skill is the lifecycle orchestrator. It owns preflight checks, stage planning, state-machine enforcement, and stage dispatch. It does not perform stage-local work itself, but it is responsible for driving the request through every required stage in order.

## Goal

Keep the repository-local backend workflow in control through one default entry skill that decomposes work into lifecycle stages, advances the request through each stage in order, and runs an explicit loop orchestrator that uses focused stage subskills for stage-local work.

## What This Skill Owns

- Establish and preserve the request goal contract so "done" is explicit and checkable.
- Establish the proactive workflow contract: trigger, context, steerability, and handoff.
- Enforce full-scope delivery by default: requirement splitting is for decomposition and ordering, not for handing module priority selection back to the user.
- Enforce autonomous progression by default: once a request enters the lifecycle, continue through every non-blocked stage in the same active run instead of stopping after a stage summary.
- Enforce the canonical lifecycle:
  `intake -> requirement-splitting -> spec -> plan -> execute -> verify -> review -> complete`
- Enforce the optional structure-design lifecycle for backend-structural requests:
  `intake -> requirement-splitting -> architecture-design -> spec -> plan -> execute -> verify -> review -> complete`
- Build or recover the active request lifecycle execution path.
- Run the loop orchestrator:
  `observe state -> decide next action -> dispatch one stage or gate recovery action -> observe result -> iterate`
- Decide whether workflow-style parallel execution is justified without conflating it with lifecycle control.
- Enforce specification-driven development so implementation follows approved repository artifacts instead of ad hoc coding decisions.
- Enforce TDD for testable behavior changes unless the current task is explicitly non-testable and that exception is recorded.
- Enforce clean-code, design-pattern, code-context, and backend contract discipline for backend delivery work.
- Enforce backend reliability concerns such as contract fidelity, idempotency, retry boundaries, observability, migration safety, and rollback readiness.
- Enforce backend safety concerns such as authentication, authorization, secret handling, sensitive-data exposure, and abuse-surface checks.
- Enforce Chinese repository artifacts by default: all request `.md` outputs should use Chinese headings and prose unless a code, contract, field, protocol, path, or other source identifier must remain in its original form.

## Built-In Constitution

This skill carries its own workflow constitution and does not require `.agents/AGENTS.md` to exist.

- Default entrypoint: use this skill as the single workflow entrypoint.
- New requirement: decompose the request into the full lifecycle and start from `intake`.
- Backend-structural requirement: after `requirement-splitting`, insert `architecture-design` before `spec` when interface surfaces, service boundaries, workflow orchestration, persistence models, async topology, or cross-service integration shape is a material part of the request.
- PRD-driven requirement: after `intake`, always run `requirement-splitting` before `architecture-design` or `spec`.
- PRD-driven requirement: unless the source itself defines phases, milestones, or optional scope branches, treat all in-scope modules as part of one continuous delivery flow.
- Default approval mode: `spec` and `plan` approvals are implicit inside the same active workflow run unless the user explicitly asks for a checkpoint or a real blocking decision remains unresolved.
- Defect input: normalize the bug through `bugfix-intake`, then route it to `architecture-design` or `spec` based on whether the fix changes backend structure.
- Existing request: resume from `docs/requests/<request-id>/state.json`, recover the current stage, and continue orchestrating from there.
- Existing request: preserve or repair `state.json.loop`, resume the active loop, and continue orchestrating from there.
- Never start coding before a request workspace exists under `docs/requests/<request-id>/`.
- Never enter `architecture-design`, `spec`, or `plan` for a PRD-driven request before durable requirement-splitting artifacts exist.
- Never implement before approved `spec/spec.md` and `plan/plan.md` exist.
- Never declare work complete before `verification/verification.md` and `review/review.md` both exist and pass.
- Never declare work complete before `release/release-readiness.md` exists and records rollout, migration, monitoring, and rollback readiness.
- Never declare work complete before `verification/verification.md` explicitly records `规格约束符合性: 通过`.
- Never declare work complete before `review/review.md` explicitly records `整洁代码评估: 通过` and `设计模式评估: 通过`.
- Never declare work complete before verification and review explicitly record security checks for auth/authz, secret handling, sensitive data, and abuse surfaces as `通过` or `不适用`.
- Never treat loop progress as proof of completion; only the goal contract and verification evidence can prove the request is done.
- Never stop merely because one task, one module, or one stage summary was completed when the next lifecycle action is already known and unblocked.
- Never turn module decomposition into a new prioritization question unless the source explicitly offers mutually exclusive rollout choices or a real blocking ambiguity prevents ordering.
- When requirements change, update request artifacts first and roll the request back to `spec` or `plan`.
- The main skill owns lifecycle decomposition, progression control, gate checks, and stage routing.
- Stage subskills own stage-local execution rules only.
- Policy and template material lives under `references/` and is loaded on demand.
- Loop state is controlled by the main skill only; stage subskills must not reset or reinterpret it.
- Goal, loop, and workflow are separate dimensions: goal decides when work is done, loop decides how the orchestrator keeps re-evaluating, and workflow decides whether parallel scale is warranted.

If a repository also provides `.agents/AGENTS.md`, treat it as an optional local override only when it does not weaken these built-in rules.

## Preconditions

- Read `references/framework-overview.md` and `references/state-machine.md` before dispatching.
- If `.agents/AGENTS.md` exists, read it as a repository-local override layer after reading this skill.
- For existing work, read `docs/requests/<request-id>/state.json` before choosing a stage.
- For existing work, require `state.json.loop` to exist; if it is missing or malformed, repair it before continuing.
- For existing work, recover the current goal contract from `request.md`, `spec/spec.md`, and verification expectations before resuming execution.
- For new work, require a readable PRD document or a stable PRD reference before intake begins.
- Acceptable new-request inputs include a Feishu/Lark PRD link, a document ID, or a local PRD file path.
- For bugfix work, accept a readable bug report or a stable bug reference before bugfix intake begins.
- Acceptable bugfix inputs include a Feishu/Lark project key plus defect/work item ID, a defect detail link, an exported defect document, or a local bug report file path.
- When a stable Feishu/Lark defect reference is provided and compatible tools exist, prefer automatic defect retrieval before asking the user for manual restatement.
- Do not create downstream artifacts before intake is complete.

## Dispatch Model

Follow this order:

1. Classify whether the request is a new requirement or an in-flight request.
2. If the input is a defect or bug report, first route it through `bugfix-intake` to create the standard request workspace.
3. If the request is a PRD-driven requirement, route it through `requirement-splitting` after `intake`.
4. If the request is backend-structural, insert `architecture-design` between `requirement-splitting` and `spec`.
5. Build or recover the lifecycle execution path for either:
   - `intake -> requirement-splitting -> spec -> plan -> execute -> verify -> review -> complete`
   - `intake -> requirement-splitting -> architecture-design -> spec -> plan -> execute -> verify -> review -> complete`
6. Validate lifecycle gates against this skill's built-in constitution and `references/state-machine.md`.
7. Load only the subskill for the current stage.
8. After the current stage finishes, determine whether the next stage can start immediately or must wait on a real gate.
9. Continue stage-by-stage in the same active run until the request reaches `complete` or a required gate truly blocks progress.

When requirement splitting yields multiple modules or functional units, keep them inside the same request scope by default and continue automatically in dependency-aware source order. Do not stop just to ask which module should be done first unless the source explicitly leaves multiple valid priority choices unresolved.

This workflow must be executed strictly in the defined lifecycle order.
Do not skip, merge, reorder, or bypass stages unless the rollback rules explicitly allow it.

## Goal Contract

Every request must have an explicit goal contract. The goal contract defines what "done" means for this request and must be concrete enough that downstream verification can confirm it without relying on taste or intuition alone.

Goal rules:

- Capture the initial goal in `request.md` during intake.
- Refine the goal into observable acceptance criteria in `spec/spec.md`.
- Translate that goal into execution and verification work in `plan/plan.md` and `verification/verification.md`.
- If the request only describes a subjective improvement such as "better", "cleaner", or "more elegant", obtain or derive a more checkable completion condition before heavy execution begins.
- The loop exists to drive work toward the goal contract; the loop itself is not the goal.

## Proactive Workflow Contract

When the request describes a repeatable or operational workflow rather than a one-off edit, define the workflow contract explicitly before heavy execution.

Workflow contract rules:

- Define what event or condition starts the workflow.
- Define which repositories, documents, tools, and data sources provide the minimum viable context.
- Define how a human can observe, redirect, pause, or stop the workflow if it starts drifting.
- Define where the workflow hands results back to people or downstream systems.
- Prefer narrow, concrete triggers over vague requests such as "look around and see what matters".
- Prefer auditable evidence and visible artifacts over hidden automation.

## Loop Orchestration Model

The main skill runs a single explicit control loop for the request. The lifecycle stages stay the same; the loop is the control mechanism that advances or re-enters those stages.

1. Observe: read `state.json`, inspect artifacts, approvals, and lifecycle gates.
2. Decide: choose exactly one next action from `dispatch_stage`, `recover_gate`, `request_approval`, `wait_external`, or `complete`.
3. Dispatch: load only the one subskill that matches the current lifecycle stage when the action is `dispatch_stage`.
4. Observe result: verify the stage exit criteria or the recovered gate result, then re-check the lifecycle gates.
5. Iterate: update `state.json.loop` and begin the next decision cycle.

Loop rules:

- Process only one current action per loop iteration.
- After each completed stage or recovered gate, re-enter the loop and decide again.
- After each completed stage, prefer immediate dispatch of the next unblocked stage over returning control to the user.
- Treat `requirement-splitting` as a full lifecycle stage with its own exit criteria and loop re-entry, not as an intake footnote.
- After `execute`, `verify`, or `review`, continue looping immediately unless the request is truly blocked or has reached `complete`.
- Unless the request is truly `blocked` or has reached `complete`, do not stop the workflow; continue the next loop action in the same active workflow run.
- The loop stops only when `stage=complete` with `loop.state=complete`, or when a real external blocking gate requires waiting.
- `request_approval` is an exception path, not the default path: use it only when the user explicitly asked for a checkpoint or when a real unresolved decision makes autonomous continuation unsafe.
- If a gate can be satisfied by gathering repository context, repairing missing artifacts, or repairing missing state, do that recovery work inside the loop before stopping.
- Rollback is a valid loop re-entry path, not an exception outside the workflow.

## Workflow Scale Model

Workflow-style parallelization is optional horizontal scale. It is not the lifecycle, not the loop, and not the completion condition.

Workflow rules:

- Default to one active main-skill path through the lifecycle.
- Treat split modules as one ordered delivery backlog inside the same request by default, not as separate user-selected work items.
- Use workflow-style parallel execution only when the approved plan identifies multiple independent work units whose cost and context split justify parallelism.
- Do not use workflow-style parallel execution for trivial edits, tightly coupled changes, or single-thread tasks that fit comfortably in one context.
- If parallel execution is used, it must still report back into the same request artifacts, gates, and loop state.

## Orchestration Rules

- Decompose every request into the full lifecycle stage sequence, even when only one immediate stage is active.
- Normalize bugfix inputs into the same request workspace structure used by PRD-driven requests.
- For PRD-driven requirements, force a durable requirement-splitting pass before any design, specification, or planning work begins.
- Treat requirement splitting as source-preserving normalization: it must split modules and functional units without inventing behavior, implementation, or backend topology that is not grounded in the source.
- After requirement splitting, derive a deterministic downstream ordering from source order, dependency order, and rollout constraints; do not ask the user to prioritize split modules unless the PRD itself requires that product decision.
- For backend-structural requests, treat `architecture-design` as a required upstream input to `spec`, not as an implementation detail.
- For backend-structural bugfix requests, do not bypass `architecture-design` merely because the entry source is a defect rather than a PRD.
- Treat the goal contract as the source of truth for "done"; do not substitute chat satisfaction or elapsed loop count.
- Execute the current stage completely through its designated subskill before attempting the next stage.
- After each stage, re-read the request state and evaluate whether the next gate is satisfied.
- If the current stage exits cleanly and no real blocker exists, dispatch the next lifecycle stage immediately in the same run instead of pausing for manual "continue".
- If `verify` fails and the failure can be addressed by implementation work defined by the approved spec and plan, route immediately back to `execute` and continue the loop without waiting for a new user prompt.
- If `review` finds blocking issues that can be addressed by implementation work defined by the approved spec and plan, route immediately back to `execute` and continue the loop without waiting for a new user prompt.
- If the next gate depends on an explicitly requested user checkpoint, a real unresolved product or risk decision, or an unavailable external resource, stop at that gate and report exactly what is required to continue.
- Do not ask the user "which part first" or equivalent prioritization questions merely because the request was decomposed into multiple modules, services, endpoints, jobs, or schemas.
- If the next gate can be satisfied by gathering repository-local context, artifacts, or evidence, obtain that prerequisite first and then continue the lifecycle.
- Do not let execution, verification, or review invent behavior that is not justified by the approved spec and plan.
- Apply `references/policies/clean-code.md`, `references/policies/design-patterns.md`, `references/policies/spec-constraints.md`, `references/policies/code-graph.md`, `references/policies/api-contracts.md`, `references/policies/backend-architecture.md`, `references/policies/backend-components.md`, `references/policies/testing.md`, `references/policies/mysql-enterprise.md`, `references/policies/redis-enterprise.md`, `references/policies/kafka-enterprise.md`, `references/policies/grpc-enterprise.md`, and `references/policies/k8s-enterprise.md` as durable downstream rules.
- Treat release readiness and security verification as delivery requirements, not optional polish.

## Loop State Contract

`docs/requests/<request-id>/state.json` must include a top-level `loop` object with this shape:

```json
{
  "iteration": 1,
  "state": "running",
  "last_completed_stage": null,
  "pending_gate": null,
  "reentry_reason": "new_request"
}
```

Field rules:

- `iteration`: increment once each time the main skill re-enters the top-level decision loop.
- `state`: only `running`, `blocked`, or `complete`.
- `last_completed_stage`: the most recent stage whose exit criteria were fully satisfied.
- `pending_gate`: the currently blocking gate name, or `null` when no blocking gate is active.
- `reentry_reason`: only `new_request`, `resume_request`, `gate_recovery`, `requirement_changed`, `verification_failed`, `review_blocked`, or `manual_reopen`.

## Sequential Stage Execution

Execute every stage in full, in order, one at a time.
At each stage boundary, update `docs/requests/<request-id>/state.json` before entering the next stage, then return to the loop controller.

1. When a stage starts, complete all required work for that stage through its subskill.
2. Before advancing, verify that the current stage exit criteria are fully satisfied.
3. Update `state.json` to reflect the stage transition while preserving the current `loop` object.
4. Re-read `state.json`, update loop metadata in the main skill, increment `loop.iteration`, and continue only if the next gate is satisfied.
5. Never leave a stage partially done and then proceed to a later stage.
6. If `verify` or `review` sends the request back to `execute`, treat that as an immediate continuation of the same workflow.

## Stage Routing

- New requirement with a readable PRD document or stable PRD reference: load `subskills/intake/SKILL.md`
- Bug or defect input with a readable bug report or stable bug reference: load `subskills/bugfix-intake/SKILL.md`
- Existing PRD-driven request with `stage=requirement-splitting`: load `subskills/requirement-splitting/SKILL.md`
- Existing request with `stage=architecture-design`: load `subskills/architecture-design/SKILL.md`
- Existing request with `stage=spec`: load `subskills/spec/SKILL.md`
- Existing request with `stage=plan`: load `subskills/plan/SKILL.md`
- Existing request with `stage=execute`: load `subskills/execute/SKILL.md`
- Existing request with `stage=verify`: load `subskills/verify/SKILL.md`
- Existing request with `stage=review`: load `subskills/review/SKILL.md`

If a request is already at `stage=complete`, do not resume implementation flow unless the requirement changed and the request is explicitly rolled back.

## Reference Loading

Load only what is needed:

- Workflow overview: `references/framework-overview.md`
- State machine and rollback rules: `references/state-machine.md`
- Framework authoring constraints: `references/authoring-guide.md`
- Feishu/Lark bug retrieval rules: `references/bugfix-feishu-project.md`
- Durable standards: `references/policies/*.md`
- Pattern catalog: `references/patterns/*.md`
- Artifact shape requirements: `references/templates/*.md`

## Hard Gates

- Never code before the request workspace exists.
- Never implement before approved spec and plan exist.
- Stage approval may be implicit inside the same active run when the stage artifact meets exit criteria and no explicit manual checkpoint is active.
- Never let backend-structural work enter `spec` without an `architecture-design` artifact when backend shape is a material requirement.
- Never declare work complete before verification and review artifacts both exist and pass.
- Never declare work complete before `release/release-readiness.md` exists and passes.
- Never treat missing compliance verdicts as implied pass results.
- When requirements change, update request artifacts first and roll the stage back to `spec` or `plan`.
- Do not bypass the local workflow by jumping straight to code.
- Mark the request `blocked` only when the gate depends on an explicitly requested user checkpoint, a real unresolved decision, or an unavailable external system, credential, or resource after internal recovery was attempted.
- Never implement behavior that is not defined, clarified, or accepted in the repository spec artifacts.
- Never skip test-first execution for testable behavior changes unless the reason is explicitly recorded in repository artifacts.
- Never stop after a failed `verify` or blocking `review` if the next required action is more implementation work and no real blocking gate exists.

## Required State Discipline

- Read `docs/requests/<request-id>/state.json` before acting on any existing request.
- Treat `state.json` as the source of truth for `stage`, approvals, artifact status, and loop metadata.
- If `state.json` and filesystem artifacts disagree, repair the state or missing artifacts before continuing.
- If `loop` is missing on a legacy request, backfill it before continuing the workflow.

## Sample Request Baseline

Use `docs/requests/prd-material-delivery-config-center/` only as a structure and quality reference.

- Reuse directory shape and artifact density when helpful.
- Do not copy its business scope, flows, contracts, decisions, tasks, or conclusions into a new request.

## Responsibilities

The main skill owns:

- request classification
- lifecycle decomposition
- goal preservation
- loop control
- workflow scale decisions
- stage-to-stage progression control
- gate checking
- stage dispatch

Stage actions belong only to subskills.

## Escalation Rules

- If a stage uncovers missing product decisions, record them in repository artifacts instead of silently filling the gap.
- If requirement change during execution alters service boundaries, contract shape, workflow shape, or persistence semantics, stop coding, set loop re-entry reason to `requirement_changed`, and route back to `spec` or `plan`.
- If requirement change during execution alters rollout sequencing, migration safety, rollback limits, or security posture, stop coding and route back to `spec` or `plan`.
- If verification fails, set loop re-entry reason to `verification_failed` and route back to `execute`.
- If review finds blockers, set loop re-entry reason to `review_blocked` and route back to `execute`.
- If execution discovers the spec is incomplete, ambiguous, or contradicted by reality, stop and route back to `spec` or `plan` before continuing implementation.
- If TDD cannot be applied to a behavior change, record the reason and the fallback verification method in repository artifacts before continuing.
- If the need for a pattern is discovered during execution, record and approve that design decision through spec or plan artifacts before cementing the abstraction in code.
- If an existing-code request depends on structural understanding and no usable code graph is present, attempt automatic graph installation or bootstrap first, then record detection status, installation or bootstrap result, and fallback status in `artifacts/code-context.md`.

## Completion Rule

Treat a request as complete only after all of the following are true:

- the goal contract is satisfied by observable results
- implementation matches the approved plan
- verification evidence exists for acceptance criteria
- release readiness evidence exists for rollout, migration, monitoring, and rollback
- `verification/verification.md` explicitly records `规格约束符合性: 通过`
- `verification/verification.md` explicitly records `安全验证: 通过` or `不适用`
- review records no unresolved blocking issues
- `review/review.md` explicitly records `整洁代码评估: 通过`
- `review/review.md` explicitly records `设计模式评估: 通过`
- `review/review.md` explicitly records `安全评估: 通过` or `不适用`
- `release/release-readiness.md` explicitly records `结果: 通过`
- `state.json` is updated to `stage="complete"`
- `state.json.loop.state` is updated to `complete`

## Non-Goals

- Do not duplicate stage instructions in this file.
- Do not expose policy or template references as top-level skills.
- Do not invent another request lifecycle beside the repository-owned one.
- Do not perform stage-local authoring, implementation, verification, or review inside the main skill body.
