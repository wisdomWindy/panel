# State Machine

## Stages

- intake
- page-design
- spec
- plan
- execute
- verify
- review
- complete

## Required Gates

- page design artifact before `spec` for page-oriented requests
- `spec_approved` before `plan`
- `plan_approved` before `execute`
- verification artifact before `review`
- review artifact with no blockers before `complete`

## Rollback Rules

- Requirement change during execute: back to `spec` or `plan`
- Verification failure: back to `execute`
- Blocking review finding: back to `execute`
