# 修复计划

## 执行顺序

### Task 1: 创建模块声明文件
- 创建 `src/typings/whisper-node.d.ts`
- 创建 `src/typings/edge-tts.d.ts`
- 创建 `src/typings/element-plus-locale.d.ts`

### Task 2: 补充插件类型定义
- 修改 `src/renderer/plugins/types.ts`：添加缺失的类、常量和接口字段

### Task 3: 扩展 IPC 接口类型
- 修改 `src/preload/index.ts`：扩展 `electronAPI` 类型声明

### Task 4: 修复 unknown 类型断言
- 修改 `src/renderer/services/recording-service.ts`
- 修改 `src/renderer/services/asr-service.ts`
- 修改 `src/renderer/services/llm-service.ts`
- 修改 `src/renderer/services/tts-service.ts`
- 修改 `src/renderer/stores/recording.ts`

### Task 5: 修复类型不匹配
- 修改 `src/main/modules/gift-sound/sound-queue.ts`
- 修改 `src/main/modules/recording/recording-engine.ts`
- 修改 `src/renderer/modules/ai/types.ts`

### Task 6: 清理未使用声明
- 删除各文件中未使用的导入和变量

### Task 7: 修复回调类型兼容性
- 修改 `src/renderer/stores/recording.ts` 中 `on` 回调类型

## 依赖关系

Task 1-3 为类型定义层，Task 4-7 依赖类型定义。按顺序执行。

## 验证

每完成一个 Task 后运行 `npm run type-check` 确认错误减少。
