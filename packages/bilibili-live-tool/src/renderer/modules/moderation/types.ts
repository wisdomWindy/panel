/**
 * 自动禁言与刷屏防护模块类型定义
 */

// 禁言原因类型
export type MuteReason = 'prohibited_word' | 'flood' | 'spam' | 'manual';

// 禁言配置
export interface ModerationConfig {
  enabled: boolean;
  autoMuteEnabled: boolean;  // 自动禁言
  floodProtectionEnabled: boolean;  // 刷屏防护
  prohibitedWords: string[];  // 违禁词列表
  floodThreshold: number;  // 刷屏阈值（条/秒）
  floodTimeWindow: number;  // 刷屏检测时间窗口（秒）
  muteDuration: number;  // 禁言时长（分钟），0=永久
  warningBeforeMute: boolean;  // 禁言前是否警告
  warningMessage: string;  // 警告消息模板
  logMuteActions: boolean;  // 是否记录禁言日志
}

// 禁言记录
export interface MuteRecord {
  id: string;
  userId: string;
  username: string;
  reason: MuteReason;
  content: string;  // 触发禁言的内容
  duration: number;  // 禁言时长（分钟）
  mutedAt: Date;
  unmutedAt?: Date;
  roomId: string;
}

// 用户消息记录（用于刷屏检测）
export interface UserMessageRecord {
  userId: string;
  username: string;
  content: string;
  timestamp: number;
}

// 刷屏检测结果
export interface FloodDetectionResult {
  isFlood: boolean;
  messageCount: number;
  timeWindow: number;
  duplicateCount: number;
}

// 禁言状态
export type ModerationStatus = 'idle' | 'warning' | 'muting';

// 默认禁言配置
export const DEFAULT_MODERATION_CONFIG: ModerationConfig = {
  enabled: false,
  autoMuteEnabled: true,
  floodProtectionEnabled: true,
  prohibitedWords: [
    '广告', '代刷', '刷粉', '加微信', '加QQ',
    '赚钱', '兼职', '日赚', '月入', '免费领'
  ],
  floodThreshold: 5,  // 5条/秒
  floodTimeWindow: 10,  // 10秒
  muteDuration: 10,  // 10分钟
  warningBeforeMute: true,
  warningMessage: '请注意言行，您已被警告',
  logMuteActions: true
};

// 预设违禁词分类
export const PROHIBITED_WORD_CATEGORIES = {
  advertising: ['广告', '代刷', '刷粉', '加微信', '加QQ', '赚钱', '兼职', '日赚', '月入', '免费领'],
  spam: ['刷屏', '灌水', '复制粘贴', '无意义'],
  provocation: ['引战', '对骂', '喷子', '杠精'],
  illegal: ['赌博', '毒品', '色情', '暴力']
};
