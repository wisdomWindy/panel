import { beforeEach, describe, expect, it, vi } from 'vitest';
import { createPinia, setActivePinia } from 'pinia';
import { invokeDesktop } from '@/platform/desktop';
import { useMediaRecordingStore } from './recording';

vi.mock('@/platform/desktop', async () => {
  const actual = await vi.importActual<typeof import('@/platform/desktop')>('@/platform/desktop');
  return { ...actual, invokeDesktop: vi.fn() };
});

describe('media recording store', () => {
  beforeEach(() => {
    setActivePinia(createPinia());
    vi.mocked(invokeDesktop).mockReset();
  });

  it('maps rejected commands and clears operation state in finally', async () => {
    vi.mocked(invokeDesktop).mockRejectedValueOnce({
      code: 'media_unavailable', message: 'resolver unavailable', retryable: false
    });
    const store = useMediaRecordingStore();
    await expect(store.start('1' as never)).rejects.toMatchObject({ code: 'media_unavailable' });
    expect(store.error?.code).toBe('media_unavailable');
    expect(store.pendingOperations.size).toBe(0);
  });

  it('rejects malformed status events without replacing valid state', () => {
    const store = useMediaRecordingStore();
    store.replaceStatus({
      state: 'idle', sessionId: null, roomId: null, elapsedSeconds: 0,
      fileSizeBytes: 0, diskRemainingBytes: 10, highlightCount: 0, lastErrorCode: null
    });
    store.replaceStatus({ state: 'recording' });
    expect(store.status?.state).toBe('idle');
    expect(store.error?.code).toBe('desktop_transport_error');
  });

  it('revokes temporary recording directory grants when the draft is discarded', async () => {
    vi.mocked(invokeDesktop)
      .mockResolvedValueOnce({ grantId: 'directory-grant', displayPath: 'D:/recordings' } as never)
      .mockResolvedValueOnce({ revoked: true } as never);
    const store = useMediaRecordingStore();
    await store.selectDirectory();
    await store.discardDirectorySelection();
    expect(invokeDesktop).toHaveBeenNthCalledWith(2, 'path_revoke', { grantId: 'directory-grant' });
  });
});
