<script setup lang="ts">
import { onMounted, ref, watch } from 'vue';
import { Download, Refresh } from '@element-plus/icons-vue';
import { normalizeDesktopError, type DataRange } from '@/platform/desktop';
import { formatCount, formatDanmuRate, formatMoneyMinor } from '@/modules/live-data/formatters';
import { useLiveAnalyticsStore } from '@/stores/live-data/analytics';
import DataExportDialog from '@/components/live-data/DataExportDialog.vue';
import DataRangeControl from '@/components/live-data/DataRangeControl.vue';
import TrendChart from '@/components/live-data/TrendChart.vue';

const analytics = useLiveAnalyticsStore();
const range = ref<DataRange>({ kind: 'liveSession' });
const exportVisible = ref(false);

const metrics = () => analytics.statistics?.metrics ?? null;

function showError(value: unknown): void {
  const error = normalizeDesktopError(value);
  if (error.code !== 'dialog_cancelled') ElMessage.error(error.message ?? '操作失败，请稍后重试');
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
  <section class="mx-auto max-w-7xl bg-white">
    <header class="flex min-h-14 flex-wrap items-center gap-3 border-b border-gray-200 py-2">
      <h2 class="mr-auto m-0 text-lg font-semibold text-gray-900">
        数据看板
      </h2>
      <DataRangeControl v-model="range" />
      <el-button :icon="Refresh" :loading="analytics.statPending" @click="query(true, true)">
        刷新
      </el-button>
      <el-button type="primary" :icon="Download" @click="exportVisible = true">
        导出 CSV
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
      title="历史数据暂不可用"
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
        class="grid grid-cols-2 gap-px overflow-hidden border border-gray-200 bg-gray-200 md:grid-cols-3 xl:grid-cols-5"
      >
        <div class="min-w-0 bg-white p-4">
          <dt class="text-sm text-gray-500">
            在线人数
          </dt>
          <dd class="m-0 mt-2 text-2xl font-semibold tabular-nums">
            {{ formatCount(metrics()?.onlineCount ?? 0) }}
          </dd>
        </div>
        <div class="min-w-0 bg-white p-4">
          <dt class="text-sm text-gray-500">
            弹幕总量
          </dt>
          <dd class="m-0 mt-2 text-2xl font-semibold tabular-nums">
            {{ formatCount(metrics()?.danmuCount ?? 0) }}
          </dd>
        </div>
        <div class="min-w-0 bg-white p-4">
          <dt class="text-sm text-gray-500">
            弹幕速率
          </dt>
          <dd class="m-0 mt-2 text-2xl font-semibold tabular-nums">
            {{ formatDanmuRate(metrics()?.danmuRate ?? 0) }}
          </dd>
        </div>
        <div class="min-w-0 bg-white p-4">
          <dt class="text-sm text-gray-500">
            礼物总价值
          </dt>
          <dd class="m-0 mt-2 text-2xl font-semibold tabular-nums">
            {{ formatMoneyMinor(metrics()?.giftValueMinor ?? 0) }}
          </dd>
        </div>
        <div class="min-w-0 bg-white p-4">
          <dt class="text-sm text-gray-500">
            舰长总数
          </dt>
          <dd class="m-0 mt-2 text-2xl font-semibold tabular-nums">
            {{ formatCount(metrics()?.captainCount ?? 0) }}
          </dd>
        </div>
      </dl>

      <div class="mt-5">
        <TrendChart
          title="在线人数趋势"
          :points="analytics.statistics.trendPoints"
          metric="onlineCount"
          color="#2563eb"
        />
        <TrendChart
          title="弹幕速率趋势"
          :points="analytics.statistics.trendPoints"
          metric="danmuRate"
          color="#059669"
        />
        <TrendChart
          title="礼物收益趋势"
          :points="analytics.statistics.trendPoints"
          metric="giftValueMinor"
          color="#d97706"
        />
      </div>
    </div>
    <div v-else class="flex min-h-72 items-center justify-center text-sm text-gray-400">
      暂无数据
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
