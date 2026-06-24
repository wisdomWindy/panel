# Verification Templates

## Required Artifact Paths

- `docs/requests/<request-id>/verification/verification.md`
- `docs/requests/<request-id>/verification/evidence/`

## `verification/verification.md`

Required top-level sections:

- acceptance coverage
- spec constraint compliance
- summary

Required fields for each acceptance item:

- acceptance item id
- verification method
- result
- evidence reference
- follow-up if failed
- handoff status

`spec constraint compliance` must include:

- result (`pass` or `fail`)
- checked constraints
- evidence reference
- follow-up if failed
