import { beforeEach, describe, expect, it } from 'vitest';
import { createPinia, setActivePinia } from 'pinia';
import type { DanmuEvent, GiftEvent } from '@/platform/desktop';
import { useLiveFeedStore } from './feed';

const audience = {
  userUid: '2' as DanmuEvent['audience']['userUid'],
  username: 'viewer',
  userLevel: null,
  medal: null,
  tags: []
} as const;

describe('live feed store', () => {
  beforeEach(() => setActivePinia(createPinia()));

  it('clears one event kind without dropping the rest of the accepted feed', () => {
    const store = useLiveFeedStore();
    store.accept({
      kind: 'danmu',
      eventId: 'danmu-1' as DanmuEvent['eventId'],
      roomId: '1' as DanmuEvent['roomId'],
      occurredAt: 1,
      audience,
      content: 'hello'
    });
    store.accept({
      kind: 'gift',
      eventId: 'gift-1' as GiftEvent['eventId'],
      roomId: '1' as GiftEvent['roomId'],
      occurredAt: 2,
      audience,
      giftId: 'gift-31036' as GiftEvent['giftId'],
      giftName: '辣条',
      giftIconUrl: null,
      quantity: 1,
      valueMinor: 100
    });

    store.clearKind('gift');

    expect(store.events.map(event => event.kind)).toEqual(['danmu']);
  });

  it('keeps session boundaries out of the danmaku feed', () => {
    const store = useLiveFeedStore();

    // PRD 9.1.1: LIVE_START/LIVE_END reset or seal the session; they are not
    // danmaku rows.
    store.accept({
      kind: 'sessionStarted',
      eventId: 'session-start:1:10' as DanmuEvent['eventId'],
      roomId: '1' as DanmuEvent['roomId'],
      occurredAt: 10,
      audience
    });
    store.accept({
      kind: 'sessionEnded',
      eventId: 'session-end:1:20' as DanmuEvent['eventId'],
      roomId: '1' as DanmuEvent['roomId'],
      occurredAt: 20,
      audience
    });

    expect(store.events).toEqual([]);
  });

  it('withdraws paid messages from the feed without touching the rest', () => {
    const store = useLiveFeedStore();
    store.accept({
      kind: 'superChat',
      eventId: 'sc-1' as DanmuEvent['eventId'],
      roomId: '1' as DanmuEvent['roomId'],
      occurredAt: 10,
      audience,
      messageId: 77,
      content: 'paid message',
      valueMinor: 3000,
      durationSeconds: 60
    });
    store.accept({
      kind: 'danmu',
      eventId: 'danmu-1' as DanmuEvent['eventId'],
      roomId: '1' as DanmuEvent['roomId'],
      occurredAt: 11,
      audience,
      content: 'hello'
    });

    store.accept({
      kind: 'superChatDeleted',
      eventId: 'sc-del-1' as DanmuEvent['eventId'],
      roomId: '1' as DanmuEvent['roomId'],
      occurredAt: 12,
      audience,
      messageIds: [77]
    });

    expect(store.events.map(event => event.kind)).toEqual(['danmu']);
  });
});
