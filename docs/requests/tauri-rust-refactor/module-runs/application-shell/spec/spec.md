# Application Shell Specification

## Delivery Unit Identifier

- request: `tauri-rust-refactor`
- module: `application-shell`
- stage: `spec`
- self-healing iteration: `7`
- architecture-design_required: `true`
- specification status: framework approval candidate

## Background And Goals

The current Vue root implements only toolbar/navigation/content, duplicates connection status, hard-codes Chinese labels and has no PRD status bar. Settings mixes unrelated module forms, writes browser downloads/pasted JSON and persists UI configuration in `localStorage`. This module replaces those shell/configuration ownership gaps with the PRD four-region desktop experience, typed native configuration operations, complete Chinese/English shell copy and build/member-aware navigation.

Goals:

1. Make the 1280x800 default and 1024x680 minimum shell stable at normal/high DPI with toolbar, navigation, content and status regions.
2. Make room control, route visibility and seven global status values reflect authoritative stores/native state without duplicate writable owners.
3. Make free/member visibility a closed typed projection; free builds contain no member label, entry or command.
4. Deliver system settings for configuration management, locale and interface/layout preferences through Rust/Tauri boundaries.
5. Remove shell-owned browser file/localStorage authority and complete in-scope i18n without redesigning downstream feature pages.

## In Scope

- Root main-window shell, toolbar, dynamic navigation, routed content container and status bar.
- Main-window default/minimum size, zoom/fullscreen/high-DPI responsive behavior already supported by the desktop shell.
- Source-defined free/member navigation and toolbar visibility projection.
- System settings workspace categories `配置管理`, `语言`, `界面`.
- Selected configuration export, native JSON import, reset confirmation, config location and last backup display.
- Simplified Chinese/English/system locale behavior and Rust error-code localization boundary.
- Theme, accent color, opacity, font size, default/custom layout and danmaku display preferences.
- Typed shell/config/locale/UI DTOs, commands/events, adapters, store ownership, tests and owned legacy deletion closure.
- Shell extension inputs for downstream membership and notification owners, without implementing their product flows.

## Out Of Scope

- Member login, payment, order, token, module download, AI/ASR and moderation workflows.
- Notification history schema, read/unread rules, notification actions or empty-state copy.
- Update and plugin business implementation, migration/backup disaster recovery beyond the current configuration UI contract.
- New landing/onboarding/help/marketing pages, cloud sync, themes marketplace, custom fonts or additional layout widgets.
- Any send-danmaku control or command.
- Restyling feature pages beyond the root content boundary and interface preference projection.
- Editing the PRD or retaining Electron/browser compatibility paths.

## Trigger And Start Conditions

- Trigger: user requested continuing the PRD-driven Tauri/Rust refactor.
- Required upstream gates are present: request, PRD snapshot, requirement analysis/map, `application-shell` module split and approved page design.
- Previous `platform-foundation`, `live-data-safety` and `interaction-media` modules are completed. Room and recording snapshots are available through their stores.
- Build channel is available from the compile-time channel contract; real member state remains an external/downstream producer.

## Requirement Split Summary

- Current module is sequence 4 of 6 and owns PRD base UI/multilanguage plus configuration UI.
- Original authoritative sections are PRD 8, 9.17, 9.20 and 9.21. The module file's `16/19/20` references are feature numbers; they do not replace the original section numbers.
- Dependencies: typed desktop gateway and Rust configuration repository; live-data room/metrics projection; media recording state.
- Downstream: `membership-intelligence` fills real member state/routes; `extensions-release-resilience` fills notification/update/plugin and complete backup/recovery behavior.

## Source Grounding And Traceability

| Spec item | Label | Product-facing | Original authority | Plan obligation |
| --- | --- | --- | --- | --- |
| four-region shell and dimensions | source-backed | yes | PRD 8 | window config + responsive shell task |
| toolbar room/connect/member/notification/settings | source-backed | yes | PRD 8.6 | toolbar component and behavior tests |
| exact free navigation | source-backed | yes | PRD 8.1 | typed navigation map and route integration |
| four member visibility states | source-backed | yes | PRD 4.3, 8.2-8.5, 8.8 | member-only projection contract/tests |
| seven status fields and color meanings | source-backed | yes | PRD 8.7 | status DTO/view model/component tests |
| configuration location/backup/scope/actions | source-backed | yes | PRD 9.17.3-9.17.5 | typed native configuration flow |
| transaction/compatibility failure behavior | source-backed | yes | PRD 9.17.6-9.17.7 | fail-closed Rust/import tests |
| Chinese/English/system locale and fallback | source-backed | yes | PRD 9.20 | locale contract, inventory and tests |
| theme/accent/opacity/font/layout/danmaku display | source-backed | yes | PRD 9.21.3-9.21.5 | UI preference contract/form/projection |
| displayed defaults: dark, #FF6699, 0.95, medium, default, show three fields, 1000 | source-backed | yes | PRD 9.21.3 wireframe | default/fallback tests |
| light/dark/system preservation | source-backed + code-fact-backed | yes | request regression goal; current `AppConfig` | migrate valid existing theme modes |
| finite small/medium/large font normalization | source-backed + code-fact-backed | yes | PRD 9.21 size dropdown with `中`; existing numeric `fontSize` | preserve `14px=medium`; validate closed values |
| fixed-position resizable four-region custom layout | source-backed + source-derived | yes | PRD 8 fixed regions + 9.21 drag editor | no panels added/removed/relabelled |
| typed adapter/store/root runtime | code-fact-backed | no | current platform/store convention | architecture owns topology |
| grant-id-only config file operations | source-backed + code-fact-backed | no | PRD 9.17 and 10.1; PathGrantStore | extend path purposes, no raw path authority |
| bounded metrics update mechanism | code-fact-backed | no | high-frequency store/runtime constraints | architecture selects event or bounded poll |
| notification content deferred | missing-source/downstream, not approved behavior | no | PRD only names the trigger/surface | shell provides open/focus boundary only |

Closed-world product content equals the page-design allowlist. Any additional field, copy, action, permission, navigation target, status meaning, analytics event or acceptance behavior is forbidden until upstream confirmation.

## Workflow Efficiency Profile

- level: `S3 broad`; this is the lowest safe profile because the shell crosses Rust DTO/IPC, build capabilities, root lifecycle, Pinia, router, i18n, native window state and multiple settings flows.
- artifact density: `full`.
- downstream context: target/root/config/locale/route contracts, their direct callers/consumers, tests and completed-module public stores; do not reload unrelated feature internals.
- Upgrade condition: none exists above S3. Architecture must roll back to spec if member/downstream contracts require new visible semantics.

## Self-Healing Completion Bar

- Requirement consistency: exact four regions, labels, visibility matrix, settings fields/defaults and forbidden free-build member content.
- Business logic: room normalization, navigation projection, status mapping, locale fallback, UI validation, dirty/persisted state, import/export/reset transactions and capability changes.
- Syntax/tooling: strict TypeScript, zero-error ESLint, Vitest, Rust fmt/test/strict Clippy, free/member Web and Tauri builds.
- Runtime: no overlap at 1024x680/1280x800 and high DPI; no focus loss/layout shift from status events; async pending/failure/cancel/stale handling; listener/timer/dialog/window cleanup.
- Organization: one root listener owner; no page Tauri import; no browser Blob/download/pasted JSON/localStorage config authority; no stale route/i18n/generated/test/mock/caller.
- Maintainability: explicit typed states, pure navigation/status/config mapping, no generic manager/mega-settings DTO/hidden class string/one-caller helper file.
- Technical-debt blockers: any free-build member string/code, raw path authority, duplicate writable UI config, fake member/notification success or retained Electron settings chain must be fixed in scope.
- Completion requires `self-healing loop compliance: pass`, `self-healing loop assessment: pass` and remaining blockers `none`.

## User Intent Contract

- Literal request: continue refactoring the project according to the named PRD.
- Practical goal: a coherent desktop application, not disconnected migrated pages; the shell must make already migrated capabilities usable and accurately expose application state.
- Success: responsive four-region shell, accurate free/member navigation contract, native configuration flow, immediate/persisted interface/locale behavior, full in-scope bilingual copy and dual builds.
- Forbidden interpretations: visual-only shell; runtime-only hiding of free-build member features; browser/localStorage file/config authority; placeholder member success; generic settings dumping ground; removing existing free workflows to simplify navigation.
- Acceptable approach: typed Rust/TS contracts, one shell store/runtime, pure view-model maps, focused SFC regions and fail-closed downstream extension inputs.
- Verify/review: compare original matrix and field semantics; scan build outputs/source for member leaks; inspect full entry->store->desktop->Rust and cleanup chains.

## User Flow

### Start And Operate

1. Rust bootstrap loads validated UI/locale settings and returns build/system state.
2. Frontend applies locale/theme/accent/font/layout before revealing the stable shell; invalid values use source defaults.
3. Toolbar validates a trimmed room ID against the existing 1-20 ASCII digit contract and calls the room store.
4. Operator uses capability-filtered navigation; current route is highlighted.
5. Status bar projects room/rate/audience/recording plus Rust system metrics without becoming writable state.
6. Route/navigation/window exit disposes shell listeners/metric source; completed room/media runtimes retain their own owners.

### Manage Configuration

1. Load overview with display-only config location, last backup and available export scopes.
2. Operator changes export-scope checkboxes; at least one must be selected on export.
3. Export selects a native save file, passes only the grant id and selected scopes, and revokes the grant in `finally`.
4. Import selects a native JSON file, validates version/schema/scopes in Rust and applies one transaction. Cancel/no selection changes nothing.
5. Reset requires a second confirmation, performs one native reset and reloads all affected snapshots after success.

### Change Locale Or Interface

1. Load saved values into a local immutable draft.
2. Locale/theme/accent/opacity/font changes preview immediately through their approved projection; the saved snapshot remains unchanged.
3. Save validates the complete draft and commits atomically. Failure restores the last persisted projection.
4. System-language mode derives Chinese or English from native locale; unsupported/missing values use Chinese.
5. Custom layout edits only region sizes in fixed toolbar/nav/content/status positions, supports pointer/keyboard manipulation, and persists only on Save.

## Page And Module Design

- `App.vue`: composition/root lifecycle only; starts completed-module runtimes and shell runtime, supplies routed content.
- Toolbar/navigation/status are explicit page-design component boundaries with normalized view-model props and intention emits.
- Settings route owns category selection/orchestration; config, language and interface panels own their local drafts/lifecycle.
- Central content is unframed and `min-width: 0`; only task content scrolls by default.
- System settings is an unframed two-column workbench, not nested cards.
- Existing feature routes remain lazy. Route metadata becomes localized/capability-aware through one typed route/navigation source.

## UI / UX Design Contract

- Reading order: global action -> navigation -> current task -> system status.
- Primary action is connect/disconnect or the active settings Save. Secondary actions are selection/navigation/import/export. Config reset is separated danger.
- Toolbar is one stable row; status is one compact row. Live values use fixed groups/tabular numerals and never resize controls.
- Empty/loading/error/disabled/pending geometry remains stable. No source-less tutorial, success next-step, upgrade teaser or notification placeholder is added.
- Pessimistic native persistence; immediate preview only for language/interface values. Save failure restores persisted projection.
- Dialog/popover close restores trigger focus; disabled member entries expose reason in member build; free build contains no such element.
- Minimum 1024x680 is fully operable. Settings form columns collapse, checkbox/action rows wrap, and status may horizontally scroll under extreme zoom instead of overlapping.
- Semantic Element Plus controls, text+icon/color states, `aria-current`, polite low-frequency status announcements and keyboard layout editing are mandatory.

## Frontend Styling Constraints

- Authored styling uses visible Tailwind-style utilities only; no new scoped CSS, CSS modules, Sass/Less, inline style objects or semantic classes.
- Utility strings stay within formatter width and remain in templates. Do not move long class values into constants/maps/computed/helpers/imports.
- Conditional classes contain only small active/disabled/status toggles.
- Element Plus plus the current icon library remain the design-system surface; do not add a second icon package in this module.
- No cards as page sections, nested cards, gradients, decorative orbs, oversized headings or viewport-width font scaling. Letter spacing remains zero.
- Dynamic accent/theme projection may use one architecture-owned CSS variable sheet; no arbitrary per-node style binding.

## Field Display Semantics And Interaction Visual-State Contracts

| Field | Meaning/source | Display/precision/empty | Interaction and visual states | Evidence |
| --- | --- | --- | --- | --- |
| product name | shell identity, PRD 8 | localized fixed name; never empty | static, no hover/click | snapshot/i18n test |
| room ID | current room identity, PRD 8.6/8.7 | trimmed 1-20 digits; absent `--`; no numeric formatting | selector editable when not connecting; clear focus/error association | validation/component test |
| connection state | Rust room lifecycle, PRD 8.7 | `已连接/重连中/断开`; connecting/error retain explicit text | green/yellow/red support text; no click in status; toolbar action separate | mapper + a11y test |
| danmaku rate | current accepted events per second, PRD 8.7 | non-negative integer plus `条/秒`; absent `--` | read-only tabular default; no highlight/hover | store/status test |
| online audience | current room online value, PRD 8.7 | non-negative integer with locale grouping; absent `--` | read-only tabular default | formatter/status test |
| recording state | Rust recorder lifecycle, PRD 8.7 | exact `未录制/录制中/暂停`; unavailable/error remain localized degraded text | icon/text plus state color; not clickable; no persistent animation | mapper/status test |
| memory | current process memory, PRD 8.7 | binary unit with at most one decimal; unavailable `--` | read-only tabular | Rust/adapter/status test |
| CPU | current process CPU, PRD 8.7 | percent with at most one decimal; unavailable `--` | read-only tabular | Rust/adapter/status test |
| member control | verified member state, PRD 8.6 | absent free; login/open/date/renew for member states | navigational default emphasis; focus/hover/active; date is still clickable; expired button active; denied unavailable state is disabled | build/matrix tests |
| navigation item | feature destination, PRD 8.1-8.8 | exact localized label/icon; absent when forbidden | active persists as default route emphasis; hover/focus support; disabled only source-authorized expired capability | matrix/router/a11y test |
| config location | native managed config directory, PRD 9.17 | display path only; no raw value sent back; unavailable `--` | text read-only; `打开目录` is separate action | contract/security test |
| last backup | last successful backup moment, PRD 9.17 | localized date-time to seconds; never substitute current time; absent `--` | read-only metadata | formatter/overview test |
| export scope | categories included in export, PRD 9.17 | exact source labels; member categories absent in free | checkbox default from native overview; disabled only when scope unavailable | build/form tests |
| language | persisted/preview locale, PRD 9.20 | `简体中文` / `English`; unsupported -> Chinese | select focus/active; preview immediate; Save persists | locale integration test |
| auto system | native locale following, PRD 9.20 | on/off switch; no ambiguous null | when on, language displays derived value; permission not applicable | state test |
| theme | visual theme, PRD 9.21/current regression | light/dark/system; invalid -> dark source fallback | select preview; saved state distinct; normal focus/active | validation/projection test |
| accent | interface emphasis color, PRD 9.21 | canonical `#RRGGBB`; default `#FF6699`; invalid -> default | color swatch/picker, focus and visible text value | validation/contrast test |
| opacity | window/content opacity, PRD 9.21 | 0-1 value displayed to two decimals; default 0.95 | slider preview; keyboard arrows; native failure restores saved value | preview/rollback test |
| font size | UI scale, PRD 9.21/current config | small/medium/large, medium maps to preserved 14px | select preview; no viewport scaling | projection/responsive test |
| layout | fixed four-region geometry, PRD 8/9.21 | default/custom; invalid -> default | selector and editor; drag plus keyboard; cancel no commit | layout reducer/component test |
| danmaku display | show level/medal/time, max count, PRD 9.21 | booleans default true; max default 1000, positive bounded integer | checkboxes/number control; save applies to panel projection | settings/feed contract test |

Visited style is not applicable to buttons/menu navigation. All invalid native values fail adapter validation; template `??` may render only explicit absence, never hide invalid state.

## Function-Complete Behavior Breakdown

### AS-F01 Bootstrap And Root Shell

- Call `app_bootstrap`, shell preference reads and initial system snapshot once per root generation.
- Keep four-region geometry during loading/degraded states. A missing bridge surfaces the shared desktop-unavailable state without fake values.
- Apply validated saved projections once; ignore stale generation results after unmount/restart.
- Dispose shell event listeners and metric source on app exit/unmount.

### AS-F02 Toolbar

- Room input normalization: trim leading/trailing whitespace; whitespace-only becomes empty; only ASCII digits; 1-20 characters after trim; line breaks or other pasted characters invalid; validate on submit, clear field error on next valid input.
- Select/create room stays local draft/history; authoritative current room comes from room snapshot.
- Connect/disconnect delegates to room store; pending dedupes repeated action; error is localized and action remains retryable according to `DesktopError.retryable`.
- Notification/settings icon buttons have tooltips/labels. Member button is compile/capability projected and cannot exist in free output.

### AS-F03 Dynamic Navigation

- Build one typed navigation tree from route ids and shell capability input; do not infer permission in templates.
- Free: exact ten source groups/items and zero member label/route.
- Member unauthenticated/unpaid: free surface plus member center; no AI/moderation.
- Active: free surface plus AI, member center and moderation child under risk center.
- Expired: free surface, disabled AI and moderation, enabled renewal member center.
- If current route becomes unavailable, replace to `/connection`; do not leave forbidden page mounted.

### AS-F04 Status Bar

- Project room connection/room/online, accepted danmaku rate, recording status and Rust memory/CPU.
- Store only latest metric snapshot; no history. Invalid/negative/NaN data fails adaptation and preserves prior valid snapshot with degraded error.
- Status updates do not create toast spam, focus changes or layout changes.

### AS-F05 Configuration Management

- Overview fields: display path, optional last backup, available scopes, selected export scopes and operation status.
- Export requires at least one allowed scope, obtains `configExport` save grant, calls native export and revokes grant in `finally` on success/failure/cancel.
- Import obtains `configImport` grant, calls native validation/transaction, refreshes affected config/stores only after success and revokes grant in `finally`.
- Reset requires explicit second confirmation; one native transaction restores source defaults and returns affected scopes/revision.
- Browser Blob/object URL/anchor/pasted textarea and raw filesystem APIs are forbidden.

### AS-F06 Locale

- Values: `zh-CN`, `en-US`; `followSystem` boolean.
- System locale mapping recognizes Chinese as `zh-CN`, English as `en-US`, all unsupported/missing values as `zh-CN`.
- Changing selection previews immediately. `locale_save` atomically persists locale/mode; `locale_set_auto` handles the source-named toggle semantics.
- Every in-scope shell/settings/validation/error key exists in both files; missing key/placeholder falls back Chinese and emits a bounded Rust/frontend log, not a user loop.

### AS-F07 Interface And Layout

- Draft fields: theme, accent, opacity, fontSize, layout and three danmaku display booleans/maxEntries.
- Validate complete draft before preview/save. Accent canonicalization is trim + uppercase `#RRGGBB`; whitespace-only invalid. Numeric text trims before integer validation; line breaks/non-digits invalid.
- Theme/accent/font and danmaku presentation preview in Vue; native opacity preview uses typed desktop command. Save persists complete snapshot; failure restores saved projection.
- Custom layout is a versioned fixed-position four-region size model. Pointer and keyboard edits operate on a draft; cancel discards; invalid/corrupt saved layout uses default.
- UI reset restores PRD defaults; it is distinct from whole-configuration reset.

### AS-F08 Notification And Member Extension Inputs

- Shell owns open/close/focus and a typed content slot only.
- No notification item/copy/actions are created in this module.
- Member build accepts a validated member-shell projection; unavailable producer cannot be converted into logged-out success. Real workflow/routes arrive in the downstream module.

## Expert Frontend Engineering Constraints

- State ownership: Rust owns persisted UI/locale/config and system metrics; completed stores own room/feed/recording; shell store owns latest normalized shell snapshot; settings panels own drafts; router owns route.
- Data lifecycle: serde DTO -> desktop contract -> adapter validation -> immutable view model -> store snapshot -> component props; draft -> validated payload -> native result -> refreshed snapshot.
- Async: one bootstrap generation, latest-wins reads, per-operation dedupe, `finally` grant cleanup, stale response rejection and no automatic retry beyond retryable user action.
- Interaction: semantic controls, disabled reason, destructive config confirmation, cancel/no-op guarantees, focus restore and keyboard layout editing.
- Performance: one root metrics subscription/bounded poll, latest-value only, no deep global watch, no metric history, lazy feature routes and no new large dependency.
- Evolution: downstream member/notification producers attach through typed inputs; no generic plugin registry or compatibility event bus.
- Diagnostics: structured codes for invalid config/import/version/layout/locale/window/system metric states; tests cover unavailable bridge and malformed DTOs.

## Frontend Architecture Design Routing Inputs

- `architecture-design_required: true`.
- Immutable behavior: exact product content/matrix, four regions, seven statuses, settings fields/defaults, native path boundary, locale fallback, preview/persist distinction and acceptance IDs.
- Architecture must decide: shell/config capability owners, Rust application/repository/OS ports, command/event grouping, state topology, root lifecycle, navigation source, theme variable projection, fixed-layout serialization, downstream extension seams, migration and deletion triggers.
- Code facts: current `App.vue` is a root orchestrator; `config.ts` uses localStorage; settings imports stores directly and Blob-downloads; base desktop gateway and config repository already exist; room/media stores are stable inputs.
- Roll back to spec if architecture changes visible labels/options, member matrix, layout meaning, validation, API semantics, acceptance or module scope.
- Later verification must include `frontend architecture quality compliance: pass` for ownership, dependency direction, budgets and cleanup.

## Production Code Quality Constraints

- Type first: exact Rust/TS DTO mirrors and discriminated status unions; keep `RoomId`, `PathGrantId`, `ConfigKey` nominal distinctions; no broad `any` or unsafe cast in scope.
- Fail fast: malformed persisted JSON, invalid enums/numbers/colors/layout, missing bridge, cancellation, transaction failure, stale metrics and impossible member state become typed outcomes.
- Null: `null` denotes explicit absent source value; omitted/undefined is transport absence only. Optional chaining cannot replace adapter validation.
- Pure functions: navigation projection, locale resolution, validation, formatters, layout reducer and DTO mappers. Classes/factories are rejected.
- Naming: `is/has/should/can` booleans; `handle` UI events; `validate/format/resolve/map/build` pure operations.
- Constants: defaults and bounds live under the narrow UI domain owner; one-off template values stay local; classes never hide in constants.
- Optimization only for real high-frequency metrics/navigation computation; no speculative memo/cache/virtualization.
- Every async form/list/surface has explicit loading/error/empty/end conditions from this spec.

## Human Review Readiness Constraints

- Follow current `platform/desktop`, Pinia setup-store, root composable, Vue SFC and Vitest conventions.
- Allowed change clusters: root shell/router/i18n/config/settings/UI components/stores/composables; typed base/shell contracts/adapters/tests; Rust UI/config/metric domain/application/infrastructure/IPC/capabilities/window config/tests; owned declarations and legacy settings cleanup.
- Do not change PRD, feature-page business behavior, membership/AI/moderation/update/plugin implementations or completed room/media contract semantics.
- Review-sensitive: root lifecycle, capability leaks, shared i18n, native config transaction, high-frequency status, theme projection and removal of browser/localStorage paths.
- Changelog maps every hunk to AS task/source/cleanup. End with exact callers/imports/events/timers/watchers/routes/i18n/tests/mocks/comments scan.
- Required evidence: focused/full tests, strict checks, free/member Web/Tauri builds, 1024/1280/high-DPI screenshots, keyboard/focus run, free-build member-string/surface scans and skipped external-path reasons.

## Functional-Programming Constraints

- Pure: `resolveLocale`, `validateUiPreferences`, `mapShellStatus`, `buildNavigationTree`, `formatSystemMetric`, config-scope filtering and layout reduction.
- Immutable inputs: serde DTOs, store snapshots, component props, route descriptors and form drafts passed to validators.
- Effects only in Rust command/application/port/repository, desktop transport, Pinia action, root lifecycle or component preview lifecycle.
- DTO semantic normalization occurs in adapters, not template/computed fallback. Writable state never duplicates derived navigation/status/form validity.
- No functional utility dependency, currying framework or reducer abstraction beyond direct readable functions.

## Architecture Reuse And Shared Ownership Constraints

| Candidate | Semantics/layer/sites/variation | Required architecture decision |
| --- | --- | --- |
| desktop errors/invoke/listen | same infrastructure semantics; many sites; stable | reuse existing transport and normalization |
| room/recording status | domain-owned completed stores; shell is one projection | consume, never copy or create new writable owner |
| navigation permission | one shell tree; varies build/member state | feature-local pure owner; do not share with router guard until architecture proves one semantics |
| locale resolution | shell, Element Plus and error copy share exact locale | one UI-domain owner with multiple callers |
| UI preference validation | Rust persistence and TS draft need exact equivalence | authoritative Rust rule + TS mirror/contract tests; no generic config validator |
| display formatters | status/settings have multiple same-semantic uses | reuse/promote domain-specific formatter only with 2+ same-semantic production callers |
| settings panels | different config/locale/interface semantics | keep separate; no optional mega-form/store |
| capability projection | build/member/downstream variation | typed adapter/record, not generic RBAC framework |

Commonality classification: desktop/metrics are infrastructure; navigation and UI validation are domain; shell regions are UI; defaults are narrow configuration constants. Shared APIs require `@see` or representative `@example`, migration of all in-scope callers and behavior-equivalence tests. Final extract/reuse/keep-separate decisions belong to architecture design.

## Locality And File-Extraction Constraints

- Root lifecycle receives one dedicated `useApplicationShellRuntime` only because it owns event/listener/metric cleanup and focused tests.
- Shell DTO mapper/validator may use a feature-local adapter file due native contract isolation and independent malformed-input tests.
- Navigation/status formatting stay in a feature-local domain module only if used by component plus tests/store; no generic `utils.ts`.
- Single-panel handlers/validators remain in the owning SFC unless also required by Rust/adapter equivalence or focused independent tests.
- No single-function file created to shorten `App.vue` or settings.

## Vue Component Extraction Constraints

| Candidate | Decision | Count/exception | State/contract/test |
| --- | --- | --- | --- |
| `AppShellToolbar` | extract | 1; cohesive business section + focused test | normalized props, intention emits, no transport/store; matrix/pending/focus tests |
| `AppShellNavigation` | extract | 1; focused permission/tree test boundary | typed entries/current path, navigate emit; no permission calculation in SFC |
| `AppShellStatusBar` | extract | 1; cohesive high-frequency status section | seven display props, no emits; semantics/geometry tests |
| `NotificationSurface` | defer | downstream slot/composition owner absent | shell open/focus local; no content contract now |
| `SettingsWorkspace` | keep inline | 1; pass-through container blocker | route owns category/orchestration |
| `ConfigManagementPanel` | extract | 1; cohesive native file/reset lifecycle | snapshot props/action emits/local scope draft; cancel/rollback tests |
| `LanguageSettings` | reuse/adapt | 1; existing cohesive locale owner | draft props/emits; remove direct persistence; locale tests |
| `InterfaceSettingsPanel` | extract | 1; cohesive form + focused test | normalized saved props; preview/save/reset emits; form tests |
| `LayoutEditorDialog` | extract | 1; interaction lifecycle boundary | local draft/focus; save/cancel emits; pointer/keyboard tests |
| individual rows | keep inline | 1; one-off/pass-through blocker | panel-owned local markup |

No extracted SFC receives a backend DTO/store object, calculates permission, invokes transport or hides long classes.

## Design Constraints

- Preserve page-design hierarchy, density, four-region geometry, unframed settings and no nested cards.
- One root owner for shell listeners/metrics and one persisted owner for UI/locale config.
- Navigation and capabilities are a closed typed tree, not scattered `v-if` or route-name string checks.
- Browser and Electron compatibility APIs are forbidden in the new chain.
- Member/notification extension points cannot expose product content before downstream source/contract arrives.
- No speculative abstraction, runtime feature-flag fallback to old owners or generalized settings/plugin framework.

## Project Bootstrap And Scaffold Decision

Not applicable: this module reuses the approved existing Vue/Vite/Tauri v2 application and platform-foundation bootstrap. It may not create another app, package, route root or Electron compatibility scaffold.

## Change Axes And Pattern Decision

- Signals: state/build/capability selection; Rust-to-TS shape adaptation; ordered bootstrap/preview/save/import workflow; root event cleanup; state-dependent navigation; access control.
- Existing patterns: typed desktop facade, adapter, Pinia setup store, root lifecycle composable and presentational SFC boundaries.
- Decision depth: Level 2 reused/formalized project patterns plus Level 1 pure typed records/reducers.
- Selected decision: `reuse existing pattern` and `adapt lightweight pattern`.
- Syntax: pure functions/typed records/discriminated unions, Pinia actions, one composable, adapters/mappers and Vue props/emits.
- Rejected: class shell manager, generic settings repository, RBAC framework, event-bus package, component registry, factory/strategy hierarchy and compatibility facade. The closed states do not justify their indirection.
- Fit evidence: command/DTO parity, one listener owner, matrix tests, no scattered permission branches and no new pattern layer with one variant/caller.

## Code Context And Impact Assumptions

- Vue graph remains unavailable; use scoped caller/import/event/route/i18n scans plus TypeScript/Vite/Rust compiler graphs.
- Current chain: `App.vue` -> room/update stores + inline menu; settings -> direct stores/localStorage/browser Blob; router has static hard-coded metadata.
- Target chain: root -> shell runtime/store -> typed desktop -> Rust UI/config/metrics; pure navigation projection -> router/components; settings drafts -> store -> typed commands.
- Direct impacted files include App/router/main/i18n runtime/locales/config/settings and their tests; Rust base IPC/config repository/lib/capabilities/window config and tests.
- Neighboring plugin/update/AI/ASR/member code must not be rewritten except removal of an in-scope shell/settings caller after its current route/category boundary is preserved.
- Post-change proof: exact scan for `localStorage` config, `new Blob`, `createObjectURL`, `window.electronAPI`, direct `@tauri-apps/api`, hard-coded in-scope Chinese and stale config store callers.

## API And Data Contracts

### Public Surface Budget

- New common shell commands: exactly 13 maximum: `shell_status`, `config_overview`, `config_open_directory`, `config_export`, `config_import`, `config_reset`, `locale_get`, `locale_set_auto`, `locale_save`, `ui_get`, `ui_preview`, `ui_save`, `ui_reset`.
- New common shell events: at most 3: `shell_status_changed`, `locale_changed`, `ui_preferences_changed`.
- Reuse existing `path_select/path_revoke` with new purposes `configImport` and `configExport`; no new generic file command.
- Free/member common surface is identical. Member-only state source stays behind the member feature and is not added to the common command list here.

### Core Types

```ts
type LocaleCode = 'zh-CN' | 'en-US';
type ThemeMode = 'light' | 'dark' | 'system';
type FontSize = 'small' | 'medium' | 'large';
type LayoutMode = 'default' | 'custom';
type MemberShellStatus = 'unavailable' | 'loggedOut' | 'unpaid' | 'active' | 'expired';

interface SystemMetrics {
  memoryBytes: number | null;
  cpuPercent: number | null;
  observedAt: number;
}

interface LocalePreferences {
  locale: LocaleCode;
  followSystem: boolean;
}

interface FourRegionLayout {
  version: 1;
  navigationWidth: number;
  toolbarHeight: number;
  statusHeight: number;
}

interface UiPreferences {
  theme: ThemeMode;
  accentColor: string;
  opacity: number;
  fontSize: FontSize;
  layoutMode: LayoutMode;
  customLayout: FourRegionLayout | null;
  showDanmakuLevel: boolean;
  showDanmakuMedal: boolean;
  showDanmakuTime: boolean;
  maxDanmakuEntries: number;
}
```

- Rust serde camelCase is authoritative; TS mirrors exact names/nullability.
- `SystemMetrics` null means OS metric unavailable; zero is valid.
- `customLayout` must be null for default mode and present/valid for custom mode.
- Architecture sets technical size bounds sufficient to preserve 1024x680 without changing region inventory; contract tests freeze them.

### Configuration Operations

- `config_overview(undefined) -> { displayPath, lastBackupAt, scopes[] }`; `lastBackupAt` nullable, scopes are typed ids with localized labels supplied by frontend i18n.
- `config_open_directory(undefined) -> { opened: true }`; Rust owns actual directory and accepts no path.
- `config_export({ grantId, scopes }) -> { revision, displayPath }`; non-empty allowed scope set; Rust writes versioned JSON.
- `config_import({ grantId }) -> { revision, importedScopes }`; full validation then one transaction; incompatible/damaged returns structured error and no mutation.
- `config_reset(undefined) -> { revision, resetScopes }`; confirmation is UI-owned, mutation is one native transaction.
- Config bundle carries `version`, build-neutral common sections and optional member sections. Free build cannot produce/consume member settings as executable capability; unknown/member sections are validated/skipped or rejected according to the later resilience contract, never silently activated.

### Locale/UI Operations

- `locale_get -> LocalePreferences`; `locale_set_auto({ isEnabled }) -> LocalePreferences`; `locale_save({ preferences }) -> LocalePreferences`.
- `ui_get -> UiPreferences`; `ui_preview({ preferences }) -> UiPreferences`; `ui_save({ preferences }) -> UiPreferences`; `ui_reset -> UiPreferences`.
- Preview does not persist. Save/reset emit changed event after the repository commit. Failed preview/save leaves/restores prior persisted projection.
- `shell_status -> SystemMetrics`; event carries the same shape. Architecture chooses bounded event cadence and deterministic cleanup.

### Error Semantics

- Required codes include invalid UI preference/color/opacity/layout/font/max entries, invalid config scope/file/version, config transaction failure, config directory unavailable, system metric unavailable and window preview failure.
- Dialog cancellation is a non-error no-op in the store even if base transport represents it as `dialog_cancelled`.
- Errors use stable codes/details; frontend translates codes and does not display raw Rust/OS text.

## Frontend / Server Responsibility Boundary

- Vue: presentation, local drafts, field association, immediate visual preview, category/route interaction, typed adapter consumption and localized feedback.
- Rust: system locale/metrics/window effect, persistence, validation authority, config bundle parsing/encryption/transaction, config directory, path grants and changed events.
- Shared: exact DTO/enums/error codes/defaults and build-channel capability contract.
- Member producer and notification content are downstream/external pending. Shell accepts validated typed input and never manufactures authenticated/active state.
- Adapters normalize serde DTO to view models; pages/templates never parse raw dates/bytes/enums or raw paths.

## Context And Dependency Sources

- TypeScript: `tsconfig.eslint.json` -> strict `tsconfig.json`; bundler resolution, ES2020/DOM, `@/*`, Vue SFC/generated declarations and noUnused govern all touched frontend files.
- Contract source: Rust serde DTOs/registries and the current typed desktop contract mirror; no OpenAPI/protobuf applies.
- UI libraries: Vue 3, Pinia, Vue Router, vue-i18n, Element Plus, Tailwind utilities.
- Native: Tauri v2, SQLite `t_app_setting`, PathGrantStore, platform system/window APIs.
- Required declaration closure: `src/env.d.ts`, platform contract maps, generated `auto-imports.d.ts/components.d.ts`, locale JSON types/usages and store/component props.

## Edge Cases

- Web/no Tauri bridge: stable degraded shell, no fake system/member/config values.
- Bootstrap/config read failure: source defaults applied, error retained, free workflows remain navigable.
- Rapid connect clicks or route changes: request dedupe, latest generation wins, no duplicate listeners.
- Member state changes while on forbidden route: replace to connection and unmount forbidden surface.
- Free build receives crafted member state/event: adapter rejects/ignores; no route/component exists in bundle.
- Negative/NaN/overflow system values: reject and preserve last valid snapshot, display unavailable if none.
- Config dialog cancel: no error toast, mutation or leaked grant.
- Corrupt/incompatible/partial import: no current data overwrite; transaction rollback.
- Reset failure: persisted/draft projection remains previous; confirmation does not imply success.
- Unsupported system language or missing key/placeholder: Chinese/default text and bounded log.
- Invalid stored accent/layout/opacity/font/max entries: source defaults; no template fallback that hides error.
- Save/preview race: generation token prevents older response replacing newer draft; preview failure restores persisted projection.
- 1024x680, high zoom, long English/member expiry date: no overlap; bounded widths/wrap/scroll rules apply.
- DPI change while layout editor open: preserve draft/focus and recompute preview bounds.

## Acceptance Criteria

- `AS-A01`: main window defaults to 1280x800, enforces 1024x680 minimum and renders toolbar/nav/content/status without overlap at normal/high DPI, zoom and fullscreen.
- `AS-A02`: toolbar room/select/connect/disconnect/notification/settings behavior is complete; room trim/validation/pending/error semantics match the contract.
- `AS-A03`: status bar displays exactly seven source fields with correct semantics, units, null handling, text-plus-color state and stable high-frequency geometry.
- `AS-A04`: free build source/bundle/routes/commands contain no member/login/AI/moderation/renewal UI or callable member surface.
- `AS-A05`: member projection tests pass for logged-out, unpaid, active and expired toolbar/navigation matrices; unavailable never becomes false logged-out success.
- `AS-A06`: unavailable current routes are replaced safely and root navigation/listener lifecycle leaks no route, listener, timer or focus owner.
- `AS-A07`: config overview/path/backup/scope UI, native open/export/import/reset and required confirmation/cancel/pending feedback pass without browser file or raw path authority.
- `AS-A08`: damaged/incompatible/partial config import preserves existing config; successful import/reset is atomic and refreshes all affected snapshots.
- `AS-A09`: Chinese/English/system locale switches immediately, saves, restores and falls back Chinese for unsupported/missing pack/placeholder with fixed error placeholders.
- `AS-A10`: theme/accent/opacity/font/layout/danmaku display defaults, validation, preview/save/reset/rollback behavior match PRD and persist through Rust.
- `AS-A11`: custom layout edits only fixed four-region sizes, supports pointer/keyboard, survives DPI changes, cancels without commit and restores default on corruption.
- `AS-A12`: Rust/TS shell contracts, command/event counts, free/member registry/capabilities and adapter malformed-input tests pass.
- `AS-A13`: root store/runtime has one owner, uses completed room/media stores, bounds metrics and cleans listeners; pages/components import no Tauri/Electron/native file/localStorage config authority.
- `AS-A14`: all in-scope shell/settings/error/validation labels exist in both locales; screenshots show no clipping/overlap at 1024x680 and 1280x800.
- `AS-A15`: full Vitest/type/lint, Rust fmt/free+member tests/strict Clippy, free/member Web and Tauri installer builds pass; Vite remains live through native builds.
- `AS-A16`: notification/member downstream gaps are explicitly recorded; verification does not claim real login/payment/notification content success.

## Human Review And Handoff

- Changelog maps every change/deletion to AS task, PRD source or dependency closure.
- Reviewer receives command/event surfaces, navigation matrices, field/display table, component decisions, pre/post caller scans, screenshots and dual installers.
- Record exact command output counts, warnings, unavailable paths and any skipped native/manual check with reason.
- Handoff to `membership-intelligence`: member-shell projection/type and route slots, without embedded fake state.
- Handoff to `extensions-release-resilience`: notification content and expanded config backup/migration/update/plugin scopes.

## Risks

- Root-shell changes affect every route; mitigate with component contracts, lazy routes, current-route screenshots and full test/build.
- Free-build member leakage can occur through shared locale/routes even when hidden; require source/bundle/registry scans.
- Theme/opacity across native window, Element Plus and Tailwind can diverge; architecture must define one projection owner and rollback.
- Config import/reset can corrupt broad state; Rust transaction, versioned bundle, backup and refresh evidence are mandatory.
- Metrics can cause reactive churn/layout shift; latest-only bounded source and fixed display geometry are mandatory.
- Member/notification external gaps can tempt placeholders; fail closed and defer their product content.

## Framework Automatic Approval Checklist

- Template sections, full S3 density, source traceability, closed-world allowlist and page-design carry-forward: pass.
- Function-complete fields/flows/validation/state/feedback/display semantics: pass.
- User intent, production quality, expert frontend, styling, functional, review-readiness and self-healing constraints: pass.
- Pattern-fit, locality, Vue extraction and architecture-routing inputs: pass.
- Missing-source content is excluded or handed downstream; no blocking product decision remains under requirement-analysis decisions.
- Decision: approve spec and advance to `architecture-design` without changing `state.json.loop`.
