/**
 * Bilibili礼物服务
 * 处理从Bilibili API获取礼物列表的业务逻辑
 */

import type { GiftInfo } from '../modules/gift-sound/types.js';
import { logger } from '../../main/modules/logger/logger.js';

interface ElectronAPI {
  invoke: (channel: string, ...args: unknown[]) => Promise<unknown>;
  on: (channel: string, callback: (...args: unknown[]) => void) => void;
  off: (channel: string, callback: (...args: unknown[]) => void) => void;
}

function getApi(): ElectronAPI {
  return (window as { electronAPI: ElectronAPI }).electronAPI;
}

export class BilibiliGiftService {
  private api = getApi();
  private giftCache: Map<string, { data: GiftInfo[]; timestamp: number }> = new Map();
  private readonly CACHE_DURATION = 60 * 60 * 1000; // 1小时缓存
  private collectedGifts: Map<string, GiftInfo> = new Map();
  private giftEventListener: ((event: unknown) => void) | null = null;

  constructor() {
    this.setupGiftEventListener();
  }

  /**
   * 设置礼物事件监听器，从礼物事件中收集礼物信息
   */
  private setupGiftEventListener(): void {
    this.giftEventListener = (event: unknown) => {
      try {
        const giftEvent = event as {
          gift_id: string | number;
          gift_name: string;
          gift_icon?: string;
          price?: number;
        };

        if (giftEvent.gift_id && giftEvent.gift_name) {
          const giftId = String(giftEvent.gift_id);
          if (!this.collectedGifts.has(giftId)) {
            const giftInfo: GiftInfo = {
              giftId,
              giftName: giftEvent.gift_name,
              giftPrice: giftEvent.price || 0,
              giftImg: giftEvent.gift_icon || '',
              giftDesc: ''
            };
            this.collectedGifts.set(giftId, giftInfo);
          }
        }
      } catch (error) {
        console.error('处理礼物事件失败:', error);
      }
    };

    // 监听礼物事件
    this.api.on('gift-event', this.giftEventListener);
  }

  /**
   * 获取礼物列表
   * @param roomId 直播间ID
   * @param forceRefresh 是否强制刷新
   * @returns 礼物列表
   */
  async getGiftList(roomId: string, forceRefresh = false): Promise<GiftInfo[]> {
    const maxRetries = 3;
    let lastError: Error | null = null;

    // 检查缓存
    if (!forceRefresh) {
      const cached = this.giftCache.get(roomId);
      if (cached && Date.now() - cached.timestamp < this.CACHE_DURATION) {
        return cached.data;
      }
    }

    // 重试机制
    for (let attempt = 1; attempt <= maxRetries; attempt++) {
      try {
        // 调用主进程获取礼物列表
        const result = await this.api.invoke('bilibili:getGiftList', roomId) as {
          success: boolean;
          gifts: GiftInfo[];
          error?: string;
        };

        if (result.success && result.gifts && result.gifts.length > 0) {
          // 更新缓存
          this.giftCache.set(roomId, {
            data: result.gifts,
            timestamp: Date.now()
          });
          return result.gifts;
        }

        // 如果主进程获取失败或返回空列表，使用收集的礼物列表
        const collectedGifts = Array.from(this.collectedGifts.values());
        
        // 更新缓存
        this.giftCache.set(roomId, {
          data: collectedGifts,
          timestamp: Date.now()
        });

        return collectedGifts;
      } catch (error) {
        lastError = error as Error;
        await logger.error('BilibiliGiftService', `获取礼物列表失败 (尝试 ${attempt}/${maxRetries})`, lastError, {
          roomId,
          attempt,
          maxRetries
        });
        
        // 如果不是最后一次尝试，等待一段时间后重试
        if (attempt < maxRetries) {
          await new Promise(resolve => setTimeout(resolve, 1000 * attempt));
        }
      }
    }

    // 所有重试都失败，返回收集的礼物列表
    await logger.error('BilibiliGiftService', '获取礼物列表最终失败', lastError!, {
      roomId,
      maxRetries
    });
    const collectedGifts = Array.from(this.collectedGifts.values());
    return collectedGifts;
  }

  /**
   * 调用Bilibili API获取礼物列表
   * @param roomId 直播间ID
   * @returns 礼物列表
   */
  private async fetchGiftListFromApi(roomId: string): Promise<GiftInfo[]> {
    const url = new URL('https://api.live.bilibili.com/xlive/web-room/v1/giftPanel/roomGiftList');
    url.searchParams.set('platform', 'web');
    url.searchParams.set('room_id', roomId);

    const response = await fetch(url.toString());
    
    if (!response.ok) {
      throw new Error(`API请求失败: ${response.status} ${response.statusText}`);
    }

    const data = await response.json();
    
    if (data.code !== 0) {
      throw new Error(`API返回错误: ${data.message || '未知错误'}`);
    }

    return await this.convertToGiftInfoList(data.data);
  }

  /**
   * 将API响应转换为GiftInfo列表
   * @param data API响应数据
   * @returns GiftInfo列表
   */
  private async convertToGiftInfoList(data: unknown): Promise<GiftInfo[]> {
    try {
      const giftData = data as {
        list?: Array<{
          id: number;
          name: string;
          price: number;
          img_basic?: string;
          img?: string;
          desc?: string;
        }>;
      };

      if (!giftData.list || !Array.isArray(giftData.list)) {
        return [];
      }

      return giftData.list.map(gift => ({
        giftId: String(gift.id),
        giftName: gift.name,
        giftPrice: gift.price || 0,
        giftImg: gift.img_basic || gift.img || '',
        giftDesc: gift.desc || ''
      }));
    } catch (error) {
      await logger.error('BilibiliGiftService', '转换礼物数据失败', error as Error);
      return [];
    }
  }

  /**
   * 根据礼物ID获取礼物信息
   * @param roomId 直播间ID
   * @param giftId 礼物ID
   * @returns 礼物信息
   */
  async getGiftById(roomId: string, giftId: string): Promise<GiftInfo | null> {
    try {
      const gifts = await this.getGiftList(roomId);
      return gifts.find(gift => gift.giftId === giftId) || null;
    } catch (error) {
      await logger.error('BilibiliGiftService', '获取礼物信息失败', error as Error, {
        roomId,
        giftId
      });
      throw error;
    }
  }

  /**
   * 搜索礼物
   * @param roomId 直播间ID
   * @param query 搜索关键词
   * @returns 匹配的礼物列表
   */
  async searchGifts(roomId: string, query: string): Promise<GiftInfo[]> {
    try {
      const gifts = await this.getGiftList(roomId);
      const lowerQuery = query.toLowerCase();
      
      return gifts.filter(gift =>
        gift.giftName.toLowerCase().includes(lowerQuery) ||
        gift.giftId.toLowerCase().includes(lowerQuery) ||
        gift.giftDesc.toLowerCase().includes(lowerQuery)
      );
    } catch (error) {
      await logger.error('BilibiliGiftService', '搜索礼物失败', error as Error, {
        roomId,
        query
      });
      throw error;
    }
  }

  /**
   * 手动添加礼物信息
   * @param gift 礼物信息
   */
  addGift(gift: GiftInfo): void {
    this.collectedGifts.set(gift.giftId, gift);
  }

  /**
   * 清除缓存
   * @param roomId 直播间ID（可选，不传则清除所有缓存）
   */
  clearCache(roomId?: string): void {
    if (roomId) {
      this.giftCache.delete(roomId);
    } else {
      this.giftCache.clear();
    }
  }

  /**
   * 获取缓存状态
   * @param roomId 直播间ID
   * @returns 缓存状态信息
   */
  getCacheStatus(roomId: string): { cached: boolean; age?: number } {
    const cached = this.giftCache.get(roomId);
    if (!cached) {
      return { cached: false };
    }

    return {
      cached: true,
      age: Date.now() - cached.timestamp
    };
  }

  /**
   * 清理资源
   */
  destroy(): void {
    if (this.giftEventListener) {
      this.api.off('gift-event', this.giftEventListener);
      this.giftEventListener = null;
    }
  }
}

// 创建单例实例
export const bilibiliGiftService = new BilibiliGiftService();