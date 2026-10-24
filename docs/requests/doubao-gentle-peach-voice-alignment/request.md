# Request: Doubao Gentle Peach Voice Alignment

## Request Identifier

`doubao-gentle-peach-voice-alignment`

## Source

Direct user defect report, chat on 2026-10-24: “温柔桃子声色怎么与豆包的温柔桃子声音不一样，预期是与豆包的温柔桃子声色一致”。

## Business Summary

The AI voice-reply selector currently presents a voice named `温柔桃子`, but playback uses a Windows system voice rather than Doubao's voice service. The label therefore promises an identity that the audible result does not provide.

## Goal

Make the `温柔桃子` selection audibly identical to the Doubao `温柔桃子` voice, or block that label from being represented as an available matching voice until an authorized Doubao synthesis path and exact speaker identifier are available.

## Initial Done Signal

- Playback selected as `温柔桃子` no longer resolves to Microsoft HuiHui or an arbitrary system default.
- The implementation uses an authorized Doubao speech route with the exact supported speaker identifier, with focused tests proving request routing and no silent local fallback.
- If the required official credential or exact public speaker contract is unavailable, the request remains blocked rather than shipping a false equivalence.

## Trigger

Selecting `温柔桃子` and testing or playing an AI voice reply.

## Context Sources

- User defect report.
- Existing `ai-voice-tone-selection` request artifacts.
- AI voice reply Vue/store/IPC/application/speech-adapter chain.
- Volcengine Doubao Voice official TTS API and voice-list documentation.

## Human Handoff

If official sources do not expose the Doubao-app `温柔桃子` speaker or the application has no authorized Doubao Voice API credential source, report the exact missing input before implementation.

## Affected Area

AI voice reply audio-route dialog and `SystemAiSpeechOutput` playback path.

## Participating Modules

- `packages/bilibili-live-tool/src/renderer/views/ai/components/AiVoiceReplyControl.vue`
- `packages/bilibili-live-tool/src-tauri/src/application/member/*`
- `packages/bilibili-live-tool/src-tauri/src/infrastructure/member/ai.rs`

