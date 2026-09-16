# Application Shell Page Design

## Delivery Unit Identifier

- request: `tauri-rust-refactor`
- module: `application-shell`
- stage: `page-design`
- self-healing iteration: `7`
- product scope: PRD sections 8, 9.17, 9.20 and 9.21; function numbers 16, 19 and 20 in the module split

## Page Objective

Deliver a quiet, work-focused desktop shell that keeps room control, navigation, current page and application health visible without competing with the live-operation task. Deliver a system-settings workspace for configuration transfer/reset, language and interface preferences. The shell must reflect build/member capabilities without inventing or leaking unavailable features.

## Target Users Or Scenarios

- Free-build operator: selects a room, connects, moves between all free tools, watches current connection/room/rate/audience/recording/resource state, and changes system settings without seeing membership language.
- Member-build operator before activation: uses the same free work surface and reaches the member center through the state-specific toolbar/navigation entry.
- Active member: sees enabled AI and moderation entries supplied by the member capability snapshot.
- Expired member: sees member capabilities disabled with renewal meaning while all free workflows remain usable.
- Any operator: switches Chinese/English, adjusts interface presentation, exports/imports selected configuration or restores defaults.

## Source And Upstream Constraints

- `artifacts/prd-snapshot.md`: closed-world 20-function scope, no send path, build-channel separation, Rust-owned native state and Chinese fallback.
- `requirements/modules/application-shell.md`: four-region shell, dynamic navigation/status, settings, theme/layout and locale boundary.
- PRD 8.1-8.8: exact four regions, size constraints, toolbar controls, seven status items and build/member visibility matrix.
- PRD 9.17: native configuration location, selected export scope, JSON import/export, transaction rollback and confirmed reset.
- PRD 9.20: Chinese/English, system-language mode, immediate preview, persistence and Chinese fallback.
- PRD 9.21: interface settings, configuration restoration, dynamic navigation, high-DPI/multi-window behavior and display controls.
- Existing code facts: `App.vue` currently owns a partial three-region shell with hard-coded labels and duplicated connection state; settings exports through a browser Blob, imports pasted JSON and persists core UI state in `localStorage`. These facts constrain migration/deletion only and do not authorize new product content.
- Previous module contracts: live-data stores own room/rate/audience snapshots; media store owns recording state; platform desktop transport is the only Tauri import boundary.
- Downstream boundaries: membership content/transactions belong to `membership-intelligence`; update/plugin behavior and full backup/migration resilience belong to `extensions-release-resilience`. This module provides only their source-backed navigation/slot surfaces.

## Product-Content Allowlist Mapping

| Proposed content | Grounding | Exact source | Design use |
| --- | --- | --- | --- |
| Brand `B站直播助手` and Logo | source-backed | PRD 8.1-8.6 | non-clickable first toolbar signal |
| Room selector; `连接` / `断开` | source-backed | PRD 8.6, 9.1 | primary global operation |
| Notification icon and notification-center surface | source-backed | PRD 8.6 | opens an anchored surface; entry schema/copy is not authored here |
| Settings icon and `系统设置` navigation | source-backed | PRD 8.1-8.6, 9.17, 9.20 | route entry |
| Free navigation labels | source-backed | PRD 8.1 | `直播间`, `弹幕面板`, `数据看板`, `排行榜`, `风控中心`, `提醒配置`, `语音播报`, `直播录制`, `插件中心`, `系统设置` |
| Member toolbar labels and member center | source-backed | PRD 8.2-8.6 | `登录`, `开通会员`, expiry date or `续费`; links to member center |
| AI and moderation visibility/disabled meaning | source-backed | PRD 4.3, 8.4, 8.5, 8.8 | capability-driven navigation only |
| Seven bottom status values | source-backed | PRD 8.7 | connection, room, danmaku rate, online audience, recording, memory, CPU |
| Connection colors and recording labels | source-backed | PRD 8.7 | green/yellow/red plus text; `未录制/录制中/暂停` |
| Main size and four regions | source-backed | PRD 8 | default 1280x800, minimum 1024x680, toolbar/nav/content/status |
| Configuration path/open directory | source-backed | PRD 9.17.3-9.17.5 | readonly opaque display plus native action |
| Last backup time | source-backed | PRD 9.17.3-9.17.4 | secondary metadata |
| Configuration scope checkboxes | source-backed | PRD 9.17.3-9.17.4 | room, sensitive words, blacklist, gift sound, UI layout, recording, plugin; member-only AI/moderation when compiled and available |
| Export/import/reset | source-backed | PRD 9.17.1-9.17.7 | native dialogs, JSON validation/transaction, two-step reset confirmation |
| Import failures | source-backed | PRD 9.17.6 | damaged/incompatible/transaction failure preserves current config |
| Language, auto-system and save | source-backed | PRD 9.20.1-9.20.7 | Chinese/English select, system toggle, immediate preview and persistence |
| Locale fallback | source-backed | PRD 9.20.6 | missing pack/placeholder falls back to Chinese/default localized copy and logs |
| Theme, accent, opacity, font and layout | source-backed | PRD 9.21.2-9.21.5 | interface settings; immediate theme/opacity preview, save and restore |
| Dark, `#FF6699`, 0.95, medium, default layout | source-backed | PRD 9.21.3 wireframe | initial/default presentation values where no stored valid config exists |
| Light/dark/system theme choices | source-backed + code-fact-backed | request regression goal; current `AppConfig.theme` | preserve existing valid choices while adding required UI |
| Font-size selector | source-backed | PRD 9.21.3 displays a font-size dropdown with `中` | closed selector centered on the source-displayed value; no arbitrary font upload/value or ungrounded option labels in page design |
| Custom layout editor | source-backed | PRD 9.21.2, 9.21.5 | opens drag editor limited to the existing four regions; no new panels/presets |
| Danmaku level/medal/time and max 1000 | source-backed | PRD 9.21.3-9.21.4 | presentation controls only |
| Layout/theme invalid fallback and DPI relayout | source-backed | PRD 9.21.6 | structured degraded state with default restoration |
| Save/reset completion and failures | source-backed | PRD 9.17, 9.20, 9.21 | pessimistic persistence; localized result feedback from existing error-code contract |

No additional dashboard, marketing, onboarding, cloud sync, send control, member teaser, upgrade prompt, notification item schema, theme marketplace, custom font upload or arbitrary layout widget is allowed.

## User Journey And Task Success

### Global Live Operation

1. App restores valid locale/interface/layout config and opens the four-region shell.
2. Operator selects or enters an approved room and uses the toolbar connection action.
3. Toolbar shows operation pending; the status bar remains the authoritative, text-labelled connection state.
4. Operator navigates through capability-allowed entries while the current route remains highlighted and the central content owns vertical scrolling.
5. Status updates do not move focus, resize the shell or interrupt page interaction.
6. Success is the requested route visible with current global state still readable; disconnect and route changes remain available under the approved live-data lifecycle.

Primary action is connect/disconnect. Room selection and navigation are secondary. No shell action sends content. Invalid room and native failures use the existing localized structured error contract.

### Configuration Management

1. Operator opens `系统设置`, selects `配置管理`, reviews the config location, backup time and export scope.
2. Export opens a native save dialog and writes selected categories through Rust.
3. Import opens a native file dialog; Rust validates before one transactional commit. Cancel changes nothing.
4. Reset opens the required second confirmation and restores defaults only after confirmation.
5. Completion is persisted configuration reflected in the page; failure preserves the previous snapshot.

### Language And Interface

1. Operator selects `语言` or `界面` inside system settings.
2. Language, theme and opacity preview immediately without hiding Save.
3. Save persists the draft; valid state is restored on next start. UI reset restores the PRD defaults.
4. Custom layout opens a focused drag editor for the existing four regions; Save commits, close/cancel keeps the persisted layout.
5. Invalid saved layout/color or missing locale resource restores the source-backed fallback and surfaces localized degraded status.

## Information Architecture And Content Strategy

- Persistent layer: toolbar first, left navigation second, central route content third, status bar last. This follows action -> destination -> task -> system-health reading order.
- Toolbar groups brand, room context and the primary connection action on the left; member state, notification and settings utilities on the right.
- Navigation groups free capabilities by PRD business areas. Member-only items appear only from the build/capability matrix; expired items remain disabled only where the PRD requires renewal meaning.
- Status bar uses compact labelled values in the exact PRD order. Connection/recording state use icon/dot plus text, never color alone.
- System settings uses a left settings category rail and one unframed content workspace. Current-scope categories are `配置管理`, `语言`, `界面`; downstream settings may occupy separate categories without merging semantics.
- Settings forms group identity/location metadata before operations, then selectable scope; interface settings group appearance before layout and danmaku display.
- Destructive reset is visually separated from Save/Import/Export. No command is hidden in explanatory cards.

## Layout Structure

### Main Shell

- Fixed-height top toolbar: 56px target, single row at supported minimum size.
- Middle region: fixed responsive navigation rail around 208px plus `min-width: 0` central workspace; only the content workspace scrolls by default.
- Fixed-height bottom status bar: approximately 32px, compact single row with horizontal overflow protection rather than wrapping into content.
- Central pages remain unframed full-width work surfaces; the shell does not wrap routes in a decorative card.
- Effect, login and payment windows are separate top-level surfaces. The main shell does not render them inside nested cards or overlays without their owning downstream flow.

### System Settings

- Full-height two-column workbench at desktop widths: 200-224px category rail and flexible content pane.
- Content pane has page title, form sections separated by dividers, then a stable action row. Individual sections are not floating cards.
- Config management uses a compact metadata definition row, operation toolbar and checkbox grid.
- Language uses one short form group. Interface uses two-column form rows where space permits and a single column at the 1024-width content minimum.
- Custom layout uses one modal with a bounded four-region preview and explicit Save/Cancel footer; it never nests another card/dialog.

## Visual Hierarchy

- Product name is the first stable toolbar signal but remains compact; current operational state and page content take priority.
- Primary blue/brand action is limited to the current submit/connect action. Destructive reset uses danger semantics and separation.
- Page titles use compact operational typography; section headers are smaller with regular spacing, not hero scale.
- Status metadata is subdued except degraded/error/recording state. Icons support familiar commands; every unfamiliar icon has a tooltip and accessible label.
- Use white/light-neutral surfaces with gray dividers, green/yellow/red state accents and the user-selected accent color. Avoid a one-hue theme, gradients, decorative blobs and ornamental cards.

## Visual Design Direction

- Tone: dense but calm desktop operations console.
- Rhythm: 8px base spacing; 16-24px section separation; borders/dividers establish regions instead of shadows or floating sections.
- Typography: normal UI text for controls, semibold compact section titles, tabular numerals for live metrics and exact dates.
- Motion: no decorative animation. Only short state transitions; respect reduced motion. Live status updates cannot pulse continuously or shift layout.
- Loading: preserve geometry with control loading states or compact skeleton rows only where data is expected; no marketing-style empty illustration.

## Section Breakdown

| Surface | Section | Priority | Content/actions |
| --- | --- | --- | --- |
| shell | top toolbar | primary/global | brand, room, connect/disconnect, member state when allowed, notification, settings |
| shell | navigation | secondary/global | dynamic source-backed route groups and active route |
| shell | content | primary/task | current routed feature |
| shell | status | persistent/context | seven exact operational values |
| settings | category rail | navigation | configuration management, language, interface; downstream categories remain separately owned |
| settings | config metadata/actions | primary | location/open, backup time, export/import/reset |
| settings | export scope | secondary | source-backed checkboxes filtered by build capability |
| settings | locale | primary | language, system auto, save |
| settings | interface appearance | primary | theme, accent, opacity, font, save/reset |
| settings | layout/display | secondary | layout/editor, danmaku fields, max entries |

## Styling Direction

- Continue Element Plus for select, button, menu, tooltip, badge, form, checkbox, slider, color picker, dialog and confirmation behavior.
- Use Lucide only if the package already becomes the project-wide icon owner; for this delivery retain the existing Element Plus icon library to avoid adding a second icon system.
- Cards are not the shell layout primitive. Settings sections are unframed with dividers; repeated notifications may later use compact rows under their owning module.
- Controls have stable heights and bounded widths. Labels wrap before they collide; English text may widen rows without changing font size by viewport.

## Tailwind CSS-Style Styling Constraints

- New or changed authored styling uses Tailwind-style utilities only.
- No new scoped CSS, CSS modules, Sass/Less, inline style objects or semantic class names.
- Conditional classes express only small active/disabled/status toggles.
- Any unavoidable dynamic accent/opacity application must use approved CSS custom properties owned by the shell configuration boundary; arbitrary template style objects are not allowed.
- Utility class values remain visible in templates and within formatter width.

## Class Length And Structure Strategy

- Split the shell into named toolbar/navigation/status components because each is a cohesive region with a focused state/interaction contract, not merely to shorten class strings.
- Split settings panels by business owner; keep short form rows inline inside each panel.
- Reduce decoration and wrapper depth before adding utilities. Do not hide long class values in constants, maps, computed properties, helpers or imports.
- Conditional status styles use short typed mappings limited to dot/text state variants; base layout utilities remain inline.

## Interaction Skeleton

- Room selection: filter/create/select supported room; validation occurs before connection; pending disables repeated connect.
- Connection: button toggles connect/disconnect according to current status. Keyboard activation and visible focus follow native button behavior.
- Navigation: click/Enter switches route; active and disabled states are both text/icon/state based. Disabled member entries do not navigate.
- Member control: absent in free build; state-specific label in member build; activation routes to member center without shell-owned purchase logic.
- Notification: icon button opens/closes the anchored notification surface and restores focus on close. The content contract is supplied downstream.
- Settings: icon and navigation item route to the same settings workspace.
- Export/import: native dialog begins a pessimistic operation; cancel is a no-op; pending disables duplicate action.
- Reset config: danger action -> second confirmation -> Rust reset. Cancel restores focus and changes nothing.
- Theme/language/opacity: preview on change; Save persists; pending disables repeat submission. UI reset restores the source-backed defaults.
- Layout editor: drag existing four region representations, Save or Cancel; keyboard reordering must be available alongside pointer drag.

## State And Feedback Model

| State | Presentation | Grounding |
| --- | --- | --- |
| shell restoring | stable four-region geometry with status placeholders | PRD 9.21.2 startup restore; source-derived stable layout |
| disconnected/connecting/connected/reconnecting/error | toolbar action plus status text; green/yellow/red only as support | PRD 8.7 and prior live-data contract |
| no room | room field empty; status room value explicitly absent | PRD 8.6-8.7; no invented empty-page copy |
| member capability loading/unavailable | free surface stays usable; no speculative member entry | PRD free-baseline requirement and capability snapshot boundary |
| member expired | required items disabled and renewal control visible | PRD 4.3, 8.5, 8.8 |
| settings loading | section geometry retained; actions disabled | source-derived from persisted config load, no new business outcome |
| valid dirty preview | UI reflects preview; Save is available | PRD 9.20.5, 9.21.5 |
| saving/importing/exporting/resetting | owning controls pending and duplicate action disabled | source-derived concurrency protection |
| save/export/import/reset success | localized completion feedback and refreshed snapshot | source-backed operations and shared localized error/result contract |
| damaged/incompatible/partial import | structured error; current config unchanged | PRD 9.17.6 |
| invalid layout/theme | default layout/color restored with degraded feedback | PRD 9.21.6 |
| missing locale/placeholder | Chinese/default localized text plus log | PRD 9.20.6 |
| notification data absent | surface contains no authored placeholder/call to action in this module | notification content is downstream/missing-source |

Updates are pessimistic for persisted/native operations. Only visual preview is immediate; the Rust snapshot remains the saved authority.

## UX Writing And Feedback Copy Stance

- Use the exact Chinese labels in the PRD and equivalent English keys; no member marketing or explanatory feature prose.
- Error messages are localized from stable Rust error codes with fixed placeholders. They identify the failed operation and preserve current config where required.
- Reset confirmation names the configuration-reset consequence; it does not mention deletion beyond the source-backed reset meaning.
- No new empty-state slogan, tutorial, keyboard-shortcut prose, retry copy, upgrade callout or notification placeholder is authored without downstream source.
- Disabled membership meaning uses the PRD terms `续费`/expired; free build contains no membership wording at all.

## Responsive Behavior

- Supported desktop window range starts at 1024x680; Tauri enforces the minimum. No mobile breakpoint or landing-page composition is introduced.
- At 1024 width, toolbar gaps/select width compress within bounded ranges; brand, room, primary action and utilities remain one row. Member expiry may use the exact date text but cannot overlap icon actions.
- Navigation width remains stable; long English labels truncate with tooltip only when required and never change route-row height.
- Status bar uses fixed metric groups and tabular numerals; it may horizontally scroll at high zoom rather than wrap over content.
- Settings category/content grid collapses only within the central pane when zoom makes two columns unsafe; forms become one column, checkboxes wrap and action rows wrap without overlap.
- Dialog width uses viewport bounds and a stable max width. High-DPI changes trigger layout measurement without losing draft or focus.

## Accessibility And Keyboard Considerations

- Semantic buttons/selects/checkboxes/sliders/color picker/menu items; no clickable `div` controls.
- Logical tab order follows toolbar -> navigation -> content -> status only when status contains a control; status values themselves are not focusable.
- Current route uses `aria-current`; status changes use a polite live region without announcing every high-frequency metric tick.
- Connection and recording states always include text; member disabled reason is exposed to keyboard/focus, not hover only.
- Tooltips name icon-only notification/settings actions. Focus returns to the trigger after popover/dialog close.
- Layout editor supplies keyboard move controls and visible focus in addition to drag.
- English/Chinese labels remain associated with controls; validation errors are tied to the relevant field.

## Usability Heuristics And Risk Controls

- Duplicate connection state in toolbar/status could conflict: status bar is authoritative; toolbar keeps only action-local pending feedback.
- Dynamic navigation could move the current route after capability change: route guard selects the nearest source-backed free route and announces the state change; it never exposes a forbidden page.
- High-frequency metrics could cause layout shift: fixed widths/tabular numerals/bounded labels preserve geometry.
- Settings can become a dense dumping ground: three scoped categories remain distinct; later modules add separate owned categories rather than optional mega-forms.
- Browser import/export bypasses native path security: all file operations move to native dialogs and Rust; pasted JSON/download anchors are removed.
- Immediate previews can diverge from persisted truth: saved snapshot and draft are distinct; failures restore the last persisted view.
- Reset is easy to trigger accidentally: config reset uses the source-required second confirmation and danger placement away from Save.
- Color-only state is prohibited; text/icon/status label accompany every color.
- Drag-only layout editing is inaccessible: keyboard reordering is mandatory.

## Component Reuse And Split Strategy

| Candidate | Decision and trigger | Callers | State owner / contract | Verification surface |
| --- | --- | --- | --- | --- |
| `AppShellToolbar` | extract; single-caller cohesive business section + focused test boundary | 1 | parent/store owns room/member/update state; normalized props; emits room select, toggle connection, membership, notification, settings intents; no slots | visibility matrix, pending/focus and label tests |
| `AppShellNavigation` | extract; single-caller focused test and dynamic permission boundary | 1 | parent navigation model owns entries; props typed normalized entries/current route; emits navigate; no permission calculation inside | four-state free/member matrix and disabled route tests |
| `AppShellStatusBar` | extract; single-caller cohesive high-frequency status section | 1 | stores/native snapshot own values; props seven normalized display fields; no emits/slots | semantics, bounded geometry and update stability |
| `NotificationSurface` | defer shell content; slot/composition boundary may be added only with downstream notification owner | 1 anticipated | shell owns open/focus only; downstream owns entries/actions/copy | application-shell verifies trigger/focus, later module verifies content |
| `SettingsWorkspace` | keep in route page; extracting would be a one-caller container/pass-through | 1 | route owns active category and orchestration | route/settings integration test |
| `ConfigManagementPanel` | extract; single-caller cohesive native file/reset sub-flow | 1 | settings store owns snapshot/pending; component owns checkbox draft and confirm/dialog focus; precise view-model props and action emits | import/export/reset/cancel/failure tests |
| `LanguageSettings` | reuse and adapt existing component; cohesive locale preview owner | 1 | locale/config store owns persisted state; component owns draft; emits preview/save intent instead of writing store directly | instant switch, fallback and save tests |
| `InterfaceSettingsPanel` | extract; single-caller cohesive appearance/layout form + focused test boundary | 1 | UI config store owns persisted state; component owns draft/preview intent | validation, preview/save/reset and responsive form tests |
| `LayoutEditorDialog` | extract; single-caller interaction lifecycle boundary | 1 | dialog owns temporary ordering, drag/keyboard focus and cancel; emits save/cancel | pointer/keyboard reorder and focus restoration |
| individual settings rows | keep inline; one-off markup and DTO/form passthrough blockers | 1 | parent panel draft | covered by panel tests |

All extracted components receive normalized view models, never Rust DTOs or whole store objects. Request transport, permission calculation, route orchestration and persistence stay outside presentational SFCs. No component exists solely to hide Tailwind strings.

## Design Risks

- PRD requires a notification center trigger but does not specify notification entry schema/copy. This module may create only the anchored shell/focus boundary; content remains downstream.
- PRD fixes the four-region shell while also requesting a drag layout editor. The conservative allowed editor manipulates only the four existing regions and may not add/hide product panels; architecture/spec must define a serializable valid grid and fallback without adding presets.
- Member state production contracts are external and belong to the next module. Shell tests must use the approved capability snapshot and must not simulate real login/payment success.
- Current translation coverage is incomplete and hard-coded Chinese exists in the shell. Spec must inventory all in-scope strings and reject missing English keys.
- Theme/accent/opacity across Element Plus, Tailwind and native window opacity can drift. Architecture must establish one persisted owner and one projection boundary.

## Open UI Questions

- `missing-source/downstream`: notification item schema, history length, read state, empty copy and actions. Not approved for this module; owned by the later update/notification integration.
- `external-interface-pending`: real member identity/expiry snapshot. The shell consumes only a typed capability result and stays free-first when unavailable.
- `code-fact-resolution`: exact drag grid serialization and snapping are implementation details constrained to the existing four regions; they may not change product-visible region inventory.
- Blocking product/UI questions: none. The conservative boundaries above avoid selecting an unsupported user-visible behavior.

## Page-Design Exit Decision

- Product-content allowlist is closed and traced to the original PRD.
- User journeys, hierarchy, interaction/state/feedback, responsive and accessibility contracts are defined.
- Component extraction/non-extraction decisions satisfy the Vue extraction policy.
- External/downstream gaps are isolated and do not authorize placeholder product content.
- Ready to advance to `spec`; `state.json.loop` remains unchanged.
