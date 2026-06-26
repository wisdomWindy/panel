# Spec Templates

## Required Artifact Paths

- `docs/requests/<request-id>/spec/spec.md`
- `docs/requests/<request-id>/spec/clarifications.md`

## `spec/spec.md`

When `design/page-design.md` exists, treat it as a required upstream input and carry its layout, hierarchy, styling, and interaction decisions into the spec instead of redefining them from scratch.
The acceptance criteria must function as the request's operational goal contract: concrete, observable, and preferably machine-checkable rather than subjective.

Required sections:

- background and goals
- in scope
- out of scope
- trigger and start conditions
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

## `spec/clarifications.md`

Required sections:

- question
- answer
- final decision
- affected spec area
