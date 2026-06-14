# 全局布局重设计

## 请求标识

- **request-id**: global-layout-redesign
- **创建时间**: 2026-06-12

## 来源

用户直接提出，无外部 PRD 文档。

## 业务摘要

在既有全局布局重设计基础上，修复项目中 Tailwind 样式未稳定生效的问题，并新增统一样式约束：`src/renderer/**/*.vue` 内禁止使用 `<style>` 标签、内联样式、行内 `style` 属性与 `:style` 动态样式，页面样式仅允许通过 Tailwind CSS 风格的 class 表达。随后追加处理构建质量问题，清理 Rollup 对 `@vueuse/core` 的第三方注释告警，将 Element Plus 从手写组件入口改为构建期自动按需加载，并将渲染端大体积依赖按实际边界拆分，避免无意义的整包入口膨胀。最新补充要求是消除 VS Code / Vue 语言服务中的 ts-plugin 报错和告警，确保自动生成声明文件被 IDE 稳定识别；并统一渲染端 Vue SFC 的块顺序，要求 `template` 位于 `script` 之前。进一步补充的是修正 `src/renderer/views/DanmakuPool.vue` 表格中 `el-input-number` 的 ts-plugin 诊断，避免对插槽行数据的 `v-model` 绑定继续触发类型告警。

## 影响范围

- `src/renderer/main.ts` — 渲染端样式入口
- `src/renderer/index.html` — HTML 样式注入方式
- `src/renderer/styles/tailwind.css` — 本地 Tailwind 样式入口
- `tailwind.config.js` — Tailwind 内容扫描范围
- `vite.config.ts` — 构建分包与告警处理
- `auto-imports.d.ts` — Element Plus 服务类自动导入声明
- `components.d.ts` — Element Plus 组件自动导入声明
- `src/vue-shims.d.ts` — Vue / TS 声明入口
- `src/renderer/**/*.vue` — 统一 SFC 块顺序
- `src/renderer/services/danmaku-service.ts` — 特效模块改为按需加载
- `src/renderer/modules/effects/webgl-renderer.ts` — Three.js 导入边界收敛
- `src/renderer/App.vue` — 主框架布局
- `src/renderer/components/StatCard.vue` — 统计卡片
- `src/renderer/**/*.vue` — 所有渲染端 Vue 组件样式约束

## 参与模块

- Tailwind 样式供应链（Vite/Electron 渲染端入口）
- 构建分包与第三方告警治理
- Element Plus 自动按需加载链路
- 全局布局 (App.vue: 侧边栏、页头、主内容区)
- 统计卡片组件
- 各页面卡片、表格、表单间距
- 渲染端 Vue 组件样式约束治理
