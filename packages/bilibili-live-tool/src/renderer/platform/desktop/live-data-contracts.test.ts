import { describe, expect, it } from 'vitest';
import {
  LIVE_DATA_COMMAND_NAMES,
  LIVE_DATA_EVENT_NAMES,
  type CommandRequestMap,
  type CommandResultMap,
  type EventId,
  type GiftId,
  type EventPayloadMap,
  type RoomId,
  type UserUid
} from './contracts';

describe('live data desktop contracts', () => {
  it('keeps the approved command and event surface closed', () => {
    expect(LIVE_DATA_COMMAND_NAMES).toHaveLength(20);
    expect(LIVE_DATA_EVENT_NAMES).toHaveLength(9);
    expect(LIVE_DATA_COMMAND_NAMES[0]).toBe('room_connect');
    expect(LIVE_DATA_COMMAND_NAMES.at(-1)).toBe('rank_export');
  });

  it('carries money in minor units and custom ranges in epoch milliseconds', () => {
    const request: CommandRequestMap['stat_query'] = {
      range: { kind: 'custom', startAt: 1_700_000_000_000, endAt: 1_700_003_600_000 }
    };
    const event: EventPayloadMap['gift_received'] = {
      kind: 'gift',
      eventId: 'event-1' as EventId,
      roomId: '123' as RoomId,
      occurredAt: 1_700_000_000_000,
      audience: {
        userUid: '456' as UserUid,
        username: 'viewer',
        userLevel: null,
        medal: null,
        tags: []
      },
      giftId: 'gift-31036' as GiftId,
      giftName: 'gift',
      quantity: 2,
      valueMinor: 300
    };
    const result: CommandResultMap['stat_query'] = {
      range: request.range,
      metrics: {
        onlineCount: 0,
        danmuCount: 0,
        danmuRate: 0,
        giftValueMinor: event.valueMinor,
        captainCount: 0,
        filteredCount: 0
      },
      trendPoints: [],
      generatedAt: 1_700_000_000_000,
      isHistoricalDataAvailable: false,
      isStale: false
    };

    expect(result.metrics.giftValueMinor).toBe(300);
    expect(event.giftId).toBe('gift-31036');
  });
});
