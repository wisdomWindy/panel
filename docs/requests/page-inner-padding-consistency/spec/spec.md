# 规格：统一四个页面外侧间距

## Delivery Unit

`page-inner-padding-consistency`

## 背景、目标与范围

四个点名页面的 route 根节点在共享 `<main>` 已有 `p-3 sm:p-5` 的情况下再次增加 padding，造成截图中的额外左侧距离。目标是删除这四个根节点的重复 padding，使 shell 成为唯一页面外侧间距 owner。[source-backed: U1; code-fact-backed: App.vue and route roots]

范围内文件：

- `views/captain-effect/index.vue`
- `views/gift-reminder/index.vue`
- `views/gift-sound/index.vue`
- `views/plugin-manager/index.vue`
- `components/application-shell/shell-components-contract.test.ts`

范围外：其他路由、内部区块间距、背景与主题 token、DOM 结构、文案、控件、状态、交互、路由、store、IPC、API 和 Rust。

## Source Grounding And Product Allowlist

| 规格项 | 标签 | 来源 |
| --- | --- | --- |
| 四页外侧边距与其他页一致 | source-backed | U1 |
| 删除根级重复 padding | code-fact-backed repair | 四个 route 根 class |
| 保留 shell `p-3 sm:p-5` | code-fact-backed compatibility | `App.vue` 与既有规范 |
| 不改内部布局或业务 | source-derived | 缺陷只指向外侧距离 |

Product-content delta: none。唯一用户可见变化是四页内容起始位置统一；不允许新增或修改字段、文案、控件、交互、状态、默认值、校验、权限、导航、API/payload、状态映射或埋点。

## User Intent And UI Contract

- Literal：修复上舰特效、礼物提醒、礼物音效、插件管理页面与其他页面不一致的边距。
- Practical：内容不再因两层 padding 显得过度缩进，四页与常规路由对齐。
- Forbidden：修改 shell 边距值、删除所有留白、调整页内表单/表格间距、顺带修改其他页面、引入 wrapper/CSS 或重构业务。
- Layout：shell 继续提供小屏 12px、`sm` 以上 20px；四个 route 根节点不得声明任何 padding utility。
- Responsive：桌面和窄屏均只使用 shell 响应式间距。
- Accessibility/interaction/state：DOM、键盘、焦点、ARIA、loading/error/empty/disabled 等全部不变。

## Functional Breakdown

1. 合同测试读取 `App.vue`，确认共享 main 继续包含 `p-3 sm:p-5`。
2. 合同测试读取四个目标 route，提取首个根 `<section>` class 并拒绝 `p-*`、`px-*`、`py-*` 及响应式前缀变体。
3. 上舰特效、礼物提醒、礼物音效根 class 从 `min-w-0 p-[18px] xl:p-6` 改为 `min-w-0`。
4. 插件管理根 class 删除 `px-7 py-6`，保留宽度和背景/主题 class。
5. 运行时对比四个路由与 TTS/录制页面标题位置，并检查桌面/窄屏无重叠或横向溢出。

## Styling, Architecture And Pattern Decision

- Tailwind utility only；不新增 CSS、inline style、语义 class、常量、map、computed 或 helper。
- `architecture-design_required: false`：无模块边界、状态拓扑、依赖方向、公共 API 或抽象变化。
- Pattern fit：Level 0 `direct code`。变化轴只是删除四处重复静态 utility；不存在运行时选择、适配、创建、编排或共享行为信号。
- 不新增/抽取 Vue 组件；现有 shell 与 route owner 保持清晰。

## Engineering And Quality Constraints

- 无类型、空值、错误、异步、请求、缓存、性能或副作用变化。
- 修改前后链路均为 `App.vue main -> router-view -> route root -> page content`。
- 只允许五个实现/测试文件与本请求文档的任务 hunk；保留工作区其他用户改动。
- TDD：先增加根级 padding 合同并观察 RED，再删除四页 token 取得 GREEN。
- 代码评审合同：无外部输入健壮性变化；无函数/常量/抽象；无运行时性能成本；保持 Vue 3 + Tailwind + Element Plus + Vitest 技术栈。

## Acceptance Criteria

1. `App.vue` main 仍包含 `p-3 sm:p-5`。
2. 四个目标页面的根 `<section>` class 均不包含 padding utility 或响应式 padding variant。
3. 四页内部 DOM、页内 class、文案、状态与交互不变。
4. 桌面和窄屏下，四页只表现 shell 统一边距，无新增重叠或溢出。
5. Focused tests、相关测试、type-check、lint 和 web build 通过，或只存在明确的既有非阻断 warning。

## Risks And Handoff

- 风险：正则误扫内部 padding；测试只读取根 `<section>` class。
- 风险：插件页背景差异；本次不改，避免越权。
- 风险：脏工作区；只提交精确 token hunk。
- 交付说明包含根因、五个文件、测试结果、运行时位置证据和预览地址。
