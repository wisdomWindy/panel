/**
 * Bilibili礼物状态管理
 * 管理礼物列表的缓存和状态
 */

import { defineStore } from 'pinia';
import { ref, computed } from 'vue';
import { bilibiliGiftService } from '../services/bilibili-gift-service';
import type { GiftInfo } from '../modules/gift-sound/types';

export const useBilibiliGiftsStore = defineStore('bilibili-gifts', () => {
  // 状态
  const gifts = ref<GiftInfo[]>([]);
  const loading = ref(false);
  const error = ref<string | null>(null);
  const currentRoomId = ref<string | null>(null);
  const lastFetchTime = ref<Date | null>(null);

  // 计算属性
  const giftCount = computed(() => gifts.value.length);
  const hasGifts = computed(() => gifts.value.length > 0);
  const isLoading = computed(() => loading.value);
  const hasError = computed(() => error.value !== null);

  // 获取礼物列表
  async function fetchGifts(roomId: string, forceRefresh = false): Promise<void> {
    // 如果是同一个房间且不是强制刷新，且数据已存在，则不重复获取
    if (currentRoomId.value === roomId && !forceRefresh && gifts.value.length > 0) {
      return;
    }

    loading.value = true;
    error.value = null;

    try {
      const giftList = await bilibiliGiftService.getGiftList(roomId, forceRefresh);
      gifts.value = giftList;
      currentRoomId.value = roomId;
      lastFetchTime.value = new Date();
    } catch (err) {
      error.value = (err as Error).message;
      console.error('获取礼物列表失败:', err);
      throw err;
    } finally {
      loading.value = false;
    }
  }

  // 搜索礼物
  function searchGifts(query: string): GiftInfo[] {
    if (!query.trim()) {
      return gifts.value;
    }

    const lowerQuery = query.toLowerCase();
    return gifts.value.filter(gift =>
      gift.giftName.toLowerCase().includes(lowerQuery) ||
      gift.giftId.toLowerCase().includes(lowerQuery) ||
      gift.giftDesc.toLowerCase().includes(lowerQuery)
    );
  }

  // 根据ID获取礼物
  function getGiftById(giftId: string): GiftInfo | undefined {
    return gifts.value.find(gift => gift.giftId === giftId);
  }

  // 清除缓存
  function clearCache(): void {
    gifts.value = [];
    currentRoomId.value = null;
    lastFetchTime.value = null;
    error.value = null;
    bilibiliGiftService.clearCache();
  }

  // 重置状态
  function reset(): void {
    gifts.value = [];
    loading.value = false;
    error.value = null;
    currentRoomId.value = null;
    lastFetchTime.value = null;
  }

  // 获取缓存状态
  function getCacheStatus(): { cached: boolean; age?: number; roomId?: string } {
    if (!currentRoomId.value) {
      return { cached: false };
    }

    const status = bilibiliGiftService.getCacheStatus(currentRoomId.value);
    return {
      ...status,
      roomId: currentRoomId.value
    };
  }

  return {
    // 状态
    gifts,
    loading,
    error,
    currentRoomId,
    lastFetchTime,

    // 计算属性
    giftCount,
    hasGifts,
    isLoading,
    hasError,

    // 方法
    fetchGifts,
    searchGifts,
    getGiftById,
    clearCache,
    reset,
    getCacheStatus
  };
});