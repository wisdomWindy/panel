# Page Design: page-content-spacing-background

## delivery unit identifier

`page-content-spacing-background`

## page objective

让主应用所有路由在浅色主题下共享连续白色内容背景和统一响应式边距，消除页面四周/底部意外灰色和“内容贴边”的视觉感受。

## target users or scenarios

主播或运营人员在 1280x800 等桌面视口中切换弹幕、数据、风控及其他功能页面。

## source and upstream constraints

- U1-U5 授权修复灰色底层、边距和全页面同步。
- `prd-snapshot.md` 禁止新增产品内容或改变页面交互。
- `App.vue` 已是所有主路由的单一 shell owner，已有 `p-3 sm:p-5` 边距合同。
- 深色主题 `zinc-950` 与页内 `zinc-900` surface 层级保持。

## product-content allowlist mapping

| Design content | Source | Decision |
| --- | --- | --- |
| 浅色连续白色内容背景 | U2/U5 | shell root 和 main 浅色背景使用白色 |
| 统一页面边距 | U3 | 保留 main 的 `p-3 sm:p-5` |
| 所有同壳层页面同步 | U4 | 只在 `App.vue` 共享 owner 修复 |
| 现有字段、文案、控件、状态和交互 | existing code + closed-world allowlist | 原样保留，product-content delta: none |

## user journey and task success

用户从导航进入任一页面 -> 内容区呈现连续白色底与固定边距 -> 按原有流程查看数据或操作。主操作、次要操作、危险操作、禁用条件、取消、错误恢复和完成反馈全部由现有页面拥有，本次不改。

## information architecture and content strategy

信息架构、阅读顺序和操作顺序不变。Shell 只提供视觉连续的底层和外边距，各页 header/body/footer 仍由原 SFC 拥有。

## layout structure

`toolbar -> navigation + white main(p-3/sm:p-5) -> status bar`。`router-view` 内的页面 max-width、表格滚动、toolbar wrap 和高度策略不变。

## visual hierarchy

浅色主题使用单一白色壳层底色，间距体现为空白而不是灰色边框。页内标题、分隔线、表格和状态色保持原层级。

## visual design direction

克制、紧凑的运营工具；不新增卡片、圆角、阴影、装饰或动画。修复只消除错误的灰色 surface 对比。

## section breakdown

- Root grid：应用最后背景。
- Main：主路由背景、滚动和响应式边距。
- Route pages：保留各自业务布局，不逐页叠加修复 class。

## styling direction

修改共享 shell 的浅色 `bg-*` utility，深色 variant 不变。

## Tailwind CSS-style styling constraints

只使用 Tailwind utility；不新增 scoped CSS、CSS module、Sass/Less、inline style object 或语义 class。

## class length and structure strategy

原有 shell class 已按 formatter 可读。只替换背景 token，不通过常量、computed、helper 或 import 隐藏 class。

## interaction skeleton

导航、搜索、筛选、表格、对话框、键盘、焦点、取消和重试交互均不变。

## state and feedback model

空、loading、error、success、disabled、validation、permission 和 pending 状态仍由原页面/组件拥有；本次仅改其外层底色，不新增文案或反馈。

## UX writing and feedback copy stance

product-content delta: none；不修改或新增任何 UX 文案。

## responsive behavior

保留小视口 `p-3`、`sm` 及以上 `p-5`；保留现有页面的 wrap 与 overflow。

## accessibility and keyboard considerations

DOM 结构、语义元素、tab 顺序、焦点和颜色状态都不改。白色底与现有深色正文对比保持。

## usability heuristics and risk controls

- 风险：逐页修复可导致漂移。缓解：只修共享 shell owner。
- 风险：移除 padding 造成真正贴边。缓解：合同测试显式锁定 `p-3 sm:p-5`。
- 风险：浅色修复误改深色层级。缓解：锁定 `dark:bg-zinc-950`。

## component reuse and split strategy

无 Vue 抽取。候选 shared page wrapper 仅会传递 slot 并包裹简单 class，是 generic pass-through，不满足抽取条件。继续使用现有 `App.vue` owner。

## design risks

某些页面自带额外 padding，本次不删除；它们属于页内布局，用户截图缺陷的共享根因是 shell 灰色底层。若运行时证据显示页内 padding 仍不一致，需回到 spec/plan 而不在 execute 中扩展。

## open UI questions

无。

