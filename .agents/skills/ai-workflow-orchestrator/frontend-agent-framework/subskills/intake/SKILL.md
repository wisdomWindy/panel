---
name: frontend-agent-framework-intake
description: Stage subskill for intake. Create the request workspace from the upstream requirement source and write only intake-stage artifacts.
---

# Intake Subskill

## Purpose

Create a new request workspace from a readable PRD document or stable PRD reference and prepare the repository-local artifacts required for downstream work.

## Use With

Invoke from the main `frontend-agent-framework` skill after routing a new requirement to the intake stage.

## Read First

- `../../references/framework-overview.md`
- `../../references/state-machine.md`
- `../../references/authoring-guide.md`
- `../../references/templates/request.md`

Read intake-specific policy references only when needed for the active request.

## Hard Rules

- Read the PRD document or resolve the stable PRD reference before creating downstream artifacts.
- Create `docs/requests/<request-id>/` as the canonical request workspace.
- Write only intake artifacts in this stage.
- Capture an initial goal statement and an initial done signal in `request.md`.
- Capture the initial trigger condition, initial context sources, and intended human handoff point in `request.md`.
- Initialize `state.json.loop` using the framework's required loop shape.
- Do not write implementation code.
- Do not write `spec/spec.md` in this stage.

## Sample Bootstrap

- Use `docs/requests/prd-material-delivery-config-center/` as the reference sample when creating a new request workspace.
- Treat the sample as a structure and quality reference, not as reusable business content.
- Copy the directory shape only when it helps bootstrap a new request.
- Rewrite `README.md`, `request.md`, `state.json`, and `artifacts/prd-snapshot.md` for the new request before moving forward.
- Do not carry over sample `spec/`, `plan/`, `execution/`, `verification/`, or `review/` content into a new request.

## Required Inputs

- Readable PRD document or stable PRD reference
- Acceptable inputs include a Feishu/Lark PRD link, a document ID, or a local PRD file path
- Enough requirement context to derive a stable `request-id`

## Required Artifacts

- `docs/requests/<request-id>/request.md`
- `docs/requests/<request-id>/artifacts/prd-snapshot.md`
- `docs/requests/<request-id>/state.json`

## Allowed Outputs

- Request source summary
- Normalized PRD snapshot
- Initial request state

## Forbidden Actions

- Planning implementation work
- Writing spec artifacts
- Writing code
- Claiming the request is implementation-ready
- Reusing the sample request's downstream stage content as active request content

## Exit Criteria

- The request workspace exists.
- The source link is captured in `request.md` and `state.json`.
- `request.md` captures an explicit goal statement plus an initial done signal that later stages can refine.
- `request.md` captures the workflow trigger, the minimum starting context, and the intended human handoff point.
- The PRD is summarized into `artifacts/prd-snapshot.md`.
- `state.json` is initialized with `stage` set to `page-design` for page-oriented requests, otherwise `spec`.
- `state.json.loop` is initialized with `iteration=1`, `state=running`, `last_completed_stage=null`, `pending_gate=null`, and `reentry_reason=new_request`.
