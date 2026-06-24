/**
 * ASR 状态管理
 * 管理语音识别相关的状态和操作
 */

import { defineStore } from 'pinia';
import { ref, computed } from 'vue';
import type { ASRConfig, ASRResult, ASRStatus, ASRError } from '../modules/asr/types.js';
import { DEFAULT_ASR_CONFIG } from '../modules/asr/types.js';
import { getASRService } from '../services/asr-service.js';
import { useWebSpeechRecognition } from '../composables/useWebSpeechRecognition.js';
import { useAudioRecording } from '../composables/useAudioRecording.js';

export const useASRStore = defineStore('asr', () => {
  const asrService = getASRService();
  const webSpeech = useWebSpeechRecognition();
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

  // Actions
  function updateConfig(updates: Partial<ASRConfig>): void {
    config.value = { ...config.value, ...updates };
    asrService.updateConfig(updates);
    saveConfig();
  }

  function toggleEnabled(): void {
    updateConfig({ enabled: !config.value.enabled });
  }

  async function startRecognition(): Promise<void> {
    try {
      asrService.updateConfig(config.value);

      if (config.value.engine === 'web-speech') {
        await startWebSpeechRecognition();
      } else {
        await startWhisperRecognition();
      }
    } catch (error) {
      lastError.value = {
        code: 'start-failed',
        message: (error as Error).message
      };
      throw error;
    }
  }

  async function startWebSpeechRecognition(): Promise<void> {
    webSpeech.init({
      continuous: config.value.continuous,
      interimResults: config.value.interimResults,
      maxAlternatives: config.value.maxAlternatives,
      language: config.value.language
    });

    webSpeech.onResult((result) => addResult(result));
    webSpeech.onError((error) => {
      lastError.value = { code: error.code, message: error.message, details: error.message };
      status.value = 'error';
    });
    webSpeech.onEnd(() => {
      if (status.value === 'listening' && config.value.continuous) {
        webSpeech.start();
      } else {
        status.value = 'idle';
      }
    });

    const hasMic = await audioRecording.requestMicrophone();
    if (!hasMic) throw new Error('无法访问麦克风');

    webSpeech.start();
    status.value = 'listening';
  }

  async function startWhisperRecognition(): Promise<void> {
    const hasMic = await audioRecording.requestMicrophone();
    if (!hasMic) throw new Error('无法访问麦克风');

    await asrService.start({
      engine: config.value.engine,
      model: config.value.model,
      language: config.value.language
    });

    status.value = 'listening';
    audioRecording.startRecording(async (blob) => {
      status.value = 'processing';
      const reader = new FileReader();
      reader.onloadend = async () => {
        const base64 = (reader.result as string).split(',')[1];
        try {
          const result = await asrService.processAudio(base64, config.value.language);
          addResult({
            text: result.text,
            confidence: result.confidence || 0.9,
            isFinal: true,
            timestamp: Date.now()
          });
        } catch (error) {
          console.error('Audio processing error:', error);
        }
        status.value = 'listening';
      };
      reader.readAsDataURL(blob);
    });
  }

  function stopRecognition(): void {
    if (config.value.engine === 'web-speech') {
      webSpeech.stop();
    } else {
      audioRecording.stopRecording();
      audioRecording.releaseMicrophone();
      asrService.stop().catch(console.error);
    }
    status.value = 'idle';
  }

  function pauseRecognition(): void {
    if (config.value.engine === 'web-speech') {
      webSpeech.stop();
    } else {
      audioRecording.pauseRecording();
    }
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
    if (config.value.engine === 'web-speech') {
      isAvailable.value = webSpeech.isAvailable.value;
    } else {
      const result = await asrService.checkAvailability();
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

  loadConfig();

  return {
    config,
    status,
    isAvailable,
    recentResults,
    lastError,
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
