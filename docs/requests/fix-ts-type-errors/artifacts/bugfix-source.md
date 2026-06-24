# 缺陷来源

## 来源系统

- 系统：本地项目 TypeScript 类型检查
- 命令：`npm run type-check` (`tsc --noEmit -p tsconfig.eslint.json`)

## 缺陷标题

项目 TypeScript 编译存在 100+ 个类型错误

## 观察到的行为

运行 typecheck 时输出大量错误，exit code 非零。主要错误分布：

### 1. 模块声明缺失 (3 个错误)
- `src/main/modules/asr/index.ts:51` - `whisper-node` 无类型声明
- `src/main/modules/tts/index.ts:14` - `edge-tts` 无类型声明
- `src/renderer/main.ts:4-5` - `element-plus/dist/locale/*.mjs` 无类型声明

### 2. 导出成员缺失 (8 个错误)
- `src/renderer/plugins/plugin-loader.ts` - 缺少 `PluginLoadError`、`PluginValidationError`、`PLUGIN_MANIFEST_FILE`、`PLUGIN_ENTRY_FILE`
- `src/renderer/plugins/plugin-manager.ts` - 缺少 `PluginEvent`、`PluginError`、`PluginLoadError`
- `src/renderer/plugins/plugin-sandbox.ts` - 缺少 `PluginPermissionError`

### 3. 属性/方法不存在 (10 个错误)
- `plugin-loader.ts` - IPC 接口缺少 `checkDirectoryExists`、`checkFileExists`、`readFile`、`getDirectories`
- `plugin-manager.ts` - IPC 接口缺少 `readFile`、`copyDirectory`、`deleteDirectory`；`Plugin` 缺少 `permissions`、`main`

### 4. unknown 类型未断言 (~60 个错误)
- `services/recording-service.ts` - 大量 `result` 未类型断言
- `services/asr-service.ts` - `result` 未类型断言
- `services/llm-service.ts` - `result` 未类型断言
- `services/tts-service.ts` - `result` 未类型断言
- `stores/recording.ts` - 大量 `result` 未类型断言

### 5. 类型不匹配 (5 个错误)
- `gift-sound/sound-queue.ts:75` - `undefined` vs `null`
- `recording/recording-engine.ts:145` - 对象 vs `RecordingStatus | null`
- `ai/types.ts:93,113` - `"friendly"` 不在允许的字面量联合类型中

### 6. 未使用声明 (10 个错误)
- 多个文件中导入了但未使用的类型/变量

### 7. 回调参数类型不兼容 (3 个错误)
- `stores/recording.ts` - `on` 回调参数类型为 `unknown`，与具体类型不兼容

## 预期行为

`npm run type-check` 应通过，无任何类型错误。

## 复现步骤

1. `npm install`
2. `npm run type-check`
3. 观察到 100+ 个错误

## 影响模块

- 主进程：ASR、TTS、礼物音效、录制模块
- 渲染进程：插件系统、服务层、状态管理、AI 类型定义

## 可用日志

完整错误输出已保存，共 100+ 行错误信息。
