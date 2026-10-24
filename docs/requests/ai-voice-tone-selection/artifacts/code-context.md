# Code Context: AI Voice Tone Selection

## Context Requirement

Structural context is required at architecture design because one setting crosses Vue, TypeScript contracts, Rust IPC/application ports, SQLite, and platform speech adapters. A code graph is preferred but not required for this bounded chain.

## Graph Availability Check

- Status: missing.
- Detection: scoped `rg --files` search for graph configuration/tooling under repository and agent roots.
- Tool/runtime found: none.
- Iteration: architecture-design loop iteration 3.

## Installation or Bootstrap Record

- Attempted: no.
- Method: repository/team-preferred tooling discovery.
- Result: `not_needed`; no repository-approved graph tool or bootstrap command exists, and adding external tooling would exceed this delivery unit.
- Next step: scoped symbol/caller search and direct chain reading.

## Fallback Record

- Used: yes.
- Method: `rg` symbol search plus direct reads of callers, DTOs, repository methods, runtime, adapters, migrations, and focused tests.
- Confidence: high for the bounded AI voice-reply chain.
- Blind spot: host SAPI runtime enumeration must be validated by compile/tests because it is OS-specific.

## Relevant Entrypoints

- UI: `AiVoiceReplyControl.vue` route dialog emits to `views/ai/index.vue`.
- Store: `useAIStore.setOutputDevice` and `testOutputDevice` invoke typed desktop commands.
- IPC: `ai_tts_set_output_device`, `ai_tts_test`, and `ai_voice_reply_snapshot`.
- Runtime: `AiState::set_output_device`, `test_output_device`, and `AiVoiceRuntime::run`.
- Platform: `SystemAiSpeechOutput` and Windows `windows_speak`.

## Functional Chain Under Change

Dialog draft -> page event handler -> Pinia action -> `invokeDesktop` payload -> Rust IPC DTO -> `AiState` validation/persistence or test -> `AiSpeechOutput` -> platform voice token. Snapshot flows back through Rust DTO -> TypeScript adapter -> immutable store snapshot -> dialog hydration. Queue playback separately reloads persisted config before calling the same speech port.

Neighboring ASR model/start/stop and general TTS page chains are not changed. Existing frontend adapter/page tests and Rust repository/application/IPC tests are adaptation targets, not production owners.

## Key Symbols and Modules

- `AiVoiceReplyControl`, `AiVoiceReplySnapshot`, `adaptAiVoiceReplySnapshot`, `useAIStore`.
- `AiVoiceConfigRecord`, `AiVoiceQueueSnapshotRecord`, `AiSpeechOutput`, `AiState`.
- `SqliteMemberRepository`, `SystemAiSpeechOutput`, `AiVoiceReplySnapshotDto`.

## Dependency and Side-Effect Boundaries

- UI owns temporary form state only.
- Store owns command orchestration and normalized snapshot state.
- IPC owns serialization, application owns validation/state transition, repository owns SQLite writes, speech adapter owns OS APIs.
- Queue worker owns asynchronous speech side effects and cancellation.

## Impact Scope

Expected edits are one Vue component/page handler, one store, TypeScript contracts/adapter/tests, one migration, member port/runtime/repository/speech adapter/IPC, and focused Rust tests.

## Open Follow-Up Checks

- Compile the Windows SAPI token enumeration/selection calls.
- Verify every `AiSpeechOutput` test double adopts the expanded port.
- Re-run reference search after edits for stale three-argument `speak` calls and payloads without `voiceId`.

