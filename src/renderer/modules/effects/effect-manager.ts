/**
 * 特效管理器
 * 负责管理和触发各种特效
 */

import {
  EffectManager,
  EffectTriggerData,
  EffectConfig,
  EffectType,
  AnimationType,
  RenderParams
} from './types';
import { soundManager } from './sound-manager';
import { createCanvasRenderer, CanvasRendererImpl } from './canvas-renderer';
import { WebGLRenderer } from './webgl-renderer';

/**
 * 默认特效配置
 */
const DEFAULT_EFFECT_CONFIGS: EffectConfig[] = [
  {
    id: 'captain_first_default',
    type: EffectType.CAPTAIN_FIRST,
    templateName: '默认首次上舰',
    textTemplate: '感谢 {username} 首次开通 {level}！🎉',
    animationType: AnimationType.SCALE,
    primaryColor: '#FFD700',
    secondaryColor: '#FFA500',
    duration: 5000,
    enabled: true,
    renderMode: 'webgl' as const,
    createdAt: new Date(),
    updatedAt: new Date()
  },
  {
    id: 'captain_renew_default',
    type: EffectType.CAPTAIN_RENEW,
    templateName: '默认续费上舰',
    textTemplate: '感谢 {username} 续费 {level}！🎊',
    animationType: AnimationType.SLIDE,
    primaryColor: '#00FF00',
    secondaryColor: '#32CD32',
    duration: 4000,
    enabled: true,
    renderMode: 'webgl' as const,
    createdAt: new Date(),
    updatedAt: new Date()
  },
  {
    id: 'gift_combo_default',
    type: EffectType.GIFT_COMBO,
    templateName: '默认礼物连击',
    textTemplate: '{username} 连击 {count} 次 {giftName}！💥',
    animationType: AnimationType.PARTICLE,
    primaryColor: '#FF69B4',
    secondaryColor: '#FF1493',
    duration: 3000,
    enabled: true,
    renderMode: 'webgl' as const,
    createdAt: new Date(),
    updatedAt: new Date()
  }
];

/**
 * 特效管理器实现
 */
export class EffectManagerImpl implements EffectManager {
  private canvas: HTMLCanvasElement;
  private canvasRenderer: CanvasRendererImpl;
  private webglRenderer: WebGLRenderer | null = null;
  private effectConfigs: Map<string, EffectConfig> = new Map();
  private activeEffects: Map<string, EffectTriggerData> = new Map();
  private isInitialized: boolean = false;
  private effectQueue: EffectTriggerData[] = [];
  private isProcessing: boolean = false;

  constructor(canvas: HTMLCanvasElement) {
    this.canvas = canvas;
    this.canvasRenderer = createCanvasRenderer(canvas);
    try {
      this.webglRenderer = new WebGLRenderer(canvas);
    } catch (err) {
      console.warn('WebGL 渲染器初始化失败，3D 特效将不可用:', err);
    }
  }

  /**
   * 初始化特效管理器
   * @param customConfigs 自定义特效配置
   */
  async initialize(customConfigs?: EffectConfig[]): Promise<void> {
    try {
      // 初始化音效管理器
      await soundManager.initialize();

      // 加载默认特效配置
      DEFAULT_EFFECT_CONFIGS.forEach(config => {
        this.effectConfigs.set(config.id, config);
      });

      // 加载自定义特效配置
      if (customConfigs) {
        customConfigs.forEach(config => {
          this.effectConfigs.set(config.id, config);
        });
      }

      this.isInitialized = true;
      console.warn('特效管理器初始化成功');
    } catch (error) {
      console.error('特效管理器初始化失败:', error);
      throw error;
    }
  }

  /**
   * 添加特效
   * @param data 特效触发数据
   */
  addEffect(data: EffectTriggerData): void {
    if (!this.isInitialized) {
      console.warn('特效管理器未初始化');
      return;
    }

    // 生成特效ID
    const id = `effect_${Date.now()}_${Math.random().toString(36).substr(2, 9)}`;

    // 添加到活动特效
    this.activeEffects.set(id, data);

    // 添加到队列
    this.effectQueue.push(data);

    // 如果没有正在处理，则开始处理队列
    if (!this.isProcessing) {
      this.processEffectQueue();
    }

    console.warn(`添加特效: ${data.type}, ID: ${id}`);
  }

  /**
   * 移除特效
   * @param id 特效ID
   */
  removeEffect(id: string): void {
    this.activeEffects.delete(id);
    console.warn(`移除特效: ${id}`);
  }

  /**
   * 清除所有特效
   */
  clearAll(): void {
    this.activeEffects.clear();
    this.effectQueue = [];
    this.canvasRenderer.clear();
    this.webglRenderer?.clear();
    console.warn('清除所有特效');
  }

  /**
   * 获取活动特效
   * @returns 活动特效列表
   */
  getActiveEffects(): EffectTriggerData[] {
    return Array.from(this.activeEffects.values());
  }

  /**
   * 处理特效队列
   */
  private async processEffectQueue(): Promise<void> {
    if (this.isProcessing || this.effectQueue.length === 0) {
      this.isProcessing = false;
      return;
    }

    this.isProcessing = true;

    while (this.effectQueue.length > 0) {
      const data = this.effectQueue.shift();
      if (!data) break;
      await this.renderEffect(data);

      // 添加延迟，避免特效重叠
      await new Promise(resolve => setTimeout(resolve, 500));
    }

    this.isProcessing = false;
  }

  /**
   * 渲染特效
   * @param data 特效触发数据
   */
  private async renderEffect(data: EffectTriggerData): Promise<void> {
    try {
      // 获取特效配置
      const config = this.getEffectConfig(data.type);
      if (!config || !config.enabled) {
        console.warn(`特效配置不存在或已禁用: ${data.type}`);
        return;
      }

      // 生成特效文本
      const text = this.generateEffectText(config, data);

      // 获取随机位置
      const { x, y } = this.getRandomPosition();

      // 创建渲染参数
      const renderParams: RenderParams = {
        text,
        x,
        y,
        fontSize: 32,
        primaryColor: config.primaryColor,
        secondaryColor: config.secondaryColor,
        animationType: config.animationType,
        duration: config.duration,
        startTime: performance.now()
      };

      // 播放音效
      if (config.soundFile) {
        const soundKey = this.getSoundKey(data.type);
        await soundManager.playSound(soundKey, 0.7);
      }

      // 渲染特效（Canvas 2D 或 WebGL 3D）
      if (config.renderMode === 'webgl') {
        if (!this.webglRenderer) {
          console.warn('WebGL 渲染器不可用，回退到 2D 渲染');
          await this.canvasRenderer.render(renderParams);
        } else {
          this.webglRenderer.addEffect(data, config);
        }
      } else {
        await this.canvasRenderer.render(renderParams);
      }

      // 设置特效过期时间（仅 2D 模式通过定时器清理，3D 模式由 WebGLRenderer 自维护）
      if (config.renderMode !== 'webgl') {
        setTimeout(() => {
          this.removeEffect(`effect_${data.timestamp}_${Math.random().toString(36).substr(2, 9)}`);
        }, config.duration);
      }
    } catch (error) {
      console.error('渲染特效失败:', error);
    }
  }

  /**
   * 获取特效配置
   * @param type 特效类型
   * @returns 特效配置
   */
  private getEffectConfig(type: EffectType): EffectConfig | undefined {
    for (const config of this.effectConfigs.values()) {
      if (config.type === type && config.enabled) {
        return config;
      }
    }
    return undefined;
  }

  /**
   * 生成特效文本
   * @param config 特效配置
   * @param data 特效触发数据
   * @returns 生成的文本
   */
  private generateEffectText(config: EffectConfig, data: EffectTriggerData): string {
    let text = config.textTemplate;

    // 替换变量
    if (data.captainInfo) {
      const captain = data.captainInfo;
      const levelNames = ['', '督察', '提督', '总督'];

      text = text
        .replace(/{username}/g, captain.username)
        .replace(/{level}/g, levelNames[captain.level] || captain.level.toString())
        .replace(/{giftName}/g, captain.giftName)
        .replace(/{count}/g, captain.count.toString());
    } else if (data.comboInfo) {
      const combo = data.comboInfo;

      text = text
        .replace(/{username}/g, combo.username)
        .replace(/{count}/g, combo.count.toString())
        .replace(/{giftName}/g, combo.giftName);
    }

    return text;
  }

  /**
   * 获取随机位置
   * @returns 随机位置
   */
  private getRandomPosition(): { x: number; y: number } {
    const margin = 100;
    const x = margin + Math.random() * (this.canvas.width - margin * 2);
    const y = margin + Math.random() * (this.canvas.height - margin * 2);

    return { x, y };
  }

  /**
   * 获取音效键名
   * @param type 特效类型
   * @returns 音效键名
   */
  private getSoundKey(type: EffectType): string {
    switch (type) {
      case EffectType.CAPTAIN_FIRST:
        return 'captain_first';
      case EffectType.CAPTAIN_RENEW:
        return 'captain_renew';
      case EffectType.GIFT_COMBO:
        return 'gift_combo';
      default:
        return '';
    }
  }

  /**
   * 添加自定义特效配置
   * @param config 特效配置
   */
  addEffectConfig(config: EffectConfig): void {
    this.effectConfigs.set(config.id, config);
    console.warn(`添加特效配置: ${config.id}`);
  }

  /**
   * 更新特效配置
   * @param id 配置ID
   * @param updates 更新内容
   */
  updateEffectConfig(id: string, updates: Partial<EffectConfig>): void {
    const config = this.effectConfigs.get(id);
    if (config) {
      const updatedConfig = { ...config, ...updates, updatedAt: new Date() };
      this.effectConfigs.set(id, updatedConfig);
      console.warn(`更新特效配置: ${id}`);
    } else {
      console.warn(`特效配置不存在: ${id}`);
    }
  }

  /**
   * 删除特效配置
   * @param id 配置ID
   */
  deleteEffectConfig(id: string): void {
    this.effectConfigs.delete(id);
    console.warn(`删除特效配置: ${id}`);
  }

  /**
   * 获取所有特效配置
   * @returns 特效配置列表
   */
  getEffectConfigs(): EffectConfig[] {
    return Array.from(this.effectConfigs.values());
  }

  /**
   * 获取特效配置
   * @param id 配置ID
   * @returns 特效配置
   */
  getEffectConfigById(id: string): EffectConfig | undefined {
    return this.effectConfigs.get(id);
  }

  /**
   * 启用/禁用特效
   * @param id 配置ID
   * @param enabled 是否启用
   */
  toggleEffect(id: string, enabled: boolean): void {
    const config = this.effectConfigs.get(id);
    if (config) {
      this.updateEffectConfig(id, { enabled });
      console.warn(`特效 ${id} ${enabled ? '启用' : '禁用'}`);
    } else {
      console.warn(`特效配置不存在: ${id}`);
    }
  }

  /**
   * 设置音量
   * @param volume 音量 (0-1)
   */
  setVolume(volume: number): void {
    soundManager.setVolume(volume);
  }

  /**
   * 获取活动动画数量
   * @returns 活动动画数量
   */
  getActiveAnimationCount(): number {
    return this.canvasRenderer.getActiveAnimationCount();
  }

  /**
   * 检查是否有活动动画
   * @returns 是否有活动动画
   */
  hasActiveAnimations(): boolean {
    return this.canvasRenderer.hasActiveAnimations();
  }
}

/**
 * 创建特效管理器工厂函数
 * @param canvas Canvas元素
 * @returns 特效管理器实例
 */
export function createEffectManager(canvas: HTMLCanvasElement): EffectManagerImpl {
  return new EffectManagerImpl(canvas);
}
