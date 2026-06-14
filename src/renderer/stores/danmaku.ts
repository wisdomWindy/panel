/**
 * 弹幕状态管理
 */

import { defineStore } from 'pinia';
import { ref, computed } from 'vue';
import { Danmaku, Gift, DanmakuPoolItem } from '../modules/danmaku/types';

export const useDanmakuStore = defineStore('danmaku', () => {
  // State
  const danmakuList = ref<Danmaku[]>([]);
  const giftList = ref<Gift[]>([]);
  const danmakuPool = ref<DanmakuPoolItem[]>([]);
  const isAutoSending = ref<boolean>(false);
  const autoSendInterval = ref<number>(5000);
  const lastSentTime = ref<number>(0);
  const failedCount = ref<number>(0);
  const sentTodayCount = ref<number>(0);
  const thankSentCount = ref<number>(0);
  const sendSuccessRate = ref<number>(100);

  // Getters
  const danmakuCount = computed(() => danmakuList.value.length);
  const giftCount = computed(() => giftList.value.length);
  const poolCount = computed(() => danmakuPool.value.length);
  const enabledPoolCount = computed(() => danmakuPool.value.filter(item => item.enabled).length);
  const recentDanmaku = computed(() => danmakuList.value.slice(-100));
  const recentGifts = computed(() => giftList.value.slice(-50));

  // Actions
  function addDanmaku(danmaku: Danmaku) {
    danmakuList.value.push(danmaku);
    // 限制列表长度
    if (danmakuList.value.length > 1000) {
      danmakuList.value = danmakuList.value.slice(-500);
    }
  }

  function addGift(gift: Gift) {
    giftList.value.push(gift);
    // 限制列表长度
    if (giftList.value.length > 500) {
      giftList.value = giftList.value.slice(-250);
    }
  }

  function clearDanmaku() {
    danmakuList.value = [];
  }

  function clearGifts() {
    giftList.value = [];
  }

  function clearAll() {
    danmakuList.value = [];
    giftList.value = [];
  }

  // 弹幕池操作
  function addPoolItem(item: Partial<DanmakuPoolItem>): DanmakuPoolItem {
    const newItem: DanmakuPoolItem = {
      id: Date.now(),
      content: item.content || '',
      type: item.type || 'timed',
      triggerWord: item.triggerWord,
      enabled: item.enabled ?? true,
      priority: item.priority || 0,
      interval: item.interval,
      createdAt: new Date(),
      updatedAt: new Date()
    };

    danmakuPool.value.push(newItem);
    return newItem;
  }

  function updatePoolItem(id: number, updates: Partial<DanmakuPoolItem>) {
    const index = danmakuPool.value.findIndex(item => item.id === id);
    if (index !== -1) {
      danmakuPool.value[index] = {
        ...danmakuPool.value[index],
        ...updates,
        updatedAt: new Date()
      };
    }
  }

  function removePoolItem(id: number) {
    danmakuPool.value = danmakuPool.value.filter(item => item.id !== id);
  }

  function enablePoolItem(id: number) {
    updatePoolItem(id, { enabled: true });
  }

  function disablePoolItem(id: number) {
    updatePoolItem(id, { enabled: false });
  }

  function clearPool() {
    danmakuPool.value = [];
  }

  // 自动发送
  function setAutoSending(sending: boolean) {
    isAutoSending.value = sending;
  }

  function setAutoSendInterval(interval: number) {
    autoSendInterval.value = interval;
  }

  function recordSentTime() {
    lastSentTime.value = Date.now();
  }

  function incrementSentCount() {
    sentTodayCount.value++;
    const total = sentTodayCount.value + failedCount.value;
    sendSuccessRate.value = total > 0 ? Math.round((sentTodayCount.value / total) * 100) : 100;
  }

  function incrementThankCount() {
    thankSentCount.value++;
  }

  function incrementFailedCount() {
    failedCount.value++;
    const total = sentTodayCount.value + failedCount.value;
    sendSuccessRate.value = total > 0 ? Math.round((sentTodayCount.value / total) * 100) : 100;
  }

  function resetFailedCount() {
    failedCount.value = 0;
    sendSuccessRate.value = 100;
  }

  // 过滤弹幕
  function filterDanmaku(filter: {
    userType?: string[];
    minLevel?: number;
    keywords?: string[];
  }): Danmaku[] {
    return danmakuList.value.filter(danmaku => {
      if (filter.userType && !filter.userType.includes(danmaku.userType)) {
        return false;
      }

      if (filter.minLevel && danmaku.badge && danmaku.badge.level < filter.minLevel) {
        return false;
      }

      if (filter.keywords && filter.keywords.length > 0) {
        const hasKeyword = filter.keywords.some(keyword => danmaku.content.includes(keyword));
        if (!hasKeyword) {
          return false;
        }
      }

      return true;
    });
  }

  // 统计信息
  function getStatistics() {
    const userTypeStats = {
      normal: 0,
      captain: 0,
      admin: 0,
      owner: 0
    };

    const hourlyStats: Record<number, number> = {};

    danmakuList.value.forEach(danmaku => {
      userTypeStats[danmaku.userType]++;

      const hour = new Date(danmaku.timestamp).getHours();
      hourlyStats[hour] = (hourlyStats[hour] || 0) + 1;
    });

    return {
      total: danmakuList.value.length,
      giftTotal: giftList.value.length,
      byUserType: userTypeStats,
      byHour: hourlyStats
    };
  }

  function exportState() {
    return {
      danmakuList: danmakuList.value,
      giftList: giftList.value,
      danmakuPool: danmakuPool.value,
      isAutoSending: isAutoSending.value,
      autoSendInterval: autoSendInterval.value,
      lastSentTime: lastSentTime.value,
      failedCount: failedCount.value,
      sentTodayCount: sentTodayCount.value,
      thankSentCount: thankSentCount.value,
      sendSuccessRate: sendSuccessRate.value
    };
  }

  function importState(state: ReturnType<typeof exportState>) {
    danmakuList.value = state.danmakuList;
    giftList.value = state.giftList;
    danmakuPool.value = state.danmakuPool;
    isAutoSending.value = state.isAutoSending;
    autoSendInterval.value = state.autoSendInterval;
    lastSentTime.value = state.lastSentTime;
    failedCount.value = state.failedCount;
    sentTodayCount.value = state.sentTodayCount;
    thankSentCount.value = state.thankSentCount;
    sendSuccessRate.value = state.sendSuccessRate;
  }

  return {
    // State
    danmakuList,
    giftList,
    danmakuPool,
    isAutoSending,
    autoSendInterval,
    lastSentTime,
    failedCount,
    sentTodayCount,
    thankSentCount,
    sendSuccessRate,

    // Getters
    danmakuCount,
    giftCount,
    poolCount,
    enabledPoolCount,
    recentDanmaku,
    recentGifts,

    // Actions
    addDanmaku,
    addGift,
    clearDanmaku,
    clearGifts,
    clearAll,
    addPoolItem,
    updatePoolItem,
    removePoolItem,
    enablePoolItem,
    disablePoolItem,
    clearPool,
    setAutoSending,
    setAutoSendInterval,
    recordSentTime,
    incrementSentCount,
    incrementThankCount,
    incrementFailedCount,
    resetFailedCount,
    filterDanmaku,
    getStatistics,
    exportState,
    importState
  };
});
