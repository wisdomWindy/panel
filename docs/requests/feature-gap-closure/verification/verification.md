# 功能缺口收口 - 验证记录

## acceptance coverage

| acceptance item id | verification method | result | evidence reference | follow-up if failed | handoff status |
|---|---|---|---|---|---|
| AC-01 | 代码检查 + 构建验证 | pass | `src/renderer/views/plugin-manager/index.vue` 已在 `onMounted` 中调用 `ensurePluginManagerInitialized()` 与 `loadPlugins()`；`rtk npm run build:vite` 通过 | 无 | ready |
| AC-02 | 代码检查 + 构建验证 | pass | `src/renderer/views/plugin-manager/index.vue` 中 `togglePlugin` / `uninstallPlugin` / `loadPlugins` 均调用真实 plugin manager；`rtk npm run build:vite` 通过 | 无 | ready |
| AC-03 | 自动化测试 + 代码检查 | pass | `rtk node --test tests/plugin-runtime.test.ts ...` 通过；`tests/plugin-runtime.test.ts` 覆盖 manifest id 安装目录语义；`src/renderer/plugins/plugin-manager.ts` / `plugin-loader.ts` 已按外部路径安装 | 无 | ready |
| AC-04 | 代码检查 | pass | `src/main/ipc-handlers.ts` 已注册 `fs:checkDirectoryExists` / `fs:checkFileExists` / `fs:readFile` / `fs:getDirectories` / `fs:copyDirectory` / `fs:deleteDirectory` | 无 | ready |
| AC-05 | 自动化测试 + 代码检查 | pass | `rtk node --test tests/settings-config-io.test.ts ...` 通过；`src/renderer/views/settings/utils/config-io.ts` 与 `src/renderer/views/settings/index.vue` 已覆盖 core / recording / tts / asr / ai / moderation | 无 | ready |
| AC-06 | 自动化测试 + 代码检查 | pass | `rtk node --test tests/settings-config-io.test.ts ...` 通过；`src/renderer/views/settings/index.vue` 中 `confirmImport()` 已将导入结果真实写回各 store，并同步 `minSendInterval` 运行态 | 无 | ready |
| AC-07 | 自动化测试 + 代码检查 | pass | `rtk node --test tests/recording-context.test.ts ...` 通过；`src/renderer/services/danmaku-service.ts` 中 `handleAutoRecordStart()` 已调用 `recordingStore.startRecording()` | 无 | ready |
| AC-08 | 自动化测试 + 代码检查 | pass | `rtk node --test tests/recording-context.test.ts ...` 通过；`src/renderer/views/recording/index.vue` 使用 `roomStore.roomInfo.roomId` 构造录制上下文，不再使用 `default` | 无 | ready |
| AC-09 | 代码检查 + 构建验证 | pass | `src/renderer/App.vue`、`src/renderer/views/dashboard/index.vue`、`src/renderer/views/settings/index.vue`、`src/renderer/views/settings/components/LanguageSettings.vue` 已接入 `t()`；`rtk npm run build:vite` 通过 | 无 | ready |
| AC-10 | 代码检查 + 构建验证 | pass | `src/renderer/App.vue` 使用根级 `<el-config-provider :locale="elementPlusLocale">`；`src/renderer/i18n-runtime.ts` 提供响应式 locale；`rtk npm run build:vite` 通过 | 无 | ready |
| AC-11 | 测试记录检查 | pass | 本次新增 `tests/plugin-runtime.test.ts`、`tests/settings-config-io.test.ts`、`tests/recording-context.test.ts`；执行 `rtk node --test ...` 结果为 `pass 7 fail 0`；执行记录见 `execution/changelog.md` | 无 | ready |

## spec constraint compliance

- result: pass
- checked constraints:
  - 页面与底层运行时边界已分离：插件页面仅做交互编排，插件单例运行时集中在 `src/renderer/plugins/runtime/index.ts`
  - 数据归一与导入写回集中在适配层：设置导入导出逻辑集中在 `src/renderer/views/settings/utils/config-io.ts` 与 `settings/index.vue`
  - 录制上下文命名与真值来源一致：统一使用 `roomId` / `roomName`，无 `default` 伪值
  - 语言切换副作用集中：`src/renderer/i18n-runtime.ts` 作为应用语言与 Element Plus locale 的单一来源
  - 未引入额外重型模式层：仅新增轻量单例运行时与小型 helper / adapter
- evidence reference:
  - `src/renderer/plugins/runtime/index.ts`
  - `src/renderer/views/settings/utils/config-io.ts`
  - `src/renderer/views/recording/utils/recording-context.ts`
  - `src/renderer/i18n-runtime.ts`
  - `rtk node --test tests/plugin-runtime.test.ts tests/settings-config-io.test.ts tests/recording-context.test.ts`
  - `rtk npm run type-check`
  - `rtk npm run build:vite`
- follow-up if failed: 无

## summary

- 新鲜验证命令：
  - `rtk node --test tests/plugin-runtime.test.ts tests/settings-config-io.test.ts tests/recording-context.test.ts`
  - `rtk npm run type-check`
  - `rtk npm run build:vite`
- 结果：
  - 测试通过，`pass 7 fail 0`
  - `type-check` 通过
  - `build:vite` 通过
- 非阻塞遗留：
  - `build:vite` 仍有大 chunk 体积告警，但不影响本次功能闭环验收
