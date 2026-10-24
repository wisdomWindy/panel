# 架构设计：开放平台会话集成

## 模块 ID

`open-live-session`

## 模块名称

开放平台会话集成

## 对应需求文件

`requirements/modules/open-live-session.md`

## 服务目标

以官方开放平台协议创建一个可取消、可重连、可清理的单房间消息会话，并只向应用层暴露现有领域事件。

## 目标调用方或场景

renderer 工具栏提交主播身份码；`LiveDataState` 管理状态、重连和消费；transport adapter 管理外部会话。

## 边界图

```text
Toolbar(identityCode)
  -> Tauri room_connect
  -> LiveDataState (status/retry/filter/stats)
  -> RoomTransport
  -> BilibiliOpenLiveTransport
       -> SignedOpenLiveApi (start/heartbeat/end)
       -> OpenLiveWebSocket (AUTH/heartbeat/read)
       -> Protocol decoder -> Event mapper
  -> mpsc<LiveEvent>
  -> existing repository/event sink/media consumers
```

## 接口面

- 应用端口：`RoomTransport::connect(&IdentityCode) -> RoomConnection`。
- `RoomConnection` 新增由平台 start 响应解析出的 `room_id`，保留 `room_name`、`online_count`、`events`。
- IPC：`room_connect` 请求由 `{ roomId }` 迁移为 `{ identityCode }`；响应仍为 `RoomSnapshot`。
- 编译期固定配置：appid、AccessKeyId 与 Secret。
- 运行时不读取任何开放平台凭证环境变量。
- 外部 API 固定基址 `https://live-open.biliapi.com`，网络测试通过可注入 base URL 隔离。

## 工作流与执行路径

1. 应用启动时构造 transport；只读取配置，不发网络请求。
2. `connect` 校验 identity code 和配置，签名调用 start。
3. 解析 `game_id`、主播、auth body 和 WSS 列表，依次尝试 WSS。
4. 成功 socket 发送 AUTH 包，等待 AUTH_REPLY 成功后创建事件 channel。
5. 单后台任务拥有 socket 与 `game_id`：同时处理传入帧、20 秒 WS 心跳、20 秒 API 心跳和 receiver drop。
6. decoder 递归展开 zlib body；mapper 将支持的 CMD 变成 `LiveEvent`。
7. interaction end、API 心跳失败、socket 关闭或 protocol error 关闭 sender；任务 finally 路径尽力调用 end。
8. `LiveDataState` 收到 channel 关闭后按既有 5 秒/最多 10 次策略重新 start。

## 数据所有权与持久化

- transport 拥有短生命周期的 identity code、`game_id`、auth body 和 socket。
- Secret 仅存在于 Rust 进程内存，由 transport 配置持有；不持久化、不序列化、不传给 renderer。
- 领域事件继续由现有 `LiveRepository` 持久化。
- 不新增 migration。

## 一致性与事务策略

start 成功到 end 完成视为一个外部会话事务。任何中间失败均进入统一 cleanup。end 是 best effort，因为进程崩溃无法保证网络补偿；平台的 180 秒项目心跳超时是最终兜底。

## 幂等 / 重试 / 失败处理

- 单次 HTTP 请求不在 adapter 内自动重试，避免 nonce 复用和隐式叠加；上层会话重连拥有重试权。
- WSS 地址在同一次 start 内按返回顺序尝试。
- HTTP 5xx、网络、socket EOF、项目心跳失败为 retryable。
- 缺配置、无效身份码、签名/权限类业务错误和协议畸形为 non-retryable；平台 7001 冷却期、服务错误可 retryable。
- 未知 CMD 和单条字段无效事件被忽略；畸形 Proto 长度/zlib 为会话级错误。

## 观测性计划

- 日志只记录阶段、endpoint 名、平台 code、request_id、operation 和错误类别。
- 禁止记录 request body（含身份码）、签名头、Secret、auth body、完整事件 JSON。
- `RoomSnapshot.last_error_code` 保持稳定内部错误码，renderer 映射中文提示。

## 设计风险

- `RoomTransport` 参数类型变化会影响测试 doubles 和前端契约，需要全量编译/契约测试。
- Tauri 窗口销毁到进程退出的时间有限，end 只能 best effort；应用内主动断开可以正常触发清理。
- 正式服务 AUTH_REPLY 可能有文档未列出的 envelope，需要兼容顶层与 `data.code`。
- 修改任何开放平台凭证都需要重新编译和发布应用。

## 开放问题

- 真实账号级联调只需输入有效主播身份码；三项应用凭证已写死且不可运行时覆盖。
- Secret 会进入源码与安装包，可被提取；这是用户明确接受的发布风险。
- 不涉及 gRPC 或 Kubernetes，对应策略不适用。
