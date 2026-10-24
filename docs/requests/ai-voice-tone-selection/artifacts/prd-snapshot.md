# Direct-Change Source Snapshot

## Verbatim Source Evidence Index

- `SRC-1` user instruction (2026-10-23): `添加ASR声色选择，默认温柔桃子`

## Source Summary

Add a user-selectable voice tone with `温柔桃子` as the default. Repository facts place audible voice selection in the AI reply TTS path; ASR itself is recognition-only.

## Key Business Goals

- `source-backed`: users can select a voice tone.
- `source-backed`: the initial/default voice tone is displayed as `温柔桃子`.
- `source-derived`: the chosen tone must affect actual AI reply playback, otherwise the selector would not satisfy the practical request.
- `source-derived`: the choice must persist, consistent with the existing persisted AI audio-route form.

## Greenfield Scaffold Sensitivity

不适用：not a project, application, package, or frontend-surface creation request.

## Explicit Behavior Constraints

- Add one voice-tone selector to the existing AI voice-reply audio configuration.
- Default an absent stored selection to `温柔桃子`.
- Apply the same selected tone to test playback and queued AI reply playback.
- Preserve existing output-device, volume, confirmation, and ASR behavior.

## Closed-World Product Scope Allowlist

1. Add the voice-tone selection authorized by `SRC-1`.
2. Name and select the default tone `温柔桃子` as authorized by `SRC-1`.
3. Add only the persistence and playback plumbing required to make that selection functional.

New providers, cloud credentials, unrelated ASR model changes, new reply rules, and visual redesign are out of scope.

## Forms, Tables, Displays, and Interactions Extracted from Source

- Container: existing `AI 音频路由` dialog.
- Field: `声色`, selectable from the available voice catalog.
- Initial value: `温柔桃子` when no saved value exists.
- Save: stored together with output device and volume.
- Test: uses the current draft selection.
- Tables: not applicable.

## Workflow and State Rules Extracted from Source

- Loading an old configuration with no voice value resolves to the default.
- Saving a valid choice updates the snapshot returned by IPC.
- Test and queued playback receive the selected voice identifier.
- Existing route-confirmation gating remains unchanged.

## Relevant Modules or Pages

- AI Assistant page and `AiVoiceReplyControl`.
- AI desktop contract/store/adapter.
- AI voice configuration repository and database migration.
- AI speech output port and Windows SAPI adapter.

## Classification Ledger

- `source-backed`: selector and `温柔桃子` default.
- `code-fact-backed`: ASR is speech recognition; audible voices are implemented by the AI TTS route.
- `confirmed-decision`: none beyond the direct instruction.
- `source-derived`: persistence and actual playback application.
- `missing-source`: no fixed list of additional branded tone names was supplied; installed system voices may be exposed without inventing branded product copy.
- `out-of-scope`: ASR model catalog, LLM profiles, general TTS page behavior, unrelated styling.

## Open Questions

None blocking. “ASR 声色” is implemented on the adjacent AI TTS output path because ASR has no synthesized voice; this is required for the requested choice to affect audible output.

