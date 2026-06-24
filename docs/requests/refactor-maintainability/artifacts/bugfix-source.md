# 代码库审查报告

## 审查范围

- `src/renderer/services/` — 9 个服务文件
- `src/renderer/stores/` — 12 个状态管理文件
- `src/renderer/views/` — 14 个视图文件
- `src/renderer/components/` — 7 个组件文件
- `src/renderer/plugins/` — 6 个插件系统文件

## 发现的问题

### A. 职责边界问题

| 文件 | 行数 | 问题 |
|------|------|------|
| `views/Settings.vue` | 1081 | 上帝组件，14 个配置表单混在一个文件 |
| `services/asr-service.ts` | 395 | 混合 IPC + Web Speech API + 录音 + 音频处理 |
| `services/tts-service.ts` | 441 | 混合 IPC + 队列管理 + 消息格式化 + 音频播放 |
| `services/llm-service.ts` | 303 | 混合 IPC + 聊天历史 + 回复逻辑 + 上下文管理 |
| `services/danmaku-service.ts` | 305 | 直接访问 stores，混合业务逻辑与状态更新 |
| `stores/recording.ts` | 372 | 直接调用 IPC，绕过 Service 层，混合工具函数 |

### B. 代码重复

| 函数 | 重复位置 |
|------|----------|
| `formatDuration` | `stores/recording.ts`, `components/RecordingList.vue`, `components/HighlightList.vue` |
| `formatFileSize` | `stores/recording.ts`, `components/RecordingList.vue` |
| `formatDate` | `components/RecordingList.vue`, `components/HighlightList.vue` |
| IPC 错误处理 | 所有 services 和 stores 文件（50+ 次） |

### C. 类型安全问题

- 所有 `window.electronAPI.invoke()` 调用使用 `as any` 断言
- 缺少统一的 IPC 响应类型定义
- 服务间通信缺乏类型约束

### D. 组件复用问题

- `StatCard` 组件可用但使用场景有限
- 表格操作按钮模式重复但未提取
- 配置表单项模式重复但未提取
