/**
 * 自动禁言与刷屏防护状态管理
 */

import { defineStore } from 'pinia';
import { ref, computed } from 'vue';
import type { ModerationConfig, MuteRecord } from '../modules/moderation/types.js';
import { DEFAULT_MODERATION_CONFIG } from '../modules/moderation/types.js';
import { getModerationService } from '../services/moderation-service.js';

export const useModerationStore = defineStore('moderation', () => {
  // State
  const config = ref<ModerationConfig>({ ...DEFAULT_MODERATION_CONFIG });
  const muteRecords = ref<MuteRecord[]>([]);
  const isEnabled = ref<boolean>(false);
  const lastMuteRecord = ref<MuteRecord | null>(null);

  // Getters
  const activeMuteCount = computed(() => {
    const now = Date.now();
    return muteRecords.value.filter(r => {
      if (r.unmutedAt) return false;
      if (r.duration === 0) return true;
      const muteEnd = r.mutedAt.getTime() + r.duration * 60 * 1000;
      return now < muteEnd;
    }).length;
  });

  // Actions
  function updateConfig(updates: Partial<ModerationConfig>): void {
    config.value = { ...config.value, ...updates };
    const moderationService = getModerationService();
    moderationService.updateConfig(updates);
    saveConfig();
  }

  function toggleEnabled(): void {
    updateConfig({ enabled: !config.value.enabled });
    isEnabled.value = config.value.enabled;
  }

  function toggleAutoMute(): void {
    updateConfig({ autoMuteEnabled: !config.value.autoMuteEnabled });
  }

  function toggleFloodProtection(): void {
    updateConfig({ floodProtectionEnabled: !config.value.floodProtectionEnabled });
  }

  function addProhibitedWord(word: string): void {
    const moderationService = getModerationService();
    moderationService.addProhibitedWord(word);
    config.value.prohibitedWords = moderationService.getProhibitedWords();
    saveConfig();
  }

  function removeProhibitedWord(word: string): void {
    const moderationService = getModerationService();
    moderationService.removeProhibitedWord(word);
    config.value.prohibitedWords = moderationService.getProhibitedWords();
    saveConfig();
  }

  function fetchMuteRecords(roomId?: string): void {
    const moderationService = getModerationService();
    muteRecords.value = moderationService.getMuteRecords(roomId);
  }

  function unmuteUser(userId: string): void {
    const moderationService = getModerationService();
    moderationService.unmute(userId);
    fetchMuteRecords();
  }

  function clearRecords(): void {
    muteRecords.value = [];
  }

  function saveConfig(): void {
    try {
      localStorage.setItem('moderation-config', JSON.stringify(config.value));
    } catch (error) {
      console.error('Failed to save moderation config:', error);
    }
  }

  function loadConfig(): void {
    try {
      const saved = localStorage.getItem('moderation-config');
      if (saved) {
        const parsed = JSON.parse(saved);
        config.value = { ...DEFAULT_MODERATION_CONFIG, ...parsed };
        isEnabled.value = config.value.enabled;
      }
    } catch (error) {
      console.error('Failed to load moderation config:', error);
    }
  }

  // 初始化
  loadConfig();

  return {
    // State
    config,
    muteRecords,
    isEnabled,
    lastMuteRecord,

    // Getters
    activeMuteCount,

    // Actions
    updateConfig,
    toggleEnabled,
    toggleAutoMute,
    toggleFloodProtection,
    addProhibitedWord,
    removeProhibitedWord,
    fetchMuteRecords,
    unmuteUser,
    clearRecords,
    saveConfig,
    loadConfig
  };
});
