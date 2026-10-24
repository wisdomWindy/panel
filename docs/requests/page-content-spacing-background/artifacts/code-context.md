# Code Context

## context requirement

- 需要确定五个点名页面与“其他页面”的共享布局 owner，避免逐页复制样式。
- 首次需求阶段：bugfix-intake。
- code graph 仅为 preferred；直接入口和 owner 可由路由与 `App.vue` 明确确认。

## graph availability check

- status: missing。
- detection: 搜索仓库脚本/文档中的 code graph bootstrap，并检查 PATH 中 `codegraph` / `code-graph` / `scip` / `lsif`。
- tool/runtime found: none。
- context: loop iteration 1。

## installation or bootstrap record

- attempted: yes（自动发现仓库/团队首选 bootstrap）。
- method: 仓库配置、脚本与 PATH 发现。
- result: fail；未找到可用工具或项目 bootstrap 入口，不自行引入未知依赖。
- next step: 使用定向文本搜索和直接调用链阅读。

## fallback record

- used: yes。
- method: `rg`、直接读取 `App.vue` / router / page roots，以及已有 shell 测试。
- reason: graph capability unavailable。
- confidence: high；所有目标页面均位于同一 `<router-view>`，且背景/边距由单一 `<main>` 直接拥有。

## relevant entrypoints

- user-facing: 左侧导航进入各路由页面。
- internal: `App.vue` 的 shell grid -> flex content row -> `<main>` -> `<router-view>`。
- route source: `src/renderer/router/index.ts`。

## functional chain under change

用户打开路由 -> `App.vue` 渲染共享 `<main>` -> `<router-view>` 渲染页面 root surface。当前 `<main>` 使用 `bg-zinc-100 p-3 sm:p-5`，页面多为 `bg-white`，因而 padding 和未覆盖高度呈现为灰色。修复只归属 shell 背景 token；页面 store、事件、异步请求和子组件均不变。

测试适配目标：`shell-components-contract.test.ts`。邻近 dark-theme page tests 保持不变。修改后重查 shell -> main -> route 链路。

## key symbols and modules

- `App.vue` template root grid and `<main>` class。
- `router/index.ts` routes（只读）。
- `views/*/index.vue` root surfaces（只读影响面）。
- `shell-components-contract.test.ts` shell style contract。

## dependency and side-effect boundaries

本变更无状态写入、订阅、事件、API 或路由副作用。Tailwind utility 由本地样式入口编译。

## impact scope

- likely production change: `App.vue` 一处共享 class。
- test change: shell contract 增加背景/间距回归断言。
- verification: focused Vitest、type-check、lint、Web build、1280x800 桌面截图，必要时检查多路由。

## open follow-up checks

- 运行时确认浅色主内容背景是白色，且 padding 生效。
- 确认深色主题 `dark:bg-zinc-950` 未变。

