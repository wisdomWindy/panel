# 重构规格

## 目标

通过系统性重构提升项目的可维护性、可读性、可扩展性、职责边界清晰度和组件复用度。

## 重构范围

### 1. 提取公共工具层

**新增文件：**
- `src/renderer/utils/format.ts` — 时间/文件大小/日期格式化
- `src/renderer/utils/ipc.ts` — 统一 IPC 调用封装
- `src/renderer/utils/ui.ts` — UI 工具函数（状态标签、确认对话框）

**消除重复：**
- `formatDuration` — 从 3 个文件提取到 `utils/format.ts`
- `formatFileSize` — 从 2 个文件提取到 `utils/format.ts`
- `formatDate` — 从 2 个文件提取到 `utils/format.ts`
- IPC 错误处理 — 从 50+ 处提取到 `utils/ipc.ts`

### 2. 统一 IPC 类型定义

**新增文件：**
- `src/renderer/utils/ipc-types.ts` — IPC 通道和响应类型定义

**消除 `as any`：**
- 为每个 IPC 通道定义请求/响应类型
- `invoke` 函数使用泛型约束返回类型

### 3. 拆分过大的 Services

**重构 `ASRService`：**
- 保留 `ASRService` — 纯 IPC 调用
- 新增 `src/renderer/composables/useWebSpeechRecognition.ts` — Web Speech API 封装
- 新增 `src/renderer/composables/useAudioRecording.ts` — 录音逻辑

**重构 `TTSService`：**
- 保留 `TTSService` — 纯 IPC 调用 + 配置管理
- 新增 `src/renderer/composables/useBroadcastQueue.ts` — 播报队列管理
- 新增 `src/renderer/composables/useSpeechSynthesis.ts` — Web Speech API 封装

**重构 `LLMService`：**
- 保留 `LLMService` — 纯 IPC 调用
- 新增 `src/renderer/composables/useChatContext.ts` — 聊天上下文管理
- 新增 `src/renderer/composables/useReplyLogic.ts` — 回复判断逻辑

### 4. 统一 Stores 层调用路径

**重构 `stores/recording.ts`：**
- 移除所有直接 IPC 调用
- 通过 `getRecordingService()` 调用
- 移除重复的工具函数，引用 `utils/format.ts`

### 5. 拆分 Settings.vue

**新增组件：**
- `src/renderer/components/settings/ConnectionSettings.vue`
- `src/renderer/components/settings/DanmakuSettings.vue`
- `src/renderer/components/settings/AutoSendSettings.vue`
- `src/renderer/components/settings/FilterSettings.vue`
- `src/renderer/components/settings/RoleSettings.vue`
- `src/renderer/components/settings/LogSettings.vue`
- `src/renderer/components/settings/UpdateSettings.vue`
- `src/renderer/components/settings/PluginSettings.vue`
- `src/renderer/components/settings/RecordingSettings.vue`
- `src/renderer/components/settings/LanguageSettings.vue`
- `src/renderer/components/settings/TTSSettings.vue`
- `src/renderer/components/settings/ASRSettings.vue`
- `src/renderer/components/settings/AISettings.vue`
- `src/renderer/components/settings/ModerationSettings.vue`

**新增 composable：**
- `src/renderer/composables/useSettingsForm.ts` — 表单状态和保存逻辑

### 6. 提取可复用组件

**新增组件：**
- `src/renderer/components/ui/ActionButtons.vue` — 表格行操作按钮组
- `src/renderer/components/ui/ConfigSection.vue` — 配置区块包装器

## 不变部分

- `src/main/` — 主进程代码不变更
- `src/preload/` — 预加载脚本不变更
- `src/renderer/modules/` — 模块类型定义不变更
- `src/renderer/plugins/` — 插件系统不变更
- `src/renderer/router/` — 路由配置不变更

## 验收标准

1. `npm run type-check` 通过
2. `npm run lint:check` 错误数不增加
3. 所有现有功能正常工作
4. 每个重构后的文件 <200 行
5. `as any` 断言减少 80%+
6. 零工具函数重复
