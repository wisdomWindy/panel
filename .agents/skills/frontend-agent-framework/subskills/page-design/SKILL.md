---
name: frontend-agent-framework-page-design
description: Stage subskill for page design. Define page UX, layout, visual hierarchy, styling direction, feedback states, responsive behavior, accessibility inputs, and interaction structure for page-oriented requests before the formal spec stage.
---

# Page Design Subskill

## 触发场景

- 当主 `frontend-agent-framework` 在 `intake` 之后判断该需求是页面型需求时使用。
- 适用于：
  - 新页面
  - 页面改版
  - 明显影响布局、样式、视觉层级、交互组织的 UI-heavy 变更
  - 明显影响用户路径、信息架构、操作反馈、响应式体验或可访问性的页面变更

## 必要输入

- `docs/requests/<request-id>/request.md`
- `docs/requests/<request-id>/artifacts/prd-snapshot.md`
- `docs/requests/<request-id>/requirements/requirement-map.md`
- `docs/requests/<request-id>/state.json`
- `docs/requests/<request-id>/requirements/modules/<current-module-id>.md`（拆分 PRD 模块适用）

当前交付单元路径规则：

- 拆分 PRD 模块：`docs/requests/<request-id>/module-runs/<current-module-id>/`
- direct-change、bugfix 或非拆分请求：`docs/requests/<request-id>/`

## 执行步骤

1. 先读：
   - `docs/requests/<request-id>/state.json`
   - `docs/requests/<request-id>/request.md`
   - `docs/requests/<request-id>/artifacts/prd-snapshot.md`
   - `docs/requests/<request-id>/requirements/requirement-map.md`
   - `docs/requests/<request-id>/requirements/modules/<current-module-id>.md`（如存在）
   - `../../references/framework-overview.md`
   - `../../references/state-machine.md`
   - `../../references/templates/page-design.md`
   - `../../references/policies/policy-index.md`
   - 按 `policy-index.md` 的 `page-design` 阶段映射读取本次页面设计适用的 policy 文件；默认至少读取 `ui-ux-design.md`、`frontend-components.md`、`doc-writing.md`、`source-grounding.md`；涉及 Vue SFC / 组件边界决策时读取 `vue-component-extraction.md`
2. 只在页面型需求中使用本阶段。
3. 仅为当前交付单元产出页面设计；拆分 PRD 工作不得跨模块混写。
4. 产出当前交付单元的 `design/page-design.md`。
5. 先建立 product-content allowlist mapping：每个拟加入设计的字段、标签、文案、控件、操作、交互、状态、默认值、校验、权限展示、导航、状态含义和反馈行为，都必须映射到上游 closed-world product scope allowlist 及需求原文 / 人工确认。布局、间距、层级、响应式和组件选择可以在项目约束内细化，但不能创造产品语义；缺少授权的内容必须记为 `missing-source` 并回到前置确认 gate，不能画进已批准设计。
   - 明确定义：
   - 页面布局结构
   - 用户旅程与任务目标
   - 信息架构和阅读 / 操作顺序
   - 区块层级
   - 视觉层级与视觉效果方向
   - 样式方向
   - 交互骨架
   - 状态反馈模型
   - 响应式考虑
   - 可访问性与可用性考虑
   - UX 风险和缓解策略
   - Tailwind CSS-style utility class 约束与 class 值长度风险
   - Vue 组件抽取 / 不抽取决策：必须写明候选区块、是否抽取、触发的抽取条件或不抽取条件、props / emits / slots 边界、状态 owner、样式边界和验证面
6. 设计结果必须足够稳定，使当前交付单元的 `spec` 和后续 `architecture-design`（如存在）能把它当作上游输入，而不是再重新发明布局决策。
7. 拆分 PRD 工作必须以 `requirements/requirement-map.md` 和当前模块工件为页面范围来源，不得跨模块混写；direct-change、bugfix 或非拆分请求以 `request.md`、`artifacts/prd-snapshot.md` 和既有 code context 为范围来源。
8. 所有设计决策必须写入仓库工件，不保留在聊天上下文里。
9. 保留 `state.json.loop`，不重置、不改写。
10. 本阶段不写代码。
11. 对于引入或实质改变当前交付单元布局、视觉层级、样式方向、用户旅程、信息架构、反馈状态、响应式体验、可访问性或交互组织的页面需求，不允许跳过本阶段。

## 输出格式

- 必须产出：
  - 当前交付单元的 `design/page-design.md`
- 最终交付物应至少包含：
  - 页面布局结构
  - 用户旅程和任务目标
  - 信息架构和内容分组
  - 区块层级
  - 视觉层级与视觉效果方向
  - 样式方向
  - Tailwind CSS-style styling constraints
  - 交互骨架
  - 状态反馈模型
  - 响应式考虑
  - 可访问性与可用性考虑
  - UX 风险与缓解策略
  - 设计风险与未决 UI 问题

## 验收标准

- 当前交付单元 `design/page-design.md` 已存在。
- 文档已定义用户旅程、信息架构、布局、层级、视觉效果方向、样式方向、交互结构、状态反馈、响应式考虑、可访问性与可用性风险。
- 文档已说明主操作、次操作、危险操作、禁用条件、错误恢复和完成反馈；不能只有静态布局描述。
- 如涉及样式变更，文档已声明只能使用 Tailwind CSS-style utility classes，且不能用常量、computed 或 helper 隐藏过长 class 值。
- 如涉及 Vue 组件拆分，文档已明确哪些组件应抽取、哪些保持内联，以及每个决定对应的具体条件；没有使用“页面太长”“以后可能复用”这类泛化理由。
- 所有未决 UI 问题已记录。
- 所有设计中的产品内容均有 product-content allowlist 引用；没有用空态、重试、toast、确认框、帮助文案、默认操作或相邻页面惯例补齐需求未写的内容。
- `state.json.stage` 已切换到当前交付单元的下一下游阶段，且保留原有 `loop`。

## 安全边界

- 不能写代码。
- 不能把本阶段强行变成像素级设计工具流程要求。
- 没有 durable design artifact 时，不能进入实现规划。
- 不能把本阶段用于不影响页面结构或样式的非 UI 任务。
- 不能用个人审美、相邻页面习惯或通用 UI 偏好扩展产品范围；体验优化必须受 source-grounding 约束。
- 不能为了满足“完整 UX”而自行增加需求未授权的空态文案、错误操作、重试、成功提示、确认、默认值、校验、权限说明或导航入口；应记录缺口并回退确认。
- 不能只描述组件摆放而不说明用户任务、信息优先级、反馈状态和交互风险。
