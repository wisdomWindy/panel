# Live Data And Safety Engineering Spec

## Delivery Unit Identifier

- request: `tauri-rust-refactor`
- module: `live-data-safety`
- lifecycle: `page-design -> spec -> architecture-design -> plan -> execute -> verify -> review`
- scope source: PRD 功能 1、2、3、5、9、10、11，以及 PRD 2.2、8、10、12-15 的适用横切约束
- upstream artifacts: `requirements/requirement-map.md`、`requirements/modules/live-data-safety.md`、`design/page-design.md`
- `architecture-design_required: true`

## Background And Goals

本交付单元把直播间只读连接、事件解析、身份识别、敏感词、黑名单、实时统计、排行榜和 CSV 导出迁到 Rust，并以 Tauri Command/Event 暴露给 Vue。Vue 只负责经批准的本地视图筛选、页面交互、图表和列表渲染。目标不是保留现有 WebView 业务实现，而是保留产品行为并修复所有权倒置。[source-backed: PRD 2.2、9.1-9.3、9.5、9.9-9.11]

## In Scope

- `room_connect`、`room_disconnect`、固定 5 秒/最多 10 次自动重连、连接状态事件和资源清理。[source-backed: PRD 9.1.2、9.1.5-9.1.7]
- Rust 解析 `DanmuEvent`、`GiftEvent`、`CaptainEvent`、`EnterEvent`，并在事件进入 UI/下游前完成过滤、日志和聚合。[source-backed: PRD 9.1.2]
- 房管、总督、提督、舰长、老爷、普通观众身份与粉丝勋章归一。[source-backed: PRD 9.2]
- 敏感词精确/模糊匹配、CRUD、启停、筛选、搜索、导入导出、编译进度、过滤日志和事务回滚。[source-backed: PRD 9.3]
- 黑名单增删、搜索、导入导出、重复 UID 更新原因分支和入口拦截。[source-backed: PRD 9.5; confirmed-decision: 2026-09-14 CSV 口径]
- 数据看板五项指标、三条趋势、范围查询、5 秒事件、手动刷新和历史降级。[source-backed: PRD 9.9]
- 送礼/弹幕/舰长三类榜单、TOP10/20/50/100、10 秒事件、3 秒变动高亮和用户详情。[source-backed: PRD 9.10; confirmed-decision: 舰长贡献金额口径]
- 看板/榜单/词库/黑名单通过系统对话框授权路径并由 Rust 写入文件。[source-backed: PRD 9.3、9.5、9.11、10.1-10.3]
- 两个构建通道都完整包含本模块，不受会员状态影响。[source-backed: PRD 4.2、9.9-9.11]

## Out Of Scope

- 任何发送弹幕、操作直播间、模拟发送、发送 Command、发送输入框或自动任务。[source-backed: PRD 0.1、9.1.1、9.1.7]
- 自动禁言、刷屏防护和 `moderate_mute` 的实现；用户详情中的禁言入口由 `membership-intelligence` 负责。[source-backed: requirement split; PRD 9.4]
- 舰长特效、礼物提醒/音效、TTS、录制和高光；这些能力只消费本模块的合格事件。[source-backed: requirement split]
- 应用四区壳层、最终导航组织、通知中心和完整设置页；本模块只提供页面与状态接入点。[source-backed: requirement split]
- OAuth、支付、模块、AI、ASR、插件、更新与发布能力。[source-backed: requirement split]
- 未配置真实 Bilibili 开放平台凭证时伪造连接成功或生成模拟直播数据。[source-backed: request non-goal; missing-source external contract]
- 在此阶段修改任何生产代码。[framework constraint: spec stage]

## Trigger And Start Conditions

- `platform-foundation` 已完成 review，统一 `AppState`、SQLite/WAL、typed desktop gateway、错误合同、路径授权与双构建通道均可用。[code-fact-backed: platform-foundation review]
- `live-data-safety` page design 已完成，五项产品缺口已由用户在 2026-09-14 以“按建议继续”确认并写入 `design/page-design.md`。[confirmed-decision]
- 原 PRD 是当前未提交的只读事实源，不得修改或回滚。[direct user constraint]
- 生产 Bilibili 凭证/端点配置仍可缺失；该缺口要求真实 unavailable/degraded 状态，不阻断本地领域、持久化、IPC 和 UI 实现。[source-backed + missing-source boundary]

## Requirement Split Summary

本模块严格承接拆分中的功能 1、2、3、5、9、10、11。它向后续模块提供经过风控的只读直播事件、统计快照与榜单快照，但不实现后续消费者。事件、统计和安全规则的权威 owner 在 Rust；Vue 不得保留第二套可写业务真相。[source-backed: `requirement-map.md`]

## Source Grounding And Traceability

| Spec item | Grounding | 产品/业务语义 | Authority | Plan obligation |
| --- | --- | --- | --- | --- |
| LDS-01 只读连接与固定重连 | source-backed | 是 | PRD 9.1.1-9.1.7 | Rust room application service、port、状态测试 |
| LDS-02 四类事件与 EventBus 前处理 | source-backed | 是 | PRD 9.1.2 | Rust parser/pipeline、事件测试 |
| LDS-03 IdentityTag 与勋章降级 | source-backed | 是 | PRD 9.2 | 纯 mapper、DTO、组件测试 |
| LDS-04 所有敏感词命中均阻断 UI/TTS/AI 并留日志 | confirmed-decision | 是 | 用户 2026-09-14；PRD 9.3 基线 | AC/regex 引擎、日志与下游隔离测试 |
| LDS-05 敏感词管理/导入导出 | source-backed | 是 | PRD 9.3.3-9.3.7 | repository、事务、进度、页面 |
| LDS-06 黑名单入口拦截 | source-backed | 是 | PRD 9.5.2 | 内存索引与 DB 一致性测试 |
| LDS-07 黑名单 CSV `uid,username,reason[,created_at]` | confirmed-decision | 是 | 用户 2026-09-14 | codec、事务和 round-trip 测试 |
| LDS-08 看板五指标/三趋势/5 秒刷新 | source-backed | 是 | PRD 9.9 | 聚合器、query/event、图表 |
| LDS-09 图表点明细显示五指标 | confirmed-decision | 是 | 用户 2026-09-14 | tooltip/detail view-model 测试 |
| LDS-10 三榜/10 秒/TOP100/3 秒高亮 | source-backed | 是 | PRD 9.10 | 排名聚合、query/event、页面 |
| LDS-11 舰长贡献为累计购买/续费人民币金额 | confirmed-decision | 是 | 用户 2026-09-14 | 金额归一和排名测试 |
| LDS-12 系统对话框 + Rust 文件写入 | source-backed | 是 | PRD 9.11、10.1-10.3 | purpose-bound path grant、CSV 测试 |
| LDS-13 简洁中文系统反馈 | confirmed-decision | 是 | 用户 2026-09-14 | 文案 allowlist 与 UI 状态 |
| LDS-14 typed gateway 单一 native boundary | code-fact-backed | 否 | platform-foundation spec/review | 扩展既有 contract map，不旁路 transport |
| LDS-15 移除 renderer 业务 owner | code-fact-backed | 否 | scoped code reading | caller 迁移、dead dependency cleanup |

### Closed-World Product Content Allowlist

- 页面/表面：`弹幕面板`、`风控中心 -> 敏感词`、`风控中心 -> 黑名单`、`数据看板`、`排行榜`、共享 `导出数据` 弹窗、共享用户详情卡、过滤日志跳转意图。
- 控件：房间选择、连接/断开、四类弹幕筛选、搜索、暂停/继续滚动、清屏；敏感词新增/编辑/删除/启停/导入/导出/分类筛选/搜索；黑名单手动添加/移除/导入/导出/搜索；范围、刷新、TOP、三榜 Tab、导出、系统路径选择、取消。
- 状态：`disconnected`、`connecting`、`connected`、`reconnecting`、`error`；列表/图表 loading、empty、filtered-empty、stale、database-degraded、import-compiling、pending、success、error；路径对话框取消。
- 精确保留文案：`房间号无效或未开播`、`开放平台凭证失效，请重新配置`、`重连失败，请手动重试`、`已清屏，日志仍保留`、`暂无数据`、`暂无数据可导出`。其余只允许本模块所需的简洁中文校验、确认、成功和可行动错误文案。[source-backed + confirmed-decision]
- 字段/数据：本 spec “Field Display Semantics” 和 API DTO 表中列出的字段；没有列出的业务字段不得临时加入 UI、CSV 或 IPC。
- 权限：本模块免费/会员共有；除后续会员禁言入口外不做会员判断。
- 分析埋点：无。不得新增遥测或行为分析事件。
- 禁止：可配置重连间隔/次数、连接页登录字段、黑名单时长/永久封禁字段、看板最近活动、WebView 统计、任意本地路径字符串、发送相关符号。

## Workflow Efficiency Profile

- profile: `S3 broad`
- 原因：同一事件链跨 Rust 网络/领域/SQLite/Tauri Event、typed gateway、Pinia、多个页面、导入导出和性能安全边界。
- artifact density: full；downstream context: 当前模块完整 caller/consumer 闭包。
- 只有发现范围扩大到会员、媒体、应用壳或数据库 schema 变更时才回到 requirement split/spec；不得通过降级 profile 绕过架构和验证。

## Self-Healing Completion Bar

- 需求一致性：逐项核对 LDS-01 至 LDS-15，确认没有发送能力、会员裁剪或未授权 UI。
- 业务逻辑：覆盖连接状态、固定重连、过滤顺序、身份降级、CRUD/导入事务、黑名单即时生效、范围查询、聚合与排行、路径授权、CSV 空数据/权限/空间错误。
- 静态质量：Rust fmt/clippy/tests，TypeScript type-check，ESLint，Vitest，free/member web build 与 Tauri build。
- 运行时安全：监听解绑、连接/定时器/任务取消、重复命令、过期响应、数据库降级、无权限路径、窗口关闭资源回收。
- 组织质量：只保留一个 native transport、一个 Rust 业务 owner；旧 renderer WebSocket/过滤/统计/数据库 caller 完成迁移或删除。
- 可维护性：精确命名、纯 mapper/validator、显式副作用、无 `any`、无隐藏全局依赖、无一次性 helper 文件。
- 技术债 blocker：任何生产 `window.electronAPI`、renderer WebSocket/SQL/权威聚合、未解绑 listener、发送能力或测试假成功都必须在本模块内解决。
- 完成条件：所有适用 verdict 为 pass，`self-healing loop compliance: pass`、`self-healing loop assessment: pass`、remaining blockers `none`。

## User Intent Contract

- literal request: 根据指定 PRD 重构项目，并按确认建议继续。
- practical goal: 获得真实可运行、严格只读、Rust 拥有核心数据链且免费/会员一致的数据与风控底座，而不是仅换 UI 或包一层 Tauri。
- success criteria: 本模块功能可从 typed gateway 端到端运行；本地领域/数据库/页面完整；外部凭证缺失时明确降级；旧 WebView 业务链不再参与生产。
- forbidden interpretations: 在 Vue 中继续 WebSocket/过滤/统计；用模拟数据假装外部连接完成；只新增 Rust 空壳；保留发送或自动禁言；用会员状态隐藏共有功能。
- acceptable approaches: Rust ports + application services + pure domain + repositories；Tauri Commands/Events；Pinia 页面状态；测试 fake port；生产 adapter unavailable until configured。
- verification/review: 除功能断言外必须检索发送符号、Electron API、WebSocket/SQL imports、免费/会员 registry 和 event cleanup。

## User Flow

1. 用户在全局房间控件选择/新增房间并点击连接；pending 期间不能重复提交。Rust 校验 RoomId 后尝试初始化与只读 WS，状态由 `room_status_changed` 驱动。
2. 连接成功后合格事件进入弹幕面板；筛选、搜索、暂停滚动和清屏只改变本地视图。清屏不改变 Rust 日志、统计或会话总数。
3. 用户点击用户名查看 UID、等级、勋章、本场弹幕数、送礼总额和黑名单状态；加入黑名单成功后该用户现存可见项和后续事件立即隐藏。
4. 用户在风控中心管理敏感词/黑名单。持久化操作采用悲观更新；导入仅在 Rust 事务成功后替换 UI 快照。
5. 用户进入看板/排行榜选择范围。页面先 query，随后只接受相同查询上下文的 5 秒/10 秒事件；旧范围响应不能覆盖当前范围。
6. 用户打开导出弹窗，选范围/类型和系统授权路径。取消无副作用；无数据、权限或空间失败可恢复；成功由 `export_done` 确认。

## Page And Module Design

- 页面延续已批准的安静、紧凑运营台方向：平铺数据区、稳定 toolbar、表格/列表/图表、固定 footer，不使用卡片嵌套或营销式 hero。[source-backed via approved page-design]
- 应用壳最终由 `application-shell` 所有；本模块路由可以先接入现有 router，但连接状态与页面业务不得依赖临时壳实现。
- 弹幕面板为稳定高度 flex：toolbar、增长列表、footer；高并发使用有界队列和虚拟/增量渲染。
- 敏感词/黑名单为 toolbar + 单一表格 + footer，表单进入专注 dialog。
- 看板先五项指标，再三条全宽趋势；排行榜为 tabs + 单表；导出为共享 dialog；用户详情为共享 popover/drawer。

## UI / UX Design Contract

- 主动作：连接、保存规则、导出；次动作：筛选、搜索、暂停、范围、刷新；删除敏感词和移除黑名单为 destructive，必须二次确认。
- pending 只禁用 owning control；对话框 Enter 提交合法表单，Escape 取消并恢复触发器焦点。
- 实时事件不得抢焦点、移动用户滚动位置或逐条 aria announce；连接状态可使用一个 polite live region。
- loading 使用稳定 skeleton/placeholder；empty、filtered-empty、database-degraded、stale、import-compiling、permission/error 都有独立终止条件。
- 1024x680 时 toolbar 可两行紧凑换行；表格横向滚动；关键动作可见；五指标 3+2；图表最小高 280px；dialog footer 不滚出视口。
- 状态和身份必须颜色 + 文本/图标双编码；所有 icon-only 控件有 tooltip/accessible name；交互目标至少 36px。
- 成功/确认/空态文案使用已确认的简洁中文，不添加帮助面板、营销文案、快捷键说明或升级引导。

## Frontend Styling Constraints

- 新增/修改的 authored DOM styling 只使用 Tailwind CSS-style utility；不新增 scoped CSS、CSS module、Sass/Less、inline style object 或隐藏的语义 class。
- class 值必须在模板中可直接审查且不超过正常 formatter 行宽；不得把长 class 移入常量、map、computed、helper 或 import。
- 条件 class 只承载 selected/increase/decrease/paused/error 等小型状态。
- ECharts option 内的颜色属于库配置，不作为 DOM authored styling；核心数据必须有相邻文本，不依赖 canvas 单独表达。

## Field Display Semantics And Interaction Visual-State Contracts

| Field | Business meaning / source | Display | Empty/invalid | Interaction and visual states | Evidence |
| --- | --- | --- | --- | --- | --- |
| connection status | 当前 Rust 连接生命周期；PRD 9.1、8.7 | 中文状态 + 绿/黄/红语义点；重连含 `attempt/10` | 无状态视为 error，不伪装 disconnected | 连接/断开按钮按状态 enabled；focus ring；不只靠颜色 | state/component tests |
| room id | 当前房间上下文；PRD 8.7、9.1 | 原始数字字符串，不做数值化/千分位 | 未连接为空 | 选择控件可操作；状态栏只读 | validation/UI tests |
| identity tags | 房管/总督/提督/舰长/老爷；PRD 9.2 | 固定中文标签和绿/紫/蓝/金/橙语义色；普通无标签 | 缺字段降普通 | 默认即强调；hover 不能替代身份色；focus 随用户名 | mapper/UI tests |
| username | 事件主体；PRD 9.1.4-9.1.5 | 1-50 字符，控制字符已由 Rust 去除 | 非法事件不进入 UI | 默认呈可点击 affordance；hover/focus/active 明确；无 visited 语义 | adapter/component tests |
| user level | 事件用户等级；PRD 9.1.4 | `Lv.<integer>` | 缺失不显示 | 只读，无点击 | mapper tests |
| fan medal | 勋章名与等级；PRD 9.2.4/9.2.6 | `勋章名 + 等级` capsule | 等级异常只显示名称；全缺失不显示 | hover/focus tooltip 展示已批准详情 | mapper/UI tests |
| danmaku content | 合格未过滤文本；PRD 9.1、9.3 | 原文展示 | 敏感命中整条不进入 UI | 只读可选文本；不得出现替换/仅记录 selector | pipeline negative tests |
| event time | 事件发生时刻；PRD 9.1.4 | `HH:mm:ss` | 无效事件拒绝 | 只读 tabular figures | formatter tests |
| gift row | 礼物名、数量、人民币价值；PRD 9.1.4 | 名称、`xN`、`¥#,##0.00` | 缺必要字段拒绝事件 | 与普通行同网格；礼物语义色非交互 | DTO/UI tests |
| captain row | 首开/续费、总督/提督/舰长和金额；PRD 9.1.4、确认决策 | 类型 + 等级；金额 `¥#,##0.00` | 未知开通类型显示不可用错误而非猜测 | 同网格，身份默认强调 | DTO/UI tests |
| stream footer | 本场总条数、条/秒速率、过滤数；PRD 9.1.4 | 整数、至多 1 位小数 `/s`、整数 | 无事件为 0 | 过滤数为可聚焦跳转意图；默认即有 affordance | store/UI tests |
| sensitive row | 词语、分类、匹配方式、状态；PRD 9.3 | 原文；固定枚举中文；启用/禁用 | 非法数据 fail fast | toggle 悲观 pending；edit/delete 常规/危险状态 | form/table tests |
| safety counts | 总数、启用数、今日命中/新增；PRD 9.3、9.5 | 非负整数 | DB degraded 保留最近快照并标 stale | 只读；过滤数可导航 | repository/UI tests |
| blacklist row | UID、用户名、原因、添加日期；PRD 9.5 | UID 原文；日期 `YYYY-MM-DD` | 原因为空显示空白；非法 UID 不返回 | 移除危险态 + confirm；用户名不是此表的详情入口 | table tests |
| dashboard metrics | 在线、弹幕总量、弹幕速率、礼物总价值、舰长总数；PRD 9.9 | 整数；速率至多 1 位小数 `/s`；金额 `¥#,##0.00` | 无数据统一 `暂无数据`，不能用 0 冒充未知 | 默认最强数字层级；非点击 | snapshot/UI tests |
| chart point | 某一采样时刻五指标；确认决策 | `HH:mm:ss` + 五指标同上格式 | 缺点不显示 tooltip/detail | hover 与 click 均读取同一 point；键盘以相邻文本可达 | chart adapter tests |
| ranking rank | 当前榜位；PRD 9.10 | 正整数，前三名持续强调 | 无行不显示 | 前三默认强调，不是 hover-only | table tests |
| ranking metric | 送礼金额/弹幕数/舰长累计金额；PRD 9.10 + 确认 | 金额 `¥#,##0.00`；弹幕 `<n> 条` | 无数据 `暂无数据` | 只读 | aggregator/UI tests |
| ranking change | 相对上一快照的位次变化；PRD 9.10 | 上/下箭头 + 绝对位次数；不变 `-` | 新入榜标记为 new，不伪造 delta | 变动背景持续 3 秒；颜色 + 图标；不闪烁 | deterministic clock tests |
| update time | 榜单快照生成时刻；PRD 9.10.4 | `HH:mm:ss` | 无快照不显示 | 只读 metadata | formatter tests |
| export path | Rust 授权目标；PRD 9.11、10.3 | 只读 displayPath，可截断/换行 | 未选择为空 | 按钮调系统 dialog；文本不可编辑；cancel 无写入 | grant integration tests |

## Function-Complete Behavior Breakdown

### Room Connection And Live Stream

- RoomId 输入先 trim；trim 后必须是 1-20 位 ASCII 数字。blur/submit 显示校验，提交再由 Rust 重验。[source-backed: PRD 10.2; code-fact-backed normalization]
- `room_connect` 在 connecting/connected 时拒绝重复启动；成功返回当前快照并发布状态。失败按稳定错误码映射精确 PRD 文案。[source-derived technical guard preserving one-room context]
- 网络断开进入 reconnecting，固定每 5 秒尝试一次，最多 10 次。手动断开/窗口关闭取消连接、heartbeat、重连 task 和缓冲，不得再发事件。[source-backed]
- 接收 pipeline 顺序：协议解包/结构校验 -> 敏感词 -> 黑名单 -> 身份归一 -> 日志/统计/排行 -> typed event。敏感或黑名单命中不进入普通 UI/TTS/AI/统计分支；敏感命中写 filtered log。[source-backed + confirmed-decision]
- 高并发不丢日志/统计；Vue 在 animation-frame 边界批量提交有界可视队列，并优先绘制高身份事件。列表容量只限制 WebView 内存，不改变 Rust 权威计数。[source-backed + source-derived implementation]
- 筛选枚举为 all/captain/gift/highlight；搜索 trim 后按用户名或内容本地不区分大小写包含匹配；纯空搜索等于无过滤。暂停只停止自动滚动。清屏只清 view rows 并提示精确文案。[source-backed; technical normalization]

### Audience Identity And Detail

- Rust 从 `guard_level`、`is_admin`、`vip` 和 medal 字段构建不可变 identity DTO；缺失身份降普通，异常 medal level 置 `null` 但保留名称。[source-backed]
- 用户详情由 Rust 权威 session aggregates 返回：UID、等级、勋章、本场弹幕数、送礼总额、黑名单状态。Vue 不扫描列表重新计算。[source-backed + ownership constraint]
- `加入黑名单` 复用 blacklist mutation；成功后 store 按 UID 删除现存可见事件并由 Rust 阻断后续事件。会员禁言不在本模块实现。[source-backed]

### Sensitive Words

- 表单字段：word 必填文本；category 必填 closed enum `illegal`/`advertising`/`personal_attack`/`custom`；matchType 必填 `exact`/`fuzzy`；isEnabled 必填 toggle。[source-backed]
- word 移除控制字符再 trim；归一后 1-100 Unicode code points；纯空、换行控制字符清理后为空、超长均失败；输入中不打扰，blur/submit 显示错误。[source-backed PRD 10.2 + code-fact-backed normalization]
- add/update/delete/toggle 均等待 Rust 成功再刷新权威列表。删除二次确认；取消无副作用。[source-backed]
- exact 使用 Aho-Corasick；fuzzy 使用预编译 regex。规则变化在后台构建新 matcher，成功后原子替换；编译失败保持旧 matcher/数据库状态。[source-backed + source-derived atomicity]
- 导入扩展名仅允许 `.txt/.csv/.json`。codec 必须 round-trip 四个已批准字段；CSV/JSON 使用 `word,category,match_type,enabled`；TXT 使用每行一个 JSON object，避免引入未授权默认分类/匹配/状态。文件格式错误返回行号和原因；重复 word 跳过并计数；全批次事务回滚。[source-backed + source-derived serialization preserving all approved fields]
- 导出由系统 save dialog 授权并按扩展名选择同一 codec；无任意路径字符串。万级导入报告 compile progress，完成返回 imported/duplicates/failed counts。[source-backed]

### Blacklist

- add 表单字段：uid、username、reason。UID 和 username 必填；reason 未被 PRD 定义为校验前提，因此允许空值，不得新增时长/永久选项。[source-backed: PRD 9.5.2、10.2; closed-world constraint]
- uid trim 后为 1-20 位 ASCII 数字；username 去控制字符再 trim，归一后 1-50 code points；reason 仅 trim 并保留业务文本，换行压成单个空格以维持 CSV 单行记录。blur/submit 显示校验。[source-backed + code-fact-backed safe serialization]
- 重复 UID 不静默覆盖；返回 `already_exists` 和 current reason，UI 让用户明确选择更新原因或取消。更新仍由同一 mutation 完成。[source-backed]
- remove 必须确认，成功后刷新列表。搜索 trim 后按 UID/用户名本地包含匹配，纯空等于全部。[source-backed]
- import 仅接受 UTF-8 CSV header `uid,username,reason`；导出为 `uid,username,reason,created_at`。错误含行号/原因；任一不可跳过错误触发整批回滚；重复 UID 进入结果集而不自动覆盖。[confirmed-decision + source-backed transaction]
- 内存 UID set 与 SQLite mutation 在同一 application operation 内提交；DB 失败不改变内存，成功后原子替换/更新。[source-derived consistency]

### Dashboard

- DataRange closed union: `live_session`、`today`、`last_7_days`、`custom { startAt, endAt }`；custom 值为带 offset 的 RFC3339，Rust 验证 start <= end。`today` 由 Rust 以当前系统时区计算，Vue 不拼 SQL/边界。[source-backed enum + source-derived deterministic boundary]
- `stat_query(range)` 返回五指标与三条同采样点序列；`stat_refresh` 强制从内存+repository 生成同合同快照；`stat_updated` 最多每 5 秒推送当前本场快照。[source-backed]
- 每个 TrendPoint 带 timestamp、onlineCount、danmuTotal、danmuRate、giftValue、captainTotal，从而满足点击点展示五指标。[confirmed-decision]
- DB 失败时保留内存本场快照，标 `isHistoricalDataAvailable=false`/`isStale=true`，历史范围显示不可用反馈。空数据只在无有效点/汇总时显示 `暂无数据`。[source-backed]
- 页面按 range request token 接受最新响应；切换/卸载取消视图消费或忽略旧响应；manual refresh pending 去重。[source-derived async correctness]

### Rankings

- `rank_query(range)` 返回 gift/danmu/captain 三组最多 100 行及 totalUsers/generatedAt；TOP 仅是 Vue 对同一 TOP100 快照的 10/20/50/100 截取，不触发新请求。[source-backed]
- gift 按累计礼物人民币金额降序；danmu 按合格弹幕数降序；captain 按累计首开/续费人民币金额降序。相同指标按稳定 UID 升序打破并列，保证快照可重复。[source-backed + confirmed-decision + source-derived stable tie]
- `rank_updated` 每 10 秒推送；Rust 或 adapter 提供 previousRank/currentRank/delta/isNew，Vue 只表现 3 秒 highlight，不重新排序或重算 delta。[source-backed + ownership]
- Tab 切换和 TOP 切换本地完成；用户名打开同一 AudienceDetail；DB 失败保留内存榜单。[source-backed]

### Export

- shared dialog fields：range、dashboard/ranking checkboxes、readonly granted path。至少一个 type 和 grant 才能提交；pending 防重复；取消/系统 dialog 取消零副作用。[source-backed]
- `stat_export`/`rank_export` 只接受 DataRange + opaque grant id，不接受 raw path。看板 CSV 覆盖时间点和五指标；榜单 CSV 覆盖榜单类型、排名、UID、用户名、对应指标、生成时间。[source-backed fields + source-derived serialization]
- CSV 使用 UTF-8，正确引用逗号/引号/换行；金额以人民币十进制定点两位输出。数据为空返回 `no_data_to_export`；空间/权限分别映射可行动错误。[source-backed + technical correctness]
- 写入成功后发布 `export_done { exportType, displayPath }`；事件只包含展示路径，不暴露未授权路径 grant 能力。[source-backed + security constraint]

## Expert Frontend Engineering Constraints

- state owner：Rust 拥有 connection、events、safety rules、session aggregates、stats/ranks 和 persistence；Pinia 仅存页面所需快照与 view state；表单草稿由 dialog local state 所有。
- lifecycle：raw backend DTO -> Rust domain validation/normalization -> IPC DTO -> gateway contract -> feature adapter/view model -> Pinia/page/component。组件不得接收 raw Bilibili payload。
- async：每个 command 有 owning `isPending`；连接/导入/导出去重；range query latest-wins；listener 在 unmount 解绑；3 秒 highlight 使用可注入 clock 并清理 timer。
- interaction：键盘、焦点恢复、危险确认、disabled reason、dialog cancellation、权限错误和 retry 均按 UI contract 执行。
- performance：1000 events/s 下 Rust 全量处理，WebView 有界队列/虚拟列表；统计/排行只按 5/10 秒快照更新；不使用 deep watch 扫描全列表。
- evolution：Bilibili transport 由 Rust port 隔离；production port unavailable 时 fail closed。旧 renderer compatibility path 在本模块完成时删除，不永久双轨。
- observability：稳定错误码、连接 attempt、import line/count、snapshot generatedAt 和测试 fake port；日志不含 token/secret/raw authorized path。

## Frontend Architecture Design Routing Inputs

- `architecture-design_required: true`。
- 不可改变：LDS-01 至 LDS-13 的行为、字段、校验、只读限制、固定 cadence、过滤后果、页面结构和验收阈值。
- 必须决定：Rust capability 边界、Room transport port、event pipeline owner、repository 分割、AppState state topology、Tauri event payload、Pinia store 分割、gateway public API、页面/组件文件组织、导入 codec、兼容代码删除顺序。
- 复用候选：既有 `DesktopTransport`、`DesktopError`、validation newtypes、`SqliteDatabase`、path grant；现有 renderer `DanmakuManager`/`BlacklistManager`/sensitive Trie/统计方法仅作迁移事实，不作为目标 owner。
- architecture-design 若需要改变 DataRange、CSV 字段、过滤语义、状态机、用户可见文案或 acceptance，必须回滚本 spec。
- 后续必须给出 `frontend architecture quality compliance/assessment` 和 `architecture reuse compliance/assessment` 证据。

## Production Code Quality Constraints

- type-first：Rust domain types/DTO 与 TS contracts 先于实现；RoomId/UserUid/PathGrantId/SensitiveWordId 使用 nominal/newtype 区分；禁止 `any`、宽松 payload 和 unsafe assertion。
- fail-fast：非法 enum/ID/text/time/amount、unknown event、regex compile、DB/transport/path/csv error 都显式失败；不吞 Promise 或返回假成功。
- null：`null` 表示来源明确缺失，`undefined` 仅用于 TS 未提供可选输入；必须字段不得用 `??` 掩盖。medal level 可 null，medal 整体可 null。
- maintainability：虚拟/分帧/节流只因明确高并发和 cadence 使用；不增加无证据 memo/cache/deep watch。
- pure over class：validator、identity mapper、filter decision、range validator、ranking/stat aggregator、CSV row builder、DTO mapper 使用纯函数；生命周期资源可由 Rust struct/application service 所有。
- naming：布尔 `is/has/should/can`；props 回调 `on`；事件 handler `handle`；query/mutation 使用精确动词。
- no magic variables：5s/10 attempts/5s stats/10s rank/3s highlight/1000sps 等真实规则由最窄 domain owner 常量化；helper 不读取隐式 store/global。
- boundary UI states 逐页显式建模，不在 template 用偶然数组长度推断所有异步状态。

## Human Review Readiness Constraints

- 遵循现有 Vue `<script setup>`、Pinia setup store、typed gateway 和 Rust 五层边界；可替换不合规旧结构但不做相邻模块视觉重构。
- 允许改动：当前模块 Rust domain/application/infrastructure/ipc、migration 兼容性所需 repository、desktop contracts/tests、room/danmaku/data/safety stores/services/pages/components/router 接入及对应 tests。
- 禁止改动：PRD；会员、媒体、插件/更新业务；与本模块无 caller 关系的页面；平台迁移行为语义。
- review-sensitive：大范围旧 caller 删除、事件竞态、DB/内存一致性、路径授权、金额精度、free/member handler parity、1000/s 内存。
- execute 结束前必须建立 changed hunk -> plan task/test/cleanup 映射，移除 debug log、dead code、unused imports/exports、stale comments/tests/mocks 和格式噪声。
- evidence 必须包含 focused Rust/TS/component/integration tests、type/lint/build、free/member registry/build、运行时页面与内存/吞吐检查；未运行项目需说明原因与替代证据。

## Functional-Programming Constraints

- 纯函数：输入归一/校验、identity mapping、event classification、sensitive/blacklist decision、view filtering、range serialization、stat/rank aggregation、change delta、display formatting、CSV rows。
- 不可变输入：Bilibili payload、Rust domain event、IPC DTO、gateway payload、Pinia snapshot、props、form submission snapshot。
- 副作用仅在 Rust transport/repository/application command/event publisher、gateway invoke/listen、Pinia action、Vue lifecycle/event handler。
- 语义归一只在 Rust adapter/domain mapper 和前端 feature adapter；不得在 template/computed/watch 临时修 raw DTO。
- 统计、filtered count、rank rows 等 derived state 不再作为多处 writable truth。
- 拒绝新增函数式工具库、point-free/curry/pipeline ceremony；使用可读的普通函数和 iterator/computed。

## Architecture Reuse And Shared Ownership Constraints

| Candidate | Existing use sites / semantics | Anti-DRY result | Architecture question |
| --- | --- | --- | --- |
| desktop invoke/listen/error | 所有 native 调用，同技术语义，2+ | 复用；infrastructure common | 扩展 contract map 与 listener cleanup API |
| RoomId/UserUid/text validation | Rust 已有 newtypes，多命令同领域语义 | 复用/扩展；domain common | reason/time/range validator owner |
| path grant | 多类导入导出，同安全语义但 purpose 不同 | 复用策略；infrastructure common | file vs directory/save semantics、purpose enum |
| renderer event parsing/filtering | 旧 client/manager/parser 多处混合副作用 | 不共享旧实现；迁入 Rust 后删除 | migration sequence and tests |
| data range control | dashboard/ranking/export，3 个同语义 caller | shared UI/domain view model | public props/emits and serialization owner |
| audience identity/detail | danmaku/ranking 2+ 同语义 | shared feature component | normalized VM and store owner |
| stat/rank formatters | 金额/时刻/计数部分同形但业务精度明确 | 仅稳定同语义 formatter 共享 | shared display helper vs feature-local |
| sensitive/blacklist import | 都有事务与行错误，但 schema/duplicate policy 不同 | pipeline 可复用，codec/rule 保持分离 | generic transaction runner boundary only |

新晋升 shared API 必须有精确 domain owner、无反向 feature 依赖、focused tests，并在非显然处用 JSDoc `@see`/`@example` 追溯调用语义。最终抽取/保持分离决定由 architecture-design 完成。

## Locality And File-Extraction Constraints

- 页面单用的 filter/search/confirmation helper 保持 owning page 内。
- Rust/TS DTO adapter 因合同隔离和独立测试可使用 feature-local private file。
- 多页面 DataRange、Audience VM、desktop contract 使用 shared domain/feature owner；禁止 `utils.ts`/`helpers.ts` catch-all。
- 单调用方但具备独立虚拟列表生命周期、表单焦点生命周期或图表资源生命周期的组件可单独 SFC；否则保持 inline。
- 任何新文件都必须在 plan 写明真实 caller、边界理由和同文件为何更差。

## Vue Component Extraction Constraints

| Candidate | Decision | Callers | Contract / owner | Verification |
| --- | --- | --- | --- | --- |
| `LiveEventStream` | extract，单 caller 性能/滚动生命周期例外 | 1 | immutable rows；own viewport/pause scroll；emit user select | batching/scroll/focus |
| `AudienceIdentity` | extract shared | 2+ same | normalized identity props；optional user intent emit | tag/fallback/a11y |
| `AudienceDetail` | extract shared | 2+ same | detail VM；emit add-blacklist/close；store owns command | focus/permission |
| `DataRangeControl` | extract shared | 3 same | value + update/custom payload；不发 request | validation/keyboard |
| `DataExportDialog` | extract shared | 2 same | local form/focus；emit submit/cancel；page/store owns command | grant/pending/focus |
| `StatMetric` | reuse/adapt `StatCard` only if stripped to presentational contract | 5 same | label/value/unit/status props | dimensions/empty |
| `RankingTable` | extract，单 page 的三榜同语义与高亮测试边界 | 1 page | rows + user select；不 query | TOP100/highlight |
| `SensitiveWordDialog` | extract，表单/focus lifecycle 例外 | 1 | form snapshot + submit/cancel | validation/pending |
| `BlacklistDialog` | extract，duplicate branch/focus lifecycle 例外 | 1 | form snapshot + add/update/cancel | validation/duplicate |
| toolbars/footers | keep inline | 1 each | page-owned | no pass-through SFC |

组件只接收 view model，不接收 raw Rust/Bilibili DTO；不用 slots；所有请求、adapter、权限和跨区编排留在 page/store。

## Design Constraints

- 责任边界：Tauri command 只做输入/错误映射；application service 编排；domain 纯规则；infrastructure I/O；Vue gateway transport；store server snapshot；page interaction；component presentation。
- domain language 固定使用 room、live event、identity tag、sensitive word、blacklist、live stat、ranking、path grant；不沿用旧 `ban`/duration 把黑名单误写成临时禁言。
- side effects 不进入 computed/template/formatter；订阅有明确 owner 和 unlisten。
- complexity guard：每条 workflow 一个入口；状态用 discriminated union/state table 而非散布 booleans；禁止 manager/factory/handler class ceremony。
- schema 以已完成的 PRD 21-table SQL 为准；本模块不得悄悄改表名/字段语义。确需 migration 必须在 architecture/design 说明兼容和 rollback。

## Project Bootstrap And Scaffold Decision

本模块不新建项目/包/入口，复用已完成的 Tauri v2 + 现有 Vue/Vite scaffold 与 `src-tauri` 五层结构。不得重新运行 scaffold、另建前端或引回 Electron。允许偏离仅限增加本模块 Rust/TS 文件、依赖和 Tauri capability/handler 条目。[code-fact-backed: platform-foundation]

## Change Axes And Pattern Decision

- signals：Bilibili contract adaptation、按连接状态行为、事件处理顺序、跨模块通知、多 repository side effects、路径访问控制、range request dedupe、三类排名策略。
- depth: Level 2 为主；必要的跨语言 event pipeline/state topology 由 architecture-design 判断是否达到 Level 3。
- selected: reuse existing typed Facade/Adapter (`DesktopTransport`)、adapt lightweight State discriminated union、explicit pipeline、Observer/Pub-Sub via Tauri Event、strategy map for ranking/codec。
- frontend syntax：types/discriminated unions、plain functions、typed records、store actions、adapter/mapper、component boundaries；Rust 使用 enum/trait/struct 与 application service。
- rejected：继续 `EventEmitter` class singleton（WebView owner 错误且 lifecycle 隐藏）；一个巨型 Pinia store（混合 server/view/form state）；通用 repository/manager/factory 层（变化轴不同且审查跳转过多）。
- lightest-fit：复用已批准 gateway，只在真实 variation/side-effect boundary 引入轻量模式；若 architecture-design 发现只有单一稳定 variant，应降回 direct function。
- overbuilt signal：单实现 trait 只做转发、单 caller wrapper、DTO 被多次无语义映射、状态分支比业务状态更多。

## Code Context And Impact Assumptions

- `stores/danmaku.ts` 当前在 WebView 统计/排行并持有可写 truth；`dashboard/index.vue` 使用 timer/deep watch 重算；必须被 Rust snapshot 替代。[code-fact-backed]
- `DanmakuManager`/`BilibiliClient` 在 renderer 使用 WebSocket、指数退避 3 秒起步、Node `EventEmitter`；与 PRD 固定 5 秒和 Rust owner 冲突。[code-fact-backed]
- 当前 blacklist 在 emit danmaku 后才检查，已造成黑名单内容先进入 UI；必须修正 pipeline 顺序。[code-fact-backed]
- 当前 sensitive helper 使用 Trie 并且调用结果未阻断 UI；必须以 Rust AC/regex 规则替换。[code-fact-backed]
- `database-service.ts` 仍依赖 `window.electronAPI`，不能作为新页面/服务依赖。[code-fact-backed]
- 当前 dashboard recent activity、gift rank bar 和 blacklist duration 字段未经 PRD 授权，应在迁移闭包中删除。[source-backed closed-world + code-fact-backed]
- TypeScript governed by `packages/bilibili-live-tool/tsconfig.eslint.json` -> `tsconfig.json`: strict、noUnused、bundler resolution、ES2020、DOM、`@/*` aliases；相关声明为 `vite/client`、Vue SFC、auto-import/components declarations 和 gateway contracts。

## API And Data Contracts

### Contract Source And Boundary

- authority: PRD command/event names和字段表；Rust serde DTO 是 backend-owned contract；TS `platform/desktop/contracts.ts` 是镜像消费合同并由 contract tests 防漂移。
- 无 protobuf/OpenAPI/generated TS source。Bilibili upstream payload 仅在 Rust infrastructure adapter 内存在；Vue 永不消费。
- external Bilibili endpoint/signing/credential distribution 缺完整部署合同。production adapter 必须 fail closed；domain/application/tests 使用 injected fake port，不得把 fake 注册为生产成功路径。

### Commands

| Command | Request | Result | Error/empty semantics | Source |
| --- | --- | --- | --- | --- |
| `room_connect` | `{ roomId: RoomId }` | `RoomSnapshot` | invalid/not-live/credential/transport/already-pending | PRD explicit |
| `room_disconnect` | none | `RoomSnapshot(disconnected)` | idempotent when already disconnected | PRD explicit + technical idempotency |
| `audience_detail_query` | `{ roomId, userUid }` | `AudienceDetail` | not found/DB degraded | technical command for PRD detail card |
| `sensitive_word_list` | `{ category?, search? }` | `SensitiveWordSnapshot` | empty list valid | technical query for PRD page |
| `sensitive_word_add/update/delete/toggle` | typed IDs/form payload | authoritative row/snapshot or delete ack | validation/duplicate/compile/DB | PRD explicit |
| `sensitive_word_import/export` | `{ grantId, format }` / typed export request | `ImportResult` / `ExportResult` | line errors, rollback, no data/path errors | PRD explicit |
| `blacklist_list` | none | `BlacklistSnapshot` | empty valid/DB unavailable | technical query |
| `blacklist_add` | `{ uid, username, reason }` | `BlacklistMutationResult` | duplicate offers update branch | PRD explicit |
| `blacklist_remove/export` | typed uid/grant | ack/export result | not found/no data/path | PRD explicit |
| `blacklist_import` | `{ grantId }` | `ImportResult` | line errors/duplicates/rollback | technical name for source-backed import |
| `stat_query` | `{ range }` | `StatSnapshot` | empty/degraded flags | PRD explicit |
| `stat_refresh` | `{ range }` | `StatSnapshot` | deduped/degraded | PRD explicit |
| `stat_export` | `{ range, grantId }` | `ExportResult` | no data/space/permission | PRD explicit |
| `rank_query` | `{ range }` | `RankingSnapshot` | empty/degraded | PRD explicit |
| `rank_export` | `{ range, grantId }` | `ExportResult` | no data/space/permission | PRD explicit |

### Events

| Event | Payload obligation | Cadence/lifecycle | Source |
| --- | --- | --- | --- |
| `room_status_changed` | status, roomId?, attempt, errorCode? | every transition | PRD explicit |
| `danmu_received` | normalized `DanmuEventDto` | qualified event, <100ms | PRD explicit |
| `gift_received` | normalized `GiftEventDto` | qualified event | PRD explicit 9.7 and “等事件” |
| `captain_received` | normalized `CaptainEventDto` | qualified event | technical name for PRD CaptainEvent |
| `enter_received` | normalized `EnterEventDto` | qualified event; no required row UI | technical name for PRD EnterEvent |
| `sensitive_word_compile_progress` | percentage, stage | import/rule rebuild only | technical event for source-backed progress |
| `stat_updated` | `StatSnapshot(live_session)` | <=5 seconds while connected | PRD explicit |
| `rank_updated` | `RankingSnapshot(live_session)` | <=10 seconds while connected | PRD explicit |
| `export_done` | exportType, displayPath | successful write only | PRD explicit |

### Core DTO Semantics

- `RoomSnapshot`: status union, roomId nullable, roomName nullable, onlineCount nullable, reconnectAttempt 0-10, lastErrorCode nullable.
- `IdentityTag`: closed enum admin/governor/admiral/captain/vip; normal is empty tag list. `AudienceIdentity` includes `userUid`、username、userLevel nullable、medal nullable、tags readonly array.
- `LiveEventDto`: discriminated union by `danmu|gift|captain|enter`; every variant has eventId、roomId、occurredAt、audience. Money uses integer minor units in domain/IPC and formats as CNY only at adapter/display boundary.
- `ImportResult`: importedCount、duplicateCount、errorCount、errors `{ line, code }`、isRolledBack. No raw file path.
- `StatSnapshot`: range、metrics、trendPoints、generatedAt、isHistoricalDataAvailable、isStale.
- `RankingSnapshot`: range、gift/danmu/captain readonly arrays、totalUsers per list、generatedAt、isHistoricalDataAvailable、isStale.
- `DesktopError` continues platform shape `{ code, message, details?, retryable }`; details cannot contain secret/token/password/path.

### SQLite Ownership

- `t_room_config`: room selection metadata; `t_sensitive_word`: word rules; `t_black_user`: blacklist; `t_event_log`: event/filtered evidence; `t_live_stat`: aggregates; `t_ranking_archive`: historical snapshots.
- Repository operations use explicit column lists and SQLx; no UI SQL. Existing schema is authoritative. In-memory indexes load after bootstrap and only swap after successful DB transaction.

## Frontend / Server Responsibility Boundary

- Vue owns view filter/search/tab/TOP/pause/clear、form draft、focus、loading/empty/error、chart/list rendering and command invocation.
- Rust owns validation、Bilibili HTTP/WS/heartbeat/reconnect、protocol parse、identity/filter/blacklist pipeline、event log、stats/ranks、DB、CSV、path authorization、event cadence and resource cleanup.
- shared contract: command/event names、DTO/enums、DesktopError codes、money/time representation、DataRange、degraded flags.
- external pending: production Bilibili credential provisioning and final endpoint/signing contract. It does not block injected port/domain/UI, but blocks claiming verified production live connectivity.
- adapter boundary: upstream payload -> Rust domain; Rust DTO -> frontend view model. No component-level raw-field fallback.

## Context And Dependency Sources

- PRD 0.1、2.2、4.2、8.6-8.8、9.1-9.3、9.5、9.9-9.11、10.1-10.3、12-15。
- request/analysis/snapshot/requirement-map/current module/page-design artifacts。
- current source chain：renderer room/danmaku stores、danmaku/database services、Bilibili client/protocol/types、danmaku parser/blacklist/sensitive helper、dashboard/blacklist/danmaku views、desktop gateway、Rust validation/database/ipc。
- toolchain：Vue 3、Pinia、Vue Router、Element Plus、Tailwind-style utilities、ECharts、Tauri v2、Rust 1.77.2、Tokio、SQLx SQLite。

## Edge Cases

- RoomId leading/trailing whitespace, Unicode digits, >20 digits, duplicate connect, disconnect during connect/reconnect, app close during reconnect.
- empty/partial/corrupt WS frames, unknown events, upstream missing identity/medal fields, invalid money/timestamp, repeated event IDs.
- sensitive duplicate, invalid regex, compile failure, 10k import, malformed UTF-8/row/JSON, mid-transaction failure, matcher swap race.
- blacklist duplicate UID, empty reason, malformed CSV, removal during event arrival, DB failure after validation.
- clear/pause during burst, filter/search with new events, queue cap, page unmount/listener cleanup.
- range rapid switching, custom reversed range, no active session, DB unavailable, stale query response, equal ranking values, entrant/leaver delta.
- path dialog cancel, revoked/stale/wrong-purpose grant, symlink/traversal, disk full, permission denied, write interruption, CSV quote/newline.
- free/member builds expose identical common commands/events and no membership gate on pages.

## Acceptance Criteria

1. Rust tests prove valid/invalid RoomId, exact 5-second bounded reconnect state sequence, cancellation and one active connection; production unavailable adapter never reports connected.
2. Protocol/domain tests prove all four event variants, missing identity downgrade, medal fallback, duplicate event handling and money/time validation.
3. Pipeline tests prove every sensitive hit is absent from UI/TTS/AI/stat paths and logged filtered; every blacklisted event is absent from display/speech/stat paths; no send symbol or network request exists.
4. Sensitive tests prove exact AC/fuzzy regex behavior, CRUD/toggle, 10k rule compile, duplicate counting, line error and transactional rollback, with matching latency <10ms for acceptance fixture.
5. Blacklist tests prove UID/username validation, immediate UID-set enforcement, duplicate-update branch, UTF-8 CSV schema/round-trip, rollback, 10k records and match response <5ms for acceptance fixture.
6. Stats tests prove five metrics/three trends, 5-second cadence, 24-hour persistence fixture, current-memory fallback and 100% expected aggregate equality.
7. Ranking tests prove three metric meanings, stable tie, TOP100, delta/new entry, 10-second cadence and 3-second UI highlight using fake clock.
8. Export tests prove only purpose-matched grants work, dialog cancellation writes nothing, CSV opens/parses as UTF-8, all approved columns round-trip, and no-data/space/permission errors map correctly.
9. Vue tests prove filter/search/pause/clear semantics, precise field displays, dialogs/confirmations/focus restoration, latest-range-wins, pending dedupe, listener cleanup and responsive no-overlap at 1280x800/1024x680.
10. Source scans prove only `platform/desktop/transport.ts` imports Tauri APIs; scoped production has no `window.electronAPI`, renderer WebSocket/SQL/authoritative aggregation, blacklist duration UI or unapproved recent-activity UI.
11. Both free/member builds register and render all common commands/pages without membership checks; Rust fmt/clippy/tests, TS type-check, ESLint, Vitest, web and Tauri builds pass.
12. Runtime/performance evidence proves qualified event display <100ms under controlled fixture, 1000 events/s processing without lost log/stat counts, smooth bounded WebView rendering, connection/resource cleanup, and no unbounded memory growth.
13. Verification records all constraint-model applicable pass verdicts; review records all matching assessment verdicts and no blocker.

## Human Review And Handoff

- Review starts from LDS source IDs, then traces Rust port -> application/domain/repository -> IPC -> gateway/store -> page/component -> tests.
- Non-obvious reviewer notes must explain event ordering, memory/DB atomic swap, reconnect cancellation, amount units, latest-range-wins and compatibility deletion.
- Handoff explicitly separates “本地完整并通过 fake/integration contract” from “生产 Bilibili 连接已验证”；后者只有真实部署凭证/合同到位后才能声明。
- 此模块 review 通过后才推进 `interaction-media`，并把合格 `gift/captain/danmu` event contract 作为下一模块只读输入。

## Risks

- PRD 要求的 Bilibili 生产连接依赖未完整配置，可能只能验证 port/contract/unavailable state。
- 当前 renderer 链同时服务媒体功能；删除旧 manager 前必须把后续模块所需事件消费切到新 gateway，或用明确的短期 adapter 且设删除触发器。
- 1000 events/s 与逐条 Tauri event 可能有序列化压力；architecture-design 必须在不改变事件语义和 <100ms 的前提下决定 batch boundary。
- SQLite 当前 aggregate schema 粒度可能不足以表达全部趋势点；若需 migration，必须显式设计兼容、备份和 rollback，不能重解释现有字段。
- 来源未提供生产 endpoint/credential distribution；不得用现有 Electron secret flow 作为已批准安全合同。
