// 弹幕类型定义
export interface Danmaku {
  id: string;
  userId: string;
  username: string;
  content: string;
  timestamp: number;
  userType: 'normal' | 'captain' | 'admin' | 'owner';
  type?: 'received' | 'sent';
  success?: boolean;
  message?: string;
  badge?: {
    level: number;
    name: string;
  };
  medal?: {
    level: number;
    name: string;
    color: string;
  };
}

// 礼物类型定义
export interface Gift {
  id: string;
  giftId: string;
  giftName: string;
  userId: string;
  username: string;
  count: number;
  price: number;
  timestamp: number;
  isCombo: boolean;
  comboId?: string;
}

// 弹幕池项
export interface DanmakuPoolItem {
  id: number;
  content: string;
  type: 'timed' | 'random' | 'trigger';
  triggerWord?: string;
  enabled: boolean;
  priority: number;
  createdAt: Date;
  updatedAt: Date;
  interval?: number; // 添加 interval 属性
}

// 连接状态
export type ConnectionStatus = 'disconnected' | 'connecting' | 'connected' | 'error';

// 直播间信息
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

// 弹幕消息类型
export interface DanmakuMessage {
  cmd: string;
  data: unknown;
  [key: string]: unknown;
}

// 船长信息
export interface CaptainInfo {
  userId: string;
  username: string;
  level: number;
  title: string;
}

// 礼物连击信息
export interface GiftComboInfo {
  comboId: string;
  giftName: string;
  count: number;
  userId: string;
  username: string;
}

// 日志条目
export interface LogEntry {
  id: string;
  timestamp: Date;
  level: 'info' | 'warn' | 'error';
  message: string;
  data?: unknown;
}

// 黑名单项
export interface BlacklistItem {
  userId: string;
  username: string;
  reason: string;
  bannedAt: number;
  bannedUntil: number; // 0 = permanent
}
