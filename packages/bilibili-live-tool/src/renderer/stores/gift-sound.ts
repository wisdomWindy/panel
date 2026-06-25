/**
 * 礼物音效状态管理
 */

import { defineStore } from 'pinia';
import { ref, computed } from 'vue';
import { giftSoundService } from '../services/gift-sound-service';
import type {
  GiftSoundConfig,
  SoundLog,
  SoundPlaybackState,
  SoundQueueItem,
  GiftSoundExportData
} from '../modules/gift-sound/types';

export const useGiftSoundStore = defineStore('giftSound', () => {
  // State
  const giftSounds = ref<GiftSoundConfig[]>([]);
  const soundLogs = ref<SoundLog[]>([]);
  const playbackState = ref<SoundPlaybackState>({
    isPlaying: false,
    currentSound: null,
    volume: 0.8,
    progress: 0,
    duration: 0
  });
  const queue = ref<SoundQueueItem[]>([]);
  const loading = ref(false);
  const error = ref<string | null>(null);

  // Getters
  const enabledGiftSounds = computed(() =>
    giftSounds.value.filter(gs => gs.enabled)
  );

  const totalGiftSounds = computed(() => giftSounds.value.length);

  const totalEnabledGiftSounds = computed(() => enabledGiftSounds.value.length);

  const recentSoundLogs = computed(() =>
    soundLogs.value
      .sort((a, b) => new Date(b.playedAt).getTime() - new Date(a.playedAt).getTime())
      .slice(0, 50)
  );

  const isPlaying = computed(() => playbackState.value.isPlaying);

  const currentPlayingSound = computed(() => playbackState.value.currentSound);

  const queueLength = computed(() => queue.value.length);

  // Actions
  async function loadGiftSounds() {
    loading.value = true;
    error.value = null;
    try {
      giftSounds.value = await giftSoundService.getAllGiftSounds();
    } catch (err) {
      error.value = (err as Error).message;
      console.error('加载礼物音效配置失败:', err);
    } finally {
      loading.value = false;
    }
  }

  async function loadSoundLogs() {
    try {
      soundLogs.value = await giftSoundService.getSoundLogs();
    } catch (err) {
      console.error('加载播放日志失败:', err);
    }
  }

  async function addGiftSound(giftSound: GiftSoundConfig) {
    try {
      await giftSoundService.addGiftSound(giftSound);
      await loadGiftSounds();
    } catch (err) {
      error.value = (err as Error).message;
      throw err;
    }
  }

  async function updateGiftSound(id: string, updates: Partial<GiftSoundConfig>) {
    try {
      await giftSoundService.updateGiftSound(id, updates);
      await loadGiftSounds();
    } catch (err) {
      error.value = (err as Error).message;
      throw err;
    }
  }

  async function deleteGiftSound(id: string) {
    try {
      await giftSoundService.deleteGiftSound(id);
      await loadGiftSounds();
    } catch (err) {
      error.value = (err as Error).message;
      throw err;
    }
  }

  async function toggleGiftSoundEnabled(id: string) {
    const giftSound = giftSounds.value.find(gs => gs.id === id);
    if (giftSound) {
      await updateGiftSound(id, { enabled: !giftSound.enabled });
    }
  }

  async function playGiftSound(giftId: string, giftName: string, username: string, count: number) {
    try {
      const result = await giftSoundService.playGiftSound({
        giftId,
        giftName,
        username,
        count
      });

      if (result.success) {
        // 更新播放状态
        playbackState.value.isPlaying = true;
        playbackState.value.currentSound = result.soundFile || null;

        // 重新加载日志
        await loadSoundLogs();
      }

      return result;
    } catch (err) {
      error.value = (err as Error).message;
      return { success: false, error: (err as Error).message };
    }
  }

  async function stopAllSounds() {
    try {
      await giftSoundService.stopAllSounds();
      playbackState.value.isPlaying = false;
      playbackState.value.currentSound = null;
    } catch (err) {
      error.value = (err as Error).message;
    }
  }

  async function previewSound(soundFile: string, volume: number) {
    try {
      await giftSoundService.previewSound(soundFile, volume);
      playbackState.value.isPlaying = true;
      playbackState.value.currentSound = soundFile;
      playbackState.value.volume = volume;
    } catch (err) {
      error.value = (err as Error).message;
      throw err;
    }
  }

  async function importGiftSounds(configs: GiftSoundConfig[]) {
    try {
      await giftSoundService.importGiftSounds(configs);
      await loadGiftSounds();
    } catch (err) {
      error.value = (err as Error).message;
      throw err;
    }
  }

  async function exportGiftSounds(): Promise<GiftSoundExportData> {
    try {
      return await giftSoundService.exportGiftSounds();
    } catch (err) {
      error.value = (err as Error).message;
      throw err;
    }
  }

  async function clearSoundLogs() {
    try {
      await giftSoundService.clearSoundLogs();
      soundLogs.value = [];
    } catch (err) {
      error.value = (err as Error).message;
    }
  }

  function addToQueue(item: SoundQueueItem) {
    queue.value.push(item);
    // 按优先级排序
    queue.value.sort((a, b) => b.priority - a.priority);
  }

  function removeFromQueue(id: string) {
    queue.value = queue.value.filter(item => item.id !== id);
  }

  function clearQueue() {
    queue.value = [];
  }

  function updatePlaybackState(state: Partial<SoundPlaybackState>) {
    playbackState.value = {
      ...playbackState.value,
      ...state
    };
  }

  function clearError() {
    error.value = null;
  }

  // 初始化
  async function initialize() {
    await loadGiftSounds();
    await loadSoundLogs();
  }

  return {
    // State
    giftSounds,
    soundLogs,
    playbackState,
    queue,
    loading,
    error,

    // Getters
    enabledGiftSounds,
    totalGiftSounds,
    totalEnabledGiftSounds,
    recentSoundLogs,
    isPlaying,
    currentPlayingSound,
    queueLength,

    // Actions
    loadGiftSounds,
    loadSoundLogs,
    addGiftSound,
    updateGiftSound,
    deleteGiftSound,
    toggleGiftSoundEnabled,
    playGiftSound,
    stopAllSounds,
    previewSound,
    importGiftSounds,
    exportGiftSounds,
    clearSoundLogs,
    addToQueue,
    removeFromQueue,
    clearQueue,
    updatePlaybackState,
    clearError,
    initialize
  };
});
