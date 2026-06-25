/**
 * 音效队列管理
 * 管理多个音效的播放顺序和优先级
 */

import type {
  GiftSoundConfig,
  PlaySoundRequest,
  SoundQueueItem
} from '../../../renderer/modules/gift-sound/types.js';

export class SoundQueue {
  private queue: SoundQueueItem[] = [];
  private isProcessing: boolean = false;
  private maxQueueSize: number = 50;
  private cooldownMap: Map<string, number> = new Map();
  private eventListeners: Map<string, Function[]> = new Map();

  constructor() {
    this.setupEventListeners();
  }

  /**
   * 添加音效到队列
   */
  add(giftSound: GiftSoundConfig, request: PlaySoundRequest): boolean {
    // 检查冷却时间
    if (this.isOnCooldown(giftSound.giftId, giftSound.cooldown)) {
      console.log(`礼物 ${giftSound.giftName} 正在冷却中`);
      return false;
    }

    // 检查队列是否已满
    if (this.queue.length >= this.maxQueueSize) {
      // 移除最旧的低优先级音效
      this.removeLowestPriority();
    }

    // 创建队列项
    const queueItem: SoundQueueItem = {
      id: Date.now().toString(),
      giftSound,
      request,
      priority: giftSound.priority,
      addedAt: new Date()
    };

    // 添加到队列并按优先级排序
    this.queue.push(queueItem);
    this.sortQueue();

    // 设置冷却时间
    this.setCooldown(giftSound.giftId, giftSound.cooldown);

    // 触发添加事件
    this.emitEvent('add', queueItem);

    console.log(`音效添加到队列: ${giftSound.giftName}, 优先级: ${giftSound.priority}`);
    return true;
  }

  /**
   * 获取下一个要播放的音效
   */
  getNext(): SoundQueueItem | null {
    if (this.queue.length === 0) {
      return null;
    }

    // 移除队首元素
    const item = this.queue.shift();
    if (item) {
      this.emitEvent('dequeue', item);
    }
    return item ?? null;
  }

  /**
   * 查看下一个音效但不移除
   */
  peek(): SoundQueueItem | null {
    return this.queue.length > 0 ? this.queue[0] : null;
  }

  /**
   * 清空队列
   */
  clear(): void {
    this.queue = [];
    this.emitEvent('clear', null);
    console.log('音效队列已清空');
  }

  /**
   * 获取队列长度
   */
  getLength(): number {
    return this.queue.length;
  }

  /**
   * 获取队列状态
   */
  getQueueInfo(): {
    length: number;
    items: SoundQueueItem[];
    isProcessing: boolean;
    } {
    return {
      length: this.queue.length,
      items: [...this.queue],
      isProcessing: this.isProcessing
    };
  }

  /**
   * 检查是否在冷却中
   */
  private isOnCooldown(giftId: string, cooldown: number): boolean {
    const lastPlayed = this.cooldownMap.get(giftId);
    if (!lastPlayed) {
      return false;
    }

    const now = Date.now();
    return now - lastPlayed < cooldown;
  }

  /**
   * 设置冷却时间
   */
  private setCooldown(giftId: string, cooldown: number): void {
    this.cooldownMap.set(giftId, Date.now());

    // 设置冷却结束后的清理
    setTimeout(() => {
      this.cooldownMap.delete(giftId);
    }, cooldown);
  }

  /**
   * 按优先级排序队列
   */
  private sortQueue(): void {
    this.queue.sort((a, b) => b.priority - a.priority);
  }

  /**
   * 移除最低优先级的音效
   */
  private removeLowestPriority(): void {
    if (this.queue.length === 0) return;

    // 找到最低优先级
    let lowestPriority = Infinity;
    let lowestIndex = -1;

    for (let i = 0; i < this.queue.length; i++) {
      if (this.queue[i].priority < lowestPriority) {
        lowestPriority = this.queue[i].priority;
        lowestIndex = i;
      }
    }

    // 移除最低优先级的音效
    if (lowestIndex >= 0) {
      const removed = this.queue.splice(lowestIndex, 1)[0];
      this.emitEvent('remove', removed);
      console.log(`移除低优先级音效: ${removed.giftSound.giftName}`);
    }
  }

  /**
   * 设置队列处理状态
   */
  setProcessing(processing: boolean): void {
    this.isProcessing = processing;
    this.emitEvent('processingChange', processing);
  }

  /**
   * 获取队列处理状态
   */
  getProcessing(): boolean {
    return this.isProcessing;
  }

  /**
   * 设置最大队列大小
   */
  setMaxQueueSize(size: number): void {
    this.maxQueueSize = Math.max(1, size);
  }

  /**
   * 设置事件监听器
   */
  private setupEventListeners(): void {
    this.eventListeners.set('add', []);
    this.eventListeners.set('dequeue', []);
    this.eventListeners.set('remove', []);
    this.eventListeners.set('clear', []);
    this.eventListeners.set('processingChange', []);
  }

  /**
   * 添加事件监听器
   */
  on(event: string, listener: Function): void {
    if (!this.eventListeners.has(event)) {
      this.eventListeners.set(event, []);
    }
    this.eventListeners.get(event)!.push(listener);
  }

  /**
   * 移除事件监听器
   */
  off(event: string, listener: Function): void {
    if (this.eventListeners.has(event)) {
      const listeners = this.eventListeners.get(event)!;
      const index = listeners.indexOf(listener);
      if (index > -1) {
        listeners.splice(index, 1);
      }
    }
  }

  /**
   * 触发事件
   */
  private emitEvent(event: string, data: any): void {
    if (this.eventListeners.has(event)) {
      this.eventListeners.get(event)!.forEach(listener => {
        try {
          listener(data);
        } catch (error) {
          console.error('事件监听器执行错误:', error);
        }
      });
    }
  }

  /**
   * 销毁队列管理器
   */
  destroy(): void {
    this.clear();
    this.cooldownMap.clear();
    this.eventListeners.clear();
    console.log('音效队列管理器已销毁');
  }
}

// 创建单例实例
export const soundQueue = new SoundQueue();
