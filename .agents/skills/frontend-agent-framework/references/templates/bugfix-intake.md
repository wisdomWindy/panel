# Bugfix Intake Template

## Required Artifact Path

- `docs/requests/<request-id>/artifacts/bugfix-source.md`
- `docs/requests/<request-id>/artifacts/code-context.md` when structural analysis is needed

## `artifacts/bugfix-source.md`

Required sections:

- source system
- project key or source scope
- defect or work item id
- defect title
- observed behavior
- expected behavior
- reproduction clues
- affected module or page
- available screenshots, logs, or comments
- verbatim defect evidence index
- closed-world fix scope allowlist
- open questions and missing context

## Normalization Rule

After writing `bugfix-source.md`, also write `artifacts/prd-snapshot.md` as the normalized problem statement for the downstream `spec` stage.

The verbatim defect evidence index must preserve the exact observed behavior, expected behavior, reproduction wording, affected scope, and any explicit acceptance statement from the source. The normalized problem statement is not independent authority.

The closed-world fix scope allowlist must list every authorized behavior change. Root-cause code facts may authorize technical repair and dependency cleanup that preserve the expected behavior, but cannot add product behavior, fields, copy, controls, validation, permissions, navigation, API semantics, or UX changes not stated by the defect source or explicitly confirmed.
