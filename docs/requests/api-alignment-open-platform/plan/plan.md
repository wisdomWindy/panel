# 实施计划: Bilibili 开放平台 API 对齐

**需求 ID**: api-alignment-open-platform
**版本**: 1.0
**状态**: 待审批
**创建日期**: 2026-06-11

---

## 一、任务分解

### 阶段 1: 基础层 — 类型定义 + 鉴权

此阶段建立全新的类型系统和鉴权基础设施，后续所有阶段依赖此阶段完成。

| 任务 | 文件 | 说明 | 依赖 |
|------|------|------|------|
| T1.1 | `src/renderer/modules/bilibili/types.ts` | 重写类型定义：移除旧类型 (`AuthParams`, `DanmakuInfo`, `MedalInfo`)、新增开放平台类型 (所有 `OpenPlatform*` 接口 + 生命周期响应 + 配置类型) | 无 |
| T1.2 | `src/renderer/modules/bilibili/auth.ts` (新建) | 实现 HMAC-SHA256 签名：`generateAuthHeaders()`、MD5 计算、header 拼接、签名生成 | 无 |
| T1.3 | `src/renderer/modules/bilibili/auth.test.ts` (新建) | 签名单元测试：验证与官方 Python demo 相同输入时输出一致的签名值 | T1.2 |

### 阶段 2: HTTP API 层

此阶段封装所有开放平台 HTTP 接口，建立项目生命周期管理。

| 任务 | 文件 | 说明 | 依赖 |
|------|------|------|------|
| T2.1 | `src/renderer/modules/bilibili/api.ts` (新建) | 实现 `OpenPlatformAPI` 类：`start()`, `end()`, `heartbeat()`, `batchHeartbeat()`，自动附加 auth headers | T1.1, T1.2 |
| T2.2 | `src/renderer/modules/bilibili/api.test.ts` (新建) | API 调用单元测试：验证请求格式正确、错误码处理 | T2.1 |

### 阶段 3: WebSocket 协议 + 客户端

此阶段重写协议解析器和客户端连接逻辑，实现完整的 `start → heartbeat → end` 生命周期。

| 任务 | 文件 | 说明 | 依赖 |
|------|------|------|------|
| T3.1 | `src/renderer/modules/bilibili/protocol.ts` | 重写协议解析：保留二进制包头解析，移除旧 cmd 解析 (`DANMU_MSG`, `SEND_GIFT`, `INTERACT_WORD`)，新增 11 种 `LIVE_OPEN_PLATFORM_*` cmd 解析 | T1.1 |
| T3.2 | `src/renderer/modules/bilibili/client.ts` | 重写客户端：移除旧流程 (`getRoomInfo`, `getAuthInfo`, `getWebSocketUrl`, `authenticate`)，实现新流程 (start → wss_link → auth_body → 双心跳 → end) | T1.1, T2.1, T3.1 |
| T3.3 | 新建测试 | 协议解析测试：验证 11 种 cmd 消息正确解析为对应类型 | T3.1 |

### 阶段 4: 上层适配

此阶段更新依赖 bilibili 模块的上层代码，确保 `DanmakuManager`、`DanmakuService`、`parser` 适配新的消息结构。

| 任务 | 文件 | 说明 | 依赖 |
|------|------|------|------|
| T4.1 | `src/renderer/modules/danmaku/parser.ts` | 重写 `DanmakuParser`: 从旧的数组索引解析改为直接映射 `OpenPlatformDanmakuMessage` 扁平字段 | T1.1, T3.2 |
| T4.2 | `src/renderer/modules/danmaku/manager.ts` | 更新 `DanmakuManager`: 适配新 BilibiliClient 接口（使用 code+appId 连接），`handleGift` 适配新 Gift 结构 | T1.1, T3.2 |
| T4.3 | `src/renderer/stores/config.ts` | 新增 `accessKeyId`, `accessKeySecret`, `appId` 配置字段 | T1.1 |
| T4.4 | `src/renderer/stores/room.ts` | 更新 RoomStore：添加 `code`, `appId` 字段，使用 `open_id` 和 `union_id` 替代 `uid` | T1.1 |
| T4.5 | `src/renderer/services/danmaku-service.ts` | 更新 `DanmakuService`: 适配新客户端连接方式 (code)，礼物处理适配新类型 | T4.1, T4.2 |
| T4.6 | `src/renderer/views/Connection.vue` | 新增身份码 (code) 和 app_id 输入框，调整连接按钮逻辑 | T4.3, T4.4 |

### 阶段 5: 测试与验证

| 任务 | 文件 | 说明 | 依赖 |
|------|------|------|------|
| T5.1 | 运行现有测试 | 确保现有测试套件通过，修复因类型变更导致的编译错误 | T4.1-T4.6 |
| T5.2 | 手动集成测试 | 使用真实 access_key/身份码 验证完整生命周期流程 | 所有 |

---

## 二、依赖关系图

```
阶段 1 (基础层)
  T1.1 types.ts 重写 ──┬── T1.2 auth.ts 新建
                       │     └── T1.3 auth 测试
                       │
阶段 2 (HTTP API)      │
                       ├── T2.1 api.ts 新建
                       │     └── T2.2 api 测试
                       │
阶段 3 (协议+客户端)   │
                       ├── T3.1 protocol.ts 重写
                       ├── T3.2 client.ts 重写 ── T3.3 协议测试
                       │
阶段 4 (上层适配)      │
                       ├── T4.1 parser.ts 重写
                       ├── T4.2 manager.ts 更新
                       ├── T4.3 config store 更新
                       ├── T4.4 room store 更新
                       ├── T4.5 danmaku-service.ts 更新
                       └── T4.6 Connection.vue 更新
                       │
阶段 5 (测试验证)
                       └── T5.1 现有测试 ── T5.2 集成测试
```

---

## 三、受影响的区域

### 直接修改

| 文件 | 变更类型 |
|------|---------|
| `src/renderer/modules/bilibili/types.ts` | 重写 |
| `src/renderer/modules/bilibili/client.ts` | 重写 |
| `src/renderer/modules/bilibili/protocol.ts` | 重写 |
| `src/renderer/modules/danmaku/parser.ts` | 重写 |
| `src/renderer/modules/danmaku/manager.ts` | 更新 |
| `src/renderer/modules/danmaku/types.ts` | 更新 (Gift 接口新增字段) |
| `src/renderer/services/danmaku-service.ts` | 更新 |
| `src/renderer/stores/config.ts` | 更新 |
| `src/renderer/stores/room.ts` | 更新 |
| `src/renderer/views/Connection.vue` | 更新 |

### 新增文件

| 文件 | 说明 |
|------|------|
| `src/renderer/modules/bilibili/auth.ts` | HMAC-SHA256 签名 |
| `src/renderer/modules/bilibili/api.ts` | HTTP API 封装 |
| `src/renderer/modules/bilibili/auth.test.ts` | 签名测试 |
| `src/renderer/modules/bilibili/api.test.ts` | API 测试 |

### 保持不变

| 文件 | 原因 |
|------|------|
| `src/renderer/modules/danmaku/sender.ts` | 弹幕发送逻辑不依赖 BilibiliClient 协议层 |
| `src/renderer/modules/danmaku/blacklist.ts` | userId 字段名不变，值改用 open_id 对接口无影响 |
| `src/renderer/modules/effects/*` | 特效管理不依赖消息协议 |
| `src/renderer/views/*.vue` (除 Connection.vue) | 视图层无需更改 |
| `src/main/*` | 主进程无需更改 |
| `src/preload/index.ts` | 预加载脚本无需更改 |

---

## 四、测试策略

### 单元测试 (TDD)

| 测试 | 覆盖内容 | 验收标准 |
|------|---------|---------|
| auth.test.ts | `generateAuthHeaders()` 签名正确性 | 与官方 Python demo 输出一致 |
| api.test.ts | API 调用参数正确编码 | start/end/heartbeat 请求格式正确 |
| protocol.test.ts | 11 种 cmd 新消息解析 | 每种消息类型正确映射到对应接口 |

### 回归测试

运行现有测试套件确保破坏性变更被识别和修复：
- `tests/danmaku-parser.test.ts` — 需更新以匹配新的消息结构
- `tests/protocol.test.ts` — 需更新以匹配新的协议解析
- `tests/danmaku-types.test.ts` — 检查类型兼容性
- `tests/variable-replacer.test.ts` — 无需变更
- `tests/sensitive-words.test.ts` — 无需变更
- `tests/blacklist.test.ts` — userId 字段名保持不变

### 集成测试 (手动)

1. 使用真实 access_key_id/secret 调用 `/v2/app/start`
2. 验证返回的 `wss_link` 可连接
3. 使用 `auth_body` 认证 WebSocket
4. 验证收到 `LIVE_OPEN_PLATFORM_DM` 弹幕消息
5. 验证收到 `LIVE_OPEN_PLATFORM_SEND_GIFT` 礼物消息
6. 验证心跳保持连接 (>60秒)
7. 验证 `/v2/app/end` 正常关闭

---

## 五、回滚说明

如果迁移过程中遇到不可克服的问题：

1. **Git 分支**: 在独立分支 `feature/api-alignment-open-platform` 上工作
2. **回退方式**: 直接切回 `main` 分支即可完整回退
3. **回退风险**: 无数据库迁移，无持久化状态变更，回退无副作用
4. **部分回退**: 如果仅鉴权层有问题，可以保留 T1 阶段内容，单独修复 T2/T3

---

## 六、执行顺序

按阶段顺序执行：阶段 1 → 阶段 2 → 阶段 3 → 阶段 4 → 阶段 5

每个阶段内部任务可部分并行：
- 阶段 1: T1.1 → T1.2 → T1.3
- 阶段 2: T2.1 → T2.2
- 阶段 3: T3.1 → T3.2 → T3.3
- 阶段 4: T4.3, T4.4 可并行 → T4.1, T4.2 可并行 → T4.5 → T4.6
- 阶段 5: T5.1 → T5.2