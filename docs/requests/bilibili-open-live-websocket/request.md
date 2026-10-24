# 请求：接入哔哩哔哩开放平台长链弹幕

## 请求标识

`bilibili-open-live-websocket`

## 来源链接

- 用户直接请求：按仓库 `description-doc/` 中的开放平台文档连接 WebSocket 获取弹幕。
- 协议来源：`description-doc/哔哩哔哩直播开放文档 (3).md`、`(4).md`、`(6).md`、`(7).md`。

## 业务摘要

用真实的哔哩哔哩直播开放平台传输实现替换当前 `UnavailableRoomTransport`，完成项目开启、长链鉴权、双心跳、消息解包、事件映射、断线清理和项目关闭。

## 目标陈述

用户输入主播身份码后，桌面应用能够通过已配置的开放平台应用凭证开启直播项目，建立 WSS 长链，并把弹幕、礼物、上舰和进入直播间消息送入现有 `LiveDataState` 流水线。

## 初始完成信号

- `/v2/app/start` 请求签名符合本地开放平台文档。
- WebSocket 首包为 `OP_AUTH`，随后每 20 秒发送长链心跳和项目 API 心跳。
- Version 0 与 Version 2 数据包均可解析，压缩包内多个嵌套 Proto 包不会丢失。
- 四类现有领域事件能从开放平台 CMD 映射并通过现有 Tauri 事件发布。
- 用户断开、连接结束和应用退出时尽力调用 `/v2/app/end`。
- Access Key Secret 按用户明确要求写入 Rust 后端，但不进入数据库、日志、Debug 输出或前端。

## 触发条件

用户在应用工具栏输入主播身份码并点击连接。

## 初始上下文来源

开放平台协议文档、现有 `RoomTransport` 端口、`LiveDataState` 会话状态机、`LiveEvent` 领域模型和 Tauri IPC/前端连接入口。

## 人工交接点

项目将用户指定的 `appid`、`AccessKeyId` 与 `Access Key Secret` 全部写为 Rust 后端固定常量，不从环境变量读取或覆盖。用户仅需在界面输入主播身份码；缺少有效主播身份码会阻止真实平台联调。

## 影响范围

Rust 基础设施传输层、依赖配置、应用装配、连接 IPC 契约、前端工具栏输入语义、错误映射及相应测试。

## 参与服务或模块

- 哔哩哔哩直播开放平台 HTTP API 与 WSS 长链
- `application/live_data`
- `infrastructure/live_data`
- `ipc/live_data`
- renderer application shell 与 live-data store
