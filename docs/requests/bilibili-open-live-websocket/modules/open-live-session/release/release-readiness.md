# 发布就绪：开放平台会话集成

## 模块标识与范围

模块 `open-live-session`；free/member 共用 Rust 会话实现，当前生成 free NSIS 安装包。

## rollout 策略

- 安装 free 版并离线启动，确认应用其余页面正常；三项开放平台凭证已经随 Rust 后端交付，不需要设置环境变量。
- 使用有效主播身份码在已开播、有效互动场次中连接，观察状态从 Connecting 到 Connected，并确认弹幕进入现有弹幕面板。
- 首次正式联调采用单主播、短观察窗；不做批量部署或并发场次扩展。

## 迁移顺序

1. 安装新版本（前后端 `identityCode` 契约原子发布）。
2. 原有开放平台凭证环境变量会被忽略，无需保留或设置；如需切换开放平台应用，必须修改 Rust 固定值并重新编译。
3. 输入主播身份码连接。
4. 无数据库 schema/data migration。

## 监控观察窗

- 首次连接后观察至少 2 个 20 秒周期，确认 WS 和 API heartbeat 均维持会话。
- 关注 UI 连接状态、重连次数、非敏感 error code、弹幕事件到达和主动断开行为。
- 日志不得包含 AccessKey、Secret、identity code、auth body 或完整事件 payload。

## 回滚触发条件与执行路径

- 触发：持续鉴权失败、弹幕不达、异常重连、退出未清理或出现凭证泄露迹象。
- 执行：停止新版本，卸载并恢复上一安装版本；代码层可恢复 `UnavailableRoomTransport` 和旧 `roomId` 入参。
- 凭证：卸载或回滚不能撤销已被提取的内置 Secret；若怀疑泄露，必须在平台侧撤销/轮换并重新构建发布。
- 数据：无 schema 变化，不需要数据回滚。
- 外部场次：如新版本未正常 end，等待平台 heartbeat 超时回收或使用平台侧场次管理终止。

## 剩余发布风险

- 缺有效主播身份码，正式平台 smoke test 尚未执行。
- 平台权限、appid 绑定、身份码有效期和直播场次状态只能在正式联调验证。
- appid、AccessKeyId 与 Secret 都可从客户端分发物中提取；这是用户明确要求并接受的发布风险。
- 进程强杀时 end 为 best effort；正常 UI 断开和窗口销毁路径已验证。

## 最终结果

结果: 通过

- 结果：通过。
- 阻塞缺口：无离线发布 blocker；正式上线前需使用有效主播身份码完成单场次 smoke test。
- 证据引用：verification、review；Rust free 115/member 132 项，Vitest 124 项，type-check 与 Lint 0 error，release NSIS build 成功；源码仅 `config.rs` 1 处命中 Secret，renderer 0 处，release EXE 命中 Secret 且不含旧凭证环境变量名。
