# PRD 快照: Bilibili 开放平台 API 对齐

## 来源

description-doc/ 目录中的 23 个官方文档文件

## 核心 API 变更汇总

### 1. 鉴权体系变更 (文档 #3)

**当前**: 项目使用 `broadcastlv.chat.bilibili.com` 广播协议，基于 roomId + token 的 AUTH 包认证

**目标**: 迁移到 `live-open.biliapi.com` 开放平台 API

- 所有请求使用 HMAC-SHA256 签名
- 公共请求头: `x-bili-accesskeyid`, `x-bili-content-md5`, `x-bili-timestamp`, `x-bili-signature-method`, `x-bili-signature-nonce`, `x-bili-signature-version`, `Authorization`
- 签名构建: 抽取带 `x-bili-` 前缀的自定义 header → 按字典排序 → 用 access_secret 做 HMAC-SHA256

### 2. 项目生命周期 API (文档 #4)

| 操作 | 当前实现 | 官方 API |
|------|---------|---------|
| 开启 | 连接广播 WSS + AUTH | `POST /v2/app/start` (code, app_id) |
| 心跳 | WSS 心跳 (30s) | `POST /v2/app/heartbeat` (game_id, 每20s) |
| 批量心跳 | 无 | `POST /v2/app/batchHeartbeat` (game_ids, 最多200) |
| 关闭 | WSS close | `POST /v2/app/end` (app_id, game_id) |

**关键**: `/v2/app/start` 返回 `wss_link[]` (长连地址) + `auth_body` (长连鉴权) + `game_id` (场次ID)

### 3. 长连消息协议变更 (文档 #6, #7)

| 消息类型 | 旧 cmd | 新 cmd |
|---------|--------|--------|
| 弹幕 | `DANMU_MSG` | `LIVE_OPEN_PLATFORM_DM` |
| 跨房弹幕 | (无) | `LIVE_OPEN_PLATFORM_DM_MIRROR` |
| 礼物 | `SEND_GIFT` | `LIVE_OPEN_PLATFORM_SEND_GIFT` |
| 付费留言 | (无) | `LIVE_OPEN_PLATFORM_SUPER_CHAT` |
| 付费留言下线 | (无) | `LIVE_OPEN_PLATFORM_SUPER_CHAT_DEL` |
| 大航海 | (无) | `LIVE_OPEN_PLATFORM_GUARD` |
| 点赞 | (无) | `LIVE_OPEN_PLATFORM_LIKE` |
| 进入房间 | `INTERACT_WORD` | `LIVE_OPEN_PLATFORM_LIVE_ROOM_ENTER` |
| 开始直播 | (无) | `LIVE_OPEN_PLATFORM_LIVE_START` |
| 结束直播 | (无) | `LIVE_OPEN_PLATFORM_LIVE_END` |
| 消息推送结束 | (无) | `LIVE_OPEN_PLATFORM_INTERACTION_END` |

### 4. 用户标识变更

- `uid` → 已废弃，固定为 0
- 新增 `open_id`: 用户唯一标识 (2024-03-11 后上线)
- 新增 `union_id`: 同开发者下的唯一标识，需要单独申请
- 新增 `fans_medal_level`, `fans_medal_name`, `fans_medal_wearing_status`, `guard_level`

### 5. 新消息结构详情

#### 弹幕消息 (LIVE_OPEN_PLATFORM_DM)
```json
{
  "room_id": 1,
  "open_id": "39b8fedb-...",
  "union_id": "U_...",
  "uname": "",
  "uface": "",
  "msg": "",
  "msg_id": "",
  "fans_medal_level": 0,
  "fans_medal_name": "",
  "fans_medal_wearing_status": true,
  "guard_level": 0,
  "timestamp": 0,
  "emoji_img_url": "",
  "dm_type": 0,
  "glory_level": 39,
  "reply_open_id": "",
  "reply_uname": "",
  "is_admin": 1
}
```

#### 礼物消息 (LIVE_OPEN_PLATFORM_SEND_GIFT)
```json
{
  "room_id": 1,
  "open_id": "",
  "union_id": "",
  "uname": "",
  "uface": "",
  "gift_id": 0,
  "gift_name": "",
  "gift_num": 0,
  "price": 0,
  "r_price": 0,
  "paid": false,
  "fans_medal_level": 0,
  "fans_medal_name": "",
  "fans_medal_wearing_status": true,
  "guard_level": 0,
  "timestamp": 0,
  "msg_id": "",
  "gift_icon": "",
  "combo_gift": true,
  "combo_info": {},
  "blind_gift": {},
  "anchor_info": {}
}
```

### 6. 其他 API 接口 (文档 #10, #12, #13, #14, #15)

| 接口 | 路径 | 用途 |
|------|------|------|
| 查询用户信息 | `/v1/game/getCustomPanelSession` | 自定义面板用户信息 |
| 搜索媒体 | `/xlive/open-platform/v2/music/search` | 站内音乐搜索 |
| 获取媒体元数据 | `/xlive/open-platform/v2/music/meta/list` | 音乐详细信息 |
| 获取媒体资源 | `/xlive/open-platform/v2/music/list` | 音乐资源详情 |
| 获取 union_id | `/v2/union_id` | 获取用户 union_id |