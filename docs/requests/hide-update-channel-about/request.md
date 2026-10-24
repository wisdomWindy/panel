# 请求：关于与许可页面隐藏更新通道

## Request Identifier

`hide-update-channel-about`

## Source

- U1：用户直接指令：“更新通道不需要显示在关于与许可页面”。

## Business Summary And Goal

关于与许可页面不再向用户展示“更新通道”标签及其值；应用版本、检查更新按钮和第三方许可内容保持不变。

## Initial Done Signal

- 关于与许可模板不再渲染更新通道行。
- 中英文更新通道展示文案无孤立引用。
- 更新检查与 release 通道数据合同不变。
- 相关测试、类型检查和构建通过。

## Trigger And Context

- 触发条件：用户明确要求移除该展示项。
- 上下文：`views/settings/index.vue`、设置页合同测试、`zh-CN.json`、`en-US.json`、release store/contract 引用。
- 人工交接点：完成后报告改动和验证结果；无待确认产品决策。

## Affected Area And Modules

- 设置页“关于与许可”分区。
- 设置页合同测试。
- 中英文设置页文案资源。
- 不涉及 release store、更新 API、Tauri/Rust 或安装包版本。

## Workflow Classification

- Direct change，`S0 trivial`，非 PRD。
- 不需要 requirement-analysis / requirement-splitting：用户目标、页面、移除项和完成信号均明确。
- 不需要 page-design：仅移除一个现有定义列表项，不重设计布局或交互。
- 不需要 architecture-design：无模块、状态、类型或依赖方向变化。

