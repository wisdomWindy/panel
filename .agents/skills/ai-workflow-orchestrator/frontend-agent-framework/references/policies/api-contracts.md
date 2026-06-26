# API Contracts Policy

## Scope

This policy defines stable standards for integrating backend contracts into frontend code.

## Standards

- Keep request success criteria explicit and testable.
- Keep error ownership consistent between the request layer and the UI layer.
- Preserve backend field semantics unless a deliberate mapping boundary is required.
- Avoid casual remapping when direct use of the backend contract is sufficient.
- Reuse authoritative backend-provided types when they exist instead of duplicating local definitions.

## Non-Goals

- This policy does not prescribe a single backend package layout for every repository.
- This policy does not define business-specific API modules.
