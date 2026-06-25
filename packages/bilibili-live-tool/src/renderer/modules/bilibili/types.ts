/**
 * Bilibili 开放平台 API 类型定义
 * 参考: https://open-live.bilibili.com/document/
 */

// ============ 协议常量 ============

/** 操作码枚举 */
export enum OpCode {
  HEARTBEAT = 2,        // 客户端发送的心跳包
  HEARTBEAT_REPLY = 3,  // 服务器心跳回复
  SEND_SMS_REPLY = 5,   // 服务器推送的消息包
  AUTH = 7,             // 客户端发送的鉴权包
  AUTH_REPLY = 8        // 服务器鉴权回复
}

/** 协议版本枚举 */
export enum ProtocolVersion {
  NORMAL = 0,   // 普通 JSON
  HEARTBEAT = 1, // 心跳（无消息体）
  GZIP = 2,     // Gzip 压缩
  BROTLI = 3    // Brotli 压缩
}

// ============ 协议结构 ============

/** 16 字节消息头结构 */
export interface ProtocolHeader {
  length: number;        // 消息总长度（4 字节）
  headerLength: number;  // 消息头长度（2 字节）
  version: ProtocolVersion; // 协议版本（2 字节）
  opCode: OpCode;        // 操作码（4 字节）
  sequenceId: number;    // 序列号（4 字节）
}

// ============ 鉴权 ============

/** HMAC-SHA256 签名请求头 */
export interface AuthHeaders {
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

// ============ 客户端配置 ============

/** 客户端配置 */
export interface ClientConfig {
  accessKeyId: string;
  accessKeySecret: string;
  appId: number;              // 应用 ID (int64)
  code: string;               // 主播身份码
  heartbeatInterval?: number; // 心跳间隔（秒），默认 20
  reconnectInterval?: number; // 重连间隔（毫秒）
  maxReconnectAttempts?: number; // 最大重连次数
}

// ============ 生命周期 API 类型 ============

/** POST /v2/app/start 请求 */
export interface StartRequest {
  code: string;
  app_id: number;
}

/** POST /v2/app/start 响应 */
export interface StartResponse {
  code: number;
  message: string;
  request_id: string;
  data: {
    game_info: {
      game_id: string;
    };
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

/** POST /v2/app/end 请求 */
export interface EndRequest {
  app_id: number;
  game_id: string;
}

/** POST /v2/app/heartbeat 请求 */
export interface HeartbeatRequest {
  game_id: string;
}

/** POST /v2/app/batchHeartbeat 请求 */
export interface BatchHeartbeatRequest {
  game_ids: string[];
}

/** POST /v2/app/batchHeartbeat 响应 */
export interface BatchHeartbeatResponse {
  code: number;
  message: string;
  data: {
    failed_game_ids: string[];
  };
}

// ============ 直播间信息 ============

/** 直播间信息 */
export interface RoomInfo {
  roomId: number;
  roomName: string;
  ownerId: number;
  ownerOpenId: string;
  ownerName: string;
  onlineCount: number;
  liveStatus: number;
  openId: string;
  unionId: string;
}

// ============ 长连消息 - 弹幕 ============

/** 弹幕信息 (LIVE_OPEN_PLATFORM_DM) */
export interface OpenPlatformDanmakuMessage {
  cmd: 'LIVE_OPEN_PLATFORM_DM';
  data: {
    room_id: number;
    uid: number;            // 已废弃，固定为0
    open_id: string;
    union_id: string;       // 默认空
    uname: string;
    uface: string;
    msg: string;
    msg_id: string;
    fans_medal_level: number;
    fans_medal_name: string;
    fans_medal_wearing_status: boolean;
    guard_level: number;    // 1:总督 2:提督 3:舰长
    timestamp: number;
    emoji_img_url: string;
    dm_type: number;        // 0:普通 1:表情包
    glory_level: number;
    reply_open_id: string;
    reply_uname: string;
    is_admin: number;       // 0或1
  };
}

/** 跨房弹幕信息 (LIVE_OPEN_PLATFORM_DM_MIRROR) */
export interface MirrorDanmakuMessage {
  cmd: 'LIVE_OPEN_PLATFORM_DM_MIRROR';
  data: {
    room_id: number;
    msg: string;
    msg_id: string;
    timestamp: number;
    emoji_img_url: string;
    dm_type: number;
  };
}

// ============ 长连消息 - 礼物 ============

/** 礼物信息 (LIVE_OPEN_PLATFORM_SEND_GIFT) */
export interface OpenPlatformGiftMessage {
  cmd: 'LIVE_OPEN_PLATFORM_SEND_GIFT';
  data: {
    room_id: number;
    uid: number;
    open_id: string;
    union_id: string;
    uname: string;
    uface: string;
    gift_id: number;
    gift_name: string;
    gift_num: number;
    price: number;          // 1000 = 1元 = 10电池
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
    anchor_info: AnchorInfo;
  };
}

export interface ComboInfo {
  combo_base_num: number;
  combo_count: number;
  combo_id: string;
  combo_timeout: number;
}

export interface BlindGiftInfo {
  blind_gift_id: number;
  status: boolean;
}

export interface AnchorInfo {
  uid: number;
  open_id: string;
  union_id: string;
  uname: string;
  uface: string;
}

// ============ 长连消息 - 付费留言 ============

/** 付费留言 (LIVE_OPEN_PLATFORM_SUPER_CHAT) */
export interface SuperChatMessage {
  cmd: 'LIVE_OPEN_PLATFORM_SUPER_CHAT';
  data: {
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
  };
}

/** 付费留言下线 (LIVE_OPEN_PLATFORM_SUPER_CHAT_DEL) */
export interface SuperChatDelMessage {
  cmd: 'LIVE_OPEN_PLATFORM_SUPER_CHAT_DEL';
  data: {
    room_id: number;
    message_ids: number[];
    msg_id: string;
  };
}

// ============ 长连消息 - 大航海 ============

/** 大航海 (LIVE_OPEN_PLATFORM_GUARD) */
export interface GuardMessage {
  cmd: 'LIVE_OPEN_PLATFORM_GUARD';
  data: {
    user_info: {
      uid: number;
      open_id: string;
      union_id: string;
      uname: string;
      uface: string;
    };
    guard_level: number;    // 1总督 2提督 3舰长
    guard_num: number;
    guard_unit: string;     // "月"
    price: number;
    fans_medal_level: number;
    fans_medal_name: string;
    fans_medal_wearing_status: boolean;
    room_id: number;
    msg_id: string;
    timestamp: number;
  };
}

// ============ 长连消息 - 点赞 ============

/** 点赞信息 (LIVE_OPEN_PLATFORM_LIKE) */
export interface LikeMessage {
  cmd: 'LIVE_OPEN_PLATFORM_LIKE';
  data: {
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
    msg_id: string;
  };
}

// ============ 长连消息 - 房间事件 ============

/** 进入房间 (LIVE_OPEN_PLATFORM_LIVE_ROOM_ENTER) */
export interface UserEnterMessage {
  cmd: 'LIVE_OPEN_PLATFORM_LIVE_ROOM_ENTER';
  data: {
    room_id: number;
    uface: string;
    uname: string;
    open_id: string;
    union_id: string;
    timestamp: number;
  };
}

/** 开始直播 (LIVE_OPEN_PLATFORM_LIVE_START) */
export interface LiveStartMessage {
  cmd: 'LIVE_OPEN_PLATFORM_LIVE_START';
  data: {
    room_id: number;
    open_id: string;
    union_id: string;
    timestamp: number;
    area_name: string;
    title: string;
  };
}

/** 结束直播 (LIVE_OPEN_PLATFORM_LIVE_END) */
export interface LiveEndMessage {
  cmd: 'LIVE_OPEN_PLATFORM_LIVE_END';
  data: {
    room_id: number;
    open_id: string;
    union_id: string;
    timestamp: number;
    area_name: string;
    title: string;
  };
}

// ============ 长连消息 - 互动结束 ============

/** 消息推送结束 (LIVE_OPEN_PLATFORM_INTERACTION_END) */
export interface InteractionEndMessage {
  cmd: 'LIVE_OPEN_PLATFORM_INTERACTION_END';
  data: {
    game_id: string;
    timestamp: number;
  };
}

// ============ 通用消息类型 ============

/** 原始 WebSocket 消息 */
export interface RawWsMessage {
  cmd: string;
  data: Record<string, unknown>;
  [key: string]: unknown;
}

/** 所有消息联合类型 */
export type OpenPlatformMessage =
  | OpenPlatformDanmakuMessage
  | MirrorDanmakuMessage
  | OpenPlatformGiftMessage
  | SuperChatMessage
  | SuperChatDelMessage
  | GuardMessage
  | LikeMessage
  | UserEnterMessage
  | LiveStartMessage
  | LiveEndMessage
  | InteractionEndMessage;

/** 解析后的消息 */
export interface ParsedMessage {
  type: 'danmaku' | 'mirrorDanmaku' | 'gift' | 'superChat' | 'superChatDel'
      | 'guard' | 'like' | 'userEnter' | 'liveStart' | 'liveEnd'
      | 'interactionEnd' | 'unknown';
  data: unknown;
  raw: RawWsMessage;
}

// ============ 客户端事件 ============

/** 客户端事件枚举 */
export enum ClientEvent {
  CONNECTED = 'connected',
  DISCONNECTED = 'disconnected',
  ERROR = 'error',
  MESSAGE = 'message',
  // 核心消息事件
  DANMAKU = 'danmaku',
  GIFT = 'gift',
  GUARD = 'guard',
  LIKE = 'like',
  USER_ENTER = 'userEnter',
  LIVE_START = 'liveStart',
  LIVE_END = 'liveEnd',
  SUPER_CHAT = 'superChat',
  INTERACTION_END = 'interactionEnd',
  // 心跳
  HEARTBEAT = 'heartbeat'
}

// ============ 错误类型 ============

/** 协议错误 */
export class ProtocolError extends Error {
  constructor(
    message: string,
    public code?: number
  ) {
    super(message);
    this.name = 'ProtocolError';
  }
}

/** 客户端错误 */
export class ClientError extends Error {
  constructor(
    message: string,
    public code?: number
  ) {
    super(message);
    this.name = 'ClientError';
  }
}

// ============ 导出 BilibiliClient 类 ============

