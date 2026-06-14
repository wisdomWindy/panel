# 实施计划

## 任务分解

### T1: 修复 Tailwind 本地接入

- 新增 `src/renderer/styles/tailwind.css`
- 在 `src/renderer/main.ts` 中导入本地 Tailwind 样式入口
- 移除 `src/renderer/index.html` 中的 Tailwind CDN 脚本
- 修正 `tailwind.config.js` 的内容扫描范围

### T2: 样式约束巡检与最小化治理

- 全量检查 `src/renderer/**/*.vue` 是否存在 `<style>`、`style=`、`:style=`、`v-bind:style`
- 若存在残留，改为 Tailwind class 表达
- 若不存在残留，仅保留巡检结果并不做无意义模板重写

### T3: 工件与执行记录同步

- 更新 `request.md`、`design/page-design.md`、`spec/spec.md`、`spec/clarifications.md`
- 更新 `plan/task-board.md` 与 `execution/changelog.md`

### T4: 验证与评审闭环

- 运行构建验证 Tailwind 本地接入生效
- 运行搜索验证 Vue 组件中无禁用样式写法
- 写回 `verification/verification.md` 与 `review/review.md`

### T5: 构建告警治理

- 将 `@vueuse/core` 注释告警限制为精准过滤，不影响其他 Rollup 告警
- 将 Element Plus 使用边界从手写组件入口收敛到 Vite 插件驱动的自动按需加载
- 将 `danmaku-service` 对特效系统改为按需加载
- 将 `three` 改为源码级模块导入，并保持独立异步 vendor chunk

### T6: Element Plus 自动按需加载改造

- 为 Vite 接入 `unplugin-vue-components` 与 `unplugin-auto-import`
- 使用 `ElementPlusResolver` 自动解析模板组件、服务类 API 与对应样式
- 移除页面和 service 中手写的 `element-plus/es/components/*` 导入
- 将自动生成的声明文件纳入 TypeScript 检查范围

### T7: ts-plugin 兼容性修正

- 将自动生成声明文件从 `src/` 调整到工作区根目录
- 通过 `tsconfig` 与声明入口文件显式接入 `auto-imports.d.ts`、`components.d.ts`
- 以命令行类型检查和 IDE 兼容性为双重目标验证 `ElMessage`、`ElMessageBox` 与 `<el-*>` 解析

### T8: Vue SFC 块顺序统一

- 全量检查 `src/renderer/**/*.vue` 的 `template` 与 `script` 块顺序
- 将所有 `script` 在前的 SFC 调整为 `template` 在前、`script` 在后
- 保持模板内容、脚本内容和组件行为不变，仅做块级重排

### T9: DanmakuPool 行输入 ts-plugin 修正

- 检查 `DanmakuPool.vue` 中 `el-input-number` 对 `row.interval` 的绑定方式
- 改为显式 `:model-value` / `@update:model-value` 写法，规避对可选字段的 `v-model` 诊断
- 保持界面展示为秒、数据写回为毫秒的既有行为

## 依赖关系

- T1 必须先完成，否则无法判断页面样式是否真正恢复
- T2 依赖 T1 的样式链路修复结果
- T3 可与 T1/T2 同步推进，但最终内容要以实际代码结果为准
- T4 依赖 T1-T3 完成
- T6 依赖现有 Element Plus 使用点盘点完成，并与 T5 协同验证
- T8 依赖受影响 SFC 盘点完成，可在不变更逻辑的前提下批量执行
- T9 依赖 `DanmakuPool.vue` 当前行数据结构与单位换算逻辑确认完成

## 并行执行建议

- 文档工件更新与代码巡检可并行
- 构建验证与最终 review 必须串行，在全部代码修改完成后执行

## 受影响文件或模块

- `src/renderer/main.ts`
- `src/renderer/index.html`
- `src/renderer/styles/tailwind.css`
- `tailwind.config.js`
- `src/renderer/**/*.vue`
- `src/renderer/modules/effects/webgl-renderer.ts`
- `src/renderer/services/danmaku-service.ts`
- `auto-imports.d.ts`
- `components.d.ts`
- `src/vue-shims.d.ts`
- `vite.config.ts`
- `docs/requests/global-layout-redesign/**`

## 测试策略

- 本次主要是样式供应链修复与模板约束治理，不涉及可独立编写单元测试的业务行为
- 使用 `npm run build:vite` 作为主验证手段，确认渲染端构建通过
- 使用全量搜索验证 `element-plus/es/components` 手写导入已移除
- 使用全量搜索作为约束验证，确认 Vue 组件中无禁用样式写法
- 检查根目录声明文件已生成并被 `tsconfig` / Vue 声明入口显式纳入
- 使用全量搜索验证 `src/renderer/**/*.vue` 中不存在 `script` 早于 `template` 的文件
- 重点检查 `DanmakuPool.vue` 数值输入的模板绑定与 `type-check` / `build` 结果

## 回滚说明

- 如本地 Tailwind 接入导致构建异常，可回滚 `main.ts`、`tailwind.css`、`tailwind.config.js`、`index.html` 的本次改动
- 如页面视觉出现回退，优先检查扫描路径与入口导入，而不是恢复页面内联样式
