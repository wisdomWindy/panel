import { createPinia, setActivePinia } from 'pinia';
import { beforeEach, describe, expect, it, vi } from 'vitest';

const invokeDesktop = vi.hoisted(() => vi.fn());

vi.mock('@/platform/desktop', async () => ({
  ...(await vi.importActual<typeof import('@/platform/desktop')>('@/platform/desktop')),
  invokeDesktop,
  normalizeDesktopError: (error: unknown) => ({
    code: 'test_error',
    message: error instanceof Error ? error.message : 'failed',
    retryable: false
  })
}));

import { useLiveRoomStore } from './room';

describe('live room store', () => {
  beforeEach(() => {
    setActivePinia(createPinia());
    invokeDesktop.mockReset();
  });

  it('connects with the broadcaster identity code and waits for the platform room id', async () => {
    invokeDesktop.mockResolvedValue({
      status: 'connecting',
      roomId: null,
      roomName: null,
      onlineCount: null,
      reconnectAttempt: 0,
      lastErrorCode: null
    });
    const store = useLiveRoomStore();

    await store.connect('ABC123');

    expect(invokeDesktop).toHaveBeenCalledWith('room_connect', { identityCode: 'ABC123' });
    expect(store.snapshot.roomId).toBeNull();
  });
});
