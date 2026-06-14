# Bugfix 来源: API 对齐代码缺陷报告

**审查日期**: 2026-06-11

---

## Bug 1 (中): client.ts connectWebSocket() 中超时与 error 事件重复调用 tryConnect

**位置**: `src/renderer/modules/bilibili/client.ts:147-181`

**问题**: 当 WebSocket 连接超时时：
1. `setTimeout` (line 159) 触发 → `ws.close()` → `tryConnect(index + 1)`
2. `ws.close()` 触发 `error` 事件 (line 172) → `clearTimeout(timeout)` → `tryConnect(index + 1)`

结果 `tryConnect` 在同一索引上被调用两次，产生重复的 WebSocket 连接。

**修复**: 添加 `isResolved` 标志防止重复调用。

---

## Bug 2 (中): parser.ts 中 owner 匹配永远失败

**位置**: `src/renderer/modules/danmaku/parser.ts:55`, `src/renderer/modules/bilibili/client.ts:91`

**问题**: `/v2/app/start` 返回的 `anchor_info.uid` 是数字（传统 UID），而弹幕消息中 `open_id` 是 UUID 字符串。`DanmakuParser.determineUserType` 比较 `data.open_id === ownerId.toString()` 永远不匹配，因为 UID(数字) ≠ open_id(UUID)。

**修复**: 使用 `anchor_info.open_id` 作为 owner 标识符。需要在 `RoomInfo` 和 `parser.ts` 中同步修改。

---

## Bug 3 (中): danmaku-service.ts connect() 每次创建新 DanmakuManager 导致事件监听器泄漏

**位置**: `src/renderer/services/danmaku-service.ts:148-159`

**问题**: 每次调用 `connect()` 创建新的 `DanmakuManager` 实例，旧实例的 EventEmitter 监听器未被释放，导致内存泄漏。

**修复**: 在创建新实例前调用 `this.manager.removeAllListeners()`。

---

## Bug 4 (低): client.ts authenticate() 未等待服务端 AUTH_REPLY

**位置**: `src/renderer/modules/bilibili/client.ts:187-193`

**问题**: `authenticate()` 在发送 AUTH 包后立即设置 `this.isAuthenticated = true`，未验证服务端的 `OP_AUTH_REPLY` 响应，可能在认证失败时继续操作。

**修复**: 在 `handleMessage` 中监听 `OP_AUTH_REPLY`，收到成功响应后再设置 `isAuthenticated = true`。

---

## Bug 5 (低): client.ts messageBuffer 可能无限增长

**位置**: `src/renderer/modules/bilibili/client.ts:219-280`

**问题**: 如果收到不完整的消息头（16 字节内），`messageBuffer` 会无限累积而不报错或丢弃无效数据。

**修复**: 添加缓冲区大小限制，超过上限后丢弃并重置。

---

## Bug 6 (低): protocol.ts parseHeader 字节序使用大端序

**位置**: `src/renderer/modules/bilibili/protocol.ts:54-57`

**问题**: 代码注释说"大端序"且使用 `view.getUint32(0, false)`。但 bilibili 广播协议的传统实现使用**小端序**。官方文档(#6)说"所有字段 大端 对齐"但历史代码通常用小端。此处可能造成消息解析错误。

**状态**: 需确认 - 如果开放平台新协议使用大端序则无问题。