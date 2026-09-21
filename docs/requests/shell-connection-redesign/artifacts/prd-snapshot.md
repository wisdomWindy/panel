# Direct-change Source Snapshot

## Verbatim Source Evidence Index

- U1：`运行之后显示如图所示的画面，使用 figma mcp重新设计页面并使用 frontend-agent-framework 开发确保页面正确展示，功能正常使用`
- U2：用户截图显示桌面应用首屏仅有“直播间”和“未连接”等少量内容，工具栏与侧边导航缺少可见信息，大面积区域空白。

## Source Summary

用户要求修复运行后的异常/贫弱展示，重设计截图对应页面，并保证正确展示与既有功能可用。[source-backed: U1, U2]

## Key Business Goals

- 首屏不再呈现近似空白的应用壳。[source-backed: U1, U2]
- 页面具有明确的导航、连接操作、房间状态与摘要信息层级。[source-derived from U1; limited to existing controls and fields]
- 已有交互保持正常使用。[source-backed: U1]

## Greenfield Scaffold Sensitivity

不适用：现有 Vue 3 + Vite + Tauri 应用的页面改造，不创建新的项目、应用、包或前端入口。

## Explicit Behavior Constraints

- 保留现有房间号输入/选择、连接/断开、通知、设置、导航和状态栏行为。[code-fact-backed impact boundary; authorized by U1 “功能正常使用”]
- 保留直播间页面已有字段：房间号、房间名称、在线人数及连接状态。[code-fact-backed current product content; behavior preservation under U1]
- 修复 `dev:web` 未指定 free/member 通道导致挂载前异常的问题。[code-fact-backed root cause; required by U1]
- 不把“重新设计”解释为新增业务能力、字段或接口。[source-derived anti-expansion from U1]

## Closed-world Product Scope Allowlist

- P01：重排和重绘既有品牌标题、直播间标题、导航、工具栏、状态标签、房间摘要与状态栏。[source-backed: U1; current items code-fact-backed]
- P02：保留并强化既有房间号输入与连接/断开主操作的视觉层级和状态可见性。[source-backed: U1; existing interaction code-fact-backed]
- P03：保留既有通知与系统设置入口。[source-backed behavior preservation: U1]
- P04：保留既有房间号、房间名称、在线人数的空值和连接态展示。[source-backed behavior preservation: U1]
- P05：支持当前 shell 已有浅色/深色主题，并保证文本对比度。[source-derived usability consequence of “正确展示”]
- P06：让 `dev:web` 默认使用 free build channel。[code-fact-backed technical fix; product-content delta: none]

任何新业务文案、快捷操作、统计字段、引导流程、模拟数据、重试按钮或新导航均为 out-of-scope。

## Forms, Tables, Displays, Interactions

- 表单：工具栏房间号输入/选择，原有 trim、纯数字和 1-20 位校验保持不变。
- 显示：产品名、当前路由标题、连接状态、房间号、房间名称、在线人数、底部七项状态。
- 交互：连接/断开；导航；通知弹层；系统设置。
- 表格：不适用。

## Workflow And State Rules

- disconnected/error：允许输入有效房间号并连接。
- connecting：主操作 pending/disabled，避免重复连接。
- connected/reconnecting：主操作切换为断开。
- desktop command 失败：沿用现有错误映射和错误展示，不伪造成功。

## Relevant Modules

`App.vue`、三个 application-shell 组件、connection view、application shell/live room stores、现有合同测试、package scripts。

## Open Questions / Missing Source

- Figma MCP：当前工具清单无 Figma MCP，无法生成 Figma 设计链接或节点。使用 repository-local page-design artifact 替代；不得声称已调用 Figma。
- 无其他阻塞性产品问题。
