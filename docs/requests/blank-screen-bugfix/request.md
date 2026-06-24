# Bugfix Request: 应用启动后界面空白

**请求日期**: 2026-06-12

**缺陷描述**: 应用启动后渲染进程显示空白页面，Vue 应用未能挂载。

**根因**: `danmaku-service.ts` 在模块顶层立即实例化 `DanmakuService` 单例，其构造函数调用 `useRoomStore()` 和 `useConfigStore()` 时 Pinia 尚未初始化（Pinia 在 `main.ts` 中通过 `createPinia()` + `app.use(pinia)` 才激活），触发 `getActivePinia()` 异常，导致整个模块依赖链加载失败，Vue 应用无法挂载到 `#app`。

## 完成信号

- 应用不再因 Pinia 初始化时序问题导致空白页
- 渲染进程构建通过
- 修复保持在惰性初始化与调用点更新范围内，不引入额外行为变更
