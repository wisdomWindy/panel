/**
 * 弹幕管理器
 * 集成 Bilibili 开放平台客户端和弹幕解析器，负责接收事件分发
 */

import { EventEmitter } from 'events';
import { Gift, ConnectionStatus, RoomInfo, BlacklistItem } from './types';
import {
  ClientEvent,
  RawWsMessage,
  OpenPlatformDanmakuMessage
} from '../bilibili/types';
import { BilibiliClient } from '../bilibili/client';
import { DanmakuParser } from './parser';
import { blacklistManager } from './blacklist';

/**
 * 弹幕管理器
 */
export class DanmakuManager extends EventEmitter {
  private client: BilibiliClient;

  private _status: ConnectionStatus = 'disconnected';
  private _roomInfo: RoomInfo | null = null;

  constructor(config?: {
    accessKeyId?: string;
    accessKeySecret?: string;
    appId?: number;
    code?: string;
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

    this.setupEventHandlers();
  }

  get status(): ConnectionStatus {
    return this._status;
  }

  get roomInfo(): RoomInfo | null {
    return this._roomInfo;
  }

  private setupEventHandlers() {
    this.client.on(ClientEvent.CONNECTED, (roomInfo: RoomInfo) => {
      this._status = 'connected';
      this._roomInfo = roomInfo;
      this.emit('statusChanged', this._status);
      this.emit('roomInfoUpdated', roomInfo);

      if (roomInfo.liveStatus === 1) {
        this.emit('autoRecordStart');
      }
    });

    this.client.on(ClientEvent.DISCONNECTED, () => {
      this._status = 'disconnected';
      this.emit('statusChanged', this._status);
    });

    this.client.on(ClientEvent.ERROR, (error: Error) => {
      this._status = 'error';
      this.emit('statusChanged', this._status);
      this.emit('error', error);
    });

    // 监听开播事件
    this.client.on(ClientEvent.LIVE_START, () => {
      if (this._roomInfo) {
        this._roomInfo.liveStatus = 1;
        this.emit('roomInfoUpdated', this._roomInfo);
        this.emit('autoRecordStart');
      }
    });

    // 监听下播事件
    this.client.on(ClientEvent.LIVE_END, () => {
      if (this._roomInfo) {
        this._roomInfo.liveStatus = 0;
        this.emit('roomInfoUpdated', this._roomInfo);
      }
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
    await this.client.disconnect();
    this._roomInfo = null;
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
