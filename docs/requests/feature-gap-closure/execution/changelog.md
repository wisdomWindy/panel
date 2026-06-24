# 功能缺口收口 - 执行记录

## 执行概览

- 当前阶段：`execute`
- 执行顺序：`T1 -> T2 -> T3 -> T4 -> T5`
- TDD 说明：
  - 对可抽离的纯逻辑与服务层行为执行 red -> green -> refactor
  - 对依赖 Electron 运行时的页面链路，使用静态验证、构建验证与局部命令验证补充证据

## T1：补齐插件系统底层运行时与文件系统 IPC

- 状态：已完成
- TDD 记录：
  1. 先新增 `tests/plugin-runtime.test.ts`，验证插件安装目标目录语义与运行时初始状态
  2. 初次执行 `rtk node --test ...` 失败，原因是 helper 尚不存在
  3. 补充 `src/renderer/plugins/runtime/helpers.ts` 后测试通过
- 已完成项：
  - 新增 `src/renderer/plugins/runtime/index.ts` 提供插件管理器单例入口
  - 修正 `plugin-loader.ts`，区分“按已安装目录 ID 加载”和“按外部路径安装加载”
  - 修正 `plugin-manager.ts`，安装时使用 manifest id 作为安装目标目录
  - 在 `src/main/ipc-handlers.ts` 中补齐 `fs:*` 文件系统 IPC
  - 插件管理页面已接入真实 manager 的加载、启停、卸载、安装链路

## T2：接通插件管理页面真实交互链路

- 状态：已完成

## T3：收口设置页配置导入导出

- 状态：进行中
- TDD 记录：
  1. 先新增 `tests/settings-config-io.test.ts`
  2. 初次执行失败，原因是导入导出 helper 尚不存在
  3. 补充 `src/renderer/views/settings/utils/config-io.ts` 后测试通过
- 已完成项：
  - 设置页导出范围扩展到 core / recording / tts / asr / ai / moderation
  - 设置页导入已改为真实写回相关 store
  - `minSendInterval` 导入后会同步更新 danmaku service 运行时状态

## T4：收口录制上下文与自动录制

- 状态：进行中
- TDD 记录：
  1. 先新增 `tests/recording-context.test.ts`
  2. 初次执行失败，原因是录制上下文 helper 尚不存在
  3. 补充 `src/renderer/views/recording/utils/recording-context.ts` 后测试通过
- 已完成项：
  - 录制页手动开始录制改为从当前 room context 构造参数
  - `danmaku-service.ts` 已接入自动录制真实启动逻辑

## T5：收口多语言与 Element Plus locale 同步

- 状态：已完成
- 已完成项：
  - 新增 `src/renderer/i18n-runtime.ts` 统一应用语言与 Element Plus locale 来源
  - `src/renderer/main.ts` 改为通过响应式 locale 注入 Element Plus 国际化
  - `App.vue`、`dashboard/index.vue`、`settings/index.vue`、`LanguageSettings.vue` 已接入目标 i18n 文案
  - 补充中英文 locale 文案键

## 执行中补充收口：历史构建阻塞修复

- 状态：进行中
- 已定位问题：
  - `src/renderer/views/settings/components/TTSSettings.vue` 仍存在历史残留的错误相对路径，导致 `build:vite` 无法解析 `modules/tts/types`
  - `vite.config.ts` 将 renderer 侧 `events` 映射到 `src/renderer/modules/utils/event-emitter.ts`，但该适配文件缺失，导致弹幕相关模块构建失败
- 已处理项：
  - 将 `TTSSettings.vue` 中 `EDGE_TTS_VOICES` 的导入修正为 `../../../modules/tts/types`
  - 新增 `src/renderer/modules/utils/event-emitter.ts`，为 renderer 侧 `BilibiliClient` / `DanmakuManager` / `BlacklistManager` 提供轻量事件实现
  - 将 Element Plus locale 同步切换到根级 `el-config-provider` 响应式注入，修复运行时语言切换只在启动期生效的问题

## T6：统一验证与工件回填

- 状态：已完成
- 验证命令与结果：
  1. `rtk node --test tests/plugin-runtime.test.ts tests/settings-config-io.test.ts tests/recording-context.test.ts` -> `pass 7 / fail 0`
  2. `rtk npm run type-check` -> 通过
  3. `rtk npm run build:vite` -> 通过，仅剩大 chunk 非阻塞告警
- 产物回填：
  - 已新增 `verification/verification.md`
  - 已新增 `review/review.md`
