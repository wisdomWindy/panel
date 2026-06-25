/**
 * 弹幕服务
 * 集成 DanmakuManager 与 UI 组件
 */

import { DanmakuManager } from '../modules/danmaku/manager';
import { ConnectionStatus, Gift } from '../modules/danmaku/types';
import { useRoomStore } from '../stores/room';
import { useDanmakuStore } from '../stores/danmaku';
import { useConfigStore } from '../stores/config';
import { useRecordingStore } from '../stores/recording';
import { filterSensitiveWords } from '../modules/utils/sensitive-words';
import { EffectType } from '../modules/effects/types';
import { databaseService } from './database-service';
import type { EffectManager } from '../modules/effects/types';
import {
  buildRecordingStartOptions,
  canStartRecording
} from '../views/recording/utils/recording-context.js';

type InitializableEffectManager = EffectManager & {
  initialize: () => Promise<void>;
};

export class DanmakuService {
  private manager: DanmakuManager;
  private roomStore = useRoomStore();
  private danmakuStore = useDanmakuStore();
  private configStore = useConfigStore();
  private recordingStore = useRecordingStore();
  private effectManager: EffectManager | null = null;

  constructor() {
    this.manager = new DanmakuManager({
      accessKeyId: this.configStore.config.accessKeyId,
      accessKeySecret: this.configStore.config.accessKeySecret,
      appId: this.configStore.config.appId,
      code: this.roomStore.code,
      minInterval: this.configStore.config.minSendInterval,
      csrfToken: this.roomStore.csrfToken
    });

    this.setupEventHandlers();
  }

  /**
   * 初始化特效管理器（需要在 Canvas 元素准备好后调用）
   */
  async initEffects(canvas: HTMLCanvasElement): Promise<void> {
    const { createEffectManager } = await import('../modules/effects/effect-manager');
    const effectManager = createEffectManager(canvas) as InitializableEffectManager;
    await effectManager.initialize();
    this.effectManager = effectManager;
  }

  /**
   * 获取特效管理器
   */
  getEffectManager(): EffectManager | null {
    return this.effectManager;
  }

  /**
   * 设置事件处理器
   */
  private setupEventHandlers() {
    // 连接状态变化
    this.manager.on('statusChanged', (status: ConnectionStatus) => {
      this.roomStore.setStatus(status);
    });

    // 房间信息更新
    // eslint-disable-next-line @typescript-eslint/no-explicit-any
    this.manager.on('roomInfoUpdated', (roomInfo: any) => {
      this.roomStore.setRoomInfo(roomInfo);
    });

    // 弹幕接收
    // eslint-disable-next-line @typescript-eslint/no-explicit-any
    this.manager.on('danmaku', (danmaku: any) => {
      this.danmakuStore.addDanmaku(danmaku);

      // 敏感词过滤
      if (this.configStore.config.enableSensitiveWordFilter) {
        filterSensitiveWords(
          danmaku.content,
          this.configStore.config.sensitiveWords,
          { replaceChar: this.configStore.config.replaceChar }
        );
      }
    });

    // 礼物接收
    this.manager.on('gift', (gift: Gift) => {
      this.danmakuStore.addGift(gift);
      databaseService.addGift(gift).catch((err) => console.error('Failed to save gift:', err));

      // 触发礼物连击特效
      if (gift.isCombo && gift.count >= 3 && this.effectManager) {
        this.effectManager.addEffect({
          type: EffectType.GIFT_COMBO,
          comboInfo: {
            userId: gift.userId,
            username: gift.username,
            giftId: gift.giftId,
            giftName: gift.giftName,
            count: gift.count,
            comboId: gift.comboId || '',
            timestamp: gift.timestamp
          },
          timestamp: Date.now()
        });
      }
    });

    // 弹幕池更新
    this.manager.on('poolUpdated', (pool) => {
      databaseService.setDanmakuPool(pool).catch((err) => console.error('Failed to save danmaku pool:', err));
    });

    // 弹幕发送成功
    this.manager.on('danmakuSent', (_data: { content: string; timestamp: number }) => {
      this.danmakuStore.recordSentTime();
      this.danmakuStore.incrementSentCount();
      ElMessage.success('弹幕发送成功');
    });

    // 弹幕发送失败
    // eslint-disable-next-line @typescript-eslint/no-explicit-any
    this.manager.on('danmakuFailed', (_data: any) => {
      this.danmakuStore.incrementFailedCount();
      ElMessage.error('发送失败');
    });

    // 错误处理
    // eslint-disable-next-line @typescript-eslint/no-explicit-any
    this.manager.on('error', (error: any) => {
      ElMessage.error(`发生错误: ${error.message}`);
    });

    // 刷屏检测 -> 自动禁言 5 分钟
    this.manager.on('spamDetected', (data: { userId: string; content: string; timestamp: number }) => {
      this.manager.addToBlacklist(data.userId, data.userId, `自动禁言：刷屏 (${data.content})`, 5);
      ElMessage.warning(`用户 ${data.userId} 因刷屏被自动禁言 5 分钟`);
    });

    // 感谢弹幕计数
    this.manager.on('danmakuSent', (data: { content: string; timestamp: number }) => {
      if (data.content.includes('感谢') || data.content.includes('谢谢')) {
        this.danmakuStore.incrementThankCount();
      }
    });

    // 自动录制事件处理
    this.manager.on('autoRecordStart', () => {
      this.handleAutoRecordStart();
    });
  }

  /**
   * 处理自动录制开始事件
   * 当连接成功且直播间开播时触发
   */
  private handleAutoRecordStart(): void {
    // 检查是否启用了自动录制功能
    if (this.configStore.config.enableAutoRecord) {
      const roomContext = this.roomStore.roomInfo
        ? {
            roomId: String(this.roomStore.roomInfo.roomId),
            roomName: this.roomStore.roomInfo.roomName
          }
        : null;
      const canStart = canStartRecording(roomContext);
      if (!canStart.ok || this.recordingStore.isRecording) {
        return;
      }
      void this.recordingStore.startRecording(
        buildRecordingStartOptions(roomContext as { roomId: string; roomName?: string }, this.recordingStore.config)
      ).catch((error) => {
        console.error('自动录制启动失败:', error);
      });
    }
  }

  /**
   * 连接直播间 (使用开放平台 code + appId)
   */
  async connect(): Promise<void> {
    try {
      this.roomStore.setStatus('connecting');

      // 清理旧 manager 的事件监听器，防止内存泄漏
      this.manager.removeAllListeners();

      // 更新 manager 中的认证信息
      this.manager = new DanmakuManager({
        accessKeyId: this.configStore.config.accessKeyId,
        accessKeySecret: this.configStore.config.accessKeySecret,
        appId: this.configStore.config.appId,
        code: this.roomStore.code,
        minInterval: this.configStore.config.minSendInterval,
        csrfToken: this.roomStore.csrfToken
      });
      this.setupEventHandlers();

      await this.manager.connect();
    } catch (error) {
      this.roomStore.setStatus('error');
      throw error;
    }
  }

  /**
   * 断开连接
   */
  async disconnect(): Promise<void> {
    await this.manager.disconnect();
    this.roomStore.reset();
    this.danmakuStore.clearAll();
    ElMessage.info('已断开连接');
  }

  /**
   * 发送弹幕
   */
  async sendDanmaku(content: string): Promise<void> {
    await this.manager.sendDanmaku(content);
  }

  /**
   * 添加弹幕到池
   */
  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  addToPool(item: any): void {
    const poolItem = this.manager.addToPool(item);
    this.danmakuStore.danmakuPool.push({
      ...poolItem,
      interval: poolItem.interval || 5000
    });
  }

  /**
   * 从池移除弹幕
   */
  removeFromPool(id: number): void {
    this.manager.removeFromPool(id);
    this.danmakuStore.removePoolItem(id);
  }

  /**
   * 更新弹幕池项
   */
  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  updatePoolItem(id: number, updates: any): void {
    this.manager.updatePoolItem(id, updates);
    this.danmakuStore.updatePoolItem(id, updates);
  }

  /**
   * 启动自动发送
   */
  startAutoSend(interval: number = 5000): void {
    this.manager.startAutoSend(interval);
    this.danmakuStore.setAutoSending(true);
    this.danmakuStore.setAutoSendInterval(interval);
  }

  /**
   * 停止自动发送
   */
  stopAutoSend(): void {
    this.manager.stopAutoSend();
    this.danmakuStore.setAutoSending(false);
  }

  /**
   * 更新最小发送间隔
   */
  updateMinInterval(interval: number): void {
    this.manager.setMinInterval(interval);
  }

  /**
   * 获取连接状态
   */
  get isConnected(): boolean {
    return this.roomStore.isConnected;
  }

  /**
   * 获取房间信息
   */
  get roomInfo() {
    return this.roomStore.roomInfo;
  }

  /**
   * 获取弹幕池
   */
  get danmakuPool() {
    return this.danmakuStore.danmakuPool;
  }

  /**
   * 获取自动发送状态
   */
  get isAutoSending() {
    return this.danmakuStore.isAutoSending;
  }
}

let _instance: DanmakuService | null = null;

/**
 * 获取 DanmakuService 单例（懒初始化）
 * 必须在 Pinia 安装之后、组件上下文中首次调用
 */
export function getDanmakuService(): DanmakuService {
  if (!_instance) {
    _instance = new DanmakuService();
  }
  return _instance;
}
