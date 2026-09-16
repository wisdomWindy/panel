# Interaction Media Engineering Specification

## Delivery Unit Identifier

- request: `tauri-rust-refactor`
- module: `interaction-media`
- sequence: 3 of 6
- stage: `spec`
- `architecture_design_required=true`
- source version: PRD `2026-09-14-final`, read-only working-tree authority

## Background And Goals

The current media implementation is split across Vue pages/stores, renderer audio engines, generic Electron IPC and Node child-process/file modules. After the prior live-data migration, TTS and automatic highlight paths no longer receive accepted events, while several listeners and media resources lack cleanup owners. This delivery moves the PRD 9.6-9.8 and 9.12-9.13 behavior to the Tauri/Rust boundary without restoring a renderer event projection.

Goals:

- Deliver local captain effects, gift alerts/sounds, TTS, FFmpeg recording and highlights in both free and member builds.
- Make Rust the only owner of accepted-event fan-out, media queues, notification, persistence, Sidecar processes, file access, disk protection, global F9 and effect-window lifecycle.
- Give Vue typed snapshots, local form drafts and presentational rendering only.
- Remove the owned Electron/browser media chain after all production callers are migrated.
- Keep real external resources fail-closed: no cloud TTS or bundled FFmpeg success is claimed while contracts/artifacts are absent.

## In Scope

- `IM-01` captain effect configuration, preview, ordered trigger queue, sound synchronization, restricted effect window and main-window fallback.
- `IM-02` gift reminder configuration, local test, current-session totals, latest 20 accepted gifts and detail view.
- `IM-03` per-gift MP3/WAV bindings, row/global volume, enable, preview, replace/remove, batch import/export and queue overflow behavior.
- `IM-04` TTS configuration, template validation, accepted-event priority queue, engine fallback, test, pause/continue, clear and status presentation.
- `IM-05` recording configuration, FFmpeg selection, start/stop/finalize, disk quota/waterline, automatic/manual highlights, F9 and recording/highlight management.
- `IM-06` typed Tauri Command/Event DTOs, purpose-bound path grants, SQLite repositories, structured errors and free/member common capabilities.
- `IM-07` PRD page/navigation inputs, feedback/responsive/accessibility contract and Tailwind-only authored styling.
- `IM-08` migration/removal of owned Electron main/preload callers, renderer services/engines/duplicate settings and invalid tests/mocks.

## Out Of Scope

- Sending danmaku, thank-you messages, moderation actions or any outward live-room interaction.
- AI/ASR, membership, plugin runtime, update/release UI and final global-shell composition.
- Inventing cloud TTS provider endpoints/credentials or shipping an unprovided FFmpeg binary.
- Unapproved effect authoring, 2D/3D render selection, editable effect text, recording pause/resume, gift/captain automatic highlight policies, arbitrary browser downloads or direct path editing.
- Final marketing names/artwork for the nine or more unspecified captain preset assets; runtime catalog support and count validation are in scope.

## Trigger And Start Conditions

- The module begins after `live-data-safety` review passed and its accepted Rust events, room snapshot and path-grant infrastructure are stable.
- Media processing starts only from accepted Rust events after identity/sensitive-word/blacklist processing.
- Effect/gift/TTS configuration can be edited while disconnected; live triggers require accepted events.
- Recording start requires a connected/live room, a valid recording-directory grant, an available configured FFmpeg source and disk remaining above the configured waterline.

## Requirement Split Summary

- Owns PRD functions 6, 7, 8, 12 and 13 only.
- Depends on `platform-foundation` for Tauri, SQLite, typed transport, build channels and grants.
- Depends on `live-data-safety` for accepted event order and room truth.
- Supplies recording status/navigation inputs to the later `application-shell` module.
- Leaves plugin effect registration to `extensions-release-resilience`; no plugin API may own media execution in this delivery.

## Source Grounding And Traceability

| Spec item | Grounding | Product semantic | Authority | Plan obligation |
| --- | --- | --- | --- | --- |
| IM-01 effect config/window/queue/fallback | source-backed | yes | PRD 9.6 | Rust queue/window/audio plus config page and renderer |
| IM-02 alert config/statistics/latest/detail | source-backed | yes | PRD 9.7 | Rust alert/stat snapshot plus page |
| IM-03 bindings/MP3-WAV/queue/import-export | source-backed | yes | PRD 9.8 | repository, audio queue, grant-backed I/O and table |
| IM-04 TTS priorities/config/queue/fallback | source-backed | yes | PRD 9.12 | Rust TTS queue/ports plus page/status event |
| IM-05 recording/highlights/disk/FFmpeg/F9 | source-backed | yes | PRD 9.13 | Rust Sidecar/storage/hotkey plus management page |
| Purpose-bound opaque path grants | source-backed + code-fact-backed | yes, implementation constrained | PRD 10.1-10.3; existing `PathGrantStore` | extend purposes; never expose raw path authority |
| Integer minor units, epoch milliseconds and `GiftEvent.giftId` | source-backed + code-fact-backed | gift identity is business-semantic | PRD 9.7.2, 9.8.2-9.8.5; approved live-data contracts and SQLite `gift_id` | add gift ID to the shared accepted-event DTO and preserve units through adapters |
| Direct Rust accepted-event fan-out | source-backed + code-fact-backed | yes, structure constrained | PRD event flow; removed renderer projection | no renderer forwarding chain |
| Structured Chinese operational feedback | confirmed-decision | yes | user 2026-09-14 `按建议继续`; PRD exact labels win | central error mapping; no marketing/help additions |
| External TTS/FFmpeg unavailable states | source-backed | yes | PRD 9.12.6, 9.13.3/9.13.8, snapshot open items | fail closed and record deployment condition |
| Tailwind-only authored DOM styling | code-fact-backed | no product delta | project policy and approved page design | remove touched scoped/inline styles |
| Delete Electron/browser media owners | code-fact-backed | no product delta | scoped caller closure in `artifacts/code-context.md` | migrate all callers/tests then remove closure |

Closed-world product content is exactly the fields, actions, states, defaults, navigation and feedback mapped in `design/page-design.md`. Items absent from that mapping are forbidden until source confirmation.

## Workflow Efficiency Profile

- `speed_profile.level`: `S3 broad`; this is the lowest safe level because the change crosses Rust domain/application/infrastructure/IPC, window/Sidecar resources, typed transport, five UI surfaces and destructive legacy removal.
- Artifact density: full. Downstream context breadth: complete owned caller/side-effect chain plus the adjacent shell/plugin consumers named in code context.
- Upgrade condition: none exists above S3. Discovery of a new external product contract requires upstream clarification, not wider speculative implementation.

## Self-Healing Completion Bar

- Requirement consistency: all IM-01..IM-08 behavior and exact PRD defaults/states remain traceable; no send path or unapproved control survives.
- Business logic: verify event filtering order, queue ordering/overflow, combo behavior, fallback states, template whitelist, recording transitions, retry count, disk waterline/quota and highlight windows.
- Syntax/tooling: Rust fmt/test/strict Clippy, TypeScript strict check, focused/full lint, Vitest, free/member Web builds and free/member Tauri builds must pass.
- Runtime safety: validate unavailable resources, invalid/null DTOs, async rejection, dedupe, stale results, listener/window/audio/process/timer cleanup, app exit, dialog cancellation and interrupted recording.
- Code organization: verify imports/exports/callers/tests/mocks/generated declarations, helper locality, Vue boundaries, capabilities and complete Electron/browser media cleanup.
- Maintainability: precise domain names, short focused functions, narrow constants, immutable transforms, explicit side effects and reviewer-readable task mapping.
- In-scope blockers may not be deferred: duplicate media ownership, raw path/browser download, lost event fan-out, listener/process leaks, fake external success or non-PRD fields.
- Completion requires `self-healing loop compliance: pass`, `self-healing loop assessment: pass` and remaining blockers `none`.

## User Intent Contract

- Literal request: refactor the project according to the named PRD.
- Practical goal: ship a genuinely usable Tauri/Rust local media subsystem, not a visually similar Vue shell over Electron or mocks.
- Success: both build channels expose working common media pages; typed Rust behavior and resource cleanup are verified; external gaps are honestly degraded.
- Forbidden interpretations: keeping Electron behind a renamed service, forwarding accepted events back to old renderer owners, simulating recording/TTS, adding send/moderation/AI behavior, or declaring completion from screenshots alone.
- Acceptable approach: reuse the completed Tauri foundation and live-data contracts, add narrow Rust ports/adapters/repositories and typed Vue stores/components, then delete the old chain.
- Verification/review checks: source allowlist scan, command/capability parity, event-order tests, failure/cleanup tests, responsive screenshots and dual installers.

## User Flow

1. Configure and preview captain effects; save makes Rust configuration immediately active.
2. Configure/test gift alerts; accepted gifts update Rust-owned totals and latest items.
3. Bind/preview/import/export gift sounds through dialogs; save row/global playback policy.
4. Configure TTS; accepted events enter one prioritized Rust queue; pause/continue/clear reflects status events.
5. Configure recording and start from a connected room; Rust launches FFmpeg, updates status and enforces disk rules.
6. F9/manual actions create highlights; stop/failure finalizes resources and refreshes file/highlight lists.
7. Preview/rename/delete/export through typed commands; cancel and failures preserve state and revoke temporary grants.

## Page And Module Design

- Routes: captain effects, gift reminders and gift sounds appear under `提醒配置`; TTS and recording have direct entries. Exact route paths are architecture-owned and must not create duplicate pages.
- Effect window is a restricted render-only Tauri window. It listens for normalized effect payloads and exposes no commands.
- Each configuration page owns one local draft. Pinia owns server snapshots and request state; Rust/SQLite owns persistence and business truth.
- Recording history/highlights share the recording page; settings is one dialog. TTS is removed from the generic settings form and becomes its PRD page.
- The later shell may reposition recording status, but may not change this module's state/action semantics.

## UI / UX Design Contract

- Preserve the approved journey, information hierarchy, page bands, effect surface, table structure and primary/secondary/destructive action hierarchy from `design/page-design.md`.
- Use quiet operational density, neutral surfaces, thin dividers and distinct semantic colors. No nested cards, landing-page composition or decorative gradients/orbs.
- Loading/empty/error/degraded/pending/dirty states keep stable geometry. Permission and external-resource failures expose the source-backed recovery path.
- Persisted writes are pessimistic. Duplicate preview/save/start/stop operations are rejected or disabled by their owner.
- Toolbars wrap at 1024px; form grids collapse to one column; tables scroll horizontally without hiding actions; dialogs are viewport-bounded.
- Keyboard/focus follows the approved page order, dialogs restore trigger focus, icon controls have names/tooltips, and state meaning is never color-only.
- Live media events do not steal focus or create per-event announcements/toast storms.

## Frontend Styling Constraints

- Authored styling uses Tailwind CSS-style utility classes only.
- No new scoped CSS, CSS modules, Sass/Less, inline style objects or semantic class names.
- Class values remain inline-reviewable within formatter width; long values cannot be hidden in constants/maps/computed/helpers/imports.
- Conditional bindings contain only small state toggles. Split a valid named region or reduce styling when a base class list becomes too long.
- Canvas/animation parameters may configure rendering but must not imperatively rewrite DOM layout styles.

## Field Display Semantics And Interaction Visual-State Contracts

| Field group | Business/display contract | Empty/invalid | Interaction/visual contract | Evidence |
| --- | --- | --- | --- | --- |
| Username/captain level/action | local effect or gift actor; reuse normalized identity name and `总督/提督/舰长`, `首开/续费` | invalid accepted-event DTO is rejected, not fabricated | username in gift detail is readable but not a new navigation target; effect text is default-emphasized | DTO/renderer/detail tests |
| Gift name/count/value/time | accepted gift identity, integer quantity, minor-unit CNY as `¥#,##0.00`, event time `HH:mm:ss` | invalid quantities/value rejected; no generic fallback | gift row is a semantic button with default affordance, hover/focus/active; no visited state | adapter/table/detail tests |
| Alert/current totals | current-session CNY, unique donor count and gift count; tabular numerals | zero displays `¥0.00`/`0`, unavailable snapshot is degraded not zero | informational only; no hover emphasis | snapshot/formatter tests |
| Sound filename/volume/enabled | authorized local binding display name; volume 0.0-1.0 with one decimal; binary playback state | no binding uses empty state; missing file is explicit red text+icon | filename not a raw-path link; replace/preview focus states; disabled remains readable | grant/row tests |
| TTS engine/voice/rate/volume | selected synthesis source/capability, voice label, provider-supported rate, volume 0.0-1.0 | unavailable option is disabled with reason; malformed capability is rejected | selected state plus text; no generic value fallback | capability/adapter/form tests |
| TTS current/count/paused | currently speaking text, nonnegative pending count and queue control state | no current item shows approved empty state; impossible negative count rejected | current item strongest; pause/continue has text+icon/focus/disabled | queue/event tests |
| Recording status | `idle/starting/recording/stopping/finalizing/error`; source-owned transition meaning | unknown status rejected | text+icon+color; active recording is persistent business emphasis, not hover-only | transition/component tests |
| Recording duration/size/disk | elapsed `HH:mm:ss`, bytes using binary units, disk remaining/quota using same unit/precision | pending unknown is `--`; invalid negative rejected | tabular figures; waterline warning persistent text+icon | formatter/status tests |
| Recording filename | authorized output filename including MP4/FLV extension | missing path/file is explicit unavailable; never invent filename | not a raw path; row preview action provides affordance | repository/table tests |
| Recording start/end/highlight time | session timestamp and clip position; session time uses local date-time, duration/position uses `HH:mm:ss` | absent end while active is expected; otherwise invalid | informational except row actions; no visited state | mapper/row tests |
| Highlight reason | only `弹幕激增` or `手动标记` in this module | unknown enum rejected | text tag plus non-color distinction | enum/row tests |
| Path display | informational path selected by system dialog; never authorization | no selection is explicit; cancellation unchanged | readonly, truncated with accessible full value; not editable/clickable | path/grant/UI tests |

Generic date/amount/path formatters are forbidden unless their semantics match this table exactly.

## Function-Complete Behavior Breakdown

### IM-01 Captain Effects

- Load one `CaptainEffectConfig` draft and a preset catalog of at least ten entries. Preset labels/assets come from trusted Rust/config resources; the page does not author missing names.
- Fields: preset, duration enum 3/5/8/10 seconds, position enum, two `#RRGGBB` colors, three optional sound grants/bindings, volume 0-1, combo enabled and combo window 5/10/30 seconds.
- Preview uses the PRD sample identity, opens/reuses the restricted window and queues exactly one local preview. Sound preview accepts only a persisted binding or `giftSoundFile` grant.
- Save validates enums/colors/volume/grants and persists atomically. Reset requires confirmation and replaces config with Rust defaults.
- Runtime captain events are queued in accepted-event order; one item renders at a time, default five seconds, then fades. Window failure uses the same queue and payload in one main-window overlay.

### IM-02 Gift Reminder

- Load/save fields: enabled, nonnegative high-value threshold in minor units, at least one of system notification/local popup/sound when enabled, combo enabled/window, popup position and duration.
- `测试提醒` uses a local test payload and exercises enabled channels; it never writes a gift event/statistic. Notification denial falls back to popup; sound failure is logged and does not fail the remaining channels.
- Runtime snapshot contains total value, unique donor count, gift count and latest accepted 20 items. Gift identity uses `giftId` while `giftName` remains the display label. Duplicate event IDs are idempotent; blacklisted/filtered items never enter the snapshot.
- Row selection opens detail with only user, gift, quantity, value and time.

### IM-03 Gift Sounds

- Load known gift rows plus persisted binding state. Each row is keyed by the accepted event's `giftId`, never by display name. Search trims only for comparison and never mutates stored names; bound-only is a local derived filter.
- Replace selects MP3/WAV via system dialog, validates format/readability in Rust and persists the binding pessimistically. Remove requires confirmation and deletes one binding.
- Row volume and enabled plus global volume are form-draft values; save validates all values and persists atomically. Reset requires confirmation and restores approved defaults.
- Preview plays once through Rust using the row binding or temporary grant; no browser `Audio`, Blob or raw path is used.
- Import/export use purpose-specific grants. Import validates the whole structured document and rolls back on any invalid entry; export writes via Rust. The technical schema preserves only gift ID/name, file reference portability metadata, volume and enabled state; it cannot add product fields.
- Runtime queue is ordered, handles at least 10 items/second and drops the oldest low-priority item on overflow. The UI does not own or edit the queue.

### IM-04 TTS

- Load/save fields: enabled; engine `system|cloud`; engine-provided voice; provider-supported rate; volume 0-1; event selections danmaku/gift/captain/entry; identity selections admin/captain/vip/ordinary; danmaku/gift/captain templates.
- Template whitespace is preserved because spoken phrasing is intentional. A whitespace-only required template is invalid. Validation runs on blur and submit; only the exact PRD placeholder whitelist is accepted per template. Newlines/control characters are rejected. Invalid save reports the field and Rust retains/uses its default template.
- Entry speech uses the confirmed concise system-copy authority and normalized username; it is not an editable fourth template.
- `tts_test` speaks a deterministic local sample derived from the selected valid configuration and writes no live event/log row.
- Accepted events are prioritized captain > gift > admin > ordinary with stable FIFO inside a priority. Pause retains current/pending state; continue resumes; clear removes pending items without deleting event logs.
- System engine failure selects cloud; cloud failure selects the configured local fallback and reports once. When no real fallback is available, status is unavailable and no speech is simulated.
- Overflow drops low-priority pending items. Current item, pending count, paused flag and degraded engine state come from Rust events/snapshots.

### IM-05 Recording And Highlights

- Load/save config: MP4/FLV, original/high/standard, recording-directory grant, segmentation enabled and duration represented as `HH:mm:ss`/canonical seconds, auto-record, danmaku heat threshold measured over the existing rolling 10-second product baseline, fixed 30 seconds before+after, quota default 10GB, waterline default 5GB, built-in/local FFmpeg and F9/custom shortcut.
- Filename rename input is a filename stem, not a path. Preserve internal/consecutive spaces; trim leading/trailing whitespace for validation/submission; reject empty, control characters, `/`, `\\`, `:` and platform-reserved results; preserve the original extension. Validate on blur/submit.
- Start validates room/live state, config, grant, FFmpeg and waterline, then transitions idle -> starting -> recording within the three-second target or structured failure. Duplicate start is rejected.
- Stop transitions recording -> stopping -> finalizing -> idle/error, waits for process exit/file flush/database write and is idempotent for the active generation. No pause/resume exists.
- Rust maintains recent 60-second memory buffer plus quota-bound disk buffer. Danmaku surge and manual F9/page action create before/after 30-second clips. At quota, oldest buffer is removed; below waterline, recording stops and warns.
- Stream start failure retries exactly three times. App close/error cancels tasks, terminates the owned Sidecar, finalizes/reports partial output consistently and releases hotkey/grants.
- Recording list actions: secure built-in preview, rename, confirmed delete. Highlight actions: secure preview, save-dialog export, confirmed delete. No browser download/raw file URL/open-folder substitution.
- Preview uses an opaque Rust-owned preview session/custom protocol; Vue never receives an authoritative local path.

## Expert Frontend Engineering Constraints

- One owner per mutable value: Rust/SQLite for media truth; Pinia for immutable snapshots/request state; page-local draft for unsaved fields; component local state only for focus/dialog/selection.
- DTO -> adapter -> view model/form snapshot -> command payload. Templates/components cannot parse raw status/path/provider values.
- Root media runtime registers each typed listener once, handles late registration cleanup and does not duplicate the live-data event bus.
- Commands use per-operation pending/dedupe. Queries use latest-wins generations; stale results cannot replace a newer page context.
- Long lists are bounded: gifts 20; media history uses pagination/bounded query. Queue/status events update narrow snapshots rather than whole-page deep watches.
- Dialog cancellation, permission denial, unavailable resources, partial finalization and app shutdown have deterministic cleanup/feedback.
- Tests cover pure adapters/validators, store concurrency, component success/failure/loading/disabled/focus and Rust port integration.

## Frontend Architecture Design Routing Inputs

- `architecture-design_required: true`.
- Architecture may not change IM-01..IM-08 behavior, product fields, defaults, queue priorities, fallback order, path/security semantics, recording transitions or acceptance thresholds.
- It must finalize business capability owners, Rust state topology, ports/repositories, command/event DTO maps, listener owner, effect window bootstrap, Sidecar/hotkey cleanup, preview transport, page/store/adapter dependency rules, complexity budgets and legacy removal triggers.
- It must decide reuse candidates listed below and prove no cross-module cycle or duplicate event/queue owner.
- Code facts: existing path grants/schema can be extended; current Electron media chains, duplicate stores/settings and disconnected event paths must be removed; plugin/shell neighbors remain bounded.
- Any need to change user-visible behavior, validation, API meaning or acceptance routes back to spec before architecture approval.

## Production Code Quality Constraints

- Type first: extend authoritative Rust serde DTOs and mirror them in TypeScript maps before handlers/pages. Reuse `RoomId`, `UserUid`, `EventId`, live event types and `PathGrantId`; add branded `GiftId`, `RecordingId`, `HighlightId`, `PreviewSessionId` where confusion is plausible.
- No `any`, broad optional mega-DTO, unchecked cast or template fallback for invalid native data.
- Fail fast on invalid enums, values, grants, files, states, timeouts, unavailable ports and impossible transitions; structured `DesktopError` remains the only frontend failure boundary.
- `null` means explicit absence (current item/end time/binding); omitted fields are not silently equated with null unless the contract says so.
- Pure functions own validation/mapping/formatting/state derivation; lifecycle stateful classes are allowed in Rust/renderer only where process/window/render state requires encapsulation.
- Use precise names (`isRecording`, `hasBinding`, `canStartRecording`, `validateTemplate`, `formatElapsedDuration`, `handleStart`). Constants live in the narrow domain owner; class strings are not constants.
- Optimize for correctness/readability first. Queue bounds, pagination, narrow events and render-frame control are approved performance mechanisms; speculative caches/debounces are forbidden.

## Human Review Readiness Constraints

- Follow existing typed desktop facade, Pinia snapshot, feature directory, Rust layered-module and focused-test conventions established by prior modules.
- Allowed diff: Rust media domain/application/infrastructure/IPC/config/capabilities/tests; desktop contracts/adapters/runtime; named media stores/pages/components/routes; owned Electron/preload/media services/modules/settings/config/tests; required package/build declarations.
- Not allowed: PRD edits, membership/plugin/update behavior, final shell redesign, unrelated logger cleanup or broad formatting.
- Every changed hunk maps to an IM task, migration cleanup, generated declaration adaptation or test. Large deletions need before/after caller evidence.
- Remove debug code, stale comments, temporary compatibility, unused exports, duplicate DTOs, invalid tests and generated references.
- Handoff evidence includes command/event surface tests, capability tests, Rust/Vitest/component tests, strict type/lint/build, dual installers, runtime screenshots and explicit external-resource gaps.

## Functional-Programming Constraints

- Pure: config validation, template tokenization, priority/overflow selection, combo grouping, gift statistics reduction, DTO adapters, view-model mapping, formatters, recording transition checks, disk threshold calculation and payload builders.
- Immutable: accepted event inputs, Rust DTOs, Pinia snapshots, component props and adapter inputs. Reducers return new snapshots or mutate only a uniquely owned Rust state under its lock.
- Effects exist only in Rust ports/repositories/command/event/window/process/audio/file/hotkey owners, desktop transport, Pinia actions and lifecycle registration.
- Semantic normalization belongs in adapters/`fromDto`; templates/computed/watchers cannot repair malformed DTOs.
- No functional utility library, currying framework or opaque reducer pipeline is introduced.

## Architecture Reuse And Shared Ownership Constraints

| Candidate | Facts/Anti-DRY input | Classification | Decision required from architecture |
| --- | --- | --- | --- |
| Desktop transport/error | 20+ existing commands, one correct owner | infrastructure | reuse existing facade/maps/error normalization |
| Path grants/dialog | existing recording/gift purpose support; new save purposes | infrastructure/security | extend narrow purposes and revoke ownership, no media-specific duplicate store |
| Accepted-event routing | four existing Rust event types; media has multiple consumers | domain/event coordination | one Rust observer/fan-out owner; no renderer projection |
| Audio queue | gift sound/effect/TTS share device output but different priority/policy | infrastructure mechanism + domain policies | share output mechanism only; keep gift/TTS policy owners separate |
| Volume validation/format | 3+ same numeric rule, different labels | domain/config | one primitive validator/formatter if stable; UI fields remain local |
| File metadata/preview | recording/highlight/sound have different grants and playback | infrastructure with different policy | share safe file/preview primitives, not one mega media DTO/service |
| Recording status mapping | page and shell same semantics | UI/domain | one normalized status model/component API |
| Duplicate TTS/recording settings | two writable owners today | state ownership defect | one page/store owner; remove generic settings copies |
| Legacy renderer manager classes | single old callers, wrong side-effect owner | migration | delete, do not wrap/promote |

Promoted shared APIs require narrow typed inputs, correct dependency direction and JSDoc `@example`/`@see` for actual callers. Architecture must record all migrated callers/tests and behavior-equivalence checks.

## Locality And File-Extraction Constraints

- DTO adapters/contract maps are separate because they isolate the native boundary and have independent tests.
- Media runtime composable may be separate because it owns multi-listener lifecycle/cleanup and is called by the root once.
- Pure single-page search/filter/form helpers remain in the owning page unless independent adapter/validation testing or unreadable file pressure is demonstrated.
- Domain validators/constants live under the narrow media capability, not catch-all `utils.ts`/`constants.ts`.
- No one-function file is approved solely to shorten a page or prepare hypothetical reuse.

## Vue Component Extraction Constraints

- Enforce the approved page-design table: `CaptainEffectWindow`, `GiftDetailDrawer`, `GiftSoundTable`, `TtsQueuePanel`, normalized `RecordingStatusBand`, `RecordingList`, `HighlightList`, and `RecordingSettingsDialog` have explicit lifecycle/business/test reasons.
- Component props are normalized view fields, not raw Rust DTOs or a parent form object. Emits express user intent; no component imports a store/transport.
- No slots are currently required. Adding one needs a real variation point.
- Captain/gift/TTS forms, volume fields and toolbars remain inline; they are one-caller pass-through or different-semantics candidates.
- Verification must include caller counts, focus/lifecycle behavior, class reviewability and removal of direct store imports from retained table children.

## Design Constraints

- Effect capability has zero Command permissions. Main capability registers only approved common media commands in both build channels.
- Effect queue, audio queues, TTS queue, recording generation and Sidecar are bounded and have one cancellation/cleanup owner.
- SQLite uses existing explicit tables/migrations; schema changes are additive and transactional. Money/path/status/time meanings cannot be inferred from old Electron types.
- Minimum view is 1024x680, default 1280x800. Effect position uses logical coordinates and display-change events.
- Exact performance targets: effect <1s and >30FPS; audio alert <500ms and >=10/s; TTS <2s and CPU <5%; recording start <3s, A/V sync <1s, highlight <5s and CPU increment <15%.

## Project Bootstrap And Scaffold Decision

- Overall scaffold sensitivity was resolved by `platform-foundation`: reuse the existing Vue/Vite package and initialized Tauri v2 Rust backend created from the official CLI path.
- This module creates no app/package/route-level build surface from scratch. It extends that approved scaffold; no new starter or alternate frontend is allowed.
- Approved deviations are media modules, a restricted effect-window entry/config and Sidecar resources required by the PRD. Electron is not a scaffold candidate because it is the migration source being removed.

## Change Axes And Pattern Decision

- Signals: provider/source selection, DTO adaptation, ordered safety-to-media pipeline, priority queues, fallback chain, stateful recording transitions, event fan-out and resource side-effect coordination.
- Depth: Level 2 for reusing the typed facade/ports-adapters/observer/state patterns; Level 1 for local typed maps/discriminated state tables. No Level 3 frontend pattern layer is approved before architecture design.
- Decision: `reuse existing pattern` plus `adapt lightweight pattern`.
- Syntax: Rust traits/enums/state structs and Tokio tasks; TypeScript discriminated unions/typed records/adapter functions; Pinia actions; one lifecycle composable; Vue props/emits SFC boundaries.
- Adapter/facade is required for native contract isolation; Observer/Pub-Sub is required for accepted-event media fan-out with lifecycle ownership; explicit state tables/unions constrain recording/TTS states; provider maps/ports isolate TTS/FFmpeg selection and fallback.
- Rejected: renderer manager/factory class hierarchy, generic media service, shared mega queue DTO, restored compatibility event bus and speculative plugin strategy. They add hidden ownership or merge different policies.
- Overbuilt signal: one-method wrappers, a pattern layer with one variant/caller, broad optional interfaces or more indirection than the direct command/store flow.

## Code Context And Impact Assumptions

- Source of truth: `artifacts/code-context.md` interaction-media refresh.
- Primary old chain: Vue pages/settings/status -> stores/services/engines -> generic Electron bridge -> Node recording/gift/TTS modules -> browser/Node side effects.
- Target: accepted Rust live event -> Rust media application/domain -> repositories/ports -> typed Tauri events -> Pinia snapshots -> pages/render-only effect window.
- Tests under root that reference Node recording paths are migration/adaptation targets, not authority for gift/captain highlight behavior.
- Post-change scans must prove no owned Electron channel, raw-path browser operation, duplicate settings owner or dead renderer media class remains.

## API And Data Contracts

### Contract Authority And Units

- Rust serde DTOs and command/event registries are authoritative; TypeScript mirrors exact camelCase names through the existing desktop contract maps.
- Reuse live-data `RoomId`, `UserUid`, `EventId`, `AudienceIdentity`, `CaptainEvent` and `EnterEvent`. Extend the shared `GiftEvent`/Rust gift variant with required `giftId`; this is an additive source-backed contract field required by PRD binding semantics.
- Money is integer minor units; timestamps are epoch milliseconds; durations are integer milliseconds/seconds as explicitly named; bytes are nonnegative integers; volume is finite 0-1.
- New path purposes: `giftSoundImport`, `giftSoundExport`, `highlightExport`, `localFfmpegExecutable`; reuse `giftSoundFile` and `recordingDirectory`.

### Core DTOs

- `CaptainEffectConfig`: presetId, durationSeconds, position, primaryColor, secondaryColor, firstSound/renewalSound/comboSound nullable binding summaries, volume, comboEnabled, comboWindowSeconds.
- `GiftNotifyConfig`: isEnabled, highValueThresholdMinor, methods `{systemNotification, localPopup, sound}`, comboEnabled, comboWindowSeconds, popupPosition, popupDurationSeconds.
- `GiftNotifySnapshot`: config, totalValueMinor, donorCount, giftCount, latestGifts (max 20), updatedAt.
- `GiftSoundBinding`: giftId, giftName, soundFileName nullable, volume, isEnabled, isMissing; no raw path.
- `GiftSoundSnapshot`: bindings, globalVolume, queueCapacityPerSecond (10 or greater), updatedAt.
- `TtsConfig`: isEnabled, selectedEngine, selectedVoiceId, rate, volume, eventKinds, identityKinds, templates.
- `TtsStatus`: state `idle|speaking|paused|degraded|unavailable`, currentText nullable, pendingCount, activeEngine, fallbackReasonCode nullable.
- `RecordingConfig`: format, quality, directory grant/display, segmentation, autoRecord, danmakuThreshold, highlightBeforeSeconds=30, highlightAfterSeconds=30, quotaBytes=10GB default, waterlineBytes=5GB default, ffmpegSource, shortcut.
- `RecordingStatus`: state, sessionId nullable, roomId nullable, elapsedSeconds, fileSizeBytes, diskRemainingBytes, highlightCount, lastErrorCode nullable.
- `RecordingSummary`: id, fileName, format, sizeBytes, durationSeconds, startedAt, endedAt nullable, status; no raw path.
- `HighlightSummary`: id, recordingId, positionSeconds, triggerType `danmakuSurge|manual`, clipDurationSeconds, createdAt, isClipAvailable.

### Command Surface

| Commands | Purpose/result |
| --- | --- |
| `captain_effect_get/save/preview` | load/persist config and queue one local preview |
| `gift_notify_get/save/test` | load/persist config+snapshot and execute one local test |
| `gift_sound_list/set/remove/save/import/export` | query bindings, validate/persist file, remove, persist row/global values and transactional I/O |
| `sound_preview` | preview persisted captain/gift binding or temporary authorized grant |
| `tts_get/save/set_enabled/test/pause/clear` | query config/status, persist, toggle, test, set paused boolean and clear pending queue |
| `recording_get_config/save_config/set_ffmpeg_source/status/start/stop/list/preview/rename/delete` | configure, control, query and manage recordings |
| `highlight_list/mark/preview/export/delete` | query, create, preview, grant-export and delete highlights |

All commands return typed success values or structured `DesktopError`; commands taking IDs/grants validate kind/ownership. `preview` returns an opaque preview session, never an authoritative path.

### Event Surface

- `captain_effect_trigger`: render-only effect payload to `effect-*`; event IDs are deduplicated.
- `gift_alert_trigger`: normalized local popup payload; system notification/sound remain Rust effects.
- `gift_notify_updated`: current-session totals/latest snapshot.
- `tts_status_changed`: queue/engine/status snapshot.
- `recording_status_changed`: recording state/status snapshot.
- `recording_completed`: completed recording summary.
- `highlight_created`: highlight summary.

Listener cleanup and late-registration semantics match the existing desktop transport contract. The effect window can listen only; it cannot invoke commands.

## Frontend / Server Responsibility Boundary

- Vue: routes, page-local drafts, validation display, search/filter view state, dialogs/focus, normalized tables/status and render-only effect content.
- Rust: authoritative validation, accepted-event routing, combo/stat reducers, queues/priorities/fallback, SQLite, dialogs/grants, notifications/audio/TTS, Sidecar/buffers/files/quota, preview sessions, F9, windows and cleanup.
- Shared: serde/TS DTO mirrors, enums, units, structured errors, command/event names and path purposes.
- External: real cloud TTS and bundled LGPL FFmpeg artifact. Missing inputs do not block local FFmpeg selection, typed ports, pages, tests or unavailable states; they block claims of complete production integration/distribution.
- Adapters reject malformed DTOs and produce immutable view models. No frontend field production, permission decision, persistence or queue transition substitutes for Rust.

## Context And Dependency Sources

- Governing TypeScript: `packages/bilibili-live-tool/tsconfig.eslint.json` extending strict `tsconfig.json`; bundler resolution, ES2020/DOM, `@/*`, noUnused and Vue generated declarations apply.
- Native declarations: Rust serde DTOs -> `src/renderer/platform/desktop/contracts.ts`; no protobuf/OpenAPI exists.
- Runtime dependencies: existing Tauri v2 dialog/event/window/path foundation, Tokio, SQLite, Vue 3, Pinia, Router, Element Plus and Tailwind utilities.
- External artifacts/contracts are tracked in `spec/clarifications.md`; execute cannot scan all ambient declarations or infer providers later.

## Edge Cases

- Duplicate or out-of-order events, queue overflow, combo expiry and window recreation.
- Missing/changed sound after binding, unsupported format, dialog cancellation and import rollback.
- Notification denial; system/cloud/local TTS availability combinations; invalid template or no selected content/identity.
- Start/stop races, app exit while starting/finalizing, FFmpeg crash/hang, stream retry exhaustion, partial file and repeated stop.
- Disk remaining crossing waterline, quota cleanup failure, directory grant revocation, filename collision and rename/delete while previewing.
- F9 conflict/re-registration, display removal/DPI change, effect renderer initialization failure and reduced motion.
- Empty/filtered histories, nullable end/current/binding values and invalid native DTOs.

## Acceptance Criteria

- `IM-A01`: captain event renders locally in <1s, >30FPS, audio sync <100ms, one at a time, correct 5s default, DPI/display reposition and no send path.
- `IM-A02`: effect config exposes only approved fields/options; save/reset/preview/fallback and missing-sound degradation pass.
- `IM-A03`: gift alert threshold/method/combo behavior, <500ms trigger, notification fallback, latest-20 and exact totals are verified from accepted events.
- `IM-A04`: gift bindings accept MP3/WAV only, use grants, show missing files, import transactionally, export via Rust and handle >=10/s with approved overflow.
- `IM-A05`: TTS priority is captain > gift > admin > ordinary, latency <2s, CPU increment <5%, template whitelist/fallback/pause/clear work and unavailable services never fake speech.
- `IM-A06`: recording start <3s, MP4/FLV and qualities/config work, A/V sync <1s, stop/finalize/process cleanup is deterministic.
- `IM-A07`: automatic danmaku-surge and manual F9/page highlights include 30s before/after and respond <5s; no unapproved gift/captain triggers remain.
- `IM-A08`: 10GB default quota, 5GB default waterline, oldest-buffer cleanup, low-space stop/warning and three stream retries pass.
- `IM-A09`: recording/highlight preview/rename/delete/export use opaque/granted paths; cancellation and failures leak no grants/files/processes.
- `IM-A10`: both free/member command registries/capabilities contain the same common media surface; effect capability contains zero commands.
- `IM-A11`: one root listener/runtime and one Rust owner exist; app close/navigation removes listeners, timers, frames, audio, window, hotkey and Sidecar resources.
- `IM-A12`: owned Electron/browser services, channels, direct paths, duplicate settings and invalid legacy tests are removed with no stale caller/import/mock.
- `IM-A13`: TypeScript/Rust contract tests, focused UI/store tests, Rust domain/integration tests, strict checks, responsive screenshots and dual installers pass.
- `IM-A14`: real cloud TTS/bundled FFmpeg commands are reported not run/unavailable until artifacts arrive; verification does not misrepresent them.

## Human Review And Handoff

- Changelog maps each changed file/symbol to plan task and source item, including why deletions are dependency closure.
- Verification reports fake-port vs real external evidence distinctly and includes cleanup/failure paths, not only happy-path UI.
- Review explicitly judges source grounding, user intent, architecture reuse, field semantics, component extraction, Tailwind constraints and self-healing.
- Handoff identifies generated installers, development URL, missing external artifacts and any later shell/plugin integration obligations.

## Risks

- Windows audio/TTS/FFmpeg behavior and process termination differ from fake ports; integration tests need deterministic adapters plus at least one local unavailable-path runtime check.
- Secure built-in preview without raw paths requires careful Tauri protocol/session design.
- The preset count requirement lacks final asset content; architecture may provide a trusted catalog contract, but release cannot claim ten finished visual assets without supplied/configured content.
- Current root tests encode unapproved recording triggers and invalid paths; adapting them must not preserve old behavior accidentally.
- Large media dependencies can worsen existing bundle size; renderer assets and Rust Sidecar distribution must be measured separately.
