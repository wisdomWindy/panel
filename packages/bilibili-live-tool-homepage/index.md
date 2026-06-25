---
layout: home

hero:
  name: Bilibili 直播助手
  text: 智能弹幕与互动工具
  tagline: 提升直播互动体验，让直播更有趣
  image:
    src: /logo.svg
    alt: Bilibili 直播助手
  actions:
    - theme: brand
      text: 快速开始
      link: /guide/
    - theme: alt
      text: 功能介绍
      link: /features/
    - theme: alt
      text: GitHub
      link: https://github.com/wisdomWindy/panel

features:
  - icon: 🎙️
    title: 智能弹幕
    details: 支持弹幕自动回复、关键词过滤、弹幕播报等功能
  - icon: 🎁
    title: 礼物音效
    details: 为不同礼物设置专属音效，增强直播互动氛围
  - icon: 🎬
    title: 直播录制
    details: 自动录制直播内容，支持精彩时刻标记
  - icon: 🤖
    title: AI 对话
    details: 集成 AI 大模型，实现智能弹幕互动
  - icon: 🔌
    title: 插件系统
    details: 可扩展的插件架构，支持自定义功能
  - icon: 🌐
    title: 多语言支持
    details: 支持中文和英文界面，满足不同用户需求
---

## 最新版本

::: tip v2.0.0 发布
- 新增礼物音效系统
- 新增错误日志记录功能
- 优化弹幕管理性能
- 支持 Monorepo 架构
:::

## 快速体验

```bash
# 克隆项目
git clone https://github.com/wisdomWindy/panel.git

# 进入项目目录
cd panel/packages/bilibili-live-tool

# 安装依赖
pnpm install

# 启动开发模式
pnpm dev
```

## 技术栈

- **前端框架**: Vue 3 + TypeScript
- **桌面框架**: Electron
- **构建工具**: Vite
- **状态管理**: Pinia
- **UI 组件**: Element Plus
- **包管理**: pnpm