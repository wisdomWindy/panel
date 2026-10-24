# Page Design: AI Voice Reply Warning Spacing

## Objective And Journey

When the AI voice-reply prerequisite warning is visible, separate it more clearly from the control row above without changing the compact operational layout.

## Layout And Hierarchy

- Keep the control row and warning in the same existing section.
- Increase only the warning's top margin from `mt-3` to `mt-4` for both warning states.
- Preserve the section's outer padding and the following queue section's `mt-5` separation.

## Interaction And States

- Missing default model and missing output device remain mutually exclusive with unchanged conditions and copy.
- Loading, disabled, keyboard, focus, responsive, and accessibility behavior are unaffected.

## Styling Contract

- Use the existing inline Tailwind utility class; no CSS block, semantic class, constant, helper, or component extraction.
- Product-content delta: none; `SRC-1` authorizes the spacing increase.

## Risks And Validation

- Risk: updating only one alert would create state-dependent layout drift. Mitigation: update and assert both branches.
- Validate source contract, ESLint, type-check, and member web build.
