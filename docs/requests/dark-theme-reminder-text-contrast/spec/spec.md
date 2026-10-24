# 规格：深色主题提醒页文字对比度

## 交付单元标识

`dark-theme-reminder-text-contrast`

## 背景与目标

三个提醒配置页面的部分浅色文字和分隔线没有深色变体。目标是使页标题、描述、分区标题、行标签、统计文字及表格文件名在深色背景下可读，并保持浅色主题和交互不变。

## 范围内

- 为 `captain-effect`、`gift-reminder`、`gift-sound` 页中现有 `text-gray-*` 文字补充匹配的 `dark:text-*`。
- 为三页及 `GiftSoundTable.vue` 的现有分隔线补充 `dark:border-zinc-800` / `dark:divide-zinc-800`。
- 为深色主题中的缺失文件错误文字补充 `dark:text-red-400`。
- 扩展现有媒体页面契约测试。

## 范围外

布局、文案、表单值、控件、路由、响应式结构、store、IPC、Rust、音频播放和保存逻辑。

## 触发与启动条件

根节点存在 `.dark` 且用户访问三个目标路由之一。

## 需求拆分摘要

不适用；这是 `S1 local` 非 PRD 缺陷，以请求作为单一交付单元。

## 来源锁定与可追溯性

| 规格项 | 标签 | 来源 | 下游义务 |
| --- | --- | --- | --- |
| 上舰特效红框文字深色可读 | source-backed | U1/U3 | 补齐标题、分区、行标签变体 |
| 礼物提醒和礼物音效同步修复 | source-backed | U2 | 遍历同类文字与分隔线 |
| 沿用 `zinc-100/200/300/400/800` 层级 | code-fact-backed | 已有深色页面规则 | 保持主次可读且不新建设计系统 |
| 不改变浅色和功能 | source-derived | U1-U3 的缺陷边界 | 仅添加 `dark:` utility |

闭世界允许列表仅包含上述文字、错误文字和分隔线的深色颜色变体，以及回归测试。

## 工作流效率配置

- 档位：`S1 local`。
- 密度：compact。
- 上下文：scoped，仅三页、子表格和相关契约测试。
- 升级条件：如果修复需要改变主题投影、Element Plus 变量或组件边界，回退重新评估。

## 自愈完成标准

- 需求一致：三页所有目标文字都有深色变体，不只修截图中的几处。
- 逻辑/运行时：无业务逻辑、状态或副作变更；路由仍可正常渲染。
- 语法/工程：聚焦测试、type-check、lint、Web build 通过。
- 组织/可维护：类名内联可审查，无新抽象、无无关 diff。
- 完成要求：`self-healing loop compliance: pass`、`self-healing loop assessment: pass`，剩余 blocker 为 none。

## 用户意图合同

- literal：修复三页深色主题下看不清的内容。
- practical：三页同类文字与层级都稳定可读。
- success：主文字、次文字、错误文字和分隔线在深色下可分辨，浅色不变。
- forbidden：只修红框不修同页同类文字；改布局、文案、数据或交互；使用全局 CSS 强制覆盖。
- acceptable：在现有 utility 上就地增加 `dark:` 变体并增加契约测试。

## 用户流程

进入目标路由 -> 主题类命中 -> 页面以现有布局和状态显示；本修复仅改变深色主题颜色结果。

## 页面与模块设计

页面归属和 `GiftSoundTable` 边界不变。不需要 page-design：无布局、信息架构、交互或响应式变更。

## UI / UX 设计合同

主标题在深色下使用 `zinc-100`；分区和关键正文使用 `zinc-200/300`；说明、未选择文件和统计辅助信息使用 `zinc-400`；分隔线使用 `zinc-800`；错误使用 `red-400`。

## 前端样式约束

仅使用现有 Tailwind utility；`dark:` 与原类保持在同一模板 `class` 或小型条件 class 中。不新增 scoped CSS、内联 style、语义 class、constant/computed class 映射。

## 字段展示语义与视觉状态

- 标题/数值：表示页面或关键结果，深色默认为 `zinc-100`。
- 分区/行标签/开关标签：表示配置语义，深色默认为 `zinc-200/300`。
- 描述/文件名/计数/队列提示：是次要信息，深色默认为 `zinc-400`。
- 缺失文件：现有错误语义不变，深色为 `red-400`。
- 可点击性、hover/focus/active/disabled 均不变。

## 功能完整行为分解

1. 契约测试对四个源文件逐行检查：出现 `text-gray-*` 的行必须同时包含 `dark:text-*`。
2. 契约测试逐行检查：出现 `border-gray-200` 的行必须同时包含 `dark:border-zinc-800`；含 `divide-gray-200` 的行还必须包含 `dark:divide-zinc-800`。
3. 生产模板按上述层级补充 utility，不改动 DOM 结构和逻辑。
4. 三个路由在深色主题下运行检查标题和目标文字的 computed color。

## 专业前端工程约束

不改变用户旅程、状态 owner、数据生命周期、异步请求、键盘/焦点或性能边界。验证证据聚焦颜色对比度与页面可读性。

## 前端架构设计路由输入

`architecture-design_required: false`；无模块边界、所有权、状态/数据拓扑、依赖方向、公开 API 或共享抽象变更。

## 生产代码质量约束

这是模板类名修改；无新类型、null/失败分支、函数、常量、缓存、watcher 或边界 UI 状态。保留现有 loading/error/empty 表达。

## 人工评审可通过性约束

只允许修改四个目标 Vue 文件、媒体页契约测试及本请求文档。不得混入格式化、命名、组件抽取或逻辑重构。执行结束时需提供任务到 hunk 映射、聚焦测试、type/lint/build 和深色运行时证据。

## 函数式约束

不适用；无数据转换、业务规则或副作编排变更。

## 架构复用与共享所有权

不适用；深色 utility 按实际 DOM 文字层级就地归属，不新建共享 wrapper 或 class 常量。

## 局部性与文件抽取

不新增或移动 helper、hook、mapper、constant、utility 或组件。

## Vue 组件抽取

不适用；已有页面与 `GiftSoundTable` 边界保持不变。

## 设计约束

保持当前信息层级和 bilibili 粉色主操作色；只改中性文字/边框的深色状态。

## 项目启动与脚手架

不适用；非 greenfield。

## 变化轴与模式决策

- 问题：四个现有 Vue 模板缺少静态深色 utility。
- 深度：Level 0。
- 选择：`direct code`，pattern `none`。
- 信号：无选择、适配、状态分支、副作协调、数据管道或跨文件行为变化轴。
- 形态：就地 Tailwind class token。
- 充分性：问题的真实 owner 是每个 DOM 文字层级，新建 wrapper/helper/theme service 只会隐藏简单样式并增加跳转。

## 代码上下文与影响假设

代码图不可用且对这类局部样式缺陷不必要；`rg` 与直接源码阅读已确认影响闭包。

## API 与数据合同

不适用；无后端集成或字段语义变更。

## 前端/服务端责任边界

仅前端模板样式；服务端无任务。

## 上下文与依赖来源

用户原文/截图、四个 Vue 模板、现有 `.dark` Tailwind 变体和 `media-pages-contract.test.ts`。

## 边界情况

- 缺失音频文件错误文字在深色下仍需保持错误语义。
- 无数据时的 Element Plus empty/table 主题不属于此次 class 修复，但路由不得因修改无法渲染。
- 小屏布局不改变。

## 验收标准

1. 四个目标文件中每个常规 `text-gray-*` 样式行都同时包含 `dark:text-*`。
2. 目标文件中每个 `border-gray-200` 样式行都包含 `dark:border-zinc-800`，`divide-gray-200` 包含 `dark:divide-zinc-800`。
3. 上舰特效页截图标出的文字在深色运行时的 computed color 不再接近黑色。
4. 礼物提醒和礼物音效页的页标题、次要描述和页内标签在深色下可读。
5. 浅色 utility、DOM 结构、文案、业务逻辑和交互不变。
6. 聚焦测试、type-check、lint 和 Web build 通过，或仅记录明确的既有非阻断 warning。

## 人工评审与交付

交付小范围 diff、自动化检查、三路由深色视觉/DOM 证据与本地预览链接。

## 风险

主要风险是动态 `:class` 分支遗漏深色变体或修复仅覆盖截图文字；由逐行契约测试和三路由运行验证约束。

