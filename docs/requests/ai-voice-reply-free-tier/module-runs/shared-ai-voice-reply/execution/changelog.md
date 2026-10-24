# Execution Changelog

## Implemented

- Added shared AI routes and navigation for free and member builds.
- Added a shared `useAiRuntime` lifecycle and removed AI/ASR lifecycle ownership from `useMemberRuntime`.
- Split AI TypeScript contracts and adapters from member contracts.
- Added `AppState.ai()` and registered the existing AI/ASR IPC surface in both Rust feature variants.
- Removed membership capability and expiry checks from AI generation and ASR startup.
- Removed AI from member capability DTOs while preserving moderation entitlement checks.
- Added foundation permissions for the shared AI/ASR commands.
- Normalized generated output into one plain-text paragraph, stripping list/Markdown markers and common courtesy or AI-self-reference prefixes before Unicode-safe truncation.
- Added persisted multi-model profiles with connection testing, default switching, profile validation, and redacted API-key state.
- Added Windows Credential Manager storage for API keys; secrets are never returned through IPC or persisted in SQLite.
- Added the production OpenAI-compatible Rust transport with custom endpoints, proxy, timeout, rate-limit mapping, and enforced system instructions.
- Added the default-off AI voice master switch, first-enable route confirmation, device enumeration, SAPI test playback, and selected-device speech output.
- Added automatic event triggers with trigger scopes, duplicate suppression, global/per-user cooldowns, queue priority, and bounded overflow handling.
- Added an independent AI speech queue with pause/resume, skip, clear, exact displayed/spoken text, and fail-closed device-loss handling.
- Added responsive model-profile, voice-rule, audio-route, and runtime queue controls to the shared AI page.
- Added migration `0004_shared_ai_voice_reply.sql` and registered all shared commands for both release channels.

## Verification

- Frontend full suite: 47 files, 153 tests passed in free mode.
- Rust free suite: 149 tests passed.
- Rust member suite: 150 tests passed.
- `pnpm type-check`: passed.
- `pnpm lint:check`: passed with two pre-existing `no-console` warnings in `modules/utils/logger.ts`.
- `pnpm build:web:free` and `pnpm build:web:member`: passed.
- Browser checks at 1440x900 and 390x844: AI page visible, no horizontal overflow, no console errors.

## External Follow-up

- Validate a real provider account and the selected virtual-audio route in OBS/Bilibili Live Assistant.
- Validate native secret storage and audio routing separately on macOS/Linux; unsupported native services fail closed rather than storing plaintext or silently falling back.
- Optional whisper.cpp/model delivery remains independent of the text-generation and TTS path.

## 2026-10-23 ASR Model Acquisition Amendment

- Removed the page-level context list, manual prompt, generate, regenerate, and clear-result controls. The read-only latest automatic result and copy action remain.
- Added the shared `asr_model_list` command and changed download requests to accept only a branded `modelId` plus explicit license acceptance.
- Added a Rust-owned Whisper tiny/base/small catalog, official Hugging Face download source, SHA-1 verification, application-data storage, partial-file cleanup, and verified installation markers.
- Replaced the production unavailable ASR adapter with `WhisperAsrProvider`. Model absence, download/storage/verification failures, unknown IDs, and engine absence now use distinct errors.
- Updated the AI store bootstrap, model adapter, selection dialog, command registration, permissions, and free/member contract counts.
- TDD evidence: the amended page/contract tests and Rust catalog test failed before implementation, then passed after the implementation.
- Built `biliLiveHelper_0.1.3_x64-setup.exe` (7,727,707 bytes, SHA-256 `96d9b725b8a56ee5bbd8c8fd5784ade3f7a090692b4d475daf562464accf1ce3`).
- Review re-entry: corrected ASR toggle routing so an installed model with `asr_engine_unavailable` reports an engine error instead of reopening the model-download dialog.

## 2026-10-23 Model Profile Proxy Removal

- Removed the proxy URL control, draft state, and edit-state hydration from `AiModelProfilesDialog`.
- Kept the existing nullable transport field for compatibility, but the frontend save boundary now always emits `proxyUrl: null`; saving a legacy profile clears any historical proxy value.
- Added source-contract coverage proving the label and draft binding are absent and the save payload remains type-compatible.
- TDD evidence: the focused AI page test failed on the existing proxy label before implementation and passed after the component change.
- Rebuilt `biliLiveHelper_0.1.3_x64-setup.exe` (7,736,489 bytes, SHA-256 `2546d4ec4a68c23cc57e4126e4e2daf182cea982e3a8334e1c0b1c849f7a45cb`).

## 2026-10-23 Model Profile Reasoning Effort

- Replaced the model-profile temperature control with a select-backed reasoning-effort contract.
- Added the closed values `none`, `minimal`, `low`, `medium`, `high`, `xhigh`, and `max`; new profiles and migrated profiles default to `medium`, while the nonstandard spelling `media` is rejected.
- Propagated `reasoningEffort` through TypeScript contracts, runtime validation, Tauri DTOs, domain records, SQLite persistence, and OpenAI-compatible request payloads as `reasoning_effort`.
- Added migration `0005_ai_model_reasoning_effort.sql`. The old non-null database column is retained as a compatibility-only value so existing databases can be upgraded without rebuilding the table; runtime model-profile code no longer reads or exposes it.
- Kept the separate legacy AI persona temperature setting unchanged because it is not the model-profile field shown in the requested drawer.
- TDD evidence: focused UI/adapter and Rust contract/database tests failed before implementation, then passed after the migration.
- Rebuilt `biliLiveHelper_0.1.3_x64-setup.exe` (7,739,214 bytes, SHA-256 `674b38f5b87d62e19896e740ef1bd69ca3f18322e7d2cefc7ee77ca4d434e246`).
