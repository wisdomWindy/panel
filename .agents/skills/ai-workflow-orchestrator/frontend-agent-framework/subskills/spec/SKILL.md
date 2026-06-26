---
name: frontend-agent-framework-spec
description: Stage subskill for specification. Turn intake artifacts into the formal frontend engineering spec and collect clarifications.
---

# Spec Subskill

## Purpose

Turn the intake request into the canonical engineering contract for frontend implementation.

## Use With

Invoke from the main `frontend-agent-framework` skill after routing a request with `stage=spec`.

## Read First

- `docs/requests/<request-id>/state.json`
- `docs/requests/<request-id>/request.md`
- `docs/requests/<request-id>/artifacts/prd-snapshot.md`
- `docs/requests/<request-id>/artifacts/code-context.md` when present
- `docs/requests/<request-id>/design/page-design.md` when the request includes the page-design stage
- `../../references/state-machine.md`
- `../../references/templates/spec.md`
- `../../references/templates/code-context.md`
- `../../references/policies/api-contracts.md`
- `../../references/policies/clean-code.md`
- `../../references/policies/code-graph.md`
- `../../references/policies/design-patterns.md`
- `../../references/policies/doc-writing.md`
- `../../references/policies/frontend-architecture.md`
- `../../references/policies/spec-constraints.md`

Load other policy references only when they materially affect the spec.

## Hard Rules

- Read intake artifacts before authoring the spec.
- Read the page design artifact before authoring the spec when the request includes the page-design stage.
- Write `spec/spec.md` and `spec/clarifications.md`.
- Keep requirement changes explicit in artifacts.
- Define behavior in a way that is implementable, reviewable, and verifiable by downstream stages.
- Make acceptance criteria concrete enough to drive planning, testing, and verification.
- Define scoped behavior at the same function-complete granularity that downstream `plan` must use, so `plan`, `execute`, `verify`, and `review` do not need to reinterpret coarse requirements into finer behavior on their own.
- When the request includes pages, modules, panels, drawers, modals, cards, tables, forms, display blocks, or workflows, explicitly describe the subfunctions inside those units instead of leaving `plan` to expand them ad hoc.
- For form-bearing scope, record the form purpose, field set, required vs optional semantics, control expectations, validation expectations, state conditions, and submission outcomes when those details are knowable from the request.
- For table-bearing scope, record the table purpose, column intent, row or batch actions, display expectations, and state expectations when those details are knowable from the request.
- For pure display scope, record the displayed fields, grouping, layout intent, formatting expectations, and visibility behavior when those details are knowable from the request.
- For interaction or workflow scope, record the trigger, state transition, side effect, success path, failure path, and loading or pending-state expectations when those details are knowable from the request.
- If material product detail is still unknown, record it as an explicit clarification or approved assumption in the spec instead of leaving `plan` to fill it in silently.
- Convert material maintainability and abstraction decisions into explicit spec constraints instead of leaving them for downstream taste-based interpretation.
- Identify where the request has real pattern-level pressures such as variation by type or state, scattered creation logic, boundary adaptation, notification fan-out, or pipeline handling.
- Define the intended responsibility boundaries, side-effect placement, duplication ownership, and naming or domain-language constraints when they materially shape the solution.
- When the request modifies existing code structure, prefer code graph to identify actual module boundaries, dependency directions, and impacted symbols before writing structural spec constraints.
- Reuse and update `artifacts/code-context.md` when structural code understanding is material instead of scattering graph or fallback notes across multiple stage artifacts.
- Treat the acceptance criteria as the formal goal contract for downstream stages.
- Preserve the existing `state.json.loop` object; stage-local state updates must not reset or reinterpret it.
- Do not write implementation code.
- Require user approval before setting `approvals.spec_approved` to `true`.
- When the environment supports structured approval UI, present spec approval through explicit options such as approve / reject / revise instead of asking the user to type approval text manually.
- Only use plain-text approval prompts when structured approval UI is unavailable.

## Sample Usage

- Use `docs/requests/prd-material-delivery-config-center/spec/spec.md` and `spec/clarifications.md` as quality baselines only.
- Reuse the sample's section density, clarification style, and acceptance granularity when useful.
- Do not copy the sample's business scope, flows, contracts, decisions, or acceptance content into a new spec.
- Every new spec must be derived from the current request's PRD snapshot and clarifications.

## Required Inputs

- `docs/requests/<request-id>/request.md`
- `docs/requests/<request-id>/artifacts/prd-snapshot.md`
- `docs/requests/<request-id>/artifacts/code-context.md` when structural analysis is needed
- `docs/requests/<request-id>/state.json`
- `docs/requests/<request-id>/design/page-design.md` when present

## Required Artifacts

- `docs/requests/<request-id>/spec/spec.md`
- `docs/requests/<request-id>/spec/clarifications.md`

## Allowed Outputs

- Scope definition
- User flow and module design
- API and data contract decisions
- Spec constraints covering module boundaries, side-effect placement, duplication ownership, naming vocabulary, complexity guardrails, and pattern decisions
- Pattern-relevant design constraints and explicit notes on whether a named pattern is needed
- Code-context findings that justify the structural spec constraints when existing-code understanding is material
- Updated code-context artifact entries when the spec stage deepens structural understanding
- Edge cases, risks, and acceptance criteria
- Clarification records
- Explicit behavior contracts that can drive tests and verification evidence
- Function-complete behavior contracts that `plan` can translate directly into equally granular tasks without inventing new product detail

## Forbidden Actions

- Writing code
- Generating implementation tasks before the spec exists
- Moving to plan without user approval
- Copying sample conclusions into a new spec without re-deriving them from the current requirement
- Leaving behavior ambiguous enough that implementation must guess intent
- Leaving behavior at a coarser granularity than the downstream plan requires for correct execution
- Leaving the goal subjective enough that verification cannot determine whether the request is done
- Leaving a major abstraction or collaboration problem implicit when the downstream plan would otherwise have to guess whether a pattern is justified
- Leaving material responsibility, side-effect, duplication, or readability constraints unspecified when they would change the implementation shape

## Exit Criteria

- `spec/spec.md` exists and covers scope, flows, modules, contracts, edge cases, acceptance criteria, and risks.
- `spec/spec.md` defines scoped behavior at a function-complete granularity that matches the expected planning granularity for forms, tables, displays, interactions, states, and workflows within the approved scope.
- The spec records the design constraints that downstream stages must preserve, including responsibility boundaries, side-effect placement, duplication ownership, complexity guardrails, and any required or explicitly rejected pattern use.
- The spec records whether design-pattern use is required, optional, or explicitly unnecessary for the main change axes.
- `spec/clarifications.md` records open questions and resolved decisions.
- The spec defines observable behavior clearly enough for plan tasks, tests, and verification evidence to map back to it.
- The spec does not force `plan` to add material product behavior that was absent from the approved spec or clarifications.
- The user approves the spec, preferably through structured approval options when available.
- `state.json` is updated with `approvals.spec_approved=true` and `stage` set to `plan` while preserving the existing `loop` object.
