# 验证报告：开放平台会话集成

## 模块标识与范围

模块 `open-live-session`；验证官方签名 HTTP、WSS/Proto、事件映射、应用装配、renderer 契约、安全与发布产物。

## 验收项覆盖

### AC1 官方签名向量

- 验收项 ID：AC1
- 验证方法：运行 open-live 单元测试中的官方 canonical header/HMAC 固定向量。
- 结果：通过。
- 证据引用：`cargo test --features free`，包含 `signs_the_official_document_header_vector`，free 共 115 项通过。
- 失败后续动作：不适用。
- 交接状态：完成。

### AC2 HTTP body 与 envelope

- 验收项 ID：AC2
- 验证方法：本地 HTTP server 校验 start/heartbeat/end 路径、JSON body、MD5 与签名头，并解析成功响应。
- 结果：通过。
- 证据引用：`sends_signed_start_heartbeat_and_end_requests`；`cargo test --features free`。
- 失败后续动作：不适用。
- 交接状态：完成。

### AC3 Proto 与 zlib

- 验收项 ID：AC3
- 验证方法：round-trip、多包、Version 2 zlib 递归、截断与非法长度测试。
- 结果：通过。
- 证据引用：open-live protocol tests；最大 body 8 MiB、递归深度 8 的实现边界。
- 失败后续动作：不适用。
- 交接状态：完成。

### AC4 事件映射

- 验收项 ID：AC4
- 验证方法：DM、SEND_GIFT、GUARD、LIVE_ROOM_ENTER fixture 与未知命令测试；字符串 `open_id` 领域测试。
- 结果：通过。
- 证据引用：open-live mapper tests、`domain::live_data::tests`；free/member 全套测试通过。
- 失败后续动作：不适用。
- 交接状态：完成。

### AC5 生命周期

- 验收项 ID：AC5
- 验证方法：本地 HTTP+WebSocket server 验证 start、AUTH、事件、WS/API 双心跳、receiver drop 后 end；断开优先于到期心跳。
- 结果：通过。
- 证据引用：`authenticates_websocket_forwards_events_and_ends_the_session` 先暴露竞争后转绿；窗口 Destroyed 调用 `disconnect_room`。
- 失败后续动作：不适用。
- 交接状态：完成。

### AC6 缺配置降级

- 验收项 ID：AC6
- 验证方法：配置校验测试、生产 `FailedRoomTransport` 装配、异步状态错误码到本地化提示映射测试。
- 结果：通过。
- 证据引用：`validates_configuration_without_exposing_the_secret`、`maps asynchronous open-live failures to actionable messages`、free release build。
- 失败后续动作：不适用。
- 交接状态：完成。

### AC7 renderer identityCode 契约

- 验收项 ID：AC7
- 验证方法：IPC DTO serde 测试、store invoke 契约、输入规范化、toolbar 语义与 roomId 响应状态测试。
- 结果：通过。
- 证据引用：frontend 124 项测试；Rust DTO/application tests；TypeScript type-check。
- 失败后续动作：不适用。
- 交接状态：完成。

### AC8 全套测试与构建

- 验收项 ID：AC8
- 验证方法：执行 Rust free/member 全套测试、Vitest 全套、type-check、ESLint、web build、Tauri release build。
- 结果：通过。
- 证据引用：T8 后 free 115 项、member 132 项、Vitest 124 项；type-check exit 0；ESLint 0 error（保留既有 logger 2 warnings）；`biliLiveHelper_0.1.3_x64-setup.exe` 重新构建成功。
- 失败后续动作：不适用。
- 交接状态：完成。

### AC9 Secret 分发边界扫描

- 验收项 ID：AC9
- 验证方法：以分段构造的 needle 扫描 Rust/renderer 源码、release EXE 和 NSIS 安装包，并检查凭证环境变量名。
- 结果：通过；Rust 仅 `config.rs` 1 处命中，renderer 0 命中，release EXE 命中 Secret 且不命中 Secret 环境变量名。
- 证据引用：本轮 PowerShell 二进制/文本扫描输出；NSIS 原始字符串因压缩不命中，但安装后 EXE 可提取，不视为保密。
- 失败后续动作：不适用。
- 交接状态：完成。

### AC10 项目固定完整应用凭证

- 验收项 ID：AC10
- 验证方法：调用 `OpenLiveConfig::embedded()` 并校验 appid、AccessKeyId 以及独立计算的 HMAC 固定结果；检查 open-live 源码不包含凭证环境变量名或 `std::env`。
- 结果：通过。
- 证据引用：`uses_embedded_credentials_to_sign_requests`；聚焦测试经历 RED 编译失败后转 GREEN；源码凭证环境变量名 0 命中。
- 失败后续动作：不适用。
- 交接状态：完成。

## 规格约束符合性

规格约束符合性: 通过

- 结果：通过。
- 已检查约束：官方 snake_case 边界、10 秒 HTTP timeout、上层重试所有权、双心跳、AUTH gate、无隐式 HTTP retry、未知 CMD 忽略、统一 cleanup、前端不接触凭证、无数据库迁移。
- Spec-plan 粒度对齐：通过；T1-T8 覆盖配置签名、协议映射、会话、契约迁移、发布验证与固定完整应用凭证，没有把完整会话拆成不可独立交付的片段。
- 契约一致性：通过；`identityCode` 是唯一有意 breaking input change，响应 DTO 与事件名保持不变。
- 幂等：start/end 本身由平台定义；客户端不重试单次 HTTP 请求，避免叠加副作用；cleanup 为 best effort。
- 迁移与回滚：无 schema migration；恢复 unavailable transport 与旧入参即可代码回滚。
- Rollout readiness：离线发布证据齐备；正式服务 smoke test 仅需有效主播身份码，不能由当前证据替代。
- 中间件专项：MySQL、Redis、Kafka、gRPC、Kubernetes 均不适用。
- 证据引用：approved spec、plan、execution changelog、上述测试与构建结果。
- 失败后续动作：不适用。

## 安全验证

安全验证: 通过

- 结果：通过。
- 已检查范围：auth 签名、Secret 隔离与 Debug 脱敏、renderer 隔离、日志脱敏、identity/auth body 不记录、包长/解压/递归边界、控制字符校验、重试与 replay 所有权、PII 最小传递。
- Auth/authz：通过；所有 API 请求均签名，WSS 必须收到成功 AUTH_REPLY 才开放事件流。
- Secret：通过（已接受分发泄露风险）；仅在 Rust `config.rs` 中固定定义，不进入 renderer、SQLite、日志或 Debug 输出；release EXE 中可提取。
- PII：通过；仅把事件处理所需 open_id/昵称映射到现有领域对象，不新增持久化字段。
- Abuse/replay：通过；限制包体与递归深度，HTTP 无隐式重试，未知/畸形事件不进入下游。
- 证据引用：config/signing/protocol/transport tests 和产物扫描。
- 失败后续动作：不适用。

## 总结

离线验收通过，可生成并安装 free 版。appid、AccessKeyId 与 Secret 已全部写死且不从环境变量读取；正式平台联调仍需有效主播身份码，并在有效直播场次中执行。
