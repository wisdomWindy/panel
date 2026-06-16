# 多语言支持

## 请求标识

- **request-id**: multi-language
- **创建时间**: 2026-06-16

## 来源

`docs/requirements/Bilibili_Live_Tool_PRD.md` 多语言支持功能。

## 业务摘要

为 Bilibili 直播助手提供多语言支持能力，使应用能够根据用户语言偏好显示对应的界面文本。当前阶段支持中文（zh-CN）和英文（en-US）两种语言，预留国际化接口便于后续扩展其他语言。

## 目标声明

在 Bilibili 直播助手中实现完整的多语言支持，具体包括：

1. 集成 vue-i18n 国际化框架
2. 配置中文和英文语言包
3. 所有界面文本使用 i18n key 替代硬编码文本
4. 设置页面提供语言切换功能
5. 语言选择持久化到本地存储
6. 应用启动时自动加载用户语言偏好

## 初始完成信号

- [ ] vue-i18n 框架集成完成
- [ ] 中文语言包配置完整
- [ ] 英文语言包配置完整
- [ ] 所有页面文本使用 i18n key
- [ ] 设置页面可切换语言
- [ ] 语言选择持久化
- [ ] 启动时加载语言偏好
- [ ] 新增功能预留国际化接口

## 影响范围

- `src/renderer/main.ts` — 初始化 vue-i18n
- `src/renderer/locales/zh-CN.json` — 中文语言包（已有，需补充）
- `src/renderer/locales/en-US.json` — 英文语言包（已有，需补充）
- `src/renderer/views/Settings.vue` — 设置页面添加语言切换
- `src/renderer/stores/config.ts` — 配置存储添加语言选项
- 所有 `.vue` 文件 — 使用 `$t()` 替代硬编码文本

## 参与模块

- vue-i18n 国际化框架
- Pinia 状态管理（语言配置持久化）
- Element Plus UI 组件（支持多语言）
- 所有页面组件（使用 i18n key）
