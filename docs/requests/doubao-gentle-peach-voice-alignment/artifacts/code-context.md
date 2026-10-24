# Code Context

## Context Requirement

Root-cause and impact analysis are material because the displayed catalog identity crosses Vue, IPC, persistence, application validation, and OS speech output. A code graph is preferred but not required for this bounded existing chain.

## Graph Availability Check

- Repository graph status: missing.
- Detection method: existing completed voice-selection code-context plus scoped symbol/reference search.
- Tool/runtime found: none.
- Iteration: bugfix intake 1.

## Installation Or Bootstrap Record

- Attempted: no.
- Method: repository-approved graph tooling discovery inherited from `ai-voice-tone-selection`.
- Result: `not_needed`; adding graph tooling exceeds this bounded defect.
- Next step: scoped direct-chain inspection.

## Fallback Record

- Fallback used: yes.
- Method: `rg` reference search and direct reads of the UI/store/IPC/application/platform path.
- Reason: no repository graph runtime is available.
- Confidence: high for the deterministic alias-to-SAPI mismatch.
- Remaining blind spot: the private Doubao-app speaker identity is not documented in the official public voice list.

## Relevant Entrypoints

- User: `AiVoiceReplyControl.vue` voice selector, test, and confirm actions.
- Internal: Pinia actions -> Tauri IPC -> `AiState` -> `AiSpeechOutput::speak`.
- Side effect: Windows SAPI playback in `windows_speak`.

## Functional Chain Under Change

The backend catalog prepends `{ voice_id: "gentle-peach", display_name: "温柔桃子" }`. Both test and queued playback pass `gentle-peach` to `SystemAiSpeechOutput`. On Windows, `windows_speak` treats that ID specially and chooses a SAPI token whose description contains `Microsoft HuiHui`; otherwise it uses the current/default SAPI token. No Doubao network request occurs anywhere in the chain.

The existing selector, persistence, and queue wiring correctly carry an opaque ID; the defect is the false preset identity and platform resolution. Other concrete system voice IDs are neighboring behavior and must remain unchanged.

## Key Symbols And Modules

- `DEFAULT_AI_VOICE_ID`, `DEFAULT_AI_VOICE_DISPLAY_NAME`
- `default_ai_voice`, `windows_voices`, `windows_speak`
- `AiSpeechOutput`, `AiState::test_output_device`, `AiVoiceRuntime::run`
- `AiVoiceReplyControl`, `useAIStore`

## Dependency And Side-Effect Boundaries

- UI/store/IPC own selection transport, not synthesis identity.
- Application owns validation and persisted route state.
- Infrastructure speech adapter owns actual synthesis/playback.
- Doubao TTS would be a new authenticated network side effect; credentials must not be guessed or borrowed from generic LLM configuration.

## Impact Scope

The smallest correct repair is centered on the default voice catalog and speech adapter, with focused application/infrastructure tests. A production Doubao route would additionally need explicit secret ownership and network/audio-output handling.

## Open Follow-Up Checks

- Confirm exact official speaker ID and service resource for the Doubao-app voice.
- Confirm how the user supplies the required API key in the packaged app.
- Once confirmed, verify no fallback path can silently emit a different system voice under the same label.

