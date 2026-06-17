/**
 * ASR 状态管理
 * 管理语音识别相关的状态和操作
 */

import { defineStore } from 'pinia';
import { ref, computed } from 'vue';
import type { ASRConfig, ASRResult, ASRStatus, ASRError } from '../modules/asr/types.js';
import { DEFAULT_ASR_CONFIG } from '../modules/asr/types.js';
import { getASRService } from '../services/asr-service.js';

export const useASRStore = defineStore('asr', () => {
  // State
  const config = ref<ASRConfig>({ ...DEFAULT_ASR_CONFIG });
  const status = ref<ASRStatus>('idle');
  const isAvailable = ref<boolean>(false);
  const recentResults = ref<ASRResult[]>([]);
  const lastError = ref<ASRError | null>(null);

  // Getters
  const isEnabled = computed(() => config.value.enabled);
  const isListening = computed(() => status.value === 'listening');
  const isProcessing = computed(() => status.value === 'processing');

  // Actions
  function updateConfig(updates: Partial<ASRConfig>): void {
    config.value = { ...config.value, ...updates };
    const asrService = getASRService();
    asrService.updateConfig(updates);
    saveConfig();
  }

  function toggleEnabled(): void {
    updateConfig({ enabled: !config.value.enabled });
  }

  async function startRecognition(): Promise<void> {
    try {
      const asrService = getASRService();
      asrService.updateConfig(config.value);

      // 设置监听器
      asrService.onResult((result) => {
        addResult(result);
      });

      asrService.onStatusChange((newStatus) => {
        status.value = newStatus;
      });

      asrService.onError((error) => {
        lastError.value = error;
      });

      await asrService.start();
    } catch (error) {
      lastError.value = {
        code: 'start-failed',
        message: (error as Error).message
      };
      throw error;
    }
  }

  function stopRecognition(): void {
    const asrService = getASRService();
    asrService.stop();
    status.value = 'idle';
  }

  function pauseRecognition(): void {
    const asrService = getASRService();
    asrService.pause();
  }

  function resumeRecognition(): void {
    const asrService = getASRService();
    asrService.resume();
  }

  function addResult(result: ASRResult): void {
    recentResults.value.unshift(result);
    // 只保留最近 100 条结果
    if (recentResults.value.length > 100) {
      recentResults.value = recentResults.value.slice(0, 100);
    }
  }

  function clearResults(): void {
    recentResults.value = [];
  }

  function clearError(): void {
    lastError.value = null;
  }

  async function checkAvailability(): Promise<void> {
    const asrService = getASRService();
    isAvailable.value = asrService.isAvailable();

    // 检查 whisper.cpp 可用性
    if (config.value.engine !== 'web-speech') {
      const result = await asrService.checkWhisperAvailability();
      isAvailable.value = result.available;
    }
  }

  function saveConfig(): void {
    try {
      localStorage.setItem('asr-config', JSON.stringify(config.value));
    } catch (error) {
      console.error('Failed to save ASR config:', error);
    }
  }

  function loadConfig(): void {
    try {
      const saved = localStorage.getItem('asr-config');
      if (saved) {
        const parsed = JSON.parse(saved);
        config.value = { ...DEFAULT_ASR_CONFIG, ...parsed };
      }
    } catch (error) {
      console.error('Failed to load ASR config:', error);
    }
  }

  // 初始化
  loadConfig();

  return {
    // State
    config,
    status,
    isAvailable,
    recentResults,
    lastError,

    // Getters
    isEnabled,
    isListening,
    isProcessing,

    // Actions
    updateConfig,
    toggleEnabled,
    startRecognition,
    stopRecognition,
    pauseRecognition,
    resumeRecognition,
    addResult,
    clearResults,
    clearError,
    checkAvailability,
    saveConfig,
    loadConfig
  };
});
