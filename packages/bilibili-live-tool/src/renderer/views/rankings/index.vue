<script setup lang="ts">
import { computed, onMounted, ref, watch } from 'vue';
import { useI18n } from 'vue-i18n';
import { normalizeDesktopError, type DataRange, type UserUid } from '@/platform/desktop';
import { formatCount, formatMoneyMinor, formatTimestamp } from '@/modules/live-data/formatters';
import { useLiveAnalyticsStore } from '@/stores/live-data/analytics';
import { useLiveRoomStore } from '@/stores/live-data/room';
import AudienceDetailDrawer from '@/components/live-data/AudienceDetailDrawer.vue';
import DataExportDialog from '@/components/live-data/DataExportDialog.vue';
import DataRangeControl from '@/components/live-data/DataRangeControl.vue';

  type RankTab = 'gift' | 'danmu' | 'captain';
  type TopLimit = 10 | 20 | 50 | 100;

const analytics = useLiveAnalyticsStore();
const { t } = useI18n();
const room = useLiveRoomStore();
const range = ref<DataRange>({ kind: 'liveSession' });
const limit = ref<TopLimit>(10);
const selectedUid = ref<UserUid | null>(null);
const exportVisible = ref(false);

/** The design shows the three boards side by side instead of behind tabs. */
const columns = computed(() =>
  (['gift', 'danmu', 'captain'] as const).map(tab => ({
    tab,
    title: t(`rankings.${tab}`),
    rows: (analytics.rankings?.[tab] ?? []).slice(0, limit.value)
  }))
);

const updatedAt = computed(() =>
  analytics.rankings ? formatTimestamp(analytics.rankings.generatedAt) : '-'
);

function metricOf(tab: RankTab, row: { amountMinor: number; danmuCount: number }): string {
  return tab === 'danmu'
    ? t('rankings.danmakuUnit', { count: formatCount(row.danmuCount) })
    : formatMoneyMinor(row.amountMinor);
}

function showError(value: unknown): void {
  const error = normalizeDesktopError(value);
  if (error.code !== 'dialog_cancelled')
    ElMessage.error(error.message ?? t('common.operationFailed'));
}

async function query(): Promise<void> {
  try {
    await analytics.queryRankings(range.value);
  } catch {
    // The persistent inline error state is sufficient for automatic queries.
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
        {{ t('rankings.title') }}
      </h2>
      <DataRangeControl v-model="range" />
      <div class="flex items-center gap-1.5">
        <button
          v-for="option in ([10, 20, 50, 100] as const)"
          :key="option"
          type="button"
          class="app-chip"
          :class="{ 'app-chip--active': limit === option }"
          :aria-pressed="limit === option"
          @click="limit = option"
        >
          TOP {{ option }}
        </button>
      </div>
      <button type="button" class="app-btn-primary h-8 px-3 text-sm" @click="exportVisible = true">
        {{ t('liveData.exportCsv') }}
      </button>
    </header>

    <el-alert
      v-if="analytics.error"
      :title="analytics.error.message"
      type="error"
      :closable="false"
    />
    <el-alert
      v-if="analytics.rankings && !analytics.rankings.isHistoricalDataAvailable"
      :title="t('rankings.historyUnavailable')"
      type="warning"
      :closable="false"
    />

    <div class="grid grid-cols-1 gap-4 xl:grid-cols-3">
      <section v-for="column in columns" :key="column.tab" class="app-card flex flex-col p-4">
        <h3 class="app-text m-0 text-sm font-semibold">
          {{ column.title }}
          <span class="app-faint">TOP{{ limit }}</span>
        </h3>
        <p v-if="column.rows.length === 0" class="app-faint mt-6 mb-0 text-sm">
          {{ t('liveData.empty') }}
        </p>
        <ul v-else class="m-0 mt-3 list-none p-0">
          <li
            v-for="row in column.rows"
            :key="row.userUid"
            class="flex items-center gap-2 border-b py-2 last:border-b-0"
            style="border-color: var(--app-border)"
          >
            <span
              class="app-rank"
              :class="row.rank <= 3 ? `app-rank--${row.rank}` : 'app-rank--plain'"
              >{{ row.rank }}</span
            >
            <button
              type="button"
              class="app-text min-w-0 flex-1 truncate text-left text-sm font-medium hover:underline"
              @click="selectedUid = row.userUid"
            >
              {{ row.username }}
            </button>
            <span class="app-muted shrink-0 text-xs tabular-nums">
              {{ metricOf(column.tab, row) }}
            </span>
          </li>
        </ul>
      </section>
    </div>

    <footer class="app-faint text-xs">
      {{ t('rankings.updatedAt', { time: updatedAt }) }}
    </footer>

    <AudienceDetailDrawer
      :visible="selectedUid !== null"
      :room-id="room.snapshot.roomId"
      :user-uid="selectedUid"
      @close="selectedUid = null"
    />
    <DataExportDialog
      :visible="exportVisible"
      :range="range"
      initial-type="rankings"
      :pending="analytics.exportPending"
      :paths="analytics.exportedPaths"
      @close="exportVisible = false"
      @submit="exportData"
    />
  </section>
</template>
