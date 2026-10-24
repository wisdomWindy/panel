<script setup lang="ts">
import { useI18n } from 'vue-i18n';
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
const { t } = useI18n();

function togglePause(): void {
  emit('pause', props.status.state !== 'paused');
}
</script>

<template>
  <section class="min-h-32 border-y border-gray-200 py-4 dark:border-zinc-800" aria-labelledby="tts-queue-title">
    <div class="flex flex-wrap items-center justify-between gap-3">
      <div class="min-w-0">
        <h3 id="tts-queue-title" class="m-0 text-base font-semibold text-gray-900 dark:text-zinc-100">
          {{ t('tts.queue.title') }}
        </h3>
        <p class="mt-2 truncate text-sm text-gray-700 dark:text-zinc-300" aria-live="polite">
          <span class="font-medium">{{ t('tts.queue.current') }}</span>
          {{ status.currentText || t('tts.queue.empty') }}
        </p>
        <p class="mt-1 text-sm tabular-nums text-gray-500 dark:text-zinc-400">
          {{ t('tts.queue.pending', { count: status.pendingCount }) }}
        </p>
      </div>
      <div class="flex shrink-0 items-center gap-2">
        <el-tooltip
          :content="status.state === 'paused' ? t('tts.queue.resume') : t('tts.queue.pause')"
        >
          <el-button
            :icon="status.state === 'paused' ? VideoPlay : VideoPause"
            :loading="pausePending"
            :disabled="status.state === 'unavailable'"
            circle
            :aria-label="status.state === 'paused' ? t('tts.queue.resume') : t('tts.queue.pause')"
            @click="togglePause"
          />
        </el-tooltip>
        <el-tooltip :content="t('tts.queue.clear')">
          <el-button
            :icon="Delete"
            :loading="clearPending"
            :disabled="status.pendingCount === 0"
            circle
            :aria-label="t('tts.queue.clear')"
            @click="emit('clear')"
          />
        </el-tooltip>
      </div>
    </div>
    <p v-if="status.fallbackReasonCode" class="mt-3 text-sm text-amber-700 dark:text-amber-400" role="status">
      {{ t('tts.queue.fallback', { reason: status.fallbackReasonCode }) }}
    </p>
  </section>
</template>
