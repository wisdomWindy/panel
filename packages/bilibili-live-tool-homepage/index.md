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

<div class="download-section">
  <div class="download-buttons">
    <a href="https://github.com/wisdomWindy/panel/releases/latest/download/Bilibili直播助手-Setup.exe" class="download-btn windows">
      <span class="download-icon">🪟</span>
      <span class="download-text">
        <span class="download-label">下载 Windows 版</span>
        <span class="download-version">v2.0.0 · exe</span>
      </span>
    </a>
    <a href="https://github.com/wisdomWindy/panel/releases/latest/download/Bilibili直播助手.dmg" class="download-btn macos">
      <span class="download-icon">🍎</span>
      <span class="download-text">
        <span class="download-label">下载 macOS 版</span>
        <span class="download-version">v2.0.0 · dmg</span>
      </span>
    </a>
    <a href="https://github.com/wisdomWindy/panel/releases/latest/download/Bilibili直播助手.AppImage" class="download-btn linux">
      <span class="download-icon">🐧</span>
      <span class="download-text">
        <span class="download-label">下载 Linux 版</span>
        <span class="download-version">v2.0.0 · AppImage</span>
      </span>
    </a>
  </div>
  <p class="download-hint">
    也可以前往 <a href="https://github.com/wisdomWindy/panel/releases">GitHub Releases</a> 页面下载其他版本
  </p>
</div>

<style>
.download-section {
  text-align: center;
  margin: 2rem 0;
}

.download-buttons {
  display: flex;
  justify-content: center;
  gap: 1rem;
  flex-wrap: wrap;
  margin-bottom: 1rem;
}

.download-btn {
  display: inline-flex;
  align-items: center;
  gap: 0.75rem;
  padding: 0.75rem 1.5rem;
  border-radius: 8px;
  text-decoration: none;
  color: white;
  font-weight: 500;
  transition: transform 0.2s, box-shadow 0.2s;
}

.download-btn:hover {
  transform: translateY(-2px);
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.15);
}

.download-btn.windows {
  background: linear-gradient(135deg, #0078d4, #005a9e);
}

.download-btn.macos {
  background: linear-gradient(135deg, #333, #000);
}

.download-btn.linux {
  background: linear-gradient(135deg, #dd4814, #b33a10);
}

.download-icon {
  font-size: 1.5rem;
}

.download-text {
  display: flex;
  flex-direction: column;
  align-items: flex-start;
}

.download-label {
  font-size: 0.9rem;
  font-weight: 600;
}

.download-version {
  font-size: 0.75rem;
  opacity: 0.8;
}

.download-hint {
  color: var(--vp-c-text-2);
  font-size: 0.875rem;
}

.download-hint a {
  color: var(--vp-c-brand);
  text-decoration: underline;
}
</style>

## 技术栈

- **前端框架**: Vue 3 + TypeScript
- **桌面框架**: Electron
- **构建工具**: Vite
- **状态管理**: Pinia
- **UI 组件**: Element Plus
- **包管理**: pnpm