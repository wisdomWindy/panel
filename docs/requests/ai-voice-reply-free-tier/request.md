# Request: AI Voice Reply Free Tier

## Request Identifier

`ai-voice-reply-free-tier`

## Source

- User instruction (2026-10-23): `需求文档已修改，根据修改后的功能模块进行开发`
- User clarification (2026-10-23): remove the manual prompt/generate block and open a model-selection download dialog when ASR is started without local model resources.
- User clarification (2026-10-23): remove the proxy URL field from the AI model profile editor because it is not needed.
- User clarification (2026-10-23): replace model temperature with a reasoning-effort selector using the values commonly exposed by current AI agent runtimes.
- Working-tree PRD: `docs/requirements/Bilibili 直播助手 - 产品需求文档 (PRD).md`
- Primary anchors: sections 0, 1.1, 2.3, 4, 5, 8, 9.14, 9.16, 9.17, 10, and 11.

## Business Summary

The revised PRD moves AI voice reply from a membership capability into the free product foundation. Both free and member installers must expose AI model configuration, AI reply generation, and dedicated TTS routing. Membership expiry may only suspend automatic moderation; it must not hide, disable, or gate AI voice reply.

## Goal

Migrate the existing AI feature chain out of the member-only build channel and implement the revised AI voice reply experience and contracts so the free and member builds share the same safe, local-only AI capability.

## Initial Done Signal

- Free and member builds both contain the AI Assistant route, navigation entry, frontend runtime, and Rust IPC commands.
- AI availability is independent of membership state and expiry.
- A user can manage an external OpenAI-compatible model profile, select a default profile, and test it without bundled LLM weights.
- AI voice reply remains off until the user configures a model and explicitly confirms an audio route.
- The AI reply page exposes the single master switch, model/persona controls, reply queue controls, output-device selection, and test playback required by PRD 9.14.
- Generated text is normalized to the PRD chat-style limits before display and TTS enqueueing, and no outgoing Bilibili danmaku command or action is introduced.
- Scoped frontend and Rust tests, type checks, and build-contract checks pass.
- The AI page no longer exposes manual prompt/generate/regenerate controls; generated reply results remain available for automatic voice-reply feedback.
- Enabling ASR without a ready local model opens a selectable model-download dialog instead of only reporting `模型资源暂不可用`.

## Trigger Condition

The working-tree PRD changed AI voice reply from a member feature to a free feature and materially expanded the model-profile, reply-rule, queue, TTS-routing, failure-state, and packaging requirements.

## Initial Context Sources

- Revised PRD and its working-tree diff.
- Existing AI page/store/contracts under `src/renderer/views/ai`, `src/renderer/stores/ai.ts`, and `src/renderer/platform/desktop/contracts/member.ts`.
- Existing member build channel under `src/renderer/platform/desktop/channel.member.ts`.
- Existing Rust AI implementation under `src-tauri/src/application/member`, `domain/member`, `infrastructure/member`, and `ipc/member.rs`.
- Free/member command registration in `src-tauri/src/lib.rs` and build-channel contract tests.

## Human Handoff Point

Deliver the implemented feature chain with verification evidence. External provider credentials, a real model endpoint, a virtual audio device, and OBS/Bilibili Live Studio capture remain deployment-time inputs and are not required to complete repository implementation.

## Affected Area

- Frontend routing and navigation build-channel ownership.
- AI page, store, desktop contracts, and tests.
- Rust AI domain/application/infrastructure ownership and IPC registration.
- TTS output-device integration and AI reply queue controls.
- Free/member packaging and capability allowlists.
- Local configuration persistence and secret-handling boundaries.

## Participating Modules

- Shared AI voice reply capability.
- Shared model profile configuration.
- Shared TTS audio routing and queue controls.
- Member-only moderation/membership boundary cleanup.

## Workflow Classification

- Type: PRD-driven incremental requirement.
- Speed profile: `S3 broad` because the change crosses build channels, Vue routing/state, Rust services, persistence, IPC security, and packaging contracts.
- Greenfield scaffold sensitivity: not applicable; this modifies an existing application and does not create a new frontend surface or package.

