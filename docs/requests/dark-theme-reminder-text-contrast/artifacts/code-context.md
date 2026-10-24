# 代码上下文

## 上下文需求

- 需要理解三个路由页和一个表格子组件的样式归属。
- 首次需要阶段：`bugfix-intake`。
- 代码图仅为可选；样式类及组件引用关系是局部且明确的。

## 代码图可用性检查

- 状态：`missing`。
- 检测方法：当前工具与仓库无代码图入口。
- 检查轮次：bugfix intake iteration 1。

## 安装或引导记录

- attempted: no
- method: not applicable
- result: `not_needed`
- summary: `code-graph.md` 允许对已知、隔离的样式缺陷使用文本检索。
- next step: 使用 `rg` 和直接源码阅读完成影响闭包。

## 降级记录

- fallback used: yes
- method: `rg` 类名搜索、直接读取 Vue 模板、读取现有契约测试。
- reason: 目标是已知页面中的 Tailwind 类，无状态、调用或副作传播。
- confidence: high；路由页、子组件和测试入口均已直接定位。

## 相关入口

- `/captain-effect` -> `views/captain-effect/index.vue`
- `/gift-reminder` -> `views/gift-reminder/index.vue`
- `/gift-sound` -> `views/gift-sound/index.vue` -> `components/media/GiftSoundTable.vue`

## 变更中的功能链

- 触发：根节点 `.dark` 存在，Tailwind `dark:` 变体命中。
- 组件链：`App.vue` shell -> router view -> 三个页面 -> `GiftSoundTable.vue` 子组件。
- 当前失败链路：目标文字仅有 `text-gray-*`，无 `dark:text-*`；分隔线仅有 `border-gray-200`。
- 修复所有权：直接在现有页面/子组件的内联 utility 中补充深色变体。
- 相邻链路：Element Plus 表单标签和控件已由现有主题变量控制，不在本次修改范围。
- 测试适配目标：`media-pages-contract.test.ts`。
- 变更后检查：类名契约、三路由 DOM/视觉检查、type/lint/build。

## 关键符号与模块

无 JavaScript/TypeScript 符号变更；仅 Vue template class 与测试源文本契约。

## 依赖与副作边界

不触及 store、IPC、文件选择、音频播放、保存或预览副作。

## 影响范围

预计修改四个 Vue 模板和一个现有测试文件。回归敏感区是浅色颜色分支、深色文字层级和礼物音效表格。

## 开放后续检查

实施后确认三页无剩余未配对的常规 `text-gray-*` / `border-gray-200` 类。

