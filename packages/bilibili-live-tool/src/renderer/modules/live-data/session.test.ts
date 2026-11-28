import { beforeEach, describe, expect, it, vi } from 'vitest';
import { invokeDesktop } from '@/platform/desktop';
import { forgetRoomSession, loadRoomSession, rememberRoomSession } from './session';

vi.mock('@/platform/desktop', async () => {
  const actual = await vi.importActual<typeof import('@/platform/desktop')>('@/platform/desktop');
  return { ...actual, invokeDesktop: vi.fn() };
});

describe('room session persistence', () => {
  beforeEach(() => {
    vi.mocked(invokeDesktop).mockReset();
  });

  it('restores a trimmed identity code and the auto-connect flag', async () => {
    vi.mocked(invokeDesktop)
      .mockResolvedValueOnce({ value: '  ABC123  ' } as never)
      .mockResolvedValueOnce({ value: '1' } as never);

    await expect(loadRoomSession()).resolves.toEqual({
      identityCode: 'ABC123',
      autoConnect: true
    });
  });

  it('treats a blank stored code as not remembered', async () => {
    vi.mocked(invokeDesktop)
      .mockResolvedValueOnce({ value: '   ' } as never)
      .mockResolvedValueOnce({ value: '0' } as never);

    await expect(loadRoomSession()).resolves.toEqual({
      identityCode: null,
      autoConnect: false
    });
  });

  it('never throws when the local store is unavailable', async () => {
    vi.mocked(invokeDesktop).mockRejectedValue({
      code: 'CONFIG_STORAGE_UNAVAILABLE',
      message: 'store unavailable',
      retryable: true
    });

    await expect(loadRoomSession()).resolves.toEqual({
      identityCode: null,
      autoConnect: false
    });
  });

  it('remembers both the code and the auto-connect flag', async () => {
    vi.mocked(invokeDesktop).mockResolvedValue({ revision: '1' } as never);

    await rememberRoomSession('ABC123');

    expect(invokeDesktop).toHaveBeenNthCalledWith(
      1,
      'config_set',
      expect.objectContaining({ key: 'live.identity', scope: 'global', value: 'ABC123' })
    );
    expect(invokeDesktop).toHaveBeenNthCalledWith(
      2,
      'config_set',
      expect.objectContaining({ key: 'live.auto_connect', scope: 'global', value: '1' })
    );
  });

  it('forgets the code and disables auto-connect', async () => {
    vi.mocked(invokeDesktop).mockResolvedValue({ revision: '2' } as never);

    await forgetRoomSession();

    expect(invokeDesktop).toHaveBeenNthCalledWith(
      1,
      'config_set',
      expect.objectContaining({ key: 'live.identity', value: '' })
    );
    expect(invokeDesktop).toHaveBeenNthCalledWith(
      2,
      'config_set',
      expect.objectContaining({ key: 'live.auto_connect', value: '0' })
    );
  });
});
