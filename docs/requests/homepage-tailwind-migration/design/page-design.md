# 官网首页文档 Tailwind 化页面设计

## page objective

在不改动官网信息架构和主要视觉效果的前提下，将首页与下载页的源码样式表达从 Markdown 内嵌 `<style>` 迁移为 Tailwind utility class，并为该 VitePress 子项目补齐独立的 Tailwind 源码入口。

## target users or scenarios

- 维护官网文档的开发者：后续新增页面或调整局部模块时，可继续用 Tailwind class 描述样式，而不必在 Markdown 中追加 `<style>`。
- 访问官网的用户：看到的首页与下载页布局、按钮层级、下载卡片和平台色彩基本保持现状，不因迁移出现明显视觉回退。

## layout structure

### 首页
- 保持 VitePress `layout: home` 的 hero 和 feature card 结构不变
- “最新版本” 区块继续使用 VitePress `tip`
- “下载安装” 区块继续保留三平台下载按钮横向排列、移动端换行
- “技术栈” 列表保持普通 Markdown 列表

### 下载页
- 继续保持三张平台下载卡片组成的响应式网格
- 卡片内部顺序保持为：图标、平台名、平台说明、下载按钮、文件大小
- 其余“安装说明”“系统要求”“其他版本”“从源码构建”仍沿用普通文档流

## visual hierarchy

- 一级层级仍由 VitePress 页面标题 / hero 承担
- 首页下载区块的主视觉焦点保持在三平台渐变按钮
- 下载页的主视觉焦点保持在三张下载卡片和卡片内按钮
- 副文案继续使用较低对比度文本
- 平台差异仍通过按钮渐变色区分：Windows 蓝、macOS 深灰、Linux 橙色

## section breakdown

### 1. Tailwind 供应链层
- 新增 VitePress theme 入口以加载 Tailwind 样式文件
- 样式文件只负责 Tailwind 引入与必要 source/config 声明，不承载自定义业务样式

### 2. 首页下载区块
- 用 Tailwind 替代 `.download-section`、`.download-buttons`、`.download-btn`、`.download-text` 等类
- 保持按钮圆角、hover 上浮、阴影和双色渐变

### 3. 下载页平台卡片区块
- 用 Tailwind 替代 `.download-cards`、`.download-card`、`.card-icon`、`.file-info` 等类
- 保持卡片 soft 背景、边框、圆角、hover 上浮和阴影

## styling direction

- 样式只通过 Tailwind utility class 表达，不在 Markdown 中保留 `<style>` 块
- 可以使用 Tailwind 任意值类表达当前颜色和尺寸，例如 `bg-[linear-gradient(...)]`、`shadow-[...]`
- 优先保留现有视觉结果，而不是借机做主题重绘
- Tailwind 入口风格与仓库现有 `packages/bilibili-live-tool/src/renderer/styles/tailwind.css` 保持一致，减少认知差异

## interaction skeleton

- 首页与下载页交互不新增状态机，仅保留现有链接跳转和 hover 反馈
- 下载按钮 hover：轻微上移与阴影增强
- 下载卡片 hover：轻微上移与阴影增强
- 不增加 JS 交互、不引入动画库

## responsive behavior

- 首页下载按钮组：桌面端横向排列，窄屏下自动换行并保持居中
- 下载卡片区：桌面端多列，窄屏下退化为单列或双列自适应网格
- 按钮与卡片的内边距、字级保持现状附近的视觉密度，不因迁移导致窄屏拥挤

## design risks

1. VitePress 子项目当前尚无独立 Tailwind theme 入口，若接入方式不对，构建可能不输出 utility class。
2. Markdown 内嵌 HTML 使用 Tailwind 任意值类时，若内容扫描范围缺失，生产构建可能漏样式。
3. 直接把复杂渐变和阴影改成 utility class 时，若类名过长，页面可读性会变差，需要适度保留语义容器结构。

## open UI questions

- 无额外 UI 待确认项。用户已确认“保持现有效果，只做 Tailwind 化迁移”。
