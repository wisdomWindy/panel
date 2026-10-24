# Clarifications: AI Voice Tone Selection

## Resolved Interpretation

- The user used `ASR` while requesting an audible `声色`. Repository semantics show ASR has no synthesis voice; the setting is therefore implemented in the AI reply TTS audio route next to ASR. This preserves the practical intent rather than adding a nonfunctional recognition field.
- `温柔桃子` is a stable built-in preset label. Additional choices are host-provided system voices with their existing names, avoiding invented branded options.

## Technical Assumptions

- The existing Microsoft HuiHui preference is the narrowest repository-backed resolution for the `温柔桃子` preset on Windows; when absent, system default speech remains a reversible fallback.
- Voice catalogs are small enough to return with the existing snapshot.

## Missing Source

- No additional branded voice names or cloud provider were specified. They remain out of scope.

## Blocking Questions

None.

