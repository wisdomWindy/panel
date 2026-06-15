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
- user flow
- page and module design
- API and data contracts
- edge cases
- acceptance criteria
- risks

## `spec/clarifications.md`

Required sections:

- question
- answer
- final decision
- affected spec area
