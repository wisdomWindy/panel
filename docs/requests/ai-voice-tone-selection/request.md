# Request: AI Voice Tone Selection

## Request Identifier

`ai-voice-tone-selection`

## Source

- Direct user instruction (2026-10-23): `添加ASR声色选择，默认温柔桃子`

## Business Summary

Add a selectable voice tone to the AI voice-reply playback path. The requested default is `温柔桃子`. In the existing product model, ASR converts speech to text while TTS owns audible voice output, so the setting belongs to the AI TTS route shown beside the ASR controls.

## Goal

Let the user choose the AI reply playback voice tone, persist the choice, use it for test and queued playback, and default previously unconfigured installations to `温柔桃子`.

## Initial Done Signal

- The AI audio-route dialog exposes a voice-tone selector.
- `温柔桃子` is selected when no prior voice-tone setting exists.
- Saving and reloading preserves the selection.
- Test playback and queued AI replies use the selected voice tone.
- Scoped frontend and Rust contract tests pass.

## Trigger Condition

The user directly requested voice-tone selection with a named default.

## Initial Context Sources

- `packages/bilibili-live-tool/src/renderer/views/ai/components/AiVoiceReplyControl.vue`
- `packages/bilibili-live-tool/src/renderer/stores/ai.ts`
- `packages/bilibili-live-tool/src/renderer/platform/desktop/contracts/ai.ts`
- `packages/bilibili-live-tool/src-tauri/src/application/member`
- `packages/bilibili-live-tool/src-tauri/src/infrastructure/member`
- `packages/bilibili-live-tool/src-tauri/src/ipc/member.rs`

## Human Handoff Point

Deliver the implemented selector and end-to-end verification. No external credential or service setup is required.

## Affected Area

- AI voice-reply audio-route UI and frontend contract.
- Rust AI voice configuration, persistence, IPC DTOs, and playback adapter.
- Focused regression tests and migration coverage.

## Participating Modules

- Shared AI voice reply capability.
- Windows/system TTS voice selection.

## Workflow Classification

- Type: stable direct-change request.
- Speed profile: `S2 scoped` because the selection crosses UI, IPC, persistence, and playback.
- PRD-only requirement analysis/splitting: not required; the direct instruction is the authoritative source and the change remains one request-level delivery unit.
- Greenfield scaffold sensitivity: not applicable.

