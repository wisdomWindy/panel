import { computed, ref } from 'vue';
import { defineStore } from 'pinia';
import {
  invokeDesktop,
  normalizeDesktopError,
  type DataRange,
  type DesktopError,
  type PathGrantId,
  type RankingSnapshot,
  type StatSnapshot
} from '@/platform/desktop';
import { adaptRankingSnapshot, adaptStatSnapshot } from '@/modules/live-data/adapters';

type Timer = ReturnType<typeof setTimeout>;

export const useLiveAnalyticsStore = defineStore('live-analytics', () => {
  const statistics = ref<StatSnapshot | null>(null);
  const rankings = ref<RankingSnapshot | null>(null);
  const statPending = ref(false);
  const rankPending = ref(false);
  const exportPending = ref(false);
  const error = ref<DesktopError | null>(null);
  const highlightedUids = ref<ReadonlySet<string>>(new Set());
  const exportedPaths = ref<readonly string[]>([]);
  let statGeneration = 0;
  let rankGeneration = 0;
  let highlightTimer: Timer | null = null;
  let schedule: typeof setTimeout = setTimeout;
  let cancel: typeof clearTimeout = clearTimeout;

  const hasStatistics = computed(() => statistics.value !== null);

  async function queryStatistics(range: DataRange, refresh = false): Promise<void> {
    const generation = ++statGeneration;
    statPending.value = true;
    error.value = null;
    try {
      const value = await invokeDesktop(refresh ? 'stat_refresh' : 'stat_query', { range });
      if (generation === statGeneration) statistics.value = adaptStatSnapshot(value);
    } catch (value) {
      if (generation === statGeneration) error.value = normalizeDesktopError(value);
      throw value;
    } finally {
      if (generation === statGeneration) statPending.value = false;
    }
  }

  async function queryRankings(range: DataRange): Promise<void> {
    const generation = ++rankGeneration;
    rankPending.value = true;
    error.value = null;
    try {
      const value = await invokeDesktop('rank_query', { range });
      if (generation === rankGeneration) replaceRankings(value);
    } catch (value) {
      if (generation === rankGeneration) error.value = normalizeDesktopError(value);
      throw value;
    } finally {
      if (generation === rankGeneration) rankPending.value = false;
    }
  }

  function replaceStatistics(value: StatSnapshot): void {
    statistics.value = adaptStatSnapshot(value);
  }

  function replaceRankings(value: RankingSnapshot): void {
    rankings.value = adaptRankingSnapshot(value);
    highlightedUids.value = new Set(
      [...value.gift, ...value.danmu, ...value.captain]
        .filter(row => row.change !== 'same')
        .map(row => row.userUid)
    );
    if (highlightTimer) cancel(highlightTimer);
    highlightTimer = schedule(() => {
      highlightedUids.value = new Set();
      highlightTimer = null;
    }, 3000);
  }

  function configureClock(nextSchedule: typeof setTimeout, nextCancel: typeof clearTimeout): void {
    schedule = nextSchedule;
    cancel = nextCancel;
  }

  function dispose(): void {
    statGeneration += 1;
    rankGeneration += 1;
    if (highlightTimer) cancel(highlightTimer);
    highlightTimer = null;
    highlightedUids.value = new Set();
  }

  async function exportData(
    types: readonly ('statistics' | 'rankings')[],
    range: DataRange
  ): Promise<void> {
    if (exportPending.value) return;
    exportPending.value = true;
    error.value = null;
    exportedPaths.value = [];
    const grants: PathGrantId[] = [];
    try {
      const paths: string[] = [];
      for (const type of types) {
        const purpose = type === 'statistics' ? 'statExport' : 'rankExport';
        const grant = await invokeDesktop('path_select', { purpose });
        grants.push(grant.grantId);
        const result =
          type === 'statistics'
            ? await invokeDesktop('stat_export', { range, grantId: grant.grantId })
            : await invokeDesktop('rank_export', { range, grantId: grant.grantId });
        paths.push(result.displayPath);
      }
      exportedPaths.value = paths;
    } catch (value) {
      error.value = normalizeDesktopError(value);
      throw value;
    } finally {
      await Promise.all(
        grants.map(grantId => invokeDesktop('path_revoke', { grantId }).catch(() => undefined))
      );
      exportPending.value = false;
    }
  }

  return {
    statistics,
    rankings,
    statPending,
    rankPending,
    exportPending,
    error,
    highlightedUids,
    exportedPaths,
    hasStatistics,
    queryStatistics,
    queryRankings,
    replaceStatistics,
    replaceRankings,
    exportData,
    configureClock,
    dispose
  };
});
