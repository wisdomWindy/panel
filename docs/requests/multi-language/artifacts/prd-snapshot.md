# PRD 快照

## 来源摘要

来自 `docs/requirements/Bilibili_Live_Tool_PRD.md` 多语言支持功能。该功能为 Could Have 级别功能，旨在为应用提供国际化支持，使应用能够根据用户语言偏好显示对应的界面文本。

## 关键业务目标

- 支持中文和英文两种语言
- 预留国际化接口，便于后续扩展其他语言
- 用户可自由切换语言
- 语言选择持久化，重启后保持用户偏好

## 相关模块/页面

- `src/renderer/main.ts`：应用入口，初始化 vue-i18n
- `src/renderer/locales/zh-CN.json`：中文语言包（已有基础）
- `src/renderer/locales/en-US.json`：英文语言包（已有基础）
- `src/renderer/views/Settings.vue`：设置页面，添加语言切换
- `src/renderer/stores/config.ts`：配置存储，添加语言选项
- 所有 `.vue` 页面组件：使用 `$t()` 替代硬编码文本

## 上游需确认问题

1. 语言切换是否需要实时生效（无需重启）？
2. 是否需要支持自动检测系统语言？
3. 后续是否需要扩展日语、韩语等其他语言？
4. Element Plus 组件库的多语言如何配置？

## 现有基础

项目已有以下国际化基础：
- `vue-i18n` 依赖已安装（package.json）
- `locales/zh-CN.json` 和 `locales/en-US.json` 已存在
- 语言包包含：app、nav、common、danmaku、effect、ai、dashboard、blacklist、connection、settings、logs、plugins、gift 等模块的翻译
