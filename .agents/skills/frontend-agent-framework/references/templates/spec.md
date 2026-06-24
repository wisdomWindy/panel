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
- design constraints
- change axes and pattern decision
- API and data contracts
- context and dependency sources
- edge cases
- acceptance criteria
- human review and handoff
- risks

## `spec/clarifications.md`

Required sections:

- question
- answer
- final decision
- affected spec area
