<script setup lang="ts">
import { computed, onMounted, ref, watch } from 'vue';
import { useI18n } from 'vue-i18n';
import { Refresh } from '@element-plus/icons-vue';
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

const metricCards = computed(() => [
  { id: 'online', label: t('dashboard.onlineCount'), value: formatCount(metrics()?.onlineCount ?? 0) },
  { id: 'danmu', label: t('dashboard.danmakuCount'), value: formatCount(metrics()?.danmuCount ?? 0) },
  { id: 'rate', label: t('dashboard.danmakuRate'), value: formatDanmuRate(metrics()?.danmuRate ?? 0) },
  {
    id: 'gift',
    label: t('dashboard.giftRevenue'),
    value: formatMoneyMinor(metrics()?.giftValueMinor ?? 0)
  },
  { id: 'captain', label: t('dashboard.captainCount'), value: formatCount(metrics()?.captainCount ?? 0) },
  { id: 'like', label: t('dashboard.likeCount'), value: formatCount(metrics()?.likeCount ?? 0) }
]);

function showError(value: unknown): void {
  const error = normalizeDesktopError(value);
  if (error.code !== 'SYS_DIALOG_CANCELLED')
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
  <section class="mx-auto flex w-full max-w-[1180px] flex-col gap-4">
    <header class="flex flex-wrap items-center gap-2">
      <h2 class="app-text mr-auto m-0 text-lg font-semibold">
        {{ t('dashboard.title') }}
      </h2>
      <DataRangeControl v-model="range" />
      <button type="button" class="app-icon-btn" :aria-label="t('common.refresh')" @click="query(true, true)">
        <el-icon><Refresh /></el-icon>
      </button>
      <button type="button" class="app-btn-primary h-8 px-3 text-sm" @click="exportVisible = true">
        {{ t('dashboard.exportCsv') }}
      </button>
    </header>

    <el-alert
      v-if="analytics.error"
      :title="analytics.error.message"
      type="error"
      :closable="false"
    />
    <el-alert
      v-if="analytics.statistics && !analytics.statistics.isHistoricalDataAvailable"
      :title="t('dashboard.historyUnavailable')"
      type="warning"
      :closable="false"
    />

    <el-skeleton v-if="analytics.statPending && !analytics.statistics" :rows="8" animated />
    <template v-else-if="analytics.statistics">
      <div class="grid grid-cols-2 gap-4 md:grid-cols-3 xl:grid-cols-5">
        <article v-for="card in metricCards" :key="card.id" class="app-metric">
          <p class="app-metric-label m-0">{{ card.label }}</p>
          <p class="app-metric-value mt-1.5 mb-0">{{ card.value }}</p>
        </article>
      </div>

      <div class="grid grid-cols-1 gap-4 xl:grid-cols-2">
        <TrendChart
          :title="t('dashboard.danmakuTrend')"
          :points="analytics.statistics.trendPoints"
          metric="danmuRate"
          color="#00aeec"
        />
        <TrendChart
          :title="t('dashboard.onlineTrend')"
          :points="analytics.statistics.trendPoints"
          metric="onlineCount"
          color="#f5a623"
        />
        <TrendChart
          :title="t('dashboard.giftTrend')"
          :points="analytics.statistics.trendPoints"
          metric="giftValueMinor"
          color="#34c759"
        />
      </div>
    </template>
    <div v-else class="app-faint flex min-h-72 items-center justify-center text-sm">
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
