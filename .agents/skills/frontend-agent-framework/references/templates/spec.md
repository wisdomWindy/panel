# Spec Templates

## Required Artifact Paths

- `docs/requests/<request-id>/spec/spec.md`
- `docs/requests/<request-id>/spec/clarifications.md`

## `spec/spec.md`

When `requirements/requirement-map.md` exists, treat it as a required upstream input for PRD-driven requests and preserve its module boundaries, source traceability, and carry-forward checklist in the spec.
When `design/page-design.md` exists, treat it as a required upstream input and carry its layout, hierarchy, styling, and interaction decisions into the spec instead of redefining them from scratch.
The acceptance criteria must function as the request's operational goal contract: concrete, observable, and preferably machine-checkable rather than subjective.

Required sections:

- background and goals
- in scope
- out of scope
- trigger and start conditions
- requirement split summary
- user flow
- page and module design
- function-complete behavior breakdown
- design constraints
- change axes and pattern decision
- code context and impact assumptions
- API and data contracts
- context and dependency sources
- edge cases
- acceptance criteria
- human review and handoff
- risks

## `API and data contracts`

For every material backend integration in scope, describe:

- contract source
  - backend TypeScript declaration
  - protobuf
  - OpenAPI or conventional API document
  - field table or other approved source
- endpoint purpose
- request method and path when applicable
- request parameters or body shape
- response shape
- field semantics that affect UI behavior
- required, optional, nullable, repeated, enum, or default-value semantics when material
- error, empty, loading, and success semantics that the frontend must honor
- whether the frontend uses the backend contract directly or through an adapter or mapper boundary
- if an adapter exists, what semantic normalization happens there

Typing rules that the spec must preserve:

- if the backend provides TypeScript declarations, use them as the preferred contract types
- if the backend does not provide TypeScript declarations, preserve backend field names and express field types using valid TypeScript types
- if the source contract is protobuf, state whether generated types already exist or whether TypeScript-facing types will be derived from protobuf definitions

## `function-complete behavior breakdown`

This section is required whenever the scoped work includes concrete frontend behavior. Describe the same behavior granularity that downstream `plan` must preserve.

At minimum, break down each scoped page, module, panel, drawer, modal, card, form, table, display block, or workflow into the subfunctions that together make the approved feature complete.

When applicable, include:

- forms: purpose, field list, required/optional semantics, control expectations, validation expectations, editable/disabled/hidden states, submission behavior, and success/failure/loading outcomes
- tables: purpose, column intent, row actions, batch actions, state handling, and display expectations
- display content: field groups, layout intent, formatting expectations, and visibility rules
- interactions: trigger, preconditions, UI change, side effect, confirmation, pending/loading behavior, and completion conditions
- workflows: start, steps, branch conditions, state transitions, retry/rollback path, and handoff points

If any material detail is unknown, record it in `spec/clarifications.md` and reference the approved assumption or open question directly from the spec.
If the source module files already contain explicit fields, columns, interactions, states, or workflow rules, the spec must carry them forward instead of collapsing them into higher-level prose.

## `spec/clarifications.md`

Required sections:

- question
- answer
- final decision
- affected spec area
