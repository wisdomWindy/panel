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
- goal statement
- initial done signal
- trigger condition
- initial context sources
- human handoff point
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
- `loop`

Required shape:

```json
{
  "request_id": "...",
  "source": {},
  "stage": "spec",
  "status": "in_progress",
  "artifacts": {},
  "approvals": {},
  "loop": {
    "iteration": 1,
    "state": "running",
    "last_completed_stage": null,
    "pending_gate": null,
    "reentry_reason": "new_request"
  }
}
```

Loop field rules:

- keep the existing top-level `status` semantics unchanged
- `iteration` increments once per main orchestrator decision cycle
- `state` only allows `running`, `blocked`, or `complete`
- `last_completed_stage` stores the most recent fully completed lifecycle stage, or `null`
- `pending_gate` stores the active blocking gate name, or `null`
- `reentry_reason` only allows `new_request`, `resume_request`, `gate_recovery`, `requirement_changed`, `verification_failed`, `review_blocked`, or `manual_reopen`
- `loop.state=complete` requires `stage=complete`
- `loop.state=blocked` is only for real external blocking gates after internal recovery was attempted

## `artifacts/prd-snapshot.md`

Required sections:

- source summary
- key business goals
- relevant modules or pages
- notable open questions from the upstream PRD
