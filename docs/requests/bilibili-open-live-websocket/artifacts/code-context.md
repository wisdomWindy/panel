# 代码上下文：开放平台会话集成

## 上下文需求

确认连接入口、transport 端口、异步重连、领域事件、Tauri 发布和前端 store 的实际依赖关系。

## graph 可用性检查

当前工作区未暴露 code graph 工具或既有图索引。

## 安装或 bootstrap 记录

无可调用的自动 bootstrap 能力；未为本请求引入额外全局工具。

## fallback 记录

使用 `rg` 符号/调用点检索与逐文件阅读恢复依赖图。

## 相关入口点

- renderer：`AppShellToolbar.vue`、`useApplicationShellRuntime.ts`、`stores/live-data/room.ts`。
- IPC：`ipc/live_data.rs::room_connect`、`room_disconnect`。
- 应用层：`LiveDataState::connect_room`、`run_room_session`、`consume_connection`。
- 外部端口：`RoomTransport::connect` 与 `RoomConnection`。
- 装配：`lib.rs::AppState::new` 从环境构造 `OpenLiveConfig` 并注入 `BilibiliOpenLiveTransport`；缺配置时注入保留具体错误码的 `FailedRoomTransport`。
- 输出：`TauriLiveEventSink::publish_live_event`。

## 关键符号与模块

`RoomId`、`UserUid`、`LiveEvent`、`AudienceIdentityInput`、`LiveRuntimeError`、`RoomSnapshot`、`ConnectionStatus`、`BilibiliOpenLiveTransport`、`OpenLiveConfig`。

## 依赖与副作用边界

- `LiveDataState` 拥有重连、过滤、统计、持久化和发布。
- transport 只应拥有远端网络、协议解析和 event channel。
- renderer 只拥有身份码输入和状态展示。
- 数据库不应接触开放平台凭证。

## 影响范围

Rust live-data application/infrastructure/lib 装配、Cargo 依赖、IPC DTO；renderer desktop contracts/store/shell runtime/toolbar/i18n 及测试。

## 开放后续检查项

所有 `RoomTransport` test doubles、`RoomConnectRequestDto` 调用方和应用退出路径均已迁移。appid、AccessKeyId 与 Secret 已成为不可运行时覆盖的后端固定配置；剩余外部检查仅为使用有效主播身份码执行正式平台 smoke test。Secret 可从源码和安装包中提取。
