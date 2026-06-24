# 请求：项目代码重构 — 可维护性、可读性、可扩展性、职责边界、组件复用

## 来源

- 来源系统：本地项目代码审查
- 触发方式：人工审查发现多个架构级问题

## 问题摘要

项目存在以下核心问题：

### 1. 职责边界模糊
- **Services 层职责过重**：`ASRService`(395行)、`TTSService`(441行)、`LLMService`(303行) 混合了 IPC 调用、业务逻辑、状态管理、媒体处理
- **Stores 层与 Services 层耦合**：`stores/recording.ts` 直接调用 `window.electronAPI.invoke()`，绕过 Service 层
- **Views 层包含业务逻辑**：`Settings.vue`(1081行) 是上帝组件，混合了 14 个配置表单

### 2. 代码重复
- `formatDuration` 在 3 个文件中重复
- `formatFileSize` 在 2 个文件中重复
- `formatDate` 在 2 个文件中重复
- IPC 错误处理模式 (`if (!result.success) throw`) 重复 50+ 次

### 3. 可维护性差
- `Settings.vue` 1081 行，难以定位和修改
- 服务类过大，违反单一职责原则
- 大量 `as any` 类型断言

### 4. 可读性差
- IPC 调用样板代码淹没业务逻辑
- 表单定义与 UI 混合

### 5. 可扩展性差
- 添加新功能需修改多处
- 无共享工具函数/组合式函数
- 组件复用度低

## 目标

通过系统性重构，使项目达到：
- 每个文件职责单一，行数可控（<200行）
- 消除代码重复，提取公共工具
- 类型安全，消除 `as any`
- 组件可复用
- 新功能易于添加

## 完成信号

- `npm run type-check` 通过
- `npm run lint:check` 错误数不增加
- 所有现有功能正常工作
- 代码行数和文件数合理增长
