# Execution Changelog: AI Voice Tone Selection

## Delivery Unit

`ai-voice-tone-selection`

## Implemented Changes

- Added the `0006_ai_voice_tone.sql` additive migration with a non-empty `voice_id` defaulting to `gentle-peach`.
- Extended AI voice configuration, repository mapping, queue snapshots, IPC DTOs, and desktop contracts with a stable voice ID.
- Added backend-owned voice catalogs and actual platform selection for Windows SAPI and the non-Windows `tts` adapter.
- Added the required `声色` selector to the existing AI audio-route dialog, hydrated from the snapshot and defaulting to `温柔桃子`.
- Routed the draft selection through test playback, save/reload, enable validation, and queued playback.
- Added frontend adapter/page contract tests plus Rust persistence, migration, validation, and playback propagation coverage.
- Final review tightened draft-route availability so removed or unavailable device/voice IDs cannot enable test or save actions.

## Scope Notes

- ASR recognition behavior is unchanged; the selector controls the audible AI reply TTS path.
- No dependency, cloud provider, general TTS page, or unrelated UI behavior was added.
- Existing working-tree changes were preserved.

## Execute Evidence

- Focused frontend tests: 2 files, 6 tests passed.
- Rust application tests: 7 passed.
- Rust full library tests: 131 passed.
- Database migration tests: 5 passed.
- TypeScript type-check: passed.
- Scoped ESLint: passed.
- Rust formatting: passed.
- Free and member web production builds: passed.
- Member Rust feature check: passed.
- Review repair recheck: focused Vitest, type-check, scoped ESLint, and member web build passed.

## Environment Notes

- The package declares Node `>=24 <25`; the local runtime is Node `22.21.0`. pnpm emitted an engine warning, while all executed frontend checks and builds completed successfully.
- Vite retained the repository's existing large-chunk warning during production builds.
