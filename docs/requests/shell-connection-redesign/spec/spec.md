# Spec: shell-connection-redesign

## Scope And User Intent Contract

- literal request：重新设计截图中的页面，确保正确展示、功能正常。[source-backed: U1, U2]
- practical goal：应用启动后首屏立即可读、可操作，不再出现近似空白或深色低对比界面；既有连接与导航链路不退化。[source-backed/source-derived: U1, U2]
- forbidden interpretations：不得只换颜色而保留信息层级问题；不得通过删除功能消除视觉问题；不得新增未授权业务字段或模拟连接成功。
- acceptable approach：在既有 Vue/Element Plus/Tailwind 组件边界内重排应用壳与 connection view，修正 free Web 启动脚本。

## Source-grounding / Product Content

只允许 `prd-snapshot.md` P01-P06。所有展示文案复用现有 i18n key 或现有 connection 文案；不得新增帮助语、快捷入口、统计字段、成功 toast、模拟数据或新错误语义。

## Behavior Contracts

### B01 Web Startup

`pnpm dev:web` 必须以 `free` mode 启动 Vite，使 `VITE_BUILD_CHANNEL` 能解析为 `free`，页面可完成 Vue mount。[P06]

### B02 Shell Composition

- 根壳维持 toolbar / body / status 三行和 navigation / content 两列结构。[P01]
- 初始化时壳层保持可见；`shell.pending` 映射为根节点 `aria-busy`，不再使用覆盖整个应用的 `v-loading`。[P01; code-fact-backed runtime safety]
- Toolbar 显示现有应用 icon、产品名、房间输入、连接/断开、通知、设置；连接事件、popover 焦点恢复与 settings 路由保持现有行为。[P01-P03]
- Navigation 继续消费 `ShellNavigationItem[]`、发出 `navigate`、支持 disabled/group/active route；宽视口显示文字，窄视口保留图标栏。[P01]
- Status bar 继续渲染七个 normalized `StatusBarItem`，不修改字段值或 format。[P01]

### B03 Connection Page

- 显示 `直播间`、当前连接状态、房间号、房间名称、在线人数。[P04]
- 状态文本映射保持：未连接、连接中、已连接、重连中 n/10、连接错误。[P04]
- null 房间号/名称/在线人数继续显示 `-`；onlineCount 非 null 继续使用 `formatCount`。[P04]
- room.error 继续显示现有不可关闭 `el-alert`。[P04]
- 深浅主题均保证标题、状态和摘要可读；状态必须同时由文字/图标和颜色表达。[P05]

## Field Display Semantics

| 字段 | 业务语义 | 格式/精度 | 空值 | 交互与视觉 |
| --- | --- | --- | --- | --- |
| 连接状态 | 当前 live room transport 生命周期 | 既有中文映射；reconnecting 带 attempt/10 | 不适用 | 不可点击；默认即强调；非颜色单一表达 |
| 房间号 | 当前连接上下文的原始房间 ID | 原字符串，不数值化 | `-` | 不可点击；tabular nums |
| 房间名称 | 后端快照的当前房间名称 | 原文本，溢出截断 | `-` | 不可点击；title 不新增 |
| 在线人数 | 当前快照计数 | 既有 `formatCount` | `-` | 不可点击；tabular nums |

## State / Async / Failure Contract

- writable state owner 保持现有 Pinia stores；本次不复制状态。
- Toolbar 的 roomInput/roomError/notificationOpen 保持 `App.vue` owner。
- 连接 pending 仍由 room store owner；按钮 loading/disabled 沿用现有语义。
- bootstrap 失败仍由 runtime/store 处理；视觉壳不因 pending 被遮蔽。
- 不新增请求、取消、重试策略或持久化。

## Responsive / Accessibility Contract

- 1280x720：所有壳区域、导航文字、三个摘要块和七项状态可见，不重叠。
- 900x720：导航收成图标栏，内容无横向溢出，工具栏控件不覆盖。
- icon-only controls 均有 tooltip + aria-label；active nav 有 `aria-current`；root 有 `aria-busy`；连接状态非颜色单一表达。

## TypeScript Context

- governing config：`packages/bilibili-live-tool/tsconfig.eslint.json -> tsconfig.json`。
- material options：strict、noUnused、ES2020/DOM、bundler resolution、`@/*` alias、Vue plugin、vite/client。
- types：复用 `RoomSnapshot`、`ShellNavigationItem`、`StatusBarItem` 和现有 generated auto-import/component declarations；新增图片 import 由 `vite/client` 提供。

## Styling / Component Contract

- authored styling 只用 inline-reviewable Tailwind utilities；无 scoped CSS、inline style object、class constant/helper。
- 复用 Element Plus controls 和 `@element-plus/icons-vue`；不替换技术栈或引入依赖。
- 现有三个 shell 组件保持；connection 子区块保持同一 SFC 内联。Vue extraction decision：`keep inline`，真实调用方 1，无独立生命周期，抽取只会增加透传。
- 视觉表面使用中性 zinc/white，品牌 icon 提供 cyan/yellow，多种语义色区分连接状态；不使用渐变、装饰 orb、超大标题或圆角大于 8px 的卡片。

## Pattern-fit Decision

- Level 0 `direct code`。
- 现有 component/store/composable 边界已经承担组件组合、状态与副作用；本次只有 markup、utility styles 和 script mode 修正，无新选择、适配、创建、流程编排或跨文件业务协作变化轴。
- 不引入新 manager/facade/adapter/composable/utility 或组件层；继续复用现有事件与数据路径。
- `architecture_design_required=false`：文件 owner、dependency direction、state topology、public API 均不变。

## Production / Review Constraints

- Type-first：复用现有 types，不新增宽松对象或 assertion。
- Fail-fast / null：不改变现有 normalizeRoomInput、command rejection 与 null display contract。
- Performance：无新增 watcher、listener、timer 或高频 handler，不需要 debounce/memoization。
- Human review：diff 仅限 request artifacts、package script、五个 scoped Vue 文件和直接合同测试；禁止无关格式化与跨页面重构。
- Change-chain：修改前后核对 App -> shell components -> router-view/status、navigation emit -> App.navigate -> router、toolbar emit -> App handlers -> stores/router。

## Acceptance Criteria

- AC01：`pnpm dev:web` 控制台不再出现 `VITE_BUILD_CHANNEL must be either free or member`，页面成功 mount。[B01]
- AC02：1280x720 free Web 首屏视觉显示品牌、房间连接控件、完整导航、直播间标题/状态、三个摘要字段和状态栏。[B02-B03]
- AC03：深色与浅色/系统主题下无标题或字段与背景近似同色。[B02-B03]
- AC04：输入空白/非法房间号仍得到既有校验；连接按钮不绕过 `normalizeRoomInput`。[B02]
- AC05：导航到 dashboard/settings 后当前项与 router-view 正常更新；通知/设置控制仍可操作。[B02]
- AC06：900x720 无重叠，导航压缩后主要连接控件与内容仍可用。[responsive]
- AC07：相关 Vitest、type-check、lint:check、build:web:free 通过。[quality]

## Risks / Non-goals

- Web 模式没有 Tauri runtime，真实连接 command 可失败；该失败不作为生产连接证据，也不得伪造成功。
- 不验证外部 Bilibili 生产凭证/端点。
- 不重设计其他路由页面内容。

## Self-healing Completion Bar

需求一致性、业务链路、类型/语法、运行时首屏、组件组织、样式政策、人工评审可读性和 scoped technical debt verdict 全部通过，verify/review 无 blocker 后才允许 complete。
