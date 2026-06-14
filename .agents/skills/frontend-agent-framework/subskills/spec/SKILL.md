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
- `docs/requests/<request-id>/design/page-design.md` when the request includes the page-design stage
- `../../references/state-machine.md`
- `../../references/templates/spec.md`
- `../../references/policies/api-contracts.md`
- `../../references/policies/doc-writing.md`
- `../../references/policies/frontend-architecture.md`

Load other policy references only when they materially affect the spec.

## Hard Rules

- Read intake artifacts before authoring the spec.
- Read the page design artifact before authoring the spec when the request includes the page-design stage.
- Write `spec/spec.md` and `spec/clarifications.md`.
- Keep requirement changes explicit in artifacts.
- Define behavior in a way that is implementable, reviewable, and verifiable by downstream stages.
- Make acceptance criteria concrete enough to drive planning, testing, and verification.
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
- `docs/requests/<request-id>/state.json`
- `docs/requests/<request-id>/design/page-design.md` when present

## Required Artifacts

- `docs/requests/<request-id>/spec/spec.md`
- `docs/requests/<request-id>/spec/clarifications.md`

## Allowed Outputs

- Scope definition
- User flow and module design
- API and data contract decisions
- Edge cases, risks, and acceptance criteria
- Clarification records
- Explicit behavior contracts that can drive tests and verification evidence

## Forbidden Actions

- Writing code
- Generating implementation tasks before the spec exists
- Moving to plan without user approval
- Copying sample conclusions into a new spec without re-deriving them from the current requirement
- Leaving behavior ambiguous enough that implementation must guess intent

## Exit Criteria

- `spec/spec.md` exists and covers scope, flows, modules, contracts, edge cases, acceptance criteria, and risks.
- `spec/clarifications.md` records open questions and resolved decisions.
- The spec defines observable behavior clearly enough for plan tasks, tests, and verification evidence to map back to it.
- The user approves the spec, preferably through structured approval options when available.
- `state.json` is updated with `approvals.spec_approved=true` and `stage` set to `plan`.
