/**
 * 录制状态管理
 * 管理录制相关的状态和操作
 */

import { defineStore } from 'pinia';
import { ref, computed } from 'vue';
import type {
  Recording,
  Highlight,
  RecordingConfig,
  RecordingSession,
  RecordingStatus,
  RecordingOptions
} from '../modules/recording/types.js';

export const useRecordingStore = defineStore('recording', () => {
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

      const result = await window.electronAPI.invoke('recording:start', options);

      if (result.success) {
        currentSession.value = result.session;
        status.value = 'recording';
        duration.value = 0;
        fileSize.value = 0;
        highlightCount.value = 0;
      } else {
        error.value = result.error;
        throw new Error(result.error);
      }
    } catch (err) {
      error.value = (err as Error).message;
      throw err;
    } finally {
      isLoading.value = false;
    }
  }

  async function pauseRecording(): Promise<void> {
    try {
      if (!currentSession.value) return;

      const result = await window.electronAPI.invoke('recording:pause', currentSession.value.id);

      if (result.success) {
        status.value = 'paused';
      } else {
        error.value = result.error;
        throw new Error(result.error);
      }
    } catch (err) {
      error.value = (err as Error).message;
      throw err;
    }
  }

  async function resumeRecording(): Promise<void> {
    try {
      if (!currentSession.value) return;

      const result = await window.electronAPI.invoke('recording:resume', currentSession.value.id);

      if (result.success) {
        status.value = 'recording';
      } else {
        error.value = result.error;
        throw new Error(result.error);
      }
    } catch (err) {
      error.value = (err as Error).message;
      throw err;
    }
  }

  async function stopRecording(): Promise<void> {
    try {
      if (!currentSession.value) return;

      const result = await window.electronAPI.invoke('recording:stop', currentSession.value.id);

      if (result.success) {
        currentSession.value = null;
        status.value = 'idle';
        duration.value = 0;
        fileSize.value = 0;
        highlightCount.value = 0;

        // 刷新录制列表
        await fetchRecordings();
      } else {
        error.value = result.error;
        throw new Error(result.error);
      }
    } catch (err) {
      error.value = (err as Error).message;
      throw err;
    }
  }

  async function markHighlight(reason: string, description?: string): Promise<void> {
    try {
      if (!currentSession.value) return;

      const result = await window.electronAPI.invoke(
        'recording:markHighlight',
        currentSession.value.id,
        reason,
        description
      );

      if (result.success) {
        highlightCount.value++;
      } else {
        error.value = result.error;
        throw new Error(result.error);
      }
    } catch (err) {
      error.value = (err as Error).message;
      throw err;
    }
  }

  async function fetchRecordings(): Promise<void> {
    try {
      isLoading.value = true;

        const result = await window.electronAPI.invoke('recording:getRecordings');

      if (result.success) {
        recordings.value = result.recordings;
      } else {
        error.value = result.error;
      }
    } catch (err) {
      error.value = (err as Error).message;
    } finally {
      isLoading.value = false;
    }
  }

  async function fetchHighlights(recordingId?: string): Promise<void> {
    try {
      isLoading.value = true;

        const result = await window.electronAPI.invoke('recording:getHighlights', recordingId);

      if (result.success) {
        highlights.value = result.highlights;
      } else {
        error.value = result.error;
      }
    } catch (err) {
      error.value = (err as Error).message;
    } finally {
      isLoading.value = false;
    }
  }

  async function deleteRecording(id: string): Promise<void> {
    try {
      const result = await window.electronAPI.invoke('recording:deleteRecording', id);

      if (result.success) {
        // 从列表中移除
        recordings.value = recordings.value.filter(r => r.id !== id);
      } else {
        error.value = result.error;
        throw new Error(result.error);
      }
    } catch (err) {
      error.value = (err as Error).message;
      throw err;
    }
  }

  async function deleteHighlight(id: string): Promise<void> {
    try {
      const result = await window.electronAPI.invoke('recording:deleteHighlight', id);

      if (result.success) {
        // 从列表中移除
        highlights.value = highlights.value.filter(h => h.id !== id);
      } else {
        error.value = result.error;
        throw new Error(result.error);
      }
    } catch (err) {
      error.value = (err as Error).message;
      throw err;
    }
  }

  async function openRecordingFolder(id: string): Promise<void> {
    try {
      await window.electronAPI.invoke('recording:openFolder', id);
    } catch (err) {
      error.value = (err as Error).message;
    }
  }

  async function updateConfig(newConfig: Partial<RecordingConfig>): Promise<void> {
    try {
      const result = await window.electronAPI.invoke('recording:setConfig', newConfig);

      if (result.success) {
        config.value = { ...config.value, ...newConfig };
      } else {
        error.value = result.error;
        throw new Error(result.error);
      }
    } catch (err) {
      error.value = (err as Error).message;
      throw err;
    }
  }

  async function fetchConfig(): Promise<void> {
    try {
      const result = await window.electronAPI.invoke('recording:getConfig');

      if (result.success) {
        config.value = result.config;
      } else {
        error.value = result.error;
      }
    } catch (err) {
      error.value = (err as Error).message;
    }
  }

  async function checkFFmpeg(): Promise<{ available: boolean; version: string }> {
    try {
      const result = await window.electronAPI.invoke('recording:checkFFmpeg');

      if (result.success) {
        return result.info;
      } else {
        error.value = result.error;
        return { available: false, version: '' };
      }
    } catch (err) {
      error.value = (err as Error).message;
      return { available: false, version: '' };
    }
  }

  // IPC 事件监听
  function setupIpcListeners(): void {
    // 监听状态更新
    window.electronAPI.on('recording:statusChanged', (data: { sessionId: string; status: string; currentDuration: number; currentSize: number; highlightCount: number }) => {
      if (currentSession.value && currentSession.value.id === data.sessionId) {
        status.value = data.status as any;
        duration.value = data.currentDuration;
        fileSize.value = data.currentSize;
        highlightCount.value = data.highlightCount;
      }
    });

    // 监听高光标记
    window.electronAPI.on('recording:highlightMarked', (highlight: Highlight) => {
      highlights.value.unshift(highlight);
      highlightCount.value++;
    });

    // 监听错误
    window.electronAPI.on('recording:error', (errorMessage: string) => {
      error.value = errorMessage;
      status.value = 'idle';
      currentSession.value = null;
    });
  }

  // 工具函数
  function formatDuration(seconds: number): string {
    const hours = Math.floor(seconds / 3600);
    const minutes = Math.floor((seconds % 3600) / 60);
    const secs = seconds % 60;
    return `${hours.toString().padStart(2, '0')}:${minutes.toString().padStart(2, '0')}:${secs.toString().padStart(2, '0')}`;
  }

  function formatFileSize(bytes: number): string {
    if (bytes === 0) return '0 B';
    const units = ['B', 'KB', 'MB', 'GB', 'TB'];
    const k = 1024;
    const i = Math.floor(Math.log(bytes) / Math.log(k));
    return parseFloat((bytes / Math.pow(k, i)).toFixed(2)) + ' ' + units[i];
  }

  // 初始化
  setupIpcListeners();

  return {
    // State
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

    // Getters
    isRecording,
    isPaused,
    isIdle,
    formattedDuration,
    formattedFileSize,

    // Actions
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
