# Live Data And Safety Execution Changelog

## Delivery Unit Identifier

- request: `tauri-rust-refactor`
- module: `live-data-safety`
- framework-approved spec: `spec/spec.md`
- framework-approved plan: `plan/plan.md`
- self-healing iteration: `7`
- execution pass: initial implementation after gate recovery

## Execution Summary

- Status: LDS-T01 through LDS-T09 complete; ready for verification and review.
- Scope: implement plan tasks LDS-T01 through LDS-T09 only.
- Product-content rule: approved closed-world allowlist; production Bilibili transport remains unavailable until externally configured.

## Self-Healing Execution Notes

- Evidence: the approved command table contains 20 independent command names while architecture/plan budget text said `<=15` by counting grouped rows.
- Defect class: `architecture-gap`; identity: `live-data command public-surface budget`.
- Attempts: same-stage 1, total 1. Progress: exact count evidence and artifact repair; no production code had changed.
- Repair: architecture and plan budget corrected to `<=20`; command names, payloads, behavior and product content are unchanged.

## Task-Board Updates

- LDS-T01..T09: done.

## Changed Files And Owned Symbols

- `src-tauri/src/ipc/dto.rs`: 20 command/9 event name constants and serde DTO contracts.
- `src/renderer/platform/desktop/contracts.ts`: mirrored branded IDs, discriminated contracts and typed maps.
- Rust/TS live-data contract tests: public-surface, camelCase, money and epoch-ms evidence.
- `domain/live_data/*`: identity normalization, validation, ordered safety pipeline, AC/regex matcher and immutable analytics reducer.
- `infrastructure/live_data/*`: explicit SQLite repository and approved file codecs; `PathGrantStore` now supports canonical save-file grants with typed purposes.
- `application/live_data/*`: injected ports, single generation runtime, fixed reconnect, cancellation, ordered persistence/aggregation/publication and cadence owners.
- `ipc/live_data.rs`, `lib.rs`, permissions and desktop barrel: 20 thin commands, 9 typed event mappings, shared Rust rank-delta presenter, purpose-bound save dialog and identical free/member common surface.
- `modules/live-data/*`, scoped live-data stores and `useLiveDataRuntime`: pure immutable adapters, shared formatters, latest-wins request generations and one root listener owner; the temporary media projection was removed with the legacy owner chain in T09.

## Introduced-Content Provenance Ledger

| File / hunk | Content | Task / allowlist | Authority | Result |
| --- | --- | --- | --- | --- |
| architecture + plan command budget | corrected grouped-row count from 15 to 20 | T01; technical only | approved spec command table | authorized; product-content delta: none |
| Rust/TS live data contracts | approved commands, events, fields, enums, nullability and units | T01; API allowlist | spec API And Data Contracts | authorized |
| Rust domain rules | identity, blacklist-first filtering, exact/fuzzy matching, five metrics and three ranking meanings | T02; technical/domain | LDS-01..04,06,08..11 | authorized; product-content delta: none |
| sensitive word timestamps | removed from DTO after diff provenance review | T01 closed-world correction | spec sensitive row fields | removed |
| sensitive/blacklist persistence and file formats | approved CRUD, transaction rollback, CSV/JSON/TXT fields | T03 allowlist | LDS-05,07,12 | authorized |
| connection/runtime states | disconnected/connecting/connected/reconnecting/error, fixed retry and qualified events | T04 allowlist | LDS-01,02,04,06,08,10 | authorized |
| command/event mappings and Chinese errors | approved DTOs, stable codes and exact PRD messages | T05 API/copy allowlist | spec API and copy contracts | authorized; no added product behavior |
| root runtime/store state | approved event ownership and async behavior | T06 architecture allowlist | spec 8.6/8.9, plan T06/T09 | authorized; temporary media projection removed |
| filtered/today footer counts | source-required panel and safety footer values omitted from initial DTOs | T07 self-healing contract repair | PRD 9.1.5, 9.3.3, 9.5.3 | authorized; same 20 commands/9 events |
| connection, danmaku and risk-control UI | approved controls, rows, dialogs, states and exact copy only | T07 page allowlist | approved page design + plan T07 | authorized |

## Implementation Decisions

- Rust serde is the canonical contract; TypeScript mirrors it and adds branded RoomId/UserUid/EventId compile-time distinctions.
- AC uses leftmost-longest matching so overlapping exact rules select the most specific phrase; analytics consumes owned state and returns the updated state without mutating caller input.
- Existing 0001 schema is sufficient for current safety CRUD and event-backed analytics; no migration was added. Save targets canonicalize the parent and preserve the selected filename.
- `LiveDataState` owns one watch cancellation channel and session generation; event effects serialize through an effect lock so safety mutations cannot race the filter decision.
- `RankingPresenter` is shared by query and event paths so previous rank and up/down/new/same are calculated once in Rust. Safety bootstrap now fails closed on blacklist repository errors instead of treating them as an empty list.

## Expert Frontend Engineering Execution Notes

- Root owns one runtime; stores contain server snapshots plus request state and never import the runtime. T09 removed the temporary media projection together with the old renderer WebSocket/filter/aggregation owners.
- T07 uses flat operational bands/tables, shared identity/detail components, pessimistic safety mutations, system path grants and root connection controls. The application shell is always mounted; login no longer blocks free shared functionality.

## Frontend Architecture Quality Execution Notes

- Pre-code budget mismatch repaired. Approved ownership, dependency matrix, state topology, extension seams and migration cleanup trigger are unchanged.

## Production Code Quality Execution Notes

- Type-first completed for the approved IPC surface. Invalid/nullable semantics remain explicit; no broad business DTO records or `any` were added.

## Human Review Readiness Notes

- Pre-code diff is limited to approved delivery artifacts. No PRD or unrelated source file was edited.

## TDD Evidence Or Documented Exception

- LDS-T01 red: Rust unresolved DTO imports; TS surface arrays undefined. Green: focused Rust 2/2, Vitest 2/2, package TypeScript type-check pass.
- LDS-T02 red: missing domain exports. Green: 6/6 domain tests, including 10k lookup fixture; clippy pass. Intermediate red exposed AC prefix collision and was fixed with leftmost-longest matching.
- LDS-T03 red: missing repository/codec modules and path purposes. Green: 3/3 infrastructure tests and clippy pass. Windows 8.3 path spelling was normalized in the assertion to compare canonical targets.
- LDS-T04 red: missing application ports/state. Green: 3/3 application tests, including paused-time fixed retry/bound evidence and unavailable/filtered pipeline integration.
- LDS-T05 green: Rust 21/21; platform gateway Vitest 7/7; TypeScript type-check; free/member `cargo clippy --all-targets -D warnings` all pass. ACL test verifies every approved live-data command is present in both common registry metadata and the main-window permission.
- LDS-T06 green: live-data Vitest 7/7, platform Vitest 7/7, TypeScript type-check and focused ESLint pass; free/member Web builds pass with only the pre-existing large-chunk warning. Tests cover idempotent nine-listener ownership, cleanup including late registration, latest-wins generations, immutable adapters and three-second rank highlight expiry.
- LDS-T07 green: live-data Vitest 10/10 including validation and pessimistic mutation tests; Rust 21/21 and free strict clippy; TypeScript and focused ESLint pass; free/member Web builds pass. Prohibited login gate, configurable reconnect and blacklist-duration scan is empty; interactive controls expose labels and destructive actions own confirmations.

## Change-Chain Review Evidence

- Pre-change scoped caller chain is recorded in `artifacts/code-context.md`; no production chain has been edited yet.

## Removal Cleanup Evidence

- Deleted the old renderer `DanmakuService`, Bilibili WebSocket client/protocol, danmaku manager/parser, local blacklist/sensitive-word engines, old room/danmaku stores and login/dashboard compatibility components.
- Gift list and recording now consume the Rust-backed live room/feed owners; no compatibility media projection remains.
- Removed configurable reconnect and duplicate local-sensitive-word controls, stores, Electron config fields and translation keys.
- Removed the stale Vite `events` alias after its deleted event-emitter target lost every caller, and excluded `src-tauri/target` from Vite file watching so concurrent Rust builds cannot terminate the development server on locked MSVC artifacts.
- Post-change symbol scans found no legacy owner, compatibility projection, rank helper, reconnect-setting or local replacement-field references in the owned chain.

## Functional-Programming Execution Notes

- Domain filtering, identity normalization, aggregation, ranking, codecs, TypeScript adapters, formatters and chart options are deterministic and tested; I/O remains in ports, repositories, IPC, stores and lifecycle owners.
- Store snapshots use replacement/copy semantics. Query generations reject stale results, and path grants are revoked in `finally` on both success and failure.

## Frontend Styling Execution Notes

- Connection, danmaku, safety, dashboard, ranking and gift list changes use reviewable Tailwind utility classes only; scoped CSS and overlong class strings were removed.
- 1024x680 screenshots verify toolbar wrapping, table/footer visibility and inline degraded states. ECharts keeps a 280px minimum height and adjacent text metrics.

## API Contract And Adapter Execution Notes

- Authoritative source is Rust serde DTO from approved spec. The 20-command count repair changes no API semantics.

## TypeScript Context Recovery Notes

- Governing config: `tsconfig.eslint.json` -> strict `tsconfig.json`; bundler resolution, ES2020/DOM, noUnused and `@/*` alias apply.
- Relevant declarations: Vue/env declarations and `platform/desktop/contracts.ts`; no repository-wide declaration sweep.

## Architecture-Design Conformance Or Invalidation Notes

- One pre-code budget mismatch was found and repaired in iteration 7. No remaining architecture invalidation is known.

## Code-Context Updates

- Added exact public-command count evidence to `artifacts/code-context.md`.

## Deviations, Blockers, And Rollback Triggers

- Current blocker: none.
- Roll back if implementation needs more than 20 commands, more than 9 events, a schema semantic change, or new product-visible behavior.

## Verification Handoff Notes

- LDS-T08: five metrics, three trend options/tooltips, four ranges, three ranking tabs, TOP 10/20/50/100, three-second rank deltas, detail drawer and grant-backed exports are implemented.
- LDS-T09: legacy owners and duplicate configuration paths are removed; gift list consumes the Rust-backed feed.
- Self-healing fixes rechecked: structured Web runtime-unavailable and concurrent-mutation errors, no automatic duplicate toast, safety/analytics grant revocation, historical export rejection, explicit store error normalization, zh-CN WiX installer codepage, stale Vite alias removal and stable development-server watching during Rust/Tauri builds.
- Evidence: Rust free/member tests and strict Clippy pass; TypeScript pass; focused Vitest 23/23 pass; full ESLint has 0 errors and 104 pre-existing warnings outside this module; free/member Web and Tauri MSI/NSIS builds pass.
- Verify must still distinguish fake-port coverage from production Bilibili connectivity, which remains intentionally unavailable without the missing external configuration.
