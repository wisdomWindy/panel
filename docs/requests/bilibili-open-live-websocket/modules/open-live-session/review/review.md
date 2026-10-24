# 代码评审：开放平台会话集成

## 模块标识与范围

模块 `open-live-session`；评审配置、签名、HTTP/WSS、Proto、事件映射、应用状态机、IPC 与 renderer 迁移。

## 阻塞问题

无剩余阻塞问题。

评审中发现并已修复：

- WSS 建连缺少单地址超时，可能阻断地址回退；增加 10 秒 timeout 和本地双地址回退测试。
- Proto header length 允许大于 16，与文档固定值不符；改为严格等于 16 并补畸形包测试。
- `OpenLiveConfig::Debug` 会显示 AccessKeyId；统一脱敏 AccessKeyId 与 Secret 并补测试。
- appid/AccessKeyId 覆盖路径与最新需求冲突；已删除两个环境查询及默认回退辅助函数，并以只查询 Secret 的回归测试锁定行为。
- Secret 环境读取路径与最终需求冲突；已删除 `from_env`/`from_lookup`，统一从 `embedded()` 取得三项固定凭证，并以独立 HMAC 固定结果锁定签名行为。

## 非阻塞问题

- release 前端 chunk 超过 550 kB，Vite 给出体积 warning；属于既有整体前端拆包问题，不影响本模块正确性。
- ESLint 保留 `modules/utils/logger.ts` 的 2 条既有 `no-console` warning；0 error，与本模块无关。

## 已接受风险

- 未使用正式平台执行 smoke test，因为缺少有效主播身份码；离线 mock 生命周期不能证明账号权限、appid 绑定和有效场次状态。
- appid、AccessKeyId 与 Secret 均按需求编译进 Rust 后端。获得源码、EXE 或安装包的用户可以提取这些值，因此三项凭证都必须按已公开凭证处理；轮换需要先在平台侧撤销旧凭证，再修改常量并重新构建发布。
- 强杀进程无法保证 `/v2/app/end`，平台依靠 heartbeat 超时最终回收；正常断开和窗口销毁均已 cleanup。

## 后续事项

- 获取有效主播身份码后，在直播场次内验证 start、AUTH、弹幕接收、断开 end。
- 如后续需要真正保密凭证，应改为服务端代签或受控凭证服务；客户端内置无法形成保密边界，本次明确不纳入。

## 整洁代码评估

整洁代码评估: 通过

- 结果：通过。
- 关键发现：配置、签名、协议、映射、API、transport 各自单一职责；网络副作用未进入 domain/application；公开面仅导出 transport 与 config。
- 失败时必须跟进的动作：不适用。

## 设计模式评估

设计模式评估: 通过

- 结果：通过。
- 关键发现：沿用现有 `RoomTransport` Ports and Adapters 边界，真实 provider 作为 integration adapter；没有引入单实现 factory/strategy。该模式隔离官方 snake_case、签名与二进制协议，属于必要而非过度设计。
- Spec-plan 行为颗粒度：对齐；完整 game lifecycle 仍由一个模块交付。
- 失败时必须跟进的动作：不适用。

## 安全评估

安全评估: 通过

- 结果：通过。
- 关键发现：三项凭证均为不可运行时覆盖的 Rust 固定值；renderer、SQLite、Debug 和日志不接收凭证；API 签名与 AUTH gate 有测试；二进制包长、解压大小、递归深度、建连/鉴权/HTTP timeout 均有边界。扫描确认 Rust 源码仅 `config.rs` 1 处包含 Secret、renderer 0 处、release EXE 包含 Secret 且不包含旧环境变量名，符合用户明确接受的分发风险。
- 中间件规范：MySQL、Redis、Kafka、gRPC、Kubernetes 不适用；SQLite schema 未变。
- 失败时必须跟进的动作：不适用。

## 代码上下文结构评估

实现后的入口链路为 Toolbar -> IPC DTO -> `LiveDataState` -> `RoomTransport` -> Open Live API/WSS -> `LiveEvent` -> 原有过滤/存储/统计/媒体。所有 test doubles、入站 DTO 和退出路径均已迁移，无遗漏调用点。

## 合并准备度总结

评审通过，blocker 为 0。free/member 全套测试、前端全套测试、类型/Lint、release build 与凭证边界扫描均有新鲜证据；正式平台 smoke test 作为发布观察项保留，不冒充已完成。
