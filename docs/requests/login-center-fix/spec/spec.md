# 登录页居中样式规格

## 范围

调整 `src/renderer/views/Login.vue` 的根容器样式，确保所有内容（标题、表单、底部链接）在视窗中水平和垂直居中。

## 当前状态

`Login.vue` 的根 `<div>` 已使用 Tailwind CSS 类：

```html
<div class="h-screen w-screen flex flex-col items-center justify-center p-10" style="background: #F0F2F5">
```

此配置已实现居中布局 (`flex items-center justify-center`)，但 `p-10` 的 padding 和 `flex-col` 可能导致内容在特定视窗尺寸下不完全居中。

## 用户流程

1. 打开应用 → 进入登录页
2. 页面内容（标题、身份码输入框、开始连接按钮、底部链接）完整显示
3. 所有内容在视窗中严格水平垂直居中

## 模块设计

- **文件**: `src/renderer/views/Login.vue`
- **修改范围**: `<template>` 中根 `<div>` 的 CSS 类
- **不改动**: `<script setup>` 逻辑、Element Plus 组件配置

## 技术规格

将根容器修改为：

```html
<div class="h-screen w-screen flex flex-col items-center justify-center" style="background: #F0F2F5">
```

移除 `p-10`（可能破坏视觉居中），保留 `flex flex-col items-center justify-center` 确保内容在 flex 方向上的居中。`h-screen w-screen` 确保容器填满整个 Electron 窗口。

## 验收标准

1. 登录页所有元素（标题、输入框、按钮、底部链接）在视窗中水平和垂直居中
2. 窗口缩放时内容保持在中心位置
3. 页面功能行为不变（连接流程正常）
4. 无新增 TypeScript/ESLint 错误

## 风险

无。仅调整一个 CSS 类，不影响功能逻辑。