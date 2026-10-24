# 请求：统一主题切换与 bilibili 主色

## 请求标识

`theme-switch-bilibili-accent`

## 来源

用户直接反馈并确认方案：深色/浅色切换无效，界面同时出现黑白区域，主要颜色改为 bilibili 主题色；确认使用 `#FB7299`，保留自定义主色并迁移旧默认 `#FF6699`。

## 业务摘要与目标

让浅色、深色、跟随系统成为单一一致的视觉状态，消除 Tailwind 与 Element Plus 主题判定冲突，并以 bilibili 粉作为默认品牌主色。

## 初始完成信号

- 三种主题选择即时生效且整页一致。
- 系统模式随操作系统主题变化。
- 默认主色与 Element Plus 派生状态统一为 `#FB7299` 色系。
- 自定义非旧默认颜色继续保留。

## 触发与上下文

入口为“设置 -> 界面 -> 主题/主色”；上下文包括 Tailwind 4、Element Plus dark variables、`applyUiProjection`、shell runtime 和 Rust UI 默认值。

## 人工交接点

设计方案已由用户明确确认，无未决产品选择。

## 影响区域与模块

应用壳层主题投影、系统主题监听、Tailwind dark variant、Element Plus 主色变量、UI 偏好默认值与相关测试。

