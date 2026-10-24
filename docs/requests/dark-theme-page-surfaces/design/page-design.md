# Page Design: dark-theme-page-surfaces

## Delivery Unit And Objective

`dark-theme-page-surfaces`：让四个现有业务页面在深色主题下形成连续、可读的表面层级，消除固定白色块；不改变任务流、布局或业务内容。

## Target Scenario

主播或运营人员在深色主题下持续查看弹幕和数据，或维护敏感词与黑名单。成功标准是进入页面后壳层、页面、指标/表格和页脚层级连贯，内容可快速扫描，所有既有操作仍在原位置工作。

## Source And Upstream Constraints

- U1-U5 授权修复四个页面的黑白混用。
- `prd-snapshot.md` 禁止新增字段、文案、控件、交互或业务状态。
- 现有 `.dark` root selector、Element Plus dark variables 和排行榜页面的 `zinc` 层级是实现约束，不重新设计主题机制。

## Product-Content Allowlist Mapping

| Design content | Source | Decision |
| --- | --- | --- |
| 四页现有标题、控件、列表/表格、指标、图表、空/加载/错误状态 | U1-U5 + existing code | 内容、顺序和行为原样保留 |
| 深色页面/指标表面、文字和分隔线 | U1-U5 | 补齐对应 dark visual state |
| 浅色主题 | code-fact-backed compatibility | 现有样式不变 |

没有新增产品内容。弹窗、toast、导入/导出、危险操作确认和导航均沿用既有合同。

## User Journey And Task Success

用户切换深色主题 -> 进入任一指定页面 -> 首先识别页面标题和工具栏 -> 查看/维护主体内容 -> 读取页脚摘要。主题变化不改变主操作、次操作、危险操作、禁用条件、错误恢复或完成反馈；所有这些行为继续由现有组件负责。

## Information Architecture And Layout

- 弹幕面板：header -> reconnect feedback -> event stream -> footer。
- 数据看板：header -> error/history feedback -> metrics -> trends/empty state -> export dialog。
- 敏感词/黑名单：header -> import/error feedback -> table -> footer -> edit/add dialog。
- 保持原有阅读顺序、高度、最大宽度、toolbar wrapping、表格横向滚动与弹窗尺寸。

## Visual Hierarchy And Direction

- Tone：克制、紧凑的运营工具。
- 页面主表面：浅色仍为 `white`，深色为 `zinc-900`。
- 深色分隔线：`zinc-800`；标题/关键数值：`zinc-100`；辅助文字/元数据/空状态：`zinc-400`。
- 数据看板指标分隔背景使用深色边框层级，五个指标块与页面同一基础表面。
- 折线颜色保留当前蓝/绿/橙业务区分，不用主题主色覆盖数据语义；图表 canvas 与 tooltip 是否需要额外适配由实际验证决定，不能新增产品语义。
- 不新增卡片、装饰、动画、阴影或圆角。

## Section Breakdown

- 四页 root section：负责整页浅/深背景。
- Header/footer：负责分隔线及标题/摘要文本对比。
- 弹幕 event rows：深色分隔线及正文/时间可读性。
- Dashboard metrics：容器分隔背景、五个表面、label/value 对比。
- Existing Element Plus tables/controls：继续由全局暗色变量负责，不包覆固定浅色背景。

## Tailwind Styling Constraints

只在模板中追加 Tailwind utility；不新增 scoped CSS、CSS module、Sass/Less、inline style object、语义 class、常量/computed/helper。深色 utility 紧邻对应浅色 utility，保持 class 可直接审查；若单行过长，允许格式化标签但不隐藏 class 字符串。

## Interaction And Feedback Model

点击、筛选、输入、滚动、导入、导出、暂停、清屏、增删改、确认与取消全部不变。loading、empty、error、warning、success 和 validation 的 owner、文案及结束条件不变，仅确保其容器和文字在深色表面可读。

## Responsive, Accessibility And Usability

- 保留 toolbar wrap、表格 overflow 和既有宽度约束。
- 不改变 tab/focus 顺序、语义标签、按钮目标或键盘事件。
- 深色主/次文字和边框使用现有高对比 `zinc` 层级；业务状态仍通过 Element Plus 类型、图标和文案表达，不依赖单一颜色。
- 风险：静态 class 测试可证明合同存在，但 canvas/tooltip 仍需真实页面检查。

## Component Reuse And Split Strategy

所有区块保持原 owning SFC 内联。候选为四页表面 wrapper，但只有相似样式、业务语义和布局不同，抽取会形成单纯 pass-through wrapper；不满足组件边界。复用现有 `TrendChart`、`DataRangeControl`、`AudienceIdentityView` 和 Element Plus 组件，不新增组件或 shared helper。

## Design Risks And Open Questions

- 风险：数据看板图表由 ECharts canvas 生成，可能仍使用浅色轴线/tooltip；verify 以实际主题截图和 computed style 检查，若失败则按现有组件 owner 做最小修复并回到 spec/plan。
- Open UI questions：无。

