# Tauri / Rust 重构需求分析

## Analysis Summary

这是一次 S3 全量迁移，不是替换构建脚本。产品范围由 PRD 的 20 项功能闭合；技术主线是把特权 I/O 和高负载规则迁入 Rust，同时保留现有 Vue 页面可复用资产。免费/会员必须同时在构建时和运行时隔离。外部会员服务与第三方二进制尚未提供，但产品行为已足够稳定，可通过 adapter、配置、feature 和受控 unavailable 状态继续拆分与实现。

## User Intent Contract

- stated request：根据指定 PRD “重构项目”。
- practical goal：现有 Electron/Node 架构需要真正迁到 PRD 指定的 Tauri/Rust 边界，并获得资源、安全、隔离、双包和可维护性收益。
- success criteria：生产源码/构建不再依赖 Electron；Rust 接管核心能力；两种包和会员状态符合矩阵；20 项功能、迁移、安全与测试均有可观察结果。
- forbidden interpretations：只改品牌/UI；在 Tauri 外壳内继续运行 Electron/Node 主进程；用前端 mock 宣称 Rust 核心完成；把发送弹幕换名后保留；免费包仅 CSS 隐藏会员入口；伪造真实支付或签名服务。
- acceptable approaches：复用现有 Vue/Vite UI，以官方 Tauri existing-frontend 初始化为 scaffold；按垂直功能模块逐步迁移；外部接口采用显式 port/adapter 和 unavailable 状态。
- downstream checks：spec/plan 必须列出 Electron 删除闭包、Rust ownership、构建 feature、安全门禁和回归证据；verify/review 同时检查字面迁移与实际职责迁移。

## Source Inventory And Reading Stance

| 来源 | 用途 | 阅读状态 |
| --- | --- | --- |
| 2026-09-14 PRD | 唯一产品范围、行为、状态、验收权威 | 全文读取 |
| PRD 9.1-9.20 | 20 个功能的表单/列表/交互/异常 | 全文读取 |
| PRD 10-15 | IPC、Sidecar、数据库、降级、隐私 | 全文读取 |
| 当前 package、Vite、TS config | 工具链和类型边界事实 | 已读取 |
| 当前 main/preload/App/router/stores/services/tests | 迁移影响与兼容事实 | 关键入口与引用扫描已读取 |
| `description-doc/` B 站开放文档 | 外部协议参考，不授权新增产品行为 | 后续连接模块按需读取 |
| Tauri v2 官方 create-project | existing frontend scaffold 可行性 | 已读取 |

优先级：用户当前 PRD > PRD 内显式 SQL/矩阵/流程 > 仓库代码事实 > 历史实现文档。代码与历史只约束兼容和迁移，不扩展产品范围。用户未提交的 PRD 保持只读。

## Source Grounding Matrix

| 项目 | 标签 | 来源/事实 | 下游处理 | 可下传 |
| --- | --- | --- | --- | --- |
| Tauri/Rust/Vue/Sidecar 架构 | source-backed | 标题、2、3 | 所有模块架构约束 | 是 |
| 20 项产品功能 | source-backed | 7、9 | 拆成顺序交付模块 | 是 |
| 双包与会员状态 | source-backed | 0、4、5、8 | build feature + runtime state | 是 |
| 只读、无发送能力 | source-backed | 0、2、9.1、9.14 | 代码/命令/网络负向审计 | 是 |
| SQLite 21 表及索引 | source-backed | 12 中显式 SQL | 以 SQL 清单而非“17 表”叙述为准 | 是 |
| 当前 Electron IPC/任意路径 API | code-fact-backed | main/preload/renderer 引用 | 迁移与删除闭包 | 是 |
| 现有 Vue/Vite/Pinia/Element Plus | code-fact-backed + source-backed | package/tsconfig 与 3.1 | 复用现有前端 | 是 |
| existing-frontend Tauri scaffold | code-fact-backed | 官方文档 + 无 sibling | spec 选型候选 | 是 |
| 会员中心预付费壳层内置 | source-derived | 4.3/8.2 要求支付前进入；5.2 又列下载模块 | 内置最小壳，下载资源只能扩展激活态 | 是 |
| OAuth/支付/会员服务具体协议 | missing-source | PRD 仅有流程名 | external interface port/config | 仅边界 |
| Cookie 禁言接口协议 | missing-source | 9.4 明示非开放 API | 默认关闭/unavailable | 仅边界 |
| FFmpeg/whisper 实际二进制与签名 | missing-source | PRD 仅给来源/许可 | 构建资源待补 | 仅边界 |
| 任何发送弹幕、云同步、远控 | out-of-scope | PRD 明示禁止或未授权 | 不创建 | 否 |

## Closed-World Product Scope Allowlist

- 功能项严格为 `prd-snapshot.md` 的 1-20；每项字段、按钮、状态、默认值、校验、异常和验收只来自 PRD 对应 9.x。
- 全局可见内容只允许第 8 节的 toolbar/nav/status bar 与第 9.20 节界面设置。
- 构建/权限只允许第 4、5、10、11 节矩阵。
- 数据语义只允许第 12-15 节 SQL、保留、降级与隐私范围。
- 未列在上述章节的产品可见行为不得由现有代码、Element Plus 惯例或 Tauri scaffold 自动引入。

## Greenfield Scaffold And Bootstrap Analysis

- 触发原文：重构为“Tauri 框架 + Rust 核心 + 前端 WebView + Sidecar”。
- owner：`packages/bilibili-live-tool`，桌面目标 Windows/macOS/Linux；pnpm monorepo。
- 前端约束：Vue 3、TypeScript strict、Vite、Pinia、Vue Router、Element Plus、Tailwind、ECharts、vue-i18n。
- 后端约束：Rust stable/Tokio/Tauri v2/SQLite；测试 cargo + frontend unit + Playwright/tauri-driver。
- 仓库发现：无 Tauri/Cargo、无同类型 sibling、无 generator；现有 Vue package 是必须复用的基线。
- candidate A：Tauri v2 CLI 对现有前端 `init`。匹配度最高，不复制前端；进入 spec。
- candidate B：`create-tauri-app` Vue template。拒绝：会创建第二份 Vue/Vite bootstrap 并增加无来源的目录/配置合并。
- candidate C：手写 `src-tauri`。拒绝：官方 scaffold 可用且手写易遗漏 capability/schema/build hook。
- 人工确认：不需要；A 不改变 PRD 技术栈或产品语义。

## Confirmed Scope

- Tauri 桌面壳、Rust 核心、typed IPC/Event、Capabilities、SQLite/migration、Sidecar manager。
- 20 项功能对应 UI、应用服务、领域规则、持久化和故障状态。
- 免费/会员双构建与会员模块生命周期。
- 旧 Electron 数据与配置迁移、Electron 依赖删除闭包。
- 安全、隐私、合规、性能、更新与测试基线。

## Out-of-scope And Non-goals

- 自建和部署 OAuth、支付、会员签发、模块 CDN、更新 CDN 服务端。
- 提供或抓取用户 Cookie；任何未被 PRD 明确授权的平台写操作。
- 重新设计 PRD 之外的营销站、云端分析或社交能力。
- 为顺手修复而改写无关 homepage package。

## Confirmed Decisions And Invariants

1. 任何环境、包型、会员状态下都没有发送弹幕 Command、按钮、定时任务或网络调用。
2. 免费版在编译边界排除会员 Command/鉴权/下载/AI/自动禁言逻辑，不能只靠 UI 隐藏。
3. 会员版未激活仍完整拥有免费底座；到期只停会员服务，模块和模型保留。
4. 数据统计、排行榜、CSV、历史与保留策略对两包一致。
5. 前端不直接访问文件、数据库、密钥、Sidecar；所有特权操作由 Rust 校验。
6. effect window 无 Command 权限，仅接收事件。
7. 会员版内置最小会员中心壳以完成登录/支付；下载模块可扩展激活态资源，但不能成为首次付费前置条件。
8. SQL 显式列出的 21 张表优先于文档中的数量叙述。

## System Boundaries And Ownership

| 系统 | 所有权 | 边界 |
| --- | --- | --- |
| Vue WebView | 本仓库 | 展示、输入、局部 UI 状态、调用 typed gateway |
| Tauri/Rust app | 本仓库 | 命令校验、流程、规则、I/O、持久化、Sidecar、事件 |
| SQLite/Keyring/文件系统 | Rust 基础设施 | 前端和插件不可直连 |
| B 站开放平台/WebSocket | 外部只读依赖 | 获取/接收事件，不提供发送链 |
| Cookie 管理接口 | 外部不稳定写依赖 | 仅会员、默认关闭、风险确认、失败降级 |
| 会员/OAuth/支付/模块/更新服务 | 外部服务 | 本仓库仅定义 client port、DTO、错误与状态消费 |
| FFmpeg/whisper | 外部二进制 | Rust 管生命周期；受许可、哈希和构建资源约束 |
| WASM 插件 | 非信任扩展 | 独立实例，只能调用宿主白名单 |

## Frontend And Server Responsibility Split

这里的“服务端”包含本地 Rust 后端和远程平台两层。

| 范围 | Vue 前端 | 本地 Rust | 远程/外部 | 状态 |
| --- | --- | --- | --- | --- |
| 房间连接 | 房间输入/状态/错误 | 校验、HTTP、WS、重连、解析 | B 站初始化/WS | 可实现 |
| 过滤/统计 | 展示、筛选、图表 | 规则、聚合、落库、推送 | 无 | 可实现 |
| 文件/录制 | 对话框触发、状态、列表 | 路径授权、Sidecar、配额、文件操作 | FFmpeg binary | 资源待补 |
| AI/ASR | 配置、上下文选择、流式预览 | 会员门禁、上下文、LLM 代理、Sidecar | LLM/whisper source | adapter 可实现 |
| 会员 | 页面、支付/下载状态 | token 验签、状态机、模块管理 | OAuth/支付/manifest | 外部接口待补 |
| 自动禁言 | 风险确认、告警、日志 | 规则、会员门禁、调用 port | Cookie API | 外部接口待补 |
| 插件 | 管理 UI/插件面板容器 | 验签、权限、WASM host | 插件包 | 本地可实现 |

共享 contract 必须定义 snake_case Tauri command、serde camelCase/显式 rename 策略、DTO nullability、error code、event payload、取消/重试/幂等语义。当前不存在远程 OpenAPI/IDL，不得猜生产字段。

## Source-backed Behavior Inventory

| 簇 | 表单/字段 | 操作/状态 | 验收重点 |
| --- | --- | --- | --- |
| 连接/弹幕 | roomId、身份/勋章/内容/时间、筛选、搜索 | connecting/connected/reconnecting/disconnected；暂停/清屏 | <100ms、1000/s、无发送 |
| 敏感词/黑名单 | 词/分类/匹配/状态；UID/用户名/原因 | CRUD、toggle、import/export、事务回滚 | <10ms/<5ms、万级 |
| 刷屏防护 | 自动开关、时长、频率/重复/相似阈值、告警 | 首次风险确认、禁言/忽略、到期降级 | 默认关、检测 >95% |
| 特效/礼物 | 模板/时长/位置/配色/音效/音量；阈值/通知类型 | preview/test/save/reset、队列/连击 | <1s/<500ms、DPI |
| 看板/榜单/导出 | 时间范围、五指标、趋势、三榜、TOP、CSV 类型 | 5s/10s tick、3s 高亮、系统路径 | 共有底座、准确、空态 |
| TTS | engine/voice/rate/volume/content/identity/template | enable/test/save/pause/clear、降级 | <2s、优先级、变量校验 |
| 录制 | format/quality/path/segment/auto/highlight/quota/watermark/source | start/stop/mark/preview/rename/delete/export | <3s、CPU、水位、LGPL |
| AI/ASR | provider/key/model/temp/length/persona/style/prompt/filter/context；模型许可 | model download、start/stop、generate/regenerate/copy/clear | 用户触发、仅预览、5s |
| 会员 | account/plan/order/QR/progress/status/expiry/modules | login/purchase/cancel/renew/manage/orders/logout | 5min install、1min restore、7d offline |
| 系统 | config scopes、update metadata、plugin permissions、locale、theme/layout | import/export/reset/update/plugin CRUD/language/UI save | 加密、回滚、沙箱、即时 i18n |

## Business-semantic Field And Interaction Inventory

| 可见项 | 业务含义 | 显示/交互后果 | 确定性 |
| --- | --- | --- | --- |
| 连接状态点 | Rust 房间连接状态 | 绿/黄/红分别 connected/reconnecting/disconnected | confirmed |
| 弹幕时间 | 当前会话事件到达时间 | `HH:mm:ss`，缺失/非法不能伪造当前时间 | confirmed |
| 身份徽章 | B 站角色/舰长等级 | 固定标签和颜色；缺失降普通，勋章等级异常只显示名 | confirmed |
| 排名变动 | 相邻刷新周期的名次差 | 上/下/不变，变动高亮 3 秒，不等于长期状态 | confirmed |
| 会员到期时间 | 已验签 token 的有效期 | 激活态显示日期，到期显示续费；不可由本地 DB 自报 | confirmed |
| 模块进度 | 每个 manifest artifact 与总下载完成度 | 下载中不可误报 installed；哈希失败删除损坏临时文件 | confirmed |
| 录制状态/大小/磁盘 | Sidecar 与实际文件系统状态 | 录制中实时更新；空间阈值触发停止而非仅警告 | confirmed |
| AI 结果 | 用户主动请求的本地建议 | 明示“仅本地预览，不发送”；敏感命中不显示成功结果 | confirmed |
| 外部服务 unavailable | 缺少/失败的远程契约 | 保留本地输入与免费底座，展示可重试/不可用，不伪造成功 | source-derived-safe |

## Developer-facing Behavior Details

- 输入校验：room/UID 1-20 数字；username 1-50 去控制字符；word 1-100；volume 0..1；threshold 非负；range/plan/module/template variable 枚举白名单。
- 所有 file picker 由 Tauri dialog 发起；Rust 保存授权路径/作用域，业务 command 接受 path token/config key 而不是任意 path。
- import 操作先解析和全量校验再事务提交；重复项按 PRD跳过/更新口径；失败保持旧数据。
- 长连接和 Sidecar 必须具有显式 cancellation/cleanup；断开、窗口退出、到期与错误均释放任务、进程和 listener。
- 高频事件先在 Rust 过滤/聚合，前端按帧/批次消费；重要身份优先展示不能导致低等级事件从持久化统计中静默丢失。
- 所有 async 页面至少区分 loading/empty/success/error/unavailable；文案仅采用 PRD 已给出的语义。
- member command 在 free feature 下不存在；member feature 中每次敏感调用重新校验 token/module/risk/config。

## Implicit Requirements And Hidden Assumptions

| 来源线索 | 隐藏问题 | 当前口径 | 分类/影响 |
| --- | --- | --- | --- |
| 两包同数据目录 | schema 能否互读 | 公共 schema 永远可读；会员表存在也不能让免费包加载能力 | confirmed technical consequence |
| 前端动态模块 | WebView 能否直接执行任意下载 JS | 下载资源必须签名并受本地协议/CSP；不能任意 eval | security source-derived |
| 支付前会员中心 + 下载会员中心模块 | 首次支付页面归属 | 内置 minimal shell；下载资源扩展激活态 | non-blocking normalization |
| 高并发优先高等级 | 是否丢低等级事件 | 只允许 UI render priority，统计/日志按过滤规则完整处理 | source-derived-safe |
| 离线 7 天 | 时间回拨处理 | token 验签之外需单调/last_verified guard；具体服务端策略待接口 | external-interface-pending |
| 自动回滚更新 | Tauri updater 是否原生提供 | 需要自有健康标记/上一版本策略或部署能力；不默认宣称 | external-interface-pending |
| WASM 注册 UI panel | 下载模块如何提供 UI | 宿主提供受限声明式/沙箱容器，不能授予主 WebView 权限 | architecture signal |

## Workflow And Dependency Assessment

依赖顺序必须是：Tauri scaffold/typed gateway/配置与 DB -> connection/event/domain -> filtering/statistics -> notification/audio/recording -> UI shell/pages -> membership/build gating -> AI/moderation modules -> plugin/update/migration -> integrated verification。会员、AI、自动禁言不能先于 build feature、token state 和 module manager；页面重构不能先于共享 DTO/error/event 约束。

## Ambiguity And Clarification Register

| 项目 | 影响 | 处理口径 | 分级 |
| --- | --- | --- | --- |
| “17 张表”与 SQL 21 表 | migration/schema | 以显式 SQL 为准；记录文档缺陷 | 非阻塞 |
| `modules/<module_name/<version/` | 路径 | 按明显 Markdown 括号错误归一为两级目录 | 非阻塞 |
| 会员中心又是预付费入口又是下载模块 | build/module | 内置支付壳，下载包不作为首次付费前置 | 非阻塞 |
| OAuth/支付/manifest DTO | 真实会员流程 | port + config + unavailable；上线前补正式契约 | 外部接口待补 |
| Cookie 管理 API | 自动禁言 | 默认关闭，接口不存在即本地告警 | 外部接口待补 |
| FFmpeg/whisper artifact | 打包/运行 | manager 与校验可实现，真实录制/ASR 打包待资源 | 外部接口待补 |
| 性能机器规格 | 数字可复现性 | 保留 PRD 场景；补记录测试机器，不改变阈值 | 非阻塞 |

## Human Confirmation Decisions

- 当前无必须阻塞 requirement splitting 的产品决定。三个文档冲突均有唯一保持主流程可达且不增加产品语义的规范化口径。
- 外部接口/二进制缺失不改变页面、状态机、错误和 adapter 边界，允许 spec/plan 自动审批；它们阻止的只是生产集成完成声明。
- 若用户后续提供服务端 OpenAPI、公钥或 binaries，必须回写对应模块 spec/plan 后再接入。

## Risk And Constraint Assessment

| 风险 | 触发/影响 | 失败模式 | 下游保护 |
| --- | --- | --- | --- |
| 大爆炸迁移 | 所有 IPC 一次替换 | UI/数据/Sidecar 同时失效 | 垂直模块顺序、gateway 单边界 |
| 数据破坏 | electron-store -> SQLite | 覆盖/半迁移 | 只读源、备份、事务、幂等版本 |
| 免费包泄漏会员能力 | 仅运行时隐藏 | 可调用隐藏 command | Cargo feature + command registry 测试 |
| 任意路径访问 | 复刻 preload API | 文件读取/删除越权 | dialog grant + canonical path scope |
| async 泄漏 | 重连/定时/sidecar/listener | CPU/内存和重复事件 | cancellation token、Drop/cleanup 测试 |
| 动态资源执行 | 下载前端 bundle | 任意代码执行 | 签名、hash、CSP、固定 module enum |
| 外部服务缺失 | 会员/禁言/更新 | 假成功或崩溃 | typed unavailable、免费底座隔离 |
| 基线不健康 | node_modules 缺失、旧 test imports | 无法判断回归 | 先恢复安装/测试 harness，再记录迁移差异 |
| 工具链缺失 | 无 Rust/Cargo | 无法编译 Tauri | execute 前安装 Rust stable 与 Tauri prerequisites |

## Blocker Classification And Handling Stance

- 业务阻塞：无。
- 外部接口待补：OAuth/支付/令牌/manifest/update/Cookie API 契约；以 port/adapter/config/feature flag 承接，不可宣称生产联通。
- 外部资源待补：FFmpeg LGPL shared、whisper binaries/models、签名公钥/许可文件；本地实现 manager、校验和 unavailable 状态。
- 环境阻塞（进入 execute 前恢复）：安装 Rust stable/Cargo，执行 pnpm install；不影响需求拆分。
- 非阻塞：PRD 数量/路径笔误、性能机型记录。

## Confirmation-reduction Notes

- downstream 不再重开：技术栈、20 功能、双包、只读、本地优先、21 表、免费底座、到期保留、页面可见性。
- 可先行：所有本地 Rust 核心、UI、DTO、ports、unavailable states、mock contract tests。
- 必须停止生产声明：真实付款、会员签发、模块 CDN、Cookie 禁言、打包 Sidecar 未提供时。
- 所有模块必须复用统一 gateway/error/event/build capability，不得各自创建 Tauri 调用风格。

## Splitting Strategy

必须拆分。按依赖和可独立验收的业务能力分为：运行时基础、数据/安全、连接事件、风控、统计、提醒音频、录制、应用 UI、会员模块、AI/自动禁言、插件更新迁移、最终发布验证。公共 contract 与 DB 不能分散复制；外部会员/AI ports 与本地实现不能合并成伪生产集成。

## Downstream Stage Signals

- 所有模块都 architecture-sensitive。
- 含页面/导航/表格/配置交互的模块需要 page-design；纯 scaffold/DB/contract 模块不需要。
- 顺序必须先基础设施，再公共免费能力，再 UI，再会员和增值能力，最后插件/更新/发布。
- requirement splitting 应控制模块数量，避免把 20 个页面各自拆成无法完成的微模块，同时确保每个模块有独立 spec/plan/verify/review。
