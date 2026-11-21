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
  <section class="app-card min-h-32 p-4" aria-labelledby="tts-queue-title">
    <div class="flex flex-wrap items-center justify-between gap-3">
      <div class="min-w-0">
        <h3 id="tts-queue-title" class="app-text m-0 text-base font-semibold">
          {{ t('tts.queue.title') }}
        </h3>
        <p class="app-muted mt-2 truncate text-sm" aria-live="polite">
          <span class="app-text font-medium">{{ t('tts.queue.current') }}</span>
          {{ status.currentText || t('tts.queue.empty') }}
        </p>
        <p class="app-faint mt-1 text-sm tabular-nums">
          {{ t('tts.queue.pending', { count: status.pendingCount }) }}
        </p>
      </div>
      <div class="flex shrink-0 items-center gap-2">
        <el-tooltip
          :content="status.state === 'paused' ? t('tts.queue.resume') : t('tts.queue.pause')"
        >
          <button
            type="button"
            class="app-icon-btn"
            :disabled="status.state === 'unavailable' || pausePending"
            :aria-label="status.state === 'paused' ? t('tts.queue.resume') : t('tts.queue.pause')"
            @click="togglePause"
          >
            <el-icon><component :is="status.state === 'paused' ? VideoPlay : VideoPause" /></el-icon>
          </button>
        </el-tooltip>
        <el-tooltip :content="t('tts.queue.clear')">
          <button
            type="button"
            class="app-icon-btn"
            :disabled="status.pendingCount === 0 || clearPending"
            :aria-label="t('tts.queue.clear')"
            @click="emit('clear')"
          >
            <el-icon><Delete /></el-icon>
          </button>
        </el-tooltip>
      </div>
    </div>
    <p v-if="status.fallbackReasonCode" class="mt-3 text-sm" style="color: var(--app-gold)" role="status">
      {{ t('tts.queue.fallback', { reason: status.fallbackReasonCode }) }}
    </p>
  </section>
</template>
