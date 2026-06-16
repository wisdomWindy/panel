<script setup lang="ts">
import { computed } from 'vue';
import { VideoCamera, VideoPause, VideoPlay, Square } from '@element-plus/icons-vue';
import { useRecordingStore } from '../stores/recording';

const recordingStore = useRecordingStore();

const statusIcon = computed(() => {
  switch (recordingStore.status) {
    case 'recording':
      return VideoCamera;
    case 'paused':
      return VideoPause;
    default:
      return VideoCamera;
  }
});

const statusClass = computed(() => {
  switch (recordingStore.status) {
    case 'recording':
      return 'recording-active';
    case 'paused':
      return 'recording-paused';
    default:
      return 'recording-idle';
  }
});

const statusText = computed(() => {
  switch (recordingStore.status) {
    case 'recording':
      return '录制中';
    case 'paused':
      return '已暂停';
    default:
      return '未录制';
  }
});

async function handleToggleRecording() {
  try {
    if (recordingStore.isRecording) {
      await recordingStore.pauseRecording();
    } else if (recordingStore.isPaused) {
      await recordingStore.resumeRecording();
    } else {
      // 开始录制需要配置，这里只是触发，实际配置在录制管理页面
      // 可以使用默认配置或提示用户去设置
    }
  } catch (error) {
    console.error('录制操作失败:', error);
  }
}

async function handleStopRecording() {
  try {
    await recordingStore.stopRecording();
  } catch (error) {
    console.error('停止录制失败:', error);
  }
}
</script>

<template>
  <div class="recording-status-bar flex items-center gap-2 px-3 py-1 bg-gray-800 text-white text-sm">
    <!-- 录制状态图标 -->
    <div class="flex items-center gap-1">
      <el-icon
        :class="statusClass"
        class="text-lg"
      >
        <component :is="statusIcon" />
      </el-icon>
      <span class="status-text">{{ statusText }}</span>
    </div>

    <!-- 录制时长 -->
    <div
      v-if="recordingStore.isRecording || recordingStore.isPaused"
      class="flex items-center gap-1"
    >
      <span class="text-gray-400">时长:</span>
      <span class="font-mono">{{ recordingStore.formattedDuration }}</span>
    </div>

    <!-- 文件大小 -->
    <div
      v-if="recordingStore.isRecording || recordingStore.isPaused"
      class="flex items-center gap-1"
    >
      <span class="text-gray-400">大小:</span>
      <span class="font-mono">{{ recordingStore.formattedFileSize }}</span>
    </div>

    <!-- 高光计数 -->
    <div
      v-if="recordingStore.highlightCount > 0"
      class="flex items-center gap-1"
    >
      <span class="text-gray-400">高光:</span>
      <span class="font-mono text-yellow-400">{{ recordingStore.highlightCount }}</span>
    </div>

    <!-- 操作按钮 -->
    <div
      v-if="recordingStore.isRecording || recordingStore.isPaused"
      class="flex items-center gap-1 ml-auto"
    >
      <el-button
        size="small"
        :type="recordingStore.isRecording ? 'warning' : 'success'"
        @click="handleToggleRecording"
      >
        <el-icon class="mr-1">
          <component :is="recordingStore.isRecording ? VideoPause : VideoPlay" />
        </el-icon>
        {{ recordingStore.isRecording ? '暂停' : '继续' }}
      </el-button>
      <el-button
        size="small"
        type="danger"
        @click="handleStopRecording"
      >
        <el-icon class="mr-1">
          <Square />
        </el-icon>
        停止
      </el-button>
    </div>
  </div>
</template>

<style scoped>
.recording-status-bar {
  min-height: 32px;
  border-top: 1px solid #374151;
}

.recording-active {
  color: #ef4444;
  animation: pulse 1s infinite;
}

.recording-paused {
  color: #f59e0b;
}

.recording-idle {
  color: #6b7280;
}

@keyframes pulse {
  0%, 100% {
    opacity: 1;
  }
  50% {
    opacity: 0.5;
  }
}

.status-text {
  min-width: 40px;
}
</style>
