/**
 * 弹幕管理器
 * 集成 Bilibili 开放平台客户端、弹幕发送器、弹幕解析器
 * 实现自动发送、触发词回复、礼物感谢、角色互动等功能
 */

import { EventEmitter } from 'events';
import { Gift, ConnectionStatus, RoomInfo, DanmakuPoolItem, BlacklistItem } from './types';
import {
  ClientEvent,
  RawWsMessage,
  OpenPlatformDanmakuMessage
} from '../bilibili/types';
import { BilibiliClient } from '../bilibili/client';
import { DanmakuSender } from './sender';
import { DanmakuParser, ParsedDanmaku } from './parser';
import { blacklistManager } from './blacklist';

/** 用户角色回复规则 */
interface RoleReplyRule {
  userType: 'normal' | 'captain' | 'admin' | 'owner';
  template: string;
  cooldownSeconds: number;
}

/**
 * 弹幕管理器
 */
export class DanmakuManager extends EventEmitter {
  private client: BilibiliClient;
  private sender: DanmakuSender;

  private _status: ConnectionStatus = 'disconnected';
  private _roomInfo: RoomInfo | null = null;
  private _danmakuPool: DanmakuPoolItem[] = [];

  // 自动发送相关
  private autoSendTimer: NodeJS.Timeout | null = null;
  private autoSendInterval: number = 5000;

  // 定时发送相关
  private timedSendTimers: Map<number, NodeJS.Timeout> = new Map();

  // 角色回复规则
  private roleReplyRules: RoleReplyRule[] = [
    { userType: 'owner', template: '欢迎 {username} 主播！', cooldownSeconds: 300 },
    { userType: 'admin', template: '房管 {username} 来了！', cooldownSeconds: 120 },
    { userType: 'captain', template: '感谢舰长 {username} 的支持！', cooldownSeconds: 60 }
  ];

  // 角色回复冷却记录 (userId -> lastReplyTimestamp)
  private roleReplyCooldowns: Map<string, number> = new Map();

  constructor(config?: {
    accessKeyId?: string;
    accessKeySecret?: string;
    appId?: number;
    code?: string;
    minInterval?: number;
    csrfToken?: string;
  }) {
    super();
    this.client = new BilibiliClient({
      accessKeyId: config?.accessKeyId || '',
      accessKeySecret: config?.accessKeySecret || '',
      appId: config?.appId || 0,
      code: config?.code || '',
      heartbeatInterval: 20,
      reconnectInterval: 3000,
      maxReconnectAttempts: 10
    });

    this.sender = new DanmakuSender(config?.minInterval || 3000, config?.csrfToken);

    this.setupEventHandlers();
  }

  get status(): ConnectionStatus {
    return this._status;
  }

  get roomInfo(): RoomInfo | null {
    return this._roomInfo;
  }

  get danmakuPool(): DanmakuPoolItem[] {
    return this._danmakuPool;
  }

  private setupEventHandlers() {
    this.client.on(ClientEvent.CONNECTED, (roomInfo: RoomInfo) => {
      this._status = 'connected';
      this._roomInfo = roomInfo;
      this.emit('statusChanged', this._status);
      this.emit('roomInfoUpdated', roomInfo);

      if (this.autoSendTimer) {
        this.startAutoSend(this.autoSendInterval);
      }

      this.restartTimedSend();
    });

    this.client.on(ClientEvent.DISCONNECTED, () => {
      this._status = 'disconnected';
      this.stopAutoSend();
      this.stopAllTimedSend();
      this.emit('statusChanged', this._status);
    });

    this.client.on(ClientEvent.ERROR, (error: Error) => {
      this._status = 'error';
      this.emit('statusChanged', this._status);
      this.emit('error', error);
    });

    // 弹幕事件
    this.client.on(ClientEvent.DANMAKU, (message: OpenPlatformDanmakuMessage) => {
      this.handleDanmaku(message);
    });

    // 礼物事件
    this.client.on(ClientEvent.GIFT, (message: RawWsMessage) => {
      // 从 RawWsMessage 中提取 gift data
      const giftMsg = message as unknown as {
        cmd: string;
        data: {
          room_id: number;
          open_id: string;
          uname: string;
          uface: string;
          gift_id: number;
          gift_name: string;
          gift_num: number;
          price: number;
          paid: boolean;
          fans_medal_level: number;
          fans_medal_name: string;
          fans_medal_wearing_status: boolean;
          guard_level: number;
          timestamp: number;
          msg_id: string;
          gift_icon: string;
          combo_gift: boolean;
          combo_info: {
            combo_base_num: number;
            combo_count: number;
            combo_id: string;
            combo_timeout: number;
          };
          blind_gift: {
            blind_gift_id: number;
            status: boolean;
          };
          anchor_info: {
            uid: number;
            open_id: string;
            uname: string;
            uface: string;
          };
        };
      };
      this.handleGift(giftMsg);
    });

    // 大航海事件
    this.client.on(ClientEvent.GUARD, (message: RawWsMessage) => {
      const guardMsg = message as unknown as {
        data: {
          user_info: { open_id: string; uname: string };
          guard_level: number;
          guard_num: number;
          price: number;
          room_id: number;
          timestamp: number;
        };
      };
      const gift: Gift = {
        id: `guard_${guardMsg.data.timestamp}_${guardMsg.data.user_info.open_id}`,
        giftId: `guard_${guardMsg.data.guard_level}`,
        giftName: ['', '总督', '提督', '舰长'][guardMsg.data.guard_level] || '大航海',
        userId: guardMsg.data.user_info.open_id,
        username: guardMsg.data.user_info.uname,
        count: guardMsg.data.guard_num,
        price: guardMsg.data.price,
        timestamp: guardMsg.data.timestamp,
        isCombo: false
      };
      this.emit('gift', gift);
      const thankMessage = this.generateThankMessage(gift);
      if (thankMessage) {
        this.sendDanmaku(thankMessage);
      }
    });

    // 黑名单管理器事件转发
    blacklistManager.on('banned', (item: BlacklistItem) => {
      this.emit('userBanned', item);
    });
    blacklistManager.on('unbanned', (item: BlacklistItem) => {
      this.emit('userUnbanned', item);
    });
    blacklistManager.on('spamDetected', (data: { userId: string; content: string; timestamp: number }) => {
      this.emit('spamDetected', data);
    });
  }

  /**
   * 处理弹幕消息
   */
  private handleDanmaku(message: OpenPlatformDanmakuMessage) {
    const danmaku = DanmakuParser.parse(
      message,
      message.data.room_id || this._roomInfo?.roomId || 0,
      this._roomInfo?.ownerOpenId || ''
    );

    this.emit('danmaku', danmaku);

    if (blacklistManager.isBlacklisted(danmaku.userId)) {
      this.emit('blacklistedUser', { userId: danmaku.userId, username: danmaku.username });
      return;
    }

    blacklistManager.checkSpam(danmaku.userId, danmaku.content);

    this.checkRoleReply(danmaku);

    this.checkTriggerWords(danmaku);
  }

  /**
   * 角色差异化回复
   */
  private checkRoleReply(danmaku: ParsedDanmaku): void {
    if (danmaku.userType === 'normal') return;

    const rule = this.roleReplyRules.find(r => r.userType === danmaku.userType);
    if (!rule) return;

    const lastReply = this.roleReplyCooldowns.get(danmaku.userId);
    if (lastReply && Date.now() - lastReply < rule.cooldownSeconds * 1000) return;

    const reply = rule.template
      .replace(/\{username\}/g, danmaku.username)
      .replace(/\{userType\}/g, danmaku.userType);

    this.roleReplyCooldowns.set(danmaku.userId, Date.now());
    this.sendDanmaku(reply);
  }

  /**
   * 处理礼物消息
   */
  private handleGift(message: {
    cmd: string;
    data: {
      open_id: string;
      uname: string;
      gift_id: number;
      gift_name: string;
      gift_num: number;
      price: number;
      paid: boolean;
      timestamp: number;
      combo_gift: boolean;
      combo_info: { combo_id: string; combo_count: number; combo_base_num: number };
      blind_gift: { blind_gift_id: number; status: boolean };
    };
  }) {
    const data = message.data;
    const gift: Gift = {
      id: `${data.timestamp}_${data.open_id}`,
      giftId: `${data.gift_id}`,
      giftName: data.gift_name,
      userId: data.open_id,
      username: data.uname,
      count: data.gift_num,
      price: data.price,
      timestamp: data.timestamp,
      isCombo: data.combo_gift,
      comboId: data.combo_info?.combo_id
    };

    this.emit('gift', gift);

    const thankMessage = this.generateThankMessage(gift);
    if (thankMessage) {
      this.sendDanmaku(thankMessage);
    }
  }

  /**
   * 检查触发词回复
   */
  private checkTriggerWords(danmaku: ParsedDanmaku) {
    for (const poolItem of this._danmakuPool) {
      if (!poolItem.enabled || poolItem.type !== 'trigger') continue;

      if (danmaku.content.includes(poolItem.triggerWord || '')) {
        let reply = poolItem.content;

        reply = reply.replace(/\{username\}/g, danmaku.username);
        reply = reply.replace(/\{content\}/g, danmaku.content);
        reply = reply.replace(/\{userType\}/g, danmaku.userType);

        this.sendDanmaku(reply);
        break;
      }
    }
  }

  /**
   * 生成感谢消息
   */
  private generateThankMessage(gift: Gift): string | null {
    const cheapGifts = ['小花花', '辣条', '棒棒糖'];

    const templates: { [key: string]: string[] } = {
      cheap: [`感谢 ${gift.username} 送的 ${gift.giftName}`, `谢谢 ${gift.username}！`],
      normal: [
        `感谢 ${gift.username} 送的 ${gift.giftName} ×${gift.count}`,
        `谢谢 ${gift.username} 的 ${gift.giftName}！`
      ],
      expensive: [
        `感谢 ${gift.username} 送的 ${gift.giftName} ×${gift.count}`,
        `谢谢 ${gift.username} 的 ${gift.giftName}！`,
        `${gift.username} 老板大气！送了 ${gift.giftName} ×${gift.count}`
      ]
    };

    let category = 'normal';
    if (cheapGifts.includes(gift.giftName)) {
      category = 'cheap';
    } else if (gift.price >= 100) {
      category = 'expensive';
    }

    const categoryTemplates = templates[category];
    return categoryTemplates[Math.floor(Math.random() * categoryTemplates.length)];
  }

  /**
   * 连接直播间
   */
  async connect(): Promise<void> {
    if (this._status === 'connected') {
      throw new Error('已连接到直播间');
    }

    this._status = 'connecting';
    this.emit('statusChanged', this._status);

    const roomInfo = await this.client.connect();
    this._roomInfo = roomInfo;
  }

  /**
   * 断开连接
   */
  async disconnect(): Promise<void> {
    this.stopAutoSend();
    this.stopAllTimedSend();
    await this.client.disconnect();
    this._roomInfo = null;
  }

  /**
   * 发送弹幕
   */
  async sendDanmaku(content: string): Promise<void> {
    if (!this._roomInfo) {
      throw new Error('未连接到直播间');
    }

    const result = await this.sender.send(content, this._roomInfo.roomId);

    if (result.success) {
      this.emit('danmakuSent', { content, timestamp: Date.now() });
    } else {
      this.emit('danmakuFailed', { content, message: result.message });
    }
  }

  /**
   * 添加到弹幕池
   */
  addToPool(item: Partial<DanmakuPoolItem>): DanmakuPoolItem {
    const newItem: DanmakuPoolItem = {
      id: Date.now(),
      content: item.content || '',
      type: item.type || 'timed',
      triggerWord: item.triggerWord,
      enabled: item.enabled ?? true,
      priority: item.priority || 0,
      interval: item.interval || 5000,
      createdAt: new Date(),
      updatedAt: new Date()
    };

    this._danmakuPool.push(newItem);
    this.emit('poolUpdated', this._danmakuPool);

    if (newItem.type === 'timed' && newItem.enabled && this._status === 'connected') {
      this.startTimedSend(newItem);
    }

    return newItem;
  }

  removeFromPool(id: number): void {
    const item = this._danmakuPool.find(i => i.id === id);

    if (item && item.type === 'timed') {
      this.stopTimedSend(id);
    }

    this._danmakuPool = this._danmakuPool.filter(item => item.id !== id);
    this.emit('poolUpdated', this._danmakuPool);
  }

  updatePoolItem(id: number, updates: Partial<DanmakuPoolItem>): void {
    const index = this._danmakuPool.findIndex(item => item.id === id);
    if (index !== -1) {
      const oldItem = this._danmakuPool[index];
      this._danmakuPool[index] = {
        ...oldItem,
        ...updates,
        updatedAt: new Date()
      };

      this.emit('poolUpdated', this._danmakuPool);

      if (this._danmakuPool[index].type === 'timed') {
        if (this._danmakuPool[index].enabled && this._status === 'connected') {
          this.startTimedSend(this._danmakuPool[index]);
        } else {
          this.stopTimedSend(id);
        }
      }
    }
  }

  startAutoSend(interval: number = 5000): void {
    this.stopAutoSend();

    this.autoSendInterval = interval;

    const randomItems = this._danmakuPool.filter(item => item.enabled && item.type === 'random');

    if (randomItems.length === 0) {
      return;
    }

    this.autoSendTimer = setInterval(async () => {
      const item = randomItems[Math.floor(Math.random() * randomItems.length)];
      await this.sendDanmaku(item.content);
    }, interval);
  }

  stopAutoSend(): void {
    if (this.autoSendTimer) {
      clearInterval(this.autoSendTimer);
      this.autoSendTimer = null;
    }
  }

  private startTimedSend(item: DanmakuPoolItem): void {
    if (!item.interval) return;

    this.stopTimedSend(item.id);

    const timer = setInterval(async () => {
      await this.sendDanmaku(item.content);
    }, item.interval);

    this.timedSendTimers.set(item.id, timer);
  }

  private stopTimedSend(id: number): void {
    const timer = this.timedSendTimers.get(id);
    if (timer) {
      clearInterval(timer);
      this.timedSendTimers.delete(id);
    }
  }

  private stopAllTimedSend(): void {
    this.timedSendTimers.forEach(timer => clearInterval(timer));
    this.timedSendTimers.clear();
  }

  private restartTimedSend(): void {
    const timedItems = this._danmakuPool.filter(item => item.enabled && item.type === 'timed');
    timedItems.forEach(item => this.startTimedSend(item));
  }

  setMinInterval(interval: number): void {
    this.sender.setMinInterval(interval);
  }

  getRoleReplyRules(): RoleReplyRule[] {
    return [...this.roleReplyRules];
  }

  setRoleReplyRules(rules: RoleReplyRule[]): void {
    this.roleReplyRules = [...rules];
  }

  addToBlacklist(userId: string, username: string, reason: string, duration?: number): void {
    blacklistManager.addToBlacklist(userId, username, reason, duration);
  }

  removeFromBlacklist(userId: string): void {
    blacklistManager.removeFromBlacklist(userId);
  }

  isBlacklisted(userId: string): boolean {
    return blacklistManager.isBlacklisted(userId);
  }

  getBlacklist(): BlacklistItem[] {
    return blacklistManager.getBlacklist();
  }

  markHighlight(label: string): void {
    this.emit('highlightMarked', { timestamp: Date.now(), label });
  }
}
