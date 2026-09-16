import { beforeEach, describe, expect, it, vi } from 'vitest';
import { createPinia, setActivePinia } from 'pinia';
import { listenDesktop } from '@/platform/desktop';
import { useApplicationShellStore } from '@/stores/application-shell';
import { useApplicationShellRuntime } from './useApplicationShellRuntime';

vi.mock('@/platform/desktop', async () => {
  const actual = await vi.importActual<typeof import('@/platform/desktop')>('@/platform/desktop');
  return { ...actual, listenDesktop: vi.fn() };
});

describe('application shell runtime', () => {
  beforeEach(() => {
    setActivePinia(createPinia());
    vi.mocked(listenDesktop).mockReset();
  });

  it('owns exactly three listeners and disposes all handles', async () => {
    const unlisten = vi.fn();
    vi.mocked(listenDesktop).mockResolvedValue(unlisten);
    const store = useApplicationShellStore();
    vi.spyOn(store, 'bootstrap').mockResolvedValue();
    vi.spyOn(store, 'refreshMetrics').mockResolvedValue();
    const runtime = useApplicationShellRuntime();
    await runtime.start();
    expect(vi.mocked(listenDesktop).mock.calls.map(call => call[0])).toEqual([
      'shell_status_changed', 'locale_changed', 'ui_preferences_changed'
    ]);
    runtime.stop();
    expect(unlisten).toHaveBeenCalledTimes(3);
  });
});
