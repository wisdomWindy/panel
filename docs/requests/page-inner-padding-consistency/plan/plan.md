# 计划：统一四个页面外侧间距

## Delivery Unit And Summary

`page-inner-padding-consistency`，S1 local，串行执行 `T1 -> T2 -> T3`。目标是让 shell 成为唯一页面外侧间距 owner，product-content delta: none。

## Source And Intent Contract

- U1 -> 四个点名页面与其他页面边距一致。
- 既有 shell 合同 -> 保留 `p-3 sm:p-5`。
- 禁止解释 -> 不改 shell 数值、内部布局、其他页面、业务或背景主题。

## T1：根级边距合同 RED

- 文件：`components/application-shell/shell-components-contract.test.ts`。
- 在 redesigned shell 测试邻近位置新增单独用例：确认 `App.vue` 仍包含 `p-3 sm:p-5`；读取四个目标页面首个根 `<section>` class，拒绝含可选响应式前缀的 `p/px/py/pt/pr/pb/pl-*`。
- 先运行单测试文件，预期四页现有根 class 触发失败。
- 测试只扫描根 class，不限制页面内部合法 padding。

## T2：删除四页重复 padding GREEN

- `captain-effect/index.vue`：根 class 删除 `p-[18px] xl:p-6`。
- `gift-reminder/index.vue`：根 class删除相同 token。
- `gift-sound/index.vue`：根 class删除相同 token。
- `plugin-manager/index.vue`：根 class删除 `px-7 py-6`，保留 `min-w-0 bg-white dark:bg-gray-900`。
- 不改第二行以后任何 DOM、class、文案、脚本、store、事件或副作用。
- 重跑 T1 用例并用 `rg` 反向确认四个根节点无 padding。

## T3：验证与评审

- Focused：shell contract。
- Related：media page contract、plugin release pages、dark theme surface tests。
- 工程：`type-check`、`lint:check`、`build:web:free`、`git diff --check`。
- Runtime：深色主题桌面 1280x800 与窄屏宽度分别检查 `/captain-effect`、`/gift-reminder`、`/gift-sound`、`/plugins`；读取 main padding、root padding 与标题左边界，确认无重叠/溢出。
- Diff review：只认领测试用例和四处根 class token；隔离同文件中已有用户改动。

## Change Chain And Ownership

```mermaid
flowchart LR
  A[App main p-3 sm:p-5] --> B[router-view]
  B --> C[目标 route 根 section]
  C --> D[页面 header/body]
```

修改前根 section 重复增加 padding；修改后 C 不再拥有外侧 padding，A 成为唯一 owner。无 import/export、调用方、状态、事件、请求、watch/computed、测试 mock 或数据消费者变化。

## UI / UX And Responsive Contract

- 信息架构、视觉层级、表单/表格/分栏、状态反馈与交互均保持。
- Desktop shell padding 20px，窄屏 12px；四页根节点 computed padding 应为 0px。
- 不引入组件、CSS、wrapper、主题 token 或任何产品文案。

## Pattern, Architecture And Reuse

Level 0 `direct code`。无真实模式信号；shell 已是共享 owner，不新增抽象。Architecture design 不需要。四个相同删除动作是纠正重复声明，不形成新的业务 helper 或组件候选。

## Production And Review Contract

- 类型、null、异常、异步、性能、内存和边界状态不变。
- Tailwind utility 原位删除，class 值保持可审查。
- 不格式化整文件，不改无关 import，不留下 debug/dead/stale 代码。
- 每个 changed hunk 必须映射 T1 或 T2；review 需独立确认 U1、spec 与实际 diff 对齐。

## Rollback And Completion

若运行时显示 shell 间距未生效或内部布局依赖根 padding，则回到 spec/plan，不在 execute 扩大改动。完成条件是 AC1-5 全过、无 blocker、verification/review 全部 pass。
