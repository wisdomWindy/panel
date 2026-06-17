<script setup lang="ts">
import { ref, computed, onMounted, onUnmounted, watch } from 'vue';
import { ElMessage } from 'element-plus';
import { Play, VideoPause, Refresh, VolumeMute, Volume } from '@element-plus/icons-vue';
import { giftSoundService } from '../services/gift-sound-service';
import type { GiftSoundConfig, SoundPlaybackState } from '../modules/gift-sound/types';

// Props
const props = defineProps<{
  soundFile?: string;
  volume?: number;
  autoPlay?: boolean;
}>();

// Emits
const emit = defineEmits<{
  (e: 'play', soundFile: string): void;
  (e: 'pause'): void;
  (e: 'stop'): void;
  (e: 'error', error: string): void;
}>();

// 响应式数据
const isPlaying = ref(false);
const currentSoundFile = ref(props.soundFile || '');
const currentVolume = ref(props.volume || 0.8);
const progress = ref(0);
const duration = ref(0);
const isMuted = ref(false);
const previousVolume = ref(0.8);
const playbackState = ref<SoundPlaybackState>({
  isPlaying: false,
  currentSound: null,
  volume: 0.8,
  progress: 0,
  duration: 0
});

// 计算属性
const progressPercent = computed(() => {
  if (duration.value === 0) return 0;
  return (progress.value / duration.value) * 100;
});

const formattedProgress = computed(() => {
  return formatTime(progress.value);
});

const formattedDuration = computed(() => {
  return formatTime(duration.value);
});

const volumeIcon = computed(() => {
  if (isMuted.value || currentVolume.value === 0) {
    return VolumeMute;
  }
  return Volume;
});

const volumePercent = computed(() => {
  return Math.round(currentVolume.value * 100);
});

// 生命周期
onMounted(() => {
  if (props.soundFile) {
    currentSoundFile.value = props.soundFile;
  }
  if (props.volume !== undefined) {
    currentVolume.value = props.volume;
  }
});

// 监听props变化
watch(() => props.soundFile, (newSoundFile) => {
  if (newSoundFile) {
    currentSoundFile.value = newSoundFile;
  }
});

watch(() => props.volume, (newVolume) => {
  if (newVolume !== undefined) {
    currentVolume.value = newVolume;
  }
});

// 方法
async function handlePlay() {
  if (!currentSoundFile.value) {
    ElMessage.warning('请先选择音效文件');
    return;
  }

  try {
    isPlaying.value = true;
    await giftSoundService.previewSound(currentSoundFile.value, currentVolume.value);
    emit('play', currentSoundFile.value);
    
    // 模拟播放进度（实际应该通过事件监听）
    simulatePlayback();
  } catch (error) {
    isPlaying.value = false;
    ElMessage.error('播放失败');
    emit('error', (error as Error).message);
  }
}

function handlePause() {
  isPlaying.value = false;
  emit('pause');
}

function handleStop() {
  isPlaying.value = false;
  progress.value = 0;
  giftSoundService.stopAllSounds();
  emit('stop');
}

function handleToggleMute() {
  if (isMuted.value) {
    isMuted.value = false;
    currentVolume.value = previousVolume.value;
  } else {
    isMuted.value = true;
    previousVolume.value = currentVolume.value;
    currentVolume.value = 0;
  }
}

function handleVolumeChange(volume: number) {
  currentVolume.value = volume;
  isMuted.value = volume === 0;
}

function handleProgressClick(event: MouseEvent) {
  if (duration.value === 0) return;
  
  const progressBar = event.currentTarget as HTMLElement;
  const rect = progressBar.getBoundingClientRect();
  const clickX = event.clientX - rect.left;
  const progressPercent = clickX / rect.width;
  
  progress.value = progressPercent * duration.value;
}

function simulatePlayback() {
  // 模拟播放进度（实际应该通过Web Audio API事件）
  const interval = setInterval(() => {
    if (!isPlaying.value) {
      clearInterval(interval);
      return;
    }
    
    progress.value += 0.1;
    if (progress.value >= duration.value) {
      progress.value = duration.value;
      isPlaying.value = false;
      clearInterval(interval);
    }
  }, 100);
}

function formatTime(seconds: number): string {
  const mins = Math.floor(seconds / 60);
  const secs = Math.floor(seconds % 60);
  return `${mins.toString().padStart(2, '0')}:${secs.toString().padStart(2, '0')}`;
}

function reset() {
  isPlaying.value = false;
  progress.value = 0;
  duration.value = 0;
}
</script>

<template>
  <div class="gift-sound-preview">
    <!-- 播放控制区域 -->
    <div class="playback-controls">
      <div class="control-buttons">
        <el-button
          v-if="!isPlaying"
          type="primary"
          :icon="Play"
          circle
          @click="handlePlay"
          :disabled="!currentSoundFile"
        />
        <el-button
          v-else
          type="warning"
          :icon="VideoPause"
          circle
          @click="handlePause"
        />
        <el-button
          :icon="Refresh"
          circle
          @click="handleStop"
          :disabled="!isPlaying && progress === 0"
        />
      </div>

      <!-- 进度条 -->
      <div class="progress-container">
        <span class="time-display">{{ formattedProgress }}</span>
        <div
          class="progress-bar"
          @click="handleProgressClick"
        >
          <div
            class="progress-fill"
            :style="{ width: `${progressPercent}%` }"
          ></div>
          <div
            class="progress-handle"
            :style="{ left: `${progressPercent}%` }"
          ></div>
        </div>
        <span class="time-display">{{ formattedDuration }}</span>
      </div>

      <!-- 音量控制 -->
      <div class="volume-controls">
        <el-button
          :icon="volumeIcon"
          circle
          @click="handleToggleMute"
          class="volume-button"
        />
        <el-slider
          v-model="currentVolume"
          :min="0"
          :max="1"
          :step="0.01"
          :show-tooltip="false"
          class="volume-slider"
          @input="handleVolumeChange"
        />
        <span class="volume-display">{{ volumePercent }}%</span>
      </div>
    </div>

    <!-- 当前播放信息 -->
    <div v-if="currentSoundFile" class="playback-info">
      <div class="sound-info">
        <span class="label">当前音效:</span>
        <span class="value">{{ currentSoundFile.split('/').pop() }}</span>
      </div>
      <div class="status-info">
        <span class="label">状态:</span>
        <el-tag :type="isPlaying ? 'success' : 'info'" size="small">
          {{ isPlaying ? '播放中' : '已停止' }}
        </el-tag>
      </div>
    </div>

    <!-- 空状态 -->
    <div v-else class="empty-state">
      <el-icon class="empty-icon"><Volume /></el-icon>
      <p>请选择音效文件进行预览</p>
    </div>
  </div>
</template>

<style scoped>
.gift-sound-preview {
  background: #f5f7fa;
  border-radius: 8px;
  padding: 20px;
}

.playback-controls {
  display: flex;
  align-items: center;
  gap: 20px;
  margin-bottom: 15px;
}

.control-buttons {
  display: flex;
  gap: 10px;
}

.progress-container {
  flex: 1;
  display: flex;
  align-items: center;
  gap: 10px;
}

.time-display {
  font-size: 12px;
  color: #606266;
  min-width: 40px;
  text-align: center;
}

.progress-bar {
  flex: 1;
  height: 6px;
  background: #dcdfe6;
  border-radius: 3px;
  position: relative;
  cursor: pointer;
}

.progress-fill {
  height: 100%;
  background: #409eff;
  border-radius: 3px;
  transition: width 0.1s linear;
}

.progress-handle {
  position: absolute;
  top: 50%;
  transform: translate(-50%, -50%);
  width: 12px;
  height: 12px;
  background: #409eff;
  border-radius: 50%;
  cursor: pointer;
  opacity: 0;
  transition: opacity 0.2s;
}

.progress-bar:hover .progress-handle {
  opacity: 1;
}

.volume-controls {
  display: flex;
  align-items: center;
  gap: 10px;
}

.volume-button {
  padding: 8px;
}

.volume-slider {
  width: 100px;
}

.volume-display {
  font-size: 12px;
  color: #606266;
  min-width: 35px;
  text-align: right;
}

.playback-info {
  background: #fff;
  padding: 12px;
  border-radius: 6px;
  border: 1px solid #ebeef5;
}

.sound-info,
.status-info {
  display: flex;
  align-items: center;
  gap: 8px;
  margin-bottom: 8px;
}

.sound-info:last-child,
.status-info:last-child {
  margin-bottom: 0;
}

.label {
  font-size: 12px;
  color: #909399;
  min-width: 60px;
}

.value {
  font-size: 14px;
  color: #303133;
  word-break: break-all;
}

.empty-state {
  text-align: center;
  padding: 30px;
  color: #909399;
}

.empty-icon {
  font-size: 48px;
  margin-bottom: 10px;
}

.empty-state p {
  margin: 0;
  font-size: 14px;
}
</style>
