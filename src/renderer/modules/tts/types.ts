/**
 * TTS 模块类型定义
 */

// TTS 引擎类型
export type TTSEngine = 'local' | 'edge-tts' | 'google' | 'azure' | 'baidu';

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
  edgeTTSVoice: string;  // edge-tts 专用语音名称
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
  engine: 'edge-tts',  // 默认使用 edge-tts，质量更好
  voice: 'default',
  speed: 1.0,
  pitch: 1.0,
  volume: 0.8,
  minInterval: 2000,
  formatTemplate: '{username}说：{content}',
  filterKeywords: [],
  broadcastGifts: true,
  giftValueThreshold: 0,
  broadcastCaptain: true,
  edgeTTSVoice: 'zh-CN-XiaoxiaoNeural'  // 微软晓晓，高质量中文语音
};

// 播报队列最大长度
export const MAX_QUEUE_SIZE = 100;

// 默认播报格式
export const DEFAULT_FORMATS = {
  danmaku: '{username}说：{content}',
  gift: '感谢{username}送出的{giftName}',
  captain: '欢迎{username}开通{captainLevel}'
};

// Edge TTS 中文语音列表
export const EDGE_TTS_VOICES = [
  { name: 'zh-CN-XiaoxiaoNeural', label: '晓晓（女，温柔）' },
  { name: 'zh-CN-YunxiNeural', label: '云希（男，阳光）' },
  { name: 'zh-CN-YunjianNeural', label: '云健（男，沉稳）' },
  { name: 'zh-CN-XiaoyiNeural', label: '晓伊（女，活泼）' },
  { name: 'zh-CN-YunyangNeural', label: '云扬（男，专业）' },
  { name: 'zh-CN-XiaochenNeural', label: '晓辰（女，知性）' },
  { name: 'zh-CN-XiaohanNeural', label: '晓涵（女，温暖）' },
  { name: 'zh-CN-XiaomengNeural', label: '晓梦（女，甜美）' },
  { name: 'zh-CN-XiaomoNeural', label: '晓墨（女，文艺）' },
  { name: 'zh-CN-XiaoruiNeural', label: '晓睿（女，成熟）' },
  { name: 'zh-CN-XiaoshuangNeural', label: '晓双（女，童声）' },
  { name: 'zh-CN-XiaoxuanNeural', label: '晓萱（女，活力）' },
  { name: 'zh-CN-XiaoyanNeural', label: '晓颜（女，优雅）' },
  { name: 'zh-CN-XiaozhenNeural', label: '晓甄（女，正式）' },
  { name: 'zh-CN-YunfengNeural', label: '云枫（男，激情）' },
  { name: 'zh-CN-YunhaoNeural', label: '云皓（男，大气）' },
  { name: 'zh-CN-YunxiaNeural', label: '云夏（男，少年）' },
  { name: 'zh-CN-YunzeNeural', label: '云泽（男，成熟）' }
];
