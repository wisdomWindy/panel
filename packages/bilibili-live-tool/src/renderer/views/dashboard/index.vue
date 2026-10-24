<script setup lang="ts">
import { onMounted, ref, watch } from 'vue';
import { useI18n } from 'vue-i18n';
import { Download, Refresh } from '@element-plus/icons-vue';
import { normalizeDesktopError, type DataRange } from '@/platform/desktop';
import { formatCount, formatDanmuRate, formatMoneyMinor } from '@/modules/live-data/formatters';
import { useLiveAnalyticsStore } from '@/stores/live-data/analytics';
import DataExportDialog from '@/components/live-data/DataExportDialog.vue';
import DataRangeControl from '@/components/live-data/DataRangeControl.vue';
import TrendChart from '@/components/live-data/TrendChart.vue';

const analytics = useLiveAnalyticsStore();
const { t } = useI18n();
const range = ref<DataRange>({ kind: 'liveSession' });
const exportVisible = ref(false);

const metrics = () => analytics.statistics?.metrics ?? null;

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
