# Membership Intelligence Page Design

## Delivery Unit Identifier

- request: `tauri-rust-refactor`
- module: `membership-intelligence`
- stage: `page-design`
- self-healing iteration: `7`
- source module: `requirements/modules/membership-intelligence.md`
- design result: `ready for spec`

## Page Objective

Design the member-build-only journeys for Bilibili identity binding, membership purchase/module lifecycle, user-triggered AI suggestions with optional local ASR, and flood detection with optional mute execution. The pages must make activation state, local-only AI output, risky moderation actions and unavailable external dependencies unambiguous while leaving every free capability usable.

The module adds no send-danmaku control or workflow. AI output ends at local preview/copy. Moderation writes are limited to the PRD-defined mute interface and remain off by default behind membership, module, local credential and first-use risk gates.

## Target Users Or Scenarios

- Full-time or virtual streamer using the member installer to bind a Bilibili identity, purchase a plan and activate downloaded modules (PRD 4.3, 5, 6.3 scenarios 2-5, 9.15-9.16).
- Streamer or operator who actively asks the AI assistant for a local reply suggestion using selected danmaku, ASR and manual context (PRD 6.2, 9.14).
- Operator reviewing flood alerts and choosing view, mute or ignore; automatic mute is an optional, separately acknowledged risk (PRD 9.4).
- Existing member whose token expired, network is offline, module download failed or an external service is unavailable, while common free functions must remain unaffected (PRD 4.3, 5.4-5.6, 9.4.6, 9.14.6, 9.16.6, 14).

## Source And Upstream Constraints

- Product authority: PRD 0, 4-5, 6.3 scenarios 2-5, 8.2-8.5, 9.4, 9.14-9.16, 10.4, 11, 12-15; structured snapshot and membership-intelligence split module.
- Closed-world functions: automatic mute/flood protection, AI assistance and membership center only. Update, plugin, complete migration/resilience and release UI remain in `extensions-release-resilience`.
- Free build: no member/auth/module-download/AI/moderation route, label, component, command or teaser. Runtime hiding is insufficient (PRD 4.2, 9.15.10, 9.16.7, 10.4).
- Member build: the application-shell four-region layout, toolbar and capability-derived navigation remain the host. This module supplies the real member status producer and routes; it does not redesign the shell.
- External-interface pending: membership service, payment/order contract, Ed25519 public key, signed manifest/CDN and Cookie moderation API are absent. Their surfaces may show source-backed unavailable/retry states but cannot show fake orders, payment success, active membership or mute success.
- External-artifact pending: whisper.cpp binary, model manifest/hash and distributable license assets are absent. The model-download dialog may render only metadata from a validated native manifest and must remain unavailable otherwise.
- Current code facts: legacy AI/ASR/moderation Pinia stores own browser/localStorage/service behavior and there are no current member routes/pages. They are migration inputs, not product authority; the design assumes typed Rust snapshots and operations.
- Existing system: Vue 3, TypeScript, Vue Router, Pinia, vue-i18n, Element Plus and Tailwind-style utilities. Page content scrolls inside the existing central workspace at native minimum 1024x680.

## Product-Content Allowlist Mapping

| Proposed content | Grounding | Exact source | Design use |
| --- | --- | --- | --- |
| member toolbar control and member-center navigation | source-backed | PRD 4.3, 8.2-8.5, 9.16.3 | member build only; state-specific login/open/renew/expiry entry |
| member states: logged out, unpaid, paying, downloading, active, expired, renewed, unavailable/offline | source-backed | PRD 5.1, 5.3-5.6, 9.16.2-9.16.6 | page projection and action gating |
| account avatar, nickname and UID | source-backed | PRD 9.15.2-9.15.7, 9.16.4 | login confirmation and member header |
| QR login: pending, scanned, success, expired, error; 180-second validity | source-backed | PRD 9.15.2-9.15.5 | login dialog state machine |
| manual UID query, result confirmation and return to QR | source-backed | PRD 9.15.2-9.15.5 | degraded login mode |
| QR login risk disclosure and Cookie memory-only statement | source-backed | PRD 9.15.6, 9.15.9 | persistent dialog disclosure; no new consent action |
| monthly/quarterly/yearly plans and server-returned price/saving presentation | source-backed | PRD 9.16.2-9.16.4 | selectable plan cards; no hard-coded success |
| payment QR/link, selected plan, order number/status and cancel | source-backed | PRD 5.3, 9.16.2-9.16.5 | payment dialog |
| module name/version/state, per-module/total progress, retry/resume/hash failure | source-backed | PRD 5.2-5.3, 5.7, 9.16.3-9.16.7 | download and module-management dialogs |
| membership status, expiry, remaining days, renew, modules, orders, logout | source-backed | PRD 9.16.3-9.16.5 | active/expired member-center content |
| 7-day offline validity and free-capability isolation | source-backed | PRD 5.6, 9.16.6-9.16.7 | status explanation and degraded behavior |
| AI persona/config: identity, four speech styles, system prompt, provider/model/temperature/max length, sensitive filtering/context window | source-backed | PRD 9.14.2-9.14.4 | persona configuration dialog |
| ASR enable, model, language, local microphone lifecycle and recent 30-second context | source-backed | PRD 9.14.2-9.14.5 | AI toolbar and context list |
| whisper model name/size/license/source, consent, download progress/retry | source-backed | PRD 9.14.3, 9.14.5-9.14.7, 11 | first-use model dialog |
| context kinds: danmaku, ASR, manual input; click inserts into prompt | source-backed | PRD 9.14.2-9.14.5 | bounded selectable context list |
| AI generate/regenerate/clear/copy and streaming result | source-backed | PRD 9.14.2-9.14.5 | explicit user-driven primary workflow |
| `仅本地预览，不发送` result meaning | source-backed | PRD 0.1, 9.14.1-9.14.7 | permanently visible result heading; no send action |
| AI timeout/rate-limit/mic/sensitive/length/module/member failures | source-backed | PRD 9.14.6 | section-level error and recovery states |
| AI log navigation and local generation logs without send status | source-backed | PRD 9.14.2, 9.14.4-9.14.5 | audit route; no delivery column |
| moderation auto switch, fixed warning, duration, rules, alerts, today counts | source-backed | PRD 9.4.3-9.4.4 | flood-protection page |
| first-enable four-point risk confirmation and explicit acknowledgement | source-backed | PRD 9.4.3, 9.4.5, 9.4.7 | modal gating automatic mute only |
| frequency/repeat/similarity rules and source defaults | source-backed | PRD 9.4.3 | rule dialog |
| alert user/UID/violation/example and view/mute/ignore | source-backed | PRD 9.4.3-9.4.5 | compact alert rows |
| manual mute duration confirmation | source-backed | PRD 9.4.5 | action dialog; unavailable when membership/credential/API gate fails |
| expired moderation still detects local alerts and offers blacklist/ignore | source-backed | PRD 9.4.2-9.4.3 | readonly/degraded page section |
| mute log filters, appeal/misjudgment, whitelist and unmute | source-backed | PRD 9.4.2, 9.4.5-9.4.6 | moderation audit route |
| Cookie invalid/API failure/version change downgrade to local alert | source-backed | PRD 9.4.1, 9.4.6 | explicit non-success behavior |

Forbidden content includes fixed production membership endpoints, fake plans/orders/tokens/manifests, fabricated user/account data, an AI auto-generate switch, send/publish buttons, hidden automatic mute consent, member teasers in the free build, cloud-sync claims, or update/plugin controls.

## User Journey And Task Success

### Identity Binding

1. A member-build user enters from the toolbar login control or member center.
2. Login opens a focused dialog and requests a native QR session. Loading owns the QR region; successful response shows the Rust-generated PNG, 180-second validity and risk disclosure.
3. Native status events advance pending -> scanned -> success, or expired/error. Closing by Cancel aborts the session; switching to UID mode explicitly uses the supported degradation path.
4. UID lookup validates the source rule, shows avatar/nickname/UID, then waits for explicit confirmation before binding.
5. Success displays the bound account and offers source-backed `进入应用` or `开通会员`. Task success is the shell/member center reflecting the account; no paid state is inferred.

### Purchase, Install And Renew

1. A logged-in unpaid user scans account identity and available monthly/quarterly/yearly plans before choosing one.
2. Selection starts a pessimistic order operation. Only a native response may open the payment surface with plan, price, order number and QR/link.
3. Payment status remains pending until a signed token and manifest are validated. Cancel invokes the source-defined order cancellation; timeout returns to plan selection.
4. Verified payment moves to module download. Each module and total progress remain visible; the dialog cannot claim installation until download, hash, extraction and database registration finish.
5. Task success is active membership plus AI/moderation navigation becoming available. Failed download keeps the verified membership truth and offers the source-defined retry/resume path without affecting free functions.
6. Renewal repeats payment. Version-compatible installed modules are reused; otherwise only required updates appear. Success is member services/navigation restored within the source target.

### Local AI Suggestion

1. An active member with an installed AI module enters AI Assistant. Expired or missing-module users receive the source-defined paused/download state, never an editable false-ready workspace.
2. Optional ASR activation checks model availability. First activation opens the license/model dialog; Start Download remains disabled until acknowledgement. Microphone failure closes only ASR capability.
3. The operator reviews bounded danmaku/ASR/manual context, clicks items to insert them, writes a prompt and clicks Generate.
4. Generate is the only initial trigger. The input remains while native context assembly and LLM streaming progress; delta text appears only in the local result region.
5. Sensitive rejection produces no successful result; timeout/rate-limit preserves input. Completion enables Copy, Regenerate and Clear. Copy does not imply publishing.
6. Task success is a locally visible, filtered/truncated suggestion matching current config. No send handoff exists.

### Flood Alert Review And Optional Mute

1. An active member enters `风控中心 -> 刷屏防护`; the fixed non-standard Cookie API warning is visible before the auto switch.
2. Local detection continuously adds compact pending alerts and today counts. The user can inspect a behavior timeline, ignore the alert, or choose manual mute.
3. First auto-enable opens the exact risk dialog. Confirm remains disabled until acknowledgement; Cancel leaves automatic mute off. Later toggles use the persisted acknowledgement but still recheck membership/credential/API capability.
4. Manual mute opens a duration selector and waits for confirmation. One native retry is owned by Rust; a final failure returns the alert to manual/local handling and never shows muted success.
5. Expiry or invalid Cookie turns auto mute off/stops execution while local detection remains visible. Renewal restores eligible capability but cannot bypass missing credential/API conditions.
6. Task success is an auditable local decision or verified mute result. Ignore, appeal, whitelist and unmute remain recorded operations; free blacklisting stays in its existing owner.

## Information Architecture And Content Strategy

- Global member state is visible first through the existing toolbar and member-center route. AI/moderation navigation derives from validated membership plus installed module state; pages do not calculate access independently.
- Member center reading order: page/account identity -> membership state or plan decision -> primary payment/renew action -> module/order details -> logout. Payment and download are focused modal subflows so progress cannot be confused with page readiness.
- AI reading order: title/audit/config actions -> ASR controls -> current context -> manual prompt and generation controls -> local-only result. The no-send meaning sits in the result heading, not in optional help text.
- Moderation reading order: title/actions -> auto-mute control and fixed risk warning -> pending alerts -> today statistics. Warning precedes the risky switch; per-alert actions remain next to the affected identity.
- AI config, ASR download and moderation rule/risk forms are progressively disclosed in dialogs because they are secondary setup tasks with their own dirty/cancel lifecycle.
- Audit logs are separate routes, not tabs inside operational pages. AI logs and moderation logs have different semantics and must not share a generic log table.
- Account identity, membership entitlement and module installation are distinct states. A logged-in user is not paid; a paid token is not an installed module; an installed module is not an active entitlement.

## Layout Structure

### Member Center

- Full-width unframed central workspace under the existing shell, constrained to a readable maximum width.
- Compact page header with `会员中心`, account identity when present and the state-appropriate primary action.
- Logged-out state centers the source login choices without a marketing hero or decorative illustration.
- Unpaid state uses a three-column plan grid at wide desktop, followed by a plain benefits list and visually separated logout action.
- Active/expired state uses a two-column information band where space permits: membership metadata/action cluster and installed-module list. Orders and module management open focused surfaces.
- Payment and login dialogs are bounded to viewport height; module download uses a wider dialog with per-module rows and persistent total progress.

### AI Assistant

- Single-column operational workspace with four unframed bands: header/actions, ASR controls, context, prompt/result.
- Context has a stable bounded height and scrolls internally. Prompt and result use distinct labeled text areas; controls sit immediately below their owner.
- Streaming updates do not resize the page action row; result area has a stable minimum height and internal overflow for long output.
- Expired/missing-module presentation preserves the page title and replaces editable bands with the source-defined paused/download state.

### Flood Protection

- Header contains page title plus `规则配置` and `日志` secondary actions.
- Auto-mute controls share one compact row; the fixed risk warning is a full-width warning band directly below.
- Pending alerts render as individual compact rows with identity/violation first, example second and actions aligned consistently.
- Today counts form one bottom metadata strip using tabular numerals.
- Expired state keeps the local alert list and replaces `禁言` with the source-defined `加入黑名单`; execution controls are disabled/removed according to capability.

### Audit And Detail Surfaces

- AI log and moderation log routes use dense, horizontally scrollable tables with source-backed filters only. Detail views use drawers/dialogs so the operational page context is retained.
- User behavior timeline is a drawer with identity header, chronological violations and permitted actions. It is not merged with the existing general audience drawer because moderation audit state and actions differ.

## Visual Hierarchy

- Page title and current state are strongest; primary action is one filled Element Plus button per active decision point.
- Account identity, expiry, order number, module version/progress, context source and alert identity are scan-first values; secondary metadata uses quieter text but sufficient contrast.
- Warnings use Element Plus warning styling with icon plus text, never color alone. Expired/unavailable uses warning/error semantics without promotional gradients or large marketing cards.
- Plan choices may use individual compact cards because they are repeated selectable items. Cards stay at 8px radius or less and are never nested inside a page-level card.
- Destructive or irreversible actions such as logout, module cleanup and mute are separated from the primary flow and use danger styling only where the PRD action consequence warrants it.
- Progress bars always retain text/name/status; active, expired, failed and installed states are not color-only.

## Visual Design Direction

- Tone: quiet desktop operations console for identity, generation and moderation; payment is a focused transaction dialog, not a marketing checkout page.
- Density: compact 8/12/16/24px rhythm, tight section headings and stable control heights; reserve generous space for QR readability, streaming output and alert scanability.
- Surfaces: unframed page sections separated by borders/dividers. Repeated plans/alerts may use compact individual cards/rows; no card inside card.
- Typography: normal UI scale; semibold section labels; tabular figures for UID, price, expiry, remaining days, order number, progress and counts. No viewport-scaled font.
- Palette: existing neutral/light-dark shell with Bilibili accent only for primary focus/selection; warnings and failures use semantic amber/red, success uses green with text/icon.
- Motion: short progress/state transitions only; no pulsing AI animation, confetti, decorative gradients or continuous alert animation. Respect reduced motion.
- Loading: geometry-preserving skeleton/spinner inside the owning QR, plan, module, context, alert or result region. Loading never replaces the global shell.

## Section Breakdown

| Surface | Section | Priority | Content/actions |
| --- | --- | --- | --- |
| toolbar | member control | global | login/open member center/expiry/renew according to validated state |
| member center | identity/state | primary | avatar, nickname, UID, entitlement and expiry |
| member center | plans/benefits | primary decision | monthly/quarterly/yearly selection and exact source benefits |
| member center | modules/orders | secondary management | name/version/state/update/cleanup and order list |
| login dialog | QR or UID mode | focused primary | QR status/cancel/refresh/manual fallback or UID lookup/confirm |
| payment dialog | order | focused primary | plan, price, order, QR/link, status, cancel |
| download dialog | module install | blocking progress | per-module and total progress, verified completion/retry |
| AI | ASR/context | secondary input | toggle/model/language, bounded source context |
| AI | prompt/result | primary | generate/regenerate/clear, local stream, copy |
| AI | config/log | secondary navigation | persona/model config and AI audit |
| moderation | execution policy | primary safety | auto switch, duration, fixed warning, rule config/log |
| moderation | alerts | primary operations | identity, rule evidence, view/mute/ignore or blacklist |
| moderation | today counts | contextual | detected/automatic/manual counts |

## Styling Direction

- Continue Element Plus for dialogs, drawers, forms, segmented choices, switches, selects, progress, tables, alerts, checkboxes and confirmation surfaces.
- Continue the current Element Plus icon owner. Use icons for QR refresh, copy, clear, logs, settings and module actions where existing icons communicate the action; icon-only actions require tooltips.
- Use badges/tags only for compact state labels with accompanying text. Do not represent active membership or mute success only by a colored pill.
- Plan selection uses a clear selected border/check state and a text action; muted/expired controls remain focusable only when they expose the source-defined renewal route or disabled reason.

## Tailwind CSS-Style Styling Constraints

- All new or changed authored styling uses Tailwind CSS-style utility classes.
- Do not add scoped CSS, CSS modules, Sass/Less, inline style objects or non-utility semantic class names.
- Conditional class bindings may express only small selected/status/disabled toggles.
- Existing application-shell CSS custom properties remain the sole dynamic shell sizing/theme boundary; member pages do not create a competing layout system.
- Utility classes must preserve long Chinese/English labels, UID/order/module strings and error messages with wrapping, truncation plus tooltip where appropriate, or horizontal table overflow.

## Class Length And Structure Strategy

- Keep section layout utilities next to the markup they govern. A class value exceeding normal formatter width must be simplified or moved into a named component boundary with real UI ownership.
- Extract dialogs/drawers and large named operational regions listed below because they own focus/state/test lifecycles, not merely to hide classes.
- Keep short account metadata rows, state banners and page headers inline when extraction would only pass values/actions through.
- Do not hide long utility strings in constants, computed values, maps, helpers or imports. Status maps may choose a short semantic color toggle only; base utilities remain visible.

## Interaction Skeleton

- Entrypoints: member toolbar control/member-center navigation; capability-projected AI route; risk-center moderation child route.
- Navigation activation uses click/Enter through the existing shell. If entitlement/module becomes unavailable while mounted, route state moves to the approved paused/member-center surface without leaving an actionable forbidden form.
- All native writes are pessimistic: buttons enter local pending, duplicate submission is disabled, and success is rendered only from normalized Rust snapshots/events.
- Login Cancel aborts the native session and restores focus to its trigger. QR refresh exists only in expired/error state. UID mode preserves valid queried identity until changed or cancelled.
- Payment Cancel invokes order cancellation; dialog close follows the same path while cancellable. Module download close behavior follows the PRD `请勿关闭应用`: close is unavailable during non-terminal installation rather than silently abandoning work.
- Persona/rules dialogs own drafts. Cancel discards; Save validates and waits for native persistence. Source does not authorize unsaved-change confirmation, so none is added.
- ASR license acknowledgement is required before Start Download; progress starts only after a native manifest is available. ASR toggle is restored off on download/microphone/start failure.
- Generate requires a non-empty valid prompt or selected context as defined later in spec from the source input contract; while pending it dedupes repeat Generate. Regenerate uses the last completed input/context snapshot.
- Context click inserts the selected text into the manual input; keyboard activation matches click. Copy reports only clipboard success/failure and never changes generation/send state.
- Auto-mute first enable requires checked risk acknowledgement; Cancel and dialog close leave it off. Manual Mute requires duration choice and confirmation. Ignore removes the pending alert only after native acknowledgement.
- Disabled controls expose source-backed reason: inactive membership, missing module, missing LLM config, missing model/microphone, invalid Cookie or unavailable interface.

## State And Feedback Model

| State | Presentation and allowed recovery | Grounding |
| --- | --- | --- |
| free build | no member routes/components/labels/actions or callable commands | PRD 4.2, 10.4 |
| member state loading | stable shell/member geometry; member-only routes remain unavailable until validated | PRD 5.6; safe source-derived anti-false-success |
| logged out | member center login choices only; toolbar login action | PRD 5.1, 9.16.3 |
| QR pending/scanned | QR plus exact status and Cancel; scanned waits for phone confirmation | PRD 9.15.2-9.15.5 |
| QR expired/error | refresh and/or manual UID fallback according to source failure | PRD 9.15.5, 9.15.8 |
| UID query invalid/error | associated validation/error; input preserved for retry | PRD 9.15.5, 9.15.8, 10.4 |
| logged in unpaid | account, plans, benefits and logout; no AI/moderation navigation | PRD 4.3, 5.1, 9.16.3 |
| purchasing/payment pending | focused order dialog; cancel where source permits; no active status | PRD 5.3, 9.16.2-9.16.5 |
| payment failed/timeout | source error, retry or return to plan; timeout order cancelled | PRD 9.16.6 |
| module downloading | per-module and total verified progress; no premature route activation | PRD 5.3, 9.16.3 |
| module failed/hash failed | retry/resume; corrupted temporary file reported removed by native result | PRD 9.16.6-9.16.7 |
| active/offline-valid | status/expiry/remaining days/modules/actions; offline validity derived from verified token | PRD 5.6, 9.16.3-9.16.7 |
| expired | member center renew; AI/moderation paused; installed modules retained; free functions unaffected | PRD 4.3, 5.4-5.5 |
| member service unavailable | explicit degraded status and later retry; no fabricated plan/order/token; free functions unaffected | PRD 9.16.6, 14 |
| AI ready | context/input/result bands and config/log actions | PRD 9.14.3-9.14.5 |
| AI assembling/streaming | Generate pending; stable local result region receives ordered deltas | PRD 9.14.2, 9.14.5 |
| AI success | filtered/truncated local result with Copy/Regenerate/Clear | PRD 9.14.2-9.14.7 |
| AI timeout/rate limit | input preserved, no successful result; retry through Generate/Regenerate | PRD 9.14.6 |
| sensitive rejection | no preview result; recorded failure reason | PRD 9.14.6 |
| ASR/model unavailable | ASR off; manual prompt/generation remains usable when LLM is available | PRD 9.14.6 |
| moderation ready | local alerts, rule controls and counts; auto remains off by default | PRD 9.4.1-9.4.4 |
| risk acknowledgement pending | focused four-point warning, required checkbox, Cancel/Confirm Enable | PRD 9.4.3, 9.4.5 |
| mute pending/success | owning alert pending; success only after verified event/log update | PRD 9.4.2, 9.4.5 |
| Cookie/API failure | auto off and local alert retained; source error explains re-login or interface degradation | PRD 9.4.6 |
| moderation expired | execution paused, local alerts retained, blacklist/ignore/renew available | PRD 9.4.2-9.4.3 |
| empty lists | preserve named section geometry; exact empty copy is `missing-source` and must not be authored in spec/execute without confirmation | source omission |

## UX Writing And Feedback Copy Stance

- Reuse exact PRD terminology: `登录`, `开通会员`, `续费`, `会员功能已暂停`, `仅本地预览，不发送`, `扫码登录暂不可用`, `UID 无效或查询失败`, `服务限流，请稍后重试`, and the four automatic-mute risk statements.
- Login, payment, module, AI and moderation failures map structured Rust codes to bilingual fixed text. Do not display raw endpoints, tokens, cookies, API keys, filesystem paths or internal service payloads.
- Success copy must distinguish identity bound, payment verified, module installed, AI generation completed and mute verified; none may imply another step also succeeded.
- No free-build upgrade copy, generic promotional slogan, fake scarcity/savings, tutorial, keyboard-shortcut text or AI “thinking personality” copy is permitted.
- Exact empty-list copy for orders, modules, AI logs, moderation logs and pending alerts is absent from source. Preserve empty geometry without authored guidance until a confirmation exists.
- Logout, module cleanup, clipboard failure and unsaved-draft confirmation wording are not source-defined. Use direct source actions and standard control state only; do not add new modal copy in page design.

## Responsive Behavior

- Supported native desktop starts at 1024x680. Central content is `min-width: 0`; page toolbars wrap before labels collide, and the shell never collapses into a mobile navigation pattern.
- Plan grid: three columns at wide content, then one column when central workspace width is insufficient. Cards keep equal action placement and prices do not overlap saving labels.
- Login/payment dialogs keep QR at a stable square size while adjacent instructions stack below when narrow; dialog body scrolls without moving primary footer actions off-screen.
- Module rows and audit tables preserve exact identifiers through horizontal scrolling or middle truncation plus tooltip; progress/status columns remain fixed enough to scan.
- AI context/result areas use stable min/max heights with internal scrolling. ASR controls wrap as a group; Generate remains adjacent to prompt owner rather than becoming a floating page action.
- Moderation alerts change from identity/evidence/actions columns to stacked rows at constrained widths. All actions wrap as a coherent group and never cover evidence text.
- High zoom may create workspace-level horizontal scrolling as already approved by the application shell; no text overlaps toolbar, navigation or status bar.

## Accessibility And Keyboard Considerations

- Logical focus order follows page title/state -> primary action -> secondary controls -> list/table actions. Dialog open moves focus to title/first actionable control and close restores the originating trigger.
- QR images have localized alternative text; live QR/payment/module/AI states use polite live regions without announcing every 2-second poll or streaming token.
- Status meaning always includes text/icon; selected plan, membership state, download result, AI state and mute result are never color-only.
- Switches, checkboxes, selects, forms, tables, buttons, dialogs and drawers use Element Plus semantic controls. Context rows and alert actions remain keyboard operable with visible focus.
- Risk acknowledgement is a labeled checkbox associated with the four-point warning; Confirm Enable stays disabled until checked and explains the disabled relationship through native semantics.
- Streaming output is readonly and selectable. Copy has a named icon/label; there is no hidden send affordance.
- Long English error/risk/license text wraps. QR and progress surfaces retain usable target sizes at high DPI; icon-only tools are at least the existing shell button size and have tooltips.
- Avoid keyboard traps in payment/download/login dialogs; Cancel remains available where the underlying native operation is cancellable. Non-cancellable install progress must still allow assistive reading and window movement.

## Usability Heuristics And Risk Controls

- State conflation risk: account, entitlement and module readiness are shown separately and all actions derive from normalized state rather than page-local booleans.
- False payment risk: no optimistic activation, celebratory effect or downloaded-route reveal before token signature and module installation complete.
- QR/session leak risk: one session owner, explicit cancel, stale event rejection and focus restoration are required inputs for spec/architecture.
- Sensitive data risk: API keys, cookies, tokens and raw module paths never appear in templates/log tables. Login disclosure explains Cookie is session-only as the PRD requires.
- AI accidental-send risk: result heading permanently says local-only/no-send; only Copy/Regenerate/Clear exist. No ambiguous arrow/paper-plane icon is used.
- AI accidental generation risk: no auto-trigger on context/ASR change; Generate is an explicit button and duplicate pending action is disabled.
- Streaming race risk: one visible generation owns ordered deltas; regenerate/cancel/route exit must invalidate stale events without merging results.
- Moderation account-risk: fixed warning cannot be dismissed; first auto enable requires explicit acknowledgement; auto is off by default and capability loss turns it off.
- False mute risk: pending, verified success and retry-exhausted local alert are distinct. A network/API failure cannot remove the alert as if muted.
- Dense alert/log risk: business evidence and action group are visually separated; tables scroll instead of compressing labels; counts stay secondary to unresolved alerts.
- Download integrity risk: progress and completion are distinct; hash failure shows failed status and retry only after corrupted temp cleanup is reported.
- External dependency risk: unavailable surfaces preserve free workflows and user input where source allows; they never simulate account, order, token, module, LLM or mute success.

## Component Reuse And Split Strategy

| Candidate | Decision / trigger | Production callers | Owner and contract | Styling / verification |
| --- | --- | --- | --- | --- |
| existing application shell toolbar/navigation | reuse existing | root shell | shell owns placement/navigation intent; member store supplies normalized projection | free/member visibility matrix and route guard |
| `MemberLoginDialog` | extract; interaction lifecycle + focused test boundary | 1 | member-center/root orchestration owns opening; dialog owns QR/UID mode, local UID draft and focus; normalized state props; emits cancel/refresh/query/confirm/enter/purchase intents; no slots | QR geometry, keyboard/focus, all status branches |
| `MembershipPlanGrid` | extract; cohesive business decision section | 1 | page/store owns catalog/selection pending; normalized plan view props; emits select; no DTO or transport | equal selectable cards, server-data/empty/unavailable tests |
| `MembershipPaymentDialog` | extract; transaction interaction lifecycle | 1 | store owns order state; dialog receives normalized order and emits cancel; no polling | QR/status/focus/cancel and no optimistic success |
| `ModuleDownloadDialog` | extract; progress lifecycle and focused test surface | 1 | store/native event owns progress; normalized module rows; emits retry only at failed terminal state; no transport | per-row/total progress, close gating, hash failure |
| `ModuleManagementDialog` | extract; cohesive module-management subflow | 1 | store owns installed snapshots/actions; dialog emits update/cleanup; no raw paths | state/action availability and long version strings |
| `MembershipOrderDrawer` | extract; cohesive audit list | 1 | store owns paged orders; normalized rows; emits refresh/select only if source contract supports it; no slots | loading/error/empty geometry/table overflow |
| account summary blocks | keep separate inline | 2+ different semantics | login confirmation owns candidate identity; center owns bound identity/status | avoids broad optional account component |
| `MembershipPausedNotice` | extract/reuse; 2+ same membership-gate semantics | AI and moderation pages | receives approved title/body/action labels or normalized view model; emits renew; moderation local alerts remain outside | active/expired route projection, text-plus-state |
| `AsrModelDownloadDialog` | extract; license/download lifecycle | 1 | AI store owns manifest/progress; dialog owns acknowledgement/focus and emits start/retry/cancel | metadata integrity, checkbox gate, progress/failure |
| `AiPersonaConfigDialog` | extract; cohesive form + dirty lifecycle | 1 | dialog owns validated draft; store owns saved snapshot; emits save/cancel; no backend DTO | four style options, validation/pending/rollback |
| `AiContextList` | extract; bounded high-update region/focused test | 1 | feed/AI store owns normalized items; component renders and emits select | source labels, bounded scroll, keyboard selection |
| AI prompt/result bands | keep inline | 1 | AI page owns manual draft and generation orchestration | extraction would create model/emit passthrough |
| AI log table | keep in dedicated route page | 1 | route/store own filters/results; rows receive normalized fields | separate audit semantics; table states |
| `ModerationRiskDialog` | extract; mandatory safety/focus lifecycle | 1 | dialog owns acknowledgement; store owns persisted risk/auto state; emits confirm/cancel | checkbox gate, exact four statements, focus restore |
| `ModerationRulesDialog` | extract; cohesive validated form | 1 | dialog owns rules draft; store owns saved rules; emits save/cancel | source defaults/ranges, pending/rollback |
| `ModerationAlertList` | extract; cohesive high-update business section | 1 | store owns normalized alerts; component emits view/mute/ignore/blacklist intentions | active/expired action matrix, compact overflow |
| `ModerationUserDrawer` | extract; interaction/audit lifecycle | 1 | store owns timeline; drawer owns focus/selection and emits approved actions | chronology, identity, action gates |
| moderation log table | keep in dedicated route page | 1 | route/store own filter/result orchestration | different from AI logs; no generic audit table |
| generic member manager/dialog wrapper | reject | hypothetical | no stable cross-flow semantics; would add optional props/flags | avoids speculative abstraction |

All extracted SFCs receive normalized view models and user-intention callbacks/events. They do not invoke Tauri, calculate membership permission, parse manifest/order payloads or persist configuration. Single-feature components stay under the membership-intelligence feature owner until real same-semantics reuse exists.

## Design Risks

- The PRD defines product flow but no production membership/payment/manifest contract. The UI can be complete only against typed unavailable/fake-port tests until official deployment contracts arrive.
- The PRD states a downloaded member-center frontend module while also requiring the member center before payment. The previously confirmed normalization holds: minimal login/purchase/module-progress center is compiled into member build; downloaded resources may extend active-state content only.
- Dynamic execution of downloaded frontend resources raises CSP/signature risk. Page design permits only validated fixed module contributions; architecture must reject arbitrary script evaluation.
- `请勿关闭应用` during download does not define application-exit cancellation/resume copy. The dialog prevents casual close while progress is non-terminal; OS/app-exit recovery is a native lifecycle concern and exact extra copy remains missing-source.
- Exact plan IDs/prices, order history fields, module manifest metadata, model catalog and API key storage semantics depend on external/native contracts. The UI renders validated values and cannot hard-code production truth from wireframe examples.
- Manual UID binding proves public identity but not Cookie availability for moderation. The moderation page must not infer a usable Cookie from a bound account.
- Existing legacy AI/ASR/moderation stores contain browser services/localStorage and broader presets. They must not silently define new design options during migration.

## Open UI Questions

- `external-interface-pending`: exact plan catalog IDs/currency/price source, payment provider/QR-vs-link result, order statuses/fields, signed token claims and module manifest fields. Design renders only normalized native values.
- `external-interface-pending`: Cookie credential acquisition/refresh and mute/unmute/appeal response contracts. Auto mute remains unavailable without them and local detection continues.
- `external-artifact-pending`: whisper binaries, model catalog/hash/license assets. The source example `whisper-small`, approximately 500 MB, MIT and official repository is the only visible model metadata currently authorized; production download cannot be claimed.
- `missing-source`: exact empty-state copy/actions for plans, orders, modules, AI logs, moderation logs and no pending alerts. No new empty copy is approved.
- `missing-source`: exact logout/module-cleanup confirmation wording and whether closing an in-progress module download should minimize or only block. Do not invent confirmation/behavior beyond native cancel/resume semantics.
- `missing-source`: AI log and moderation log exact visible columns beyond the PRD-described audit meanings and stored fields. Spec must use PRD SQL/flow fields only and omit unsupported columns.
- Blocking UI/product questions: none for source-backed local implementation and fail-closed unavailable states. These gaps block production integration/success claims, not the next `spec` stage.

## Page-Design Exit Decision

- Product content is closed and traced to the PRD/snapshot/module; no free-build member content or send-danmaku affordance is admitted.
- Login, purchase/download/renewal, AI/ASR and moderation user journeys define hierarchy, interactions, state feedback, errors, responsive behavior and accessibility.
- Component extract/keep-inline decisions have named owners, real caller counts, precise contracts and test surfaces.
- External contracts/artifacts remain explicit unavailable boundaries and do not authorize fake success.
- Ready to advance to `spec`; `state.json.loop` remains unchanged.
