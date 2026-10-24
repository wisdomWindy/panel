# PRD 快照：哔哩哔哩开放平台长链弹幕

## 来源摘要

用户要求以仓库内 `description-doc` 为准接入 WebSocket 弹幕。核心资料分别定义统一鉴权、项目生命周期、长链二进制协议和 CMD 消息结构。

## 关键业务目标

将当前不可用的直播间传输占位实现替换为可工作的开放平台客户端，并复用既有过滤、落库、统计、榜单及前端事件流水线。

## 显式行为约束

- 所有 HTTP API 使用 `https://live-open.biliapi.com`、POST、JSON。
- 签名头包含 body MD5、秒级时间戳、UUID nonce、AccessKeyId、HMAC-SHA256 方法和 1.0 版本；Authorization 为小写十六进制 HMAC-SHA256。
- `/v2/app/start` 请求体包含主播身份码 `code` 与 64 位 `app_id`。
- 长链连接后第一包必须是 Operation 7 的鉴权包，Body 原样使用 `auth_body`。
- Proto 头固定 16 字节且全部大端；Version 2 使用 zlib 解压并递归解析内部多个完整 Proto 包。
- 长链心跳和 `/v2/app/heartbeat` 均按 20 秒间隔维持。
- 项目结束时调用 `/v2/app/end`，不得仅关闭 WebSocket。
- `wss_link` 可返回多个集群地址，连接失败时应依次兜底。
- HTTP 状态码为 200 不代表成功，必须检查响应 JSON 的 `code`。

## 接口、事件、数据模型与流程规则

- `POST /v2/app/start` -> `game_info.game_id`、`websocket_info.auth_body`、`websocket_info.wss_link[]`、`anchor_info`。
- `POST /v2/app/heartbeat` -> 维持项目会话。
- `POST /v2/app/end` -> 主动关闭项目。
- Operation：2 心跳、3 心跳回复、5 消息、7 鉴权、8 鉴权回复。
- 现有领域映射：`LIVE_OPEN_PLATFORM_DM` -> Danmu；`SEND_GIFT` -> Gift；`GUARD` -> Captain；`LIVE_ROOM_ENTER` -> Enter。
- `LIVE_OPEN_PLATFORM_INTERACTION_END` 表示当前 `game_id` 已失效，传输事件流应结束并交由上层重连策略处理。

## 相关服务或模块

`RoomTransport`、`RoomConnection`、`LiveDataState`、`LiveEvent`、`TauriLiveEventSink`、工具栏连接表单和前端 live-room store。

## 开放问题

- 真实联调仍需 `AccessKeyId` 和主播身份码。
- 文档没有定义对未知 CMD 的错误策略；建议忽略并记录不含消息体的调试信息。
- 文档未明确 AUTH_REPLY body 的完整错误结构；建议以可解析的 `code == 0` 为成功，非零或畸形响应结束连接。

