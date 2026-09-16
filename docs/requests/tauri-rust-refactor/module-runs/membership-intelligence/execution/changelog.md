# Membership Intelligence Execution Changelog

## T01 - Contract And Red-Test Foundation

- status: completed
- source: MI-S01..MI-S14; ADR-MI-01/02/05/07; plan T01
- product-content delta: none
- changes:
  - added member registry red test; observed member build failure `left: 6, right: 28`
  - expanded the member registry truth table to the 28 approved non-send operation names; focused member/free tests pass
  - added strict readonly TypeScript command/event contracts and branded member identifiers
  - added member Rust domain identifiers/status snapshots and safe camelCase status/QR DTOs
  - added frontend platform contract tests and Rust serialization/no-secret tests
  - refreshed scoped code context and corrected enum field serialization to camelCase after self-review
- evidence so far:
  - package TypeScript typecheck: pass
  - frontend platform Vitest: 5 files, 13 tests pass
  - member focused Rust contract tests: 2 pass
  - free member-surface Rust contract test: 1 pass
- introduced-content provenance: command/event names and status fields map to the approved spec API tables; all other hunks are technical contracts with product-content delta none.

## T02 - Rust Domain, Persistence, And Safety

- status: completed
- source: MI-S02/04/05/07-10/12; ADR-MI-02/03/04/06; plan T02
- product-content delta: none
- changes:
  - added pure entitlement/capability/progress/AI filtering rules and exact QR/offline/latency/moderation constants
  - added schema v3 migration that removes plaintext account tokens/API-key storage, uses integer minor amounts, narrows module paths, and adds moderation alert/audit storage
  - added explicit member repository, authentication, commerce, entitlement, artifact, LLM, ASR, and moderation ports
  - added SQLite member repository with validated row mapping, encrypted-byte persistence boundaries, and single-account transactions
  - added Ed25519 verification, SHA-256 verification, confined module paths, archive traversal rejection, and stable fail-closed external adapters
- TDD evidence:
  - domain rule red: 6/6 tests failed against deliberate stubs; green: 6/6 pass
  - schema red: missing v3 tables/marker; green: 4/4 database tests pass with sensitive-column assertions
  - member infrastructure: 5/5 pass, including tamper/hash/path/corrupt-row/redaction coverage
  - member Rust full suite: 74/74 pass
  - free Rust full suite: 62/62 pass
- introduced-content provenance: numeric thresholds and lifecycle constants are exact PRD values; persistence/port/error structure is technical implementation with product-content delta none.

## T03 - Rust Application, IPC, Events, And Build Wiring

- status: completed
- source: MI-S01..12; ADR-MI-02..06; plan T03
- product-content delta: none
- changes:
  - added three explicit state owners for membership, AI/ASR, and moderation
  - replaced six placeholder handlers with all 28 typed member commands and strict camelCase DTOs
  - added accepted-event fanout so media and moderation consumers run independently on accepted events
  - added signed entitlement/module capability projection, pre-command checks, hourly validation, AI dedupe/cancel cleanup/default timeout, moderation detection and one retry
  - wired member-only repository/adapters/event sink into AppState and registered exact handlers only in member builds
  - added window shutdown cleanup for validation tasks and ASR
- verification:
  - member application state tests: 5/5 pass
  - member Rust full suite: 80/80 pass
  - free Rust full suite: 63/63 pass
  - strict Clippy member/free: pass with `-D warnings`
- introduced-content provenance: command/event/state behaviors map to approved spec; technical event fanout and task ownership add no visible product content.

## T04 - Frontend Channel, Stores, And Runtime

- status: completed
- source: MI-S01/03/04/06-12/14; ADR-MI-01/05/07; plan T04
- product-content delta: none
- changes:
  - added compile-time free/member Vite channel implementations; free channel has no member routes/runtime/store imports
  - added strict unknown-to-readonly adapters for account, entitlement, module, order, AI, ASR, and moderation payloads
  - replaced browser/localStorage AI and moderation stores with typed desktop-command Pinia stores; added membership store
  - added one member runtime owning all 11 listener lifecycles, partial-registration rollback, stale guards and disposal
  - added member routes and capability-based navigation/route guards without page-level permission logic
- verification:
  - TypeScript typecheck: pass
  - focused member adapter/channel/runtime/store/navigation tests: 13/13 pass
- introduced-content provenance: route titles and navigation labels reuse approved PRD terminology; remaining changes are technical boundaries with no product-content delta.

## T05 - Membership UI

- status: completed
- source: MI-S02..05/11..13; approved Account/Membership allowlist; plan T05
- product-content delta: none
- changes:
  - added member-center state surfaces for loading, logged-out, unpaid, active, expired, and unavailable states
  - added QR/manual-UID login, explicit identity confirmation, account identity, entitlement dates, modules, orders and logout controls
  - extracted login, native-plan unavailable, module management, and order interaction boundaries
  - added member-only toolbar control through the build channel; free toolbar receives no member component or copy
  - kept catalog/payment/module-success states fail closed because production service contracts are absent
- verification:
  - TypeScript typecheck: pass
  - focused page/channel/adapter/runtime/store tests: 11/11 pass
- introduced-content provenance: visible labels and degraded copy reuse PRD/page-design allowlist; no plan, amount, order or success copy was invented.

## T06 - AI And ASR UI

- status: completed
- source: MI-S06..08/11/12; approved AI allowlist; plan T06
- product-content delta: none
- changes:
  - added an explicit-generation AI workspace with bounded accepted-danmaku context and a permanent local-preview/no-send result label
  - added native-backed persona/model configuration for the four approved styles without provider or model presets
  - added ASR status and license/download gating; absent model artifacts remain unavailable and cannot report download success
  - added a separate source-field-only AI audit route and strict wire-data adaptation
  - added a shared membership-paused surface for member capability loss
- verification:
  - TypeScript typecheck: pass
  - focused AI page/adapter/store tests: 7/7 pass before the added log adapter assertion
- introduced-content provenance: labels, fields, actions and degraded states reuse PRD/page-design allowlist; no send action, automatic trigger, provider preset or model artifact was introduced.

## T07 - Moderation UI

- status: completed
- source: MI-S09..12; approved Moderation allowlist; plan T07
- product-content delta: none
- changes:
  - added moderation state, fixed warning, source-default rule configuration and today counters
  - added the exact four-statement first-enable acknowledgement with native-result-only switch updates
  - added alert inspection, credential-gated manual mute, persisted ignore and expired-state blacklist fallback
  - added a separate moderation audit route with user/time filters and false-positive action
  - kept mute/log/false-positive integration fail closed because the external Cookie API and log implementation are absent
- verification:
  - TypeScript typecheck: pass
  - focused AI/moderation page, adapter and store tests: 11/11 pass
- introduced-content provenance: warning, rules, duration values, actions and degraded copy reuse PRD/page-design allowlist; no unsupported mute, appeal, whitelist or unmute success was introduced.

## T08 - Cleanup, Verification, And Review

- status: completed
- source: MI-S01..14; all acceptance criteria; plan T08
- product-content delta: none
- changes:
  - removed the closed legacy AI, ASR, and moderation browser/Electron service, engine, composable, type, and store dependency chains
  - moved member navigation labels and route contributions behind the compile-time channel so the free bundle contains no member routes, labels, components, or commands
  - made Cargo free/member features mutually exclusive and updated Rust contract tests to the 28-command member registry
  - retained unavailable states for missing production membership, artifact, LLM, ASR, and moderation contracts instead of simulating success
- verification:
  - frontend free/member full Vitest: 33 files and 89 tests pass in each channel
  - TypeScript typecheck: pass; ESLint: 0 errors with 63 pre-existing warnings outside this module
  - Rust free: 63 library plus 18 contract tests pass; member: 80 library plus 18 contract tests pass
  - Rust fmt and strict Clippy for both feature sets: pass
  - free/member Vite builds and free/member Tauri MSI/NSIS builds: pass
  - production send/localStorage/Electron scans and free-bundle member leakage scans: zero matches
  - runtime member unavailable state inspected at 1280x720; one non-duplicated error surface and no overlap observed
- artifacts:
  - free MSI 7,557,120 bytes; free NSIS 5,356,206 bytes
  - member MSI 7,942,144 bytes; member NSIS 5,601,223 bytes
- residuals:
  - production external service contracts and signed artifacts remain intentionally unavailable
  - Vite reports a main-chunk size warning; bundle optimization belongs to the release-resilience module
- introduced-content provenance: cleanup and verification only; no new product copy or behavior.
