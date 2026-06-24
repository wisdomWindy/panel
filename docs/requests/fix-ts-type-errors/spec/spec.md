# TypeScript 类型错误修复规格

## 目标

修复项目中所有 TypeScript 类型错误，使 `npm run type-check` 通过。

## 错误分类与修复策略

### A. 模块声明缺失 (TS2307 / TS7016)

**涉及文件：**
- `src/main/modules/asr/index.ts:51` — `whisper-node`
- `src/main/modules/tts/index.ts:14` — `edge-tts`
- `src/renderer/main.ts:4-5` — `element-plus/dist/locale/zh-cn.mjs`、`element-plus/dist/locale/en.mjs`

**修复方案：**
在 `src/typings/` 目录下创建模块声明文件：
- `whisper-node.d.ts` — 声明 `whisper-node` 模块
- `edge-tts.d.ts` — 声明 `edge-tts` 模块
- `element-plus-locale.d.ts` — 声明 `element-plus/dist/locale/*.mjs` 模块

### B. 插件类型导出缺失 (TS2305 / TS2724)

**涉及文件：**
- `src/renderer/plugins/plugin-loader.ts:10-13` — 缺少 `PluginLoadError`、`PluginValidationError`、`PLUGIN_MANIFEST_FILE`、`PLUGIN_ENTRY_FILE`
- `src/renderer/plugins/plugin-manager.ts:13-15` — 缺少 `PluginEvent`、`PluginError`、`PluginLoadError`
- `src/renderer/plugins/plugin-sandbox.ts:6` — 缺少 `PluginPermissionError`

**修复方案：**
在 `src/renderer/plugins/types.ts` 中补充缺失的导出：
- 添加 `PluginLoadError` 类（继承 Error，包含 pluginId）
- 添加 `PluginValidationError` 类（继承 Error，包含 errors 数组）
- 添加 `PluginPermissionError` 类（继承 Error，包含 pluginId）
- 添加 `PluginEvent` 类型（事件联合类型）
- 添加 `PluginError` 类型（错误联合类型）
- 添加 `PLUGIN_MANIFEST_FILE` 常量（`'manifest.json'`）
- 添加 `PLUGIN_ENTRY_FILE` 常量（`'index.js'`）

### C. IPC 接口类型不完整 (TS2339)

**涉及文件：**
- `src/preload/index.ts` — `window.electronAPI` 类型声明缺少方法
- `src/renderer/plugins/plugin-loader.ts` — 使用 `checkDirectoryExists`、`checkFileExists`、`readFile`、`getDirectories`
- `src/renderer/plugins/plugin-manager.ts` — 使用 `readFile`、`copyDirectory`、`deleteDirectory`

**修复方案：**
扩展 `src/preload/index.ts` 中的 `electronAPI` 类型声明，添加：
```typescript
checkDirectoryExists: (path: string) => Promise<boolean>;
checkFileExists: (path: string) => Promise<boolean>;
readFile: (path: string) => Promise<string>;
getDirectories: (path: string) => Promise<string[]>;
copyDirectory: (source: string, target: string) => Promise<void>;
deleteDirectory: (path: string) => Promise<void>;
```

### D. Plugin 接口缺少字段 (TS2339)

**涉及文件：**
- `src/renderer/plugins/plugin-manager.ts` — `Plugin` 类型缺少 `permissions`、`main` 字段

**修复方案：**
在 `src/renderer/plugins/types.ts` 的 `Plugin` 接口中添加：
- `permissions?: PluginPermission[]` — 可选权限列表
- `main?: string` — 可选入口文件路径

### E. unknown 类型未断言 (TS18046)

**涉及文件：**
- `src/renderer/services/recording-service.ts` — 大量 `result` 未类型断言
- `src/renderer/services/asr-service.ts` — `result` 未类型断言
- `src/renderer/services/llm-service.ts` — `result` 未类型断言
- `src/renderer/services/tts-service.ts` — `result` 未类型断言
- `src/renderer/stores/recording.ts` — 大量 `result` 未类型断言

**修复方案：**
为 `window.electronAPI.invoke()` 的返回值添加类型断言。每个调用点根据预期返回类型添加 `as` 断言。

### F. 类型不匹配 (TS2322)

**涉及文件：**
- `src/main/modules/gift-sound/sound-queue.ts:75` — `undefined` vs `null`
- `src/main/modules/recording/recording-engine.ts:145` — 对象 vs `RecordingStatus | null`
- `src/renderer/modules/ai/types.ts:93,113` — `"friendly"` 不在联合类型中

**修复方案：**
- `sound-queue.ts`: 将 `shift()` 结果用 `?? null` 转换
- `recording-engine.ts`: 添加显式类型标注
- `ai/types.ts`: 在 `tone` 联合类型中添加 `'friendly'`

### G. 未使用声明 (TS6196 / TS6133)

**涉及文件：**
- `src/main/modules/gift-sound/index.ts` — `PlaySoundResult`
- `src/main/modules/recording/index.ts` — `sessionId`、`reason`
- `src/renderer/plugins/plugin-loader.ts` — `PluginFileInfo`、`PluginLoadError`、`PluginValidationError`、`PLUGIN_ENTRY_FILE`
- `src/renderer/plugins/plugin-manager.ts` — `PluginEvent`、`PluginError`
- `src/renderer/plugins/plugin-sandbox.ts` — `checkPermission`
- `src/renderer/stores/recording.ts` — `RecordingStatus`

**修复方案：**
- 删除未使用的导入
- 对于有副作用的导入（如类），保留但确保被使用
- 删除未使用的变量声明

### H. 回调参数类型不兼容 (TS2345)

**涉及文件：**
- `src/renderer/stores/recording.ts:296,306,312` — `on` 回调参数类型为 `unknown`

**修复方案：**
在 `on` 方法的回调处添加类型断言，或修改 `on` 方法签名支持泛型。

## 验收标准

- `npm run type-check` 输出 exit code 0
- 不引入新的 lint 错误
- 不改变运行时行为
