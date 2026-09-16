# Interaction Media Architecture Design

## Delivery Unit Identifier

- request: `tauri-rust-refactor`
- module: `interaction-media`
- sequence: 3 of 6
- stage: `architecture-design`
- approved spec: `spec/spec.md`
- architecture result: ready for executable planning

## Architecture Objective

Replace the disconnected Electron/browser media paths with one accepted-event ingress and four explicit Rust lifecycle owners: captain effects, gift media, TTS and recording. Preserve the typed Tauri boundary, keep Vue as a snapshot/draft/render layer, isolate unavailable external adapters, and delete all owned legacy paths once contract and behavior evidence passes.

## Architecture Scope And Triggers

- Cross-language Command/Event expansion and additive `GiftEvent.giftId` contract.
- Event fan-out after live-data safety acceptance.
- Stateful audio/TTS queues, effect windows, FFmpeg process, buffers, preview sessions and global shortcut.
- SQLite/file/dialog/notification/audio/TTS/Sidecar side effects.
- Five UI surfaces plus a separate render-only window and shared recording status.
- Duplicated stores/settings/services and unsafe browser/Electron file ownership.

These signals require architecture design; page-local direct implementation cannot provide safe ownership or cleanup.

## Upstream Inputs And Assumptions

- Behavior authority is the framework-approved spec and PRD 9.6-9.8/9.12-9.13; architecture cannot change it.
- `design/page-design.md` fixes page grouping, action hierarchy, feedback states, component candidates and responsive/accessibility behavior.
- Existing Rust live-data domain models, `PathGrantStore`, SQLite bootstrap, desktop transport and free/member registry are reusable facts.
- Cloud TTS and bundled FFmpeg remain unavailable ports until external inputs in `spec/clarifications.md` arrive.
- The code graph fallback and scoped media caller closure in root `artifacts/code-context.md` are sufficient; Vue compilation/builds will provide post-change structural proof.

## Scaffold Alignment And Bootstrap Boundaries

- Reuse the approved Vue/Vite package and existing Tauri v2 backend. No new app, package, starter or alternate renderer is created.
- The effect surface reuses the same Vite entry and dynamically selects a minimal root from `?surface=effect`; it is not a second hand-written scaffold or HTML entry.
- Tauri deviations are limited to media modules, a restricted window root, custom preview protocol and declared Sidecar resource boundary.
- Package scripts, Router, Pinia, Element Plus, Tailwind and the desktop facade remain the bootstrap baseline. Electron files are migration sources, not scaffold output to retain.

## Expert Architecture Decision Records

### ADR-IM-01 Accepted Event Ingress

- Problem: media consumers need every accepted live event, but restoring a renderer projection would duplicate authority and filtered events must never leak.
- Choice/owner: add an async `AcceptedLiveEventConsumer` port owned by `application/live_data`; inject one `MediaEventIngress` implementation backed by `MediaState`.
- Smallest fit: one typed trait and one production consumer; `LiveDataState.consume_event` calls it after persistence/aggregation and before/independently of UI publication. Ingress performs only bounded in-memory reduction/enqueue and isolates subsystem failure.
- Alternatives: renderer listeners forwarding events (rejected duplicate path); Tauri sink calling media directly (rejected IPC-to-application coupling); generic event bus registry (rejected speculative indirection).
- Direction: live-data application -> consumer trait <- media application. Neither imports IPC/infrastructure.
- State/side effects: media state becomes the only downstream queue/stat owner; UI Tauri sink remains presentation delivery only.
- Cleanup/evidence: default no-op consumer for focused live tests; media ingress integration proves filtered events never arrive, accepted order/idempotency and media faults do not disconnect the room.

### ADR-IM-02 Four Media Runtime Owners

- Problem: effects, gift alerts/sounds, TTS and recording have different queues, persistence, fallbacks and cleanup.
- Choice: `MediaState` is a narrow application facade over `EffectRuntime`, `GiftMediaRuntime`, `TtsRuntime` and `RecordingRuntime`.
- Smallest fit: each runtime has one mutex/state and explicit cancellation handles; facade routes commands/events without a generic manager or mega state.
- Alternatives: one `MediaManager` (rejected mixed state/locks); one runtime per page action (rejected duplicate resources); separate microservice/Sidecar per feature (unsupported).
- Direction/state: IPC -> facade -> one runtime -> ports; each runtime owns its writable business state.
- Extension seam: only current provider/process/window/file ports; no plugin registry.
- Cleanup/evidence: runtime shutdown test proves queue close, task join, audio stop, window close, hotkey unregister and process termination.

### ADR-IM-03 Rust Ports And Concrete Adapters

- Problem: notifications, audio, TTS, FFmpeg, window/display, files, preview and hotkeys vary by environment and need deterministic tests.
- Choice: narrow traits `MediaRepository`, `MediaFilePort`, `AudioOutputPort`, `SpeechEnginePort`, `RecorderProcessPort`, `NotificationPort`, `EffectWindowPort`, `HotkeyPort`, `PreviewPort`.
- Smallest fit: traits correspond to real I/O boundaries and have fake/unavailable implementations; no one-method class layer on the frontend.
- Selection: TTS engine ports are a typed record keyed by `system|cloud|localFallback`; recorder source is direct enum matching inside `RecordingRuntime`.
- Alternatives: generic media provider/factory (rejected broad semantics); commands executing OS calls directly (rejected testability/cleanup); shared environment globals (rejected hidden state).
- Direction: application owns traits; infrastructure implements them; IPC only maps errors/DTOs.
- Evidence: fake-port tests for state/fallback and one unavailable concrete-adapter runtime test per external dependency.

### ADR-IM-04 Persistence And Path Authority

- Problem: current pages expose raw paths/browser writes; media configs span typed tables and general settings.
- Choice: `SqliteMediaRepository` owns `t_gift_sound`, `t_recording`, `t_highlight`, `t_tts_config`; strongly decoded JSON values in `t_app_setting` own captain/gift-notify/global-sound/recording config. Add migration `0002_interaction_media.sql` for required columns/indexes only.
- Grants: extend existing `PathPurpose`; file adapters resolve grants inside Rust at use time and never serialize authoritative paths.
- Alternatives: new tables for every config (rejected outside explicit PRD schema); continue Electron store (rejected); path strings in DTO (forbidden).
- Transactions: config/binding/import/rename/delete/finalize operations are atomic; file + DB operations have explicit rollback/compensation order.
- Evidence: migration/idempotency/repository round trips, wrong-purpose/revoked-grant tests, import rollback and file/DB failure tests.

### ADR-IM-05 Effect Render Surface

- Problem: effect content needs a restricted transparent window and a main-window fallback without duplicated queues or unapproved authoring controls.
- Choice: Rust `EffectRuntime` owns queue/window targeting; `CaptainEffectSurface.vue` is the single normalized renderer used by effect root and fallback overlay. A feature-local Canvas renderer owns RAF/resize/dispose; WebGL/render-mode/user template managers are removed.
- Bootstrap: `main.ts` reads the technical surface marker, dynamically imports either main `App` or `EffectWindowRoot`; effect root invokes no command and listens only to targeted events.
- Alternatives: route under full App shell (rejected visible chrome); separate Vite app (unnecessary scaffold); retain effect manager/WebGL classes (unapproved controls and bundle cost).
- State: Rust owns queue/current event; component owns only current animation frame/lifecycle.
- Evidence: target-window/fallback tests, canvas pixel/screenshot checks, 30FPS/latency measurement, DPI/reduced-motion and dispose tests.

### ADR-IM-06 Frontend Snapshot Architecture

- Problem: pages bypass stores, child tables import stores and settings duplicate writable configuration.
- Choice: feature stores `effects`, `gifts`, `tts`, `recording` own immutable native snapshots and request status. Pages own form drafts; adapters own DTO semantics; components receive narrow view props and emit intentions.
- Runtime: one `useMediaRuntime` main-root lifecycle owns six main-window listeners; effect root owns its single targeted listener separately.
- Alternatives: service singletons/classes (rejected hidden mutable state); components importing stores (rejected coupling); route-level duplicate listeners (rejected duplication/leaks).
- State/cleanup: latest-wins query generations; per-operation pending/dedupe; root teardown handles late async registrations.
- Evidence: store generation tests, listener idempotency/late cleanup tests and component tests with no transport/store imports.

### ADR-IM-07 Opaque Built-In Preview

- Problem: built-in media preview must not expose raw local paths to Vue.
- Choice: `PreviewPort` issues a random, expiring `PreviewSessionId`; a Tauri custom protocol resolves session -> canonical authorized file and streams range requests. DTO returns only session URL/token metadata.
- Alternatives: `convertFileSrc(rawPath)` (rejects path secrecy); browser Blob/download (forbidden); OS default player/open folder (not the built-in preview requirement).
- Owner: Rust preview registry owns authorization, expiry and active-read cleanup; preview Vue component owns player UI only.
- Evidence: unknown/expired/wrong-window token denial, range response, grant revocation, player close cleanup and no-path DTO scans.

### ADR-IM-08 Migration Without Compatibility Projection

- Problem: old Electron media owners are partially broken and keeping dual paths would allow divergence.
- Choice: implement/test Rust contracts and adapters first, switch each route/store to the new gateway, then delete its old frontend/main/preload closure in the same delivery. No runtime feature flag or Electron fallback.
- Rollback: source control reverts the delivery; source-backed permanent fallbacks remain Rust-owned and are not compatibility paths.
- Cleanup trigger: each new capability's contract/domain/integration/UI tests pass and exact caller scan shows only planned legacy references before deletion.
- Evidence: before/after symbol scans, TypeScript/build graph, Tauri dual builds and adapted tests.

## Business Capability And Ownership Map

| Capability | Business/rule owner | Writable state owner | UI owner | I/O owner |
| --- | --- | --- | --- | --- |
| Accepted media routing | `MediaEventIngress` | none beyond runtime enqueues | none | none |
| Captain effect | `EffectRuntime` + domain effect rules | Rust runtime/repository | captain page + effect surface | window/audio/file adapters |
| Gift alerts/sounds/stats | `GiftMediaRuntime` + gift reducers | Rust runtime/repository | reminder/sound pages | notification/audio/file adapters |
| TTS | `TtsRuntime` + priority/template rules | Rust runtime/repository | TTS page/queue panel | speech/audio adapters |
| Recording/highlights | `RecordingRuntime` + transition/disk/heat rules | Rust runtime/repository | recording page/components | FFmpeg/file/hotkey/preview adapters |
| Native contract | Rust serde + registry | none | TS platform mirror/adapters | Tauri transport |
| Page drafts | none | owning page only | owning page | store submit action |
| Main listener lifecycle | none | `useMediaRuntime` registration state | App bootstrap | desktop event transport |

Forbidden owners: templates/computed/watch for DTO normalization; child SFCs for requests/stores; IPC commands for domain rules; shared utilities for media policy; renderer code for files/processes/queue truth.

## Module Boundary Design

- `domain/media`: pure models, validation, reducers, queue ordering/overflow, combo grouping, recording transition/disk/heat decisions.
- `application/media`: four runtimes, accepted-event ingress, use-case methods, cancellation/generation coordination and I/O ports.
- `infrastructure/media`: SQLite, path/file codecs, audio/speech/notification, FFmpeg process, window/display, hotkey and preview adapters.
- `ipc/media`: thin commands, serde DTO mapping, Tauri event/window delivery and error conversion.
- `platform/desktop/contracts`: authoritative TypeScript mirror; transport stays generic and business-free.
- `modules/media`: pure adapters, validators, formatters and view types.
- `stores/media`: snapshots/request state/actions; no lifecycle registration.
- `views`/`components/media`: orchestration and presentation per page design.

## File And Directory Structure

```text
src-tauri/src/
  domain/media/{mod.rs,models.rs,rules.rs,tests.rs}
  application/media/{mod.rs,ports.rs,state.rs,effects.rs,gifts.rs,tts.rs,recording.rs,tests.rs}
  infrastructure/media/{mod.rs,repository.rs,files.rs,audio.rs,speech.rs,ffmpeg.rs,notification.rs,window.rs,hotkey.rs,preview.rs,tests.rs}
  ipc/{media.rs,media_contract_test.rs,dto.rs,error.rs}

src/renderer/
  platform/desktop/contracts/{base.ts,live-data.ts,media.ts,index.ts}
  modules/media/{adapters.ts,validation.ts,formatters.ts,*.test.ts}
  stores/media/{effects.ts,gifts.ts,tts.ts,recording.ts,*.test.ts}
  composables/{useMediaRuntime.ts,useMediaRuntime.test.ts}
  components/media/{CaptainEffectSurface.vue,GiftDetailDrawer.vue,GiftSoundTable.vue,TtsQueuePanel.vue,RecordingStatusBand.vue}
  views/captain-effect/{index.vue,EffectWindowRoot.vue}
  views/gift-reminder/index.vue
  views/gift-sound/index.vue
  views/tts/index.vue
  views/recording/{index.vue,components/RecordingList.vue,components/HighlightList.vue,components/RecordingSettingsDialog.vue}
```

The current single desktop contract file may be split behind the unchanged `platform/desktop` barrel because 34 new commands would otherwise create one mixed contract owner. Generated declarations and every current import migrate atomically.

## Code Relationship And Dependency Direction

```text
accepted LiveEvent
  -> AcceptedLiveEventConsumer
  -> MediaState facade
  -> Effect/Gift/Tts/Recording runtimes
  -> application ports
  -> infrastructure adapters

Vue page -> media store -> desktop facade -> Tauri media command
Tauri media event -> root media runtime -> media store snapshot -> page/component
effect-targeted event -> EffectWindowRoot -> CaptainEffectSurface
```

No reverse dependency from Rust domain/application to Tauri/SQLite/OS, from platform to stores/pages, from components to stores/platform, or from shared contracts to feature-private view models.

## State Topology And Data Lifecycle

| State | Writable owner | Derived state | Async/concurrency |
| --- | --- | --- | --- |
| effect config/catalog/current queue | Rust repository/`EffectRuntime` | selected preset view and visual frame | single FIFO actor, cancellation on shutdown |
| gift config/bindings/stats/latest | Rust repository/`GiftMediaRuntime` | search/bound filter/totals display | event-id dedupe; atomic mutations; bounded audio actor |
| TTS config/queue/status | Rust repository/`TtsRuntime` | button state/current label | priority FIFO, one speaker task, pause/clear generation |
| recording config/session/history | Rust repository/`RecordingRuntime` | `canStart`, formatted metrics | one session generation, idempotent stop, joined process/tasks |
| preview sessions | Rust `PreviewRegistry` | player-ready URL | random token, expiry, close/revoke cleanup |
| store snapshots | one Pinia store per capability | filtered/display lists | latest-wins queries; operation-specific pending/error |
| form drafts/dialog/focus | owning page/component | dirty/valid state | reset from confirmed snapshot only |

Data flow is `Rust domain model -> serde DTO -> TS contract validation -> immutable view model -> page draft or component props -> typed payload -> Rust validation/use case`. `null`/enum/unit/path semantics normalize only at DTO/adapters; templates never repair them.

## Dependency Rule Matrix

| From | Allowed | Forbidden | Public API |
| --- | --- | --- | --- |
| Rust domain/media | domain validation/types | application, infrastructure, IPC/Tauri | pure models/rules |
| Rust application/media | domain + owned ports | infrastructure, IPC/Tauri/SQLite | `MediaState`, ingress, port traits |
| Rust infrastructure/media | application ports + domain + DB/path primitives | IPC/UI policy | adapter constructors |
| Rust IPC/media | application facade + DTO/error mapper | repositories/domain policy | 34 commands, 7 events |
| TS platform | contract types + Tauri SDK in transport only | stores/pages/view policy | invoke/listen/maps/barrel |
| media adapters | platform DTOs + feature view types | stores/pages/transport effects | pure `fromDto`/payload builders |
| stores/media | platform facade + adapters | Vue pages/components/root lifecycle | snapshot/action API |
| page/container | stores + feature components | Tauri direct import/raw DTO parsing | route/page behavior |
| presentational component | normalized props/icons | stores/platform/router/business normalization | props/emits |

Cycle risks: contract split must remain leaf-like; stores cannot be imported by components; live-data imports only the consumer trait it owns, while media implements it. Compile/import scans are required.

## Extension Seams And Rejected Future-Proofing

- Real seams: `SpeechEnginePort` for system/cloud/local fallback; `RecorderProcessPort` for built-in/local FFmpeg; target-aware `EffectWindowPort`; custom preview protocol; accepted-event consumer.
- Provider selection uses typed enum maps/direct match, not factories or registries. Add a provider only when source/config supplies a real variant.
- Rejected: generic `MediaManager`, plugin media registry, arbitrary render strategy registry, queue framework, event-bus package, DI container, abstract base classes and compatibility facade.
- Remove/simplify a seam if it has no concrete unavailable/fake/production adapter or does not isolate an actual external dependency.

## Complexity Budget

- New production files: at most 45 (<=23 Rust, <=22 frontend); tests may mirror focused owners. Exceeding requires consolidation by capability, not catch-all files.
- New media commands: exactly/at most 34 from the approved spec table; new events at most 7. Any additional public surface requires spec rollback.
- Indirection: page -> store -> desktop facade -> IPC -> application runtime -> port is the maximum effect path; pure adapter steps do not add runtime ownership.
- One writable owner per state; one main listener per event plus one targeted effect listener; no route-level duplicate listeners.
- Functions remain single-purpose; no mixed orchestration function >150 lines. Actors/runtimes expose narrow use-case methods.
- Reactive fan-out: six main media event subscriptions maximum; latest gift list 20; history queries bounded/paginated; no deep watcher or local polling for native status.
- Out of scope: membership/plugin/update/logger/global shell restructuring.

## Migration, Compatibility, And Cleanup Design

1. Add contract tests and `giftId`; adapt existing live-data tests/callers.
2. Add pure media models/rules and repository/file/port tests.
3. Create runtimes with fake/unavailable adapters and accepted-event integration.
4. Register commands/events/capabilities identically in free/member; preserve effect zero-command capability.
5. Add TS adapters/stores/runtime and migrate routes/pages/components one capability at a time.
6. Switch main bootstrap/event ownership, then delete old renderer services/engines/effect managers, Electron modules/channels, duplicate settings/config fields and invalid tests.

No production dual path survives verification. Permanent source-backed fallbacks have explicit owners: main overlay (`EffectRuntime`), TTS fallback (`TtsRuntime`), local FFmpeg (`RecordingRuntime`). Rollback is source-control restoration of the whole delivery, not runtime Electron fallback.

## Locality And File-Extraction Decisions

- Separate Rust runtime files: each owns a distinct async lifecycle and focused fake-port test surface; a single `state.rs` would mix locks/processes/queues.
- `MediaEventIngress` stays with application `state.rs` unless its independent accepted-event tests make a small `ingress.rs` clearer; no one-function file.
- TS DTO adapters/validation/formatters are separate feature files because they have multiple pages/stores and deterministic tests.
- Page-only search/filter/form conversion stays inside its page. No `views/*/utils.ts`; the old single-use `recording-context.ts` is folded into the recording page/store adapter unless tests prove a domain adapter boundary.
- Component SFC extraction follows the approved table; simple toolbars/sliders/forms stay inline.
- Constants live in `domain/media` or `platform media contracts` only when they are real shared enum/default registries, not to hide expressions/classes.

## Reuse Candidate Inventory

| Candidate | Locations/callers | Sites | Semantics/layer/variation | In chain |
| --- | --- | --- | --- | --- |
| desktop transport/error | all prior stores and new 4 stores | 3+ | identical infrastructure contract | yes |
| path grants | live exports + 4 media file purposes | 3+ | identical authorization mechanism; purpose varies | yes |
| accepted event identity/units | live pages + 4 media runtimes | 3+ | same domain event, additive gift ID | yes |
| volume validator/formatter | effect/gift/TTS | 3 | same 0-1 rule; labels/lifecycle differ | yes |
| audio output | effect/gift/TTS | 3 | same device mechanism; queue/priority policy differs | yes |
| queue policy | effect/gift/TTS | 3 | superficially similar, different ordering/overflow/pause | yes |
| file preview | recording/highlight/sound | 3 | same secure protocol mechanism; MIME/action policy differs | yes |
| recording status UI | page + shell bar | 2 same | identical normalized status semantics | yes |
| media file field | effect/gift/recording | 3 different | different purpose, validation, actions | yes |
| page form helpers | each form | 1 | page-private changing fields | yes |

## Anti-DRY Abstraction Matrix

| Candidate | Business semantics | Layer | Stability/sites | Variation axes | Decision |
| --- | --- | --- | --- | --- | --- |
| desktop transport/error | same | infrastructure | stable, 3+ | command/payload only | reuse existing |
| path grants | same authorization | infrastructure | stable, 3+ | purpose/kind/save filter | reuse/extend |
| volume rule | same numeric invariant | domain/adapter | stable, 3 | label/owner only | extract primitive pure validator/formatter |
| audio device | same output mechanism | infrastructure | stable, 3 | payload/priority/cooldown | share port/output, inject policy |
| queue algorithms | different domain meaning | application/domain | 3 but divergent | priority, overflow, pause, combo, timing >3 | keep separate |
| preview protocol | same authorization/stream | infrastructure | stable, 3 | MIME and lifetime | extract one registry/port with typed purpose |
| status component | same recording meaning | UI | 2, approved shared owner | action availability/placement | reuse one component with narrow variant props |
| media file field | different semantics | UI | 3 different | purpose, validation, action, permission >3 | keep local |
| form helpers | local | UI | one each/unstable | all form fields | colocate |

## Commonality Classification

- Technical/infrastructure: desktop facade/error, path grants, audio device output, preview registry/protocol.
- Business/domain: volume invariant, accepted event identity, per-capability queue/reducer/transition rules.
- UI/design system: recording status component only; standard controls reuse Element Plus directly.
- Configuration/constants: media enums/defaults under the narrow Rust domain and mirrored DTO types.
- No broad shared utils directory is created.

## Shared Abstraction Decisions

- Reuse `invokeDesktop/listenDesktop/normalizeDesktopError`; migrate every media store and delete generic `utils/ipc.ts` media callers. Existing JSDoc remains authoritative.
- Extend `PathGrantStore`/`PathPurpose`; `MediaFilePort` accepts grant IDs plus exact purpose. Tests cover each real caller. No new public frontend path helper.
- Add pure `validateVolume(value) -> Result<NormalizedVolume>` and TS adapter equivalent only if all three callers keep exact 0-1 semantics; document three `@example` callers.
- `AudioOutputPort` exposes device actions; `EffectRuntime`/`GiftMediaRuntime`/`TtsRuntime` supply typed jobs/policies. Shared output never imports private config entities.
- `PreviewRegistry` public application API is issue/revoke/resolve-by-token with typed preview purpose; protocol adapter owns byte ranges. Document recording/highlight/sound examples.
- Keep effect/gift/TTS queues separate by explicit domain names. Similar heaps/deques may use standard collections but not one generic queue framework.
- Reuse one `RecordingStatusBand` with normalized props; page/shell action variation is explicit narrow props/emits, not mode flags with unrelated branches.

## Dependency Injection And Strategy Boundaries

- Constructors receive `Arc<dyn ...Port>` and a clock where timing tests require it; production assembly occurs only in `AppState::new`.
- `SpeechEnginePort` map provides mechanisms/capabilities; TTS config/priority/fallback policy remains in `TtsRuntime`.
- `RecorderProcessPort` abstracts probe/start/stop/status; source selection/three retries/state transitions remain in `RecordingRuntime`.
- Audio output receives normalized jobs; cooldown/overflow/combo decisions remain in gift/effect owners.
- Pure validators/reducers receive values/config/timestamps, never access database, OS, AppHandle, store, router or global window.

## Responsibility Split

- Pages orchestrate load/draft/actions and feedback.
- Stores coordinate typed requests and immutable snapshots.
- Adapters validate/map DTO and construct payloads.
- Root composable owns main-window listeners.
- Effect component owns only render lifecycle.
- Rust IPC maps DTO/error and window targets.
- Rust application coordinates business state and effects through ports.
- Rust domain owns deterministic rules.
- Infrastructure owns external I/O.

## Function Design And Public Entrypoints

- `MediaState::accept_live_event`, `shutdown` and grouped capability accessors are the only cross-capability application entries.
- Each runtime exposes spec command-shaped methods; internal helpers are private and verb-named.
- IPC exports exactly the 34 command functions from spec; repetitive DTO conversion uses private mapper functions, not macro-generated hidden behavior.
- TS public surface remains `platform/desktop` plus four stores and `useMediaRuntime`; pages/components do not become cross-feature APIs.
- Render root public contract is `CaptainEffectSurface` normalized props and a `dispose`-owning renderer module.

## Functional Boundaries And Side-Effect Model

- Pure: validation, combo reduction, donor/stat reduction, priority/overflow selection, TTS token validation, recording transition, heat window reduction, disk decision, DTO mapping, formatters and filters.
- Effects: repository/file transactions, notification/audio/speech, Tauri emit/window/protocol, FFmpeg, hotkey, Pinia write and lifecycle registration.
- Inputs/snapshots are immutable; mutable deques/process handles exist only inside the unique runtime/registry owner.
- Derived filters/counts/button states are computed and never duplicated as writable state.
- Reject functional libraries, generic reducer frameworks and point-free pipelines.

## State Ownership And Data Flow

- Rust repositories restore configuration/history; runtimes load them at bootstrap and publish initial snapshots.
- Accepted events update the relevant runtime synchronously/boundedly, then enqueue I/O work; subsystem errors update its status/event without failing live connection.
- UI initial query and subsequent events converge on the same adapter/view-model shape.
- Page drafts clone the confirmed snapshot once per open/reset; failed save retains draft, successful save replaces snapshot/draft.
- Shutdown order: stop accepting media events -> cancel actors -> stop/finalize recording -> revoke preview sessions/grants -> unregister hotkey -> close effect window -> join tasks.

## Data Structures And Type Strategy

- Rust newtypes/validated values for `GiftId`, IDs, `Volume`, `MoneyMinor`, colors and durations; serde DTOs remain primitive/camelCase at IPC.
- Enums/discriminated unions for effect position/action, alert method, TTS engine/status, recording source/status/format/quality and highlight type; exhaustive matches reject impossible states.
- `VecDeque` for ordered/bounded event/audio/effect/history windows; binary heap or four FIFO deques for stable TTS priority.
- `HashSet<EventId>` with bounded eviction for idempotency; `HashMap<(UserUid, GiftId), ComboState>` for combo windows.
- File DTOs expose display filename/path only where approved and never canonical authority; preview uses branded opaque ID.
- Split TypeScript native contracts by base/live-data/media behind one barrel; feature view types live under `modules/media` and never cross into native maps.

## Contract And Adapter Boundaries

- Rust serde and registry arrays are the source; TS tests assert exact 34-command/7-event names, camelCase shapes, required/null fields and units.
- Add `giftId` through Rust domain event, serde DTO, TS contract, adapters, fixtures and existing consumers in one task.
- `from*Dto` functions validate enum/range/null semantics and return readonly view models. No template fallback for invalid data.
- IPC errors map to stable Chinese `DesktopError` codes; store actions call shared normalization.
- Import documents are versioned structured data limited to approved gift-binding fields. Exact schema lives beside Rust codec and mirrored TS type only where preview is needed.

## Pattern Decisions And Rejected Alternatives

- Level 2 reuse: ports/adapters, typed desktop facade, observer/event sink and state-owner patterns established by prior modules.
- Level 1 adaptation: discriminated state unions/tables, provider typed record, explicit pipeline and Vue lifecycle/component boundaries.
- Direct code remains inside each runtime for stable command orchestration; no separate Command objects for 34 Tauri functions.
- Direct code alone is insufficient for external I/O/test substitution, provider fallback, state transitions and event fan-out.
- Rejected factory/DI container/manager/registry/chain framework/proxy class hierarchy. Only the custom preview protocol acts as a narrow access proxy because raw path denial is a real requirement.
- Simplify any pattern whose removal preserves dependency direction, deterministic testing and single ownership.

## Readability And Maintenance Guardrails

- No mixed function >150 lines; complex flows use short named phases, not one callback chain.
- No catch-all media `utils`, `helpers`, `manager`, optional mega-interface or compatibility DTO.
- Boolean/effect names follow spec; comments explain only cleanup/order/security constraints.
- Explicit lock order: facade never holds one runtime lock while awaiting another runtime or I/O port.
- Every task records public surface delta, callers migrated and deletions. Generated declarations are reviewed, not blindly accepted.
- Tailwind class values remain inline and short; rendering configuration is not used to hide DOM styling.

## Architecture Risks

- Custom protocol range streaming and process finalization are security/runtime hot spots; implement focused integration tests before UI migration.
- System TTS/audio crates may have platform-specific thread requirements; adapters must keep those requirements outside application state.
- App shutdown ordering could deadlock if actors await ports while locks are held; lock/I/O separation is mandatory.
- `GiftEvent.giftId` is an additive shared-contract change affecting completed-module fixtures and all event adapters.
- Ten final effect assets remain external-content pending; the catalog boundary cannot manufacture approved marketing content.
- Large media code can exceed budgets; split by lifecycle capability, not by generic technical nouns.

## Open Architecture Questions

- No blocking ownership or dependency question remains.
- Concrete system audio/TTS library versions and platform support must be selected in plan from primary documentation and locked in Cargo; they may not alter the approved port/behavior contract.
- The built-in FFmpeg adapter stays unavailable until the approved artifact exists; local executable support is the only concrete process adapter available now.
- Final recording-status placement and plugin effect integration remain explicit downstream-module handoffs, not compatibility work in this module.
