# 计划：深色主题提醒页文字对比度

## 目标与范围

在不改变 DOM、文案、业务状态或交互的前提下，为三个页面和礼物音效表格的现有浅色文字/分隔线补齐深色 Tailwind 变体。`product-content delta: none`。

## 任务流

```mermaid
flowchart LR
  T1[契约测试 RED] --> T2[四个模板补齐 dark utilities]
  T2 --> T3[测试与三路由验证]
```

## T1 契约测试（RED）

- 来源：spec 验收 1-2，U1-U3，`product-content delta: none`。
- 文件：`src/renderer/components/media/media-pages-contract.test.ts`。
- 实施：读取 `captain-effect/index.vue`、`gift-reminder/index.vue`、`gift-sound/index.vue`、`GiftSoundTable.vue`；对每行进行静态契约断言：
  - 含 `text-gray-*` 的行必须同时含 `dark:text-*`。
  - 含 `border-gray-200` 的行必须同时含 `dark:border-zinc-800`。
  - 含 `divide-gray-200` 的行必须同时含 `dark:divide-zinc-800`。
- 预期：修复前测试因当前未配对 class 失败。
- 不改：现有媒体功能契约断言。

## T2 模板修复（GREEN）

- 来源：spec UI/UX 层级、样式约束与验收 1-5，`product-content delta: none`。
- 文件：
  - `views/captain-effect/index.vue`
  - `views/gift-reminder/index.vue`
  - `views/gift-sound/index.vue`
  - `components/media/GiftSoundTable.vue`
- 精确规则：
  - `text-gray-900` -> 增加 `dark:text-zinc-100`。
  - `text-gray-800` -> 增加 `dark:text-zinc-200`。
  - `text-gray-700` / `text-gray-600` -> 增加 `dark:text-zinc-300`。
  - `text-gray-500` -> 增加 `dark:text-zinc-400`。
  - `text-red-600` -> 增加 `dark:text-red-400`。
  - `border-gray-200` -> 增加 `dark:border-zinc-800`；`divide-gray-200` -> 增加 `dark:divide-zinc-800`。
- 保留：所有现有浅色 utility、DOM、Element Plus 组件、状态与事件绑定。
- 抽象边界：Level 0 `direct code`，不新建 CSS、helper、class map、wrapper 或组件。
- 变更链路：`.dark` -> Tailwind 变体 -> 现有 DOM 文字；不触及 store/IPC/audio 副作。

## T3 验证与评审

- 来源：spec 验收 3-6 与自愈标准。
- 自动化：
  - `pnpm --filter @monorepo/bilibili-live-tool exec vitest run --mode free src/renderer/components/media/media-pages-contract.test.ts`
  - 相关深色页契约测试。
  - `pnpm --filter @monorepo/bilibili-live-tool type-check`
  - `pnpm --filter @monorepo/bilibili-live-tool lint:check`
  - `pnpm --filter @monorepo/bilibili-live-tool build:web:free`
- 运行时：打开 `#/captain-effect`、`#/gift-reminder`、`#/gift-sound`，在深色主题检查主/次文字 computed color、分隔线与布局；若主机无法切换浅色，以 class/token 契约保护浅色分支。
- 链路复查：`rg` 确认目标文件无未配对常规 gray text/border；`git diff --check` 通过。

## 生产代码质量合同

- 无类型、数据、null、错误或函数变更。
- 不引入缓存、watcher、监听、定时器、请求或资源清理。
- 保持类名内联可审查，不用常量隐藏 utility。

## 代码评审预检合同

- 健壮性：无外部数据/API 变更；原 loading/error/empty 保留。
- 可维护性：不增加函数或抽象；每个 class hunk 映射到明确文字层级。
- 性能/内存：静态 utility 无运行时开销或清理义务，明确不做优化。
- 项目规范：Vue 3 + Tailwind utilities + Element Plus + Vitest，不改技术栈。
- 业务可见忠实性：主、次、错误颜色层级符合 spec，不替换字段语义或交互态。

## 人工评审可通过性

只审查本请求的四个 Vue 文件、一个契约测试和文档。现有 dirty worktree 中的其他修改不得回退、重排或声称为本任务成果。执行结束前检查无无关格式化、debug/dead/stale 代码或未适配测试。

## 执行就绪判定

- 文字层级、文件、精确 class 映射、测试断言和验证命令均已明确。
- 无需 execute 推测的产品、数据、交互、错误或 API 语义。
- blocker：none。

