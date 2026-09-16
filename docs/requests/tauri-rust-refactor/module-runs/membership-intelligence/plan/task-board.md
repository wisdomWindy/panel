# Membership Intelligence Task Board

## 全局执行合同

- speed/context: `S3 broad`；只读 member 功能链、共享 shell/desktop/live-event/build/migration 直接闭包。
- executor: 当前 Codex 主 agent；模块整体串行，T05-T07 在 T04 后可逻辑并行但默认依次执行。
- common quality: type-first, fail-fast, strict null, immutable DTO/snapshot, exact side-effect owner, no `any`, no magic globals, no speculative abstraction, no unapproved copy/behavior.
- review: 每个 hunk 必须映射任务/source/cleanup/test；禁止无关格式化、旧注释、debug、dead/unused/stale 代码和 mock。

## T01 契约与红测试

- status: completed; execute-readiness: ready; mode: serial.
- prerequisite: approved spec/architecture; covers MI-S01..14, ADR-MI-01/02/05/07.
- units/scope: Rust member models/DTO contract, TS desktop contract, registry/build expectations, caller/code-context refresh.
- target: `domain/member/{mod.rs,models.rs}`, `ipc/member.rs`, `ipc/member_contract_test.rs`, `platform/desktop/contracts/member.ts`, contract indexes and focused tests.
- interaction/state: all enums/IDs/null semantics before implementation; no UI behavior yet.
- source/product delta: source-backed/code-fact-backed; `none`.
- API/type: Rust authoritative DTO; readonly TS union/branded IDs; no external raw DTO.
- tests: serialization/unknown enum/free-member registry/no-send and adapter red tests.
- FP/pattern/locality: Level 1 union/mapper + existing Level 2 contracts; contract files justified by generated/boundary isolation.
- change-chain: scan App/router/shell/stores/services/engines/composables/tests before edits; append code-context.
- assumptions: only reversible file grouping from architecture; no product assumption.

## T02 Rust 领域/持久化/安全

- status: completed; execute-readiness: ready; mode: serial after T01.
- covers: MI-S02/04/05/07-10/12; member rules, ports, repositories, migration, verifier/artifact/unavailable adapters.
- target: `domain/member/rules.rs`, `application/member/ports.rs`, `infrastructure/member/{mod.rs,repository.rs,external.rs,artifacts.rs,tests.rs}`, migration 0002.
- rules: exact QR/7d/module/AI/moderation rules from plan; invalid/null/partial states fail closed.
- source/product delta: source-backed; `none`.
- API/type: opaque IDs/secrets, integer UTC, minor amount+currency, private SQL/upstream rows.
- tests: clock/signature/tamper/transition/path/hash/transaction/redaction/AI/moderation fixtures.
- FP/side effects: pure rules immutable; SQL/file/crypto/network only injected ports/repository.
- reuse: capability derivation + artifact integrity extracted; audit/config state machines remain separate per Anti-DRY.
- review: no plaintext secret or path serialization; migration additive; no god utils/factory.

## T03 Rust 应用/IPC/事件/构建

- status: completed; execute-readiness: ready; mode: serial after T02.
- covers: MI-S01..12; ADR-MI-02..06.
- target: `application/member/{mod.rs,state.rs,tests.rs}`, `ipc/member.rs`, live-data fanout owner/tests, `build_channel.rs`, `lib.rs`, Cargo dependencies.
- units: three state owners, read-only capabilities, 28 commands, sourced events, QR/order/install/ASR/generation/mute tasks, shutdown.
- source/product delta: source-backed API/state semantics; no visible copy.
- async: generation IDs, dedupe, cancel, stale rejection, one mute retry, consumer failure isolation, hourly/pre-command checks.
- API: unavailable missing external contracts; Bilibili auth isolated cookie client/public identity only.
- tests: Tokio time/race/shutdown/fanout/registry/error redaction; both features compile/test.
- pattern: Level 2 ports/Command + Level 1 State + small Composite; reject bus/manager/factory.
- review: functions >150 mixed responsibilities split; every timer/task/listener/process disposed.

## T04 Frontend channel/contracts/stores/runtime

- status: completed; execute-readiness: ready; mode: serial after T03.
- covers: MI-S01/03/04/06-12/14; ADR-MI-01/05/07.
- target: Vite channel alias, `channel.free.ts`, `channel.member.ts`, `adapters.ts`, `stores/{membership,ai,moderation}.ts`, `useMemberRuntime.ts`, App/router/navigation/i18n/contracts/tests.
- state: three store snapshots; pending/error operation sets; runtime owns listeners; App consumes derived status; route guard follows capability.
- source/product delta: allowlist route/navigation/status labels only; no missing-source copy.
- API/type: validate unknown -> readonly view; null/undefined strict; no localStorage/fetch/browser engine/direct listen in pages.
- tests: free empty contribution, member routes, adapter invalid data, dedupe/stale/dispose/navigation matrix.
- FP/pattern: Level 2 gateway/store/build Adapter; normalization pure in adapter; computed view-only.
- change-chain: do not delete legacy paths until T08; no dual production callers after T04 consumers migrate.
- review: no new memo/cache/debounce; one listener group; free bundle import graph proof deferred T08.

## T05 Membership UI

- status: completed; execute-readiness: ready; mode: parallel-capable after T04.
- covers: MI-S02..05/11..13 and Account/Membership allowlist.
- target: `views/membership/index.vue` and named membership components/tests from page design.
- states/actions: logged-out/unpaid/payment/install/active/expired/unavailable; exact QR/UID/order/module/renew/logout rules; pessimistic pending and focus restoration.
- fields: public identity exact; catalog-native amount/currency; order exact; expiry date-only; whole days; module exact version/status/progress; unknown is error.
- input: UID trims only at submit, rejects empty/non-decimal, no length rule; validation on submit and backend error association.
- styling/a11y: Element Plus + inline-reviewable Tailwind utilities; keyboard/focus/live region/table overflow/1024x680; no scoped CSS.
- components: approved single-caller lifecycle/business/test exceptions; props are view models, emits intentions, no slots/store/gateway.
- source/product delta: exact allowlist only; no promotions/extra plans/empty/confirmation prose.
- tests: all visible branches, actions, disabled/loading terminal, no optimistic activation and field formats.

## T06 AI/ASR UI

- status: completed; execute-readiness: ready; mode: parallel-capable after T04.
- covers: MI-S06..08/11/12 and AI allowlist.
- target: `views/ai/{index.vue,log.vue}`, approved AI components/shared paused notice/tests.
- fields/input: raw free text/newlines, four style values, native provider/model/ranges, context source/text, readonly local result, sourced log fields only.
- state/actions: explicit Generate, prompt-or-context, pending dedupe, sequence/stale handling, Regenerate/Copy/Clear, ASR license/model/mic lifecycle, expired pause.
- source/product delta: exact AI labels/errors/actions; permanent local-only/no-send label; no provider presets/send UI/empty copy.
- styling/a11y: bounded scroll, keyboard context selection, selectable result, polite terminal announcements, responsive controls.
- tests: no auto generation, all input combinations/failures, stream race, filter/truncate, clipboard, ASR gates, expired/renewal, no send source/registry.
- pattern/locality: page prompt/result inline; dialogs/list extracted for approved lifecycle/test boundaries; no generic log table.

## T07 Moderation UI

- status: completed; execute-readiness: ready; mode: parallel-capable after T04.
- covers: MI-S09..12 and Moderation allowlist.
- target: `views/moderation/{index.vue,log.vue}`, approved moderation components/tests, shared paused notice.
- fields/rules: exact fixed warning, defaults, sourced duration choices, alert/user/evidence/count/log fields; unknown editable range is unavailable.
- state/actions: default off, first-enable acknowledgement, manual confirmation, pending until verified, one retry native, failed/expired alert retention, ignore-after-persist, false-positive whitelist.
- source/product delta: exact allowlist risk/error/action copy; no unsupported mute/unmute/appeal success or empty copy.
- styling/a11y: text-plus-state, checkbox association, focus restore, alert stack/overflow, disabled reasons, Tailwind only.
- tests: risk gate, Cookie/API/expiry, retry/false-success prevention, ignore rollback, filtering/whitelist, responsive/keyboard.
- pattern/locality: distinct domain UI, no shared audit/config abstraction; extracted components only approved exceptions.

## T08 Cleanup, verification, review evidence

- status: completed; execute-readiness: ready; mode: serial after T05-T07.
- covers: MI-S01..14; all acceptance criteria and applicable framework verdicts.
- target: legacy AI/ASR/moderation services/engines/composables/types/stores/tests/mocks/imports plus verification/changelog/review artifacts.
- removal: close full dependency chain; test-only references adapt/delete; retain no localStorage/browser member behavior/Electron caller/send symbol.
- source/product delta: remove unapproved legacy presets/actions/storage; add nothing.
- verification: Rust fmt/free-member test/strict clippy; free-member Vitest/typecheck/lint/Vite/Tauri builds; registry/bundle/source/security scans; performance fixtures; runtime screenshots/a11y.
- evidence: artifact sizes/timestamps, exact counts, manual viewports, skipped reason, remaining external limitations, hunk-task map.
- self-heal: verification failure returns owning stage; review blocker returns execute; changed semantics returns spec; architecture mismatch returns architecture-design; no progress stops per loop policy.
- completion: all verdicts pass, blockers none, module review completed.

## 任务状态更新规则

- 开始一项时仅将该项改为 `in_progress`；红测试、实现、focused verification 都完成后才改 `completed`。
- 发现 spec/architecture/plan 缺口立即回流对应 stage，不在 execute 猜测。
- 当前开放的外部契约只允许 unavailable 分支，不影响 execute-readiness。
