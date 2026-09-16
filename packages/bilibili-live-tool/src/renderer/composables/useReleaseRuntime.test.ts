import { createPinia, setActivePinia } from 'pinia';
import { beforeEach, describe, expect, it, vi } from 'vitest';
import type { EventName, EventPayloadMap } from '@/platform/desktop';
import { useReleaseStore } from '@/stores/release';
import { createReleaseRuntime } from './useReleaseRuntime';

beforeEach(() => setActivePinia(createPinia()));

describe('release runtime', () => {
  it('rolls back listeners when registration fails', async () => {
    const store = useReleaseStore();
    store.bootstrap = vi.fn().mockResolvedValue(undefined);
    const unlisten = vi.fn();
    let calls = 0;
    const listen = vi.fn(async <K extends EventName>(
      _name: K,
      _listener: (payload: EventPayloadMap[K]) => void
    ) => {
      calls += 1;
      if (calls === 2) throw new Error('registration failed');
      return unlisten;
    });
    const runtime = createReleaseRuntime(listen, store);

    await expect(runtime.start()).rejects.toThrow('registration failed');
    expect(unlisten).toHaveBeenCalledTimes(2);
    expect(runtime.isActive()).toBe(false);
  });

  it('disposes every registered listener', async () => {
    const store = useReleaseStore();
    store.bootstrap = vi.fn().mockResolvedValue(undefined);
    const unlisteners = [vi.fn(), vi.fn(), vi.fn()];
    let index = 0;
    const runtime = createReleaseRuntime(
      vi.fn(async () => unlisteners[index++]),
      store
    );

    await runtime.start();
    runtime.stop();

    expect(unlisteners.every(unlisten => unlisten.mock.calls.length === 1)).toBe(true);
  });
});
