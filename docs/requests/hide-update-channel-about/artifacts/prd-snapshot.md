# 直接变更来源快照

## Verbatim Source Evidence Index

- U1：“更新通道不需要显示在关于与许可页面”

## Source Summary And Goal

从关于与许可页面移除更新通道这一展示字段。[source-backed: U1]

## Greenfield Scaffold Sensitivity

不适用：不是项目、应用、包或前端业务面的创建请求。

## Explicit Behavior Constraints

- 删除“更新通道”标签及当前通道值的可见行。[source-backed: U1]
- 页面其余信息和操作保持不变。[source-derived: U1 的封闭移除范围]
- 删除只服务于该展示的中英文文案键。[code-fact-backed: 引用闭包清理]
- 保留 `release.about.channel` 数据合同及更新逻辑。[code-fact-backed: store/desktop contract 仍拥有该字段]

## Closed-World Product Scope Allowlist

- 允许：关于与许可页面不再展示更新通道。
- 不允许：删除或修改更新通道数据、更新检查、版本展示、许可表格、反馈文案、导航或其他设置项。
- 不新增字段、文案、控件、状态、默认值、校验、权限、导航、API/payload 或埋点。

## Displays And Interactions

- 删除展示字段：`settings.aboutInfo.channel` 标签和 `release.about?.channel` 值。
- 保留交互：检查更新按钮及其 loading/success/error 行为。
- 表单、表格、空态、错误态：不变。

## Relevant Modules And Open Questions

- `views/settings/index.vue`
- `views/settings/settings-shell-contract.test.ts`
- `locales/zh-CN.json`
- `locales/en-US.json`
- Open questions：none。

