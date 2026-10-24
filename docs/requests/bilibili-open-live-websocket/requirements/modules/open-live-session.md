# 模块：开放平台会话集成

## 模块标识

`open-live-session`

## 模块名称

开放平台会话集成

## 来源快照

见请求级 `artifacts/prd-snapshot.md`，覆盖鉴权、start/heartbeat/end、Proto、CMD 四组约束。

## Markdown 归一化快照

不适用；来源已经是 Markdown。

## 来源追踪引用

- `(3).md`：公共头、签名字符串、HMAC 和错误码。
- `(4).md`：`/v2/app/start`、`heartbeat`、`end` 与 `game_id` 生命周期。
- `(6).md`：AUTH、心跳、16 字节大端头、zlib 和 Operation。
- `(7).md`：DM、SEND_GIFT、GUARD、LIVE_ROOM_ENTER、INTERACTION_END。

## 业务意图

为现有直播工具提供官方、可维护且不会泄漏密钥的真实消息源。

## 调用方可见范围

- 工具栏输入主播身份码并连接。
- 连接后快照显示开放平台返回的房间号与主播昵称。
- 弹幕、礼物、上舰、进入事件按现有 UI 展示和业务规则处理。
- 配置或平台错误返回明确、可定位的错误信息。

## 接口、事件、数据与流程规则

执行 `start -> WSS/AUTH -> WS heartbeat + API heartbeat + message consume -> end`。映射四类 CMD 到 `LiveEvent`；忽略未知 CMD；畸形 Proto 为会话错误，单条无效业务消息只丢弃该条。

## 依赖关系与受影响邻接模块

依赖 HTTP/WSS/zlib/crypto crates；邻接 `LiveDataState`、Tauri IPC、application shell store/toolbar。下游过滤、统计与媒体模块只消费现有 `LiveEvent`，不修改契约。

## architecture-design 路由决策

需要。理由：新增外部适配器、后台任务、取消清理和跨边界契约。

## 下游入口阶段

`architecture-design`

## 下游模块工作区

`docs/requests/bilibili-open-live-websocket/modules/open-live-session/`

## 下游 spec 义务

必须给出配置变量、错误码、签名/Proto API、事件映射、生命周期状态、清理保证、UI 契约和可观察验收标准。

## 开放问题

真实联调凭证与主播身份码待提供；不影响离线实现。

