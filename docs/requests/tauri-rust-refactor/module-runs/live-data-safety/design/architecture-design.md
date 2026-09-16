# Live Data And Safety Architecture Design

## Delivery Unit Identifier

- request: `tauri-rust-refactor`
- module: `live-data-safety`
- stage input: framework-approved `spec/spec.md` and `design/page-design.md`
- architecture sensitivity: Rust/Vue ownership migration, multi-source async state, repository atomicity, event fan-out, path security and shared UI contracts
- downstream: `plan`

## Architecture Objective

建立一条唯一、可追踪的生产数据链：Bilibili 只读 transport -> Rust 领域 pipeline -> SQLite/内存聚合 -> Tauri typed events/commands -> Pinia server snapshots -> Vue view models/components。旧 renderer WebSocket、filter、blacklist、statistics 和 Electron database path 必须退出生产链，且后续媒体模块只能消费新链的只读投影。

## Architecture Scope And Triggers

- 新建 Rust live domain/application/infrastructure/IPC capability。
- 扩展 platform-foundation 的 AppState、path grant、typed gateway、permission registry。
- 重构 room/danmaku stores，新增 safety/statistics/ranking stores 和根级 listener lifecycle。
- 重写/新增弹幕、敏感词、黑名单、看板、排行榜、导出和最小全局连接控件。
- 删除本模块闭包内旧 renderer owners；保留一个只读 media compatibility projection，直到 `interaction-media` 完成迁移。
- 不改变 spec 的行为、字段、CSV、状态、权限、页面结构或验收阈值。

## Upstream Inputs And Assumptions

- `platform-foundation` 已提供唯一 Tauri transport、AppState、DesktopError、SQLite/WAL、path grants、free/member features。
- production Bilibili endpoint/credential provisioning 仍为外部待补；架构提供 `RoomTransport` port 和 unavailable adapter，不嵌入假凭证或假成功。
- SQLite `0001_initial.sql` 的 21 表/7 索引保持权威；如趋势持久化需要新表/列，只能通过追加 migration，不改写 0001。
- page-design 的信息架构和 Vue component decisions 是固定输入。

## Scaffold Alignment And Bootstrap Boundaries

不产生新 scaffold。保留现有 Vue/Vite/Tauri package、`src-tauri` 五层和构建脚本；增加 domain-specific 文件、依赖、permissions 和 tests。禁止另建前端、第二个 Tauri crate、第二个 transport 或 Electron fallback。

## Expert Architecture Decision Records

### ADR-LDS-01 One Rust LiveDataState In Composition Root

- problem: 连接、规则、聚合和 background tasks 需要共享生命周期，但现有 AppState 只有 bootstrap/path。
- decision: `AppState` 新增 `live_data: LiveDataState`。`LiveDataState` 持有 `Arc<dyn RoomTransport>`、`Arc<dyn LiveRepository>`、`Arc<dyn LiveFilePort>`、`Arc<dyn LiveEventSink>` 与一个 `tokio::sync::Mutex<LiveRuntimeState>`；commands 只调用其 application methods。
- alternatives: 每个 command 创建 service；全局 static singleton；把 Tauri AppHandle 传入 domain。前两者破坏生命周期/测试，后者反转依赖。
- smallest sufficient: 一个 composition-root service，没有 registry/factory 层。
- direction: `ipc -> application -> domain <- infrastructure`; `ipc/event_sink` implements application port。
- ownership change: renderer singleton -> Rust managed state；网络/DB/event side effects -> injected ports。
- seam: only transport/repository/file/event ports; no provider registry。
- cleanup: 删除 `DanmakuService` 连接 owner、`BilibiliClient` runtime imports。
- evidence: fake-port integration tests、single-connect/cancel tests、dependency search。

### ADR-LDS-02 Ordered Event Pipeline With Explicit Outcomes

- problem: current renderer emits danmaku before blacklist and ignores sensitive result。
- decision: pure `evaluate_live_event` sequence returns `PipelineOutcome::Accepted(NormalizedLiveEvent)`、`SensitiveFiltered(FilterRecord)` or `Blacklisted(UserUid)`. Application service alone persists/aggregates/emits based on outcome。
- alternatives: scattered subscriber filters; generic middleware registry; component filtering. They allow order drift or unapproved extension。
- smallest sufficient: fixed functions and exhaustive enum, not dynamic chain objects。
- owner: domain owns decisions; application owns ordered effects; repository/event sink own I/O。
- extension: none for arbitrary rules. `membership-intelligence` may insert its approved moderation consumer only after a future spec/architecture pass。
- evidence: negative fan-out tests and exhaustive match compile check。

### ADR-LDS-03 Server Snapshots, Local View State

- problem: current Pinia computes authoritative counts/ranks and pages create timers/deep watchers。
- decision: Rust is the only writable owner of connection/safety/session aggregates/stat/rank. Pinia stores replace whole immutable snapshots and own only async/view state. Page-local filter/search/tab/TOP/pause/dialog drafts remain local or store view state where cross-page continuity is required。
- alternatives: one God store; component fetches; writable mirrored totals. Rejected for fan-out and drift。
- data flow: serde DTO -> gateway contract -> pure adapter -> readonly view model -> component props。
- cleanup: delete `getStatistics/get*Rank` and dashboard generated histories/timers/deep watch。
- evidence: store tests prove no local aggregation and stale response protection。

### ADR-LDS-04 Root Runtime Composable Owns Event Subscriptions

- problem: events feed multiple pages/stores and must survive route changes without duplicate listeners。
- decision: `useLiveDataRuntime()` is mounted once by `App.vue`, owns all gateway `listen` registrations, dispatches payloads to narrow store actions, and returns/stores idempotent `start/stop` cleanup. Stores do not each register global listeners。
- alternatives: page-scoped listeners lose events; singleton manager hides lifecycle; each store listener multiplies fan-out。
- smallest sufficient: one composable with an explicit array of unlisten callbacks; no event bus wrapper。
- cleanup: `App.vue` removes `getDanmakuService` and beforeunload fire-and-forget path; runtime stop invokes disconnect/cleanup deliberately。
- evidence: mount/unmount and duplicate-start tests。

### ADR-LDS-05 Repository Transaction + Post-Commit Memory Swap

- problem: DB/matcher/blacklist index can diverge during mutation/import failure。
- decision: repository transaction validates/writes an immutable candidate snapshot; only after commit does application atomically swap matcher/index/snapshot. Single mutations follow the same post-commit update rule。
- alternatives: mutate memory then persist; repository callbacks mutate application state; two-phase distributed abstraction. Rejected as unsafe or excessive。
- owner: repository SQL only; domain candidate compile; application swap。
- evidence: injected failure tests assert pre-state preserved。

### ADR-LDS-06 Purpose-Bound File Codec Facade

- problem: four import/export workflows share path security and CSV mechanics but differ schemas/duplicate rules。
- decision: extend `PathPurpose` with exact input/output purposes. `LiveFilePort` exposes typed operations (`read_sensitive_words`, `write_sensitive_words`, `read_blacklist`, `write_blacklist`, `write_stats`, `write_rankings`) accepting resolved grants, not paths. Shared CSV escaping/UTF-8 is internal infrastructure; domain codecs stay separate typed functions。
- alternatives: generic `read_file/write_file`; frontend raw path; one mega import strategy. Rejected for capability widening and schema coupling。
- seam: format enum only for sensitive txt/csv/json; no plugin codec registry。
- evidence: wrong-purpose/stale/symlink tests and round trips。

### ADR-LDS-07 Feature Components Share View Models, Not DTOs

- problem: identity/range/export UI repeats while raw DTO mapping in components would leak backend semantics。
- decision: `modules/live-data/contracts.ts` defines UI view models and pure adapters/formatters; shared components consume those models. Backend DTO types remain in `platform/desktop/contracts.ts`。
- alternatives: direct DTO props; catch-all shared utils; duplicated mapping in pages。
- smallest sufficient: one domain-named frontend module with stable shared types/functions used by 3+ surfaces。
- evidence: adapter tests and import graph search show components never import platform contracts。

### ADR-LDS-08 Bounded Compatibility Projection For Interaction Media

- problem: old gift/effect/recording pages currently depend on `useDanmakuStore` shapes, while the next module has not migrated。
- decision: the rewritten danmaku store exposes readonly `legacyDanmakuList/legacyGiftList` projections only where an existing in-scope caller cannot yet consume normalized events. Projection maps new events, never opens transport, writes DB, filters, aggregates or mutates Rust truth。
- owner: `stores/danmaku.ts`; cleanup trigger: `interaction-media` review passes or all callers migrate, whichever comes first。
- rollback: projection can be removed without Rust/API change; if it breaks current build, restore projection mapping, never restore old WebSocket path。
- stale detection: `rg useDanmakuStore|getGiftRank|giftList` in interaction-media review。
- evidence: consumer build tests and negative ownership search。

## Business Capability And Ownership Map

| Capability | Writable/data owner | Orchestration owner | UI owner | Forbidden owner |
| --- | --- | --- | --- | --- |
| room lifecycle | Rust `LiveRuntimeState` | `LiveDataState` | global connection control + room store snapshot | Vue service/WebSocket |
| protocol adaptation | Rust `RoomTransport` adapter | transport task | none | component/store |
| identity/filter decisions | Rust domain | application event loop | identity component presentation | template/computed |
| sensitive words | SQLite + immutable Rust matcher | application safety methods | safety store/page/dialog | localStorage/renderer Trie |
| blacklist | SQLite + immutable Rust UID set | application safety methods | safety store/page/dialog | renderer Map/EventEmitter |
| event log | SQLite repository | application pipeline | log navigation handoff | WebView DB client |
| statistics/rank | Rust aggregate state + repository | application snapshot tasks | analytics stores/pages/charts | Pinia reducers/deep watch |
| file import/export | Rust resolved grant + file adapter | application methods | export dialog/path display | raw frontend filesystem |
| gateway lifecycle | Tauri transport | root runtime composable | stores/pages consume | individual components |
| view state | Pinia/page local | store actions/page handlers | components | Rust domain |
| route state | Vue Router | App/router | navigation | Rust |

## Module Boundary Design

- Rust `domain/live_data`: immutable business types, validators, pipeline outcomes, aggregators; no Tauri/SQLx/network/file imports。
- Rust `application/live_data`: use cases, ports, runtime state, cancellation, effect ordering; no concrete SQLx/Tauri dialog/Bilibili SDK details。
- Rust `infrastructure/live_data`: SQLite repositories, file codecs, unavailable/production transport adapter。
- Rust `ipc/live_data`: serde DTO mapping, commands, Tauri event sink; no business branching beyond boundary validation/error mapping。
- TS `platform/desktop`: exact backend contract mirror and transport only。
- TS `modules/live-data`: UI-safe types, adapters, stable formatters; no invoke/listen/store/global side effect。
- Pinia stores: snapshot/async state and command actions; no protocol/filter/stat/rank business rules。
- views/components: interaction/rendering; no raw DTO normalization or persistence。

## File And Directory Structure

```text
packages/bilibili-live-tool/src-tauri/src/
  domain/live_data/
    mod.rs
    models.rs
    identity.rs
    safety.rs
    analytics.rs
  application/live_data/
    mod.rs
    ports.rs
    runtime.rs
  infrastructure/live_data/
    mod.rs
    repository.rs
    files.rs
    transport.rs
  ipc/live_data/
    mod.rs
    dto.rs
    commands.rs
    events.rs

packages/bilibili-live-tool/src/renderer/
  modules/live-data/
    contracts.ts
    adapters.ts
    formatters.ts
  composables/useLiveDataRuntime.ts
  stores/
    room.ts
    danmaku.ts
    safety.ts
    statistics.ts
    ranking.ts
  components/live-data/
    LiveConnectionControls.vue
    LiveEventStream.vue
    AudienceIdentity.vue
    AudienceDetail.vue
    DataRangeControl.vue
    DataExportDialog.vue
    StatMetric.vue
    RankingTable.vue
    SensitiveWordDialog.vue
    BlacklistDialog.vue
  views/
    danmaku-list/index.vue
    sensitive-words/index.vue
    blacklist/index.vue
    dashboard/index.vue
    ranking/index.vue
```

Tests colocate beside TS pure modules/components where supported; Rust domain unit tests remain beside modules and cross-layer contracts live in `src-tauri/tests/live_data_contracts.rs`。Do not add barrel files whose only role is re-exporting everything。

## Code Relationship And Dependency Direction

```text
Bilibili adapter ─┐
SQLite repository ├─implements─> application ports <─ LiveDataState
file adapter      ┤                              │
Tauri event sink ─┘                              v
                                            pure domain
                                                 │
Tauri commands/dto <─────────────────────────────┘
          │
desktop transport/contracts
          │
root runtime composable -> Pinia stores -> pages -> components
                              │              └-> modules/live-data pure UI model
                              └-> command actions
```

Shared UI/model code never imports a feature page/store. Rust domain/application never imports IPC/infrastructure concrete modules。

## State Topology And Data Lifecycle

### Rust writable state

- `ConnectionState`: exhaustive enum plus optional room/error/reconnect attempt and a cancellation token/task handles。
- `SafetySnapshot`: immutable compiled sensitive matcher + blacklist UID set, swapped under one narrow lock。
- `SessionAggregate`: dedupe IDs, event/user aggregates, trend ring buffer and previous rankings。
- Background task handles for room read loop/stat tick/rank tick; all owned by `LiveRuntimeState` and cancelled together。

### Frontend writable state

- room store: latest `RoomSnapshot`, command pending/error。
- danmaku store: bounded accepted event rows, Rust footer snapshot, clear epoch and compatibility projection。
- safety store: sensitive/blacklist authoritative snapshots and per-operation pending/error。
- statistics/ranking stores: current range token, latest snapshot, pending/error/stale。
- page local: search/filter/pause/tab/TOP/dialog drafts/highlight presentation clock。

### Derived state

- visible rows from bounded events + local filter/search；TOP slice from Rust TOP100；formatted labels/amount/time from pure formatters；button availability from state union。None becomes writable duplicate state。

### Lifecycle

1. raw upstream payload is validated/adapted to Rust domain event。
2. pipeline yields accepted/filtered/blacklisted outcome。
3. application persists and updates aggregates, then emits only contract-approved payload。
4. Tauri serde DTO reaches desktop transport。
5. root runtime passes immutable payload to owning store。
6. adapter creates UI view model; component consumes props。
7. form creates a local draft, store action builds typed command payload, Rust revalidates and returns authoritative result。

Null/enum/money/time semantics normalize in Rust DTO mapper; UI adapter only converts display forms. Async request IDs live in stores; cancellation/unlisten lives in runtime/page lifecycle。

## Dependency Rule Matrix

| From | May import | Must not import | Public API |
| --- | --- | --- | --- |
| Rust domain | std, pure crates | Tauri, SQLx, reqwest, filesystem | domain types/functions |
| Rust application | domain, port traits, Tokio sync/time | IPC DTO, concrete infra | `LiveDataState` use cases |
| Rust infrastructure | domain/application ports, SQLx/file/network crates | IPC/UI | port implementations |
| Rust IPC | application/domain, Tauri, DTO | SQL queries/upstream payload | commands/events/dto |
| desktop contracts | TS types only | Pinia/Vue/page | command/event maps |
| desktop transport | Tauri API + contracts | business/store/UI | invoke/listen |
| modules/live-data | gateway contract types as input types | invoke/listen/store/router | UI models/adapters/formatters |
| runtime composable | gateway + stores + Vue lifecycle | pages/components | start/stop |
| stores | gateway + adapters/contracts | components/pages/ECharts | actions/readonly state |
| pages | stores + components + local helpers | raw Tauri/upstream/SQL | route component |
| components | UI models + Element Plus | gateway/store/raw DTO | props/emits |

Cycle risks: runtime imports stores, stores must not import runtime; adapters may import platform contract types, platform must not import adapters; components never import stores。

## Extension Seams And Rejected Future-Proofing

- real seams: one `RoomTransport` port for unavailable vs configured production; repository/file/event ports for deterministic tests; typed codec enum; ranking kind typed record。
- no multi-provider transport registry, dependency container, dynamic pipeline, plugin filters, generic event middleware, generic CRUD repository or UI schema renderer。
- a second real Bilibili API version/provider, not hypothetical support, is the trigger for a strategy map/adapter registry。
- if only one file codec remains, collapse the codec strategy to direct functions; if projection callers reach zero, delete compatibility immediately。

## Complexity Budget

- new Rust production files: target <=16; frontend production files: target <=20, excluding focused tests。Exceeding requires merging by cohesive owner, not catch-all files。
- new Tauri commands: <=20 listed in spec；events: <=9 listed in spec。No unapproved public names。The budget counts every slash-separated mutation name in the approved table as an independent Tauri command。
- indirection: UI action -> store -> gateway -> command -> application -> domain/repository, maximum one mapper on each side。
- reactive fan-out: one native listener per event at app root；no deep watch on live arrays；no page timer for server cadence；one presentation timer owner for rank highlight。
- component responsibility: one business concept/lifecycle；page remains orchestration, not business calculation。
- migration scope excludes member/media internals except compatibility caller adaptation required to keep build/runtime working。

## Migration, Compatibility, And Cleanup Design

1. Add Rust types/ports/domain/repository/application with tests while existing renderer path still compiles。
2. Extend IPC/gateway contracts and build permissions; add fake/unavailable transport。
3. Add runtime composable/stores and switch App connection lifecycle to new path。
4. Rewrite module pages/components and routes against new stores。
5. Adapt existing gift/media consumers to readonly projection; remove `DanmakuService`, renderer client/manager/parser/blacklist/sensitive owners and Electron DB dependency from this chain。
6. Search callers/imports and remove obsolete exports/tests/mocks/dependencies where no other module owns them。

Temporary projection owner: danmaku store. Cleanup trigger: `interaction-media` review. There is no rollback to old native/runtime path; rollback means disable new UI entry and preserve unavailable state while fixing Rust path。No feature flag or dual active connection is allowed。

## Locality And File-Extraction Decisions

- view search/filter/confirm/toolbar helpers stay in owning page because one caller and no independent lifecycle。
- `adapters.ts` is separate despite finite callers because it isolates backend DTO from view model and has focused contract tests。
- `formatters.ts` is shared because time/amount/count semantics have 3+ stable callers; functions accept primitives and no environment state。
- each dialog/list/chart component is separate only where page-design approved a cohesive interaction/performance/focus boundary。
- Rust analytics/safety split is domain cohesion and independent test surface, not speculative reuse。
- no local `utils.ts`, one-function wrapper files or manager/factory classes。

## Reuse Candidate Inventory And Anti-DRY Matrix

| Candidate | Locations/callers | Semantic/layer stability | Axes | Decision |
| --- | --- | --- | --- | --- |
| desktop transport/error | platform + all new stores | identical infrastructure, established | command/event type only | reuse existing Level 2 |
| numeric/text validation | current Rust validation + all mutations | same domain constraints, 3+ | max length/type | reuse/extend domain newtypes |
| path grant | platform + 5 file flows | same security mechanism, 3+ | purpose, kind | reuse mechanism; typed purposes |
| time display | stream/update/chart, 3+ | same event-clock `HH:mm:ss` where spec says so | invalid handling | extract `formatClockTime` |
| money display | gift/dashboard/rank/detail, 3+ | same CNY fixed 2 decimals | minor unit | extract `formatCny` |
| range model | dashboard/ranking/export, 3 | same business enum | custom interval | shared contract/control |
| identity UI | stream/detail/ranking, 3 | same business meaning | clickable context | shared component with explicit flag |
| page request state | 4 stores | similar code but different concurrency/errors | lifecycle, stale, mutation | keep separate; no generic async store |
| sensitive/blacklist import | 2 | different schema/duplicate/business effects | format, validation, duplicates, compile | share file/security mechanism only |
| stats/rank aggregation | old store/dashboard vs Rust | same desired result but wrong layer | metric kind/range | migrate to Rust; never share TS logic |
| event compatibility mapping | gift/media callers <=2 | temporary, next module changes | shape/lifecycle | feature-local projection; defer cleanup |

Anti-DRY verdict: extract only established infrastructure plus 3+ stable UI/domain mechanisms. Keep async stores and safety codecs separate because business/lifecycle axes differ or use sites are fewer than three。All candidates lie in the approved caller chain。

## Commonality Classification

- technical/infrastructure: desktop transport/error, path grant, CSV escaping。
- business/domain: RoomId/UserUid validation, DataRange, IdentityTag, event/metric/ranking models。
- UI/design-system: AudienceIdentity, DataRangeControl, DataExportDialog, StatMetric。
- configuration/constants: reconnect/stat/rank/highlight cadence and bounded queue limits under narrow live-data owners。

## Shared Abstraction Decisions

- reuse `DesktopTransport` without wrapper; add contract entries/tests and JSDoc examples for live query/event callers。
- reuse/extend Rust validation types; no TS business-validation authority, only matching immediate feedback。
- promote `DataRange` and display formatters in `modules/live-data`; JSDoc `@see` names dashboard/ranking/export and tests verify identical semantics。
- components share normalized view models; variation via props/emits (`isInteractive`, `onUserSelect`) rather than optional mega DTO。
- CSV escape helper stays infrastructure-private; typed row builders remain capability-specific。

## Dependency Injection And Strategy Boundaries

- `LiveDataState::new(transport, repository, files, events, clock)` receives explicit ports; composition root chooses unavailable/production implementations。
- `Clock` injection is justified for cadence/rank-highlight deterministic tests; if Tokio paused time suffices, architecture may use Tokio clock directly and avoid another trait。
- ranking selection uses `RankingKind` exhaustive match or typed function record, not dynamic boxed strategy objects。
- sensitive file format uses closed enum dispatch to pure codecs；filesystem operation stays injected port boundary。
- frontend pure helpers receive values/DTO arguments only; no store/router/window/localStorage access。

## Responsibility Split

- commands: deserialize, invoke application, map DesktopError。
- application: check state, sequence effects, transactions, cancellation, event publication。
- domain: validate/normalize/decide/aggregate without I/O。
- infrastructure: network, SQL, path resolution, file encoding。
- runtime composable: one event subscription lifecycle and store dispatch。
- stores: command pending/latest-wins and immutable snapshots。
- pages: route-level workflow, local filters/forms/dialogs。
- components: rendering and user intent emits only。

## Function Design And Public Entrypoints

- Rust public application methods mirror approved commands: `connect_room`、`disconnect_room`、`query_audience`、sensitive/blacklist mutations/import/export、`query/refresh/export_stats`、`query/export_rankings`。
- background private functions: `run_room_session`、`consume_frame`、`publish_stat_tick`、`publish_rank_tick`、`shutdown_session`。
- pure domain public functions: parse newtypes、`normalize_identity`、`evaluate_live_event`、`aggregate_event`、`build_stat_snapshot`、`build_ranking_snapshot`、`calculate_rank_changes`。
- frontend store actions use corresponding exact verbs；handlers use `handle*`；components only emit approved intentions。
- no generic `execute`, `process`, `manage`, `handleData` or broad service facade names。

## Functional Boundaries And Side-Effect Model

- pure: all validation/mapping/filter decision/range/ranking/stat/CSV row/display functions；inputs immutable and outputs new values。
- effects: transport read/connect, repository transactions, file reads/writes, event emit, gateway invoke/listen, Pinia assignment, route/focus/dialog UI。
- application may orchestrate effects but does not hide them in query-looking helpers。
- computed state never writes source state；watch is limited to library lifecycle where event handlers cannot express the relationship and must be justified。
- no functional utility dependency or unreadable reducer/curry chain。

## State Ownership And Data Flow

- one writable owner per business state as defined above。
- command results and events may race: stores compare range/session/request generation before assignment。Disconnect increments session generation so late events are discarded。
- `isPending` terminates in `finally`; error is cleared only on a new owned operation or successful completion, not by unrelated events。
- rank highlight expiry is presentation-only and does not mutate ranking snapshot。
- listener cleanup is idempotent and awaited where API permits；window close cancels Rust tasks even if WebView disappears first。

## Data Structures And Type Strategy

- Rust newtypes for RoomId/UserUid/EventId/SensitiveWordId/PathGrantId and integer money minor units。
- Rust enums for ConnectionState、LiveEvent、IdentityTag、WordCategory、MatchType、DataRange、RankingKind、PipelineOutcome；exhaustive matches prevent impossible states。
- serde DTOs use camelCase and explicit optionality；TS uses matching discriminated unions and branded IDs。
- `null` crosses IPC only for explicit absence; absent optional fields are not overloaded as empty/zero。
- snapshots use readonly arrays in TS and owned immutable values from Rust；no `Record<string, unknown>` for domain payloads。

## Contract And Adapter Boundaries

- Rust upstream adapter preserves external field names privately, then maps once to domain。
- Rust IPC mapper converts domain to approved camelCase DTO and stable errors。
- TS platform contracts mirror backend; `modules/live-data/adapters.ts` maps to presentation values without business recomputation。
- file codecs map typed domain rows to/from explicit formats after path resolution。
- a contract fixture test serializes representative Rust DTO JSON and validates expected TS shape/key semantics; compilation enforces frontend map keys。

## Pattern Decisions And Rejected Alternatives

- `reuse existing pattern`, Level 2: typed Facade/Adapter via DesktopTransport。
- `adapt lightweight pattern`, Level 2: State via Rust/TS discriminated unions; Observer via native Tauri events + one lifecycle composable; fixed Pipeline via explicit functions; Strategy via exhaustive enum dispatch for rank/codec。
- ports are dependency inversion boundaries with real unavailable/fake/concrete and I/O isolation reasons, not one-method ceremony。
- reject dynamic Chain of Responsibility, event bus wrapper, service locator/DI container, factory registry, generic CRUD repository, class-based Vue service and schema-driven UI。
- remove/simplify signals: a second mapper with no semantic conversion, a port with one implementation and no test/I/O boundary, or a shared component with one pass-through caller and no lifecycle/test benefit。

## Readability And Maintenance Guardrails

- files own one named capability; no `utils` dumping ground。
- functions are short enough to expose effect order; event pipeline uses exhaustive outcome, not nested booleans。
- constants live with rule owner; comments explain protocol/atomicity only。
- page templates contain display expressions only, not DTO fallbacks or large conditional classes。
- errors preserve code/context without secret/path leakage。
- all temporary projection symbols include removal note referencing `interaction-media` artifact, and no compatibility branch can initiate transport。

## Architecture Risks

- Existing media/effect code is coupled to the old service; compatibility projection must remain narrowly read-only or the old owner will leak back。
- Current DB schema stores aggregate rows but not explicit high-resolution trend series; plan must inspect whether `t_event_log` queries suffice before proposing an additive migration。
- Tauri event throughput may require frame batching. Any batching must preserve qualified event semantics, ordering within priority rules, <100ms latency and full Rust counts。
- System dialog currently grants directories for export; save-file purposes require path-grant extension without exposing raw path to commands。
- Unavailable production transport limits live end-to-end verification; handoff must not conflate deterministic fake integration with real upstream connectivity。

## Open Architecture Questions

- None that change approved behavior. Plan must select Tokio paused time versus a small clock port after checking test ergonomics, and event-per-item versus bounded batch payload after a measured serialization spike; both choices must satisfy the same public event semantics and acceptance thresholds。
