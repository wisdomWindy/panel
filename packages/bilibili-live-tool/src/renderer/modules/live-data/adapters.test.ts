import { describe, expect, it } from 'vitest';
import type { DanmuEvent, StatSnapshot } from '@/platform/desktop';
import { adaptLiveEvent, adaptStatSnapshot } from './adapters';

describe('live data adapters', () => {
  it('copies nested server snapshots and keeps integer-minor money semantics', () => {
    const source: StatSnapshot = {
      range: { kind: 'liveSession' },
      metrics: {
        onlineCount: 1,
        danmuCount: 2,
        danmuRate: 0.5,
        giftValueMinor: 1234,
        captainCount: 3,
        filteredCount: 4
      },
      trendPoints: [
        {
          occurredAt: 10,
          onlineCount: 1,
          danmuCount: 2,
          danmuRate: 0.5,
          giftValueMinor: 1234,
          captainCount: 3,
          filteredCount: 4
        }
      ],
      generatedAt: 10,
      isHistoricalDataAvailable: true,
      isStale: false
    };
    const result = adaptStatSnapshot(source);
    expect(result).toEqual(source);
    expect(result).not.toBe(source);
    expect(result.metrics).not.toBe(source.metrics);
    expect(result.trendPoints[0]).not.toBe(source.trendPoints[0]);
    expect(result.metrics.giftValueMinor).toBe(1234);
  });

  it('copies accepted typed events without sharing audience state', () => {
    const event: DanmuEvent = {
      kind: 'danmu',
      eventId: 'event-1' as DanmuEvent['eventId'],
      roomId: '1' as DanmuEvent['roomId'],
      occurredAt: 10,
      content: 'hello',
      audience: {
        userUid: '2' as DanmuEvent['audience']['userUid'],
        username: 'viewer',
        userLevel: null,
        medal: { name: '粉丝牌', level: 8 },
        tags: ['captain']
      }
    };
    const copy = adaptLiveEvent(event);
    expect(copy.audience).not.toBe(event.audience);
    expect(copy).toEqual(event);
  });
});
