# 移除主动发送弹幕能力规格

## 1. Scope 与来源

- authoritative source: `SRC-1`，用户指令“移除 以主播账号发送弹幕 互动面板弹幕指令 功能”。
- in scope：主播账号主动发送、所有依赖发送原语的自动/事件/插件链路、对应状态与入口、产品自有文档宣称。
- out of scope：弹幕/礼物接收，展示、统计、排行榜、TTS、录制、高光、黑名单；`description-doc/` 官方资料快照；历史 request 工件和历史 changelog。

## 2. Source grounding mapping

| Spec ID | 合同 | Grounding | 来源 |
| --- | --- | --- | --- |
| S-1 | 删除主播账号主动发送弹幕能力 | source-backed | SRC-1 |
| S-2 | 删除互动面板弹幕指令产品能力/宣称 | source-backed | SRC-1 |
| S-3 | 删除手动、定时、随机、触发词、角色、礼物/舰长和插件发送路径 | source-derived-safe | S-1 的必要依赖闭包 |
| S-4 | 保留只读弹幕和礼物消费链路 | code-fact-backed | 现有接收链路不依赖发送原语 |
| S-5 | 删除发送侧持久化、配置、状态、统计与测试 | code-fact-backed | S-1/S-3 的引用闭包清理 |
| S-6 | 官方资料快照和历史审计记录保持不变 | code-fact-backed | 来源/审计完整性 |

## 3. Closed-world product content allowlist

本次仅允许删除以下用户可见内容：发送按钮和弹窗、Cookie 发送配置、弹幕池菜单/页面、自动发送/触发回复/礼物感谢设置、发送侧统计与状态、插件发送 API 及当前能力说明。不得新增替代文案、替代接口或新交互。

## 4. User intent contract

- literal request：移除两个发送相关功能。
- practical goal：产品不再依赖无正式开放合同的主动发送能力，也不通过隐藏入口保留同类行为。
- success criteria：生产代码没有发送请求、入口、定时器、事件回复或扩展 API；保留链路通过验证。
- forbidden interpretations：仅隐藏菜单但保留发送 API；仅删除 `/msg/send` 请求但保留自动发送配置；将发送能力换名或迁移到插件/辅助层。
- acceptable approach：按引用闭包直接删除并收窄类型、状态和文档。

## 5. 用户旅程与可见行为

### 5.1 弹幕列表

- 保留接收弹幕列表、总数、清空列表、时间与用户内容展示。
- 删除“发送弹幕”按钮、发送对话框、发送 loading/error 状态。
- 每条消息统一表达为“收到”，不再有已发送样式、成功/失败标签。
- 删除今日发送、发送成功率和最后发送统计；不新增替代统计。

### 5.2 连接配置

- 删除 Cookie 输入和 CSRF 状态展示。
- 保留开放平台连接状态、房间号、在线人数等只读连接信息。

### 5.3 导航和设置

- 删除弹幕池路由与菜单，旧地址不再是产品路由。
- 删除弹幕设置与自动发送设置页签及其配置表单。
- 保留过滤、TTS、AI、录制等与发送无关的设置。

### 5.4 仪表盘

- 删除发送统计卡片和感谢弹幕数量。
- 保留接收弹幕数、礼物统计、排行榜和最近活动。
- 不新增替代卡片；允许现有布局按剩余内容自然收敛，不做视觉重设计。

### 5.5 插件

- `PluginAPI` 不再包含 `sendDanmaku`，沙箱不再暴露该全局函数。
- `danmaku` 权限只代表监听弹幕，不再表达读写能力。
- `onDanmaku` 保留。

## 6. 生产行为与状态合同

- `DanmakuManager` 只负责接收、解析和分发，不再拥有 sender、弹幕池、自动发送 timer、触发词/角色/礼物回复状态。
- 礼物和舰长事件继续向上层发出，但不产生发送副作用。
- `DanmakuService` 不再暴露发送、弹幕池和发送间隔方法，不再处理发送成功/失败事件。
- `useDanmakuStore` 删除弹幕池和发送侧计数；排行逻辑直接统计所有接收弹幕。
- main database/IPC 删除弹幕池和发送凭证/频控配置；已存在的 electron-store 历史键允许自然遗留但不再读取或写入，不做破坏性数据迁移。
- 配置 import 遇到历史发送字段时按现有宽松结构读取后由 `AppConfig` 合并路径忽略/不消费，不新增兼容 UI。

## 7. API 与数据合同

- 删除非正式 `/msg/send` 集成及 `DanmakuSender`。
- 官方 `live-open.biliapi.com` start/end/heartbeat 与 WebSocket 接收合同保持不变。
- 删除 `DanmakuPoolItem`、`DanmakuOptions` 和发送态 `Danmaku.type='sent'|success|message`。
- `Danmaku.type` 可删除；解析结果继续代表接收消息。
- 不新增 adapter 或替代 API。
- API contract source：现有官方开放平台文档与 `modules/bilibili/types.ts`；本次只删除无正式合同的发送边界。

## 8. TypeScript context

- governing config：`packages/bilibili-live-tool/tsconfig.json`，eslint/type-check 使用 `tsconfig.eslint.json` extends 链。
- material options：`strict`、`noUnusedLocals`、`noUnusedParameters`、`moduleResolution=bundler`、`@/*` renderer alias。
- scoped type sources：`modules/danmaku/types.ts`、`plugins/types.ts`、`stores/config.ts`、`main/database.ts` 本地接口和 `vite/client` globals。
- 删除后必须通过 scoped package type-check；不得用 `any` 或断言掩盖残留引用。

## 9. Architecture-design routing input

- `architecture_design_required=true`：发送能力横跨 manager/service/store/views/router/settings/plugin/main persistence，删除会改变所有权和依赖图。
- architecture-design 只决定清理顺序、最终 owner 和保留链路，不得改变 S-1 至 S-6。
- 必须证明 sender、timer、pool、send-state、plugin API 没有孤儿引用。

## 10. Pattern-fit decision

- problem：删除既有跨层功能链并收窄剩余边界。
- depth：Level 0。
- decision：direct code；pattern `none`。
- rationale：没有新的选择、适配或变化轴；新增 facade/strategy/compatibility wrapper 会保留用户要求删除的抽象。按已有层级直接删除是最轻且最可审查的实现。
- overbuilt signal：任何仅为保留旧发送接口而新增的空实现、deprecated wrapper 或 feature flag 都属于过度设计。

## 11. Production quality 与评审约束

- 删除完整引用闭包，不留 dead exports、stale comments、发送文案、未使用 import 或空设置组件。
- 保留事件分发必须无新增异步副作用和无未处理 promise。
- 不做无关格式化或相邻重构。
- 每个 changed hunk 必须映射到 S-1 至 S-6 或测试适配。
- 验证证据至少包括：残留关键词搜索、package type-check、lint check、现有相关 tests、构建；无法运行项需记录原因。

## 12. TDD 与验证策略

- 本次以删除行为为主，先用残留引用搜索和现有类型测试形成 red evidence，再删除代码并更新类型测试。
- 重点断言：无 `/msg/send`、`sendDanmaku`、`DanmakuSender`、弹幕池路由/配置/IPC；接收相关类型测试和 package 检查通过。
- 不新增仅验证文件不存在的脆弱单测，引用闭包由编译和静态搜索验证。

## 13. Acceptance criteria

1. AC-1：生产代码不存在主播账号主动发送弹幕的请求或 callable API。
2. AC-2：手动、自动、触发词、角色、礼物/舰长和插件发送入口全部删除。
3. AC-3：弹幕池页面、路由、菜单、配置、store 和 main persistence 闭包全部删除。
4. AC-4：连接、弹幕列表、仪表盘和设置不再展示发送相关字段或文案。
5. AC-5：产品自有当前能力文档不再宣称主动发送或互动面板弹幕指令；官方快照、历史请求和历史 changelog 保持不变。
6. AC-6：弹幕/礼物接收、统计、TTS、录制等保留链路通过类型检查、lint、相关测试与构建。
7. AC-7：change-chain、removal cleanup、API contract、TypeScript context 和 human review readiness 均可验证为 pass。

## 14. 风险与边界

- 删除旧持久化字段不会主动清除用户磁盘数据；旧键不再消费，避免破坏性迁移。
- 第三方插件若调用旧 `sendDanmaku` 将不兼容；这是用户明确删除 API 的直接结果，不保留兼容壳。
- 旧 `/danmaku-pool` 收藏地址将无法匹配；不新增重定向，以免保留已删除能力入口。

