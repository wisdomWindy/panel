# Request: 深色排行榜主题与空状态修复

## Request Identifier

`dark-ranking-theme-empty-state`

## Source

2026-09-22 用户直接缺陷反馈及截图，详见 `artifacts/bugfix-source.md`。

## Business Summary

排行榜在深色主题下仍显示大面积浅色容器，视觉上黑白割裂；无排行榜数据时，向下滚动会看到第二个“暂无数据”。

## Goal

使排行榜页面完整响应深色主题，并在任意滚动位置只呈现一个空状态。

## Initial Done Signal

- 深色主题下排行榜页面根容器、标题、分隔线和页脚使用协调的深色样式。
- 排行榜无数据时仅由表格呈现一个“暂无数据”，不存在页面下方的重复空状态。
- 浅色主题、排行榜数据行、筛选、导出和页脚统计行为不变。

## Trigger Condition

用户切换到深色主题并进入排行榜；排行榜当前无数据。

## Context Sources

- 用户截图与文字反馈。
- `src/renderer/views/rankings/index.vue`。
- Element Plus 深色变量入口与应用壳主题投影。

## Human Handoff

交付代码、自动化验证和深色页面截图供用户复查。

## Affected Area

排行榜页面的主题样式与空数据渲染。

## Participating Modules

仅 renderer 排行榜页面；不改后端、数据契约或主题状态所有权。

