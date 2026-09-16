<script setup lang="ts">
import { Delete, VideoPause, VideoPlay } from '@element-plus/icons-vue';
import type { TtsStatus } from '@/platform/desktop';

const props = defineProps<{
  status: TtsStatus;
  pausePending?: boolean;
  clearPending?: boolean;
}>();

const emit = defineEmits<{
  pause: [isPaused: boolean];
  clear: [];
}>();

function togglePause(): void {
  emit('pause', props.status.state !== 'paused');
}
</script>

<template>
  <section class="min-h-32 border-y border-gray-200 py-4" aria-labelledby="tts-queue-title">
    <div class="flex flex-wrap items-center justify-between gap-3">
      <div class="min-w-0">
        <h3 id="tts-queue-title" class="m-0 text-base font-semibold text-gray-900">
          播报队列
        </h3>
        <p class="mt-2 truncate text-sm text-gray-700" aria-live="polite">
          <span class="font-medium">当前：</span>
          {{ status.currentText || '暂无播报' }}
        </p>
        <p class="mt-1 text-sm tabular-nums text-gray-500">
          队列中：{{ status.pendingCount }} 条
        </p>
      </div>
      <div class="flex shrink-0 items-center gap-2">
        <el-tooltip :content="status.state === 'paused' ? '继续播报' : '暂停播报'">
          <el-button
            :icon="status.state === 'paused' ? VideoPlay : VideoPause"
            :loading="pausePending"
            :disabled="status.state === 'unavailable'"
            circle
            :aria-label="status.state === 'paused' ? '继续播报' : '暂停播报'"
            @click="togglePause"
          />
        </el-tooltip>
        <el-tooltip content="清空队列">
          <el-button
            :icon="Delete"
            :loading="clearPending"
            :disabled="status.pendingCount === 0"
            circle
            aria-label="清空队列"
            @click="emit('clear')"
          />
        </el-tooltip>
      </div>
    </div>
    <p
      v-if="status.fallbackReasonCode"
      class="mt-3 text-sm text-amber-700"
      role="status"
    >
      当前已使用降级引擎：{{ status.fallbackReasonCode }}
    </p>
  </section>
</template>
