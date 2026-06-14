# 登录页居中样式调整

## 请求标识

- **request-id**: login-center-fix
- **创建时间**: 2026-06-12

## 来源

用户直接提出的界面调整需求，无外部 PRD 文档。

## 业务摘要

调整 `Login.vue` 页面的样式，确保页面内容在视窗中水平和垂直居中显示。当前页面使用 Tailwind CSS 的 flex 布局，但内容居中效果可能需要进一步优化。

## 影响范围

- **文件**: `src/renderer/views/Login.vue`
- **模块**: 登录页面 UI
- **技术栈**: Vue 3 + Tailwind CSS + Element Plus

## 参与模块

- 渲染进程登录视图 (`Login.vue`)