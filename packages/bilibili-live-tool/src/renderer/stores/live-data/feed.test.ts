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
      quantity: 1,
      valueMinor: 100
    });

    store.clearKind('gift');

    expect(store.events.map(event => event.kind)).toEqual(['danmu']);
  });
});
