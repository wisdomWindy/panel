import { beforeEach, describe, expect, it, vi } from 'vitest';
import { createPinia, setActivePinia } from 'pinia';
import { invokeDesktop, type DesktopError, type UiPreferences } from '@/platform/desktop';
import { useApplicationShellStore } from './application-shell';

vi.mock('@/platform/desktop', async () => {
  const actual = await vi.importActual<typeof import('@/platform/desktop')>('@/platform/desktop');
  return { ...actual, invokeDesktop: vi.fn() };
});

const ui = (theme: UiPreferences['theme'] = 'dark'): UiPreferences => ({
  theme, accentColor: '#FF6699', opacity: 0.95, fontSize: 'medium',
  layoutMode: 'default', customLayout: null, showDanmakuLevel: true,
  showDanmakuMedal: true, showDanmakuTime: true, maxDanmakuEntries: 1000
});

describe('application shell store', () => {
  beforeEach(() => {
    setActivePinia(createPinia());
    vi.mocked(invokeDesktop).mockReset();
  });

  it('loads metrics locale and saved UI as one latest generation', async () => {
    vi.mocked(invokeDesktop)
      .mockResolvedValueOnce({ buildChannel: 'free', schemaVersion: 1, status: 'ready' })
      .mockResolvedValueOnce({ memoryBytes: 1, cpuPercent: 2, observedAt: 3 })
      .mockResolvedValueOnce({ locale: 'zh-CN', followSystem: true })
      .mockResolvedValueOnce(ui());
    const store = useApplicationShellStore();
    expect(store.pending).toBe(true);
    await store.bootstrap();
    expect(invokeDesktop).toHaveBeenNthCalledWith(1, 'app_bootstrap', undefined);
    expect(store.metrics?.observedAt).toBe(3);
    expect(store.locale?.followSystem).toBe(true);
    expect(store.savedUi?.theme).toBe('dark');
    expect(store.pending).toBe(false);
  });

  it('keeps saved UI after a failed save and always clears operation pending', async () => {
    const failure: DesktopError = { code: 'invalid_ui_color', message: 'invalid', retryable: false };
    vi.mocked(invokeDesktop).mockRejectedValueOnce(failure);
    const store = useApplicationShellStore();
    store.replaceUi(ui('dark'));
    await expect(store.saveUi(ui('light'))).rejects.toEqual(failure);
    expect(store.savedUi?.theme).toBe('dark');
    expect(store.previewUi?.theme).toBe('dark');
    expect(store.pendingOperations.size).toBe(0);
  });

  it('revokes export grants in finally and deduplicates the same operation', async () => {
    let rejectExport!: (value: DesktopError) => void;
    vi.mocked(invokeDesktop)
      .mockResolvedValueOnce({ grantId: 'grant' as never, displayPath: 'config.json' })
      .mockReturnValueOnce(new Promise((_, reject) => { rejectExport = reject; }))
      .mockResolvedValueOnce({ revoked: true });
    const store = useApplicationShellStore();
    const first = store.exportConfig(['interfaceLayout']);
    const duplicate = store.exportConfig(['interfaceLayout']);
    expect(invokeDesktop).toHaveBeenCalledTimes(1);
    rejectExport({ code: 'config_export_failed', message: 'failed', retryable: true });
    await expect(Promise.all([first, duplicate])).rejects.toMatchObject({ code: 'config_export_failed' });
    expect(invokeDesktop).toHaveBeenLastCalledWith('path_revoke', { grantId: 'grant' });
  });

  it('treats a cancelled native config dialog as a no-op', async () => {
    vi.mocked(invokeDesktop).mockRejectedValueOnce({
      code: 'dialog_cancelled', message: 'cancelled', retryable: false
    });
    const store = useApplicationShellStore();
    await expect(store.importConfig()).resolves.toBeNull();
    expect(store.error).toBeNull();
    expect(store.pendingOperations.size).toBe(0);
  });

  it('keeps an imported transaction successful when preference refresh degrades', async () => {
    const imported = { revision: 'revision-1', importedScopes: ['interfaceLayout'] as const };
    vi.mocked(invokeDesktop)
      .mockResolvedValueOnce({ grantId: 'grant' as never, displayPath: 'config.json' })
      .mockResolvedValueOnce(imported)
      .mockRejectedValueOnce({ code: 'locale_refresh_failed', message: 'failed', retryable: true })
      .mockResolvedValueOnce(ui('light'))
      .mockResolvedValueOnce({ revoked: true });
    const store = useApplicationShellStore();

    await expect(store.importConfig()).resolves.toEqual(imported);

    expect(store.savedUi?.theme).toBe('light');
    expect(store.error?.code).toBe('locale_refresh_failed');
    expect(invokeDesktop).toHaveBeenLastCalledWith('path_revoke', { grantId: 'grant' });
    expect(store.pendingOperations.size).toBe(0);
  });

  it('advances the status clock even when metric sampling fails', async () => {
    vi.useFakeTimers();
    vi.setSystemTime(1_000);
    vi.mocked(invokeDesktop).mockRejectedValueOnce(new Error('metrics unavailable'));
    const store = useApplicationShellStore();
    vi.setSystemTime(2_000);
    await store.refreshMetrics();
    expect(store.statusObservedAt).toBe(2_000);
    vi.useRealTimers();
  });

  it('lets rapid UI previews run independently and keeps the latest result', async () => {
    let resolveFirst!: (value: UiPreferences) => void;
    let resolveSecond!: (value: UiPreferences) => void;
    vi.mocked(invokeDesktop)
      .mockReturnValueOnce(new Promise(resolve => { resolveFirst = resolve; }))
      .mockReturnValueOnce(new Promise(resolve => { resolveSecond = resolve; }));
    const store = useApplicationShellStore();
    store.replaceUi(ui('dark'));

    const first = store.previewPreferences(ui('light'));
    const second = store.previewPreferences(ui('system'));
    expect(invokeDesktop).toHaveBeenCalledTimes(2);

    resolveSecond(ui('system'));
    await second;
    resolveFirst(ui('light'));
    await first;

    expect(store.previewUi?.theme).toBe('system');
    expect(store.pendingOperations.has('previewUi')).toBe(false);
  });

  it('ignores a stale preview failure after a newer preview succeeds', async () => {
    let rejectFirst!: (value: DesktopError) => void;
    let resolveSecond!: (value: UiPreferences) => void;
    vi.mocked(invokeDesktop)
      .mockReturnValueOnce(new Promise((_, reject) => { rejectFirst = reject; }))
      .mockReturnValueOnce(new Promise(resolve => { resolveSecond = resolve; }));
    const store = useApplicationShellStore();
    store.replaceUi(ui('dark'));

    const first = store.previewPreferences(ui('light'));
    const second = store.previewPreferences(ui('system'));
    resolveSecond(ui('system'));
    await second;
    rejectFirst({ code: 'preview_failed', message: 'stale', retryable: true });
    await expect(first).resolves.toMatchObject({ theme: 'system' });

    expect(store.previewUi?.theme).toBe('system');
    expect(store.error).toBeNull();
  });
});
