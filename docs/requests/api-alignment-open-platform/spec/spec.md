# 技术规格: Bilibili 开放平台 API 对齐

**需求 ID**: api-alignment-open-platform
**版本**: 1.0
**状态**: 草稿
**创建日期**: 2026-06-11

---

## 背景与目标

### 背景

当前项目使用 Bilibili 广播协议 (`broadcastlv.chat.bilibili.com`) 连接直播间，通过 AUTH 包认证并接收旧版消息格式 (`DANMU_MSG`, `SEND_GIFT`, `INTERACT_WORD`)。Bilibili 直播开放平台已推出官方 API 体系，旧版广播协议已于 2022 年 6 月 15 日起逐步停止支持，用户标识 `uid` 已废弃。

### 目标

将项目 API 实现从旧版广播协议完全迁移至 Bilibili 开放平台官方 API 体系 (`live-open.biliapi.com`)，确保项目符合当前的官方技术规范、支持新的消息类型、使用正确的用户标识体系。

---

## 范围

### 在范围内

| 模块 | 说明 |
|------|------|
| 鉴权系统 | 实现 HMAC-SHA256 签名机制，替换旧 AUTH 包认证 |
| 连接管理 | 实现 `/v2/app/start` → `/v2/app/end` 生命周期 |
| 项目心跳 | 实现 `/v2/app/heartbeat` (每 20 秒) 和 `/v2/app/batchHeartbeat` |
| WebSocket 消息解析 | 解析新版 CMD 消息格式 (LIVE_OPEN_PLATFORM_* 系列) |
| 数据类型系统 | 更新所有消息类型以匹配官方 API 字段 (open_id, union_id 等) |
| 弹幕消息 | LIVE_OPEN_PLATFORM_DM, LIVE_OPEN_PLATFORM_DM_MIRROR |
| 礼物消息 | LIVE_OPEN_PLATFORM_SEND_GIFT (含 anchor_info, combo_info, blind_gift) |
| 付费留言 | LIVE_OPEN_PLATFORM_SUPER_CHAT, LIVE_OPEN_PLATFORM_SUPER_CHAT_DEL |
| 大航海 | LIVE_OPEN_PLATFORM_GUARD |
| 点赞消息 | LIVE_OPEN_PLATFORM_LIKE |
| 房间事件 | LIVE_OPEN_PLATFORM_LIVE_ROOM_ENTER, LIVE_OPEN_PLATFORM_LIVE_START, LIVE_OPEN_PLATFORM_LIVE_END |
| 消息结束通知 | LIVE_OPEN_PLATFORM_INTERACTION_END |
| 配置管理 | 添加 access_key_id, access_key_secret, app_id 等配置项 |

### 不在范围内

- 自定义互动面板 (`/v1/game/getCustomPanelSession`) — 项目当前不需要
- 站内版权曲库 API (`/xlive/open-platform/v2/music/*`) — 项目当前不需要
- 获取 union_id API (`/v2/union_id`) — 项目当前不需要
- H5 插件 API 签名验证 — 项目为独立客户端应用，非 H5 插件
- 互动礼物创建/管理接口 — 项目当前不需要
- UI 变更 — 仅涉及 API 层调整

---

## 模块设计

### 模块结构

```
src/renderer/modules/bilibili/
├── types.ts          # 类型定义 (重写)
├── auth.ts           # 新增: HMAC-SHA256 签名
├── client.ts         # 重写: 开放平台客户端
├── protocol.ts       # 重写: WebSocket 协议解析
└── api.ts            # 新增: HTTP API 调用 (/v2/app/start, /heartbeat, /end)
```

### 1. 鉴权模块 (`auth.ts`) — 新增

**职责**: 生成 HMAC-SHA256 签名请求头

**API**:
```typescript
interface AuthHeaders {
  'Accept': 'application/json';
  'Content-Type': 'application/json';
  'x-bili-content-md5': string;
  'x-bili-timestamp': string;
  'x-bili-signature-method': 'HMAC-SHA256';
  'x-bili-signature-nonce': string;
  'x-bili-accesskeyid': string;
  'x-bili-signature-version': '1.0';
  'Authorization': string;
}

function generateAuthHeaders(
  accessKeyId: string,
  accessKeySecret: string,
  body: string
): AuthHeaders
```

**签名算法**:
1. 对请求体计算 MD5，设为 `x-bili-content-md5`
2. 生成 UUID 设为 `x-bili-signature-nonce`
3. 生成 Unix 时间戳(秒)设为 `x-bili-timestamp`
4. 构建待签名字符串 (x-bili- 前缀 header 按字典排序):
   ```
   x-bili-accesskeyid:{value}
   x-bili-content-md5:{value}
   x-bili-signature-method:HMAC-SHA256
   x-bili-signature-nonce:{value}
   x-bili-signature-version:1.0
   x-bili-timestamp:{value}
   ```
5. 用 access_key_secret 对上述字符串做 HMAC-SHA256，结果为小写 hex 字符串

### 2. HTTP API 模块 (`api.ts`) — 新增

**职责**: 封装开放平台 HTTP API 调用

**接口**:

| 方法 | 路径 | 参数 | 返回 |
|------|------|------|------|
| `POST` | `/v2/app/start` | `{code, app_id}` | `{game_info: {game_id}, websocket_info: {auth_body, wss_link[]}, anchor_info: {room_id, uname, uface, uid, open_id, union_id}}` |
| `POST` | `/v2/app/end` | `{app_id, game_id}` | `{}` |
| `POST` | `/v2/app/heartbeat` | `{game_id}` | `{}` |
| `POST` | `/v2/app/batchHeartbeat` | `{game_ids: string[]}` | `{failed_game_ids: string[]}` |

**API 类设计**:
```typescript
class OpenPlatformAPI {
  constructor(accessKeyId: string, accessKeySecret: string)
  start(code: string, appId: number): Promise<StartResponse>
  end(appId: number, gameId: string): Promise<void>
  heartbeat(gameId: string): Promise<void>
  batchHeartbeat(gameIds: string[]): Promise<string[]>
}
```

### 3. 客户端模块 (`client.ts`) — 重写

**变更**:
- 移除 `getRoomInfo()` (通过 `/v2/app/start` 获取)
- 移除 `getAuthInfo()` (通过 `/v2/app/start` 返回 auth_body)
- 移除 `getWebSocketUrl()` (使用 `/v2/app/start` 返回的 wss_link)
- 移除 `authenticate()` (auth_body 由平台返回)
- 连接流程: `/v2/app/start` → 连接 wss_link → 发送 auth_body (OP_AUTH) → 等待 OP_AUTH_REPLY → 开始心跳
- 双心跳: WebSocket 层心跳 (20s) + HTTP 项目心跳 (20s `/v2/app/heartbeat`)

**新的连接流程**:
```
1. POST /v2/app/start {code, app_id}
2. 接收响应: {game_id, auth_body, wss_link[], anchor_info}
3. 连接 WebSocket (wss_link[0])
4. 发送 OP_AUTH 包 (body = auth_body JSON)
5. 等待 OP_AUTH_REPLY
6. 开始双心跳:
   - WebSocket 心跳: 每 20s 发送空 body OP_HEARTBEAT
   - HTTP 心跳: 每 20s POST /v2/app/heartbeat {game_id}
7. 接收消息 (OP_SEND_SMS_REPLY)
8. 断开时: POST /v2/app/end {app_id, game_id} → 关闭 WebSocket
```

### 4. WebSocket 协议模块 (`protocol.ts`) — 重写

**变更**:
- 保留二进制包格式解析 (Packet Length + Header Length + Version + Operation + Sequence ID + Body)
- 保留 zlib 压缩解压 (Version = 2)
- 移除旧 cmd 解析 (`DANMU_MSG`, `SEND_GIFT`, `INTERACT_WORD`, `ONLINE_COUNT`)
- 新增 cmd 解析 (所有 `LIVE_OPEN_PLATFORM_*` 系列)

**消息分发映射**:
```
LIVE_OPEN_PLATFORM_DM → DanmakuMessage
LIVE_OPEN_PLATFORM_DM_MIRROR → MirrorDanmakuMessage
LIVE_OPEN_PLATFORM_SEND_GIFT → GiftMessage
LIVE_OPEN_PLATFORM_SUPER_CHAT → SuperChatMessage
LIVE_OPEN_PLATFORM_SUPER_CHAT_DEL → SuperChatDelMessage
LIVE_OPEN_PLATFORM_GUARD → GuardMessage
LIVE_OPEN_PLATFORM_LIKE → LikeMessage
LIVE_OPEN_PLATFORM_LIVE_ROOM_ENTER → UserEnterMessage
LIVE_OPEN_PLATFORM_LIVE_START → LiveStartMessage
LIVE_OPEN_PLATFORM_LIVE_END → LiveEndMessage
LIVE_OPEN_PLATFORM_INTERACTION_END → InteractionEndMessage
```

### 5. 类型定义 (`types.ts`) — 重写

**变更**:

保留:
- `OpCode` (AUTH=7, HEARTBEAT=2, HEARTBEAT_REPLY=3, MESSAGE=5, AUTH_REPLY=8)
- `ProtocolVersion` (NORMAL=0, HEARTBEAT=1, GZIP=2, BROTLI=3)
- `ProtocolHeader`
- `ClientConfig`, `ClientEvent`
- `ProtocolError`, `ClientError`

新增:
- `AuthHeaders`, `SignParams`
- `StartRequest`, `StartResponse`, `GameInfo`, `WebSocketInfo`, `AnchorInfo`
- `EndRequest`, `HeartbeatRequest`, `BatchHeartbeatRequest`, `BatchHeartbeatResponse`
- 所有新的消息类型接口 (见下方合约)

移除:
- `AuthParams` (不再手动构建 AUTH 包)
- `DanmakuInfo`, `MedalInfo` (替换为新的扁平结构)

---

## API 和数据合约

### 配置新增

```typescript
interface AppConfig {
  accessKeyId: string;     // 开放平台 access_key_id
  accessKeySecret: string;  // 开放平台 access_key_secret
  appId: number;           // 应用 ID (int64)
}
```

### 生命周期响应合约

```typescript
// POST /v2/app/start 响应
interface StartResponse {
  code: number;
  message: string;
  request_id: string;
  data: {
    game_info: { game_id: string };
    websocket_info: {
      auth_body: string;
      wss_link: string[];
    };
    anchor_info: {
      room_id: number;
      uname: string;
      uface: string;
      uid: number;
      open_id: string;
      union_id: string;
    };
  };
}

// POST /v2/app/heartbeat 响应
interface HeartbeatResponse {
  code: number;
  message: string;
  data: {};
}

// POST /v2/app/batchHeartbeat 响应
interface BatchHeartbeatResponse {
  code: number;
  message: string;
  data: {
    failed_game_ids: string[];
  };
}
```

### 弹幕消息合约

```typescript
// LIVE_OPEN_PLATFORM_DM
interface OpenPlatformDanmakuMessage {
  room_id: number;
  open_id: string;
  union_id: string;        // 默认空
  uname: string;
  uface: string;
  msg: string;
  msg_id: string;
  fans_medal_level: number;
  fans_medal_name: string;
  fans_medal_wearing_status: boolean;
  guard_level: number;     // 1:总督 2:提督 3:舰长
  timestamp: number;
  emoji_img_url: string;
  dm_type: number;         // 0:普通 1:表情包
  glory_level: number;
  reply_open_id: string;
  reply_uname: string;
  is_admin: number;        // 0或1
  uid: number;             // 已废弃，固定为0
}

// LIVE_OPEN_PLATFORM_DM_MIRROR
interface MirrorDanmakuMessage {
  room_id: number;
  msg: string;
  msg_id: string;
  timestamp: number;
  emoji_img_url: string;
  dm_type: number;
}
```

### 礼物消息合约

```typescript
// LIVE_OPEN_PLATFORM_SEND_GIFT
interface OpenPlatformGiftMessage {
  room_id: number;
  uid: number;             // 已废弃，固定为0
  open_id: string;
  union_id: string;
  uname: string;
  uface: string;
  gift_id: number;
  gift_name: string;
  gift_num: number;
  price: number;           // 1000 = 1元
  r_price: number;
  paid: boolean;
  fans_medal_level: number;
  fans_medal_name: string;
  fans_medal_wearing_status: boolean;
  guard_level: number;
  timestamp: number;
  msg_id: string;
  gift_icon: string;
  combo_gift: boolean;
  combo_info: ComboInfo;
  blind_gift: BlindGiftInfo;
  anchor_info: {
    uid: number;
    open_id: string;
    union_id: string;
    uname: string;
    uface: string;
  };
}

interface ComboInfo {
  combo_base_num: number;
  combo_count: number;
  combo_id: string;
  combo_timeout: number;
}

interface BlindGiftInfo {
  blind_gift_id: number;
  status: boolean;
}
```

### 其他消息合约

```typescript
interface OpenPlatformSuperChatMessage {
  room_id: number;
  uid: number;
  open_id: string;
  union_id: string;
  uname: string;
  uface: string;
  message_id: number;
  message: string;
  msg_id: string;
  rmb: number;
  timestamp: number;
  start_time: number;
  end_time: number;
  guard_level: number;
  fans_medal_level: number;
  fans_medal_name: string;
  fans_medal_wearing_status: boolean;
}

interface OpenPlatformSuperChatDelMessage {
  room_id: number;
  message_ids: number[];
  msg_id: string;
}

interface OpenPlatformGuardMessage {
  user_info: {
    uid: number;
    open_id: string;
    union_id: string;
    uname: string;
    uface: string;
  };
  guard_level: number;
  guard_num: number;
  guard_unit: string;
  price: number;
  fans_medal_level: number;
  fans_medal_name: string;
  fans_medal_wearing_status: boolean;
  room_id: number;
  msg_id: string;
  timestamp: number;
}

interface OpenPlatformLikeMessage {
  uname: string;
  uid: number;
  open_id: string;
  union_id: string;
  uface: string;
  timestamp: number;
  room_id: number;
  like_text: string;
  like_count: number;
  fans_medal_wearing_status: boolean;
  fans_medal_name: string;
  fans_medal_level: number;
}

interface OpenPlatformUserEnterMessage {
  room_id: number;
  uface: string;
  uname: string;
  open_id: string;
  union_id: string;
  timestamp: number;
}

interface OpenPlatformLiveStartMessage {
  room_id: number;
  open_id: string;
  union_id: string;
  timestamp: number;
  area_name: string;
  title: string;
}

interface OpenPlatformLiveEndMessage {
  room_id: number;
  open_id: string;
  union_id: string;
  timestamp: number;
  area_name: string;
  title: string;
}

interface OpenPlatformInteractionEndMessage {
  game_id: string;
  timestamp: number;
}
```

### 内部消息分发合约

```typescript
type OpenPlatformCmd =
  | 'LIVE_OPEN_PLATFORM_DM'
  | 'LIVE_OPEN_PLATFORM_DM_MIRROR'
  | 'LIVE_OPEN_PLATFORM_SEND_GIFT'
  | 'LIVE_OPEN_PLATFORM_SUPER_CHAT'
  | 'LIVE_OPEN_PLATFORM_SUPER_CHAT_DEL'
  | 'LIVE_OPEN_PLATFORM_GUARD'
  | 'LIVE_OPEN_PLATFORM_LIKE'
  | 'LIVE_OPEN_PLATFORM_LIVE_ROOM_ENTER'
  | 'LIVE_OPEN_PLATFORM_LIVE_START'
  | 'LIVE_OPEN_PLATFORM_LIVE_END'
  | 'LIVE_OPEN_PLATFORM_INTERACTION_END';

interface ParsedOpenPlatformMessage {
  cmd: OpenPlatformCmd;
  type: 'danmaku' | 'mirrorDanmaku' | 'gift' | 'superChat' | 'superChatDel'
      | 'guard' | 'like' | 'userEnter' | 'liveStart' | 'liveEnd'
      | 'interactionEnd' | 'unknown';
  data: unknown;
}
```

---

## 边缘情况与风险

### 边缘情况

1. **多集群切换**: `/v2/app/start` 返回多个 `wss_link`，第一个连接失败后需切换到备用地址
2. **心跳过期**: 互动玩法超过 60s 无心跳自动关闭，需确保心跳稳定
3. **消息推送结束**: 收到 `LIVE_OPEN_PLATFORM_INTERACTION_END` 后不应再尝试心跳，应触发重连
4. **签名过期**: `x-bili-timestamp` 超过当前时间 10 分钟请求被丢弃，需使用 NTP 时间
5. **压缩消息**: Version=2 时 Body 是 zlib 压缩数据，解压后可能包含多个完整的 proto 包
6. **批量心跳限制**: 单次最多 200 个 game_id，重复 ID 会报错 7005
7. **end 调用必要性**: 游戏结束必须调用 `/v2/app/end`，否则影响礼物投放和下一场次启动

### 风险

| 风险 | 影响 | 缓解措施 |
|------|------|---------|
| access_key/secret 泄露 | 被冒用身份 | 在用户设置页脱敏显示，加密存储 |
| 签名时间不同步 | 请求被拒 | 使用系统时间，监控 4003 错误码 |
| WebSocket 连接不稳定 | 消息丢失 | 多集群切换 + 重连机制 |
| 旧版用户代码兼容 | 编译错误 | 分阶段废弃旧类型，保留映射层过渡 |

---

## 验收标准

### AC-1: 鉴权系统
- [ ] `auth.ts` 正确生成 HMAC-SHA256 签名
- [ ] 签名结果与官方 Python demo 输出一致 (同输入同输出)
- [ ] 请求头 x-bili-content-md5 正确

### AC-2: 生命周期 API
- [ ] `POST /v2/app/start` 正确返回 `game_id`, `wss_link`, `auth_body`
- [ ] `POST /v2/app/heartbeat` 每 20 秒成功调用
- [ ] `POST /v2/app/end` 在断开连接时成功调用
- [ ] 心跳过期 (<60s 互动玩法) 后自动关闭逻辑正确

### AC-3: WebSocket 消息解析
- [ ] 能正确解析 `LIVE_OPEN_PLATFORM_DM` 为弹幕消息
- [ ] 能正确解析 `LIVE_OPEN_PLATFORM_SEND_GIFT` 为礼物消息 (含 anchor_info, combo_info)
- [ ] 能正确解析 `LIVE_OPEN_PLATFORM_SUPER_CHAT` 为付费留言
- [ ] 能正确解析 `LIVE_OPEN_PLATFORM_GUARD` 为大航海上舰
- [ ] 能正确解析 `LIVE_OPEN_PLATFORM_LIKE` 为点赞
- [ ] 能正确解析 `LIVE_OPEN_PLATFORM_LIVE_START` / `LIVE_OPEN_PLATFORM_LIVE_END`

### AC-4: 类型系统
- [ ] 所有新消息类型使用 `open_id` 替代 `uid` 作为用户标识
- [ ] 所有字段与官方文档一致
- [ ] TypeScript 编译无错误

### AC-5: 向后兼容
- [ ] 上层 `DanmakuManager` 与 `DanmakuService` 无需修改或最小修改
- [ ] 现有 `parser.ts` 的 `ParsedDanmaku` 输出不变
- [ ] 礼物处理逻辑 (`handleGift`) 适配新 Gift 结构

---

## 依赖

- `crypto-js` 或 Node.js 原生 `crypto` 库 (HMAC-SHA256)
- `pako` (已有, zlib 解压)
- `uuid` (生成 nonce)

---

## 约束

- 所有 HTTP API 使用 `Content-Type: application/json`，method 为 POST
- 签名使用小写 hex 字符串
- `app_id` 为 int64 类型，JavaScript 中使用 number 即可 (安全整数范围内)
- WebSocket 协议保持大端序
- HTTP 响应 http status code 均为 200，需根据 body 中的 code 判断成功/失败