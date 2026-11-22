<script setup lang="ts">
import { computed } from 'vue';
import { useI18n } from 'vue-i18n';
import { useMediaTtsStore } from '@/stores/media/tts';

/**
 * Shows the staged speech-runtime preparation. Mounted by every page that can
 * enable speech (语音播报 and AI 助手), so the one-click install looks the same
 * wherever it is triggered.
 */
const { t } = useI18n();
const media = useMediaTtsStore();

const visible = computed(() => media.preparing);
const message = computed(
  () => media.environmentProgress?.message ?? t('tts.environment.starting')
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
      <p class="app-faint m-0 text-xs">{{ t('tts.environment.sizeHint') }}</p>
    </div>
  </el-dialog>
</template>
