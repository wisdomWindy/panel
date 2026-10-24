# 执行记录

## Delivery Unit

`captain-effect-window-empty`；approved spec/plan/architecture-design。

## Execution Summary

TDD 已完成 runtime active read model、current IPC contract、nullable renderer adapter 和 listen-then-query 窗口编排。全量回归发现 effect capability 仍为空，生产 WebView 将无法调用新命令；该 `plan-gap` 已回流计划并补入最小权限任务。

## Self-Healing Notes

- iteration: 5，initial execute 内部回流 plan -> execute。
- defect: `plan-gap/effect-capability-current-command`，attempt 1/1。
- evidence: `capabilities/effect.json` permissions 为 `[]`，而 `EffectWindowRoot` 需要调用 `captain_effect_current`。
- repair scope: 新增单命令 permission，effect capability 仅引用该 permission；不授予 foundation。
- focused recheck: Rust capability contract、desktop build。

## TDD Evidence

- RED 1：缺少 `EffectRuntime::active_effect` 编译失败；GREEN：preview active/clear 通过。
- RED 2：缺少 live audience 字段编译失败；GREEN：真实 audience 保留测试通过。
- RED 3：缺少 Rust payload mapper 编译失败；GREEN：preview DTO 通过。
- RED 4：缺少 TS nullable adapter/command，2 个 focused tests 失败；GREEN：8 tests 通过。
- RED 5：effect root 缺少 current query，contract test 失败；GREEN：13 tests 通过。

## Changed Files And Owned Symbols

- Rust runtime：`application/media/effects.rs` 的 `ActiveCaptainEffect`、`EffectQueueState.active`、`EffectRuntime::active_effect` 及 worker active 建立/清理；`EffectItem` 保留 live audience。
- Rust IPC/注册：`ipc/media.rs` 的 `captain_effect_current`、`captain_effect_payload` 和共享 builder；`lib.rs` 注册 free/member handler；相关 DTO/contract tests 同步。
- 权限：`permissions/foundation.toml` 新增单命令 permission，`capabilities/effect.json` 仅授予 `captain-effect-current`；foundation/main 未扩大权限。
- Renderer：desktop media command map 增加 nullable response；media adapter 增加 nullable 适配；`EffectWindowRoot.vue` 实现 listener-then-query、event-wins 和 unmount cleanup；相邻测试同步。
- 无新生产文件、组件、store、通用 helper 或第三方依赖；未修改 surface、模板视觉、样式、窗口几何、音频与其他 media 能力。

## Introduced-Content Provenance Ledger

所有 hunk 均为 `product-content delta: none`：active/read command、nullable adapter、竞态保护、最小权限和测试是恢复 U1 既有特效显示的技术闭包。preview synthetic identity 只为复用既有 payload 合同，画面仍显示原有 `用户A`；未新增产品文案、控件、字段、导航、模板或状态语义。

## Architecture / Quality Notes

active 单 owner、DTO adapter、Vue lifecycle 边界按 ADR-1/2 落地。event/query 共享 Rust payload builder，nullable 仅在 command adapter 外层表达；Vue 单调用方竞态逻辑保持本地。无 cache/debounce/retry/双路径；监听在卸载时释放，迟到 query 受 active guard 与 revision guard 约束。TS strict/bundler/`@/` alias 已由 package tsconfig 确认。

## Verification Handoff

- `cargo test --features free`：122 passed，0 failed。
- `cargo fmt --check`：通过。
- `pnpm exec vitest run --mode free`：44 files / 140 tests passed。
- `pnpm type-check`：通过。
- `pnpm lint:check`：0 error；仅 `logger.ts` 两条既有 `no-console` warning，与本次无关。
- `pnpm build:web:free` 与完整 `pnpm build:free`：通过；NSIS 安装包生成。
- release 进程 smoke：新进程曾成功响应；因本机已有旧实例，新进程随后退出，未把该结果冒充原生窗口交互验收。
- 变更文件 `git diff --check`：通过；定向 symbol closure 检查无孤立 command、permission、adapter、active 或 listener cleanup 路径。

## 2026-09-23 Reopen Execution

- Loop iteration 3, manual reopen for the same user-visible empty-window defect; narrower diagnosis: the restricted effect capability lacked Tauri event listen/unlisten permissions, and listener rejection skipped the current-payload query.
- TDD RED: `foundation_contracts` saw only `captain-effect-current`; `EffectWindowRoot.test.ts` observed `['listen']` instead of `['listen', 'current']` after a rejected subscription.
- GREEN: `effect.json` now grants only `captain-effect-current`, `core:event:allow-listen`, and `core:event:allow-unlisten`; `EffectWindowRoot.vue` keeps listener-before-query ordering with independent error boundaries.
- Changed production owners: effect-window capability and effect-window startup lifecycle. New test owner: native Vue lifecycle regression using a custom renderer and mocked external desktop transport.
- Product-content delta: none. No template, canvas drawing, copy, field, control, style, navigation, payload semantic, window geometry, or unrelated media behavior changed.
- Pre-review self-check: task-owned hunks only; no new production file, shared abstraction, debug output, stale comment, unused import, or formatting churn introduced by T4.
- GREEN/full evidence: focused Vitest 7/7; full Vitest 144/144; Rust 129/129; type-check pass; fmt pass; Clippy with `-D warnings` pass; lint 0 errors with two pre-existing logger warnings; web and Tauri free builds pass.
- NSIS: `src-tauri/target/release/bundle/nsis/biliLiveHelper_0.1.3_x64-setup.exe`, 7,422,260 bytes, SHA-256 `b3bdce721adb79842692bf6d2bfd541e705845535fd08676233b23656b85ff9d`.
- Remaining verification boundary: native click-through still requires launching the newly built desktop artifact; automated evidence now covers the actual permission contract and the listener-failure/query-continuation behavior that caused the blank window.

## 2026-09-23 T5 Captain Purchase Semantic Cleanup

- Source: U2 / AC9-AC12. Product-content delta: removed the contradictory room-entry sentence; added no replacement copy.
- RED: added a real-source contract assertion that `CaptainEffectSurface.vue` must retain action/level/username rendering and must not reference `captainEffect.welcome`. Focused Vitest failed only on the forbidden reference.
- GREEN: removed the welcome draw call and its two orphan locale entries. No geometry, animation, payload, event, accessibility, title, menu, or configuration behavior changed.
- Focused evidence: media-page plus i18n tests passed, 2 files / 14 tests.
- Change-chain closure: production/source locale search found no entry sentence or welcome-key reference; the only remaining key text is the test's negative assertion. `LiveEvent::Captain`, purchase/renewal mapping, action label, level, and username render references remain intact.
- Pre-review: no new file, abstraction, style, type, dependency, debug code, dead code, or unrelated formatting; targeted diff check passed.
