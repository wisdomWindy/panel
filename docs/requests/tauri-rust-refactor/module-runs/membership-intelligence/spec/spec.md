# Membership Intelligence Engineering Specification

## Delivery Unit Identifier

- request: `tauri-rust-refactor`
- module: `membership-intelligence`
- upstream: `requirements/modules/membership-intelligence.md` and `design/page-design.md`
- speed profile: `S3 broad`
- architecture-design required: `true`

## Background And Goals

Implement the member-build-only identity, membership, signed-module, AI/ASR, and moderation capabilities from PRD sections 4, 5, 9.4, 9.14, 9.15, and 9.16. Preserve every free capability while preventing the free binary from compiling, registering, routing to, or displaying member capabilities. Member success is valid only after native verification; missing production services remain explicit unavailable ports rather than simulated success.

## In Scope

- Member-build account binding through Bilibili QR login and manual UID fallback.
- Member center states: logged out, logged in/unpaid, payment pending, module installation, active, expired, and service unavailable.
- Signed entitlement validation, seven-day offline window, renewal, module download/hash/install/update/cleanup, and Sidecar lifecycle.
- AI persona/LLM configuration, explicit generation, ordered streaming, local filtering/truncation, local-only preview, copy/clear/regenerate, logs, and ASR lifecycle.
- Local spam detection, alerts, optional manual/automatic mute, risk acknowledgement, rules/logs, false-positive whitelist, and expiry/Cookie/API degradation.
- Member routes, toolbar projection, native contracts, SQLite repositories, and removal of legacy browser/localStorage/Electron member paths.

## Out Of Scope

- Any danmaku send button, send command, automatic sending, or hidden sending task.
- Member routes, labels, commands, Rust modules, or dependencies in the free build.
- Fabricated payment, entitlement, signature, module manifest, Cookie API, LLM, or Whisper-artifact success.
- New plan prices/IDs, order fields/statuses, provider presets, moderation ranges, empty-state prose, confirmation prose, analytics, or UI actions absent from the PRD.
- Plugin/update/release behavior owned by `extensions-release-resilience` and free-feature changes owned by completed modules.

## Trigger And Start Conditions

- The application is the member build for all member flows. The free build omits all member code and routes.
- Login starts from toolbar or member center. Purchase requires a bound account. AI and automatic moderation require a verified active entitlement and ready corresponding module.
- A missing deployment contract/artifact produces a typed unavailable state and leaves free workflows operational.

## Requirement Split Summary

The requirement map assigns PRD functions 4, 14, and 15 to this module after the four completed modules. This unit consumes build-feature isolation, typed IPC, accepted live-event streams, path/security boundaries, SQLite, and shell navigation projection; it does not reopen their approved behavior.

## Source Grounding And Traceability

| ID | Spec item | Grounding | Product/business | Exact authority | Downstream obligation |
| --- | --- | --- | --- | --- | --- |
| MI-S01 | Free build has no member code/UI/commands | source-backed | yes | PRD 4.2, 10.4, 9.15.10, 9.16.7 | cfg, router, bundle and registry tests |
| MI-S02 | Account identity and Cookie separation | source-backed | yes | PRD 9.15.1-9.15.9 | isolated native session; public data only on disk |
| MI-S03 | QR state/polling and manual UID fallback | source-backed | yes | PRD 9.15.2-9.15.8 | command/event state machine tests |
| MI-S04 | Purchase, signed token, module install, renewal | source-backed | yes | PRD 5.3-5.7, 9.16.2-9.16.7 | fail-closed ports; verified transitions only |
| MI-S05 | Offline entitlement for seven days | source-backed | yes | PRD 5.6 | clock/signature tests |
| MI-S06 | AI is explicit, local preview only, never sends | source-backed | yes | PRD 9.14.1-9.14.7 | no send surface; source/registry scans |
| MI-S07 | AI stream/filter/truncate/log flow | source-backed | yes | PRD 9.14.2-9.14.6 | ordered generation state machine |
| MI-S08 | ASR on-demand Whisper model and 30-second cache | source-backed | yes | PRD 9.14.2-9.14.7 | license, integrity, lifecycle tests |
| MI-S09 | Moderation defaults off and first-use risk gate | source-backed | yes | PRD 9.4.3-9.4.7 | persisted acknowledgement/action gate |
| MI-S10 | Failed/expired moderation degrades to local alerts | source-backed | yes | PRD 9.4.2, 9.4.6 | never resolve on unverified mute |
| MI-S11 | Shell routes follow entitlement/readiness | source-backed + code-fact-backed | yes | PRD 4.3, 5.1; `MemberShellStatus` | one normalized member projection |
| MI-S12 | External gaps remain unavailable | source-backed | yes | PRD 14; request Done Signal/non-goals | typed unavailable adapters |
| MI-S13 | Minimal pre-payment member center is built in | confirmed-decision | yes | requirement-analysis normalization of PRD 5.1/5.2 | downloaded UI extends active state only |
| MI-S14 | Legacy browser member stores are migration inputs | code-fact-backed | no | current AI/ASR/moderation stores | remove callers after replacement |

### Closed-World Product Content Allowlist

- Routes: member center, AI assistant/log, moderation/log; member build only.
- Account: avatar, nickname, UID, login time; QR states `Pending`, `Scanned`, `Success`, `Expired`, `Error`.
- Membership: server-returned plans, sourced benefits, order number/status, membership status, expiry date, remaining days, module name/version/status.
- Member actions: login/cancel/refresh, UID query/bind, enter, purchase/cancel order/renew, module manage/update/cleanup, order query, logout.
- AI: sourced persona/provider/model fields, four speaking styles, ASR model/language/state, contexts, prompt/result, generate/regenerate/clear/copy, logs.
- Moderation: fixed risk notice, auto switch, sourced duration/default rules, user/UID/violation/evidence, view/mute/ignore, today counts, user/time filters, false-positive whitelist, renew.
- Approved copy is exact PRD terminology/errors/risks including `仅本地预览，不发送`. Empty regions have stable geometry and no invented prose.
- Anything absent, including extra plans, promotions, badges, analytics, sending affordances, provider/model catalogs, or extra actions, is forbidden.

## Workflow Efficiency Profile

- `S3 broad` is required by build features, network/file/process boundaries, multiple state machines, shared shell projection, SQLite, routing/stores, and destructive cleanup.
- Full artifact density applies. Downstream context is limited to the member chain, shared shell/desktop/accepted-event contracts, migrations, build registry, and direct callers/tests.
- New product semantics return to spec; code architecture incompatibility returns to architecture-design.

## Self-Healing Completion Bar

- Every hunk maps to MI-S01..MI-S14 or explicit cleanup/test work; no free-build or send leakage.
- Validate entitlement/signature/time, module integrity, login isolation, generation ordering/filtering/truncation, moderation retry/degradation, permissions, and visible transitions.
- Run Rust fmt/clippy/tests, TypeScript typecheck, lint, Vitest, free/member web and Tauri builds, bundle/registry scans, and focused runtime checks.
- Require explicit null/error/empty/loading/disabled handling, stale-event rejection, task/Sidecar/listener cleanup, and no secret/path exposure or optimistic success.
- Require one state owner, typed contracts, no orphan legacy caller, unrelated edit, unused export/test/mock/comment/debug artifact, or format churn.
- Completion: `self-healing loop compliance: pass`, `self-healing loop assessment: pass`, blockers `none`.

## User Intent Contract

- Literal: continue refactoring the project according to the named PRD.
- Practical goal: a real Tauri/Rust member slice that preserves free functionality and fails honestly where contracts are missing.
- Success: approved workflows exist in member build only; local/native state machines are testable; external gaps cannot succeed; AI never sends; expiry/failure preserves free operation.
- Forbidden: runtime hiding in free build, legacy localStorage/browser engines as owners, fake payment/tokens/manifests, conflating login/payment/install, or adding sending.
- Acceptable: unavailable production ports with injected deterministic tests, source-backed Bilibili login adapter, signed fixed resources, typed Vue view models.
- Review: registry/bundle/source scans and all state/failure branches map to source IDs.

## User Flow

1. Member build loads public account, verified entitlement, modules, and capability projection from Rust.
2. Logged-out user starts QR or UID fallback. Native owns polling; cancel aborts. Binding writes only public metadata.
3. Logged-in/unpaid user selects a server-provided plan. Order stays pending until verified; cancellation/timeout never activates.
4. Verified payment validates Ed25519 token, persists entitlement, downloads signed entries sequentially with resume/hash/temp cleanup, installs fixed modules, then exposes ready routes.
5. Active user explicitly generates AI text or enables moderation; every member operation rechecks entitlement.
6. Expiry stops AI/ASR/auto moderation and Sidecars, retains modules/alerts, disables actions, and routes to renewal.
7. Renewal validates a new token, reuses or updates modules, restarts allowed services, and restores routes.

## Page And Module Design

- Member center orchestrates mutually exclusive logged-out, unpaid, transaction/install, active, expired, and unavailable sections.
- AI uses context, prompt, result bands; persona/model are dialogs; logs are a route.
- Moderation uses persistent risk, controls, alerts, user drawer, and logs route.
- Shell owns placement/guards; member store supplies normalized projection. Pages never infer entitlement from route/account presence.
- `design/page-design.md` is normative for hierarchy, density, responsiveness, accessibility, and component decisions.

## UI / UX Design Contract

- Reading order is page/status, primary decision/action, supporting fields, lists/audits. Account, entitlement, and readiness are distinct.
- Native writes are pessimistic and deduped. Success renders only from normalized Rust result/event.
- Dialogs own focus and restore it. Non-terminal install cannot be casually closed; OS-exit recovery is native-owned without invented copy.
- Lists preserve geometry for loading/error/empty. Errors use approved text and never reveal secrets/endpoints/raw payloads.
- At 1024x680 toolbars wrap, tables scroll, dialog footers remain reachable, AI bands scroll internally, and alerts stack when constrained.
- Use semantic Element Plus controls, keyboard paths, visible focus, text-plus-icon states, associated validation, and polite live regions; do not announce every delta/poll.

## Frontend Styling Constraints

- New styling uses Tailwind-style utilities only; no new scoped CSS/modules/Sass/Less/inline style objects/semantic styling classes.
- Class values stay inline and within formatter width; never hide long values in constants/maps/computed/helpers/imports.
- Conditional classes are short state toggles. Split meaningful markup/component boundaries if needed.
- Member pink is limited to brand/member emphasis; success/warning/error/neutral/disabled/focus/active/visited remain distinct and not color-only.

## Field Display Semantics And Interaction Visual-State Contracts

| Field | Meaning/source | Display/empty | Interaction/visual | Evidence |
| --- | --- | --- | --- | --- |
| account avatar/name/UID | public identity, not entitlement; 9.15.2/9.15.7 | image, source name, exact UID; image failure uses existing behavior | neutral, non-clickable unless wrapped by sourced action | login/component tests |
| QR state/image | current session; 9.15.2-9.15.5 | base64 PNG and exact state text; absent on error | refresh only expired/error; cancel pending/scanned; visible focus | state/dialog tests |
| plan/amount/savings | purchasable catalog; 9.16.2-9.16.4 | validated label/amount/currency; no production hard-code; no catalog is unavailable | default/focus/selected/disabled; all disabled pending | adapter/component tests |
| order number/status | payment transaction; 9.16.3-9.16.6 | exact ID and normalized approved state; unknown is error | read-only; cancel pending/cancellable only | adapter/state tests |
| entitlement status | verified access; 5.1/5.4-5.6 | exact active/expired/unavailable; never inferred from login/payment | persistent text/icon; renew command | signature/clock/UI tests |
| expiry/remaining | deadline/whole calendar-day remainder; 9.16.3 | `YYYY-MM-DD`, integer `N 天`; invalid verified value makes unavailable | read-only persistent emphasis | clock/format tests |
| module/version/state/progress | signed install; 5.2-5.7 | exact validated values, text state, integer verified-byte percent | update/cleanup by state; retry failure | event/download tests |
| AI context | danmaku/ASR/manual source; 9.14.2-9.14.5 | source tag + text; no invented timestamp | click/Enter inserts; distinct focus/active | keyboard tests |
| AI prompt/result | user intent/local suggestion; 9.14.2-9.14.5 | raw prompt; read-only selectable result with local-only label | explicit Generate; Copy/Regenerate/Clear only | UI/registry scans |
| AI log | generation audit, never send audit; 9.14.2/SQL | only sourced SQL/workflow fields; omit unknown columns | only sourced filters/navigation; no send action | repository/page tests |
| moderation alert | unresolved local risk; 9.4.3-9.4.6 | user, UID, violation, evidence; retained on fail/expiry | View/Mute/Ignore; mute gated | action tests |
| moderation counts | today's recognized/auto/manual; 9.4.4 | non-negative integers with exact labels; invalid is error | read-only secondary | repository/view tests |
| risk notice | mandatory disclosure; 9.4.3 | exact four statements and acknowledgement | persistent; confirm disabled until checked | a11y/dialog tests |

Generic date/amount/status/link/tag formatters are forbidden unless exact semantics match. Default business emphasis remains before hover; interaction states cannot replace it.

## Function-Complete Behavior Breakdown

### Account And Login

- `auth_qrcode_generate` creates one isolated native session, replaces/aborts prior session, returns key/image, and polls every 2 seconds for at most 180 seconds/90 polls. Five consecutive transport failures produce `Error`; offline pauses/resumes.
- `auth_qrcode_status` maps `86101/Pending`, `86090/Scanned`, `0/Success`, `86038/Expired`, other/Error. Reject stale session events.
- Cancel awaits `auth_qrcode_cancel`, closes, restores focus. Refresh only after expired/error.
- UID draft is preserved. Submit trims surrounding whitespace for request; empty/non-decimal input is invalid without request. No source-absent length rule. Query result persists until UID changes/cancel; bind writes returned UID/name/avatar/login time.
- Logout removes account metadata/session, not modules. No additional confirmation copy.

### Membership, Order, And Modules

- Startup and every member command validate channel, account, signature, expiry, offline age, and required module. Tampered tokens never activate.
- Catalog/order/token/manifest DTOs are adapter-controlled. Missing contract returns `external_service_unavailable`; production never uses fixture data.
- Purchase accepts catalog `plan_id`; pending dedupes. Timeout cancels; failure preserves selection; payment response alone is not activation.
- Install downloads to temp, resumes where transport supports, verifies hash before extraction, confines path to `modules/<name>/<version>/`, records `t_module`, and emits only after commit. Hash failure deletes corrupt temp and allows retry.
- Total progress derives from verified bytes. Same-version valid module is reused on renewal; new version updates. Expiry retains files. Cleanup uses opaque module IDs, never UI paths.
- Controlled performance targets: install <=5 minutes; no-download renewal <=1 minute.

### AI Configuration, ASR, And Generation

- Persona form has allowlisted fields only. Free text preserves whitespace/newlines; no hidden trim/length/character rules. Provider/model options and numeric ranges come from validated native/provider contracts; absent contract disables save as unavailable.
- Style enum: `简短短句`, `幽默玩梗`, `礼貌正式`, `随性闲聊`. Save is pessimistic; cancel discards draft; failure keeps draft and prior saved snapshot.
- First ASR enable opens model/license. Start disabled until acknowledgement. Download verifies manifest/hash/resume; start requires valid model/microphone. Failure turns ASR off but manual AI remains. Stop cleans capture/Sidecar.
- Generate requires ready capability and either non-whitespace prompt or selected context, as source specifies input-or-context. Payload preserves raw prompt and immutable context. No input auto-generates.
- Native assembly combines persona + accepted danmaku + recent ASR + manual input and is separately measured <=500ms. Default LLM timeout is 5s.
- One generation ID owns monotonic deltas. Repeat is deduped; regenerate uses last completed snapshot; new generation/route exit invalidates stale deltas. Completion filters sensitive text then truncates by max length. Sensitive hit shows no preview and logs failure; timeout/rate limit preserves input and no successful result.
- Copy only reports clipboard outcome. Clear resets prompt/result/selection, not logs/config. No symbol sends generated text.

### Moderation

- Detection consumes accepted events. Defaults: `10s >5`, `1min >3 identical`, `1min >5 similarity >80%`, mute `1h`; adjustable ranges come from native contract because PRD gives none.
- Auto defaults off. First enable presents exact four-point warning and checked acknowledgement before `moderate_set_auto`. Cancel/close stays off. Entitlement/Cookie/API failure turns it off.
- Manual mute uses sourced `10m`/`1h`/permanent duration and confirmation. Native retries once. Only verified success resolves/increments; second failure retains local alert with error.
- Ignore removes only after persisted ignore log. View opens full behavior timeline. Logs filter by user/time. False-positive marking adds whitelist.
- On expiry, local detection/alerts remain; auto/manual execution pauses; existing blacklist/ignore remain; renew navigation appears.

## Expert Frontend Engineering Constraints

- Rust owns account, entitlement, modules, order, ASR/generation/moderation runtime/persistence; Pinia owns normalized snapshots and pending/error; dialogs own drafts/focus; route owns route; capability/status is derived.
- Lifecycle: native DTO/event -> desktop contract -> adapter -> immutable view model -> props; form -> validated payload -> native response/event -> refreshed snapshot. Templates never normalize DTOs.
- IDs reject stale events; polling/download/generation dedupe; cancellation/teardown explicit; only source retries; completion idempotent.
- Bound high-update lists/result regions; avoid broad subscriptions/duplicate writable state. No memoization/virtualization without evidence.
- External adapters and fixed signed manifests are seams. Arbitrary script evaluation, generalized plugin registry, and dual legacy path are forbidden.
- Diagnostics use structured safe context, never tokens/API keys/Cookie/endpoints/raw paths/full sensitive payloads.

## Frontend Architecture Design Routing Inputs

- `architecture-design_required: true`.
- Architecture cannot change MI-S01..MI-S14 behavior, fields, actions, defaults, free/member separation, no-send, unavailable behavior, or acceptance.
- It decides capability/native state owners, repository/ports, event fan-out, route/store integration, DTO grouping, fixed resource loading, Sidecar cleanup, migration order, and complexity budget.
- Inspect command stubs/registry, shell member status, legacy stores/services, initial SQL, accepted-event consumer, capabilities/features, and direct callers.
- Return to spec for changed product behavior/scope/acceptance/API meaning. Verification later records architecture compliance/assessment.

## Production Code Quality Constraints

- Types first: Rust enums/DTOs and matching TS contracts before logic. Nominally distinguish session ID, UID, order ID, token, module ID, generation ID, and path IDs where confusable. No `any` or broad assertions.
- Fail fast on invalid signature/time/enum/module/provider/install/event/transition. Never collapse error to active/success.
- `null` is explicit absent native value; `undefined` is uninitialized frontend state. Validate required fields instead of success-looking fallback.
- Pure functions/immutable snapshots over classes. Rust actor/task types only for real lifecycle; no frontend manager/factory hierarchy.
- Naming uses boolean prefixes, `on*` props, `handle*` events, precise data verbs. Constants have narrow owners and never hide styling.
- Lists/forms/async actions own loading/error/empty/disabled termination. Readability beats micro-optimization.

## Human Review Readiness Constraints

- Match gateway/contracts, setup stores, route meta/projection, Element Plus/Tailwind, Rust command/application/domain/infrastructure, DTO/error/repository, and focused test conventions.
- Allowed: member Rust/IPC/repositories/migrations/build/dependencies, member Vue routes/stores/views/components/contracts/tests, shell projection integration, and replaced legacy member deletion.
- Forbidden: completed free/media/shell behavior, PRD, unrelated docs, extension/release scope.
- Risks: large diff, compile exclusion, networking, secret/file/process/task ownership, migrations, event state, cleanup.
- Before verify map hunks to tasks, scan callers/imports/registry, remove dead/debug/temporary/stale artifacts/churn, and record evidence/limitations.

## Functional-Programming Constraints

- Pure: QR mapping, entitlement/offline decision, capability projection, module transitions/progress, AI precondition/assembly/filter/truncate, moderation detection/mapping, DTO-view mapping, payload validation.
- Treat DTOs, props, events, snapshots, manifests, contexts, rules immutable. Mutate only owned Rust state/store actions/dialog refs.
- Side effects only in native ports/repositories/task runtimes, desktop command/listener, Pinia actions, UI handlers, lifecycle setup/teardown.
- Adapters own semantic normalization; computed is view-only; templates/watchers do not repair DTOs or duplicate state.
- No functional library, point-free pipeline, generic reducer, or hidden mutable cache.

## Architecture Reuse And Shared Ownership Constraints

| Candidate | Existing/callers | Anti-DRY conclusion | Routing |
| --- | --- | --- | --- |
| member capability projection | shell + member routes | same stable domain rule, 3+ consumers | one member owner; migrate consumers |
| legacy AI/ASR/moderation stores | separate browser services | related but different workflows/lifecycles | replace separately over shared capability |
| module/model downloads | membership + ASR | shared integrity/resume, different policies | evaluate native mechanism with injected policy |
| audit tables | AI + moderation | syntactic similarity, different semantics/actions | keep separate; reuse design-system table only |
| paused notice | AI + moderation | same gate, 2 callers | reuse `MembershipPausedNotice` |
| status/error adapters | gateway + member feature | technical normalization, multiple consumers | reuse gateway error; no catch-all utils |

Share stable capability/integrity mechanisms, not account/order/AI/moderation policies. Promoted APIs need JSDoc `@see`/examples, correct dependency direction, injected external/clock/file/process ports, and equivalence tests.

## Locality And File-Extraction Constraints

- External adapters, repositories, signed-download runtime, and typed contracts are separate for I/O/lifecycle/test boundaries.
- Stores are lifecycle owners. Small page-only format/guard helpers remain local.
- Extract shared capability/download mechanism only after architecture confirms callers/API/direction. No `utils.ts`, `helpers.ts`, manager/factory wrapper, or one-function file.
- Dialogs follow page design; adapters/transport never enter SFCs.

## Vue Component Extraction Constraints

`design/page-design.md` is normative. Extract its named single-caller dialogs/lists/drawers only for documented business, lifecycle, focus, or focused-test boundaries. Reuse `MembershipPausedNotice` for two same-semantics callers. Keep account summaries, AI prompt/result bands, and domain log tables inline/separate. Props are normalized view fields, emits are intentions, no speculative slots, and stores/requests/normalization stay outside presentational SFCs.

## Design Constraints

- Preserve application-shell geometry/navigation/status ownership and page-design density; no marketing surface.
- Keep account, entitlement, readiness, generation, and mute verification distinct.
- Only a signed validated fixed contribution contract may extend compiled member surfaces; no arbitrary downloaded execution.
- No hidden global event bus, duplicated writable projection, browser persistence, raw filesystem access, or UI polling.

## Project Bootstrap And Scaffold Decision

Not applicable as a new scaffold: the approved Tauri v2 + existing Vue/Vite scaffold is implemented by `platform-foundation`. This module extends it and creates no second runtime/package/frontend.

## Change Axes And Pattern Decision

- Signals: state-dependent behavior, contract adaptation, ordered workflow, retry/cancel, access control, side-effect coordination, build selection.
- Depth: Level 2 existing command/gateway/store/adapter/state patterns; Level 1 typed unions/state tables locally. Level 3 only if architecture approves a shared native signed-download/task boundary.
- Choose reuse plus lightweight state tables/adapters/store actions/components.
- Reject page branching (scatters security/state), runtime flags (cannot exclude binary), class managers/factories (ceremony), generic event bus (hidden lifecycle), arbitrary module/plugin loader (scope/security expansion).
- Overbuilt signs: one-variant wrapper, unowned single-caller file, broad optional DTO, extra store-gateway hop, or shared code importing private AI/moderation types.

## Code Context And Impact Assumptions

- Six current member commands are fail-closed stubs compiled/registered only under `member`.
- Shell has member slots/status but currently projects unavailable; replace its source without duplicate state.
- Legacy AI/ASR/moderation stores use localStorage/browser services and are removal targets.
- Initial SQL has account/membership/order/module/AI/ASR/moderation tables. Schema changes are additive migrations.
- Package tsconfig is strict, no-unused, ES2020/DOM, bundler resolution, Vue JSX preserve, `@/*`, Vite/Vue declarations. Relevant contracts are under `platform/desktop/contracts` and imported generated declarations.
- Vue code graph remains unavailable; scoped `rg` caller scans plus compiler/build graph are the approved substitute.

## API And Data Contracts

### Native Commands

| Capability | Operations | Result/error |
| --- | --- | --- |
| bootstrap | `member_status` | normalized account/entitlement/modules/capabilities or unavailable/error |
| auth | `auth_qrcode_generate`, `auth_qrcode_cancel`, `auth_query_uid`, `auth_bind_uid`, `auth_logout` | session/identity; never Cookie/token |
| membership | `membership_purchase`, `membership_cancel_order`, `membership_orders` | catalog-selected order/status; unavailable allowed |
| modules | `module_list`, `module_update`, `module_cleanup` | signed inventory/actions; no raw path |
| AI | `ai_config_get`, `ai_config_save`, `ai_generate`, `ai_regenerate`, `ai_log_query` | config/generation/log; no send semantic |
| ASR | `asr_status`, `asr_model_download`, `asr_start`, `asr_stop` | verified model/runtime/progress |
| moderation | `moderate_status`, `moderate_set_auto`, `moderate_update_rules`, `moderate_mute`, `moderate_ignore`, `moderate_log_query`, `moderate_mark_false_positive` | rules/alerts/action/log |

Each uses one typed request and response. Stable errors include `membership_required`, `module_required`, `external_service_unavailable`, `invalid_signature`, `offline_window_expired`, `stale_operation`, `rate_limited`, `timeout`, `credential_expired`, `artifact_invalid`, and validation errors. Existing frontend error mapping localizes safe messages.

### Native Events

| Event | Payload |
| --- | --- |
| `auth_qrcode_status` | session ID, exact QR state, optional safe identity/error |
| `auth_logged_in` | public account snapshot |
| `membership_expired` / `membership_renewed` | verified entitlement/capabilities |
| `module_download_progress` / `module_installed` | operation ID, module/version, verified bytes/state |
| `ai_generation_delta` / `ai_generation_finished` | generation ID, monotonic sequence, delta/terminal state |
| `asr_status_changed` | model/runtime state, no raw audio |
| `moderation_alert` / `moderation_action_result` | alert/action IDs, safe evidence/result |

One root member runtime listener owns subscriptions/teardown and dispatches to stores. Pages never `listen` directly.

### External Contracts

- Bilibili QR uses PRD-documented generate/poll/nav/manual-UID endpoints. Rust preserves upstream fields privately and maps approved public identity/states. Each session has isolated cookie storage; Cookie is memory-only.
- Membership/payment/token/manifest endpoint, schema, public key, catalog/currency, states, and host are absent. Production returns unavailable; deterministic signed test adapters cannot ship as success.
- LLM provider/stream contract comes from approved configuration. API key stays native and is never returned clear after save. Missing provider is unavailable.
- Moderation Cookie API contract is absent. Local detection works; mute transport is unavailable and auto stays off.
- Whisper manifest/hash/binary/model/license artifacts are absent. PRD example metadata may appear in tests, not production success.
- No protobuf/OpenAPI/backend TS types exist. Rust DTOs are authoritative desktop contracts; TypeScript preserves backend field names through explicit adapters.

### Persistence

- Repositories own public account file and encrypted membership/order/module/config/log data. Secrets never reach Vue.
- Additive migrations correct schema gaps; invalid/tampered existing entitlement becomes inactive.
- Native timestamps are UTC; adapters format only fields defined above.

## Frontend / Server Responsibility Boundary

- Vue: routes, drafts, semantic controls/focus, clipboard, pending/error presentation, intention dispatch.
- Rust: validation/authorization, Bilibili requests/polling/Cookie isolation, signatures/clocks, persistence, download/hash/extract, task/process lifecycle, context assembly, LLM/filter/truncate, detection/retry/audit, safe events.
- External: membership/payment/token/manifest truth, LLM transport, Cookie mute API, Whisper artifacts.
- Gaps do not block local/domain/UI and honest unavailable states; they block production success claims.

## Context And Dependency Sources

- PRD 4, 5, 9.4, 9.14-9.16, 10.4, 12-14; snapshot/map/module/page design.
- Current Tauri commands/build/registry/database; renderer router/shell/legacy stores; tsconfig/contracts.
- Existing Tauri v2, Vue 3, Pinia, Vue Router, Element Plus, Tailwind-style utilities, Vitest, SQLx, Tokio, and Cargo feature conventions. New dependencies require architecture and dual-build proof.

## Edge Cases

- Free registry/bundle/route leakage and pasted member URL.
- Stale QR poll after refresh/cancel, network pause, fifth failure, nav failure, UID changed after query.
- Login without entitlement; invalid token after paid response; clock tamper; exactly/over seven offline days; expiry races.
- Duplicate purchase, cancel/timeout race, resume, invalid bytes/path/archive/hash, partial commit, same-version renewal, cleanup while running.
- Duplicate/out-of-order/stale generation, route exit, prompt/context combinations, timeout/rate limit, sensitive hit, multibyte truncation, copy denial.
- Corrupt ASR model, missing license acknowledgement, microphone denial, Sidecar crash, expiry while capturing.
- Risk cancel, unknown rule range, duplicate alert, retry outcomes, Cookie expiry, expiry during mute, ignore failure, whitelist update.
- Missing/unknown/null external values never render active/success.

## Acceptance Criteria

1. Free registry/bundle has no member commands/routes/modules/labels; member build has only approved surfaces.
2. QR mapping/timing/failure/cancel/stale handling, UID confirmation, account persistence, and Cookie non-persistence pass deterministic tests.
3. Tampered/expired/over-seven-day tokens never activate; valid offline token works through seven days; startup/hourly/call checks pass.
4. Module install is confined, resumable, hash-verified, transactional, removes corrupt temp, retains at expiry, activates after verified completion.
5. Controlled adapter tests meet <=5-minute install and <=1-minute renewal; missing production service is unavailable without free regression.
6. AI is explicit, assembly <=500ms, default timeout 5s, streaming/filter/truncate/log correct, and no send UI/command/request/task exists.
7. ASR license/integrity/lifecycle/30-second cache and >=90% defined-fixture accuracy evidence pass; failures isolate manual AI.
8. Moderation defaults off, gates first enable, uses sourced defaults, retries once, meets <2s/>98% available-adapter targets and >95% detection/<3% fixture targets, retaining alerts on fail/expiry.
9. Login/unpaid/install/active/expired/renew/module-missing/unavailable routes and focus/keyboard/disabled/error states work.
10. Legacy member paths have no production caller; static/tests/free-member web and Tauri builds and all applicable framework verdicts pass.

## Human Review And Handoff

- Separate implemented local/native behavior from unavailable production integrations.
- Evidence includes source/registry/bundle scans, hunk-task mapping, dual binaries, unit/component/integration tests, runtime screenshots/interactions, and reasons for unrun checks.
- Required applicable verdicts: source/spec/pattern/intent/change/removal/production/human-review/expert/field/functional/architecture/reuse/styling/API/TypeScript/self-healing/code-review all pass.

## Risks

- Non-public Bilibili endpoints may change; adapter failure preserves UID fallback and no Cookie leak.
- Missing membership/payment/key/manifest/Cookie/LLM/Whisper contracts permit unavailable production behavior only.
- Downloaded frontend code is a CSP/supply-chain risk; fixed signed resources only.
- Initial schema has credential-like account columns and incomplete moderation audits; additive migration/secret ownership need architecture review.
- Long-running tasks/events can race/leak; one listener/task owner and shutdown are blockers.

## Spec Approval Check

- Template, source/closed-world traceability, module/intent, function-complete behavior, field semantics, API/type/server boundaries, quality/pattern/architecture/styling/testing/review constraints, and unavailable decisions: pass.
- Framework decision: automatically approved; advance to `architecture-design` without changing `loop`.
