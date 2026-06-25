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
      text: 立即下载
      link: /download/
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

## 下载安装

<div class="my-8 text-center">
  <div class="mb-4 flex flex-wrap justify-center gap-4">
    <a
      href="https://github.com/wisdomWindy/panel/releases/latest/download/Bilibili直播助手-Setup.exe"
      class="inline-flex translate-y-0 items-center gap-3 rounded-lg bg-[linear-gradient(135deg,#0078d4,#005a9e)] px-6 py-3 text-white no-underline transition-[transform,box-shadow] duration-200 hover:-translate-y-0.5 hover:shadow-[0_4px_12px_rgba(0,0,0,0.15)]"
    >
      <span class="text-2xl">🪟</span>
      <span class="flex flex-col items-start">
        <span class="text-sm font-semibold">下载 Windows 版</span>
        <span class="text-xs opacity-80">v2.0.0 · exe</span>
      </span>
    </a>
    <a
      href="https://github.com/wisdomWindy/panel/releases/latest/download/Bilibili直播助手.dmg"
      class="inline-flex translate-y-0 items-center gap-3 rounded-lg bg-[linear-gradient(135deg,#333,#000)] px-6 py-3 text-white no-underline transition-[transform,box-shadow] duration-200 hover:-translate-y-0.5 hover:shadow-[0_4px_12px_rgba(0,0,0,0.15)]"
    >
      <span class="text-2xl">🍎</span>
      <span class="flex flex-col items-start">
        <span class="text-sm font-semibold">下载 macOS 版</span>
        <span class="text-xs opacity-80">v2.0.0 · dmg</span>
      </span>
    </a>
    <a
      href="https://github.com/wisdomWindy/panel/releases/latest/download/Bilibili直播助手.AppImage"
      class="inline-flex translate-y-0 items-center gap-3 rounded-lg bg-[linear-gradient(135deg,#dd4814,#b33a10)] px-6 py-3 text-white no-underline transition-[transform,box-shadow] duration-200 hover:-translate-y-0.5 hover:shadow-[0_4px_12px_rgba(0,0,0,0.15)]"
    >
      <span class="text-2xl">🐧</span>
      <span class="flex flex-col items-start">
        <span class="text-sm font-semibold">下载 Linux 版</span>
        <span class="text-xs opacity-80">v2.0.0 · AppImage</span>
      </span>
    </a>
  </div>
  <p class="text-sm text-[var(--vp-c-text-2)]">
    也可以前往
    <a
      href="https://github.com/wisdomWindy/panel/releases"
      class="text-[var(--vp-c-brand)] underline"
    >
      GitHub Releases
    </a>
    页面下载其他版本
  </p>
</div>

## 技术栈

- **前端框架**: Vue 3 + TypeScript
- **桌面框架**: Electron
- **构建工具**: Vite
- **状态管理**: Pinia
- **UI 组件**: Element Plus
- **包管理**: pnpm
