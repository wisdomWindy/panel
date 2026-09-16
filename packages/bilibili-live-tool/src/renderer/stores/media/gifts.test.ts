import { beforeEach, describe, expect, it, vi } from 'vitest';
import { createPinia, setActivePinia } from 'pinia';
import { invokeDesktop, type GiftNotifySnapshot, type PathGrantId } from '@/platform/desktop';
import { useMediaGiftsStore } from './gifts';

vi.mock('@/platform/desktop', async () => {
  const actual = await vi.importActual<typeof import('@/platform/desktop')>('@/platform/desktop');
  return { ...actual, invokeDesktop: vi.fn() };
});

const notifySnapshot = (): GiftNotifySnapshot => ({
  config: {
    isEnabled: true,
    highValueThresholdMinor: 10000,
    methods: { systemNotification: true, localPopup: true, sound: true },
    comboEnabled: true,
    comboWindowSeconds: 10,
    popupPosition: 'topRight',
    popupDurationSeconds: 5
  },
  totalValueMinor: 0,
  donorCount: 0,
  giftCount: 0,
  latestGifts: [],
  updatedAt: 1
});

describe('media gifts store', () => {
  beforeEach(() => {
    setActivePinia(createPinia());
    vi.mocked(invokeDesktop).mockReset();
  });

  it('retains the previous snapshot when an event payload is malformed', () => {
    const store = useMediaGiftsStore();
    store.replaceNotify(notifySnapshot());
    store.replaceNotify({ ...notifySnapshot(), totalValueMinor: Number.NaN });
    expect(store.notifySnapshot?.totalValueMinor).toBe(0);
    expect(store.error?.code).toBe('desktop_transport_error');
  });

  it('deduplicates import and revokes its path grant after failure', async () => {
    const grantId = 'grant-1' as PathGrantId;
    let rejectImport!: (reason: unknown) => void;
    vi.mocked(invokeDesktop)
      .mockResolvedValueOnce({ grantId, displayPath: 'sounds.json' })
      .mockReturnValueOnce(new Promise((_, reject) => { rejectImport = reject; }))
      .mockResolvedValueOnce({ revoked: true });
    const store = useMediaGiftsStore();
    const first = store.importSounds();
    const duplicate = store.importSounds();
    await vi.waitFor(() => expect(invokeDesktop).toHaveBeenCalledTimes(2));
    rejectImport({ code: 'media_io', message: 'failed', retryable: true });
    await expect(first).rejects.toMatchObject({ code: 'media_io' });
    await expect(duplicate).rejects.toMatchObject({ code: 'media_io' });
    expect(invokeDesktop).toHaveBeenLastCalledWith('path_revoke', { grantId });
    expect(store.pendingOperations.size).toBe(0);
  });
});
