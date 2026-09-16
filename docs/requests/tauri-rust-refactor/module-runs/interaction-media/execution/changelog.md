# Interaction Media Execution Changelog

## Delivery Unit Identifier

- request: `tauri-rust-refactor`
- module: `interaction-media`
- stage: `execute`
- self-healing iteration: 7
- pass: initial implementation after gate recovery

## Framework-Approved References

- `spec/spec.md`
- `spec/clarifications.md`
- `design/page-design.md`
- `design/architecture-design.md`
- `plan/plan.md`
- `plan/task-board.md`

## Execution Summary

- Completed IM-T01: split desktop contracts into base/live-data/media owners, added exact 34/7 media surfaces and Rust/TypeScript core DTO mirrors, and added `giftId` end to end.
- Completed IM-T02: pure config/template/queue/stat/combo/recording/heat/disk/highlight/filename rules.
- Completed IM-T03: migrated the SQLite media schema, added strongly decoded config/table repositories, and added purpose-bound import/export plus file/DB compensation.
- Completed IM-T04: added application-owned media I/O ports and concrete audio/system-speech/local-process/notification/effect-window/hotkey/preview adapters.
- Completed IM-T05: composed four media runtimes behind one accepted-event consumer with bounded queues, fallback states, recording protection and deterministic shutdown.
- Completed IM-T06: registered the exact common media IPC surface, targeted main/effect events, added a restricted effect bootstrap and an owner/range-bound preview protocol.
- Completed IM-T07: added fail-closed immutable media adapters, pure validators/formatters, four generation-aware stores and the single six-listener main-window runtime.
- Completed IM-T08: migrated captain effect, gift reminder and gift-sound pages to store-only flows, added the restricted Canvas effect surface and removed browser/Electron media controls.
- Completed IM-T09: migrated TTS, recording and highlight pages and closed authorized native recording-file operations.
- Completed IM-T10: removed the full owned Electron/browser media closure, eliminated the stale `/effect-preview` compatibility route, ran strict checks and produced both free/member installers.

## Self-Healing Execution Notes

- No verify/review blocker re-entry is active; this is the initial implementation pass for this module.
- Progress: plan gate repaired and approved; TDD red tests added for gift identity and the closed media surface.
- Verification focus: exact 34/7 surface, camelCase gift identity, null/unit/path semantics, and unchanged live-data behavior.

## Task-Board Updates

- IM-T01 through IM-T10: `done`.

## Changed Files And Owned Symbols

- IM-T01 changed Rust live event/domain DTO mapping, TypeScript contract owners, path-purpose enums, focused fixtures/tests and execution records.
- IM-T02 added `domain/media` pure models, validators, reducers and deterministic state/queue rules.
- IM-T03 added migration `0002_interaction_media.sql` and `infrastructure/media/{repository,files,tests}.rs`; it extended media config models and advanced `SCHEMA_VERSION` to 2.
- IM-T04 added `application/media/ports.rs` and seven infrastructure adapters; it registered Rust-side notification/global-shortcut plugins and locked the approved media dependencies.
- IM-T05 added `application/media/{effects,gifts,tts,recording,state,tests}.rs`, extended the accepted-event port, composed media state during Tauri bootstrap and registered main-window shutdown cleanup.
- IM-T06 added `ipc/media.rs`, closed request/result DTOs, common free/member handlers and permissions, targeted event delivery, a render-only effect bootstrap and the `media-preview` protocol.
- IM-T07 added `modules/media/{adapters,validation,formatters}.ts`, four explicit `stores/media` owners, `useMediaRuntime.ts`, root lifecycle wiring and focused pure/store/runtime tests. It also carried `roomId` through `GiftRecord` so valid gift snapshots survive strict adaptation.
- IM-T08 added the captain/gift pages and approved extracted components, a DPR/reduced-motion Canvas lifecycle, persisted sound-grant reuse and the reminder navigation group.
- IM-T09 added the TTS route/page/queue panel and rebuilt recording around the new store, status band, settings dialog and store-free recording/highlight tables. Rust file ports now stage, restore, finalize, move and export repository-owned media files.

## Introduced-Content Provenance Ledger

| File/symbol | Content | Task/allowlist | Authority | Result |
| --- | --- | --- | --- | --- |
| `live-data-contracts.test.ts` gift fixture | `giftId` separate from display name | IM-T01 / gift identity | PRD 9.7.2, 9.8.2-9.8.5 | authorized |
| `media-contracts.test.ts` | exact 34 commands, 7 events and representative approved DTO semantics | IM-T01 / IM-06 | spec Command/Event/Core DTO tables | authorized |
| `0002_interaction_media.sql` | TTS/recording/highlight fields and query/identity indexes | IM-T03 / persistence | PRD 9.8/9.12/9.13 + ADR-IM-04 | authorized |
| `GiftSoundDocument` | version 1 gift identity/name/file portability metadata/volume/enabled | IM-T03 / batch import-export | approved technical-contract clarification | authorized |
| `compensated_delete` / `compensated_move` | staged file operations with database-failure restoration | IM-T03 / atomic file+DB operations | ADR-IM-04 | authorized |
| `application/media/ports.rs` | narrow repository/file/audio/speech/process/notification/window/hotkey/preview traits | IM-T04 / I/O boundaries | ADR-IM-03/07 | authorized |
| `rodio 0.20.1`, `tts 0.26.3`, Tauri notification/global-shortcut v2 | approved concrete local integrations | IM-T04 / dependency gate | approved plan + locked official crate APIs | authorized |
| `PreviewRegistry` | random owner-bound expiring sessions and bounded byte ranges | IM-T04 / opaque preview | ADR-IM-07 | authorized |
| `MediaState` and four runtimes | accepted-only idempotent media routing, bounded FIFO/priority work, fallback, recording retry/waterline/highlights and shutdown | IM-T05 / runtime boundary | PRD 9.7/9.8/9.12/9.13 + ADR-IM-01/02 | authorized |
| `ipc/media.rs` and media request DTOs | exact 34 commands, main/effect-targeted event sink and bounded owner-checked custom protocol | IM-T06 / native boundary | IM-06, IM-A09..11, ADR-IM-05/07 | authorized |
| `EffectWindowRoot.vue` bootstrap | one listener and no invoke access on `?surface=effect` | IM-T06 / effect isolation | approved plan and zero-command capability | authorized |
| media adapters/stores/root runtime | fail-closed immutable DTO mapping, latest-wins reads, per-operation dedupe and six main-window listeners | IM-T07 / frontend boundary | IM-06/07 and ADR-IM-06 | authorized |
| captain/gift pages and media components | store-only configuration, bounded gift presentation and restricted Canvas effect rendering | IM-T08 / approved page allowlist | BF-01..03 and IM-A01..04 | authorized |
| TTS/recording pages and file mutation port | queue controls, status/config/files/highlights UI and compensated authorized file actions | IM-T09 / approved page and native file flow | BF-04..07 and IM-A05..09 | authorized |
| plan/state/execution documents | workflow gate and evidence | product-content delta: none | framework execution contract | authorized |

## Implementation Decisions

- Rust serde remains authoritative and TypeScript mirrors exact camelCase fields.
- `giftId` is identity; `giftName` remains display text.
- No raw authoritative path may enter media contracts.
- Media JSON configs are validated both before persistence and after decoding; invalid stored data fails closed.
- Gift-sound replacement validates the complete document before an explicit SQL transaction; recording/highlight paths remain Rust-only persistence data.
- `rodio::OutputStream` remains on its dedicated worker thread because the Windows backend is not `Send`; only a stop/finished handle crosses the async boundary.
- Recorder output is drained continuously and diagnostics are bounded; every process is `kill_on_drop`, explicitly stoppable and waitable.
- Live-data persistence and analytics complete before media consumption; media errors remain isolated from the accepted event pipeline.
- System speech waits for each utterance to finish while releasing its mutex between polls, preserving serial priority order and allowing stop to interrupt playback.
- Recording performs exactly three spawn attempts, stops below the configured disk waterline, marks surge/manual highlight windows and unregisters its shortcut during shutdown.

## TDD Evidence

- Red: Rust failed because `GiftEventDto.gift_id` did not exist; Vitest failed because `MEDIA_COMMAND_NAMES` was undefined.
- Green: Rust full suite 43/43, platform/feed Vitest 12/12, strict TypeScript and focused ESLint passed.
- IM-T02 red: 8 focused domain tests failed at explicit unimplemented rule boundaries.
- IM-T02 green: 8/8 focused tests, 51/51 total Rust tests, strict Clippy and TypeScript passed.
- IM-T03 red: focused compilation failed on the intentionally missing media repository/files modules and eleven missing typed config models.
- IM-T03 green: 5/5 focused persistence/file tests, 37/37 Rust library tests, 18/18 foundation contract tests, strict Clippy and TypeScript typecheck passed.
- IM-T04 red: seven missing adapter modules; the first implementation compile then exposed the Windows `OutputStream: !Send` constraint and trait-object test Debug bounds.
- IM-T04 green: 10/10 focused adapter tests, 43/43 Rust library tests, 18/18 foundation contract tests, strict Clippy, member all-target check and TypeScript typecheck passed.
- IM-T05 red: missing runtime modules first failed compilation; follow-up compilation exposed the heat-buffer borrow boundary and strict MSRV lint.
- IM-T05 green: 4/4 focused runtime tests, 47/47 Rust library tests, 18/18 foundation contract tests, strict Clippy, free/member all-target checks and TypeScript typecheck passed.
- IM-T06 red: the frozen media registry initially had no command implementations, request DTOs, permissions, targeted sink or protocol handler.
- IM-T06 green: 52/52 Rust library tests, 18/18 foundation contract tests, strict Clippy, free/member all-target checks, TypeScript typecheck and free/member Web builds passed.
- IM-T07 green: 20/20 focused Vitest tests, focused ESLint, strict TypeScript, 29/29 focused Rust media tests and sequential free/member Web builds passed.
- IM-T08 green: 23/23 focused Vitest tests, focused ESLint, strict TypeScript, 29/29 focused Rust media tests and free Web build passed; 1280 and 1024 viewport checks showed no overlap.
- IM-T09 green: 25/25 focused Vitest tests and strict TypeScript passed; native file adapter 12/12, media runtime 4/4 and IPC contract 5/5 tests passed.
- IM-T10 green: all 50 Vitest tests, strict TypeScript and zero-error ESLint passed; all 53 Rust tests and strict Clippy passed in both free and member feature builds. Free/member Tauri MSI and NSIS installers were produced. Legacy Electron media services, engines, stores, duplicate settings, orphan callers and the stale `/effect-preview` redirect were removed.

## Change-Chain Review Evidence

- Pre-change chain checked: Rust `LiveEvent::Gift` definition -> validation/analytics/repository -> IPC `GiftEventDto` -> typed desktop event -> root live-data runtime -> feed/pages.
- Old plugin, gift service and media stores are recorded as later migration targets, not current production contract owners.

## Production Code Quality / Architecture / Functional Notes

- Rust persistence owns media config/table truth and application traits own all external I/O boundaries.
- DTO inputs will be immutable and normalization will remain at domain/adapter boundaries.
- ADR-IM-01/04/06 and dependency matrix remain applicable.

## Human Review / Styling / Removal Notes

- No style or removal change in IM-T01.
- Unrelated working-tree changes and the PRD modification remain untouched.

## API And TypeScript Context Notes

- Read `tsconfig.eslint.json` -> strict `tsconfig.json`: bundler resolution, ES2020/DOM, `@/*`, noUnused and Vue declarations apply.
- Read only the current desktop contracts, transport consumers and related focused tests; no broad `.d.ts` scan.

## Deviations, Blockers, And Rollback Triggers

- None currently. Any additional command/event/product field triggers rollback to plan/spec.

## Verification Handoff Notes

- IM-T05 is ready for review/verify. Cloud TTS and bundled FFmpeg remain explicitly unavailable pending external inputs; the runtime reports those states without fake success. IM-T06 must expose only the approved 34 commands and 7 targeted events.
- IM-T06 is ready for review/verify. Live stream URL resolution and recording/highlight file mutations remain explicit unavailable outcomes until T09 supplies their safe transaction owners; no command reports fake success. IM-T07 must retain one main listener owner and map DTOs before store commit.
- IM-T07 is ready for review/verify. All main-window media events now enter through one root owner; malformed payloads preserve the previous snapshot and surface a mapped error. IM-T08 must keep page drafts local and consume stores only.
- IM-T08 is ready for review/verify. Missing captain assets remain visibly unavailable and no renderer audio/file authority was reintroduced. IM-T09 must preserve opaque preview and keep recording start unavailable until a real stream resolver and FFmpeg source exist.
- IM-T10 is ready for verify/review. The local Web preview proves responsive/degraded page behavior only; it has no Tauri bridge and is not native success evidence. Cloud TTS, bundled FFmpeg, a real live-stream resolver and final captain artwork remain explicit external inputs, so their real-runtime performance paths were not run and never report fake success.
