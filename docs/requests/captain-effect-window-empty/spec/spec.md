# 工程规格

## Delivery Unit Identifier

`captain-effect-window-empty`，request-level bugfix。

## Background And Goals

窗口创建与事件发布没有可靠的首帧同步：预览不发布 payload，直播事件可能早于窗口监听器。目标是让窗口挂载后可读取 Rust runtime 当前活动特效，稳定完成首帧绘制。[U1, code-context]

## In Scope

- 活动特效状态保留绘制所需的 audience/action/level/config/isPreview。
- 新增只读 IPC 命令返回当前活动 payload 或 `null`。
- `EffectWindowRoot` 先监听实时事件，再补拉当前活动 payload。
- 覆盖预览、直播、无活动项及监听时序测试。

## Out Of Scope

模板视觉、动画算法、窗口尺寸/位置、音频、队列容量、其他媒体提醒。

## Trigger And Start Conditions

预览命令或真实上舰事件进入 `EffectRuntime` 队列；worker 将条目标记为 active 后创建/显示窗口。

## Requirement Split Summary

不适用：非拆分 bugfix，仅一条特效窗口数据链。

## Source Grounding And Traceability

| Spec item | Label | Product semantic | Source | Plan obligation |
| --- | --- | --- | --- | --- |
| 窗口显示当前特效 | source-backed | yes | U1 | 端到端修复并验证 |
| active query 返回 payload/null | code-fact-backed | no | runtime 是当前状态 owner；事件会丢失 | 新命令与 adapter |
| 监听后补拉 | code-fact-backed | no | `onMounted` 异步监听与新窗口竞态 | lifecycle 编排测试 |
| 保留原用户/动作/等级/配置 | source-derived | yes, supporting only | U1 + 现有 payload contract | 等价测试 |

闭集产品内容 allowlist：仅恢复现有上舰特效的可见显示；不得新增文案、控件、状态、导航、校验或模板语义。

## Workflow Efficiency Profile

`S2 scoped`，full artifact density，允许读取 captain-effect 的 Rust/runtime/IPC/Vue 契约闭包；若需要通用窗口总线或影响其他 media 事件则升级 S3。

## Self-Healing Completion Bar

- requirement：首次预览和真实事件均有 payload，视觉与配置语义不变。
- logic：active 的建立/清理、preview 标记、无活动项 `null`、竞态顺序正确。
- checks：focused tests、全量 Vitest、Rust tests、type-check、lint、build。
- runtime：监听失败和查询失败不得产生伪 payload；卸载后不写状态。
- organization：命令注册、contract、adapter、tests 无孤立路径或重复 DTO 构造。
- completion：self-healing compliance/assessment 均 pass，blocker none。

## User Intent Contract

- literal：特效窗口不能什么都没有。
- practical：首次打开就稳定看到当前特效。
- success：预览/直播共享可靠路径；不要求第二次触发。
- forbidden：延时重发、画静态背景掩盖空状态、只修预览或只修后续事件。
- acceptable：监听后查询 runtime 当前活动状态。
- verify：测试事件早于监听建立时仍能补拉并绘制 payload。

## User Flow

触发 -> runtime active -> 创建窗口 -> Vue 注册事件 -> 查询 active -> adapter 校验 -> surface 绘制 -> 时长结束清理并关闭。

## Page And Module Design

不改变页面结构。Rust runtime 拥有活动状态；IPC 转换 DTO；`EffectWindowRoot` 编排订阅与初始查询；surface 继续只负责绘制。

## UI / UX Design Contract

沿用当前透明窗口和现有动效。查询期间保持透明；成功取得 payload 后立即绘制。查询失败不伪造用户可见内容。

## Frontend Styling Constraints

不修改样式。

## Field Display Semantics And Visual States

沿用现有字段：username、action、level、primary/secondary color。格式、空值、可点击性和视觉状态均不变；本修复仅保证已有 payload 到达。

## Function-Complete Behavior Breakdown

1. worker 处理队列项时，在窗口创建前把该项及有效 config、`isPreview` 保存为 active。
2. 时长结束或 shutdown 时清理 active。
3. 只读命令将 active 转为现有 `CaptainEffectPayloadDto`；无 active 返回 `null`。
4. window root 完成 `listenDesktop` 注册后调用命令并适配结果；若期间事件已更新 payload，不允许旧查询结果覆盖更新事件。
5. 组件卸载后不再写入响应式状态并释放监听。

## Expert Frontend Engineering Constraints

- state owner：Rust runtime 单一 writable active；Vue payload 是窗口本地展示状态。
- lifecycle：event DTO 与 query DTO 均经过同一 `adaptCaptainEffectPayload`。
- async race：使用本地事件版本/是否已收到事件标记，阻止迟到 query 覆盖新事件。
- retry/cancel：窗口生命周期短，不自动重试；卸载 guard 生效。
- performance：每次窗口挂载一次查询，无 cache/debounce。
- testability：Rust active 单测 + TS lifecycle/contract 静态或组件测试。

## Frontend Architecture Design Routing Inputs

`architecture-design_required: true`。后续必须决定 active payload 的 owner/shape、DTO 构造复用、query 与 event 的依赖方向和最小文件范围；不得改变现有 payload 语义、视觉或验收条件。若需要通用事件持久化、全局 store 或新用户行为，回退 spec。

## Production Code Quality Constraints

- type-first：命令映射为 `CaptainEffectPayload | null`，禁止 `any`。
- fail-fast：adapter 拒绝 malformed non-null payload；`null` 是合法无活动态。
- strict null：`null` 与失败分开；失败保持空白并不中断卸载清理。
- naming：query 使用 `get/fetchCurrent...` 语义，事件处理使用 `handle`。
- pure conversion：active -> DTO 复用窄 helper；side effect 只在 command/lifecycle。
- 不为一次查询引入 class、cache 或通用 manager。

## Human Review Readiness Constraints

允许改动：media effects/state/IPC DTO conversion、lib command registration、desktop contract/adapter、EffectWindowRoot 及相邻测试。禁止无关格式化和模板视觉改动。review 重点是竞态、active 清理、DTO 等价与脏工作区隔离；需提供测试/type/lint/build 证据和 changed-hunk 映射。

## Functional-Programming Constraints

DTO/adapter 转换保持纯函数和不可变输入；runtime mutex 内 active 是明确状态副作用；Tauri invoke 和 listen 只位于 lifecycle/command 边界。不引入函数式工具库。

## Architecture Reuse And Shared Ownership Constraints

现有 `captain_config`、`audience_identity`、action/level DTO mapper 和 `adaptCaptainEffectPayload` 必须复用。Anti-DRY：event/query 是同一 payload 语义、两个 transport 入口，应共享 Rust DTO builder；owner 为 media IPC。活动状态仅一个生产 owner，留在 `EffectRuntime`，不抽共享 utility。

## Locality And File Extraction Constraints

不新增独立 helper 文件。Rust DTO builder 就近位于 `ipc/media.rs`；Vue 时序 helper 保持 `EffectWindowRoot.vue` 本地，除非测试设施证明需要已有边界。

## Vue Component Extraction Constraints

保持 `EffectWindowRoot` 与 `CaptainEffectSurface` 现有边界；无新组件候选。

## Design Constraints

透明窗口、时长和关闭行为保持不变；查询是恢复首帧状态，不是第二套状态流。

## Project Bootstrap And Scaffold Decision

不适用：现有项目缺陷。

## Change Axes And Pattern Decision

选择 Level 2 `reuse existing pattern`：沿用 typed desktop command + Rust DTO mapper + Vue lifecycle listener。拒绝延时重发（时序不可靠）、全局 store（窗口独立且生命周期短）、通用事件总线持久化（超出单缺陷复杂度）。

## Code Context And Impact Assumptions

定向调用链已闭合；code graph 不可用。若实现发现 active 无法在现有 queue mutex 中可靠拥有，回退 architecture-design。

## API And Data Contracts

- command：`captain_effect_current`，无参数。
- response：现有 camelCase `CaptainEffectPayloadDto | null`。
- adapter：非 null 复用 `adaptCaptainEffectPayload`；null 直接保留。
- event：`captain_effect_trigger` 合同不变。
- errors：invoke rejection 由窗口 lifecycle 捕获，不制造 fallback payload。

## Frontend / Server Responsibility Boundary

Rust desktop runtime 负责 active 状态与 DTO；Vue 负责监听、查询编排和展示。无远端服务依赖。

## Context And Dependency Sources

Tauri command mapping、现有 desktop wrapper、media adapter、Vue 3 lifecycle、Rust tokio mutex。

## Edge Cases

- 窗口挂载时无 active：保持透明。
- event 在 query 返回前到达：event payload 胜出。
- 查询返回后 event 到达：event 正常覆盖。
- 窗口卸载时 query 才返回：忽略。
- active 到期：清理，下一窗口不得读到旧 payload。

## Acceptance Criteria

- AC1：预览 active 可查询为 `isPreview=true` 的完整 payload。
- AC2：真实 captain active 保留 audience/action/level/config 且 `isPreview=false`。
- AC3：无 active 返回 `null`，结束后旧 payload 不可查询。
- AC4：EffectWindowRoot 先注册 event，再查询 current，迟到查询不覆盖较新 event。
- AC5：现有 12 模板、配置、窗口几何和 event contract 测试继续通过。

### 2026-09-23 Reopen Amendment

- AC6: the restricted `effect-*` capability grants the minimum Tauri core permission required by the existing `listenDesktop('captain_effect_trigger')` subscription.
- AC7: a listener-registration failure must not prevent the independent `captain_effect_current` query from supplying the preview's first visible frame.
- AC8: clicking Preview with a valid active preview must render the configured demonstration on the first opened window; a transparent empty window is a failure.
- The amendment is source-backed by the reopen evidence in `artifacts/reopen-2026-09-23.md`; it adds no copy, control, template, or payload semantic.
- Pattern decision remains Level 2 reuse: keep the typed Tauri event/query and Vue lifecycle pattern, but isolate the two async failure boundaries locally in `EffectWindowRoot.vue`.
- Architecture routing remains unchanged: the existing owner and dependency direction are valid, so no new architecture-design pass is required.

## Human Review And Handoff

交付用户安装包或桌面构建复测；说明根因、修改链和验证范围。

## Risks

主要风险是双通道竞态和 active 生命周期泄漏；通过事件优先规则、结束清理和测试约束。

## 2026-09-23 Semantic Consistency Amendment

### Source And Intent

- Source: U2 in `artifacts/semantic-reopen-2026-09-23.md` (`source-backed`).
- Literal request: resolve whether the feature represents captain purchase/renewal or captain room entry, and remove the naming/content contradiction.
- Practical goal: every visible label in this feature communicates the same purchase/renewal business event.
- Forbidden interpretations: rename the page as an entry effect, route `LiveEvent::Enter` into this effect, keep the entry sentence, or invent replacement promotional copy.

### Display Contract

- Feature meaning: captain purchase/renewal effect, established by U2 plus the existing `LiveEvent::Captain`/`purchase|renewal` contract.
- Preserve: page/menu title “上舰特效”, action label, captain level, username, accessible announcement, template visuals, and configuration behavior.
- Remove: the Canvas line sourced from `captainEffect.welcome` and the now-unused Chinese/English locale keys.
- Empty/invalid/click/interaction semantics: unchanged; the removed sentence has no interaction or state variants.
- Product-content allowlist: removal only. No new copy, field, formatter, control, default, validation, navigation, API semantic, analytics, or visual emphasis.

### Engineering Constraints

- Locality: edit the existing surface and locale owners; add no helper, hook, mapper, utility, component, store, or public API.
- Pattern fit: Level 0 `direct code`; this is deletion of one contradictory render call and its orphan locale entries, with no structural variation or coordination problem.
- Production quality: preserve the typed payload and drawing lifecycle; do not change Canvas timing, dimensions, decoration, or async behavior.
- Human review: every hunk must map to U2 removal, dependency cleanup, or regression adaptation; no unrelated copy or formatting churn.
- Architecture reuse: not triggered; no business rule or shared logic is added or duplicated.
- Styling: no authored styling change.
- TypeScript context: existing Vue/Vite package tsconfig and i18n JSON loading remain authoritative; no type changes.

### Acceptance Criteria

- AC9: the feature remains wired only to captain purchase/renewal semantics and retains the page/menu name “上舰特效”.
- AC10: `CaptainEffectSurface` no longer renders or references “欢迎加入直播间” / `captainEffect.welcome`.
- AC11: the orphan `captainEffect.welcome` entries are removed from both locale files while action/level/username display remains present.
- AC12: focused media-page and i18n contract tests, type-check, and relevant regression checks pass.

### Architecture Routing

`architecture-design_required=false`: ownership and dependency direction do not change; direct deletion is the complete fix.
