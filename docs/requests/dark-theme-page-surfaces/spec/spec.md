# Spec: 四个页面深色表面一致性

## Delivery Unit Identifier

`dark-theme-page-surfaces`

## Background And Goals

应用根主题和 Element Plus 暗色变量已生效，但四个页面仍用固定浅色 utility，造成截图中的黑白割裂。目标是在不改变业务行为和浅色外观的前提下，使指定页面完整响应根 `.dark`。

## In Scope

- 弹幕面板、数据看板、敏感词、黑名单的根表面、标题、边框和辅助文字暗色状态。
- 弹幕事件行及页脚关键数值的暗色可读性。
- 数据看板指标容器、五个指标块、标签/数值、图表区域及空状态的暗色可读性。
- 聚焦源码契约测试与运行时视觉验证。

## Out Of Scope

布局、文案、字段、业务状态、交互、store/IPC/Rust、主题投影逻辑、品牌色、其他页面、图表数据语义或折线颜色重设计。

## Trigger And Requirement Split

Trigger：根节点存在 `.dark` 且用户进入四个指定页面。非 PRD、非拆分 bugfix，`module_flow=null`。

## Source Grounding And Traceability

| Spec item | Label | Product-facing | Evidence | Plan obligation |
| --- | --- | --- | --- | --- |
| 四页消除固定白色大块 | source-backed | 是 | U1-U5 | root/header/body/footer 暗色 utility |
| 数据看板指标/图表区视觉连续 | source-backed | 是 | U1/U3 | metrics 与 chart 周边表面、文字、边框暗色 utility |
| 浅色主题和业务行为不变 | source-derived + code-fact-backed | 是 | 缺陷限定“切换深色主题之后”；现有交互链 | 只追加 `dark:` utility，不改 script/props |
| 使用根 `.dark` 和既有 `zinc` 层级 | code-fact-backed | 否 | Tailwind custom variant、已完成排行榜合同 | 复用现有主题机制，不新增 owner |

Closed-world allowlist：只允许上述视觉状态和对应契约测试；不新增产品内容或行为。

## Workflow Efficiency Profile

`S2 scoped`，full artifact、scoped context。四页同类修复且涉及 ECharts/Element Plus 视觉边界，需要跨页验证；若发现必须改共享主题投影、图表数据逻辑或组件 API，必须回到 spec。

## Self-Healing Completion Bar

- 需求：四页深色完整、浅色保持、无额外行为。
- 逻辑：数据/交互/异步 owner 不变，空/加载/错误状态仍由原组件控制。
- 静态：聚焦测试、全量 Vitest、type-check、lint、Web build。
- 运行时：深色四页截图/像素检查无大块固定浅色面；数据看板 canvas/tooltip 可读。
- 组织：仅计划文件有增量、无格式化噪音、无 unused/dead/debug/stale artifact。
- 完成：所有适用 verdict 及 self-healing compliance/assessment 为 pass，blockers none。

## User Intent Contract

- Literal：修复四页深色模式“一块白、一块黑”。
- Practical：整个内容工作区像一个一致的深色应用，而不是只把壳和部分组件变黑。
- Success：root、页头、主体、指标/表格、页脚视觉连续且文字可读。
- Forbidden：强制全局深色、删除浅色样式、仅遮挡白块、把全部内容涂成无层级纯黑、修改业务语义。
- Acceptable：在 owning SFC 内为现有浅色 utility 补齐项目既有 `dark:` 对应值。

## User Flow And UI/UX Contract

沿用 `design/page-design.md`。切换主题 -> 进入页面 -> 扫描标题/工具 -> 查看或维护内容 -> 读取摘要；布局、信息架构、操作优先级、反馈、响应式和键盘路径均不变。

## Frontend Styling Constraints

只使用模板内 Tailwind utilities。禁止 scoped CSS、style object、语义 class、class 常量/computed/helper。深色状态采用 `dark:bg-zinc-900`、`dark:border-zinc-800`、`dark:text-zinc-100/400` 等既有层级；不覆盖状态色和图表系列色。

## Field Display And Visual-State Contracts

- 页面标题、关键指标和弹幕关键数值：业务主信息；浅色 `gray-900`，深色 `zinc-100`，不可点击者无 hover 状态。
- labels、时间、页脚摘要、空状态和导入结果：辅助信息；浅色原值，深色 `zinc-400`。
- 分隔线/指标 gap：结构边界；浅色 `gray-200/100`，深色 `zinc-800`。
- 过滤统计 router-link：现有粉色默认强调与 hover 行为不变。
- 折线：现有蓝/绿/橙系列语义不变；只允许修复周边背景/轴文案对比，不改变数据和精度。

## Function-Complete Behavior Breakdown

1. 四个 root section 保留 `bg-white`，增加 `dark:bg-zinc-900`。
2. 四页 header/footer 边框增加 `dark:border-zinc-800`；标题增加 `dark:text-zinc-100`；footer/辅助文字增加 `dark:text-zinc-400`。
3. 弹幕事件 row 边框增加深色状态，事件正文/时间/空状态在深色下可读，关键 footer 数值增加主文字暗色状态。
4. Dashboard metrics 外层在暗色下使用 `zinc-800` 分隔背景，五个块使用 `zinc-900`，label/value 分别使用辅助/主文字暗色状态；图表区与 empty state 不出现固定浅色容器。
5. 敏感词/黑名单 import result 与 footer 暗色可读；Element Plus 表格继续使用全局暗色变量。
6. 不修改四个 SFC 的 script 逻辑、组件 props/events、store/IPC、文案或路由。

## Expert Frontend Engineering Constraints

主题状态仍由 shell projection 单一拥有，页面样式为 derived state；不新增 watcher、状态或副作用。异步查询、竞态、loading/error/empty owner、表格滚动、键盘和焦点合同不变。性能影响只增加静态 selector，无 reactive fan-out 或 bundle dependency。

## Frontend Architecture Design Routing Inputs

`architecture-design_required: false`。不改变模块边界、ownership、state/data topology、依赖方向、公共 API、共享抽象或跨文件协作。

## Production Code Quality Constraints

不新增类型、函数、状态、空值处理、缓存或 class。现有类型和边界 UI state 原样保留。模板 class 直接可审查，变更应保持最小、确定且无运行时分支。

## Human Review Readiness Constraints

允许修改四个 owning SFC、dashboard 专用 `TrendChart`、一个聚焦测试文件、使真实 SFC SSR 测试可运行的 Vitest-only 依赖内联配置和本请求工件。当前工作树中这些 SFC 已有用户的 i18n 改动，必须只追加目标 class 并保留其余 diff。每个 hunk 映射到一页主题合同；提供 RED/GREEN、测试、类型、lint、build 和截图证据。

## Functional Programming And Architecture Reuse

不适用：不改转换、规则、状态派生、adapter、mapper 或副作用。相似 class 是页面表现，不抽取共享 helper/wrapper；各页面保持 owning template 内联。

## Locality And Vue Component Extraction

Level 0 local markup change。四个页面业务结构不同，共享 wrapper 会成为纯 pass-through；禁止新增 helper、utility、composable 或 Vue SFC。现有 `TrendChart` 等组件边界不变。

## Design And Scaffold Constraints

沿用 Element Plus、Tailwind 和现有 `zinc` 深色系统；非 greenfield，无脚手架决策。

## Change Axes And Pattern Decision

变化轴仅为静态主题视觉状态，没有选择、创建、编排、副作用协调、状态分支或跨文件行为协作信号。Decision：Level 0 `direct code`。拒绝 theme wrapper/service，因为会新增 owner/间接层且无法改善四个明确 class 漏洞。

## Code Context, API And Responsibilities

根因和链路见 `artifacts/code-context.md`。无 API/data contract 或服务端任务。TypeScript 由 `tsconfig.eslint.json` -> `tsconfig.json` 治理，但生产修改仅在 Vue template class；执行前仍需读取这两个配置和 Vue ambient declarations 的直接来源。

## Edge Cases

- 深色切换前后：同一 DOM 通过根 `.dark` 切换，不重新创建业务状态。
- empty/loading/error：保持原 owner 和文案，容器文字可读。
- 表格主体：继续由 Element Plus dark variables 控制，page wrapper 不覆盖固定浅色。
- 窄视口/滚动：不改尺寸或 overflow。
- 有/无统计数据：dashboard 两个分支均不显示固定浅色大块。

## Acceptance Criteria

- A1：四个页面 root 同时包含 `bg-white dark:bg-zinc-900`。
- A2：四页标题和 header/footer 分隔/辅助文字包含相应 `dark:text-zinc-100/400`、`dark:border-zinc-800`。
- A3：dashboard 五个指标块均有 `dark:bg-zinc-900`、五个 label 均有 `dark:text-zinc-400`、五个 value 均有 `dark:text-zinc-100`，metrics gap 有 `dark:bg-zinc-800`。
- A4：danmaku row、空状态、时间和 footer 关键数值在深色下可读；sensitive/blacklist import result 与 footer 可读。
- A5：聚焦测试经历 RED -> GREEN；全量前端测试、type-check、lint、build 通过或只记录明确的既有非阻塞 warning。
- A6：实际深色四页截图及像素/DOM 检查无截图中的大块白色表面；浅色检查保持白色表面。

## Human Review And Handoff

交付小范围 diff、命令结果、视觉证据和本地运行 URL；用户可按原步骤切换主题复核。

## Risks

ECharts canvas/tooltip 可能使用初始化时颜色且不随根主题切换；只有实际验证失败才允许按既有 `TrendChart` owner 做最小修复并重新进入 spec/plan。
