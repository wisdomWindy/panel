# 请求：dark-theme-tts-recording-contrast

## 来源

用户在完成提醒配置页深色对比度修复后直接补充：“语音播报、直播录制也有相同问题”。

## 业务摘要

语音播报与直播录制页中的部分自定义 Tailwind 文字、状态、分隔线和快捷键表面只有浅色主题颜色，在深色主题下对比度不足。

## 目标

为两个页面及其直接可见子组件补齐深色主题 utility，保持浅色主题、布局、文案、数据、状态和交互不变。

## 初始完成信号

- `/tts` 和 `/recording` 在根节点 `.dark` 下的标题、辅助文字、状态、数值、提示及分隔线清晰可读。
- 契约测试、类型检查、lint、Web 构建和两路由运行时检查通过。

## 触发条件

用户访问语音播报或直播录制页，且应用根节点带有 `.dark`。

## 初始上下文

- 用户原文和上一批同类深色对比度修复标准。
- `views/tts/index.vue`、`TtsQueuePanel.vue`、`views/recording/index.vue`、`RecordingStatusBand.vue`、`RecordingSettingsDialog.vue`。
- `media-pages-contract.test.ts`。

## 人工交接点

实现和验证完成后交付两个本地预览路由及工程检查结果。

## 影响区域

仅上述两个页面的可见深色主题样式和回归契约。

## 参与模块

语音播报页、语音队列、直播录制页、录制状态栏、录制设置弹窗和媒体页契约测试。
