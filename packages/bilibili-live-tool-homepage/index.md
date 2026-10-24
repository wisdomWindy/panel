---
layout: home

hero:
  name: Bilibili 直播助手
  text: 直播间观测与内容工具
  tagline: 实时掌握直播动态，沉淀直播内容
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
---

<div class="features-grid">
  <div class="feature-item">
    <div class="feature-icon"><el-icon :size="16"><ChatDotRound /></el-icon></div>
    <h3>智能弹幕</h3>
    <p>支持弹幕监控、关键词过滤、弹幕播报等功能</p>
  </div>
  <div class="feature-item">
    <div class="feature-icon"><el-icon :size="16"><Present /></el-icon></div>
    <h3>礼物音效</h3>
    <p>为不同礼物设置专属音效，增强直播互动氛围</p>
  </div>
  <div class="feature-item">
    <div class="feature-icon"><el-icon :size="16"><VideoCamera /></el-icon></div>
    <h3>直播录制</h3>
    <p>自动录制直播内容，支持精彩时刻标记</p>
  </div>
  <div class="feature-item">
    <div class="feature-icon"><el-icon :size="16"><MagicStick /></el-icon></div>
    <h3>AI 文本解析</h3>
    <p>集成 AI 大模型，在本地生成回复预览</p>
  </div>
  <div class="feature-item">
    <div class="feature-icon"><el-icon :size="16"><Globe /></el-icon></div>
    <h3>多语言支持</h3>
    <p>支持中文和英文界面，满足不同用户需求</p>
  </div>
</div>

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
      class="home-download-btn windows-btn"
    >
      <span class="text-2xl"><el-icon :size="16"><Monitor /></el-icon></span>
      <span class="flex flex-col items-start">
        <span class="text-sm font-semibold">下载 Windows 版</span>
        <span class="text-xs opacity-80">v2.0.0 · exe</span>
      </span>
    </a>
    <a
      href="https://github.com/wisdomWindy/panel/releases/latest/download/Bilibili直播助手.dmg"
      class="home-download-btn macos-btn"
    >
      <span class="text-2xl"><el-icon :size="16"><Platform /></el-icon></span>
      <span class="flex flex-col items-start">
        <span class="text-sm font-semibold">下载 macOS 版</span>
        <span class="text-xs opacity-80">v2.0.0 · dmg</span>
      </span>
    </a>
    <a
      href="https://github.com/wisdomWindy/panel/releases/latest/download/Bilibili直播助手.AppImage"
      class="home-download-btn linux-btn"
    >
      <span class="text-2xl"><el-icon :size="16"><Terminal /></el-icon></span>
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

