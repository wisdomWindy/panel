# 归一化需求快照

## Verbatim Source Evidence Index

- U1：“语音播报、直播录制也有相同问题”。

## 来源摘要与目标

将已确认的深色主题可读性修复同步到语音播报和直播录制，不扩展业务行为。

## Greenfield Scaffold Sensitivity

不适用：非项目/应用/包/前端表面创建请求。

## 显式行为约束

- `.dark` 下主文字、次文字、警告/错误状态和分隔线需可读。
- 浅色 utility、DOM、文案、数据和交互保持不变。

## Closed-World Product Scope Allowlist

仅允许现有节点增加 `dark:` 颜色 utility 及相应回归断言；`product-content delta: none`。

## 表单、表格、展示与交互

触及页标题、队列标题/内容/计数、录制状态与数值、警告/错误提示、快捷键标识及弹窗辅助标签。不改字段语义或交互。

## 工作流与状态规则

路由和原有 TTS/录制状态链不变；仅根节点 `.dark` 命中时颜色结果改变。

## 相关模块

`/tts`、`/recording`、`TtsQueuePanel`、`RecordingStatusBand`、`RecordingSettingsDialog`。

## 开放问题

无。
