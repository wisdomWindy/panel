import { beforeEach, describe, expect, it, vi } from 'vitest';
import { createPinia, setActivePinia } from 'pinia';
import {
  invokeDesktop,
  type DesktopError,
  type PathGrantId,
  type RankingSnapshot,
  type StatSnapshot
} from '@/platform/desktop';
import { useLiveAnalyticsStore } from './analytics';

vi.mock('@/platform/desktop', async () => {
  const actual = await vi.importActual<typeof import('@/platform/desktop')>('@/platform/desktop');
  return { ...actual, invokeDesktop: vi.fn() };
});

const rankSnapshot = (): RankingSnapshot => ({
  range: { kind: 'liveSession' },
  gift: [
    {
      rank: 1,
      userUid: '1' as RankingSnapshot['gift'][number]['userUid'],
      username: 'one',
      amountMinor: 100,
      previousRank: null,
      change: 'new'
    }
  ],
  danmu: [],
  captain: [],
  giftTotalUsers: 1,
  danmuTotalUsers: 0,
  captainTotalUsers: 0,
  generatedAt: 1,
  isHistoricalDataAvailable: true,
  isStale: false
});

describe('live analytics store', () => {
  beforeEach(() => {
    setActivePinia(createPinia());
    vi.mocked(invokeDesktop).mockReset();
  });

  it('replaces snapshots immutably and expires rank highlights after three seconds', () => {
    vi.useFakeTimers();
    const store = useLiveAnalyticsStore();
    const source = rankSnapshot();
    store.replaceRankings(source);
    expect(store.rankings).not.toBe(source);
    expect(store.highlightedUids.has('1')).toBe(true);
    vi.advanceTimersByTime(3000);
    expect(store.highlightedUids.size).toBe(0);
    vi.useRealTimers();
  });

  it('can invalidate an in-flight query generation on dispose', async () => {
    const desktop = await import('@/platform/desktop');
    let resolve!: (value: StatSnapshot) => void;
    vi.mocked(desktop.invokeDesktop).mockReturnValueOnce(
      new Promise(done => {
        resolve = done;
      })
    );
    const store = useLiveAnalyticsStore();
    const pending = store.queryStatistics({ kind: 'liveSession' });
    store.dispose();
    resolve({
      range: { kind: 'liveSession' },
      metrics: {
        onlineCount: 1,
        danmuCount: 1,
        danmuRate: 1,
        giftValueMinor: 0,
        captainCount: 0,
        likeCount: 0,        filteredCount: 0
      },
      trendPoints: [],
      generatedAt: 1,
      isHistoricalDataAvailable: true,
      isStale: false
    });
    await pending;
    expect(store.statistics).toBeNull();
  });

  it('keeps the latest range response when requests resolve out of order', async () => {
    const desktop = await import('@/platform/desktop');
    let resolveFirst!: (value: StatSnapshot) => void;
    let resolveSecond!: (value: StatSnapshot) => void;
    vi.mocked(desktop.invokeDesktop)
      .mockReturnValueOnce(
        new Promise(done => {
          resolveFirst = done;
        })
      )
      .mockReturnValueOnce(
        new Promise(done => {
          resolveSecond = done;
        })
      );
    const store = useLiveAnalyticsStore();
    const first = store.queryStatistics({ kind: 'today' });
    const second = store.queryStatistics({ kind: 'last7Days' });
    const snapshot = (generatedAt: number): StatSnapshot => ({
      range: { kind: 'last7Days' },
      metrics: {
        onlineCount: generatedAt,
        danmuCount: 0,
        danmuRate: 0,
        giftValueMinor: 0,
        captainCount: 0,
        likeCount: 0,        filteredCount: 0
      },
      trendPoints: [],
      generatedAt,
      isHistoricalDataAvailable: true,
      isStale: false
    });
    resolveSecond(snapshot(2));
    await second;
    resolveFirst(snapshot(1));
    await first;
    expect(store.statistics?.generatedAt).toBe(2);
  });

  it('revokes an issued path grant when export fails', async () => {
    const grantId = 'grant-1' as PathGrantId;
    const failure: DesktopError = {
      code: 'STORAGE_HISTORICAL_UNAVAILABLE',
      message: '历史数据暂不可用',
      retryable: false
    };
    vi.mocked(invokeDesktop)
      .mockResolvedValueOnce({ grantId, displayPath: 'D:/exports/statistics.csv' })
      .mockRejectedValueOnce(failure)
      .mockResolvedValueOnce({ revoked: true });
    const store = useLiveAnalyticsStore();

    await expect(store.exportData(['statistics'], { kind: 'today' })).rejects.toEqual(failure);

    expect(invokeDesktop).toHaveBeenNthCalledWith(3, 'path_revoke', { grantId });
    expect(store.exportPending).toBe(false);
    expect(store.exportedPaths).toEqual([]);
  });
});
