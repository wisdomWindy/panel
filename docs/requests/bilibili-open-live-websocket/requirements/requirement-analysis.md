# Requirement Analysis：哔哩哔哩开放平台长链弹幕

## 资料来源与阅读口径

- 用户请求决定目标和范围。
- `description-doc/哔哩哔哩直播开放文档 (3).md` 是 HTTP 鉴权 source of truth。
- `(4).md` 是项目 start/heartbeat/end 生命周期 source of truth。
- `(6).md` 是 WebSocket Proto、AUTH 和心跳 source of truth。
- `(7).md` 是 CMD 与消息字段 source of truth。
- 现有代码决定内部端口、领域模型和 UI 兼容边界。若旧占位行为与开放平台文档冲突，以开放平台文档为准。

## 已确认口径

- 连接流程由主播身份码驱动，不再假设仅凭房间号可以调用官方 `/v2/app/start`。
- 项目开启响应中的 `anchor_info.room_id` 是实际会话房间号，并写入现有 `RoomSnapshot`。
- appid、AccessKeyId 与 Secret 都使用后端编译期固定值，不接受环境覆盖；Secret 不进入 renderer、SQLite、日志或 Debug 输出，但会随源码和构建产物分发。
- 连接失败、鉴权失败、项目心跳失败和 interaction end 均结束当前事件流，由现有 `LiveDataState` 决定是否重连。
- 重连会重新调用 start，产生新的 `game_id`；旧会话在传输任务结束时尽力调用 end。

## 请求摘要

新增一个实现 `RoomTransport` 的开放平台基础设施适配器，负责从身份码到领域事件的完整外部通信，并调整现有连接契约和工具栏文案以反映真实输入。

## 需求目标与业务价值

让弹幕面板、风控、统计、榜单、礼物提醒、TTS 等既有能力获得真实直播消息源，而不重复实现下游业务逻辑。

## 范围内事项

- 开放平台凭证配置与启动前校验。
- HTTP 请求签名和 start/heartbeat/end 客户端。
- WSS 集群地址依次尝试、AUTH、双心跳、二进制帧解析。
- Version 0、Version 2/zlib、嵌套多包解析。
- Danmu、Gift、Captain、Enter 四类 CMD 到现有领域模型的映射。
- interaction end、协议错误和网络错误传播。
- 前端身份码输入、IPC 契约、错误提示和测试。
- 应用窗口销毁时触发直播会话关闭。

## 非目标与范围外事项

- 不新增哔哩哔哩账号登录或身份码获取服务。
- 不实现文档中的点赞、开播/停播、付费留言等当前领域模型未承载的事件。
- 不把 Secret 做成普通设置项或导入导出配置。
- 不改变敏感词、黑名单、统计、排行、媒体消费逻辑。

## 关键参与者与调用方

- 主播：提供身份码并启动连接。
- renderer 工具栏：提交身份码、显示连接状态。
- IPC/live-data：保持桌面边界与错误 DTO。
- `LiveDataState`：拥有重连、过滤、统计和消费流程。
- `BilibiliOpenLiveTransport`：拥有单次外部会话生命周期。
- 哔哩哔哩开放平台：HTTP 鉴权与 WSS 消息源。

## 项目、服务与数据归属边界

- 所有网络与密钥处理均在 Rust backend。
- renderer 仅接触主播身份码和非敏感状态。
- 数据库继续只存业务事件和非敏感设置，不新增凭证表。
- transport 只生成领域事件，不直接落库或调用 UI。

## 显式约束归纳

- API 请求体字节必须与 MD5 所用字节一致。
- `x-bili-*` 待签名头按字典序、换行连接且首尾无额外换行。
- `Authorization` 为 Secret 对待签名字符串计算的 HMAC-SHA256 小写 hex。
- `app_id` 使用 64 位整数。
- 成功必须同时满足 HTTP 成功和 JSON `code == 0`。
- Proto 包长不得小于 16 字节或超过当前 buffer；畸形包不能 panic。
- WSS 文本帧、未知 Operation 和未知 CMD 不应制造领域事件。
- 每个 start 成功的 `game_id` 最终都应尽力 end；end 失败不得暴露 Secret 或阻塞进程退出。

## 隐含约束与假设

- `open_id` 是文档中稳定的用户唯一标识，优先映射到现有 `UserUid`；若缺失才回退到已废弃的数值 `uid`。
- 文档价格单位 `1000 = 1 元`，现有 `value_minor` 代表分，因此礼物总价值按 `price * gift_num / 10` 换算；大航海 `price` 同样除以 10。
- 消息时间戳为秒，现有领域使用毫秒，因此乘以 1000；缺失或非正值消息视为无效而忽略。
- `guard_level` 映射：1 总督、2 提督、3 舰长；其他值不产生 Captain 事件。
- `GUARD` 文档没有明确续费字段，现有模型默认映射为 Purchase。
- AUTH_REPLY 只有明确非零 code 才拒绝；成功后开始转发事件。

## 开发实现关键口径

| 问题 | 结论 | 依据 | 实现影响 |
| --- | --- | --- | --- |
| 连接输入是什么 | 主播身份码 | `/v2/app/start` 必填 `code` | IPC 和工具栏改为 `identityCode` |
| 房间号从哪里来 | start 响应 | `anchor_info.room_id` | transport 校验并构造 `RoomId` |
| Secret 如何保存 | 按用户要求编译进 Rust 产物 | 接受可被安装包提取的风险，仍禁止前端/日志/Debug 输出 | 固定配置构造器不查询环境 |
| 未知 CMD 怎么办 | 忽略 | 前向兼容 | parser 返回 `None` |
| 单条坏消息怎么办 | 丢弃该消息，不中断整条连接 | 外部数据不应拖垮会话 | 解析层区分包级致命错误与事件级无效 |
| 项目心跳失败怎么办 | 结束会话并触发上层重连 | 心跳维持鉴权和推送 | 关闭事件 channel，并尽力 end |

## 核心流程与状态分析

1. 校验身份码与凭证配置。
2. 签名调用 start，获得 `game_id`、主播信息、`auth_body`、WSS 地址列表。
3. 依次尝试 WSS 地址；全部失败则 end 并返回可重试错误。
4. 发送 AUTH Proto，等待成功回复。
5. 返回 `RoomConnection`，后台任务每 20 秒发送 WS 心跳与项目心跳，并读取/解包消息。
6. 事件 channel 关闭后，`LiveDataState` 按现有规则重连或断开。
7. 用户取消或应用退出导致后台 sender 被关闭，传输任务退出并调用 end。

## 多模式/多角色差异流程

当前只实现开放平台 client 模式和单身份码单房间连接。server 模式、演播厅跨房模式和多账号聚合不在范围内。

## 关键实体、接口与事件观察

- 配置：`app_id`、`access_key_id`、`access_key_secret`。
- 会话：`identity_code`、`game_id`、`auth_body`、WSS 集群、anchor。
- Proto：length/header length/version/operation/sequence/body。
- CMD：DM、SEND_GIFT、GUARD、LIVE_ROOM_ENTER、INTERACTION_END。

## 外部系统与既有能力复用边界

必须复用 `RoomTransport`、`LiveDataState`、`LiveEvent` 和 `TauriLiveEventSink`。HTTP/WSS 使用 Rust 成熟库，压缩使用 zlib 库；不在 renderer 手写网络协议。

## 验收与回归基线

- 固定向量验证签名字符串、MD5 与 HMAC 输出。
- 固定二进制 fixture 验证 Proto 编解码、zlib 嵌套多包和畸形长度。
- 固定 JSON fixture 验证四类领域事件及未知/无效消息。
- mock HTTP/WSS 边界验证 start/heartbeat/end 请求路径与生命周期。
- 现有 Rust、renderer live-data、application shell 契约测试继续通过。

## 研发拆分建议

按一个集成模块推进，内部依次完成配置与签名、Proto 与事件映射、网络会话、应用装配和 UI 契约。项目心跳与 WebSocket 生命周期不可拆成独立发布模块，否则容易泄漏 `game_id` 会话。

## 风险、歧义与待确认项

### 业务阻塞

无。身份码仍由连接操作者输入，协议主链路明确。

### 外部接口待补

- 真实联调仍缺少有效主播身份码。触发条件：访问正式 API；影响：无法证明账号级鉴权与真实消息接收；防护：完成离线契约测试；不阻止编码和离线验证。
- AUTH_REPLY 完整 schema 未在本地文档列出。影响：服务端异常回复识别；防护：兼容顶层和 `data.code`，非零失败、格式异常失败；不阻止实现。

### 非阻塞

- 在线人数没有对应开放平台字段，保持 `None`。
- `GUARD` 无续费标记，当前统一为 Purchase；后续字段出现时再扩展。

## 拆分策略与建议拆分轴

按“开放平台会话集成”而非页面或 endpoint 数量拆分，因为 start、双心跳、WSS 和 end 共享同一 `game_id` 事务边界，必须一起验证。

## architecture-design 前置信号

本请求新增外部网络适配器、异步后台任务和跨协议生命周期，属于 backend-structural，需要 architecture-design。

## requirement-splitting 输入检查清单

- 范围边界稳定：是。
- 关键协议来源完整：是。
- 阻止无损拆分的业务歧义：无。
- 外部联调凭证缺口已记录且可由 adapter/config 边界隔离：是。
