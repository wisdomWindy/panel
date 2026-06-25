/**
 * ASR 状态管理
 * 使用 Strategy 模式，通过引擎接口调用不同的 ASR 引擎
 */

import { defineStore } from 'pinia';
import { ref, computed } from 'vue';
import type { ASRConfig, ASRResult, ASRStatus, ASRError } from '../modules/asr/types.js';
import { DEFAULT_ASR_CONFIG } from '../modules/asr/types.js';
import type { ASREngine } from '../engines/asr-engine.js';
import { createASREngine, type ASREngineType } from '../engines/engine-factory.js';
import { useAudioRecording } from '../composables/useAudioRecording.js';

export const useASRStore = defineStore('asr', () => {
  let engine: ASREngine;
  const audioRecording = useAudioRecording();

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
  const engineName = computed(() => engine?.name ?? 'unknown');

  // 初始化引擎
  function initEngine(): void {
    engine = createASREngine(config.value.engine as ASREngineType);

    engine.onResult((result) => addResult(result));
    engine.onError((error) => {
      lastError.value = { code: error.code, message: error.message, details: error.message };
      status.value = 'error';
    });
    engine.onEnd(() => {
      if (status.value === 'listening' && config.value.continuous) {
        engine.resume();
      } else {
        status.value = 'idle';
      }
    });
  }

  // Actions
  function updateConfig(updates: Partial<ASRConfig>): void {
    const oldEngine = config.value.engine;
    config.value = { ...config.value, ...updates };

    // 引擎类型变化时重新初始化
    if (updates.engine && updates.engine !== oldEngine) {
      initEngine();
    }

    saveConfig();
  }

  function toggleEnabled(): void {
    updateConfig({ enabled: !config.value.enabled });
  }

  async function startRecognition(): Promise<void> {
    try {
      const hasMic = await audioRecording.requestMicrophone();
      if (!hasMic) throw new Error('无法访问麦克风');

      await engine.start({
        language: config.value.language,
        continuous: config.value.continuous,
        interimResults: config.value.interimResults,
        maxAlternatives: config.value.maxAlternatives,
        model: config.value.model
      });

      status.value = 'listening';
    } catch (error) {
      lastError.value = {
        code: 'start-failed',
        message: (error as Error).message
      };
      throw error;
    }
  }

  function stopRecognition(): void {
    engine.stop().catch(console.error);
    audioRecording.releaseMicrophone();
    status.value = 'idle';
  }

  function pauseRecognition(): void {
    engine.pause();
    status.value = 'idle';
  }

  function resumeRecognition(): void {
    if (config.value.enabled && status.value !== 'listening') {
      startRecognition();
    }
  }

  function addResult(result: ASRResult): void {
    recentResults.value.unshift(result);
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
    isAvailable.value = await engine.isAvailable();
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

  loadConfig();
  initEngine();

  return {
    config,
    status,
    isAvailable,
    recentResults,
    lastError,
    engineName,
    isEnabled,
    isListening,
    isProcessing,
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
