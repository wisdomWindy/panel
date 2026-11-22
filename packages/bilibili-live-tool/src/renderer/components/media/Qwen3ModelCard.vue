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
    // Starting the service first keeps the button useful before the app is
    // ready; the progress bar then reports the real download percentage.
    if (!modelServiceAvailable.value) await startService(false);
    if (!modelServiceAvailable.value) return;
    await media.downloadQwen3Model(primaryModel.value?.modelId ?? null);
    ElMessage.success(t('tts.qwen3Model.started'));
  } catch (error) {
    ElMessage.error(normalizeDesktopError(error).message ?? t('tts.qwen3Model.failed'));
  } finally {
    requesting.value = false;
  }
}

const requesting = ref(false);
const starting = ref(false);

/** Boots the bundled local service so the operator never opens a terminal. */
async function startService(announce = true): Promise<void> {
  if (starting.value) return;
  starting.value = true;
  try {
    await media.startQwen3Service();
    // The catalog is served by the same process, so refresh it once it is up.
    await media.loadQwen3Models().catch(() => undefined);
    if (announce) ElMessage.success(t('tts.qwen3Model.serviceStarted'));
  } catch (error) {
    if (announce) {
      ElMessage.error(normalizeDesktopError(error).message ?? t('tts.qwen3Model.serviceStartFailed'));
    }
  } finally {
    starting.value = false;
  }
}

onMounted(async () => {
  await media.loadQwen3Models().catch(() => undefined);
  // Transparent start-up: try once, stay quiet if it cannot work on this machine.
  if (!modelServiceAvailable.value) void startService(false);
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
      <span class="app-badge" :class="modelInstalled ? 'app-badge--captain' : 'app-badge--plain'">
        {{ modelStatusLabel }}
      </span>
      <button
        type="button"
        class="app-btn-primary h-8 px-3 text-sm"
        :disabled="modelDownloading || requesting || starting"
        @click="downloadModel"
      >
        {{
          modelDownloading
            ? modelProgress === null
              ? t('tts.qwen3Model.downloading')
              : t('tts.qwen3Model.downloadingPercent', { percent: modelProgress })
            : requesting || starting
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
    <!-- No start button: enabling 语音播报 owns the service lifecycle (PRD 9.11). -->
    <div v-else-if="!modelServiceAvailable" class="mt-2">
      <p class="m-0 text-sm" style="color: var(--app-gold)">
        {{ starting ? t('tts.qwen3Model.serviceStarting') : t('tts.qwen3Model.serviceDown') }}
      </p>
    </div>
  </section>
</template>
