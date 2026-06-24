# 请求：修复项目 TypeScript 类型错误

## 来源

- 来源系统：本地项目 typecheck
- 触发方式：运行 `npm run type-check` 发现大量类型错误

## 问题摘要

项目运行 `tsc --noEmit -p tsconfig.eslint.json` 时报 100+ 个类型错误，涉及 16 个文件。主要错误类型包括：

1. **模块声明缺失** (TS2307/TS7016)：`whisper-node`、`edge-tts`、`element-plus/dist/locale/*.mjs` 缺少类型声明
2. **导出成员缺失** (TS2305/TS2724)：`plugin/types.ts` 中缺少 `PluginLoadError`、`PluginValidationError`、`PluginEvent`、`PluginError`、`PluginPermissionError`、`PLUGIN_MANIFEST_FILE`、`PLUGIN_ENTRY_FILE` 等导出
3. **属性不存在** (TS2339)：IPC 接口类型不完整，缺少 `checkDirectoryExists`、`checkFileExists`、`readFile`、`getDirectories`、`copyDirectory`、`deleteDirectory` 等方法；`Plugin` 类型缺少 `permissions`、`main` 属性
4. **unknown 类型未断言** (TS18046)：`window.electron.invoke()` 返回 `unknown`，大量地方未做类型断言
5. **类型不匹配** (TS2322)：`undefined` vs `null`、字面量类型不兼容等
6. **未使用声明** (TS6196/TS6133)：导入了但未使用的类型和变量
7. **回调参数类型不兼容** (TS2345)：`on` 方法的回调参数类型为 `unknown`，与具体类型不兼容

## 目标

修复所有 TypeScript 类型错误，使 `npm run type-check` 通过。

## 完成信号

- `npm run type-check` 输出 `exit code 0`，无任何错误
- 不引入新的 lint 错误
- 不改变运行时行为

## 影响范围

- 主进程模块：`asr/index.ts`, `gift-sound/index.ts`, `gift-sound/sound-queue.ts`, `recording/index.ts`, `recording/recording-engine.ts`, `tts/index.ts`
- 渲染进程模块：`main.ts`, `plugins/*`, `services/*`, `stores/*`
- 类型定义：`modules/ai/types.ts`
