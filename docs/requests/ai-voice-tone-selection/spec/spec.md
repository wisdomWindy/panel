# Specification: AI Voice Tone Selection

## Delivery Unit

`ai-voice-tone-selection`

## Goal and Intent Contract

- Literal request: add ASR voice-tone selection with default `温柔桃子` (`SRC-1`).
- Practical goal: the audible AI reply voice is selectable and the named default is used without prior configuration.
- Forbidden interpretation: do not attach a cosmetic field to ASR recognition or add a selector that does not affect playback.
- Success: selection is visible, persisted, returned in the typed snapshot, and applied to both test and queued TTS playback.

## Source-Grounding Map

| Contract | Grounding |
| --- | --- |
| `声色` selector | `SRC-1` source-backed |
| `温柔桃子` default | `SRC-1` source-backed |
| Persist and apply selected voice | source-derived functional necessity |
| Place field in AI audio-route dialog | code-fact-backed ownership plus page design |
| Preserve ASR behavior | closed-world scope / out-of-scope boundary |

## Closed-World Product Content

Allowed additions are the `声色` label, the `温柔桃子` default option, installed system voice options, and the data/behavior plumbing required for selection. No cloud TTS provider, branded catalog beyond `温柔桃子`, ASR model change, or unrelated AI control is authorized.

## User-Visible Behavior

1. The existing `AI 音频路由` dialog contains a required `声色` `el-select` between output device and volume.
2. Its options come from the backend-owned voice catalog. The catalog always contains `{ voiceId: "gentle-peach", displayName: "温柔桃子", isAvailable: true, isDefault: true }`, followed by available host system voices without invented aliases.
3. When an existing/legacy configuration has no voice column, migration supplies `gentle-peach`; new configurations use the same default.
4. Opening the dialog copies `snapshot.queue.selectedVoiceId` into local draft state.
5. Test playback uses the draft device, voice, and volume. Confirm/save persists all three and preserves existing route-confirmation and enable-after-save behavior.
6. Confirm/save and test are disabled when device or voice is absent. Existing loading, success, and error ownership stays unchanged.
7. Queued replies read the persisted voice at playback time and pass it to the speech output adapter.

## Field Semantics

- `voiceId`: opaque, stable selector for the voice used to synthesize AI reply audio; it is not an ASR model ID and is never displayed directly.
- `displayName`: user-visible catalog label. `温柔桃子` is source-authorized; other labels come unchanged from the host speech engine.
- `isDefault`: identifies `gentle-peach`; it controls initial selection only and is not a status badge.
- `isAvailable`: disables a catalog option when false. The default preset remains available because it resolves to a host fallback.
- Empty value: invalid for save/test and never persisted.

## Type and Contract Changes

- Frontend branded ID: `AiVoiceId`.
- Frontend catalog item: `AiVoice { voiceId, displayName, isAvailable, isDefault }`.
- `AiVoiceReplySnapshot.voices`: immutable voice catalog.
- `AiVoiceQueueSnapshot.selectedVoiceId`: non-null `AiVoiceId` after migration.
- `ai_tts_set_output_device` and `ai_tts_test` request payloads add `voiceId`.
- Rust adds `AiVoiceRecord`, `AiVoiceConfigRecord.voice_id`, `AiVoiceQueueSnapshotRecord.selected_voice_id`, and `AiSpeechOutput::voices()`; `speak` receives `voice_id`.
- IPC keeps camelCase field semantics through existing serde rules.

## Persistence and Migration

- Add migration `0006_ai_voice_tone.sql`.
- Add `voice_id TEXT NOT NULL DEFAULT 'gentle-peach'` to `t_ai_voice_config` with a non-empty check.
- Load, validate, and save the field in `SqliteMemberRepository`.
- Migration is additive and rollback surface is limited to the new column; existing rows receive the requested default.

## Playback Resolution

- Stable preset ID: `gentle-peach`; display: `温柔桃子`.
- Windows: enumerate SAPI voice tokens. For `gentle-peach`, prefer the existing project default Chinese voice (`Microsoft HuiHui` match); otherwise use the SAPI current/default token. A concrete system voice ID selects that exact token.
- Non-Windows: enumerate `tts` crate voices. `gentle-peach` resolves to the existing engine default; a concrete ID is selected through the engine voice API.
- Unknown/unavailable IDs fail fast with `ai_tts_voice_invalid`; no silent switch to another concrete saved voice.

## State, Error, and Async Rules

- Dialog draft state is the only writable UI owner until submission; the store snapshot remains server-owned.
- Existing store operation dedupe remains unchanged.
- Backend validates device, voice, and volume before persistence or test playback.
- Changing device or voice disables AI voice reply until the route is reconfirmed, matching the safety semantics of a changed audible route.
- Device-loss behavior remains unchanged.
- No new retry, toast, or error copy is introduced.

## Architecture and Ownership Signals

- Architecture design is required because one selection crosses Vue form state, TypeScript IPC types/adapters, Rust DTO/domain records, repository migration, runtime queue, and OS adapters.
- Backend speech catalog owns available voices; frontend only consumes normalized DTOs.
- Repository owns the persisted opaque ID; runtime owns resolution at playback.
- The behavior contract above must not be changed by architecture design.

## Pattern-Fit Decision

- Decision: reuse existing port/adapter and DTO/adapter patterns, Level 2.
- Triggered signal: host-specific voice enumeration and selection varies by platform.
- `AiSpeechOutput` remains the strategy boundary; Windows and non-Windows implementations own platform mechanics.
- Reject a new registry/factory/class hierarchy: there is one current variation axis already represented by the port.
- Frontend uses the existing adapter/store/component flow; no new composable, helper file, or Vue component is justified.

## Functional and Immutability Constraints

- Frontend DTO normalization stays in `modules/ai/adapters.ts`; templates do not coerce backend values.
- Voice option arrays and snapshots remain frozen.
- Validation and preferred-voice selection are deterministic helpers inside the narrow Rust infrastructure owner.
- Side effects remain in store actions, IPC commands, repository methods, and speech adapters.
- No new functional utility library is allowed.

## Production Code Quality

- Validate non-empty voice IDs at repository and application boundaries.
- Preserve nominal frontend ID separation so voice, device, model, and profile IDs cannot be interchanged.
- Do not use `null` for the persisted/default voice; absence is only tolerated at pre-migration/adapter rejection boundaries.
- Use precise `voice`, `voice_id`, and `selectedVoiceId` naming; avoid generic `selection` variables.
- No cache or memoization is needed; catalogs are small and loaded with the snapshot.

## TypeScript Context

- Governing config: `packages/bilibili-live-tool/tsconfig.json` with strict mode, bundler resolution, `@/*` path alias, Vue types, and no emit.
- Required declarations are the project-owned desktop contracts and Vue SFC typing; no repository-wide declaration scan is needed.

## Styling and Component Boundary

- Reuse Element Plus and short Tailwind utility classes only; no CSS block or hidden class constant.
- Keep the field in `AiVoiceReplyControl.vue`. One production caller and no independent lifecycle means a new SFC would be pass-through indirection.

## Testing Contract

- Red/green coverage for frontend DTO adaptation and source-level page contract.
- Rust repository test proves legacy/default persistence and round trip.
- Rust application test proves selected voice is passed to test/queued playback and invalid voices are rejected.
- IPC contract test proves camelCase snapshot and request fields.
- Run scoped Vitest, TypeScript check, Rust member tests, and formatting/lint checks appropriate to touched files.

## Human Review Readiness

- Scope edits to the AI voice-reply chain and one additive migration.
- Preserve unrelated working-tree changes.
- Map every changed hunk to selector, contract, persistence, playback, or tests.
- Remove temporary names/debug code and avoid generated-file churn.
- Review evidence must include exact commands and any environment-limited checks.

## Acceptance Criteria

1. The dialog displays a `声色` selector and `温柔桃子` is the default on an unconfigured database.
2. The selected ID survives save/reload and appears in the reply snapshot.
3. Test and queued playback receive the selected ID.
4. Invalid or missing IDs cannot be saved or tested.
5. ASR recognition behavior and unrelated TTS settings remain unchanged.

