/**
 * 自动禁言与刷屏防护服务
 * 检测违禁词和刷屏行为，自动执行禁言操作
 */

import type {
  ModerationConfig,
  MuteRecord,
  UserMessageRecord,
  FloodDetectionResult,
  MuteReason
} from '../modules/moderation/types.js';
import { DEFAULT_MODERATION_CONFIG } from '../modules/moderation/types.js';

export class ModerationService {
  private config: ModerationConfig = { ...DEFAULT_MODERATION_CONFIG };
  private userMessages: Map<string, UserMessageRecord[]> = new Map();
  private muteRecords: MuteRecord[] = [];
  private warningUsers: Set<string> = new Set();
  private statusListeners: Array<(status: string) => void> = [];
  private muteListeners: Array<(record: MuteRecord) => void> = [];

  constructor() {}

  /**
   * 更新配置
   */
  updateConfig(config: Partial<ModerationConfig>): void {
    this.config = { ...this.config, ...config };
  }

  /**
   * 获取配置
   */
  getConfig(): ModerationConfig {
    return { ...this.config };
  }

  /**
   * 添加状态监听器
   */
  onStatusChange(listener: (status: string) => void): void {
    this.statusListeners.push(listener);
  }

  /**
   * 添加禁言监听器
   */
  onMute(listener: (record: MuteRecord) => void): void {
    this.muteListeners.push(listener);
  }

  /**
   * 处理弹幕消息
   */
  handleMessage(userId: string, username: string, content: string, roomId: string): MuteRecord | null {
    if (!this.config.enabled) return null;

    // 1. 检查违禁词
    if (this.config.autoMuteEnabled) {
      const prohibitedResult = this.checkProhibitedWords(content);
      if (prohibitedResult) {
        return this.executeMute(userId, username, content, 'prohibited_word', roomId);
      }
    }

    // 2. 检查刷屏
    if (this.config.floodProtectionEnabled) {
      const floodResult = this.checkFlood(userId, content);
      if (floodResult.isFlood) {
        // 检查是否需要警告
        if (this.config.warningBeforeMute && !this.warningUsers.has(userId)) {
          this.warningUsers.add(userId);
          this.statusListeners.forEach(listener => listener(`warning:${userId}`));
          return null;
        }
        return this.executeMute(userId, username, content, 'flood', roomId);
      }
    }

    // 记录用户消息
    this.recordMessage(userId, username, content);

    return null;
  }

  /**
   * 检查违禁词
   */
  private checkProhibitedWords(content: string): boolean {
    const lowerContent = content.toLowerCase();
    return this.config.prohibitedWords.some(word =>
      lowerContent.includes(word.toLowerCase())
    );
  }

  /**
   * 检查刷屏
   */
  private checkFlood(userId: string, content: string): FloodDetectionResult {
    const now = Date.now();
    const windowStart = now - this.config.floodTimeWindow * 1000;

    // 获取用户消息记录
    let messages = this.userMessages.get(userId) || [];

    // 清理过期消息
    messages = messages.filter(m => m.timestamp >= windowStart);

    // 统计重复消息
    const duplicateCount = messages.filter(m => m.content === content).length;

    // 更新消息记录
    messages.push({
      userId,
      username: '',
      content,
      timestamp: now
    });
    this.userMessages.set(userId, messages);

    // 判断是否刷屏
    const isFlood = messages.length >= this.config.floodThreshold ||
                    duplicateCount >= 3;  // 3条相同内容视为刷屏

    return {
      isFlood,
      messageCount: messages.length,
      timeWindow: this.config.floodTimeWindow,
      duplicateCount
    };
  }

  /**
   * 记录用户消息
   */
  private recordMessage(userId: string, username: string, content: string): void {
    const now = Date.now();
    const windowStart = now - this.config.floodTimeWindow * 1000;

    let messages = this.userMessages.get(userId) || [];
    messages = messages.filter(m => m.timestamp >= windowStart);
    messages.push({
      userId,
      username,
      content,
      timestamp: now
    });

    // 限制消息记录数量
    if (messages.length > 100) {
      messages = messages.slice(-100);
    }

    this.userMessages.set(userId, messages);
  }

  /**
   * 执行禁言
   */
  private executeMute(
    userId: string,
    username: string,
    content: string,
    reason: MuteReason,
    roomId: string
  ): MuteRecord {
    const record: MuteRecord = {
      id: `mute_${Date.now()}_${Math.random().toString(36).substr(2, 9)}`,
      userId,
      username,
      reason,
      content,
      duration: this.config.muteDuration,
      mutedAt: new Date(),
      roomId
    };

    // 添加到禁言记录
    this.muteRecords.push(record);

    // 清理用户消息记录
    this.userMessages.delete(userId);

    // 清理警告状态
    this.warningUsers.delete(userId);

    // 通知监听器
    this.muteListeners.forEach(listener => listener(record));
    this.statusListeners.forEach(listener => listener(`muted:${userId}`));

    return record;
  }

  /**
   * 手动禁言
   */
  manualMute(userId: string, username: string, roomId: string, duration?: number): MuteRecord {
    const record: MuteRecord = {
      id: `mute_${Date.now()}_${Math.random().toString(36).substr(2, 9)}`,
      userId,
      username,
      reason: 'manual',
      content: '',
      duration: duration || this.config.muteDuration,
      mutedAt: new Date(),
      roomId
    };

    this.muteRecords.push(record);
    this.muteListeners.forEach(listener => listener(record));

    return record;
  }

  /**
   * 解除禁言
   */
  unmute(userId: string): void {
    const record = this.muteRecords.find(r => r.userId === userId && !r.unmutedAt);
    if (record) {
      record.unmutedAt = new Date();
    }
    this.warningUsers.delete(userId);
  }

  /**
   * 获取禁言记录
   */
  getMuteRecords(roomId?: string): MuteRecord[] {
    if (roomId) {
      return this.muteRecords.filter(r => r.roomId === roomId);
    }
    return [...this.muteRecords];
  }

  /**
   * 获取用户禁言状态
   */
  isUserMuted(userId: string): boolean {
    const now = Date.now();
    return this.muteRecords.some(r => {
      if (r.userId !== userId || r.unmutedAt) return false;
      if (r.duration === 0) return true;  // 永久禁言
      const muteEnd = r.mutedAt.getTime() + r.duration * 60 * 1000;
      return now < muteEnd;
    });
  }

  /**
   * 清理过期禁言记录
   */
  cleanupExpiredMutes(): void {
    const now = Date.now();
    this.muteRecords = this.muteRecords.filter(r => {
      if (r.unmutedAt) return false;
      if (r.duration === 0) return true;
      const muteEnd = r.mutedAt.getTime() + r.duration * 60 * 1000;
      return now < muteEnd;
    });
  }

  /**
   * 添加违禁词
   */
  addProhibitedWord(word: string): void {
    if (!this.config.prohibitedWords.includes(word)) {
      this.config.prohibitedWords.push(word);
    }
  }

  /**
   * 删除违禁词
   */
  removeProhibitedWord(word: string): void {
    this.config.prohibitedWords = this.config.prohibitedWords.filter(w => w !== word);
  }

  /**
   * 获取违禁词列表
   */
  getProhibitedWords(): string[] {
    return [...this.config.prohibitedWords];
  }

  /**
   * 清空消息记录
   */
  clearMessageHistory(): void {
    this.userMessages.clear();
  }

  /**
   * 销毁服务
   */
  destroy(): void {
    this.userMessages.clear();
    this.muteRecords = [];
    this.warningUsers.clear();
    this.statusListeners = [];
    this.muteListeners = [];
  }
}

// 单例
let moderationServiceInstance: ModerationService | null = null;

export function getModerationService(): ModerationService {
  if (!moderationServiceInstance) {
    moderationServiceInstance = new ModerationService();
  }
  return moderationServiceInstance;
}
