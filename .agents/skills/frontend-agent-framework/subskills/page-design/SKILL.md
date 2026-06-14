---
name: frontend-agent-framework-page-design
description: Stage subskill for page design. Define page layout, visual hierarchy, styling direction, and interaction structure for page-oriented requests before the formal spec stage.
---

# Page Design Subskill

## Purpose

Turn a page-oriented request into a durable page design artifact that defines layout, visual hierarchy, styling direction, and interaction structure before the formal engineering spec is written.

## Use With

Invoke from the main `frontend-agent-framework` skill after `intake` when the request is a new page, a page redesign, or a UI-heavy change that materially affects layout, styling, or interaction structure.

## Read First

- `docs/requests/<request-id>/state.json`
- `docs/requests/<request-id>/request.md`
- `docs/requests/<request-id>/artifacts/prd-snapshot.md`
- `../../references/framework-overview.md`
- `../../references/state-machine.md`
- `../../references/templates/page-design.md`
- `../../references/policies/doc-writing.md`
- `../../references/policies/frontend-components.md`
- `../../references/policies/frontend-architecture.md`

## Hard Rules

- Use this stage only for page-oriented requests.
- Write `design/page-design.md`.
- Define the page structure clearly enough that `spec` can treat it as upstream input instead of recreating layout decisions from scratch.
- Keep design decisions durable and explicit in repository artifacts.
- Do not write implementation code.
- Do not skip this stage for page requests that introduce or materially change layout, visual hierarchy, styling direction, or interaction organization.

## Required Inputs

- `docs/requests/<request-id>/request.md`
- `docs/requests/<request-id>/artifacts/prd-snapshot.md`
- `docs/requests/<request-id>/state.json`

## Required Artifacts

- `docs/requests/<request-id>/design/page-design.md`

## Allowed Outputs

- Page layout structure
- Section hierarchy
- Styling direction
- Interaction skeleton
- Responsive considerations
- Design risks and unresolved UI questions

## Forbidden Actions

- Writing code
- Turning this stage into a pixel-perfect design tool workflow requirement
- Moving to implementation planning without producing a durable design artifact
- Using this stage for non-UI tasks that do not materially affect page structure or styling

## Exit Criteria

- `design/page-design.md` exists.
- The page design artifact defines layout, hierarchy, style direction, interaction structure, and responsive considerations.
- Any unresolved UI questions are recorded.
- `state.json` is updated with `stage` set to `spec`.
