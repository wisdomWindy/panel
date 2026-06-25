/**
 * TTS 状态管理
 * 使用 Strategy 模式，通过引擎接口调用不同的 TTS 引擎
 */

import { defineStore } from 'pinia';
import { ref, computed } from 'vue';
import type { TTSConfig, TTSStatus, BroadcastMessage } from '../modules/tts/types.js';
import { DEFAULT_TTS_CONFIG } from '../modules/tts/types.js';
import { getTTSService } from '../services/tts-service.js';
import { useBroadcastQueue } from '../composables/useBroadcastQueue.js';

export const useTTSStore = defineStore('tts', () => {
  const ttsService = getTTSService();
  const broadcastQueue = useBroadcastQueue();

  // State
  const config = ref<TTSConfig>({ ...DEFAULT_TTS_CONFIG });
  const status = ref<TTSStatus>('idle');
  const recentMessages = ref<BroadcastMessage[]>([]);

  // Getters
  const isEnabled = computed(() => config.value.enabled);
  const isSpeaking = computed(() => status.value === 'speaking');
  const isPaused = computed(() => status.value === 'paused');
  const queueLength = computed(() => broadcastQueue.getQueueLength());
  const engineName = computed(() => ttsService.getEngine().name);

  // Actions
  function updateConfig(updates: Partial<TTSConfig>): void {
    config.value = { ...config.value, ...updates };
    ttsService.updateConfig(updates);
    saveConfig();
  }

  function toggleEnabled(): void {
    updateConfig({ enabled: !config.value.enabled });
  }

  function handleDanmaku(username: string, content: string): void {
    if (!config.value.enabled) return;
    if (broadcastQueue.shouldFilter(content, config.value.filterKeywords)) return;

    const message: BroadcastMessage = {
      id: `danmaku_${Date.now()}_${Math.random().toString(36).substr(2, 9)}`,
      type: 'danmaku',
      username,
      content,
      timestamp: Date.now()
    };

    addMessage(message);
  }

  function handleGift(username: string, giftName: string, giftCount: number, giftValue: number): void {
    if (!config.value.enabled || !config.value.broadcastGifts) return;
    if (giftValue < config.value.giftValueThreshold) return;

    const message: BroadcastMessage = {
      id: `gift_${Date.now()}_${Math.random().toString(36).substr(2, 9)}`,
      type: 'gift',
      username,
      content: `${giftName} x${giftCount}`,
      timestamp: Date.now(),
      giftName,
      giftCount,
      giftValue
    };

    addMessage(message);
  }

  function handleCaptain(username: string, level: number): void {
    if (!config.value.enabled || !config.value.broadcastCaptain) return;

    const levelNames = ['舰长', '提督', '总督'];
    const levelName = levelNames[level - 1] || '舰长';

    const message: BroadcastMessage = {
      id: `captain_${Date.now()}_${Math.random().toString(36).substr(2, 9)}`,
      type: 'captain',
      username,
      content: levelName,
      timestamp: Date.now(),
      captainLevel: level
    };

    addMessage(message);
  }

  function addMessage(message: BroadcastMessage): void {
    broadcastQueue.addToQueue(message);
    addRecentMessage(message);

    if (!broadcastQueue.isProcessing.value) {
      status.value = 'speaking';
      broadcastQueue.processQueue(
        (text) => ttsService.speak(text),
        config.value
      ).then(() => {
        status.value = 'idle';
      });
    }
  }

  function stopBroadcast(): void {
    ttsService.stop().catch(console.error);
    broadcastQueue.clearQueue();
    status.value = 'idle';
  }

  function pauseBroadcast(): void {
    ttsService.pause();
    status.value = 'paused';
  }

  function resumeBroadcast(): void {
    ttsService.resume();
    status.value = 'speaking';
  }

  function addRecentMessage(message: BroadcastMessage): void {
    recentMessages.value.unshift(message);
    if (recentMessages.value.length > 50) {
      recentMessages.value = recentMessages.value.slice(0, 50);
    }
  }

  function clearRecentMessages(): void {
    recentMessages.value = [];
  }

  function saveConfig(): void {
    try {
      localStorage.setItem('tts-config', JSON.stringify(config.value));
    } catch (error) {
      console.error('Failed to save TTS config:', error);
    }
  }

  function loadConfig(): void {
    try {
      const saved = localStorage.getItem('tts-config');
      if (saved) {
        const parsed = JSON.parse(saved);
        config.value = { ...DEFAULT_TTS_CONFIG, ...parsed };
      }
    } catch (error) {
      console.error('Failed to load TTS config:', error);
    }
  }

  loadConfig();

  return {
    config,
    status,
    queueLength,
    recentMessages,
    engineName,
    isEnabled,
    isSpeaking,
    isPaused,
    updateConfig,
    toggleEnabled,
    handleDanmaku,
    handleGift,
    handleCaptain,
    stopBroadcast,
    pauseBroadcast,
    resumeBroadcast,
    addRecentMessage,
    clearRecentMessages,
    saveConfig,
    loadConfig
  };
});
