# 问题规范化描述

## 重构目标

将项目从当前的"大文件 + 职责混合"架构，重构为"分层清晰 + 职责单一 + 组件可复用"的架构。

## 重构维度

### 1. 提取公共工具层 (`utils/`)
- 时间格式化工具：`formatDuration`, `formatFileSize`, `formatDate`
- IPC 通信封装：统一的 IPC 调用和错误处理
- 通用 UI 工具：状态标签映射、确认对话框封装

### 2. 拆分过大的 Services
- `ASRService` → `ASRService`(纯 IPC) + `useWebSpeechRecognition`(composable)
- `TTSService` → `TTSService`(纯 IPC) + `useBroadcastQueue`(composable)
- `LLMService` → `LLMService`(纯 IPC) + `useChatContext`(composable)

### 3. 统一 Stores 层调用路径
- Stores 只调用 Services，不直接调用 IPC
- Stores 只负责状态管理和业务编排

### 4. 拆分上帝组件
- `Settings.vue` → 14 个独立的设置子组件
- 提取 `useSettingsForm` composable

### 5. 提取可复用组件
- `ActionButtons` — 表格行操作按钮组
- `ConfigSection` — 配置区块包装器
- `EmptyState` — 空状态展示

## 验收标准

- 每个文件 <200 行（除配置和类型定义）
- 零代码重复（工具函数统一引用）
- `as any` 断言减少 80%+
- 新增设置项只需添加一个子组件
