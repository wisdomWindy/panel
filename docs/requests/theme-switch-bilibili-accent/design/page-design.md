# 页面设计：一致主题与 bilibili 品牌色

## 交付单元标识

`theme-switch-bilibili-accent`

## 页面目标与场景

用户在设置页选择浅色、深色或跟随系统后，应用壳层、内容页和 Element Plus 控件立即呈现同一主题；主操作统一使用 bilibili 粉。

## 来源与上游约束

来源为 `bugfix-source.md` U1-U4。保留现有布局、控件、文案、导航、反馈与保存流程；仅修复主题/色彩视觉状态。

## Product-Content Allowlist Mapping

| 设计项 | 来源 |
| --- | --- |
| light/dark/system 即时且一致 | U1/U2/U4 |
| system 随系统变化 | U4 |
| 默认主色 `#FB7299` 与完整状态色 | U3/U4 |
| 旧默认迁移、其他自定义色保留 | U4 |

无新增字段、标签、操作、状态、校验、权限或导航。

## 用户旅程与任务成功

设置 -> 界面 -> 选择主题/主色 -> 现有预览请求返回 -> 整个应用同步更新 -> 保存。失败反馈继续使用现有消息；本次不新增反馈文案。

## 信息架构、布局与区块

不改变现有设置页双列配置、侧栏、工具栏、主内容和状态栏结构。主题一致性覆盖所有现有区域，不新增卡片或容器。

## 视觉层级与方向

- 浅色：浅灰页面背景、白色表面、深色正文。
- 深色：近黑页面背景、深灰表面、浅色正文。
- 品牌主色：`#FB7299`，用于主按钮、选中、焦点及其 hover/active/浅背景派生状态。
- 成功、警告、错误等语义色保持原有绿色/黄色/红色，不被品牌色吞并。

## Styling Direction

继续使用 Tailwind utilities 与 Element Plus 变量。Tailwind 4 的 dark variant 改为 `.dark` selector；不新增 scoped CSS、CSS module、Sass/Less 或语义 class。

## Class Length And Structure

现有 template class 不改，不通过常量/helper 隐藏 class；主题配置由框架 directive、CSS variables 和投影逻辑承担。

## Interaction / State / Feedback

- light/dark：以显式用户选择优先于系统主题。
- system：当前系统值决定，并监听后续变化。
- runtime stop：移除媒体查询监听。
- 自定义颜色：基础与派生色同步；旧默认精确迁移。
- loading/disabled/success/error：沿用现有设置页行为。

## UX Writing

无新增或修改文案。

## 响应式与可访问性

布局不变；浅/深主题均需保持文本、边框与表面可辨识。主色不能成为唯一状态信号，现有按钮文字、选中控件和焦点结构保留。

## 组件复用与拆分

不抽取或新增 Vue 组件。主题所有权保持在 projection/runtime；避免单调用方展示组件或 wrapper。

## 风险与缓解

- 风险：Element Plus 派生变量残留默认蓝色；通过同一 accent 生成全部主色色阶。
- 风险：系统监听泄漏；由 shell runtime start/stop 对称管理。
- 风险：旧保存值覆盖新默认；仅迁移精确旧默认，保留其他自定义值。

## Open UI Questions

none。

