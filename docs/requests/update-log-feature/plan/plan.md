# 更新日志功能 - 执行计划

## 任务拆解

### T1: 创建 CHANGELOG.md 数据文件

**描述**: 在项目根目录创建 CHANGELOG.md 文件，遵循 Keep a Changelog 规范，包含初始版本记录。

**关联规格**: spec/spec.md - CHANGELOG.md 格式

**执行内容**:
- 创建 `CHANGELOG.md` 文件
- 添加 v0.1.0 版本记录（初始版本）
- 添加 v0.2.0 版本记录（更新日志功能）

**验收标准**: 文件存在且格式符合 Keep a Changelog 规范

**执行模式**: 串行

---

### T2: 实现更新检测服务 (update-service.ts)

**描述**: 创建更新检测服务，实现 CHANGELOG.md 解析、GitHub Releases API 调用、版本号比较逻辑。

**关联规格**: spec/spec.md - update-service.ts 接口定义

**执行内容**:
- 创建 `src/renderer/services/update-service.ts`
- 实现 `parseChangelog()` 方法：解析 CHANGELOG.md 为结构化数据
- 实现 `checkForUpdates()` 方法：调用 GitHub Releases API
- 实现 `compareVersions()` 方法：Semantic Versioning 比较
- 导出 `UpdateService` 接口和实现类

**验收标准**:
- CHANGELOG.md 解析正确（单元测试）
- 版本号比较逻辑正确（单元测试）
- 网络异常时静默降级

**执行模式**: 串行（TDD：先写测试再实现）

---

### T3: 实现更新状态管理 (stores/update.ts)

**描述**: 创建 Pinia store 管理更新相关状态。

**关联规格**: spec/spec.md - stores/update.ts 接口定义

**执行内容**:
- 创建 `src/renderer/stores/update.ts`
- 定义 `UpdateState` 接口
- 实现 `checkForUpdates()` action
- 实现 `parseChangelog()` action
- 实现 `setCurrentVersion()` action

**验收标准**: Store 状态管理正确，actions 可正常调用

**执行模式**: 串行（依赖 T2）

---

### T4: 实现新版本特性弹窗 (WhatsNewModal.vue)

**描述**: 创建新版本特性弹窗组件，展示当前版本的更新摘要。

**关联规格**: spec/spec.md - WhatsNewModal.vue 设计

**执行内容**:
- 创建 `src/renderer/components/WhatsNewModal.vue`
- 实现 Props: visible, version, changelog
- 实现 Events: close, dontRemind
- 实现 localStorage 持久化逻辑
- 使用 Element Plus el-dialog 组件

**验收标准**:
- 弹窗正确显示版本信息
- 「知道了」按钮关闭弹窗
- 「不再提醒此版本」勾选后记忆

**执行模式**: 串行（TDD：先写测试再实现）

---

### T5: 实现更新日志页面 (UpdateLog.vue)

**描述**: 创建更新日志查看页面，展示历史版本变更记录。

**关联规格**: spec/spec.md - UpdateLog.vue 设计，design/page-design.md

**执行内容**:
- 创建 `src/renderer/views/UpdateLog.vue`
- 实现版本列表渲染（时间倒序）
- 实现版本卡片组件（版本号、日期、变更分类）
- 实现当前版本和最新版本信息展示
- 实现「前往下载」按钮
- 使用 Tailwind CSS 类名样式

**验收标准**:
- 页面正确渲染版本列表
- 版本卡片包含版本号、日期和变更分类
- 下载按钮跳转正确

**执行模式**: 串行（TDD：先写测试再实现）

---

### T6: 配置路由和菜单入口

**描述**: 在路由配置中添加更新日志页面路由，在侧边栏菜单中添加入口。

**关联规格**: spec/spec.md - 页面与模块设计

**执行内容**:
- 修改 `src/renderer/router/index.ts` 添加 `/update-log` 路由
- 修改 `src/renderer/App.vue` 添加侧边栏菜单项（Bell 图标）
- 修改 `src/renderer/App.vue` 添加状态栏更新提示（红点）

**验收标准**:
- 路由 `/update-log` 可正常访问
- 侧边栏显示「更新日志」菜单项
- 状态栏显示更新提示红点（有更新时）

**执行模式**: 串行（依赖 T5）

---

### T7: 集成更新检测到应用启动流程

**描述**: 在应用启动时初始化更新检测，显示新版本特性弹窗。

**关联规格**: spec/spec.md - 用户流程

**执行内容**:
- 修改 `src/renderer/main.ts` 初始化更新检测
- 实现启动时异步检查更新（不阻塞主界面）
- 实现版本变更后首次启动弹出 WhatsNewModal
- 修改 `src/renderer/App.vue` 集成 WhatsNewModal 组件

**验收标准**:
- 启动时异步检查更新（< 3秒）
- 版本变更后首次启动弹出弹窗
- 不阻塞主界面加载

**执行模式**: 串行（依赖 T3, T4）

---

### T8: 设置页面集成更新检查开关

**描述**: 在设置页面中集成更新检查配置项。

**关联规格**: spec/spec.md - 页面与模块设计

**执行内容**:
- 修改 `src/renderer/views/Settings.vue` 添加「更新设置」tab
- 实现更新检查开关（使用 configStore.checkUpdates）
- 实现「查看更新日志」按钮入口

**验收标准**:
- 设置页面显示更新检查开关
- 开关状态正确持久化
- 「查看更新日志」按钮跳转正确

**执行模式**: 串行（依赖 T6）

---

### T9: 端到端测试与验证

**描述**: 执行端到端测试，验证所有验收标准。

**关联规格**: spec/spec.md - 验收标准

**执行内容**:
- 验证 AC-01 到 AC-15 所有验收条件
- 记录测试结果到 verification/verification.md

**验收标准**: 所有验收条件通过

**执行模式**: 串行（依赖 T1-T8）

---

## 依赖关系

```
T1 (CHANGELOG.md)
  ↓
T2 (update-service.ts) ← 需要 CHANGELOG.md
  ↓
T3 (stores/update.ts) ← 依赖 T2
  ↓
T4 (WhatsNewModal.vue) ← 依赖 T3
  ↓
T5 (UpdateLog.vue) ← 依赖 T2
  ↓
T6 (路由和菜单) ← 依赖 T5
  ↓
T7 (启动流程集成) ← 依赖 T3, T4
  ↓
T8 (设置页面集成) ← 依赖 T6
  ↓
T9 (端到端验证) ← 依赖 T1-T8
```

## 并行执行建议

当前任务集规模适中（9 个任务），任务间依赖关系紧密，不建议启用 workflow-style 并行执行。所有任务按串行顺序执行。

**推荐执行顺序**: T1 → T2 → T3 → T4 → T5 → T6 → T7 → T8 → T9

## 受影响文件或模块

| 文件/模块 | 变更类型 | 关联任务 |
|-----------|----------|----------|
| `CHANGELOG.md` | 新增 | T1 |
| `src/renderer/services/update-service.ts` | 新增 | T2 |
| `src/renderer/stores/update.ts` | 新增 | T3 |
| `src/renderer/components/WhatsNewModal.vue` | 新增 | T4 |
| `src/renderer/views/UpdateLog.vue` | 新增 | T5 |
| `src/renderer/router/index.ts` | 修改 | T6 |
| `src/renderer/App.vue` | 修改 | T6, T7 |
| `src/renderer/main.ts` | 修改 | T7 |
| `src/renderer/views/Settings.vue` | 修改 | T8 |

## 测试策略

### 单元测试

| 测试目标 | 测试内容 | 测试框架 |
|----------|----------|----------|
| update-service.ts | CHANGELOG.md 解析、版本号比较、更新检查 | Vitest |
| stores/update.ts | 状态管理、actions | Vitest |
| WhatsNewModal.vue | 弹窗交互、localStorage 持存 | Vue Test Utils |

### 集成测试

| 测试目标 | 测试内容 |
|----------|----------|
| 更新检测流程 | 启动 → 检查 → 状态更新 → UI 反馈 |
| 弹窗触发流程 | 版本变更 → 弹窗 → 记忆 → 不再弹出 |

### 端到端测试

验证所有 AC-01 到 AC-15 验收条件。

## 回滚说明

如果实现过程中发现问题：

1. **T2-T5 失败**: 回退到 T1，重新设计接口
2. **T6-T8 失败**: 回退到 T5，检查组件实现
3. **T9 验证失败**: 回退到对应任务修复问题

每个任务完成后应提交代码，便于回滚。
