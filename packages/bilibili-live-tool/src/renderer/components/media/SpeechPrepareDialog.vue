<script setup lang="ts">
import { computed, onUnmounted, ref, watch } from 'vue';
import { useI18n } from 'vue-i18n';
import { ElMessage } from 'element-plus';
import { useMediaTtsStore } from '@/stores/media/tts';

/**
 * Shows the staged speech-runtime preparation. Mounted once in the shell so the
 * panel appears wherever the operator is, including a start-up install that the
 * stored configuration triggers.
 */
const { t } = useI18n();
const media = useMediaTtsStore();

const visible = computed(() => media.preparing);
const message = computed(
  () => media.environmentProgress?.message ?? t('tts.environment.starting')
);

/** Elapsed seconds keep the panel informative while pip works silently. */
const elapsed = ref(0);
let timer: ReturnType<typeof setInterval> | null = null;

watch(visible, (active, wasActive) => {
  // A finished preparation that closed the panel deserves a confirmation: the
  // operator only saw an indeterminate bar for minutes.
  if (!active && wasActive && media.environment.ready) {
    ElMessage.success(t('tts.environment.ready'));
  }
  if (active && timer === null) {
    elapsed.value = 0;
    timer = setInterval(() => {
      elapsed.value += 1;
    }, 1000);
  } else if (!active && timer !== null) {
    clearInterval(timer);
    timer = null;
  }
});

onUnmounted(() => {
  if (timer !== null) clearInterval(timer);
});

const elapsedLabel = computed(() =>
  t('tts.environment.elapsed', { seconds: elapsed.value })
);
</script>

<template>
  <el-dialog
    :model-value="visible"
    :close-on-click-modal="false"
    :show-close="false"
    width="min(460px, calc(100vw - 32px))"
  >
    <div class="flex flex-col gap-3">
      <p class="app-text m-0 text-sm font-semibold">{{ t('tts.environment.starting') }}</p>
      <p class="app-muted m-0 text-sm">{{ message }}</p>
      <el-progress :percentage="100" indeterminate :stroke-width="8" :show-text="false" />
      <div class="flex items-center justify-between">
        <span class="app-faint text-xs">{{ t('tts.environment.sizeHint') }}</span>
        <span class="app-muted text-xs">{{ elapsedLabel }}</span>
      </div>
    </div>
  </el-dialog>
</template>
