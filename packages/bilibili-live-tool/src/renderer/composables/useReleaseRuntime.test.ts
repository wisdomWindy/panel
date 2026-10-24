import { createPinia, setActivePinia } from 'pinia';
import { beforeEach, describe, expect, it, vi } from 'vitest';
import type { EventName, EventPayloadMap } from '@/platform/desktop';
import { useReleaseStore } from '@/stores/release';
import { createReleaseRuntime } from './useReleaseRuntime';

beforeEach(() => setActivePinia(createPinia()));

describe('release runtime', () => {
  it('stays inactive when listener registration fails', async () => {
    const store = useReleaseStore();
    store.bootstrap = vi.fn().mockResolvedValue(undefined);
    store.checkUpdate = vi.fn().mockResolvedValue(undefined);
    const listen = vi.fn(async <K extends EventName>(
      _name: K,
      _listener: (payload: EventPayloadMap[K]) => void
    ) => {
      throw new Error('registration failed');
    });
    const runtime = createReleaseRuntime(listen, store);

    await expect(runtime.start()).rejects.toThrow('registration failed');
    expect(store.checkUpdate).not.toHaveBeenCalled();
    expect(runtime.isActive()).toBe(false);
  });

  it('checks for updates once after bootstrap and listener registration', async () => {
    const store = useReleaseStore();
    const order: string[] = [];
    store.bootstrap = vi.fn(async () => { order.push('bootstrap'); });
    store.checkUpdate = vi.fn(async () => { order.push('check'); });
    const runtime = createReleaseRuntime(
      vi.fn(async name => {
        order.push(name);
        return vi.fn();
      }),
      store
    );

    await runtime.start();
    await runtime.start();

    expect(order).toEqual([
      'bootstrap',
      'update://status',
      'check'
    ]);
    expect(runtime.isActive()).toBe(true);
  });

  it('keeps the runtime active when the automatic check fails', async () => {
    const store = useReleaseStore();
    store.bootstrap = vi.fn().mockResolvedValue(undefined);
    store.checkUpdate = vi.fn().mockRejectedValue(new Error('offline'));
    const runtime = createReleaseRuntime(vi.fn(async () => vi.fn()), store);

    await expect(runtime.start()).resolves.toBeUndefined();

    expect(runtime.isActive()).toBe(true);
  });

  it('disposes every registered listener', async () => {
    const store = useReleaseStore();
    store.bootstrap = vi.fn().mockResolvedValue(undefined);
    store.checkUpdate = vi.fn().mockResolvedValue(undefined);
    const unlisteners = [vi.fn()];
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
