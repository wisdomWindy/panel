/**
 * 高光检测器
 * 监听弹幕/礼物/舰长事件，自动标记高光时刻
 */

import { EventEmitter } from 'events';
import type { HighlightConfig, HighlightReason } from '../../../renderer/modules/recording/types.js';

interface DanmakuEvent {
  userId: string;
  username: string;
  content: string;
  timestamp: number;
}

interface GiftEvent {
  userId: string;
  username: string;
  giftId: string;
  giftName: string;
  price: number;
  count: number;
  timestamp: number;
}

interface CaptainEvent {
  userId: string;
  username: string;
  level: number;
  timestamp: number;
}

export class HighlightDetector extends EventEmitter {
  private config: HighlightConfig;
  private danmakuTimestamps: number[] = [];
  private lastHighlightTime: number = 0;
  private isRunning: boolean = false;

  constructor(config: HighlightConfig) {
    super();
    this.config = config;
  }

  /**
   * 启动检测器
   */
  start(): void {
    this.isRunning = true;
    this.danmakuTimestamps = [];
    this.lastHighlightTime = 0;
    this.emit('started');
  }

  /**
   * 停止检测器
   */
  stop(): void {
    this.isRunning = false;
    this.danmakuTimestamps = [];
    this.emit('stopped');
  }

  /**
   * 更新配置
   */
  updateConfig(config: Partial<HighlightConfig>): void {
    this.config = { ...this.config, ...config };
  }

  /**
   * 处理弹幕事件
   */
  handleDanmaku(event: DanmakuEvent): void {
    if (!this.isRunning) return;

    const now = event.timestamp || Date.now();
    this.danmakuTimestamps.push(now);

    // 清理过期的弹幕时间戳（滑动窗口）
    const windowStart = now - this.config.danmakuWindow * 1000;
    this.danmakuTimestamps = this.danmakuTimestamps.filter(t => t >= windowStart);

    // 检测弹幕激增
    if (this.danmakuTimestamps.length >= this.config.danmakuThreshold) {
      this.tryMarkHighlight('danmaku_surge', `弹幕激增：${this.config.danmakuWindow}秒内${this.danmakuTimestamps.length}条弹幕`);
    }
  }

  /**
   * 处理礼物事件
   */
  handleGift(event: GiftEvent): void {
    if (!this.isRunning) return;

    // 检测高价值礼物
    const giftValue = event.price * event.count;
    if (giftValue >= this.config.giftValueThreshold) {
      this.tryMarkHighlight('high_value_gift', `高价值礼物：${event.username} 送出 ${event.giftName} x${event.count}（价值 ${giftValue}）`);
    }
  }

  /**
   * 处理舰长事件
   */
  handleCaptain(event: CaptainEvent): void {
    if (!this.isRunning) return;

    if (this.config.captainTrigger) {
      const levelNames = ['舰长', '提督', '总督'];
      const levelName = levelNames[event.level - 1] || '舰长';
      this.tryMarkHighlight('captain', `${event.username} 开通${levelName}`);
    }
  }

  /**
   * 手动标记高光
   */
  manualMark(description?: string): void {
    this.tryMarkHighlight('manual', description || '手动标记');
  }

  /**
   * 尝试标记高光（检查冷却时间）
   */
  private tryMarkHighlight(reason: HighlightReason, description: string): void {
    const now = Date.now();

    // 检查冷却时间
    if (now - this.lastHighlightTime < this.config.cooldownSeconds * 1000) {
      this.emit('cooldown', { reason, description, remainingCooldown: this.config.cooldownSeconds - (now - this.lastHighlightTime) / 1000 });
      return;
    }

    this.lastHighlightTime = now;

    const highlight = {
      reason,
      description,
      timestamp: now,
      clipDuration: this.config.clipDuration,
      beforeSeconds: this.config.beforeSeconds,
      afterSeconds: this.config.afterSeconds
    };

    this.emit('highlight', highlight);
  }

  /**
   * 获取当前弹幕频率
   */
  getDanmakuFrequency(): number {
    const now = Date.now();
    const windowStart = now - this.config.danmakuWindow * 1000;
    const recentDanmaku = this.danmakuTimestamps.filter(t => t >= windowStart);
    return recentDanmaku.length;
  }

  /**
   * 获取检测器状态
   */
  getStatus(): {
    isRunning: boolean;
    danmakuFrequency: number;
    lastHighlightTime: number;
    cooldownRemaining: number;
    } {
    const now = Date.now();
    return {
      isRunning: this.isRunning,
      danmakuFrequency: this.getDanmakuFrequency(),
      lastHighlightTime: this.lastHighlightTime,
      cooldownRemaining: Math.max(0, this.config.cooldownSeconds - (now - this.lastHighlightTime) / 1000)
    };
  }

  /**
   * 销毁检测器
   */
  destroy(): void {
    this.stop();
    this.removeAllListeners();
  }
}
