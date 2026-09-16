<script setup lang="ts">
import { computed } from 'vue';
import { VideoCamera, Warning } from '@element-plus/icons-vue';
import type { RecordingConfig, RecordingStatus } from '@/platform/desktop';
import { formatBytes, formatDuration } from '@/modules/media/formatters';

const props = defineProps<{
  status: RecordingStatus;
  config: RecordingConfig;
}>();

const stateText = computed(() => ({
  idle: '未录制',
  starting: '正在启动',
  recording: '录制中',
  stopping: '正在停止',
  finalizing: '正在生成文件',
  error: '录制失败'
}[props.status.state]));

const lowDisk = computed(() =>
  props.status.diskRemainingBytes > 0
  && props.status.diskRemainingBytes < props.config.waterlineBytes
);
</script>

<template>
  <section class="min-h-24 border-y border-gray-200 py-4" aria-label="录制状态" aria-live="polite">
    <div class="flex flex-wrap items-center gap-x-7 gap-y-3">
      <div class="flex items-center gap-2 font-medium" :class="status.state === 'error' ? 'text-red-700' : status.state === 'recording' ? 'text-red-600' : 'text-gray-800'">
        <el-icon><VideoCamera /></el-icon>
        <span>{{ stateText }}</span>
      </div>
      <div class="whitespace-nowrap text-sm text-gray-600">
        时长：<span class="font-mono tabular-nums text-gray-900">{{ formatDuration(status.elapsedSeconds) }}</span>
      </div>
      <div class="whitespace-nowrap text-sm text-gray-600">
        文件：<span class="font-mono tabular-nums text-gray-900">{{ formatBytes(status.fileSizeBytes) }}</span>
      </div>
      <div class="whitespace-nowrap text-sm text-gray-600">
        高光：<span class="font-mono tabular-nums text-gray-900">{{ status.highlightCount }}</span>
      </div>
      <div class="whitespace-nowrap text-sm" :class="lowDisk ? 'font-medium text-amber-700' : 'text-gray-600'">
        <el-icon v-if="lowDisk" class="mr-1 align-middle">
          <Warning />
        </el-icon>
        磁盘：<span class="font-mono tabular-nums">{{ status.diskRemainingBytes > 0 ? formatBytes(status.diskRemainingBytes) : '--' }}</span>
      </div>
      <div class="whitespace-nowrap text-sm text-gray-600">
        配额：<span class="font-mono tabular-nums text-gray-900">{{ formatBytes(config.quotaBytes) }}</span>
      </div>
    </div>
    <p v-if="status.lastErrorCode" class="mb-0 mt-3 text-sm text-red-700">
      {{ status.lastErrorCode }}
    </p>
  </section>
</template>
