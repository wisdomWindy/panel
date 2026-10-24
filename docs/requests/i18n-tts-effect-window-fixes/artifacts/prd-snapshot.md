# 规范化缺陷快照

## 原文证据索引

见 `artifacts/bugfix-source.md` 的 U1-U5，均来自用户当前直接指令。

## 来源摘要

五项缺陷分属全局界面语言、TTS 默认/重置交互、上舰特效窗口尺寸/位置，但均落在当前免费版桌面应用的用户可见配置链路。

## 关键业务目标

- 用户选择 English 后能完整使用业务页面，而不是仅导航英文。
- TTS 的常用 Windows 中文音色默认可用，重置结果明确。
- 上舰特效窗口尺寸和位置与页面设置一致。

## 绿地脚手架敏感度

不适用：这是既有应用缺陷修复。

## 显式行为约束

- 语言切换必须复用现有 vue-i18n locale 状态，不新增第二套语言状态。
- 默认音色仅在当前配置没有已选音色时应用，不覆盖用户已有选择。
- 重置需调用现有保存通道；取消确认不得显示成功提示或修改配置。
- 窗口尺寸使用像素宽高，现有尺寸为兼容默认。
- 位置以主显示器为基准，支持现有五个枚举值。

## 闭集产品范围

只允许 U1-U5 对应的文案绑定、默认选择、结果反馈、宽高字段、位置实现及必要契约/测试变更。

## 表单、显示和交互

- TTS 音色下拉框：在无已选值时显示 Microsoft HuiHui（若系统返回该音色）。
- TTS 重置：确认、保存中、成功、失败、取消状态。
- 上舰特效：增加窗口宽度、窗口高度数值输入；保存和预览使用当前草稿值。
- 全局页面：切换 locale 后页面标题、标签、按钮、提示、空状态和弹窗文案同步变化。

## 工作流与状态规则

- locale 仍由 application-shell projection 驱动。
- TTS reset 由页面确认后调用 store.save，成功后用返回 snapshot 更新草稿。
- CaptainEffectConfig 从 renderer contract 经 IPC DTO 转成 domain config，worker 构造含尺寸和位置的 EffectWindowSpec，infrastructure 创建对应窗口。

## 相关模块或页面

免费版导航路由、live-data/media UI 组件、TTS 页面和 store、captain-effect 页面、media contracts/adapters、Rust media domain/application/IPC/infrastructure。

## 上游开放问题

无。

