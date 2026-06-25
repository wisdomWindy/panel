/**
 * 特效系统类型定义
 */

/**
 * 特效类型枚举
 */
export enum EffectType {
  CAPTAIN_FIRST = 'captain_first', // 首次上舰
  CAPTAIN_RENEW = 'captain_renew', // 续费上舰
  GIFT_COMBO = 'gift_combo' // 礼物连击
}

/**
 * 动画类型枚举
 */
export enum AnimationType {
  FADE = 'fade', // 淡入淡出
  SLIDE = 'slide', // 滑动
  SCALE = 'scale', // 缩放
  ROTATE = 'rotate', // 旋转
  PARTICLE = 'particle' // 粒子效果
}

/**
 * 特效配置接口
 */
export interface EffectConfig {
  id: string;
  type: EffectType;
  templateName: string;
  textTemplate: string; // 如 "感谢 {username} 开通 {level}！"
  animationType: AnimationType;
  primaryColor: string; // 十六进制颜色
  secondaryColor: string;
  duration: number; // 毫秒
  soundFile?: string; // 音效文件路径
  enabled: boolean;
  renderMode: 'canvas' | 'webgl'; // 渲染模式: 2D Canvas / 3D WebGL
  createdAt: Date;
  updatedAt: Date;
}

/**
 * 舰长信息接口（特效系统专用）
 * 注意：此类型与 danmaku/types.ts 中的 CaptainInfo 不同，请根据具体使用场景选择
 */
export interface CaptainInfo {
  userId: string;
  username: string;
  level: number; // 1: 督察, 2: 提督, 3: 总督
  giftName: string;
  giftId: string;
  count: number;
  timestamp: number;
  isFirstTime: boolean; // 是否首次上舰
  isRenewal: boolean; // 是否续费
}

/**
 * 礼物连击信息接口
 */
export interface GiftComboInfo {
  userId: string;
  username: string;
  giftId: string;
  giftName: string;
  count: number;
  comboId: string;
  timestamp: number;
}

/**
 * 特效触发数据接口
 */
export interface EffectTriggerData {
  type: EffectType;
  captainInfo?: CaptainInfo;
  comboInfo?: GiftComboInfo;
  timestamp: number;
}

/**
 * 特效渲染参数接口
 */
export interface RenderParams {
  text: string;
  x: number;
  y: number;
  fontSize: number;
  primaryColor: string;
  secondaryColor: string;
  animationType: AnimationType;
  duration: number;
  startTime: number;
}

/**
 * 音效管理接口
 */
export interface SoundManager {
  playSound: (filePath: string, volume?: number) => Promise<void>;
  stopSound: () => void;
  setVolume: (volume: number) => void;
}

/**
 * Canvas渲染器接口
 */
export interface CanvasRenderer {
  render: (params: RenderParams) => Promise<void>;
  clear: () => void;
  resize: (width: number, height: number) => void;
}

/**
 * 特效管理器接口
 */
export interface EffectManager {
  addEffect: (data: EffectTriggerData) => void;
  removeEffect: (id: string) => void;
  clearAll: () => void;
  getActiveEffects: () => EffectTriggerData[];
}

// 添加字符串字面量类型以支持数据库存储
export type EffectTypeString = 'captain_first' | 'captain_renew' | 'gift_combo';
export type AnimationTypeString = 'fade' | 'slide' | 'scale' | 'rotate' | 'particle';
