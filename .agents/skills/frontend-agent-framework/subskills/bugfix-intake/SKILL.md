---
name: frontend-agent-framework-bugfix-intake
description: Stage subskill for defect intake. Normalize a Feishu/Lark project defect or bug report into the standard request workspace before entering spec, plan, execution, verification, and review.
---

# Bugfix Intake Subskill

## Purpose

Create a standard request workspace from a defect input such as a Feishu/Lark project bug, defect ID, or equivalent bug report, then hand the normalized request to the standard lifecycle.

## Use With

Invoke from the main `frontend-agent-framework` skill when the incoming work item is a bug or defect rather than a PRD-driven new feature request.

## Read First

- `../../references/framework-overview.md`
- `../../references/state-machine.md`
- `../../references/authoring-guide.md`
- `../../references/bugfix-feishu-project.md`
- `../../references/templates/request.md`
- `../../references/templates/bugfix-intake.md`
- `../../references/policies/doc-writing.md`

Load repository- or tool-specific bug retrieval instructions only when they exist in the active environment.

## Hard Rules

- Resolve the bug source before creating downstream artifacts.
- If the source is a Feishu/Lark defect and `meegle` is available, treat `meegle` as the default retrieval tool and automatically obtain the defect content from the provided project key and defect/work item ID before asking the user to restate it manually.
- If `meegle` is available but not authenticated, stop at the authentication gate, complete login, and then continue automatic retrieval.
- If the defect content cannot be retrieved automatically, record that limitation and request the missing bug details instead of guessing.
- Create `docs/requests/<request-id>/` as the canonical request workspace.
- Write only bugfix intake artifacts in this stage.
- Capture the bugfix goal and initial done signal in `request.md`.
- Initialize `state.json.loop` using the framework's required loop shape.
- Do not write implementation code.
- Do not write `spec/spec.md` in this stage.

## Automatic Retrieval Rules

- Prefer source-of-truth retrieval over manual restatement when the bug input is a stable Feishu/Lark defect reference.
- Prefer `meegle` as the retrieval backend when it exists in the runtime.
- Run `meegle` commands in an environment that can read the local Meegle credentials; sandboxed environments may report `authenticated: false` even after a successful login.
- Check `meegle auth status` before defect retrieval when using `meegle`.
- If unauthenticated, run `meegle auth login` or `meegle auth login --device-code` before defect retrieval.
- If the bug input is a Feishu Project defect URL, run `meegle url decode --url <defect-url>` first and extract `simple_name` plus `work_item_id`.
- After URL decode succeeds, use the extracted `simple_name` as `--project-key` in `meegle workitem get`.
- Retrieve the defect brief first, then comments or discussion history when more context is needed.
- Normalize the fetched defect data into repository artifacts before moving to downstream stages.
- If retrieval succeeds only partially, continue gathering only the missing context instead of discarding the fetched data.
- If no compatible retrieval tool exists in the runtime, stop automatic retrieval cleanly and ask for an exported defect artifact or the missing details.

## Required Inputs

- A readable bug report or a stable bug reference
- Acceptable inputs include:
  - Feishu/Lark project key plus defect/work item ID
  - Defect detail link
  - Exported defect document
  - Local bug report file path
- Enough context to identify the affected module, observed behavior, and expected behavior

## Required Artifacts

- `docs/requests/<request-id>/request.md`
- `docs/requests/<request-id>/artifacts/bugfix-source.md`
- `docs/requests/<request-id>/artifacts/prd-snapshot.md`
- `docs/requests/<request-id>/state.json`

## Required Intake Outputs

- Source system and defect identifier
- Problem summary
- Reproduction clues or known failing scenario
- Affected module or page
- Expected behavior
- Observed behavior
- Risk notes for regression-sensitive areas

## Allowed Outputs

- Request source summary
- Normalized bug snapshot
- Initial request state
- Standardized bug context that downstream spec and plan stages can consume
- Automatic retrieval status, including whether source content came from tool fetch or manual fallback
- Authentication status for the retrieval tool when the bug source is a Feishu/Lark project defect

## Forbidden Actions

- Planning implementation work before the bug context is normalized
- Writing spec artifacts
- Writing code
- Inventing missing reproduction or expected behavior
- Claiming the bug is implementation-ready without a durable source snapshot

## Exit Criteria

- The request workspace exists.
- The bug source is captured in `request.md` and `state.json`.
- `request.md` captures an explicit bugfix goal plus an initial done signal for the repaired behavior.
- The defect context is summarized into `artifacts/bugfix-source.md`.
- A normalized downstream summary is written to `artifacts/prd-snapshot.md`.
- `state.json` is initialized with `stage` set to `spec`.
- `state.json.loop` is initialized with `iteration=1`, `state=running`, `last_completed_stage=null`, `pending_gate=null`, and `reentry_reason=new_request`.
