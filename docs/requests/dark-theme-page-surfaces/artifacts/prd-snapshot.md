# Normalized Defect Snapshot

## Verbatim Source Evidence Index

- U1：“切换深色主题之后弹幕面板页面、数据看板页面、敏感词、黑名单页面还是一块白、一块黑的”。
- U2-U5：当前会话附件中的四个对应页面截图，分别显示固定浅色页面表面或白黑白分层。

## Source Summary And Goal

深色主题已作用于应用壳和部分 Element Plus 组件，但四个指定页面自身仍有固定浅色 utility。目标是使这些页面在深色主题下视觉连续，同时不改变浅色主题及业务行为。

## Greenfield Scaffold Sensitivity

不适用：not a project/app/package/frontend-surface creation request。

## Explicit Behavior Constraints

- 深色模式：四个页面不再显示截图中的大块白色表面。
- 浅色模式：保留当前白色表面及既有层级。
- 不改变表格字段、筛选、按钮、空状态、图表数据、查询和导出行为。

## Closed-World Product Scope Allowlist

只允许修改四个页面的主题响应样式和对应主题契约测试；没有新字段、文案、控件、交互、状态、校验、权限、导航、API 语义或分析事件。

## Forms, Tables, Displays, And Interactions

- 弹幕面板：现有页头、事件流、空状态、页脚。
- 数据看板：现有页头、五个指标块、三个趋势图和空状态。
- 敏感词、黑名单：现有页头、表格区、导入结果/错误状态、页脚和弹窗。
- 所有字段与交互原样保留，只修复深色视觉状态。

## Workflow And State Rules

根节点 `.dark` 是现有最终主题投影；页面通过 Tailwind `dark:` utility 派生视觉状态，不新增可写主题状态。

## Relevant Pages

`danmaku-list/index.vue`、`dashboard/index.vue`、`sensitive-words/index.vue`、`blacklist/index.vue`。

## Open Questions

无。

