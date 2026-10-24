# 实施计划：四个页面深色表面一致性

## 交付目标与范围

按 approved spec A1-A6 修复 `danmaku-list`、`dashboard`、`sensitive-words`、`blacklist` 和 dashboard 专用 `TrendChart` 的暗色表现。Product-content delta：none；只增加既有元素的主题视觉状态。

## 前置合同

- Governing TS：`tsconfig.eslint.json` extends `tsconfig.json`；strict、bundler resolution、Vue plugin 与 `@/*` alias 已确认。
- Direct declarations：`src/vue-shims.d.ts`、`src/env.d.ts` 已读取；不新增/修改类型。
- Theme owner：根 `.dark` + Tailwind custom variant；页面只消费派生状态。
- 当前 SFC 含用户未提交 i18n 改动；只对现有 class 做定点追加，禁止重排 script 或宽泛格式化。

## TDD 与任务顺序

### T1：建立 RED 主题契约测试

- Source：A1-A4，source-backed U1-U5；product-content delta none。
- 新建 `src/renderer/views/dark-theme-page-surfaces.test.ts`，通过 Vue SSR 渲染真实 SFC 而不是读取/grep 源文件；在 `vite.config.ts` 中仅为 Vitest 内联 `element-plus`，使包内 CSS 由 Vite 管线处理。
- 读取五个 owning Vue source，断言四页 root/header/title/footer 的深色 class、dashboard 五组指标 class count、danmaku row/正文/时间/关键值及 `TrendChart` border/title/empty class。
- 先运行 `vitest run --mode free src/renderer/views/dark-theme-page-surfaces.test.ts`，先排除测试基础设施错误，再确认断言因当前缺失 `dark:` utility 失败；保存有效失败摘要。

### T2：修复弹幕面板主题层级

- Source：A1/A2/A4；product-content delta none。
- 文件：`views/danmaku-list/index.vue` template only。
- root `dark:bg-zinc-900`；header/footer `dark:border-zinc-800`；title/关键值 `dark:text-zinc-100`；empty/time/footer `dark:text-zinc-400`；event row `dark:border-zinc-800`，正文 `dark:text-zinc-200`。
- 保留现有 router-link 粉色强调、数据过滤、暂停/清屏和 drawer 行为。

### T3：修复数据看板与趋势图主题层级

- Source：A1-A3/A4；product-content delta none。
- `views/dashboard/index.vue`：root/header/title、metrics gap/border、五个 block/label/value、页面 empty 加对应 `dark:` utility。
- `components/live-data/TrendChart.vue`：section border、title、empty text 加深色 utility；ECharts options 和系列颜色不改。
- 运行聚焦测试，若 canvas 运行验证仍显示浅色大块，停止执行并回到 spec/plan，不能临时扩展 chart option 语义。

### T4：修复敏感词与黑名单主题层级

- Source：A1/A2/A4；product-content delta none。
- 两个页面均追加 root/header/title/import result/footer 的深色 utility。
- Element Plus table/dialog/control 保持全局变量 owner，不新增 wrapper 或局部 CSS。
- 不改表格字段、导入导出、校验、增删改和时间格式。

### T5：GREEN、链路复查与静态验证

- 运行聚焦 test 并证明 GREEN。
- 用 `git diff --` 检查五个生产文件和新测试：每个 hunk 映射 T2-T4，无 script/业务变化、无用户 i18n diff 丢失。
- `rg` 复查四个 root 的 `bg-white` 均有相邻 dark variant，dashboard 五组 count 完整，未新增 style block/helper/component。
- 运行全量 Vitest、`type-check`、`lint:check`、`build:web:free`。

### T6：运行时视觉验收

- 启动 `dev:web` 到可用端口。
- 在桌面和窄视口下设置根 `.dark`，依次打开四页并截图/像素检查：无大块白色 surface、无文字重叠、toolbar/table overflow 正常。
- 对 dashboard 检查有统计/空状态条件下的指标和趋势图；检查 canvas/tooltip 可读性。
- 切换浅色，确认 root/指标仍为白色且层级未回归。

## 变更链合同

入口 `.dark` -> Tailwind selector -> owning template utilities -> Element Plus/ECharts 子表面。修改 owner 仅为五个模板；状态、请求、store、IPC 和事件均为邻近影响面且禁止修改。测试文件是适配面，不是生产 owner。完成后以 scoped diff、`rg` 和运行截图证明链路无缺口、重复 owner 或 collateral change。

## Code Review Checklist Contract

- Robustness：无外部输入/异步逻辑新增；现有 empty/loading/error 不删除、不遮挡。
- Maintainability：直接 class 修复，无 helper/constant/wrapper；不制造超长隐藏 class 或宽泛格式化。
- Performance/memory：静态 selectors，无监听、watcher、资源或 cleanup 变化；no optimization required。
- Stack anchoring：Vue 3 + Tailwind utilities + Element Plus；禁止新 CSS 技术、依赖或主题 service。
- Business-visible fidelity：主/辅文字层级、链接状态色和图表系列色保持 spec 语义。

## 回退与自愈

- 聚焦测试失败：分类为 syntax/type 或 logic error，回 `execute` 修正明确 class。
- 视觉验证发现 canvas/tooltip 固定浅色：分类为 requirement/plan gap，回 `spec` 或 `plan` 明确 TrendChart theme option，之后继续同一循环。
- 任何需要共享主题机制、组件 API 或业务逻辑的发现：停止 execute，升级 scope 后回 spec。

## 完成证据

RED/GREEN 输出、全量测试/type/lint/build、scoped diff/chain audit、四页深色与浅色截图、适用 verdict 和 blockers none。
