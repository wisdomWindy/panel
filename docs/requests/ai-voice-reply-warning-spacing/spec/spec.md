# Specification: AI Voice Reply Warning Spacing

## Goal And Scope

Increase the vertical spacing between the AI voice-reply control row and whichever prerequisite warning is rendered. Preserve every other layout and behavior.

## Behavior Contract

- Both warning alerts use `mt-4` instead of `mt-3`.
- Conditions, copy, alert styling, section padding, following queue spacing, and all interactions remain unchanged.
- Acceptance: source contains exactly two relevant `mt-4` alert classes and no `mt-3` remains on those warning branches.

## Source And Display Semantics

- `SRC-1` authorizes the spacing increase shown by the annotated screenshot.
- The gap separates prerequisite feedback from the configuration controls; it does not create a new section or status.
- Product-content delta: none.

## Implementation Constraints

- Tailwind utility only; no CSS, helper, constant, new component, state, type, or dependency.
- Pattern fit: Level 0 direct code. No selection, variation, lifecycle, or reuse signal justifies an abstraction.
- Vue component extraction: keep inline; the change belongs to the existing cohesive component and has one owner.
- TDD: add a source-contract assertion before changing the utility.

## Acceptance Criteria

1. Both prerequisite alerts have a larger and identical top gap.
2. No behavior or red-frame-external spacing changes.
3. Focused test, ESLint, type-check, and member web build pass.
