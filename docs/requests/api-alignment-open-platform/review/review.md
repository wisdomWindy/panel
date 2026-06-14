# 审查报告: Bilibili 开放平台 API 对齐

**需求 ID**: api-alignment-open-platform
**审查日期**: 2026-06-11

---

## 审查摘要

| 分类 | 数量 |
|------|------|
| 阻塞问题 | 0 |
| 非阻塞问题 | 2 |
| 已接受风险 | 1 |
| 后续事项 | 3 |

---

## 阻塞问题

无。

---

## 非阻塞问题

### NB-1: auth.ts 使用纯 JS 实现 MD5/HMAC-SHA256

**严重程度**: 低
**描述**: `auth.ts` 中的 `md5Simple()` 和 `hmacSha256Simple()` 使用自定义纯 JavaScript 实现，代码体量较大且未经充分测试向量验证。
**建议**: 后续建议替换为 `crypto-js` 库，或使用 Node.js 内置 `crypto` 模块。
**影响**: `src/renderer/modules/bilibili/auth.ts`

### NB-2: danmaku-service.ts connect() 每次创建新 DanmakuManager

**严重程度**: 低
**描述**: `DanmakuService.connect()` 每次连接都创建新的 `DanmakuManager` 实例，旧实例的事件监听器可能未被清理。
**建议**: 添加 `this.manager.removeAllListeners()` 或重构为配置更新模式。
**影响**: `src/renderer/services/danmaku-service.ts`

---

## 已接受风险

### AR-1: 缺少真实 API 集成测试

**描述**: 缺少真实的 `access_key_id/secret` 和 `code`，无法执行端到端集成测试。
**缓解**: 代码实现严格遵循官方文档字段定义，CMD_TYPE_MAP 映射与官方文档一一对应，TypeScript 编译零错误。
**接受原因**: 集成测试需要真实凭据和直播环境。

---

## 后续事项

| ID | 说明 | 参考 |
|----|------|------|
| FU-1 | 创建 auth.test.ts / api.test.ts 单元测试 | T1.3, T2.2 |
| FU-2 | 完成 Connection.vue UI 更新 (身份码输入框) | T4.6 |
| FU-3 | 更新现有测试套件 (parser, protocol) | T5.1 |

---

## 审查结论

**状态**: ✅ 通过 — 无阻塞问题，可标记完成。