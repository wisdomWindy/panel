# Request Templates

## Required Artifact Paths

- `docs/requests/<request-id>/request.md`
- `docs/requests/<request-id>/state.json`
- `docs/requests/<request-id>/artifacts/prd-snapshot.md`

## `request.md`

Required sections:

- request identifier
- source link
- business summary
- affected area
- participating modules

## `state.json`

Required fields:

- `request_id`
- `source`
- `stage`
- `status`
- `artifacts`
- `approvals`

## `artifacts/prd-snapshot.md`

Required sections:

- source summary
- key business goals
- relevant modules or pages
- notable open questions from the upstream PRD
