# 现有代码上下文

## Context Requirement

- 首次需要阶段：`requirement-analysis`
- 原因：这是跨 Electron 主进程、preload、Vue stores/services/views、构建脚本和测试的架构迁移，必须识别调用链和副作用所有权。
- Code graph：强烈推荐，但 Vue SFC 解析受工具限制；TypeScript 与 Electron main 子图可用。

## Graph Availability Check

- 仓库内状态：`missing`
- 检测：根/应用 package、lockfile、`.agents` 和可执行命令中无已配置 code graph 工具。
- 环境：Node 24.19.0、pnpm 10.13.1；Cargo 不在默认 PATH，后续通过 `C:\Users\yangjianlin\.cargo\bin\cargo.exe` 恢复并完成验证。
- 检查时点：loop iteration 2。

## Installation Or Bootstrap Record

- attempted: `yes`
- 方法：`pnpm dlx madge@8.0.0`
- 结果：`partial pass`
- 结果摘要：成功处理 Electron main 的 22 个 TS 文件和 renderer 的 83 个 TS 文件；直接解析 Vue SFC 时因 template 语法报错。
- 图发现：logger 子图存在 3 条循环依赖，均涉及 `log-formatter.ts`、`logger.ts`、`log-storage.ts`。

## Fallback Record

- fallback used: `yes`
- 方法：`rg` 检索 import、Electron API、IPC channel，再读取入口、store、service、route 与配置文件。
- 原因：Madge 不能可靠解析当前 Vue SFC；TS 图有 44 个解析警告。
- 置信度：可确认 Electron/Tauri 迁移边界与主要调用链；具体模块执行前仍需按模块读取全部直接 caller/consumer。

## Relevant Entrypoints

- 桌面入口：`src/main/index.ts` 创建 Electron 窗口、托盘、F9 快捷键并初始化 IPC 模块。
- preload：`src/preload/index.ts` 暴露通用 invoke/on/off 和任意路径文件 API。
- 前端入口：`src/renderer/main.ts`、`App.vue`、`router/index.ts`。
- 连接入口：login/connection views -> room store/danmaku service -> bilibili API/协议。
- 持久化入口：renderer database service -> Electron IPC -> `src/main/database.ts` (`electron-store`)。
- Sidecar 类入口：recording/asr/tts/ai/gift-sound services -> Electron IPC modules。
- 插件入口：plugin store/manager/loader -> preload 文件 API -> 本地 JS sandbox/runtime。

## Functional Chain Under Change

1. 当前桌面链：Vue view -> Pinia store/service -> `window.electronAPI` -> preload -> `ipcMain` -> Node/Electron module -> renderer event。
2. 目标链：Vue view -> typed desktop gateway -> Tauri `invoke/listen` -> Rust command/application service -> domain/infrastructure -> typed Tauri event。
3. 当前连接链一部分直接在 renderer 执行 WebSocket/协议解析；目标由 Rust RoomManager/EventBus 所有。
4. 当前数据库基于 `electron-store`，数据契约与 PRD SQLite schema 不一致；目标由 Rust Repository 所有并提供迁移入口。
5. 当前插件 manager/loader 可要求 preload 对任意路径读、复制、删目录；目标只允许系统对话框授权的安装包与 Rust 管理目录，运行时迁到 WASM host。
6. 当前录制/TTS/ASR/AI 的 IPC 名称与 PRD Command 不同；迁移期只能在单一 gateway 中保留显式兼容映射，页面不得直接判断运行时。
7. 测试位于根 `tests/`，多项 import 仍指向已失效的根 `src/`；它们是迁移适配面，不是保留旧生产代码的理由。

## Key Symbols And Modules

- Electron shell：`createWindow`、`createTray`、`registerGlobalShortcuts`、`registerIpcHandlers`。
- Frontend desktop boundary：`src/renderer/utils/ipc.ts`、`window.electronAPI` declaration。
- Stores：room、danmaku、config、recording、gift-sound、tts、asr、ai、moderation、plugin、update。
- Services：danmaku、database、recording、gift-sound、TTS、ASR、LLM、moderation、update。
- Existing domain helpers：protocol/parser/blacklist/sensitive words/variable replacer/highlight detector/file manager。

## Dependency And Side-Effect Boundaries

- Vue 页面应只消费 store/use-case，不直接读文件、数据库、密钥或 Sidecar。
- desktop gateway 只负责编码 Command/Event 与错误，不承载业务规则。
- Rust commands 只做输入边界和错误映射，应用服务拥有流程，domain 拥有纯规则，infrastructure 拥有 I/O。
- build feature 决定免费/会员二进制能力；会员状态决定会员二进制中已编译能力是否可用。
- 数据库、日志、网络、文件、通知、音频和 Sidecar 都是显式副作用边界。

## Impact Scope

- 必改：应用 package/build config、Vite config、桌面 gateway、所有 Electron API caller、路由/App shell、测试配置。
- 迁移/删除：`src/main`、`src/preload`、Electron scripts/dependencies；删除前逐模块确认功能已由 Rust 接管。
- 新增：`src-tauri`、Rust modules/migrations/capabilities、构建通道配置、Tauri plugin 前端依赖。
- 回归高风险：连接重连、事件过滤顺序、录制进程清理、插件路径权限、配置迁移、会员可见性、免费包二进制隔离。

## Open Follow-up Checks

- 每个模块执行前生成/复查对应 caller 闭包，避免一次性删除 Electron 链造成漏迁移。
- logger 循环依赖随 Node logger 移除而消失；若保留前端 logger，需要单独验证。
- 补齐依赖后重新建立 Vue-aware 图或以 Vite/TypeScript 编译图验证。

## Live Data And Safety Scoped Refresh (2026-09-14)

- graph route: 既有 Madge 结果仍不能可靠解析 Vue SFC，本阶段按 `code-graph.md` 使用 scoped `rg` caller closure + owning file reads；无需重复安装同一失败能力。
- direct callers: `DanmakuService` 仅由 `App.vue`、`views/connection`、`views/login` 创建；`useDanmakuStore` 由 danmaku/gift/dashboard/leaderboard 和该 service 消费。
- duplicated/incorrect owners: `stores/danmaku.ts`、`dashboard/index.vue`、`dashboard/components/Leaderboard.vue` 分别重复统计/排行；`modules/danmaku/parser.ts` 与 store 都做筛选/统计；全部属于本次迁移闭包。
- ordering defect: `DanmakuManager.handleDanmaku` 在 blacklist check 前先 emit `danmaku`，而 `DanmakuService` 的 sensitive filter 返回值未决定 UI fan-out。
- timing defect: renderer client 使用 3000ms 起步的指数退避，PRD 要求固定 5000ms、最多 10 次；dashboard 本地 5 秒 timer/deep watch 伪造 server cadence。
- unapproved UI: connection reconnect settings、full-screen streamer-code login、blacklist duration/permanent fields、dashboard recent activity 与 client-generated gift rank 均不在当前闭世界 allowlist。
- TypeScript context: `tsconfig.eslint.json` extends strict `tsconfig.json`; bundler resolution、ES2020/DOM、noUnused、Vue SFC、`@/*` aliases；gateway contracts are the relevant backend-facing type source。
- architecture consequence: root listener lifecycle + split Pinia snapshots replace renderer singleton；T09 已删除短期 readonly media projection，邻近页面只允许直接消费 Rust-backed feed/room state。
- confidence: high for current module caller/side-effect closure；后续 module 必须重新扫描自己的 direct consumers，但不得恢复 compatibility projection。
- pre-code contract recovery: the approved spec table expands to 20 independent Tauri command names (room 2, audience 1, sensitive 7, blacklist 5, stat 3, rank 2). Architecture/plan originally counted grouped rows as `<=15`; iteration 7 corrected the technical budget to `<=20` before production edits. Product/API semantics are unchanged.

## Live Data And Safety Review Refresh (2026-09-15)

- post-change chain: `App/root runtime -> platform/desktop -> Tauri IPC -> Rust application/domain/infrastructure -> Pinia -> scoped pages`; one native boundary and one listener owner remain.
- removed closure: renderer WebSocket/protocol/parser/manager, old room/danmaku stores, local blacklist/sensitive rule owners, login gate, duplicate dashboard/rank owners, media projection and reconnect/local-filter configuration.
- retained neighboring owners: Electron database/log/gift/recording pieces still have real callers in later modules; only live-data-specific config fields were removed.
- graph evidence: Vue-aware graph remains unavailable, so post-change confidence comes from scoped import/caller scans, TypeScript compilation, Rust/TS contract tests, dual Tauri builds and runtime screenshots.
- open structural uncertainty: production Bilibili adapter awaits an external endpoint/credential contract; it remains a deliberate fail-closed port rather than a legacy fallback.

## Interaction Media Scoped Refresh (2026-09-15)

- graph route: Vue-aware graph remains unavailable; this module uses scoped import/caller/event scans plus direct owner reads. The current Tauri/Rust compiler and dual-build graph remain available for post-change proof.
- frontend entrypoints: routes `/effect-preview`, `/gift-sound` and `/recording`; duplicated media forms under `views/settings`; persistent `RecordingStatusBar`; no dedicated gift-reminder or TTS route currently exists.
- gift-sound chain: page -> `gift-sound-service` -> `window.electronAPI`; a separate Pinia gift-sound store wraps the same service but the page bypasses it. The page also writes browser Blob downloads and exposes unapproved duration/play-mode/cooldown/priority fields.
- TTS chain: settings component -> TTS store -> renderer TTS service/engine -> generic Electron IPC -> Node TTS module. No current accepted-live-event caller invokes the store's danmaku handler, so the queue is disconnected from the Rust-backed feed.
- recording chain: page/child tables/status bar/settings -> recording store -> recording service -> generic Electron IPC -> Node recording engine/FFmpeg/file/highlight owners. Store event listeners and the page F9 listener have no cleanup owner; child tables import the store directly and browser-download raw file paths.
- event-chain gap: Node recording handlers still wait for renderer `recording:handleDanmaku/Gift/Captain` events, but no renderer producer remains after the live-data migration. Automatic highlight and media fan-out therefore require direct Rust accepted-event integration, not restoration of a compatibility projection.
- effect chain: the current preview page owns 2D/WebGL managers, browser audio and ad hoc animation frames; its editable template/render/test controls exceed PRD 9.6. The target keeps rendering in a restricted `effect-*` window with no Command permission and moves queue/window/audio ownership to Rust.
- persistent storage: migration `0001_initial.sql` already defines `t_gift_sound`, `t_recording`, `t_highlight` and `t_tts_config`; schema semantics must be reviewed against the PRD rather than replaced by Electron store shapes.
- existing reusable boundary: `PathPurpose::RecordingDirectory` and `GiftSoundFile` plus the opaque grant store already cover directory/file authorization. Save-file purposes must be added for gift-sound configuration and highlight export without exposing raw paths to Vue.
- cleanup owners: Rust must own audio/TTS queue tasks, Sidecar process/timers, hotkey registration, effect queue/window lifecycle and path grant revocation; Vue root/runtime owns one typed listener registration with deterministic teardown.
- retained neighbors: plugin effect-registration types belong to `extensions-release-resilience`; global recording status placement and settings navigation belong to `application-shell`; neither authorizes keeping Electron media implementations.
- test adaptation: root recording/highlight/file-manager tests target removed Node paths and include unapproved gift/captain highlight policies. They are migration inputs to rewrite against Rust, not production owners or product authority.
- confidence: high for direct media caller/side-effect closure. External cloud TTS and the distributable LGPL FFmpeg artifact remain intentionally unavailable until deployment contracts/resources are supplied.

## Application Shell Scoped Refresh (2026-09-15)

- graph route: Vue-aware graph remains unavailable; scoped owner reads and caller/import/route/i18n scans are used, with TypeScript/Vite/Rust compiler graphs required after implementation.
- current root: `App.vue` owns a partial toolbar/navigation/content shell, starts live/media runtimes, duplicates connection display, hard-codes most Chinese labels and has no bottom status region.
- current settings chain: `views/settings/index.vue` imports multiple feature stores, writes browser Blob downloads, accepts pasted JSON and exposes one global action row; `stores/config.ts` persists unvalidated broad UI values through `localStorage`.
- current native boundary: common `config_get/config_set` and purpose-bound `path_select/path_revoke` exist; SQLite `t_app_setting` repository owns safe non-secret key/value persistence but no typed shell/config-package/locale/UI/system-metric surface exists.
- stable upstream owners: live-data room/feed stores own connection, room, audience and rate inputs; media recording store owns recording status. The shell must project these snapshots, not duplicate writable state.
- dynamic capability gap: compile-time build channel exists, while the real member state producer/routes belong to the next module. Free-build absence and the four navigation matrices can be modeled/tested without fabricating production login/payment state.
- route/i18n gap: routes use hard-coded title metadata; shell navigation omits system settings and includes a top-level logs item not present in PRD 8. In-scope shell/settings labels are not consistently backed by both locale files.
- target chain: root shell runtime/store -> existing typed desktop transport -> Rust shell/config/metrics owners; pure navigation/status projection -> toolbar/navigation/status components; settings draft -> typed action -> Rust transaction -> refreshed snapshot.
- cleanup closure: remove shell/settings browser file and `localStorage` configuration authority, static scattered nav permission branches, duplicate connection display and stale in-scope hard-coded strings; retain neighboring plugin/update/member business owners for their later modules.
- confidence: high for direct shell/config/i18n closure. Notification entry content and real member state remain explicit downstream/external gaps.

## Membership Intelligence Scoped Refresh (2026-09-15)

- graph route: Vue-aware graph remains unavailable; targeted route/store/service/engine/contract/build/SQL scans plus compiler graphs are the approved substitute.
- current native surface: six fail-closed member stubs are member-gated; T01 expands the registry contract to 28 approved operations while implementations remain absent until T03.
- current shell gap: `App.vue` supplies literal `memberStatus: 'unavailable'`; navigation already defines member slots behind the Vite channel check. The member runtime must become the sole reactive status producer without moving member domain state into the shell store.
- legacy frontend owners: `stores/ai.ts`, `stores/asr.ts`, and `stores/moderation.ts` own browser engines/localStorage and directly call legacy services; direct production callers are currently absent outside those stores because the prior settings page removed their panels. They are migration/deletion targets, not authority.
- accepted-event seam: `LiveDataState` has one `AcceptedLiveEventConsumer`, currently `MediaState`. A small fanout at this existing application seam is required so the member consumer can receive only post-filter events without reversing `live_data -> member` dependencies.
- persistence surface: initial migration already has account/membership/order/module/AI/ASR/moderation tables, but credential-like account columns and moderation audit/alert gaps require additive migration only.
- TypeScript contract: package strict/bundler tsconfig governs the feature; T01 adds readonly member command/event maps and branded identifiers under the existing desktop contract owner.
- external boundary: Bilibili QR endpoints are PRD-sourced; membership/payment/key/manifest, LLM, moderation Cookie transport and Whisper artifacts remain unavailable production ports.
- confidence: high for ownership/import/removal closure; external success paths remain intentionally unimplemented until contracts arrive.
