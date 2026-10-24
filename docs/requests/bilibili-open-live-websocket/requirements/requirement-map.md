# Requirement Map：哔哩哔哩开放平台长链弹幕

## 需求分析摘要

范围是以单一 `game_id` 为边界完成项目开启、WSS 消息、双心跳、事件映射和项目关闭。协议足以离线实现；真实联调所缺的凭证是外部输入，不影响模块拆分。不可拆散边界是 start/heartbeat/WSS/end 的会话生命周期。

## 请求摘要

替换 `UnavailableRoomTransport`，让现有直播数据流水线接入开放平台真实消息。

## 拆分原则

按集成事务边界拆分，不按 API 数量、文件目录或页面机械拆分。

## 来源清单

- 鉴权：`description-doc/哔哩哔哩直播开放文档 (3).md`
- 项目生命周期：`description-doc/哔哩哔哩直播开放文档 (4).md`
- 长链协议：`description-doc/哔哩哔哩直播开放文档 (6).md`
- 事件结构：`description-doc/哔哩哔哩直播开放文档 (7).md`
- 内部边界：`RoomTransport`、`LiveDataState`、`LiveEvent` 和 renderer connection flow。

## Markdown 归一化说明

来源已经是可读 Markdown，无需额外格式归一化，不生成重复快照。

## 模块清单

| 模块 ID | 名称 | 类型 | 来源章节 | 范围 | 需要设计 | 入口 | 工作区 |
| --- | --- | --- | --- | --- | --- | --- | --- |
| `open-live-session` | 开放平台会话集成 | integration flow | 鉴权、官方 API、长链协议、直播间数据 | 从身份码到现有领域事件的完整会话 | 是 | architecture-design | `modules/open-live-session` |

## 模块依赖与处理顺序说明

仅一个模块。内部实施顺序为配置/签名、Proto/事件映射、网络会话、装配/IPC/UI，最终按整条链路验收。

## 下游模块执行策略

`open-live-session` 单独经历 `architecture-design -> spec -> plan -> execute -> verify -> review`。只有 review 与 release-readiness 通过后，整个请求才能完成。

## architecture-design 候选项

`open-live-session` 必须设计：它新增外部服务 adapter、异步任务生命周期、取消/清理语义和前后端契约变化。

## spec 承接检查清单

- 凭证来源、缺失错误和明文隔离。
- 签名固定向量、请求体字节一致性、业务 code 检查。
- Proto 大端头、Version 0/2、嵌套多包和长度防御。
- 四类事件字段与单位换算、无效消息策略。
- WSS 地址回退、AUTH gate、双心跳、interaction end。
- 每个 start 成功会话的 end 尽力清理。
- 身份码 IPC/前端输入与真实房间号快照。
- 现有过滤、落库、统计和媒体消费者不回归。

## 开放问题与未解决歧义

真实平台联调仍需要有效 `AccessKeyId` 和主播身份码。AUTH_REPLY schema 采用兼容解析，待真实响应验证。

