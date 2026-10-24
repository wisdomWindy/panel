# 规格：关于与许可隐藏更新通道

## Delivery Unit, Goal And Scope

- Delivery unit：`hide-update-channel-about`。
- 目标：关于与许可页面不再展示更新通道标签和值。[source-backed: U1]
- 范围：`settings/index.vue`、其合同测试、`zh-CN.json`、`en-US.json`。
- 非范围：release store/类型/desktop contract、更新检查、版本信息、许可表格、其他设置页内容。

## Source Grounding And Product Allowlist

| Item | Grounding | Authority |
| --- | --- | --- |
| 隐藏更新通道标签和值 | source-backed | U1 |
| 删除两份孤立 locale 键 | code-fact-backed cleanup | 唯一 UI 引用随展示移除 |
| 保留 channel 数据合同与更新检查 | code-fact-backed compatibility | store/desktop 更新链路仍拥有 |

Product-content delta：只减少 U1 指定的一项展示；不新增或修改其他字段、文案、控件、交互、状态、默认值、校验、权限、导航、API/payload 或埋点。

## User Intent And Display Contract

- Literal：关于与许可中不显示“更新通道”和通道值。
- Practical：用户查看该分区时只能看到应用版本、检查更新入口和第三方许可内容，不再看到 channel 元数据。
- Forbidden：仅用 CSS 隐藏、删除 release channel 数据合同、禁用更新检查、修改版本或许可展示、修改其他页面。
- 展示语义：该字段整体移除，不保留占位符、空白行、ARIA 文本或不可见 DOM。
- 交互/状态/响应式/可访问性：除阅读顺序直接从应用版本进入许可分隔线外，全部保持不变。

## Implementation Contract

1. 在设置页现有合同测试中先断言页面不包含 `settings.aboutInfo.channel` 与 `release.about?.channel`，观察 RED。
2. 从关于 `<dl>` 删除对应 `<dt>` 和 `<dd>`。
3. 从中英文 `settings.aboutInfo` 删除失去使用者的 `channel` 键。
4. 保留 `release.checkUpdate()`、pending 状态、反馈逻辑、`release.about.applicationVersion` 和 licenses。

## Architecture, Pattern And Quality

- `architecture_design_required: false`：无边界、状态、类型、数据流或依赖方向变化。
- Pattern fit：Level 0 `direct code`，pattern `none`。这是稳定的局部展示移除，没有选择、适配、创建、编排、状态分支或跨文件协作信号；原位删除最安全。
- 不新增 helper、constant、component、CSS、style 或 class。
- 类型、null、异常、异步、性能和边界状态不变。
- 变更前后链路均须复查；locale 键属于删除闭包，channel 数据模型不属于。
- Human review：只允许一个测试 hunk、一个模板 hunk和两个 locale hunk；不得格式化整文件或混入其他清理。

## Acceptance Criteria

1. 关于与许可模板不包含 `settings.aboutInfo.channel`。
2. 关于与许可模板不包含 `release.about?.channel`，且不留下对应占位行。
3. 中英文 locale 不再包含 `settings.aboutInfo.channel` 键。
4. 更新检查、应用版本和第三方许可展示合同保持不变。
5. 目标测试、类型检查、lint、web build 和 scoped diff 检查通过，或仅有明确的既有非阻断 warning。

