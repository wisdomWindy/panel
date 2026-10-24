# Code Context

## Context Requirement

缺陷涉及四个既有 Vue 页面，但根因是局部模板 utility，依赖链简单。bugfix-intake 需要确认主题 owner、页面 owner 及测试适配面；code graph 仅为 preferred，不是 required。

## Graph Availability Check

- Status：missing。
- Detection：仓库脚本和配置未暴露 code graph 命令；相邻已完成请求 `dark-ranking-theme-empty-state` 也记录为 missing。
- Iteration：bugfix-intake，2026-09-22。

## Installation Or Bootstrap Record

- Attempted：no。
- Result：not_needed。
- Reason：根因由四个 owning SFC 中的固定 `bg-white` 直接证明，且变更不涉及调用方、状态或模块边界；按 workflow-efficiency/code-graph policy 使用 scoped text fallback。

## Fallback Record

- Used：yes。
- Method：`rg` 搜索固定背景/颜色类，阅读四个页面、Tailwind dark variant、相邻排行榜主题契约和当前 git diff。
- Confidence：high；所有可见缺陷表面均由目标 SFC 静态 class 拥有。
- Blind spots：运行时图表 canvas 颜色需在 verify 阶段实际检查。

## Relevant Entrypoints And Functional Chain

用户设置主题 -> `applyUiProjection` 在根节点切换 `.dark` -> Tailwind `dark:` selector 与 Element Plus 暗色变量生效 -> 路由渲染目标页面 -> 页面静态 utility 决定根表面/文字/边框。当前四个页面的固定 `bg-white` 覆盖了暗色外壳；敏感词/黑名单表格由 Element Plus 暗色变量渲染，因此出现混层。

## Key Modules

- `styles/tailwind.css`：`@custom-variant dark (&:where(.dark, .dark *))`，主题 selector 已正确。
- 四个页面 `index.vue`：本次样式 owner。
- `components/live-data/TrendChart.vue`：数据看板图表渲染 owner，需验证是否依赖页面文字颜色。
- 页面契约测试：适配目标，不是生产 owner。

## Dependency And Side-Effect Boundaries

仅模板类名变化；不修改 Pinia、异步查询、ECharts 数据、Element Plus props、路由或 IPC。现有用户的 i18n 等未提交修改必须保留。

## Impact Scope

预计修改四个页面和一个聚焦契约测试文件。回归面是浅/深主题、空/有数据、表格和图表的可读性。

## Open Follow-Up Checks

verify 阶段检查数据看板 canvas/tooltip 的深色实际效果，以及四页完整视口是否仍有固定浅色块。

