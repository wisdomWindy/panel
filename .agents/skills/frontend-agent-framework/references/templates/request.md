# Request Templates

## Required Artifact Paths

- `docs/requests/<request-id>/request.md`
- `docs/requests/<request-id>/state.json`
- `docs/requests/<request-id>/artifacts/prd-snapshot.md`
- `docs/requests/<request-id>/requirements/requirement-map.md` for PRD-driven requests after requirement splitting

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
  "stage": "requirement-splitting",
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

For PRD-driven requests, initialize `stage` to `requirement-splitting`.
For bugfix requests normalized through `bugfix-intake`, initialize `stage` to `spec` unless the main skill explicitly routes the defect through requirement splitting.

Loop field rules:

- keep the existing top-level `status` semantics unchanged
- `iteration` increments once per main orchestrator decision cycle
- `iteration` must continue increasing across same-run re-entries such as `verification_failed`, `review_blocked`, and `gate_recovery`
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
- explicit behavior constraints
- forms, tables, displays, and interactions extracted from source
- workflow and state rules extracted from source
- relevant modules or pages
- notable open questions from the upstream PRD

### `explicit behavior constraints`

This section must preserve explicit upstream requirement detail whenever the PRD already states it.

At minimum, extract and normalize:

- form-related constraints already present in the source
- table-related constraints already present in the source
- display-field constraints already present in the source
- interaction rules already present in the source
- loading, disabled, empty, error, success, and failure-state rules already present in the source
- workflow steps, branches, and state-transition rules already present in the source

Do not replace explicit upstream detail with vague summaries such as "contains a form" or "supports table operations" when the source already names the fields, columns, actions, validations, or state rules.

### `forms, tables, displays, and interactions extracted from source`

When applicable, organize the extracted source content by functional unit and preserve:

- page or container name
- form field names and rules
- table column names and rules
- display fields and formatting notes
- interaction triggers and outcomes

### `workflow and state rules extracted from source`

When applicable, preserve:

- start and end states
- branch conditions
- loading or pending-state rules
- success and failure outcomes
- retry, rollback, or confirmation behavior

## `requirements/requirement-map.md`

Required sections for PRD-driven requests:

- splitting summary
- source modules
- split modules and functional units
- source traceability
- module dependencies
- page-design routing hint
- spec input checklist
- open questions

Requirements:

- split the normalized requirement by module, page, feature group, or workflow unit based on the source
- preserve source wording for each module snapshot instead of paraphrasing away explicit detail
- for every split module, point back to the matching source section or excerpt anchor
- explicitly mark whether the module requires page design before spec
- identify upstream details that must be carried into `spec` without loss, including fields, columns, displays, interactions, validations, and state rules

## `requirements/modules/<module-id>.md`

Recommended sections for each split module:

- module name
- source snapshot
- source-trace references
- business goal
- user-visible behaviors
- forms, tables, displays, and interactions from source
- workflow and state rules from source
- upstream constraints that spec must preserve
- page-design need
- open questions
