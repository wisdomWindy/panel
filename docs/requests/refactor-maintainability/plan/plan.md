# 重构执行计划

## 执行顺序

### Phase 1: 基础设施层（其他阶段依赖此阶段）

**Task 1.1: 创建公共工具函数**
- 创建 `src/renderer/utils/format.ts`
- 创建 `src/renderer/utils/ipc.ts`
- 创建 `src/renderer/utils/ui.ts`

**Task 1.2: 创建 IPC 类型定义**
- 创建 `src/renderer/utils/ipc-types.ts`

### Phase 2: Composables 层

**Task 2.1: 创建 ASR 相关 composables**
- 创建 `src/renderer/composables/useWebSpeechRecognition.ts`
- 创建 `src/renderer/composables/useAudioRecording.ts`

**Task 2.2: 创建 TTS 相关 composables**
- 创建 `src/renderer/composables/useBroadcastQueue.ts`
- 创建 `src/renderer/composables/useSpeechSynthesis.ts`

**Task 2.3: 创建 LLM 相关 composables**
- 创建 `src/renderer/composables/useChatContext.ts`
- 创建 `src/renderer/composables/useReplyLogic.ts`

**Task 2.4: 创建 Settings composable**
- 创建 `src/renderer/composables/useSettingsForm.ts`

### Phase 3: Services 层重构

**Task 3.1: 重构 ASRService**
- 移除 Web Speech API 和录音逻辑
- 使用 utils/ipc.ts 统一 IPC 调用

**Task 3.2: 重构 TTSService**
- 移除队列管理和 Web Speech 逻辑
- 使用 utils/ipc.ts 统一 IPC 调用

**Task 3.3: 重构 LLMService**
- 移除聊天历史和回复逻辑
- 使用 utils/ipc.ts 统一 IPC 调用

**Task 3.4: 重构其他 services**
- `recording-service.ts` 使用 utils/ipc.ts
- `gift-sound-service.ts` 使用 utils/ipc.ts
- `danmaku-service.ts` 使用 utils/ipc.ts

### Phase 4: Stores 层重构

**Task 4.1: 重构 stores/recording.ts**
- 移除直接 IPC 调用，通过 Service 层
- 使用 utils/format.ts 替换重复函数

### Phase 5: Components 层重构

**Task 5.1: 拆分 Settings.vue**
- 创建 14 个设置子组件
- 重构 Settings.vue 为容器组件

**Task 5.2: 更新现有组件**
- `RecordingList.vue` 使用 utils/format.ts
- `HighlightList.vue` 使用 utils/format.ts

**Task 5.3: 创建可复用 UI 组件**
- 创建 `ActionButtons.vue`
- 创建 `ConfigSection.vue`

## 依赖关系

```
Phase 1 (基础) → Phase 2 (Composables) → Phase 3 (Services) → Phase 4 (Stores) → Phase 5 (Components)
```

## 验证策略

每个 Phase 完成后：
1. `npm run type-check` 确认类型正确
2. `npm run lint:check` 确认无新错误
3. 手动验证相关功能正常

## 回滚策略

每个 Task 使用独立 git commit，可单独回滚。
