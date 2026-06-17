/**
 * TTS 模块类型定义
 */

// TTS 引擎类型
export type TTSEngine = 'local' | 'google' | 'azure' | 'baidu';

// TTS 配置
export interface TTSConfig {
  enabled: boolean;
  engine: TTSEngine;
  voice: string;
  speed: number;  // 0.5-2.0
  pitch: number;  // 0.5-2.0
  volume: number;  // 0-1
  minInterval: number;  // 毫秒
  formatTemplate: string;
  filterKeywords: string[];
  broadcastGifts: boolean;
  giftValueThreshold: number;
  broadcastCaptain: boolean;
}

// 播报消息类型
export type BroadcastMessageType = 'danmaku' | 'gift' | 'captain';

// 播报消息
export interface BroadcastMessage {
  id: string;
  type: BroadcastMessageType;
  username: string;
  content: string;
  timestamp: number;
  giftName?: string;
  giftCount?: number;
  giftValue?: number;
  captainLevel?: number;
}

// 播报日志
export interface TTSLog {
  id: string;
  messageType: BroadcastMessageType;
  username: string;
  content: string;
  broadcastAt: Date;
}

// TTS 状态
export type TTSStatus = 'idle' | 'speaking' | 'paused';

// TTS 引擎信息
export interface TTSEngineInfo {
  name: string;
  lang: string;
  localService: boolean;
  voiceURI: string;
}

// 默认 TTS 配置
export const DEFAULT_TTS_CONFIG: TTSConfig = {
  enabled: false,
  engine: 'local',
  voice: 'default',
  speed: 1.0,
  pitch: 1.0,
  volume: 0.8,
  minInterval: 2000,
  formatTemplate: '{username}说：{content}',
  filterKeywords: [],
  broadcastGifts: true,
  giftValueThreshold: 0,
  broadcastCaptain: true
};

// 播报队列最大长度
export const MAX_QUEUE_SIZE = 100;

// 默认播报格式
export const DEFAULT_FORMATS = {
  danmaku: '{username}说：{content}',
  gift: '感谢{username}送出的{giftName}',
  captain: '欢迎{username}开通{captainLevel}'
};
