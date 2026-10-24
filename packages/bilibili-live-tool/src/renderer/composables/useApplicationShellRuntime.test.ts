import { afterEach, beforeEach, describe, expect, it, vi } from 'vitest';
import { createPinia, setActivePinia } from 'pinia';
import { listenDesktop } from '@/platform/desktop';
import { useApplicationShellStore } from '@/stores/application-shell';
import { createApplicationShellRuntime } from './useApplicationShellRuntime';

const projectionMocks = vi.hoisted(() => ({
  applyLocaleProjection: vi.fn(),
  applyUiProjection: vi.fn()
}));

vi.mock('@/platform/desktop', async () => {
  const actual = await vi.importActual<typeof import('@/platform/desktop')>('@/platform/desktop');
  return { ...actual, listenDesktop: vi.fn() };
});

vi.mock('@/modules/application-shell/projection', () => projectionMocks);

describe('application shell runtime', () => {
  beforeEach(() => {
    setActivePinia(createPinia());
    vi.mocked(listenDesktop).mockReset();
    projectionMocks.applyLocaleProjection.mockReset();
    projectionMocks.applyUiProjection.mockReset();
  });

  afterEach(() => vi.unstubAllGlobals());

  it('owns exactly three listeners and disposes all handles', async () => {
    const unlisten = vi.fn();
    vi.mocked(listenDesktop).mockResolvedValue(unlisten);
    const store = useApplicationShellStore();
    vi.spyOn(store, 'bootstrap').mockResolvedValue();
    vi.spyOn(store, 'refreshMetrics').mockResolvedValue();
    const runtime = createApplicationShellRuntime(listenDesktop, store);
    await runtime.start();
    expect(vi.mocked(listenDesktop).mock.calls.map(call => call[0])).toEqual([
      'shell_status_changed', 'locale_changed', 'ui_preferences_changed'
    ]);
    runtime.stop();
    expect(unlisten).toHaveBeenCalledTimes(3);
  });

  it('reprojects system theme changes and removes the media listener on stop', async () => {
    const systemThemeListeners = new Set<() => void>();
    const mediaQuery = {
      matches: false,
      addEventListener: vi.fn((_name: string, listener: () => void) => systemThemeListeners.add(listener)),
      removeEventListener: vi.fn((_name: string, listener: () => void) => systemThemeListeners.delete(listener))
    };
    vi.stubGlobal('window', { matchMedia: vi.fn(() => mediaQuery) });
    vi.mocked(listenDesktop).mockResolvedValue(vi.fn());
    const store = useApplicationShellStore();
    const preferences = {
      theme: 'system', accentColor: '#FB7299', opacity: 0.95, fontSize: 'medium',
      layoutMode: 'default', customLayout: null, showDanmakuLevel: true,
      showDanmakuMedal: true, showDanmakuTime: true, maxDanmakuEntries: 1000
    } as const;
    store.replaceUi(preferences);
    vi.spyOn(store, 'bootstrap').mockResolvedValue();
    vi.spyOn(store, 'refreshMetrics').mockResolvedValue();
    const runtime = createApplicationShellRuntime(listenDesktop, store);

    await runtime.start();
    projectionMocks.applyUiProjection.mockClear();
    systemThemeListeners.forEach(listener => listener());
    expect(projectionMocks.applyUiProjection).toHaveBeenCalledOnce();
    expect(projectionMocks.applyUiProjection).toHaveBeenCalledWith(preferences);

    store.replacePreviewUi({ ...preferences, theme: 'light' });
    projectionMocks.applyUiProjection.mockClear();
    systemThemeListeners.forEach(listener => listener());
    expect(projectionMocks.applyUiProjection).not.toHaveBeenCalled();

    runtime.stop();
    expect(mediaQuery.removeEventListener).toHaveBeenCalledOnce();
    expect(systemThemeListeners).toHaveLength(0);
  });
});
