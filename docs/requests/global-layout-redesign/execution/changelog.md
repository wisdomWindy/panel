# 执行记录

## 2026-06-12

- 回滚并更新 `global-layout-redesign` 请求工件，补充 Tailwind 本地接入和样式约束
- 新增 `src/renderer/styles/tailwind.css`，改为由 `src/renderer/main.ts` 引入本地 Tailwind
- 移除 `src/renderer/index.html` 中的 Tailwind CDN 脚本
- 校正 `tailwind.config.js` 的内容扫描范围到渲染端实际路径
- 复核 `src/renderer/**/*.vue`，确认未残留 `<style>`、`style`、`:style`
- 将 Rollup 对 `@vueuse/core/dist/index.js` 的无害第三方注释告警改为精准过滤
- 将 Dashboard 的 ECharts 改为模块化使用，保留 `echarts` / `zrender` 独立 vendor chunk
- 将 Element Plus 使用方式收敛到组件级入口，避免继续通过总入口放大共享主块
- 新增 `unplugin-vue-components` 与 `unplugin-auto-import`，使用 `ElementPlusResolver` 将 Element Plus 组件、服务和样式切到自动按需加载
- 移除 `src/renderer/**` 与 `src/renderer/services/danmaku-service.ts` 中手写的 `element-plus/es/components/*` 导入
- 移除 `src/renderer/main.ts` 中的 `element-plus/dist/index.css` 全量样式入口，改由按需样式 side effects 注入
- 排查 IDE ts-plugin 诊断差异，确认根因是 `src/` 下自动生成声明在 Vue 语言服务中识别不稳定
- 将自动生成声明文件迁移到项目根目录，并在 `src/vue-shims.d.ts` 中通过三斜线引用显式接入，清理 `src/` 下旧声明副本
- 盘点 `src/renderer/**/*.vue` 中 SFC 块顺序，准备统一为 `template` 在前、`script` 在后
- 批量调整 `src/renderer/**/*.vue` 的 SFC 块顺序，统一为 `template` 在前、`script` 在后，未改动模板与脚本内容本身
- 复核 `DanmakuPool.vue` 行内 `el-input-number` 对 `row.interval` 的绑定方式，准备改为显式单向绑定与写回事件以消除 ts-plugin 诊断
- 将 `DanmakuPool.vue` 表格中的 `row.interval` 输入改为 `:model-value` 与 `@update:model-value`，并通过辅助函数保持秒/毫秒换算逻辑不变
- 将 `DanmakuPool.vue` 操作列的测试按钮从传递整行 `row` 改为仅传递 `row.content`，并将 `handleTest` 参数收窄为 `string`，消除 `DefaultRow` 与业务参数结构不兼容的 ts-plugin 报错
- 将 `danmaku-service` 对特效系统改为动态加载，避免常驻主链路静态拉入特效依赖
- 将 `webgl-renderer.ts` 改为 Three.js 源码级模块导入，并保持 `three` 独立异步 vendor chunk
- 将渲染端 `chunkSizeWarningLimit` 调整为 550，避免对独立异步 Three.js vendor chunk 产生误导性体积告警
