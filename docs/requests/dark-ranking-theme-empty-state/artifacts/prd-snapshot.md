# Normalized Bug Snapshot

## Verbatim Source Evidence Index

- U1：“我切换为深色主题之后显示的一块白、一块黑不协调”
- U2：“而且排行榜滚动之后出现两个暂无数据”
- U3：截图中的目标页面为排行榜，应用处于深色主题，页面上半部和下半部各出现一个空状态。

## Source Summary

修复排行榜的深色主题一致性和重复空状态。

## Key Business Goals

- 深色主题下页面视觉一致且文字可读。
- 空排行榜只传达一次无数据状态。

## Greenfield Scaffold Sensitivity

不适用：既有 Vue 页面局部缺陷修复。

## Explicit Behavior Constraints

- 深色主题下不能保留大面积固定白色页面表面。
- 无数据时只显示一个“暂无数据”。
- 不改变数据查询、排名切换、TOP 数量、导出或用户详情交互。

## Closed-World Product Scope Allowlist

- 排行榜页面深色样式修正。
- 排行榜重复空状态移除。

## Displays And Interactions

- 页面：排行榜。
- 显示：标题、筛选控制、榜单标签、表格、空状态、页脚。
- 交互：沿用现有范围、榜单、TOP 数量、导出和详情交互，不新增控件。

## Workflow And State Rules

- 主题状态仍由应用壳投影到根 `.dark` 类。
- 排行榜空数据仍由 `rows.length === 0` 触发表格空状态，不新增页面级状态。

## Relevant Modules

`src/renderer/views/rankings/index.vue` 及其页面契约测试。

## Open Questions

无。

