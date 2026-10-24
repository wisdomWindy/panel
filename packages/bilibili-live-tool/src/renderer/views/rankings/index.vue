<script setup lang="ts">
import { computed, onMounted, ref, watch } from 'vue';
import { useI18n } from 'vue-i18n';
import { ArrowDown, ArrowUp, Download, Minus, Plus } from '@element-plus/icons-vue';
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
const activeTab = ref<RankTab>('gift');
const limit = ref<TopLimit>(20);
const selectedUid = ref<UserUid | null>(null);
const exportVisible = ref(false);

const rows = computed(() => analytics.rankings?.[activeTab.value].slice(0, limit.value) ?? []);
const totalUsers = computed(() => {
  if (!analytics.rankings) return 0;
  if (activeTab.value === 'gift') return analytics.rankings.giftTotalUsers;
  if (activeTab.value === 'danmu') return analytics.rankings.danmuTotalUsers;
  return analytics.rankings.captainTotalUsers;
});

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

function movementIcon(change: 'new' | 'up' | 'down' | 'same') {
  if (change === 'up') return ArrowUp;
  if (change === 'down') return ArrowDown;
  if (change === 'new') return Plus;
  return Minus;
}

function movementText(change: 'new' | 'up' | 'down' | 'same'): string {
  return t(`rankings.${change}`);
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
  <section class="mx-auto flex h-full min-h-[560px] max-w-7xl flex-col bg-white dark:bg-zinc-900">
    <header
      class="flex min-h-14 flex-wrap items-center gap-3 border-b border-gray-200 py-2 dark:border-zinc-800"
    >
      <h2 class="mr-auto m-0 text-lg font-semibold text-gray-900 dark:text-zinc-100">
        {{ t('rankings.title') }}
      </h2>
      <DataRangeControl v-model="range" />
      <el-segmented
        v-model="limit"
        :options="[10, 20, 50, 100].map(value => ({ label: `TOP ${value}`, value }))"
        :aria-label="t('rankings.limitLabel')"
      />
      <el-button type="primary" :icon="Download" @click="exportVisible = true">
        {{ t('liveData.exportCsv') }}
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
      v-if="analytics.rankings && !analytics.rankings.isHistoricalDataAvailable"
      class="mt-4"
      :title="t('rankings.historyUnavailable')"
      type="warning"
      :closable="false"
    />

    <el-tabs v-model="activeTab" class="mt-3 shrink-0">
      <el-tab-pane :label="t('rankings.gift')" name="gift" />
      <el-tab-pane :label="t('rankings.danmaku')" name="danmu" />
      <el-tab-pane :label="t('rankings.captain')" name="captain" />
    </el-tabs>

    <div class="min-h-0 flex-1 overflow-x-auto">
      <el-table
        v-loading="analytics.rankPending"
        :data="rows"
        height="100%"
        class="min-w-[720px]"
        :empty-text="t('liveData.empty')"
        row-key="userUid"
      >
        <el-table-column
          prop="rank"
          :label="t('rankings.rank')"
          width="90"
          align="center"
        />
        <el-table-column :label="t('rankings.user')" min-width="220">
          <template #default="{ row }">
            <button
              type="button"
              class="font-medium text-pink-600 hover:underline focus-visible:outline-2 active:text-pink-700"
              @click="selectedUid = row.userUid"
            >
              {{ row.username }}
            </button>
            <span class="ml-2 text-xs tabular-nums text-gray-400">{{ row.userUid }}</span>
          </template>
        </el-table-column>
        <el-table-column :label="t('rankings.metric')" width="190" align="right">
          <template #default="{ row }">
            <strong class="tabular-nums">
              {{
                activeTab === 'danmu'
                  ? t('rankings.danmakuUnit', { count: formatCount(row.danmuCount) })
                  : formatMoneyMinor(row.amountMinor)
              }}
            </strong>
          </template>
        </el-table-column>
        <el-table-column :label="t('rankings.change')" width="150" align="center">
          <template #default="{ row }">
            <span
              class="inline-flex items-center gap-1"
              :class="
                analytics.highlightedUids.has(row.userUid)
                  ? row.change === 'up' || row.change === 'new'
                    ? 'font-semibold text-emerald-600'
                    : row.change === 'down'
                      ? 'font-semibold text-red-600'
                      : 'text-gray-500'
                  : 'text-gray-500'
              "
            >
              <el-icon><component :is="movementIcon(row.change)" /></el-icon>
              {{ movementText(row.change) }}
            </span>
          </template>
        </el-table-column>
      </el-table>
    </div>

    <footer
      class="flex min-h-11 items-center justify-between border-t border-gray-200 px-3 text-sm text-gray-600 dark:border-zinc-800 dark:text-zinc-400"
    >
      <span>{{ t('rankings.totalUsers', { count: formatCount(totalUsers) }) }}</span>
      <span>
        {{
          t('rankings.updatedAt', {
            time: analytics.rankings ? formatTimestamp(analytics.rankings.generatedAt) : '-'
          })
        }}
      </span>
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
