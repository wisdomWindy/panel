/**
 * 弹幕状态管理
 */

import { defineStore } from 'pinia';
import { ref, computed } from 'vue';
import { Danmaku, Gift } from '../modules/danmaku/types';
import { GiftRankItem, DanmakuRankItem, CaptainRankItem } from '../modules/recording/types';

export const useDanmakuStore = defineStore('danmaku', () => {
  // State
  const danmakuList = ref<Danmaku[]>([]);
  const giftList = ref<Gift[]>([]);

  // Getters
  const danmakuCount = computed(() => danmakuList.value.length);
  const giftCount = computed(() => giftList.value.length);
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
    };
  }

  function importState(state: ReturnType<typeof exportState>) {
    danmakuList.value = state.danmakuList;
    giftList.value = state.giftList;
  }

  // 排行榜数据聚合函数

  // 获取送礼榜
  function getGiftRank(limit: number = 20): GiftRankItem[] {
    const userGifts: Record<string, { userId: string; username: string; totalValue: number }> = {};

    for (const gift of giftList.value) {
      if (!userGifts[gift.userId]) {
        userGifts[gift.userId] = {
          userId: gift.userId,
          username: gift.username,
          totalValue: 0
        };
      }
      userGifts[gift.userId].totalValue += gift.price * gift.count;
    }

    return Object.values(userGifts)
      .sort((a, b) => b.totalValue - a.totalValue)
      .slice(0, limit)
      .map((item, index) => ({
        rank: index + 1,
        userId: item.userId,
        username: item.username,
        totalValue: item.totalValue
      }));
  }

  // 获取弹幕榜
  function getDanmakuRank(limit: number = 20): DanmakuRankItem[] {
    const userDanmaku: Record<string, { userId: string; username: string; count: number }> = {};

    for (const danmaku of danmakuList.value) {
      if (!userDanmaku[danmaku.userId]) {
        userDanmaku[danmaku.userId] = {
          userId: danmaku.userId,
          username: danmaku.username,
          count: 0
        };
      }
      userDanmaku[danmaku.userId].count++;
    }

    return Object.values(userDanmaku)
      .sort((a, b) => b.count - a.count)
      .slice(0, limit)
      .map((item, index) => ({
        rank: index + 1,
        userId: item.userId,
        username: item.username,
        count: item.count
      }));
  }

  // 获取舰长贡献榜
  function getCaptainRank(): CaptainRankItem[] {
    const captainList = giftList.value.filter(gift => gift.giftName.includes('舰长') || gift.giftId === 'captain');

    return captainList
      .sort((a, b) => {
        // 按舰长等级降序，同等级按时间升序
        const levelA = getCaptainLevel(a.giftName);
        const levelB = getCaptainLevel(b.giftName);
        if (levelA !== levelB) {
          return levelB - levelA;
        }
        return a.timestamp - b.timestamp;
      })
      .map((item, index) => ({
        rank: index + 1,
        userId: item.userId,
        username: item.username,
        level: getCaptainLevel(item.giftName),
        levelName: getCaptainLevelName(getCaptainLevel(item.giftName)),
        timestamp: item.timestamp
      }));
  }

  // 辅助函数：获取舰长等级
  function getCaptainLevel(giftName: string): number {
    if (giftName.includes('总督')) return 3;
    if (giftName.includes('提督')) return 2;
    return 1; // 默认为督察
  }

  // 辅助函数：获取舰长等级名称
  function getCaptainLevelName(level: number): string {
    const levelNames: Record<number, string> = {
      1: '督察',
      2: '提督',
      3: '总督'
    };
    return levelNames[level] || '督察';
  }

  return {
    // State
    danmakuList,
    giftList,

    // Getters
    danmakuCount,
    giftCount,
    recentDanmaku,
    recentGifts,

    // Actions
    addDanmaku,
    addGift,
    clearDanmaku,
    clearGifts,
    clearAll,
    filterDanmaku,
    getStatistics,
    exportState,
    importState,
    getGiftRank,
    getDanmakuRank,
    getCaptainRank
  };
});
