# Request: AI Voice Reply Warning Spacing

- Source: direct user instruction with annotated screenshot, 2026-10-23.
- Goal: increase the vertical space between the top control row and the warning bar inside the highlighted AI voice-reply block.
- Done signal: both warning variants use the next existing Tailwind spacing step and no surrounding layout or behavior changes.
- Trigger: render either missing-default-model or missing-output-device warning.
- Context: `AiVoiceReplyControl.vue` lines containing the two warning alerts.
- Human handoff: visual confirmation in the running page; no unresolved product decision.
- Workflow: direct change, request-level delivery unit; PRD analysis and splitting are not applicable.

## Closed Scope

- Change only the vertical margin above both warning alerts.
- Preserve copy, conditions, controls, container padding, and queue spacing.
