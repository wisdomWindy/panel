/**
 * TTS 状态管理
 * 管理 TTS 相关的状态和操作
 */

import { defineStore } from 'pinia';
import { ref, computed } from 'vue';
import type { TTSConfig, TTSStatus, BroadcastMessage, TTSEngineInfo } from '../modules/tts/types.js';
import { DEFAULT_TTS_CONFIG } from '../modules/tts/types.js';
import { getTTSService } from '../services/tts-service.js';

export const useTTSStore = defineStore('tts', () => {
  // State
  const config = ref<TTSConfig>({ ...DEFAULT_TTS_CONFIG });
  const status = ref<TTSStatus>('idle');
  const queueLength = ref<number>(0);
  const recentMessages = ref<BroadcastMessage[]>([]);
  const voices = ref<TTSEngineInfo[]>([]);
  const chineseVoices = ref<TTSEngineInfo[]>([]);

  // Getters
  const isEnabled = computed(() => config.value.enabled);
  const isSpeaking = computed(() => status.value === 'speaking');
  const isPaused = computed(() => status.value === 'paused');

  // Actions
  function updateConfig(updates: Partial<TTSConfig>): void {
    config.value = { ...config.value, ...updates };
    const ttsService = getTTSService();
    ttsService.updateConfig(updates);
    saveConfig();
  }

  function toggleEnabled(): void {
    updateConfig({ enabled: !config.value.enabled });
  }

  function startBroadcast(): void {
    const ttsService = getTTSService();
    ttsService.updateConfig(config.value);
    // 设置消息监听器
    ttsService.onMessage((message) => {
      addRecentMessage(message);
    });
    // 设置状态监听器
    ttsService.onStatusChange((newStatus) => {
      status.value = newStatus;
      queueLength.value = ttsService.getQueueLength();
    });
  }

  function stopBroadcast(): void {
    const ttsService = getTTSService();
    ttsService.stop();
    queueLength.value = 0;
  }

  function pauseBroadcast(): void {
    const ttsService = getTTSService();
    ttsService.pause();
  }

  function resumeBroadcast(): void {
    const ttsService = getTTSService();
    ttsService.resume();
  }

  function addRecentMessage(message: BroadcastMessage): void {
    recentMessages.value.unshift(message);
    // 只保留最近 50 条消息
    if (recentMessages.value.length > 50) {
      recentMessages.value = recentMessages.value.slice(0, 50);
    }
  }

  function clearRecentMessages(): void {
    recentMessages.value = [];
  }

  function loadVoices(): void {
    const ttsService = getTTSService();
    voices.value = ttsService.getVoices();
    chineseVoices.value = ttsService.getChineseVoices();
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

  // 初始化
  loadConfig();

  return {
    // State
    config,
    status,
    queueLength,
    recentMessages,
    voices,
    chineseVoices,

    // Getters
    isEnabled,
    isSpeaking,
    isPaused,

    // Actions
    updateConfig,
    toggleEnabled,
    startBroadcast,
    stopBroadcast,
    pauseBroadcast,
    resumeBroadcast,
    addRecentMessage,
    clearRecentMessages,
    loadVoices,
    saveConfig,
    loadConfig
  };
});
