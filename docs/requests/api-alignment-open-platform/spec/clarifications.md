# 澄清记录: Bilibili 开放平台 API 对齐

**需求 ID**: api-alignment-open-platform
**创建日期**: 2026-06-11

---

## 问题 1: 旧版 API 兼容性处理策略

**提问**: 项目是否需要在过渡期同时支持旧版广播协议和新版开放平台 API？

**回答**: 不需要。根据官方文档 (#2)，旧版互动玩法 API 已于 2022 年 6 月 15 日停止支持。本次迁移为完整替换，不保留旧版协议兼容层。

**最终决策**: 完全替换，不保留旧版兼容代码。

**影响范围**: `types.ts`, `client.ts`, `protocol.ts`

---

## 问题 2: 用户标识 (uid → open_id) 迁移策略

**提问**: 项目中大量使用 `uid` 作为用户唯一标识（黑名单、赠送记录等），如何迁移到 `open_id`？

**回答**: `uid` 在开放平台中已废弃并固定为 0，必须使用 `open_id`。上层代码（DanmakuManager, DanmakuService）中所有基于 userId 的逻辑需同步替换。外部接口（如 ParsedDanmaku.userId）保持字段名不变但值使用 open_id。

**最终决策**: 内部存储和逻辑全部使用 `open_id`，外部接口字段名保持 `userId` 表示用户标识但值使用 `open_id`。

**影响范围**: `types.ts`, `manager.ts`, `parser.ts`, `danmaku-service.ts`, `blacklist.ts`

---

## 问题 3: Access Key 配置存储位置

**提问**: `access_key_id` 和 `access_key_secret` 存放在哪里？是否需要加密？

**回答**: 存放在应用的配置文件中（`config.ts` store），与现有的 `cookie`、`csrfToken` 同级别管理。由于 `access_key_secret` 为敏感信息，建议在渲染进程内存中持有，不持久化到明文存储；如需持久化，应使用操作系统密钥链或主进程加密存储。

**最终决策**: 添加 `accessKeyId` 和 `accessKeySecret` 到 `ConfigStore`，当前阶段存储为内存变量（类似 csrfToken），后续可升级为安全存储。

**影响范围**: `stores/config.ts`, UI 设置页

---

## 问题 4: 身份码 (code) 的获取方式

**提问**: 项目如何获取主播的身份码 (code)？是否需要新增 UI？

**回答**: 根据文档 (#4)，身份码可以通过以下方式获取：
1. 直播姬启动时通过 args 传递 `code=XXXX`
2. 主播在互动应用中心手动获取

当前项目已有 `Connection.vue` 页面供用户输入 roomId 和 cookie。需新增 `code` 输入框，替代原来的 roomId 作为连接凭据。

**最终决策**: 在 `Connection.vue` 中新增身份码输入框，连接使用 `code` + `app_id` 代替 `roomId`。

**影响范围**: `Connection.vue`, `stores/room.ts`, `client.ts`

---

## 问题 5: 项目心跳间隔调整

**提问**: 官方文档要求心跳间隔为 20 秒，项目当前心跳为 30 秒，如何调整？

**回答**: 调整为 20 秒以符合官方规范。互动玩法超过 60 秒无心跳会自动关闭，因此 20 秒间隔可提供 3 次容错窗口。

**最终决策**: 心跳间隔统一调整为 20 秒，WebSocket 心跳和 HTTP 心跳同步。

**影响范围**: `client.ts`, `config.ts`

---

## 问题 6: 新消息类型是否需要全部实现

**提问**: 官方文档列出了 11 种新消息类型，项目是否需要全部处理？

**回答**: 全部在 `protocol.ts` 中定义解析逻辑，但部分消息类型（如付费留言删除、跨房弹幕）当前业务层暂不使用，可在后续迭代中启用。核心消息（弹幕、礼物、大航海、进入房间、直播开始/结束）需完整实现到客户端事件。

**最终决策**: 在 `protocol.ts` 中解析全部 11 种消息类型；在 `client.ts` 中为以下 7 种核心类型触发事件：弹幕、礼物、大航海、进入房间、点赞、直播开始、直播结束。其余类型预留扩展点。

**影响范围**: `protocol.ts`, `client.ts`, `types.ts`

---

## 问

[User Cancelled]