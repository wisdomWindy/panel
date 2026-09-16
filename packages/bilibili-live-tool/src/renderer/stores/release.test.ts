import { createPinia, setActivePinia } from 'pinia';
import { beforeEach, describe, expect, it, vi } from 'vitest';
import type { PluginSnapshot } from '@/platform/desktop';

const invokeDesktop = vi.hoisted(() => vi.fn());

vi.mock('@/platform/desktop', async () => ({
  ...await vi.importActual<typeof import('@/platform/desktop')>('@/platform/desktop'),
  invokeDesktop,
  normalizeDesktopError: (error: unknown) => ({
    code: 'test_error',
    message: error instanceof Error ? error.message : 'failed',
    retryable: false
  })
}));

import { useReleaseStore } from './release';

function plugin(status: PluginSnapshot['status']): PluginSnapshot {
  return {
    id: 'captain-effect',
    name: 'Captain Effect',
    version: '1.0.0',
    status,
    permissions: ['effect'],
    errorCode: null
  };
}

describe('release store', () => {
  beforeEach(() => {
    setActivePinia(createPinia());
    invokeDesktop.mockReset();
  });

  it('keeps the prior plugin snapshot until native toggle succeeds', async () => {
    let resolveRequest!: (value: PluginSnapshot) => void;
    invokeDesktop.mockReturnValue(new Promise(resolve => { resolveRequest = resolve; }));
    const store = useReleaseStore();
    store.replacePlugin(plugin('disabled'));
    const initialPlugin = store.plugins[0];
    if (!initialPlugin) throw new Error('expected the seeded plugin');

    const request = store.togglePlugin(initialPlugin);
    expect(store.plugins[0]?.status).toBe('disabled');
    expect(store.pending.has('plugin:captain-effect')).toBe(true);

    resolveRequest(plugin('enabled'));
    await request;
    expect(store.plugins[0]?.status).toBe('enabled');
    expect(store.pending.size).toBe(0);
  });

  it('removes a plugin only after native uninstall confirms success', async () => {
    invokeDesktop.mockResolvedValueOnce(false).mockResolvedValueOnce(true);
    const store = useReleaseStore();
    store.replacePlugin(plugin('disabled'));

    await store.uninstallPlugin('captain-effect');
    expect(store.plugins).toHaveLength(1);
    await store.uninstallPlugin('captain-effect');
    expect(store.plugins).toHaveLength(0);
  });
});
