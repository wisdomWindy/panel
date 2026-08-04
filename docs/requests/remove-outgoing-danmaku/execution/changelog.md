# Execution Changelog: remove-outgoing-danmaku

## 交付单元与批准输入

- delivery unit：`remove-outgoing-danmaku`，非拆分 direct-change，`S2 scoped`。
- framework-approved spec：`spec/spec.md` S-1 至 S-6、AC-1 至 AC-7。
- framework-approved plan：`plan/plan.md` T-1 至 T-3。
- architecture：`design/architecture-design.md` ADR-1 至 ADR-3。

## 执行摘要

- 删除 `DanmakuSender` 与非正式 `/msg/send` 请求，收窄 manager/service/store 为接收链。
- 删除手动、定时/随机、触发词、角色、礼物/舰长感谢和 AI 自动回复发送闭包。
- 删除弹幕池页面、路由、菜单、配置、IPC、持久化、发送统计和 Cookie/CSRF 凭证。
- 删除插件 `sendDanmaku` 类型、实现、依赖注入、沙箱/runtime 暴露，保留 `onDanmaku`。
- 删除当前产品文档的发送能力宣称；历史 changelog、官方快照和历史 request 保持不变。
- 历史配置通过 `mergeAppConfig` / `mergeAppDataConfig` 在 store/database 数据边界白名单归一，旧发送字段不再进入当前模型或重新写回。

## Self-healing execution notes

- loop iteration：4，初次执行阶段；非 verify/review 回流。
- 本轮新证据：生产源码残留检索为零，三项相关测试和 app/homepage build 通过。
- 本轮修复：残留审查发现并删除 main database 的 `aiMinInterval`；移除执行期曾新增但未获 closed-world allowlist 授权的 AI “本地预览”替代文案，恢复中性既有标签。
- defect class：`code-organization-error` 与 `requirement-mismatch`，均在本执行阶段首次发现并修复；remaining blocker：none。
- verify 重点：复核发送符号/文案残留、配置白名单、插件只读 API、接收链、基线 type/lint 失败归属。

### Iteration 6 verify 回流

- 来源：iteration 5 verify 发现 AC-5 requirement-mismatch。
- blocker identity：当前新 PRD 仍宣称会员 AI 自动发送弹幕；same-stage attempt 1，total attempt 1。
- owning stage：`execute`，修复已在批准 T-3 “当前 requirements 文档”范围内，不需要改 spec/plan。
- 修复边界：仅删除或改写自动发送/互动指令语义；不回滚该未跟踪 PRD 的其他用户内容。
- 修复结果：当前 PRD 的会员定位、场景/用户故事、功能总览、4.12 AI 模块、数据库字段、安全/权限/风险与修订记录已统一为本地 AI 文案生成；主动发送相关 symbol/Cookie/风控发送字段检索为零。
- progress：new repair + narrower evidence；下一步重新验证 AC-5，并复跑全量 residual 与 diff check。

### Iteration 9 review 回流

- 来源：iteration 8 review 发现 AI 历史配置仍可通过对象 spread 保留已删除字段。
- defect class：`logic-error` / `removal cleanup`；same-stage attempt 1，total attempt 1。
- owning stage：`execute`；批准范围为 T-1/T-2 的 AI 自动回复状态和历史配置清理。
- 修复要求：在 `useAIStore` 数据边界对白名单字段和嵌套 personality 做归一；LLM service 只接收归一后的当前合同。
- 修复结果：新增同文件 `mergeAIConfig`，对白名单 scalar/enum/personality/examples 做类型归一；`loadConfig` 与设置 import/update 均通过该边界，`saveConfig` 只能持久化当前 `AIConfig`；LLM service 接收归一后的完整配置。
- 验证：app renderer/main/preload build 通过，相关 tests 全绿，发送 residual 为零，`git diff --check` 通过。

### Iteration 12 review 回流

- 来源：iteration 11 独立 review 发现当前官网仍宣称“启用 AI 回复 / 设置触发条件”，且 LLM service 仍引用已删除的回复逻辑 composable。
- defect class：`requirement-mismatch` 与 `code-organization-error` / `removal cleanup`；两个 blocker 的 same-stage attempt 1，total attempt 1。
- owning stage：`execute`；修复仍属于 T-3 当前文档宣称和 T-1/T-2 删除注释闭包，无需改动 spec/plan。
- 修复：`features/ai.md` 收窄为本地 AI 文案辅助，明确无事件触发和无直播间发送；当前项目/架构说明同步去除自动互动语义；`llm-service.ts` 注释改为当前 store owner。
- provenance：仅删除或收窄 SRC-1 禁止的发送/自动触发宣称；代码注释是 `product-content delta: none` 的失效依赖闭包清理。
- progress：new repair + narrower residual search target；验证重点为当前文档自动触发语义、删除符号残留、homepage/app build 与 diff check。

### Iteration 15 review 回流

- 来源：iteration 14 将残留搜索扩大到中文发送语义后，在 4 个不同当前文档中发现新的发送宣称。
- defect class：文件级 `requirement-mismatch` / removal cleanup；每个 affected file/symbol 的 same-stage attempt 1，total attempt 1。
- owning stage：`execute`；修复属 T-3 明确列出的当前 plugin/architecture/implementation 文档闭包。
- 修复：删除插件能力列表的自动弹幕、实施计划的 `sender.ts` 文件、项目总结的自动弹幕风控建议；架构模块职责收窄为接收/解析/分发/黑名单。
- provenance：全部是 SRC-1/S-2 授权的当前能力宣称删除，未新增产品能力、接口或交互。
- progress：new repair + broader semantic residual evidence；验证重点为发送中英文符号、中文语义、删除文件名和 `git diff --check`。

## Task-board updates

- T-1：完成，运行时/状态/持久化发送闭包已删除。
- T-2：完成，UI/插件/路由/设置发送入口已删除。
- T-3：完成，测试、当前文档、残留搜索、构建和 diff 自审已执行。

## Changed files and owned symbols

- 核心运行时：删除 `modules/danmaku/sender.ts`；收窄 `manager.ts`、`types.ts`、`danmaku-service.ts`、`stores/danmaku.ts`、`stores/room.ts`。
- 配置/持久化：收窄 `stores/config.ts`、renderer `database-service.ts`、main `database.ts`、`ipc-handlers.ts`。
- AI：删除 `useReplyLogic.ts`；收窄 `modules/ai/types.ts`、`stores/ai.ts`、`AISettings.vue`，只保留手动文本生成能力。
- UI/导航：删除弹幕池页和三个发送设置组件；收窄 App、router、connection、danmaku-list、dashboard、settings、locales 与生成组件声明。
- 插件：收窄 `plugins/types.ts`、`plugin-api.ts`、manager、sandbox、runtime 与 barrel export。
- 测试：适配 `danmaku-types.test.ts`、`settings-config-io.test.ts`、`plugin-runtime.test.ts`。
- 当前文档：更新当前 requirements PRD、README、plugin/architecture/implementation 文档和 homepage 当前功能/指南/FAQ。
- 不属于本交付单元且未修改/恢复：`pi-code-agent.zip`、`docs/requests/project-optimization-assessment/` 及工作区既有 requirements 文件变更。

## Introduced-content provenance ledger

| Hunk | 内容 | Plan / 来源 | Product-content delta | 结果 |
| --- | --- | --- | --- | --- |
| 页面、路由、设置、统计、locale | 删除发送入口、状态与文案 | T-2；SRC-1 | allowlist 内删除 | authorized |
| 插件公开面 | 删除 `sendDanmaku`，保留监听 | T-2；S-2/S-3 | allowlist 内删除 | authorized |
| 当前产品文档（含现行 PRD） | 删除主动发送/互动指令宣称，并将保留的 AI 能力明确为本地生成 | T-3；S-2/S-4/S-6 | allowlist 内删除与保留链说明 | authorized |
| manager/service/store/database/types | 删除发送 I/O、状态、timer、配置与持久化 | T-1；S-1/S-3/S-5 | none，依赖闭包清理 | authorized |
| `mergeAppConfig` / `mergeAppDataConfig` | 白名单归一并忽略历史发送字段 | T-1；历史键不消费合同 | none，数据边界约束 | authorized |
| AI 设置 | 删除自动回复专属控件与说明 | T-2；S-3 | allowlist 内删除 | authorized |
| AI “本地预览”替代文案 | 执行期新增文案 | 无 allowlist 授权 | 新增 | removed |
| 测试 import/断言 | 适配 monorepo 路径与删除后的合同 | T-3；测试适配义务 | none | authorized |

未新增字段、控件、交互、默认值、权限、导航、API/payload 语义、状态映射或埋点。

## Implementation decisions

- pattern-fit：按批准的 Level 0 direct code 落地；没有新增 facade、adapter、feature flag、deprecated wrapper 或重定向。
- 历史 electron-store/localStorage 键不做破坏性迁移，仅通过白名单 merge 停止消费和写回。
- 第三方插件与旧 `/danmaku-pool` 地址不保留兼容路径，这是批准的删除结果。

## Expert frontend engineering execution notes

- 用户旅程由“接收 + 发送”收窄为只读接收；连接、接收弹幕/礼物、排行榜、TTS、录制、高光与黑名单入口保持。
- 删除发送 writable/derived state、loading、timer 和副作用，未新增 async 竞态、缓存或 reactive fan-out。
- 现有 Element Plus 语义控件、剩余 empty/loading 状态及键盘/焦点行为未改写。
- 无迁移双路径或兼容开关；Git 文件级 diff 是唯一回滚面。

## Frontend architecture quality execution notes

- ADR-1/2/3 均落实：发送垂直切片删除、无兼容层、历史资料保持。
- capability owner：manager 只拥有接收/分发；service 拥有 store/database 写入；插件只订阅事件；无发送 owner。
- state topology：room store 保留连接状态，danmaku store 保留接收状态；发送/pool/频控状态删除。
- data lifecycle：官方事件 -> parser -> manager event -> service -> store/database/view 保持。
- dependency matrix：生产源码不存在 renderer -> `/msg/send`、plugin -> send API、receive -> sender/pool 依赖。
- complexity budget：新增生产文件/API/间接层级均为 0；响应式 fan-out 和 timer 数量减少。

## Production code quality execution notes

- type-first：直接收窄已有 `Danmaku`、`AIConfig`、`PluginAPI`、`AppConfig`、main config 类型，没有 `any`/断言用于掩盖本次残留。
- fail-fast/null：保留连接/接收原有错误与空值边界；历史配置的非法/旧字段在 store/database merge 边界被忽略。
- naming/constants：未新增模糊布尔名或无意义常量；白名单 helper 为配置边界局部函数。
- performance/memory：删除发送 timers、cooldown maps 与响应式发送统计；未新增 cache/debounce/watch。
- pure/immutable：配置 merge 返回新对象；副作用仍位于 service/store/database/lifecycle owner。

## Human review readiness notes

- changed hunks 均映射到 T-1/T-2/T-3、依赖闭包清理或测试适配；未做宽泛格式化、技术栈替换或顺手重构。
- 已检查 removed imports/exports、orphan helpers、stale route/locale/test/mock/comment、debug code 和发送侧事件。
- 用户已有无关变更不回滚；当前新 PRD 仅修改发送闭包，其余已有内容保留。
- reviewer evidence：相关测试、locale JSON、残留搜索、app/homepage build、type check、lint startup、`git diff --check`。

## TDD evidence / exception

- 本次主要是删除行为。按计划用删除前引用链与残留关键词作为 red evidence，用现有类型/配置/插件测试、编译和构建作为 green evidence。
- 未新增“断言文件不存在”的脆弱单测；删除闭包由 TypeScript/build 与静态搜索验证。

## Change-chain review evidence

- pre-change：确认 views/plugins -> service -> manager -> sender -> `/msg/send`，以及 manager -> timer/trigger/role/gift reply、store/database/pool/config 的完整链。
- post-change：views -> service/store -> manager -> official receive client 保持；plugin -> event API 保持；发送原语、事件、状态、配置、持久化和当前文档链均无生产引用。
- 邻近功能：gift/guard 仍 emit，recording `autoRecordStart`、blacklist/spam、highlight 与接收排行仍有生产 owner。

## Removal cleanup evidence

- 已检查并清理 imports、exports、helpers、types、request wrapper、timers、events、refs/computed、store state、IPC、routes、SFC、locales、tests、comments 和 docs。
- 保留 helper 均存在真实接收/录制/过滤/插件监听生产调用方；测试引用未用于保留生产发送 API。

## Functional-programming execution notes

- `mergeAppConfig` / `mergeAppDataConfig` 是无副作用白名单转换，输入不被 mutate。
- `mergeAIConfig` 同样在 AI store 数据边界返回新对象，历史自动回复字段不会进入 runtime model 或重新写回。
- 接收事件副作用保持在 manager/service/store/database 明确边界。
- 表单/持久化历史字段语义在数据边界归一，没有在 component computed/watch/template 兜底。

## Frontend styling execution notes

- 本次仅随发送 DOM 删除失去 owner 的既有样式；未新增 authored styling、class 常量或组件抽取。

## API contract and adapter execution notes

- 删除无正式开放合同的 `api.live.bilibili.com/msg/send`；保留 `live-open.biliapi.com` 与 WebSocket 接收合同。
- 未新增或重命名 backend DTO；配置兼容仅在 store/database merge 边界白名单化。

## TypeScript context recovery notes

- governing config：`packages/bilibili-live-tool/tsconfig.json` 与 `tsconfig.eslint.json` extends 链。
- material options：strict、unused checks、bundler module resolution、renderer `@/*` alias。
- scoped types：danmaku、AI、plugin、config、main database 本地接口；未做全仓 `.d.ts` 扫描。

## Architecture conformance / code context

- 实际代码结构与 architecture-design 一致，无需回退。
- code graph 不可用时使用 scoped `rg`、直接 caller/import 阅读、编译与构建完成闭包复查。

## Deviations, blockers, rollback triggers

- package type-check 未全绿：仅有未改动的 logger 导入/索引与 gift-service unused 共 5 个基线错误。
- lint 未启动：项目未声明/链接 `vue-eslint-parser`，ESLint 在读取配置时失败；不在本次删除功能范围内安装依赖。
- `tests/ai-service.test.ts` 指向不存在的仓库根 `src`，属于未改动测试的既有路径错误；相关 AI 生产代码由 app build 覆盖。
- 上述均不由本次 diff 引入；本次 remaining framework-recoverable blocker：none。

## Verification handoff

- 通过：danmaku types 41/41、settings config 2/2、plugin runtime 3/3、locale JSON、app renderer/main/preload build、homepage build、残留搜索、`git diff --check`。
- app build 仍显示既有 logger browser-external 与 chunk-size warnings；没有构建失败。
- verify 应将 AC-1 至 AC-7 逐项映射到上述证据，并独立复核 changed diff 与历史材料排除范围。
