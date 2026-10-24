# Spec: page-content-spacing-background

## Delivery unit identifier

`page-content-spacing-background`

## Background and goals

用户指出弹幕面板、数据看板、排行榜、敏感词、黑名单以及其他页面存在灰色底层露出、内容区边距观感异常。目标是在浅色主题下让共享内容区域呈现连续白色背景，同时保留现有响应式外边距。[source-backed: U1-U5]

## In scope

- 将 `App.vue` 根壳层和主内容区的浅色背景从 `zinc-100` 调整为白色。[source-backed: U2/U5; code-fact-backed: App.vue]
- 保留主内容区 `p-3 sm:p-5`、滚动行为和 `dark:bg-zinc-950`。[source-backed: U3; code-fact-backed: App.vue]
- 通过共享壳层覆盖全部 `<router-view>` 页面，并增加壳层样式契约测试。[source-backed: U4; code-fact-backed: router/App.vue]

## Out of scope

不调整任一页面内部 padding、最大宽度、表格、筛选、按钮、空态、文案、路由、状态、API、弹窗或业务行为；不新增包装组件、CSS 文件或设计装饰。

## Trigger and start conditions

用户在浅色主题下进入任一主应用路由；应用已渲染共享 `App.vue` shell。

## Requirement split summary

非 PRD 拆分型 bugfix，单一交付单元，所有目标页面共享同一 shell owner。

## Source grounding and traceability

| Spec item | Grounding | Product-facing | Evidence | Downstream obligation |
| --- | --- | --- | --- | --- |
| 消除灰色底层 | source-backed | yes | U2/U5 与截图 | shell 浅色背景为白色 |
| 保留内容边距 | source-backed | yes | U3 | 锁定 `p-3 sm:p-5` |
| 同步其他页面 | source-backed | yes | U4 | 只改共享 owner |
| 深色主题不变 | source-derived + code-fact-backed | no | 缺陷限定浅色，现有 dark token | 保留 `dark:bg-zinc-950` |
| 页面业务行为不变 | source-derived | no | 纯样式缺陷且所有解释结果一致 | 禁止扩大 diff |

Closed-world product content allowlist：仅允许“浅色 shell 背景为白色、主内容区保留响应式边距、全部共享路由继承修复”。字段、文案、控件、交互、状态、默认值、校验、权限、导航、API/payload、状态映射和埋点均不得变化。

## Workflow efficiency profile

`S1 local`。单一共享 owner、无状态/API/架构变化，是最低安全等级；使用 compact artifacts 和 scoped context。若运行时证据表明存在独立页面 owner、深色回归或必须逐页变更，则升级范围并回退 spec。

## Self-healing completion bar

- 需求一致性：灰色底层消失、边距保留、所有共享路由生效、深色 token 未变。
- 业务逻辑：条件、字段、权限、状态、校验、adapter、payload、页面空/loading/error 状态均无改动。
- 工程检查：focused Vitest、相关页面测试、type-check、lint、web build 和桌面视口运行时检查通过，或明确记录替代证据。
- 运行安全：无新增异步、可空、竞态、清理或响应过期路径。
- 组织与维护性：仅修改 owner 与契约测试；无无关格式化、死代码、调试代码、临时命名或陈旧注释。
- 完成条件：`self-healing loop compliance: pass`、`self-healing loop assessment: pass`、remaining blockers `none`。

## User intent contract

- Literal request：修复五个点名页面的灰色背景与无边距观感，并同步其他页面。
- Practical goal：页面之间保持统一、干净且连续的浅色内容背景，边距真实存在而非被灰色底层误读为边框。
- Success criteria：共享内容区白色、`p-3 sm:p-5` 保留、全部路由继承、深色模式不退化。
- Forbidden interpretations：删除边距、逐页复制修复、把整页改成卡片、改业务内容或交互。
- Acceptable approach：在现有 `App.vue` shell owner 中直接替换浅色背景 utility，并锁定契约。
- Verification：静态 class 契约、相关测试、1280x800 运行时视觉检查和 changed-hunk review。

## User flow

侧栏进入任一页面 -> 共享主内容区以白色底呈现并提供响应式边距 -> 页面原有内容、状态与操作照常工作。失败、重试、取消和完成反馈继续由原页面拥有。

## Page and module design

结构保持 `toolbar -> navigation + main -> status bar`。`main` 继续拥有滚动和外边距，route SFC 继续拥有 header/body/footer、最大宽度及页面内部布局。

## UI / UX design contract

浅色主题使用连续白色 shell，间距表现为空白而非灰色框；不新增卡片、阴影、圆角或动画。空态、filtered empty、loading、error、permission、readonly、validation、success、stale 和 pending 状态原样保留。小视口 `p-3`，`sm` 及以上 `p-5`；DOM、键盘、焦点和语义控制不变。

## Frontend styling constraints

仅使用可直接审查的 Tailwind utility；不新增 scoped CSS、CSS module、Sass/Less、inline style 或语义 class。只替换已有背景 token，不把 class 隐藏到常量、map、computed、helper 或 import 中。

## Field display semantics and interaction visual-state contracts

不适用：无字段、列、链接、标签、状态或交互视觉状态变化。

## Function-complete behavior breakdown

1. 根 shell 在浅色主题下使用白色背景，避免页面高度不足时底部露灰。
2. 主内容区在浅色主题下使用白色背景，避免 padding 区形成灰色框。
3. 主内容区继续使用 `overflow-y-auto p-3 sm:p-5`。
4. 根 shell 与主内容区继续使用 `dark:bg-zinc-950`。
5. 所有 route 页面原有渲染和操作不变。

## Expert frontend engineering constraints

状态、数据生命周期和异步 owner 不变；无请求、缓存、竞态、重试、bundle 或 reactive fan-out 影响。共享 shell 是唯一修改点和回滚面。测试应能确定性证明 class 合同，运行时检查证明实际布局视觉连续。

## Frontend architecture design routing inputs

`architecture-design_required: false`。共享 owner、依赖方向、状态拓扑、数据流、公共 API 和模块边界均未改变；无需架构设计阶段。

## Production code quality constraints

无类型、null、异常、数据转换、class 或性能优化需求。保持现有命名和模板结构；不引入 helper、常量或抽象。页面边界状态保持原有实现。

## Human review readiness constraints

允许改动 `App.vue` 的两个浅色背景 token、壳层契约测试及本请求工件。禁止格式化整文件或修改页面 SFC。共享代码风险通过 focused regression、dark-theme regression、type/lint/build、运行时检查覆盖；changed hunks 必须逐项映射到计划任务。

## Functional-programming constraints

不适用：无业务规则、转换、派生状态或副作用。

## Architecture reuse and shared ownership constraints

现有等价 owner 即 `App.vue` shell。各页面的白色 surface 不是独立修复 owner；逐页补 class 会形成重复和漂移。决定复用现有共享 owner，不新增 shared wrapper/API，因此无需 Anti-DRY 抽取或 JSDoc。

## Locality and file-extraction constraints

不新增或移动 helper、hook、mapper、adapter、constant、utility 或 feature module。样式变更就地保留在 `App.vue`。

## Vue component extraction constraints

候选 shared page wrapper 决定 `keep inline/no extraction`：它只有 pass-through slot 与简单 class，不拥有独立业务概念、状态或交互生命周期；现有 `App.vue` 已是正确 owner。

## Design constraints

保留现有信息架构、视觉层级、密度、页面最大宽度、滚动和 toolbar wrap；仅消除错误的浅色 surface 对比。

## Project bootstrap and scaffold decision

不适用：既有 Vue 应用的局部修复，不创建项目、应用、包或新前端 surface。

## Change axes and pattern decision

变化轴只有共享 shell 的浅色背景 token。不存在选择、适配、创建、编排、副作用协调、状态分支或跨文件运行时协作信号。采用 `Level 0 direct code` / `direct code` / pattern `none`，使用 Vue 模板中的 Tailwind utility；新增 wrapper 或 page-by-page abstraction 更重且会模糊唯一 owner。

## Code context and impact assumptions

`App.vue` 的 root grid 与 `<main>` 是所有目标路由的真实共享边界；`router-view` 下页面根 surface 为白色。code graph 不可用，已通过 router、App 和目标页面定向检索获得高置信 fallback 证据。

## API and data contracts

不适用：无后端集成、DTO、protobuf、adapter 或 payload 变化。

## Frontend / server responsibility boundary

仅前端呈现层 class 变更；无服务端或共享契约任务。

## Context and dependency sources

实现受 `packages/bilibili-live-tool/tsconfig.eslint.json` -> `tsconfig.json` 治理，严格模式、bundler module resolution、Vue plugin 和 path aliases 不变。本次无 TypeScript 或声明文件改动；Tailwind/Vue 构建链沿用现有配置。

## Edge cases

- 页面内容不足一屏：底部仍为白色。
- 页面内容溢出：`main` 继续纵向滚动。
- 小/大视口：分别保持 12px/20px 外边距。
- 深色主题：继续使用 `zinc-950` shell 与页面既有 dark surface。
- 页面自带额外 padding：保持不动，避免扩大用户未授权范围。

## Acceptance criteria

1. `App.vue` 根 shell 和 `<main>` 的浅色背景均为 `bg-white`，且不再使用 `bg-zinc-100`。
2. `<main>` 仍包含 `overflow-y-auto`、`p-3 sm:p-5` 和 `dark:bg-zinc-950`。
3. 弹幕面板、数据看板、排行榜、敏感词、黑名单及其他共享路由均通过同一 shell 继承修复。
4. 页面业务 DOM/状态/API/交互未被修改。
5. 回归测试、类型检查、lint、web build 与 1280x800 运行时视觉检查通过。

## Human review and handoff

交付需说明根因、共享修复位置、测试命令、运行时地址与视觉验证结果；reviewer 可从两处 class token 和新增断言快速复核，无需理解页面业务逻辑。

## Risks

- 共享 owner 影响所有路由：用全局 shell 契约和多路由运行时检查降低风险。
- 误删 padding：用静态断言锁定。
- 浅色修复误伤深色：保留并测试 dark token。
- 现有脏工作区：只编辑精确 hunk，不覆盖用户改动。
