# Spec：开放平台会话集成

## 模块标识与范围

`open-live-session`。负责从主播身份码到现有 `LiveEvent` 的官方开放平台会话，不负责下游业务处理。

## 背景与目标

当前生产装配注入 `UnavailableRoomTransport`，导致连接必然失败。目标是接入仓库文档定义的官方 API/WSS 流程，同时保持现有过滤、落库、统计、排行和媒体消费者不变。

## 范围内事项

运行时配置、HMAC 签名、start/heartbeat/end、WSS 地址回退、AUTH、WS 心跳、zlib Proto 解包、四类事件映射、连接契约迁移、退出清理和测试。

## 范围外事项

账号登录、身份码获取、普通直播间非开放平台抓取、点赞/SC/开停播事件、新数据库表、Secret 设置页面。

## 触发与启动条件

- 用户输入非空、长度不超过 128、只包含非控制字符的主播身份码。
- 后端使用项目内置的 appid、AccessKeyId 与 Secret 固定值，不查询也不接受任何开放平台凭证环境变量覆盖。
- 点击连接后创建异步会话；点击断开或窗口销毁时取消。

## 需求拆分摘要

唯一模块覆盖完整 `game_id` 生命周期，来源追踪见 requirement map。会话生命周期不可拆散。

## 请求与工作流流程

1. renderer 调用 `room_connect({ identityCode })`。
2. `LiveDataState` 进入 Connecting，启动 transport connect。
3. transport 签名 start；成功响应构造 anchor `RoomId` 和会话参数。
4. 依次尝试 WSS，发送 AUTH，收到成功 AUTH_REPLY 后返回 `RoomConnection`。
5. 后台任务每 20 秒发送 OP 2 空 body，同时签名调用 API heartbeat，并消费 socket。
6. OP 5 解包映射事件；OP 3 忽略；interaction end 结束流。
7. 所有终止路径进入 end best effort。
8. channel 关闭后上层根据错误可重试性决定重连；主动取消不重连。

## 服务与模块设计

- `config.rs`：内置凭证与格式验证。
- `signing.rs`：纯函数 MD5、canonical headers、HMAC。
- `protocol.rs`：Proto encode/decode 与 zlib 递归。
- `event_mapper.rs`：外部 JSON 到领域模型。
- `api.rs`：签名 HTTP 调用与 envelope/error 解析。
- `transport.rs`：WSS、AUTH、双心跳、地址回退和 cleanup。
- `mod.rs`：最小公开面 `BilibiliOpenLiveTransport` 与配置。

## function-complete 行为拆解

- 三项凭证始终使用编译期固定值；固定值在构造时仍执行非空与 appid 数值校验。
- identity code 无效返回 `invalid_identity_code`。
- start 业务 code 非零映射 `open_live_api_error`；7001/5xxx/network 可重试，其余默认不可重试。
- WSS 全部失败返回 `open_live_websocket_unavailable`，并调用 end。
- AUTH_REPLY 明确非零返回 `open_live_auth_failed`。
- heartbeat 失败关闭事件流，使上层重连。
- 未知 CMD、无效单事件字段不发送事件；不会 panic。
- interaction end 不生成领域事件，直接结束流。

## 设计约束

- 外部字段名保持官方 snake_case；只在 mapper 进入领域模型时转换。
- 网络侧 effect 集中在 api/transport；domain/application 不依赖 reqwest/websocket/zlib。
- 签名、协议与 mapper 均为可独立测试的纯逻辑。
- 不引入 provider factory 或策略层；只有一个真实 provider，直接 adapter 足够。

## 变更轴与模式决策

采用 Ports and Adapters 中的 integration adapter：现有 `RoomTransport` 是端口，开放平台实现隔离第三方契约。它解决外部字段、签名和协议与领域模型不兼容的问题。更简单地把网络代码写进 `LiveDataState` 会混合重连业务和供应商协议，因此不采用。

## 代码上下文与影响假设

见请求级 `artifacts/code-context.md`。`RoomTransport` 当前仅有占位实现和测试 doubles，允许把输入改为 `IdentityCode`；`RoomSnapshot.room_id` 对 renderer 的输出语义不变。

## 接口与数据契约

### IPC 入站

- command：`room_connect`
- payload：`{ identityCode: string }`
- success：现有 `RoomSnapshotDto`，初始 Connecting 时 `roomId=null`，连接成功后为平台 anchor room id。
- failure：稳定 `DesktopError` code/message/retryable。

### HTTP 出站

- source：本地文档 `(3).md` 与 `(4).md`。
- `POST /v2/app/start`：`{ code, app_id }`。
- `POST /v2/app/heartbeat`：`{ game_id }`。
- `POST /v2/app/end`：`{ app_id, game_id }`。
- timeout：单请求 10 秒。
- retry：API client 不自动重试；会话上层拥有重试。
- auth：完整 x-bili 签名头；绝不记录敏感值。

### WSS 与 Proto

- source：本地文档 `(6).md`。
- Header：`u32 packet_len`、`u16 header_len=16`、`u16 version`、`u32 operation`、`u32 sequence`，大端。
- outbound AUTH：version 0、operation 7、body 为 `auth_body` 原始 UTF-8。
- outbound heartbeat：version 0、operation 2、空 body。
- inbound message：operation 5；version 0 JSON，version 2 zlib 后递归 Proto。
- 最大解包深度 8，最大单帧/解压 body 8 MiB，防止资源滥用。

### 事件映射

- `DM`：`msg_id`、room id、timestamp 秒转毫秒、open_id/uid、uname、msg、粉丝牌、guard level。
- `SEND_GIFT`：gift id/name/num，`value_minor = price * gift_num / 10`，使用 checked/saturating 边界避免溢出。
- `GUARD`：guard level、num、price/10，总价值按文档字段语义映射；action 默认为 Purchase。
- `LIVE_ROOM_ENTER`：基本身份与时间。

## 基础设施依赖策略

使用 `reqwest` rustls、`tokio-tungstenite` rustls、`futures-util`、`hmac`/`sha2`、`md-5`、`flate2`。不涉及 MySQL、Redis、Kafka、gRPC 或 Kubernetes；SQLite schema 不变。

## 安全与滥用防控

- appid、AccessKeyId 与 Secret 使用用户指定的编译期固定值，不允许环境变量覆盖；Secret 保留在 Rust 边界，不进入 renderer、SQLite、日志或 Debug 输出。
- 安装包中的 Secret 可被逆向提取，这是用户明确接受的安全风险，不得将“未在日志中输出”误解为“安装包中不可提取”。
- 禁止 Debug 派生/日志输出 credentials、签名 headers、identity code、auth body 和完整事件 payload。
- 对包长度、解压长度、嵌套深度、字符串控制字符和数值转换做边界校验。
- renderer 永远拿不到 AccessKey。

## rollout、迁移与回滚策略

无数据库迁移。回滚为恢复装配到 unavailable transport 和旧 IPC 契约；新版本缺配置时以明确错误降级，不影响应用其他设置页面启动。

## 上下文与依赖来源

本地开放平台文档、requirement analysis/map/module、architecture design、现有 live-data 端口和 code-context。

## 边界情况

- start 返回空 WSS 列表或无 anchor room id。
- 首个地址失败、后续地址成功。
- AUTH_REPLY 与消息在同一或相邻帧到达。
- zlib body 包含多个 Proto。
- frame 含截断 header、非法 packet/header length、未知 version/operation。
- open_id 缺失但 uid 可用；两者都缺失。
- timestamp、gift_num、price 或 guard_level 无效。
- receiver 被主动 drop、socket close、API heartbeat 失败、interaction end。

## 验收标准

1. 官方示例 header 可生成文档给出的 HMAC 小写 hex。
2. start 请求使用与 MD5 完全一致的 body 字节并解析成功 envelope。
3. Proto round-trip、多个包、Version 2 递归和畸形输入测试通过。
4. 四类官方 fixture 产生有效 `LiveEvent`，未知/无效 fixture 不产生事件。
5. 生命周期测试证明 start 成功后会 AUTH、双心跳，并在结束时调用 end。
6. 缺配置时应用可启动，点击连接返回明确错误且不泄露配置值。
7. renderer 工具栏显示身份码输入语义，连接后使用响应房间号。
8. Rust 全套测试、frontend type-check 与相关 Vitest 全部通过。
9. renderer、SQLite、日志和 `Debug` 输出不包含 Secret；Rust 源码与构建产物包含固定 Secret 属于预期行为。

## 人工评审与交接

离线验收完成后，只需使用有效主播身份码做正式服务 smoke test；不应在聊天、截图或日志中再次回显 Secret。

## 风险

- 正式服务协议可能包含文档未展示的 envelope；通过兼容解析和明确错误降低风险。
- Secret 随桌面程序分发后无法对用户保密；泄露或需轮换时必须重新编译发布。
- 进程强杀无法保证 end；平台项目心跳超时负责最终回收。
