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
      code: 'RECORD_UNAVAILABLE', message: 'resolver unavailable', retryable: false
    });
    const store = useMediaRecordingStore();
    await expect(store.start('1' as never)).rejects.toMatchObject({ code: 'RECORD_UNAVAILABLE' });
    expect(store.error?.code).toBe('RECORD_UNAVAILABLE');
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

  it('re-reads the status after deleting a file so its size stops counting', async () => {
    vi.mocked(invokeDesktop)
      .mockResolvedValueOnce(undefined as never)
      .mockResolvedValueOnce({
        state: 'idle', sessionId: null, roomId: null, elapsedSeconds: 0,
        fileSizeBytes: 0, diskRemainingBytes: 10, highlightCount: 0, lastErrorCode: null
      } as never);
    const store = useMediaRecordingStore();
    store.replaceStatus({
      state: 'idle', sessionId: null, roomId: null, elapsedSeconds: 0,
      fileSizeBytes: 2_800_000, diskRemainingBytes: 10, highlightCount: 0, lastErrorCode: null
    });

    await store.deleteRecording(1 as never);

    expect(invokeDesktop).toHaveBeenNthCalledWith(1, 'recording_delete', { recordingId: 1 });
    expect(invokeDesktop).toHaveBeenNthCalledWith(2, 'recording_status', undefined);
    expect(store.status?.fileSizeBytes).toBe(0);
  });
});
