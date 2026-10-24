# Bugfix Source

## Source System

Codex 对话中的用户直接反馈。

## Project Key Or Source Scope

`biliLiveHelper` renderer / 排行榜页面。

## Defect ID

本地请求 `dark-ranking-theme-empty-state`。

## Defect Title

深色主题排行榜黑白割裂且滚动后出现两个空状态。

## Observed Behavior

1. 切换深色主题后，排行榜标题、控制区、标签区和页脚仍为浅色，与深色表格和应用壳不协调。
2. 排行榜无数据时，初始视口和向下滚动后的区域各显示一次“暂无数据”。

## Expected Behavior

1. 排行榜在深色主题下使用一致、可读的深色表面和文字/边框颜色。
2. 无数据时只有一个“暂无数据”，滚动不会暴露重复空状态。

## Reproduction Clues

切换为深色主题，进入“排行榜”，保持排行榜数据为空，然后向下滚动页面。

## Affected Module Or Page

`packages/bilibili-live-tool/src/renderer/views/rankings/index.vue`。

## Available Evidence

- 用户截图：`codex-clipboard-d38ca070-0be2-4dda-aae7-c685625620a8.png`。
- 代码事实：页面根节点固定 `bg-white` 且缺少 `dark:*` 变体。
- 代码事实：`el-table` 使用空数据时自带空状态，其后另有 `rows.length === 0` 的自定义“暂无数据”；表格 `height="100%"` 使第二个区块位于首屏下方。

## Verbatim Defect Evidence Index

- U1（observed）：“我切换为深色主题之后显示的一块白、一块黑不协调”
- U2（observed）：“而且排行榜滚动之后出现两个暂无数据”
- U3（scope）：用户截图明确显示“排行榜”页面和深色主题。
- U4（expected，直接反义）：深色主题不再黑白割裂，空状态不再重复。

## Closed-World Fix Scope Allowlist

- `source-backed`：修正排行榜页面深色主题表面、文字和边框颜色。
- `source-backed`：删除重复空状态，仅保留一个可见的“暂无数据”。
- `code-fact-backed`：继续使用现有 Element Plus 表格空状态和已导入的深色 CSS 变量，不新增状态所有者。
- `out-of-scope`：其他页面的深色主题审计、排行榜数据逻辑、后端协议、文案重写或布局重构。

## Open Questions And Missing Context

无。缺陷现象、页面、复现步骤和期望结果均足够明确。

