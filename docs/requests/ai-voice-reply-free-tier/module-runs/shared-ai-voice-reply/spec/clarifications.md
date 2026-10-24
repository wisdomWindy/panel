# Clarifications

## Resolved Upstream

- AI voice reply is free and membership-independent.
- One master switch controls generation and AI TTS.
- Software does not bundle model weights or send generated text to Bilibili. ASR weights are selected and downloaded on demand from the Rust-owned official catalog.
- Provider credentials and audio/OBS availability are runtime integration inputs.

## Framework Assumptions

- The existing typed Tauri transport and Pinia conventions are retained because they are reversible technical choices and do not change product semantics.
- Existing shared SQLite bootstrap remains the persistence foundation; schema evolution must preserve the current single AI configuration as the default migrated profile/persona.

## External Interface Pending

- Production credentials/models and physical/virtual audio devices are unavailable in repository tests. Tests use deterministic ports/fakes and verify honest failure for unavailable real dependencies.
- Automatic OBS meter inspection is not implemented because no source-backed API contract exists; test playback plus explicit user confirmation satisfies the source flow.

## 2026-10-23 Approved Clarification

- The user explicitly removed the context/manual prompt/generate/regenerate/clear-result surface from the AI page. Existing internal generation commands may remain for runtime compatibility, but they are not exposed as page controls.
- Starting ASR without an installed model must open a model-selection dialog. The approved initial choices are Whisper tiny, base, and small; Rust owns URLs, checksums, and filesystem destinations.
- The existing generic “模型资源暂不可用” dead end is replaced by actionable loading, selection, downloading, installed, retryable download failure, and distinct engine-unavailable states.

## 2026-10-23 Model Profile Amendment

- The user explicitly removed the proxy URL field from the AI model profile add/edit surface.
- Frontend profile saves always submit `proxyUrl: null`; the nullable transport and persistence field remains only for backward-compatible contract/schema handling and is not user-configurable.

## 2026-10-23 Reasoning Effort Amendment

- The user explicitly replaced the numeric temperature field with a reasoning-effort selector.
- The user's `media` example is normalized to the standard value `medium`; `media` is not accepted or persisted.
- The approved closed set is `none`, `minimal`, `low`, `medium`, `high`, `xhigh`, and `max`, matching the current official OpenAI API/Agents SDK vocabulary. New and migrated profiles default to `medium`.
- Unsupported provider/model combinations fail through the existing connection-test/provider-error path. The application must not silently substitute another effort or restore temperature.

## Blocking Clarifications

None.

