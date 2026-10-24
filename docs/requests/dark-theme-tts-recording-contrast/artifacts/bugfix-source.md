# Bugfix Source

## 来源系统

当前本地 Codex 对话。

## 来源范围 / 缺陷标识

- 范围：语音播报、直播录制页。
- 标识：`dark-theme-tts-recording-contrast`。
- 标题：语音播报和直播录制深色主题内容不清晰。

## Observed Behavior

两页存在与已确认问题相同的深色对比度不足。代码定位到只带 `text-gray-*` / `text-red-*` / `text-amber-*` / 浅色边框与背景 utility 的可见节点。

## Expected Behavior

两页及其直接可见子组件在深色主题下保持文字、状态和分隔层级清晰可读。

## 复现线索

启用深色主题后访问 `/tts` 或 `/recording`。

## 受影响模块

TTS 页与队列面板；录制页、状态栏与设置弹窗。

## 可用证据

用户对同类问题的直接补充；目标源码中未配对的浅色主题 utility。

## Verbatim Defect Evidence Index

- U1 observed/scope：“语音播报、直播录制也有相同问题”。
- U1 expected 通过“相同问题”继承当前上下文：深色主题下内容需清晰可读。

## Closed-World Fix Scope Allowlist

- 两页及其当前直接可见子组件的深色文字、状态、边框、分隔线与快捷键表面颜色。
- 针对上述颜色配对的回归测试。
- `product-content delta: none`。

## 非范围

布局、文案、数据、表单、按钮、路由、store、IPC、Rust、语音合成与录制逻辑。

## 开放问题

无；深色层级已由当前问题上下文和项目已有变体明确。
