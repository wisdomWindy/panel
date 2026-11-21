import { beforeEach, describe, expect, it, vi } from 'vitest';
import { createPinia, setActivePinia } from 'pinia';
import {
  invokeDesktop,
  type DesktopError,
  type PathGrantId,
  type SensitiveWord
} from '@/platform/desktop';
import { useLiveSafetyStore } from './safety';

vi.mock('@/platform/desktop', async () => {
  const actual = await vi.importActual<typeof import('@/platform/desktop')>('@/platform/desktop');
  return { ...actual, invokeDesktop: vi.fn() };
});

describe('live safety store', () => {
  beforeEach(() => {
    setActivePinia(createPinia());
    vi.mocked(invokeDesktop).mockReset();
  });

  it('keeps the table unchanged until a mutation and authoritative refresh complete', async () => {
    const desktop = await import('@/platform/desktop');
    const stored: SensitiveWord = {
      id: 1,
      word: '规则',
      category: 'custom',
      matchType: 'exact',
      isEnabled: true
    };
    let resolveAdd!: (value: SensitiveWord) => void;
    vi.mocked(desktop.invokeDesktop)
      .mockReturnValueOnce(
        new Promise(done => {
          resolveAdd = done;
        })
      )
      .mockResolvedValueOnce({ items: [stored], todayHitCount: 0 });
    const store = useLiveSafetyStore();
    const saving = store.saveSensitiveWord({
      word: stored.word,
      category: stored.category,
      matchType: stored.matchType,
      isEnabled: stored.isEnabled
    });

    expect(store.mutationPending).toBe(true);
    expect(store.sensitiveWords.items).toEqual([]);
    await expect(store.toggleSensitiveWord(1, false)).rejects.toEqual({
      code: 'operation_pending',
      message: '操作正在进行，请稍后重试',
      retryable: true
    });
    expect(invokeDesktop).toHaveBeenCalledTimes(1);
    resolveAdd(stored);
    await saving;
    expect(store.sensitiveWords.items).toEqual([stored]);
    expect(store.mutationPending).toBe(false);
  });

  it('revokes a path grant when a safety export fails', async () => {
    const grantId = 'grant-1' as PathGrantId;
    const failure: DesktopError = {
      code: 'STORAGE_PERMISSION_DENIED',
      message: '没有文件写入权限，请更换位置',
      retryable: false
    };
    vi.mocked(invokeDesktop)
      .mockResolvedValueOnce({ grantId, displayPath: 'D:/exports/blacklist.csv' })
      .mockRejectedValueOnce(failure)
      .mockResolvedValueOnce({ revoked: true });
    const store = useLiveSafetyStore();

    await expect(store.exportBlacklist()).rejects.toEqual(failure);

    expect(invokeDesktop).toHaveBeenNthCalledWith(3, 'path_revoke', { grantId });
    expect(store.mutationPending).toBe(false);
  });
});
