# Request: dark-theme-page-surfaces

## Source

2026-09-22 用户直接缺陷反馈及四张界面截图（证据 U1-U5）。

## Business Summary

切换深色主题后，弹幕面板、数据看板、敏感词和黑名单页面仍存在固定浅色表面，与应用壳及 Element Plus 深色表面形成明显黑白割裂。

## Goal

让四个指定页面的页面表面、标题、分隔线、辅助文字及数据看板指标区在深色主题下形成连续且可读的深色层级，同时保持浅色主题、业务数据和交互不变。

## Initial Done Signal

- 四个页面在根 `.dark` 存在时不再出现截图中的大块白色表面。
- 数据看板指标区及图表区与页面深色表面一致，文字和分隔线可读。
- 敏感词和黑名单的页头、表格、页脚不再呈现白黑白分层。
- 聚焦契约测试、前端测试、类型检查、lint、构建和实际深色页面检查通过。

## Trigger, Context, Handoff

- Trigger：用户切换到深色主题并进入四个指定路由。
- Context：用户截图、四个页面 SFC、既有 `.dark` Tailwind variant、Element Plus 暗色变量和相邻主题修复工件。
- Steerability：以本请求 `state.json`、spec、plan、verification 和 review 为准。
- Handoff：完成后交付改动文件、验证结果和可访问的本地运行地址。

## Affected Area

`packages/bilibili-live-tool/src/renderer/views/{danmaku-list,dashboard,sensitive-words,blacklist}` 及聚焦页面契约测试。

## Participating Modules

Vue 页面模板、Tailwind utility、Element Plus 表格/控件暗色变量；不涉及 store、IPC、Rust 后端或业务契约。

