# 任务看板

| 任务 ID | 任务名称 | 状态 | 执行人或执行说明 | 关联规格区域 |
|---|---|---|---|---|
| T1 | 修复 Tailwind 本地接入 | 已完成 | Codex：新增本地样式入口并替换 CDN 接入 | page and module design / AC-1 / AC-2 / AC-3 |
| T2 | 样式约束巡检与最小化治理 | 已完成 | Codex：全量检查 `src/renderer/**/*.vue`，无禁用样式写法残留 | in scope / AC-4 |
| T3 | 工件与执行记录同步 | 已完成 | Codex：回滚完成态请求并同步设计规格计划 | background and goals / acceptance criteria |
| T4 | 验证与评审闭环 | 已完成 | Codex：完成构建验证、搜索验证与评审写回 | AC-5 / AC-6 |
| T5 | 构建告警治理 | 已完成 | Codex：清理 `@vueuse/core` 注释告警，收敛 Element Plus / 特效 / Three.js 依赖边界 | AC-7 / AC-8 |
| T6 | Element Plus 自动按需加载改造 | 已完成 | Codex：接入 Vite 自动解析插件并移除手写组件入口导入 | AC-9 |
| T7 | ts-plugin 兼容性修正 | 已完成 | Codex：调整声明文件输出与显式引用方式，消除 IDE ts-plugin 诊断 | AC-10 |
| T8 | Vue SFC 块顺序统一 | 已完成 | Codex：将渲染端 Vue 文件统一调整为 `template` 在前、`script` 在后 | AC-11 |
| T9 | DanmakuPool 行输入 ts-plugin 修正 | 已完成 | Codex：修正表格 `el-input-number` 可选字段绑定方式，并收窄操作列测试按钮的插槽行参数传递 | AC-12 |
