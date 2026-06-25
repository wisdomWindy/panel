/**
 * 礼物音效类型定义
 */

// 礼物音效配置
export interface GiftSoundConfig {
  id: string;
  giftId: string;
  giftName: string;
  soundFile: string;
  soundName: string;
  volume: number;  // 0-1
  duration: number;  // 毫秒，0=完整播放
  playMode: 'once' | 'loop';
  cooldown: number;  // 毫秒
  priority: number;
  enabled: boolean;
  createdAt: Date;
  updatedAt: Date;
}

// 音效播放请求
export interface PlaySoundRequest {
  giftId: string;
  giftName: string;
  username: string;
  count: number;
}

// 音效播放结果
export interface PlaySoundResult {
  success: boolean;
  soundFile?: string;
  error?: string;
}

// 音效播放日志
export interface SoundLog {
  id: string;
  giftId: string;
  giftName: string;
  username: string;
  soundFile: string;
  playedAt: Date;
}

// 音效播放状态
export interface SoundPlaybackState {
  isPlaying: boolean;
  currentSound: string | null;
  volume: number;
  progress: number;
  duration: number;
}

// 音效队列项
export interface SoundQueueItem {
  id: string;
  giftSound: GiftSoundConfig;
  request: PlaySoundRequest;
  priority: number;
  addedAt: Date;
}

// 音效导入导出格式
export interface GiftSoundExportData {
  version: string;
  exportedAt: Date;
  giftSounds: GiftSoundConfig[];
}

// 音效播放器配置
export interface AudioPlayerConfig {
  masterVolume: number;
  fadeInDuration: number;
  fadeOutDuration: number;
  maxConcurrentSounds: number;
}

// 礼物信息（用于配置页面）
export interface GiftInfo {
  giftId: string;
  giftName: string;
  giftPrice: number;
  giftImg: string;
  giftDesc: string;
}

// 音效文件信息
export interface SoundFileInfo {
  name: string;
  path: string;
  size: number;
  duration: number;
  format: string;
}

// 音效播放事件
export interface SoundPlayEvent {
  type: 'start' | 'end' | 'error' | 'pause' | 'resume';
  soundFile: string;
  timestamp: Date;
  error?: string;
}

// 音效配置验证结果
export interface ConfigValidationResult {
  valid: boolean;
  errors: string[];
  warnings: string[];
}

// 批量操作类型
export type BatchOperationType = 'enable' | 'disable' | 'delete' | 'export';

// 批量操作请求
export interface BatchOperationRequest {
  type: BatchOperationType;
  ids: string[];
}

// 批量操作结果
export interface BatchOperationResult {
  success: boolean;
  processed: number;
  failed: number;
  errors: string[];
}
