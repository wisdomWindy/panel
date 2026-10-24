# Spec: 深色排行榜主题与唯一空状态

## Delivery Unit Identifier

`dark-ranking-theme-empty-state`

## Background And Goals

排行榜根容器固定浅色且存在两个空状态 owner。目标是让页面响应根 `.dark` 类，并将空状态所有权收敛到 `el-table`。

## In Scope

- 为排行榜根表面、标题、分隔线和页脚补齐可读的深色 utility。
- 删除表格之后的自定义空状态，给表格保留明确的“暂无数据”空文案。
- 新增排行榜页面契约测试。

## Out Of Scope

其他页面主题审计、布局重构、数据查询/store/IPC、榜单字段、文案国际化和表格滚动高度调整。

## Trigger And Start Conditions

进入 `/rankings`；深色问题在根 `.dark` 存在时触发，重复空状态在 `rows` 为空时触发。

## Requirement Split Summary

不适用：S1 非拆分 bugfix，单页单交付单元。

## Source Grounding And Traceability

| Spec item | Label | Product-facing | Evidence | Plan obligation |
| --- | --- | --- | --- | --- |
| 深色排行榜不再显示固定白色表面 | source-backed | 是 | U1/U3 | 补齐 section/header/title/footer 深色 utility |
| 空数据仅显示一个“暂无数据” | source-backed | 是 | U2/U4 | `el-table` 为唯一 empty owner，移除后置区块 |
| 不改数据与交互链 | code-fact-backed | 否 | 影响链只落在模板表现层 | 保持 script/store/IPC 不变 |

Closed-world allowlist：只允许上述主题样式修正和空状态去重；不新增字段、控件、状态、导航、验证、API 语义或分析事件。

## Workflow Efficiency Profile

- `S1 local`，compact artifact。
- 下游只读取排行榜页、相邻测试、主题入口和 package 检查命令。
- 若发现需修改共享主题投影、Element Plus 全局变量或多个页面，升级为 S2 并回到 spec。

## Self-Healing Completion Bar

- 需求：深色表面一致且只有一个空状态 owner。
- 逻辑：空/加载/有数据的表格所有权不冲突，查询与交互不变。
- 静态：聚焦/全量 Vitest、TypeScript、ESLint、构建通过。
- 运行时：深色视口截图无白色大块，滚动后不出现第二个空状态。
- 组织：无无关改动、死代码、未使用 import 或重复渲染分支。
- 完成要求：self-healing compliance/assessment 均 pass，blocker 为 none。

## User Intent Contract

- literal request：修复深色主题黑白割裂和滚动后的两个“暂无数据”。
- practical goal：排行榜在深色模式下视觉连续，空页面只传达一次无数据。
- success criteria：深色根表面/文字/边框可读；DOM 中只存在一个空状态展示。
- forbidden interpretations：不得通过隐藏滚动条、缩短容器、遮挡第二个空状态或强制浅色模式规避问题。
- acceptable approach：补齐现有 `.dark` utility，并删除重复空状态 owner。

## User Flow

用户切换深色主题 -> 进入排行榜 -> 页面使用深色表面 -> 查询期间沿用加载态 -> 无数据时表格显示一次“暂无数据” -> 滚动不再出现第二份文案。

## Page And Module Design

保持单个 `views/rankings/index.vue`。页面 section 仍编排 header/tabs/table/footer，表格继续拥有列表空状态。

## UI / UX Design Contract

不新增布局或交互结构。深色模式使用现有应用壳 `zinc` 表面层级：内容页 `zinc-900`、边框 `zinc-800`、主文字 `zinc-100`、次文字 `zinc-400`。浅色样式保持原值。

## Frontend Styling Constraints

仅使用模板内 Tailwind utility；不新增 CSS、style block、class 常量或 helper。class 值保持可直接审查，深色变体与对应浅色 utility 相邻。

## Field Display Semantics And Visual States

- “暂无数据”：业务含义为当前筛选榜单结果为空；不可点击，只在非加载且表格数据为空时由 Element Plus 表格空态显示一次。
- 排行榜标题：页面主标题，深色模式为高对比主文字。
- 页脚总人数/更新时间：辅助元数据，深色模式为次级文字；格式和精度不变。

## Function-Complete Behavior Breakdown

- 页面表面：浅色 `white` 不变；深色切换到 `zinc-900`。
- header/footer：保留布局尺寸，深色边框改为 `zinc-800`。
- 标题/页脚：保留文案与字号，深色分别使用 `zinc-100`/`zinc-400`。
- 表格：保留数据、列、loading、height、row-key；显式设置唯一 empty text。
- 删除：表格后的 `rows.length === 0` 自定义空状态区块及其额外高度。

## Expert Frontend Engineering Constraints

状态 owner 不变：Pinia 提供 rankings/pending，computed `rows` 派生；不复制状态。异步查询、错误提示、键盘语义、数据量和响应式边界均不变。运行时证据需覆盖空、深色和滚动后的最终视口。

## Frontend Architecture Design Routing Inputs

`architecture-design_required: false`。不改模块边界、状态 topology、依赖方向、公共 API 或共享抽象。

## Production Code Quality Constraints

不新增类型、函数、状态或 class。空数据和 loading 继续由表格既有 API 处理；不引入 watcher、缓存或微优化。删除后不得遗留无用代码。

## Human Review Readiness Constraints

允许修改：排行榜 SFC、同目录契约测试、本请求工件。禁止共享主题/其他页面的顺手清理。每个 hunk 必须分别映射到深色主题或空状态去重；提供 RED/GREEN、类型、lint、构建和截图证据。

## Functional-Programming Constraints

不适用：不修改转换、派生、校验或副作用逻辑；现有 `rows` computed 保持不变。

## Architecture Reuse And Shared Ownership Constraints

无重复业务逻辑抽取信号。深色 utility 复用项目既有 `.dark` 机制；空状态复用已有 `el-table` owner。禁止新增共享 helper。

## Locality And File-Extraction Constraints

所有生产修改留在 owning page；不新增 helper、utility、composable 或组件文件。

## Vue Component Extraction Constraints

`keep inline`：没有新组件候选，单页模板变更不构成独立业务/生命周期边界。

## Design Constraints

保留 Element Plus 表格及其 dark CSS variables；不绕过应用壳主题投影；不改变现有页面最大宽度和高度约束。

## Project Bootstrap And Scaffold Decision

不适用：既有页面修复。

## Change Axes And Pattern Decision

- scoped axis：模板静态样式与空状态 owner。
- pattern signal：无类型选择、适配、创建、工作流编排、副作用协调或跨文件状态分支。
- decision：Level 0 `direct code`，沿用 Tailwind dark variant 和 Element Plus empty API。
- rejected：新建主题 wrapper 或空状态组件会增加单调用方间接层，且不能比直接修正 owner 更清晰。

## Code Context And Impact Assumptions

根因和链路见 `artifacts/code-context.md`。受影响生产文件仅 `views/rankings/index.vue`；测试为 adaptation surface。

## API And Data Contracts

不变。`RankingSnapshot`、查询 IPC、导出与用户详情契约均不修改。

## Frontend / Server Responsibility Boundary

纯前端表现缺陷；服务端无任务或依赖。

## Context And Dependency Sources

Vue 3 SFC、Tailwind v4 custom dark variant、Element Plus dark variables；TypeScript 由 `tsconfig.eslint.json` -> `tsconfig.json` 严格配置治理。

## Edge Cases

- loading + empty：只显示表格 loading，结束后显示一次空文案。
- 切换榜单后结果为空：同一表格 owner 更新，不创建第二节点。
- 有数据：不显示 empty 文案，行和滚动保持现状。
- 浅色模式：既有白色表面和灰色文字/边框不变。

## Acceptance Criteria

- A1：源码契约包含根 section 的 `dark:bg-zinc-900`、标题 `dark:text-zinc-100`、header/footer `dark:border-zinc-800` 和 footer `dark:text-zinc-400`。
- A2：排行榜模板中“暂无数据”只有一个 owner，位于 `el-table empty-text`，不存在后置 `rows.length === 0` 空状态区块。
- A3：聚焦测试经历 RED -> GREEN；全量前端测试、type-check、lint、build 通过或仅有已知非阻塞 warning。
- A4：深色运行截图中无固定白色排行榜表面，滚动区域只显示一个空状态。

## Human Review And Handoff

交付小范围 diff、验证命令、截图和新安装包/运行地址；用户可按原复现步骤复核。

## Risks

Element Plus 内部暗色表格颜色由既有 CSS variables 控制；若运行时未加载，属于全局主题链回归并需升级范围。当前 `main.ts` 已明确导入该变量文件。

