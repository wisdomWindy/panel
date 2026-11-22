<script setup lang="ts">
import { computed, onMounted, ref } from 'vue';
import { useI18n } from 'vue-i18n';
import { ElMessage } from 'element-plus';
import { normalizeDesktopError } from '@/platform/desktop';
import { useMediaTtsStore } from '@/stores/media/tts';

/**
 * Qwen3-TTS model download. It lives with the speech feature, not on the
 * dashboard: the weights are only useful for 播报 and AI 语音回复 (PRD 9.11/9.13).
 */
const { t } = useI18n();
const media = useMediaTtsStore();

const qwenModels = computed(() => media.qwen3Catalog?.models ?? []);
const primaryModel = computed(
  () => qwenModels.value.find(model => model.isDefault) ?? qwenModels.value[0] ?? null
);
const modelDownloading = computed(() => media.qwen3Catalog?.download.status === 'downloading');
const modelInstalled = computed(() => primaryModel.value?.isInstalled ?? false);
const modelProgress = computed(() => media.qwen3Catalog?.download.progressPercent ?? null);
const modelServiceAvailable = computed(() => media.qwen3Catalog !== null);
const modelError = computed(() => {
  const download = media.qwen3Catalog?.download;
  if (!download || download.status !== 'error') return null;
  return download.error ?? t('tts.qwen3Model.failed');
});
const modelStatusLabel = computed(() => {
  if (!modelServiceAvailable.value) return t('tts.qwen3Model.serviceDownShort');
  if (modelDownloading.value) {
    return modelProgress.value === null
      ? t('tts.qwen3Model.downloading')
      : t('tts.qwen3Model.downloadingPercent', { percent: modelProgress.value });
  }
  if (media.qwen3Catalog?.download.status === 'error') return t('tts.qwen3Model.failed');
  return modelInstalled.value ? t('tts.qwen3Model.ready') : t('tts.qwen3Model.notDownloaded');
});

async function downloadModel(): Promise<void> {
  if (requesting.value || modelDownloading.value) return;
  requesting.value = true;
  try {
    // The service is started by enabling 语音播报, not by visiting this page.
    if (!modelServiceAvailable.value) {
      ElMessage.warning(t('tts.qwen3Model.enableFirst'));
      return;
    }
    await media.downloadQwen3Model(primaryModel.value?.modelId ?? null);
    ElMessage.success(t('tts.qwen3Model.started'));
  } catch (error) {
    ElMessage.error(normalizeDesktopError(error).message ?? t('tts.qwen3Model.failed'));
  } finally {
    requesting.value = false;
  }
}

const requesting = ref(false);

onMounted(() => {
  // Read-only on purpose: opening the page must not start the service, install
  // dependencies or download anything; only enabling 语音播报 does that.
  void media.loadQwen3Models().catch(() => undefined);
});
</script>

<template>
  <section class="app-card p-4" aria-labelledby="qwen3-model-title">
    <div class="flex flex-wrap items-center gap-3">
      <div class="mr-auto min-w-0">
        <h3 id="qwen3-model-title" class="app-text m-0 text-base font-semibold">
          {{ t('tts.qwen3Model.title') }}
        </h3>
        <p class="app-muted mt-1 mb-0 text-sm">
          {{ t('tts.qwen3Model.description') }}
        </p>
      </div>
      <span v-if="modelServiceAvailable" class="app-badge" :class="modelInstalled ? 'app-badge--captain' : 'app-badge--plain'">
        {{ modelStatusLabel }}
      </span>
      <button
        type="button"
        class="app-btn-primary h-8 px-3 text-sm"
        :disabled="modelDownloading || requesting"
        @click="downloadModel"
      >
        {{
          modelDownloading
            ? modelProgress === null
              ? t('tts.qwen3Model.downloading')
              : t('tts.qwen3Model.downloadingPercent', { percent: modelProgress })
            : requesting
                ? t('tts.qwen3Model.preparing')
                : modelInstalled
                ? t('tts.qwen3Model.redownload')
                : t('tts.qwen3Model.download')
        }}
      </button>
    </div>
    <el-progress
      v-if="modelDownloading || requesting"
      class="mt-3"
      :percentage="modelProgress ?? 0"
      :indeterminate="modelProgress === null"
      :stroke-width="10"
    />
    <p
      v-if="modelDownloading && modelProgress === null"
      class="app-muted mt-1 mb-0 text-xs"
    >
      {{ t('tts.qwen3Model.progressHint') }}
    </p>
    <p v-if="modelError" class="mt-2 mb-0 text-sm" style="color: var(--app-red)" role="alert">
      {{ modelError }}
    </p>
  </section>
</template>
