import { createPinia, setActivePinia } from 'pinia';
import { beforeEach, describe, expect, it, vi } from 'vitest';
import type { UpdateSnapshot } from '@/platform/desktop';

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

function currentUpdate(currentVersion = '0.1.0'): UpdateSnapshot {
  return {
    currentVersion,
    channel: 'free',
    status: 'current',
    candidate: null,
    progressPercent: null,
    errorCode: null
  };
}

describe('release store', () => {
  beforeEach(() => {
    setActivePinia(createPinia());
    invokeDesktop.mockReset();
  });

  it('ignores an update check that resolves after the store is disposed', async () => {
    let resolveRequest!: (value: UpdateSnapshot) => void;
    invokeDesktop.mockReturnValue(new Promise(resolve => { resolveRequest = resolve; }));
    const store = useReleaseStore();

    const request = store.checkUpdate();
    store.dispose();
    resolveRequest(currentUpdate());
    await request;

    expect(store.update).toBeNull();
    expect(store.pending.size).toBe(0);
  });

  it('does not let a disposed request clear a newer check pending state', async () => {
    let resolveFirst!: (value: UpdateSnapshot) => void;
    let resolveSecond!: (value: UpdateSnapshot) => void;
    invokeDesktop
      .mockReturnValueOnce(new Promise(resolve => { resolveFirst = resolve; }))
      .mockReturnValueOnce(new Promise(resolve => { resolveSecond = resolve; }));
    const store = useReleaseStore();

    const firstRequest = store.checkUpdate();
    store.dispose();
    const secondRequest = store.checkUpdate();
    resolveFirst(currentUpdate('0.1.0'));
    await firstRequest;

    expect(store.pending.has('checkUpdate')).toBe(true);
    expect(store.update).toBeNull();

    resolveSecond(currentUpdate('0.2.0'));
    await secondRequest;
    expect(store.update?.currentVersion).toBe('0.2.0');
    expect(store.pending.size).toBe(0);
  });

  it('does not restore an error from a request rejected after dispose', async () => {
    let rejectRequest!: (reason: unknown) => void;
    invokeDesktop.mockReturnValue(new Promise((_, reject) => { rejectRequest = reject; }));
    const store = useReleaseStore();

    const request = store.checkUpdate();
    store.dispose();
    rejectRequest(new Error('offline'));
    await expect(request).rejects.toMatchObject({ code: 'test_error' });

    expect(store.error).toBeNull();
    expect(store.pending.size).toBe(0);
  });
});
