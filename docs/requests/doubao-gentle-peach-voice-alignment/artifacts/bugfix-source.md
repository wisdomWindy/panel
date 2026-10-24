# Bugfix Source

## Source System

Direct user defect report.

## Source Scope

Current local workspace, AI voice reply `声色` selection.

## Defect ID

`doubao-gentle-peach-voice-alignment`

## Defect Title

`温柔桃子` playback does not match Doubao's `温柔桃子` voice.

## Observed Behavior

The application voice named `温柔桃子` sounds different from the voice with the same name in Doubao.

## Expected Behavior

The application `温柔桃子` voice is audibly the same as Doubao's `温柔桃子` voice.

## Reproduction Clues

1. Open the AI audio-route dialog.
2. Select the default `温柔桃子` voice.
3. Run test playback or trigger an AI voice reply.
4. Compare the result with Doubao's `温柔桃子` voice.

## Affected Module Or Page

AI voice reply audio route and backend TTS output adapter.

## Screenshots, Logs, Or Comments

No screenshot or runtime log supplied. Scoped code evidence shows the mismatch deterministically.

## Verbatim Defect Evidence Index

- `SRC-1 observed/expected`: “温柔桃子声色怎么与豆包的温柔桃子声音不一样，预期是与豆包的温柔桃子声色一致”

## Closed-World Fix Scope Allowlist

- Make the `温柔桃子` selection use the actual Doubao voice identity and synthesis route.
- Remove the current silent mapping from that label to Microsoft HuiHui/system-default playback.
- Add only the technical authentication, request, playback, error handling, and tests required by that route after its official contract is confirmed.
- Preserve other installed system voices and unrelated AI/ASR behavior.

## Open Questions And Missing Context

- `missing-source`: Volcengine's official public voice list currently does not expose an entry named `温柔桃子`; an exact official speaker ID for the Doubao-app voice is not confirmed.
- `missing-source`: the application has no confirmed source for a Doubao Voice `X-Api-Key`; the existing model-profile API key is provider-agnostic and cannot be assumed to authorize TTS.
- An unofficial identifier found outside the official documentation cannot be treated as a stable production contract without confirmation.

