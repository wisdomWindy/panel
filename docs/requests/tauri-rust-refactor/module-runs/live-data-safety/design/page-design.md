# Live Data And Safety Page Design

## Delivery Unit Identifier

- Request: `tauri-rust-refactor`
- Module: `live-data-safety`
- Sequence: 2 of 6
- Stage: page-design
- Status: approved design input for spec

## Page Objective

Provide a compact read-only live-operations workspace where a streamer or operator can connect one room, monitor high-volume events, inspect audience identity, maintain local safety rules, review common statistics/rankings and export source-approved CSV data. The UI must make connection health and data freshness obvious without adding any capability to send content or operate the live room.

## Target Users Or Scenarios

- Streamer preparing a room and starting/stopping a read-only monitoring session.
- Operator scanning live danmaku and gifts, pausing auto-scroll to inspect history, and opening a viewer detail card.
- Operator maintaining sensitive words or a local blacklist before/during a stream.
- Streamer reviewing current/historical metrics and top contributors in either installer.
- Operator exporting dashboard or ranking data through a Rust-authorized system dialog.

## Source And Upstream Constraints

- Product authority: PRD 8, 9.1, 9.2, 9.3, 9.5, 9.9, 9.10 and 9.11.
- Module boundary: `requirements/modules/live-data-safety.md`; no TTS, recording, AI, moderation, membership or plugin page behavior.
- Foundation contract: Vue uses the typed desktop gateway; Rust owns room HTTP/WebSocket, filtering, logging, statistics, ranking, database and CSV writes.
- Existing code facts: `/connection`, `/danmaku-list`, `/dashboard` and `/blacklist` exist; sensitive-word/ranking pages are missing; existing dashboard derives server data in Vue and must not remain authoritative.
- Global toolbar/navigation/status-bar composition is finalized by `application-shell`; this module defines its required connection/status integration inputs but does not redesign the whole shell.
- Free and member builds show identical live-data/safety/dashboard/ranking/export surfaces.

## Product-Content Allowlist Mapping

| Surface | Approved content | Exact source |
| --- | --- | --- |
| Global room control | `房间选择`, add/select room, `连接`/`断开`; statuses disconnected/connecting/connected/reconnecting/error | PRD 8.6, 9.1.2, 9.1.5-9.1.6; module state list |
| Connection feedback | invalid/not-live, credential invalid with settings navigation, reconnect yellow state, manual retry after 10 failures | PRD 9.1.6 |
| Danmaku toolbar | `全部`, `仅舰长`, `仅礼物`, `仅高亮`, username/content search, `暂停滚动`, `清屏` | PRD 9.1.3-9.1.5 |
| Danmaku rows | identity badge, username, level, fan medal name/level, content, HH:mm:ss, gift name/count/value, captain type/level | PRD 9.1.4, 9.2.3-9.2.4 |
| Danmaku footer | total count, rate, filtered count; filtered count navigates to filter log | PRD 9.1.3-9.1.5, 9.3.3/9.3.5 |
| Clear feedback | `已清屏，日志仍保留`; panel only, database unchanged | PRD 9.1.5 |
| User detail | UID, level, medal, session danmaku count, gift total, blacklist status, add-to-blacklist | PRD 9.1.5, 9.2.5 |
| Identity visuals | admin green, governor purple, admiral blue, captain gold, VIP orange, ordinary no badge; malformed medal omits level | PRD 9.2.3, 9.2.6 |
| Sensitive-word toolbar | `新增`, `导入`, `导出`; category filter and word search | PRD 9.3.3-9.3.5 |
| Sensitive-word rows/form | word, category, exact/fuzzy match, enabled state, edit/delete; categories illegal/advertising/attack/custom | PRD 9.3.3-9.3.5 |
| Sensitive-word footer/states | total, enabled, today's hits; format line error, duplicate count, compile progress, transactional rollback | PRD 9.3.3, 9.3.6 |
| Blacklist toolbar | `手动添加`, `导入`, `导出`, username/UID search | PRD 9.5.3-9.5.5 |
| Blacklist rows/form | UID, username, reason, added time, remove; add form has UID/username/reason only | PRD 9.5.3-9.5.5 |
| Blacklist footer/states | total, today added; duplicate UID offers reason update; import line error/rollback | PRD 9.5.3, 9.5.6 |
| Dashboard toolbar | range `本场/今日/近7天/自定义`, `导出 CSV`, `刷新` | PRD 9.9.3-9.9.5 |
| Dashboard metrics | online, danmaku total, danmaku rate, gift total value, captain total | PRD 9.9.3-9.9.4 |
| Dashboard charts | online, danmaku rate and gift revenue trends; point tooltip/detail | PRD 9.9.3-9.9.5 |
| Dashboard states | `暂无数据`; memory data remains with historical-data-unavailable feedback on DB failure | PRD 9.9.6 |
| Ranking controls | range, TOP 10/20/50/100, export, tabs gift/danmaku/captain | PRD 9.10.3-9.10.5 |
| Ranking rows/footer | rank, user, metric value, change; total people, update time; 3-second change highlight | PRD 9.10.2-9.10.4 |
| Ranking states | `暂无数据`; in-memory ranking remains on DB error | PRD 9.10.6 |
| Export dialog | range, dashboard/ranking checkboxes, system-dialog path display, cancel/export | PRD 9.11.2-9.11.5 |
| Export failures | disk space, path permission, no data; `export_done` success feedback | PRD 9.11.5-9.11.6 |
| Forbidden | send input/button, arbitrary paths, frontend WebSocket/SQL/statistics, free/member differences for shared data | PRD 2.2, 4.2, 9.1.1, 9.9-9.11, 10.1-10.5 |

Unapproved existing UI removed from the design: configurable reconnect interval/count, connection-page user login field, blacklist duration/permanent-ban fields, dashboard recent-activity feed, client-generated chart history and a main-process API fallback.

## User Journey And Task Success

1. User selects a room in the global toolbar and activates `连接`. The control becomes pending and the status progresses through connecting to connected, or displays the exact PRD failure/recovery route.
2. Connected user enters `弹幕面板`; current events append continuously. Filters/search are local view controls. `暂停滚动` stops viewport movement while ingestion/counts continue.
3. User opens a username detail card. `加入黑名单` is the only free safety action; after success that user's entries disappear immediately and no longer affect speech/statistics.
4. User enters `风控中心` to manage sensitive words or blacklist rows. Create/update/toggle/remove/import actions use pessimistic completion: the table changes only after Rust confirms persistence/compilation.
5. User opens dashboard/ranking, chooses a source-approved range and reads Rust-owned snapshots. Manual refresh and periodic events update freshness without overlapping requests.
6. User opens export from dashboard/ranking, selects approved range/types and an opaque system-dialog grant, then exports. Success is `export_done`; cancellation closes without side effect.

Primary actions: connect, add/save safety rule, export. Secondary actions: filters, search, range, refresh, pause. Destructive actions: sensitive-word delete and blacklist remove, both require the PRD-specified second confirmation. Non-goal actions: sending danmaku, remote moderation and changing fixed reconnect policy.

## Information Architecture And Content Strategy

- `弹幕面板`: connection context -> filter/search/actions -> virtualized event stream -> live counters. Current live information stays in one scan path.
- `风控中心`: two child navigation targets, `敏感词` and `黑名单`; each uses command toolbar -> filter/search -> dense table -> aggregate footer.
- `数据看板`: range/actions -> five compact key metrics -> three full-width trend bands. No recent-activity or decorative card feed competes with operational data.
- `排行榜`: range/TOP/export -> three tabs -> one consistent table -> freshness footer.
- Export is a shared modal invoked from dashboard/ranking, not a route.
- User detail is a shared anchored popover/drawer invoked from danmaku and rankings, not an independent page.
- Connection ownership stays in the global toolbar/status bar; `/connection` may serve room selection/history only if application-shell source later authorizes it. This module does not preserve the current unsourced reconnect settings form.

## Layout Structure

- All pages use an unframed constrained content region inside the global shell, with a compact 48-56px page toolbar and full-width content bands.
- Toolbars keep title left, high-frequency filters in the middle, commands right. Dangerous commands are visually separated from primary commands.
- Danmaku uses a stable-height flex column: toolbar, growing virtual list, fixed footer. Rows are not standalone decorative cards.
- Safety pages use a single table surface with sticky header and footer; create/edit forms open in focused dialogs.
- Dashboard metric tiles form five equal tracks at wide width and wrap to 3+2 at minimum width. Trend charts are stacked full width to preserve 24-hour readability.
- Ranking uses tabs directly above one table; TOP/range controls never resize the table region.
- Export dialog width is bounded and its path line wraps/truncates without making raw path editable.
- No card is nested inside another card; sections are separated by borders, whitespace and full-width bands.

## Visual Hierarchy

- Page title is compact operational text, not hero typography.
- Connection/freshness state is always visible but secondary to the current data stream.
- Numeric metrics use tabular figures and the strongest type weight in dashboard only.
- Identity badges and ranking movement use color plus text/icon shape; content remains dominant.
- Destructive actions use restrained red only at the point of action/confirmation.
- Filtered/blacklisted state is not shown as leaked event content; only aggregate counts/log navigation expose the approved evidence.

## Visual Design Direction

- Tone: quiet broadcast operations console, dense enough for repeated monitoring, with neutral white/gray surfaces and clear separators.
- Brand pink is reserved for selected navigation/primary accent. Connection green, reconnect amber, error red, identity colors and ranking movement provide independent semantic colors, avoiding a one-hue palette.
- Border radius is 6-8px for dialogs/table containers and at most 6px for repeated metrics.
- Icons come from the existing Element Plus icon set for refresh, search, pause/play, clear, import/export, add, edit/delete and status where available; unfamiliar icon-only controls receive tooltips.
- Motion is limited to 3-second rank-change emphasis and short state transitions; no decorative animation.
- Loading uses stable skeleton rows/chart placeholders so toolbars and page dimensions do not shift.

## Section Breakdown

### Danmaku Panel

- Filters are a segmented/select control, search is one compact input, pause uses pause/play icon plus current state, clear uses a trash/clear icon and source-approved completion feedback.
- Virtual rows reserve columns for identity/user metadata, content/event summary and time. Gift/captain rows use the same grid so arrival does not shift layout.
- When paused, new items remain queued and footer counts continue; the viewport position stays unchanged.

### Sensitive Words

- Table columns: word, category, match type, state, operations. Toggle is direct but pessimistic.
- Create/edit dialog owns word/category/match/state validation presentation. Delete owns a confirmation dialog.
- Import shows background AC compile progress and the PRD result counts/errors only after Rust responds.

### Blacklist

- Table columns: UID, username, reason, added time, remove. Search filters locally.
- Add/edit-reason dialog contains only UID, username and reason. No duration field appears.
- Removing a user is confirmed and table/event visibility updates after Rust acknowledgement.

### Dashboard

- Five metrics precede charts. Range is the single query context for metrics/charts/export.
- Three charts retain stable aspect/height constraints; tooltips show source timestamp and exact metric value.
- Rust events update at five-second cadence; the page does not synthesize data with a local timer.

### Rankings

- Gift, danmaku and captain tabs share table structure but use their approved metric semantics/format.
- Top-three rank emphasis is always visible, not hover-only. Movement uses arrow plus signed position count and a temporary background accent.
- Rust events update at ten-second cadence; UI throttles paint without changing data order.

### Export

- Range mirrors the invoking page. Type checkboxes are dashboard/ranking only.
- Path selection invokes a system dialog and stores an opaque grant; the display path is readonly informational text.
- Export is disabled while no type/path is selected or an export is pending. Cancellation and system-dialog cancellation cause no write.

## Styling Direction

- Use existing Element Plus controls for semantic behavior and Tailwind CSS-style utilities for authored layout/spacing/color adjustments.
- Prefer borders/dividers over shadow-heavy cards. Repeated event rows and table rows remain flat scan surfaces.
- Use stable grid/flex tracks and explicit min/max heights to prevent arrival/update layout shifts.

## Tailwind CSS-Style Styling Constraints

- New/changed authored styling uses Tailwind CSS-style utility classes only.
- No new scoped CSS, CSS modules, Sass/Less, inline style objects or non-utility semantic class names.
- Conditional class bindings express only small states such as selected, increasing/decreasing, paused or error.
- Existing ECharts option objects may define chart rendering colors because they are library configuration, not DOM authored styling.

## Class Length And Structure Strategy

- Keep each class value within normal formatter width. Split dense row markup into approved named components rather than multi-line class strings.
- Reduce ornament before adding more utilities.
- Never hide long classes in constants, maps, computed values, helpers or imports.
- Stable outer dimensions live on the semantic component root so loading/empty/data states share geometry.

## Interaction Skeleton

- Keyboard order follows toolbar left-to-right, then content/table, then footer. Enter submits valid dialogs; Escape cancels and restores focus to the trigger.
- Search/filter/tab/TOP changes do not steal focus or trigger network calls except source-approved range queries.
- Connect/disconnect, refresh, mutation/import/export commands disable only their owning control while pending and reject duplicate submission.
- Sensitive delete/blacklist remove require confirmation; clear screen does not add an unsourced confirmation and instead uses the exact retained-log success feedback.
- Usernames are semantic buttons/links with focus state. Medal tooltip is available on hover and keyboard focus.
- Ranking and dashboard event updates preserve scroll/focus and ignore stale range responses.

## State And Feedback Model

| State | Presentation/behavior | Source |
| --- | --- | --- |
| Disconnected/connecting/connected/reconnecting/error | fixed status location, text plus color; connection command reflects allowed next action | PRD 9.1; module state list |
| Reconnecting | amber state with attempt progression bounded to 10; ingestion buffer remains Rust-owned | PRD 9.1.2/9.1.6 |
| First/section load | stable skeleton rows/metrics/charts; controls with dependent data disabled | source-derived presentation of approved async operations; no new copy |
| Danmaku empty | empty panel without invented guidance | PRD allows empty state through Vue responsibility; copy `暂无弹幕` is existing only and not authority, so final copy is missing-source |
| Filtered empty | blank result region distinct from total counters; exact copy missing-source | source-derived state; copy missing-source |
| Dashboard/ranking empty | `暂无数据` | PRD 9.9.6/9.10.6 |
| Database degraded | current in-memory data remains; historical unavailable feedback | PRD 9.9.6/9.10.6 |
| Import compiling | determinate progress when Rust reports it; mutation controls disabled | PRD 9.3.6 |
| Import failure | line/reason and rollback result; no partial table mutation | PRD 9.3.6/9.5.6 |
| Export pending/success/error | export control pending; `export_done`; no-data/disk/permission failure | PRD 9.11.2/9.11.6 |
| Mutation success | refresh authoritative row/list; exact generic toast copy remains missing-source unless PRD states it | approved mutation outcome; copy missing-source |

Updates are pessimistic for persisted safety data/export and event-driven for live streams. No optimistic database write is designed.

## UX Writing And Feedback Copy Stance

- Preserve exact PRD copy: `房间号无效或未开播`, `开放平台凭证失效，请重新配置`, `重连失败，请手动重试`, `已清屏，日志仍保留`, `暂无数据`, `暂无数据可导出` and the named controls/labels.
- Validation messages identify the source field/rule but exact wording beyond PRD is `missing-source`; spec may define stable technical error codes, not invent user copy.
- Destructive confirmations exist because PRD explicitly requires them; final confirmation sentence is `missing-source` and must not be authored from convention.
- No help panel, keyboard-shortcut copy, upgrade prompt or explanatory marketing text is added.

## Responsive Behavior

- Primary target remains desktop 1280x800, minimum 1024x680.
- At minimum width, toolbar controls wrap into two compact rows without overlapping; the primary action remains right-aligned and visible.
- Danmaku metadata collapses to two rows while content retains full available width; time remains visible.
- Safety/ranking tables keep critical identity/value/action columns and use horizontal scrolling for the rest; action cells remain fixed only if Element Plus can do so without overlap.
- Dashboard metrics wrap 5 -> 3+2; charts stay stacked with a minimum 280px height.
- Dialogs use viewport-bounded width/height and internal scrolling; footer actions remain visible.

## Accessibility And Keyboard Considerations

- All commands use native Element Plus button/input/select/tab/dialog semantics and visible focus rings.
- Identity, connection, movement and error states include text/icon in addition to color.
- Minimum interactive target is 36px desktop; icon-only controls expose accessible names/tooltips.
- Dialog open moves focus to the first meaningful field; cancel/success restores trigger focus.
- Live event arrivals do not move keyboard focus and are not announced item-by-item; status changes may use one polite announcement boundary.
- Charts have adjacent textual metric titles/values; canvas is not the only carrier of core data.

## Usability Heuristics And Risk Controls

- High event volume: virtual/incremental rendering, stable keys, bounded visible rows and separate Rust persistence prevent browser lockup/data loss.
- Pause ambiguity: the control changes state while counters continue, making clear that ingestion is not paused.
- Clear ambiguity: exact retained-log feedback prevents users mistaking panel clear for data deletion.
- Dense identity metadata: consistent row columns and compact badges preserve content scanning.
- Rank flicker: one ten-second render cadence and 3-second highlight; no per-event resort animation.
- Stale range data: latest-range-wins/cancellation prevents a slow older query replacing current context.
- Destructive safety changes: confirmation and pessimistic update prevent accidental removal/deletion.
- Path trust: display path is readonly; Rust grant, not visible text, authorizes writes.

## Component Reuse And Split Strategy

| Candidate | Decision and trigger | Callers | State/contract | Styling/test boundary |
| --- | --- | --- | --- | --- |
| `LiveEventStream` | extract; single-caller interaction/performance lifecycle boundary | 1 (`DanmakuPanel`) | receives normalized immutable rows; owns viewport/pause scroll only; emits username select | virtual rendering, focus/scroll tests |
| `AudienceIdentity` | extract feature-shared; same identity semantics | 2+ danmaku/user detail/ranking | precise tag/name/medal props; emits user select only where enabled | identity color/text/fallback tests |
| `AudienceDetail` | extract feature-shared; same user-inspection action | 2+ danmaku/ranking | receives normalized detail; emits add-blacklist/close | dialog/popover focus and permission tests |
| `DataRangeControl` | extract feature-shared; identical approved ranges | 2+ dashboard/ranking/export | range value and `update:range`; custom range payload; no request ownership | range/keyboard tests |
| `DataExportDialog` | extract feature-shared; identical export lifecycle | 2 dashboard/ranking | local selection/focus state; emits submit/cancel; page/store owns command | grant/cancel/disabled/focus tests |
| `StatMetric` | reuse/adapt existing `StatCard` only if it accepts precise label/value/unit without domain logic | 5 same-semantics metrics | presentational props, no store | stable dimension/empty tests |
| `RankingTable` | extract; cohesive business table and focused update-highlight tests | 1 page, 3 same-semantic tab datasets | normalized rows; emits user select; no query ownership | TOP100/render/highlight tests |
| `SensitiveWordDialog` | extract; single-caller form lifecycle/validation/focus boundary | 1 | local form snapshot; emits submit/cancel | validation/pending/focus tests |
| `BlacklistDialog` | extract; single-caller form lifecycle and duplicate-update branch | 1 | local form snapshot; emits submit/cancel/update-reason | validation/duplicate/focus tests |
| Page toolbars/table footers | keep inline | 1 each | simple page-owned filters/counts | extraction would be one-caller pass-through |

Parent page/store remains the request, route, server-state and cross-section orchestration owner. Components never receive raw Rust DTOs or own adapter normalization. No slots are needed for the approved candidates.

## Design Risks

- The current global shell does not match PRD navigation/toolbar; this module needs temporary integration that application-shell later owns and removes.
- Existing Element Plus pages mix card-heavy surfaces and long classes; implementation must avoid unrelated visual churn while achieving the approved flat data layout.
- TOP100 and 1000 events/s require measured virtualization/update boundaries; decorative transitions could violate performance.
- Some exact user-facing validation/confirmation copy is absent from the PRD and cannot be invented.
- External Bilibili credential/API details remain unavailable; connection error states must remain real unavailable/degraded outcomes until configured.

## Confirmed UI Decisions

- `confirmed-decision` (2026-09-14): blacklist import uses UTF-8 CSV columns `uid,username,reason`; export appends `created_at`.
- `confirmed-decision` (2026-09-14): every sensitive-word hit is blocked from display, TTS and AI, while the filter log is retained; no action selector is added.
- `confirmed-decision` (2026-09-14): captain contribution ranks accumulated captain purchase/renewal amount in CNY.
- `confirmed-decision` (2026-09-14): chart point detail presents the five dashboard metrics for that timestamp.
- `confirmed-decision` (2026-09-14): concise Chinese system copy may be authored for unspecified empty, confirmation and success feedback while preserving exact PRD wording where supplied.

## Open UI Questions

- None. Missing production Bilibili credential details remain an external integration condition, not a UI decision; unavailable/degraded behavior remains mandatory until configuration exists.
