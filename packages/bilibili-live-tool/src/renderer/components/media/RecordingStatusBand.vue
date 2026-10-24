<script setup lang="ts">
import { computed } from 'vue';
import { useI18n } from 'vue-i18n';
import { VideoCamera, Warning } from '@element-plus/icons-vue';
import type { RecordingConfig, RecordingStatus } from '@/platform/desktop';
import { formatBytes, formatDuration } from '@/modules/media/formatters';

const props = defineProps<{
    status: RecordingStatus;
    config: RecordingConfig;
  }>();
const { t } = useI18n();

const stateText = computed(() => t(`recording.${props.status.state}`));

const lowDisk = computed(
  () =>
    props.status.diskRemainingBytes > 0 &&
      props.status.diskRemainingBytes < props.config.waterlineBytes
);
</script>

<template>
  <section
    class="min-h-24 border-y border-gray-200 py-4 dark:border-zinc-800"
    :aria-label="t('recording.recordingStatus')"
    aria-live="polite"
  >
    <div class="flex flex-wrap items-center gap-x-7 gap-y-3">
      <div
        class="flex items-center gap-2 font-medium"
        :class="
          status.state === 'error'
            ? 'text-red-700 dark:text-red-400'
            : status.state === 'recording'
              ? 'text-red-600 dark:text-red-400'
              : 'text-gray-800 dark:text-zinc-200'
        "
      >
        <el-icon><VideoCamera /></el-icon>
        <span>{{ stateText }}</span>
      </div>
      <div class="whitespace-nowrap text-sm text-gray-600 dark:text-zinc-300">
        {{ t('recording.duration') }}:
        <span class="font-mono tabular-nums text-gray-900 dark:text-zinc-100">
          {{ formatDuration(status.elapsedSeconds) }}
        </span>
      </div>
      <div class="whitespace-nowrap text-sm text-gray-600 dark:text-zinc-300">
        {{ t('recording.file') }}:
        <span class="font-mono tabular-nums text-gray-900 dark:text-zinc-100">
          {{ formatBytes(status.fileSizeBytes) }}
        </span>
      </div>
      <div class="whitespace-nowrap text-sm text-gray-600 dark:text-zinc-300">
        {{ t('recording.highlightCount') }}:
        <span class="font-mono tabular-nums text-gray-900 dark:text-zinc-100">{{ status.highlightCount }}</span>
      </div>
      <div
        class="whitespace-nowrap text-sm"
        :class="lowDisk ? 'font-medium text-amber-700 dark:text-amber-400' : 'text-gray-600 dark:text-zinc-300'"
      >
        <el-icon v-if="lowDisk" class="mr-1 align-middle">
          <Warning />
        </el-icon>
        {{ t('recording.disk') }}:
        <span class="font-mono tabular-nums">
          {{ status.diskRemainingBytes > 0 ? formatBytes(status.diskRemainingBytes) : '--' }}
        </span>
      </div>
      <div class="whitespace-nowrap text-sm text-gray-600 dark:text-zinc-300">
        {{ t('recording.quota') }}:
        <span class="font-mono tabular-nums text-gray-900 dark:text-zinc-100">
          {{ formatBytes(config.quotaBytes) }}
        </span>
      </div>
    </div>
    <p v-if="status.lastErrorCode" class="mb-0 mt-3 text-sm text-red-700 dark:text-red-400">
      {{ status.lastErrorCode }}
    </p>
  </section>
</template>
