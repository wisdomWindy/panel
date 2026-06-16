// 录制模块类型定义

// 录制格式
export type RecordingFormat = 'mp4' | 'flv';

// 录制质量
export type RecordingQuality = 'original' | '720p' | '1080p';

// 录制状态
export type RecordingStatus = 'recording' | 'paused' | 'stopped' | 'completed' | 'failed';

// 高光触发原因
export type HighlightReason = 'danmaku_surge' | 'high_value_gift' | 'captain' | 'manual';

// 录制配置
export interface RecordingConfig {
  enabled: boolean;
  autoStart: boolean;
  format: RecordingFormat;
  quality: RecordingQuality;
  segmentSize: number;  // MB，0 表示不分段
  segmentDuration: number;  // 秒，0 表示不分段
  outputPath: string;
  maxStorageSize: number;  // GB，0 表示不限制
  autoCleanup: boolean;
  cleanupDays: number;
  highlightEnabled: boolean;
  highlightConfig: HighlightConfig;
}

// 高光配置
export interface HighlightConfig {
  danmakuThreshold: number;  // 弹幕数量阈值
  danmakuWindow: number;     // 弹幕统计窗口（秒）
  giftValueThreshold: number;  // 礼物价值阈值
  captainTrigger: boolean;   // 舰长触发
  clipDuration: number;      // 高光片段时长（秒）
  beforeSeconds: number;     // 高光前录制秒数
  afterSeconds: number;      // 高光后录制秒数
  cooldownSeconds: number;   // 冷却时间（秒）
}

// 录制选项
export interface RecordingOptions {
  roomId: string;
  roomName?: string;
  format: RecordingFormat;
  quality: RecordingQuality;
  segmentSize?: number;  // MB
  segmentDuration?: number;  // 秒
  outputPath: string;
  highlightEnabled: boolean;
  highlightConfig?: HighlightConfig;
}

// 录制会话
export interface RecordingSession {
  id: string;
  roomId: string;
  roomName: string;
  status: RecordingStatus;
  filePath: string;
  fileName: string;
  fileSize: number;
  duration: number;
  format: RecordingFormat;
  quality: RecordingQuality;
  startedAt: Date;
  endedAt?: Date;
  highlightCount: number;
}

// 录制结果
export interface RecordingResult {
  sessionId: string;
  filePath: string;
  fileSize: number;
  duration: number;
  highlightCount: number;
  highlights: Highlight[];
}

// 录制状态信息
export interface RecordingStatusInfo {
  sessionId: string;
  status: RecordingStatus;
  currentSize: number;
  currentDuration: number;
  highlightCount: number;
  lastHighlightAt?: Date;
}

// 高光时刻
export interface Highlight {
  id: string;
  recordingId: string;
  roomId: string;
  timestamp: number;  // 相对于录制开始的秒数
  reason: HighlightReason;
  description: string;
  clipPath?: string;
  clipDuration: number;
  createdAt: Date;
}

// 录制文件
export interface Recording {
  id: string;
  roomId: string;
  roomName: string;
  filePath: string;
  fileName: string;
  fileSize: number;
  duration: number;
  format: RecordingFormat;
  quality: RecordingQuality;
  status: RecordingStatus;
  isHighlight: boolean;
  highlightReason?: HighlightReason;
  startedAt: Date;
  endedAt?: Date;
  createdAt: Date;
}

// 查询选项
export interface QueryOptions {
  roomId?: string;
  status?: RecordingStatus;
  startDate?: Date;
  endDate?: Date;
  limit?: number;
  offset?: number;
  sortBy?: string;
  sortOrder?: 'asc' | 'desc';
}

// 默认录制配置
export const DEFAULT_RECORDING_CONFIG: RecordingConfig = {
  enabled: false,
  autoStart: false,
  format: 'mp4',
  quality: 'original',
  segmentSize: 0,
  segmentDuration: 0,
  outputPath: '',
  maxStorageSize: 0,
  autoCleanup: false,
  cleanupDays: 30,
  highlightEnabled: true,
  highlightConfig: {
    danmakuThreshold: 30,
    danmakuWindow: 10,
    giftValueThreshold: 100,
    captainTrigger: true,
    clipDuration: 30,
    beforeSeconds: 15,
    afterSeconds: 15,
    cooldownSeconds: 10
  }
};

// 默认快捷键
export const DEFAULT_HOTKEYS = {
  toggleRecording: 'CommandOrControl+Shift+R',
  markHighlight: 'F9'
};
