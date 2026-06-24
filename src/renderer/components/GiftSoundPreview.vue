<script setup lang="ts">
import { ref, computed, onMounted, watch } from 'vue';
import { ElMessage } from 'element-plus';
import { Play, VideoPause, Refresh, VolumeMute, Volume } from '@element-plus/icons-vue';
import { giftSoundService } from '../services/gift-sound-service';

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

</script>

<template>
  <div class="bg-[#f5f7fa] rounded-lg p-5">
    <!-- 播放控制区域 -->
    <div class="flex items-center gap-5 mb-4">
      <div class="flex gap-2.5">
        <el-button
          v-if="!isPlaying"
          type="primary"
          :icon="Play"
          circle
          :disabled="!currentSoundFile"
          @click="handlePlay"
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
          :disabled="!isPlaying && progress === 0"
          @click="handleStop"
        />
      </div>

      <!-- 进度条 -->
      <div class="flex-1 flex items-center gap-2.5">
        <span class="text-xs text-[#606266] min-w-[40px] text-center">{{ formattedProgress }}</span>
        <div
          class="flex-1 h-1.5 bg-[#dcdfe6] rounded-[3px] relative cursor-pointer"
          @click="handleProgressClick"
        >
          <div
            class="h-full bg-[#409eff] rounded-[3px] transition-[width] duration-100 linear"
            :style="{ width: `${progressPercent}%` }"
          />
          <div
            class="absolute top-1/2 -translate-x-1/2 -translate-y-1/2 w-3 h-3 bg-[#409eff] rounded-full cursor-pointer opacity-0 transition-opacity duration-200 hover:opacity-100"
            :style="{ left: `${progressPercent}%` }"
          />
        </div>
        <span class="text-xs text-[#606266] min-w-[40px] text-center">{{ formattedDuration }}</span>
      </div>

      <!-- 音量控制 -->
      <div class="flex items-center gap-2.5">
        <el-button
          :icon="volumeIcon"
          circle
          class="p-2"
          @click="handleToggleMute"
        />
        <el-slider
          v-model="currentVolume"
          :min="0"
          :max="1"
          :step="0.01"
          :show-tooltip="false"
          class="w-25"
          @input="handleVolumeChange"
        />
        <span class="text-xs text-[#606266] min-w-[35px] text-right">{{ volumePercent }}%</span>
      </div>
    </div>

    <!-- 当前播放信息 -->
    <div v-if="currentSoundFile" class="bg-white p-3 rounded-md border border-[#ebeef5]">
      <div class="flex items-center gap-2 mb-2">
        <span class="text-xs text-[#909399] min-w-[60px]">当前音效:</span>
        <span class="text-sm text-[#303133] break-all">{{ currentSoundFile.split('/').pop() }}</span>
      </div>
      <div class="flex items-center gap-2 mb-0">
        <span class="text-xs text-[#909399] min-w-[60px]">状态:</span>
        <el-tag :type="isPlaying ? 'success' : 'info'" size="small">
          {{ isPlaying ? '播放中' : '已停止' }}
        </el-tag>
      </div>
    </div>

    <!-- 空状态 -->
    <div v-else class="text-center py-8 text-[#909399]">
      <el-icon class="text-5xl mb-2.5">
        <Volume />
      </el-icon>
      <p class="m-0 text-sm">
        请选择音效文件进行预览
      </p>
    </div>
  </div>
</template>
