/**
 * 录制状态管理
 * 通过 Service 层调用 IPC，只负责状态管理
 */

import { defineStore } from 'pinia';
import { ref, computed } from 'vue';
import type {
  Recording,
  Highlight,
  RecordingConfig,
  RecordingSession,
  RecordingOptions
} from '../modules/recording/types.js';
import { getRecordingService } from '../services/recording-service.js';
import { formatDuration, formatFileSize } from '../utils/format.js';

export const useRecordingStore = defineStore('recording', () => {
  const service = getRecordingService();

  // State
  const currentSession = ref<RecordingSession | null>(null);
  const status = ref<'idle' | 'recording' | 'paused'>('idle');
  const duration = ref<number>(0);
  const fileSize = ref<number>(0);
  const highlightCount = ref<number>(0);
  const recordings = ref<Recording[]>([]);
  const highlights = ref<Highlight[]>([]);
  const isLoading = ref<boolean>(false);
  const config = ref<RecordingConfig>({
    enabled: false,
    autoStart: false,
    format: 'mp4',
    quality: 'original',
    segmentSize: 0,
    segmentDuration: 0,
    outputPath: '',
    maxStorageSize: 0,
    autoCleanup: false,
    cleanupDays: 30,
    highlightEnabled: true,
    highlightConfig: {
      danmakuThreshold: 30,
      danmakuWindow: 10,
      giftValueThreshold: 100,
      captainTrigger: true,
      clipDuration: 30,
      beforeSeconds: 15,
      afterSeconds: 15,
      cooldownSeconds: 10
    }
  });
  const error = ref<string | null>(null);

  // Getters
  const isRecording = computed(() => status.value === 'recording');
  const isPaused = computed(() => status.value === 'paused');
  const isIdle = computed(() => status.value === 'idle');
  const formattedDuration = computed(() => formatDuration(duration.value));
  const formattedFileSize = computed(() => formatFileSize(fileSize.value));

  // Actions
  async function startRecording(options: RecordingOptions): Promise<void> {
    try {
      isLoading.value = true;
      error.value = null;
      const session = await service.startRecording(options);
      currentSession.value = session;
      status.value = 'recording';
      duration.value = 0;
      fileSize.value = 0;
      highlightCount.value = 0;
    } catch (err) {
      error.value = (err as Error).message;
      throw err;
    } finally {
      isLoading.value = false;
    }
  }

  async function pauseRecording(): Promise<void> {
    if (!currentSession.value) return;
    try {
      await service.pauseRecording(currentSession.value.id);
      status.value = 'paused';
    } catch (err) {
      error.value = (err as Error).message;
      throw err;
    }
  }

  async function resumeRecording(): Promise<void> {
    if (!currentSession.value) return;
    try {
      await service.resumeRecording(currentSession.value.id);
      status.value = 'recording';
    } catch (err) {
      error.value = (err as Error).message;
      throw err;
    }
  }

  async function stopRecording(): Promise<void> {
    if (!currentSession.value) return;
    try {
      await service.stopRecording(currentSession.value.id);
      currentSession.value = null;
      status.value = 'idle';
      duration.value = 0;
      fileSize.value = 0;
      highlightCount.value = 0;
      await fetchRecordings();
    } catch (err) {
      error.value = (err as Error).message;
      throw err;
    }
  }

  async function markHighlight(reason: string, description?: string): Promise<void> {
    if (!currentSession.value) return;
    try {
      await service.markHighlight(currentSession.value.id, reason, description);
      highlightCount.value++;
    } catch (err) {
      error.value = (err as Error).message;
      throw err;
    }
  }

  async function fetchRecordings(): Promise<void> {
    try {
      isLoading.value = true;
      recordings.value = await service.getRecordings();
    } catch (err) {
      error.value = (err as Error).message;
    } finally {
      isLoading.value = false;
    }
  }

  async function fetchHighlights(recordingId?: string): Promise<void> {
    try {
      isLoading.value = true;
      highlights.value = await service.getHighlights(recordingId);
    } catch (err) {
      error.value = (err as Error).message;
    } finally {
      isLoading.value = false;
    }
  }

  async function deleteRecording(id: string): Promise<void> {
    try {
      await service.deleteRecording(id);
      recordings.value = recordings.value.filter(r => r.id !== id);
    } catch (err) {
      error.value = (err as Error).message;
      throw err;
    }
  }

  async function deleteHighlight(id: string): Promise<void> {
    try {
      await service.deleteHighlight(id);
      highlights.value = highlights.value.filter(h => h.id !== id);
    } catch (err) {
      error.value = (err as Error).message;
      throw err;
    }
  }

  async function openRecordingFolder(id: string): Promise<void> {
    try {
      await service.openRecordingFolder(id);
    } catch (err) {
      error.value = (err as Error).message;
    }
  }

  async function updateConfig(newConfig: Partial<RecordingConfig>): Promise<void> {
    try {
      await service.updateConfig(newConfig);
      config.value = { ...config.value, ...newConfig };
    } catch (err) {
      error.value = (err as Error).message;
      throw err;
    }
  }

  async function fetchConfig(): Promise<void> {
    try {
      config.value = await service.getConfig();
    } catch (err) {
      error.value = (err as Error).message;
    }
  }

  async function checkFFmpeg(): Promise<{ available: boolean; version: string }> {
    try {
      return await service.checkFFmpeg();
    } catch (err) {
      error.value = (err as Error).message;
      return { available: false, version: '' };
    }
  }

  // IPC 事件监听
  function setupIpcListeners(): void {
    window.electronAPI.on('recording:statusChanged', ((data: {
      sessionId: string;
      status: string;
      currentDuration: number;
      currentSize: number;
      highlightCount: number;
    }) => {
      if (currentSession.value && currentSession.value.id === data.sessionId) {
        status.value = data.status as any;
        duration.value = data.currentDuration;
        fileSize.value = data.currentSize;
        highlightCount.value = data.highlightCount;
      }
    }) as (...args: unknown[]) => void);

    window.electronAPI.on('recording:highlightMarked', ((highlight: Highlight) => {
      highlights.value.unshift(highlight);
      highlightCount.value++;
    }) as (...args: unknown[]) => void);

    window.electronAPI.on('recording:error', ((errorMessage: string) => {
      error.value = errorMessage;
      status.value = 'idle';
      currentSession.value = null;
    }) as (...args: unknown[]) => void);
  }

  setupIpcListeners();

  return {
    currentSession,
    status,
    duration,
    fileSize,
    highlightCount,
    recordings,
    highlights,
    isLoading,
    config,
    error,
    isRecording,
    isPaused,
    isIdle,
    formattedDuration,
    formattedFileSize,
    startRecording,
    pauseRecording,
    resumeRecording,
    stopRecording,
    markHighlight,
    fetchRecordings,
    fetchHighlights,
    deleteRecording,
    deleteHighlight,
    openRecordingFolder,
    updateConfig,
    fetchConfig,
    checkFFmpeg
  };
});
