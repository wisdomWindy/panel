# PRD 快照

## 来源摘要

来自 `docs/requirements/Bilibili_Live_Tool_PRD.md` 第 3.4 节「更新日志与版本管理」。该功能为桌面应用标准能力，使用户在应用内查看版本更新信息，无需外部跳转。

## 关键业务目标

- 用户可在应用内查看完整的历史更新日志
- 应用启动时静默检测新版本，不打扰用户操作
- 版本变更后首次启动展示新特性摘要，提升用户感知
- 更新检查失败时优雅降级，不影响正常使用

## 相关模块/页面

- `src/renderer/App.vue`：主框架，状态栏更新提示入口
- `src/renderer/views/Settings.vue`：设置页面，更新检查开关
- 新增 `src/renderer/views/UpdateLog.vue`：更新日志查看页面
- 新增 `src/renderer/components/WhatsNewModal.vue`：新版本特性弹窗
- 新增 `src/renderer/services/update-service.ts`：更新检测服务
- 新增 `src/renderer/stores/update.ts`：更新状态管理
- `src/renderer/main.ts`：启动初始化
- `CHANGELOG.md`：更新日志数据源

## 上游需确认问题

1. CHANGELOG.md 的格式规范：采用 [Keep a Changelog](https://keepachangelog.com/) 规范，包含 Added / Changed / Fixed / Removed 分类
2. 更新检查源：当前阶段从 GitHub Releases 获取版本信息，需确认仓库地址
3. 版本号规范：遵循 Semantic Versioning（MAJOR.MINOR.PATCH）
4. "新版本特性"弹窗的本地状态持久化方式：使用 localStorage 记录已提示版本
