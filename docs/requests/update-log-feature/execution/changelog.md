# 更新日志功能 - 执行记录

## 2026-06-16

### T1: 创建 CHANGELOG.md
- 创建 `CHANGELOG.md` 文件，遵循 Keep a Changelog 规范
- 添加 v0.1.0 和 v0.2.0 版本记录

### T2: 实现更新检测服务
- 创建 `src/renderer/services/update-service.ts`
- 实现 `parseChangelog()` 方法：解析 CHANGELOG.md 为结构化数据
- 实现 `checkForUpdates()` 方法：调用 GitHub Releases API
- 实现 `compareVersions()` 方法：Semantic Versioning 比较
- 导出 `UpdateService` 接口和实现类

### T3: 实现状态管理
- 创建 `src/renderer/stores/update.ts`
- 定义 `UpdateState` 接口
- 实现 `checkForUpdates()` action
- 实现 `parseChangelog()` action
- 实现 `setCurrentVersion()` action

### T4: 实现新版本特性弹窗
- 创建 `src/renderer/components/WhatsNewModal.vue`
- 实现 Props: visible, version, changelog
- 实现 Events: close, dontRemind
- 实现 localStorage 持久化逻辑
- 使用 Element Plus el-dialog 组件

### T5: 实现更新日志页面
- 创建 `src/renderer/views/UpdateLog.vue`
- 实现版本列表渲染（时间倒序）
- 实现版本卡片组件（版本号、日期、变更分类）
- 实现当前版本和最新版本信息展示
- 实现「前往下载」按钮

### T6: 配置路由和菜单入口
- 修改 `src/renderer/router/index.ts` 添加 `/update-log` 路由
- 修改 `src/renderer/App.vue` 添加侧边栏菜单项（Bell 图标）
- 修改 `src/renderer/App.vue` 添加状态栏更新提示（红点）

### T7: 集成更新检测到启动流程
- 修改 `src/renderer/App.vue` 在 onMounted 中异步检查更新
- 实现版本变更后首次启动弹出 WhatsNewModal
- 集成 WhatsNewModal 组件

### T8: 设置页面集成更新检查开关
- 修改 `src/renderer/views/Settings.vue` 添加「更新设置」tab
- 实现更新检查开关（使用 configStore.checkUpdates）
- 实现「查看更新日志」按钮入口

### T9: 端到端测试与验证
- 验证所有 15 个验收标准
- 记录验证结果到 verification/verification.md
