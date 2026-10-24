# 页面设计：统一页面外侧间距

## Delivery Unit Identifier

`page-inner-padding-consistency`

## 页面目标与场景

用户从侧栏进入上舰特效、礼物提醒、礼物音效或插件管理时，页面内容应与其他路由使用相同的 shell 外侧间距，不出现额外缩进。目标用户、任务、主次操作和完成反馈均保持现状。

## 来源与上游约束

- U1 和截图授权修复四页边距不一致。
- 既有 `page-content-spacing-background` 约定 `App.vue` `<main>` 以 `p-3 sm:p-5` 统一负责页面外侧间距。
- 四个目标页面根节点分别叠加 `p-[18px] xl:p-6` 或 `px-7 py-6`，是本次可删除的重复 presentation owner。

## Product-Content Allowlist Mapping

| 设计项 | 来源 | 处理 |
| --- | --- | --- |
| 四页外侧距离与其他页面一致 | U1 | 删除 route 根级重复 padding |
| Shell 响应式边距 | 既有规范与代码事实 | 保留 `p-3 sm:p-5` |
| 页面内容与交互 | source-derived 非变化约束 | 原样保留 |

无新增字段、标签、文案、控件、操作、状态、默认值、校验、权限、导航或反馈行为。

## 用户旅程与任务成功

导航进入目标页 -> 内容从 shell 的统一内边距后开始 -> 用户照常查看或编辑页面内容。成功信号是相同视口下四页标题左边界与 TTS/录制页面一致；取消、重试、错误和成功反馈继续由原页面拥有。

## 信息架构、布局与区块层级

Toolbar、侧栏、main、页面 header/body/footer 的结构、阅读顺序和视觉层级不变。唯一变化是 route 根节点不再额外包裹 padding；页面内部 header 间距、分栏 gap、表单项和表格区域保持原值。

## 视觉与样式方向

- 延续当前安静、紧凑的桌面工具界面。
- 页面外侧留白只由 shell 呈现：小视口 12px，`sm` 及以上 20px。
- 不新增卡片、圆角、阴影、背景、分隔线、颜色或动画。
- 仅使用 Tailwind utility；不新增 CSS、inline style、语义 class、常量或 helper。
- class 值保持原位可审查；删除 token 后不需要拆分组件或容器。

## Interaction Skeleton And State Model

所有点击、选择、表单、安装、保存、预览、禁用、loading、empty、error、success、pending 和权限状态完全保持现有实现。没有新的交互或文案。

## 响应式、可访问性与可用性

- 桌面宽度：标题距 main 左边界 20px，不再增加 24/28px。
- 窄屏：标题距 main 左边界 12px，不再增加 18/28px。
- 既有 toolbar wrapping、表格横向滚动、键盘、焦点、ARIA 和语义控件不变。
- 风险：误删页内布局间距。控制：只编辑根 `<section>` class，并用静态合同和运行时位置对比验证。

## Component Reuse And Split Strategy

不新增或抽取组件。现有 `App.vue` main 已是正确共享 owner；新增 wrapper 会形成无业务语义的单调用 pass-through。四个页面保留现有子组件边界。

## Design Risks And Open UI Questions

- 风险：插件页根背景仍与部分页面不同；不在 U1 的边距授权内，本次保持不变。
- 风险：其他非点名页面也可能存在自带 padding；按 closed-world scope 不顺带处理。
- 开放问题：无。
