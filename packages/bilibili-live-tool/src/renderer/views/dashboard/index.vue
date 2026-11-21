<script setup lang="ts">
import { computed, onMounted, ref, watch } from 'vue';
import { useI18n } from 'vue-i18n';
import { Download, Refresh } from '@element-plus/icons-vue';
import { normalizeDesktopError, type DataRange } from '@/platform/desktop';
import { formatCount, formatDanmuRate, formatMoneyMinor } from '@/modules/live-data/formatters';
import { useLiveAnalyticsStore } from '@/stores/live-data/analytics';
import { useMediaTtsStore } from '@/stores/media/tts';
import DataExportDialog from '@/components/live-data/DataExportDialog.vue';
import DataRangeControl from '@/components/live-data/DataRangeControl.vue';
import TrendChart from '@/components/live-data/TrendChart.vue';

const analytics = useLiveAnalyticsStore();
const media = useMediaTtsStore();
const { t } = useI18n();
const range = ref<DataRange>({ kind: 'liveSession' });
const exportVisible = ref(false);

const metrics = () => analytics.statistics?.metrics ?? null;

// -- Qwen3-TTS model download -------------------------------------------
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
  return download.error ?? t('dashboard.qwen3Model.failed');
});
const modelStatusLabel = computed(() => {
  if (!modelServiceAvailable.value) return t('dashboard.qwen3Model.serviceDownShort');
  if (modelDownloading.value) {
    return modelProgress.value === null
      ? t('dashboard.qwen3Model.downloading')
      : t('dashboard.qwen3Model.downloadingPercent', { percent: modelProgress.value });
  }
  if (media.qwen3Catalog?.download.status === 'error') return t('dashboard.qwen3Model.failed');
  return modelInstalled.value
    ? t('dashboard.qwen3Model.ready')
    : t('dashboard.qwen3Model.notDownloaded');
});

async function downloadModel(): Promise<void> {
  try {
    await media.downloadQwen3Model(primaryModel.value?.modelId ?? null);
    ElMessage.success(t('dashboard.qwen3Model.started'));
  } catch (error) {
    showError(error);
  }
}

function showError(value: unknown): void {
  const error = normalizeDesktopError(value);
  if (error.code !== 'dialog_cancelled')
    ElMessage.error(error.message ?? t('common.operationFailed'));
}

async function query(refresh = false, notify = false): Promise<void> {
  try {
    await analytics.queryStatistics(range.value, refresh);
  } catch (error) {
    if (notify) showError(error);
  }
}

async function exportData(
  types: readonly ('statistics' | 'rankings')[],
  selectedRange: DataRange
): Promise<void> {
  try {
    await analytics.exportData(types, selectedRange);
  } catch (error) {
    showError(error);
  }
}

watch(
  range,
  () => {
    void query();
  },
  { deep: true }
);
onMounted(() => {
  void query();
  void media.loadQwen3Models().catch(() => undefined);
});
</script>

<template>
  <section class="mx-auto max-w-7xl bg-white dark:bg-zinc-900">
    <header class="flex min-h-14 flex-wrap items-center gap-3 border-b border-gray-200 py-2 dark:border-zinc-800">
      <h2 class="mr-auto m-0 text-lg font-semibold text-gray-900 dark:text-zinc-100">
        {{ t('dashboard.title') }}
      </h2>
      <DataRangeControl v-model="range" />
      <el-button :icon="Refresh" :loading="analytics.statPending" @click="query(true, true)">
        {{ t('common.refresh') }}
      </el-button>
      <el-button type="primary" :icon="Download" @click="exportVisible = true">
        {{ t('dashboard.exportCsv') }}
      </el-button>
    </header>

    <el-alert
      v-if="analytics.error"
      class="mt-4"
      :title="analytics.error.message"
      type="error"
      :closable="false"
    />
    <el-alert
      v-if="analytics.statistics && !analytics.statistics.isHistoricalDataAvailable"
      class="mt-4"
      :title="t('dashboard.historyUnavailable')"
      type="warning"
      :closable="false"
    />

    <el-skeleton
      v-if="analytics.statPending && !analytics.statistics"
      class="mt-6"
      :rows="8"
      animated
    />
    <div v-else-if="analytics.statistics" class="mt-5">
      <dl
        class="grid grid-cols-2 gap-px overflow-hidden border border-gray-200 bg-gray-200 dark:border-zinc-800 dark:bg-zinc-800 md:grid-cols-3 xl:grid-cols-5"
      >
        <div class="min-w-0 bg-white p-4 dark:bg-zinc-900">
          <dt class="text-sm text-gray-500 dark:text-zinc-400">
            {{ t('dashboard.onlineCount') }}
          </dt>
          <dd class="m-0 mt-2 text-2xl font-semibold tabular-nums text-gray-900 dark:text-zinc-100">
            {{ formatCount(metrics()?.onlineCount ?? 0) }}
          </dd>
        </div>
        <div class="min-w-0 bg-white p-4 dark:bg-zinc-900">
          <dt class="text-sm text-gray-500 dark:text-zinc-400">
            {{ t('dashboard.danmakuCount') }}
          </dt>
          <dd class="m-0 mt-2 text-2xl font-semibold tabular-nums text-gray-900 dark:text-zinc-100">
            {{ formatCount(metrics()?.danmuCount ?? 0) }}
          </dd>
        </div>
        <div class="min-w-0 bg-white p-4 dark:bg-zinc-900">
          <dt class="text-sm text-gray-500 dark:text-zinc-400">
            {{ t('dashboard.danmakuRate') }}
          </dt>
          <dd class="m-0 mt-2 text-2xl font-semibold tabular-nums text-gray-900 dark:text-zinc-100">
            {{ formatDanmuRate(metrics()?.danmuRate ?? 0) }}
          </dd>
        </div>
        <div class="min-w-0 bg-white p-4 dark:bg-zinc-900">
          <dt class="text-sm text-gray-500 dark:text-zinc-400">
            {{ t('dashboard.giftRevenue') }}
          </dt>
          <dd class="m-0 mt-2 text-2xl font-semibold tabular-nums text-gray-900 dark:text-zinc-100">
            {{ formatMoneyMinor(metrics()?.giftValueMinor ?? 0) }}
          </dd>
        </div>
        <div class="min-w-0 bg-white p-4 dark:bg-zinc-900">
          <dt class="text-sm text-gray-500 dark:text-zinc-400">
            {{ t('dashboard.captainCount') }}
          </dt>
          <dd class="m-0 mt-2 text-2xl font-semibold tabular-nums text-gray-900 dark:text-zinc-100">
            {{ formatCount(metrics()?.captainCount ?? 0) }}
          </dd>
        </div>
      </dl>

      <div class="mt-5">
        <TrendChart
          :title="t('dashboard.onlineTrend')"
          :points="analytics.statistics.trendPoints"
          metric="onlineCount"
          color="#2563eb"
        />
        <TrendChart
          :title="t('dashboard.danmakuTrend')"
          :points="analytics.statistics.trendPoints"
          metric="danmuRate"
          color="#059669"
        />
        <TrendChart
          :title="t('dashboard.giftTrend')"
          :points="analytics.statistics.trendPoints"
          metric="giftValueMinor"
          color="#d97706"
        />
      </div>
    </div>
    <div
      v-else
      class="flex min-h-72 items-center justify-center text-sm text-gray-400 dark:text-zinc-400"
    >
      {{ t('liveData.empty') }}
    </div>

    <section
      class="mt-5 border border-gray-200 p-4 dark:border-zinc-800"
      aria-labelledby="qwen3-model-title"
    >
      <div class="flex flex-wrap items-center gap-3">
        <div class="mr-auto min-w-0">
          <h3
            id="qwen3-model-title"
            class="m-0 text-base font-semibold text-gray-900 dark:text-zinc-100"
          >
            {{ t('dashboard.qwen3Model.title') }}
          </h3>
          <p class="mt-1 mb-0 text-sm text-gray-500 dark:text-zinc-400">
            {{ t('dashboard.qwen3Model.description') }}
          </p>
        </div>
        <el-tag :type="modelInstalled ? 'success' : 'info'" effect="plain">
          {{ modelStatusLabel }}
        </el-tag>
        <el-button
          type="primary"
          :icon="Download"
          :loading="modelDownloading"
          :disabled="modelDownloading || !modelServiceAvailable"
          @click="downloadModel"
        >
          {{
            modelInstalled
              ? t('dashboard.qwen3Model.redownload')
              : t('dashboard.qwen3Model.download')
          }}
        </el-button>
      </div>
      <el-progress
        v-if="modelDownloading"
        class="mt-3"
        :percentage="modelProgress ?? 0"
        :indeterminate="modelProgress === null"
        :stroke-width="10"
      />
      <p
        v-if="modelError"
        class="mt-2 mb-0 text-sm text-red-600 dark:text-red-400"
        role="alert"
      >
        {{ modelError }}
      </p>
      <p
        v-else-if="!modelServiceAvailable"
        class="mt-2 mb-0 text-sm text-amber-700 dark:text-amber-400"
      >
        {{ t('dashboard.qwen3Model.serviceDown') }}
      </p>
    </section>

    <DataExportDialog
      :visible="exportVisible"
      :range="range"
      initial-type="statistics"
      :pending="analytics.exportPending"
      :paths="analytics.exportedPaths"
      @close="exportVisible = false"
      @submit="exportData"
    />
  </section>
</template>
