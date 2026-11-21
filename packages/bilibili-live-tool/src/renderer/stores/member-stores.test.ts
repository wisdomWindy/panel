import { createPinia, setActivePinia } from 'pinia';
import { beforeEach, describe, expect, it, vi } from 'vitest';

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

import { useAIStore } from './ai';
import { useMembershipStore } from './membership';

const memberStatus = {
  shellState: 'loggedOut',
  account: null,
  entitlement: null,
  capabilities: { moderation: 'unavailable' },
  serviceErrorCode: null
};

describe('member stores', () => {
  beforeEach(() => {
    setActivePinia(createPinia());
    invokeDesktop.mockReset();
  });

  it('keeps unrelated AI errors out of model configuration and generation', async () => {
    const store = useAIStore();
    invokeDesktop.mockRejectedValueOnce(new Error('log query failed'));
    await expect(
      store.loadLogs({ userUid: null, startedAt: null, endedAt: null })
    ).rejects.toBeDefined();
    expect(store.errorFor('loadLogs')?.message).toBe('log query failed');
    expect(store.errorFor('saveProfile', 'deleteProfile', 'setDefaultProfile', 'testProfile')).toBeNull();
    expect(store.errorFor('generate')).toBeNull();
    store.clear();
    expect(store.errorFor('loadLogs')).toBeNull();
  });

  it('deduplicates an in-flight membership bootstrap', async () => {
    let resolveRequest: (value: unknown) => void = () => undefined;
    invokeDesktop.mockReturnValue(new Promise(resolve => { resolveRequest = resolve; }));
    const store = useMembershipStore();

    const first = store.bootstrap();
    const second = store.bootstrap();
    expect(invokeDesktop).toHaveBeenCalledTimes(1);
    resolveRequest(memberStatus);
    await Promise.all([first, second]);

    expect(store.status.shellState).toBe('loggedOut');
    expect(store.pendingOperations.size).toBe(0);
  });

  it('accepts only ordered deltas for the current generation', () => {
    const store = useAIStore();
    store.acceptGeneration({
      generationId: 'generation-1',
      status: 'streaming',
      result: 'A',
      lastSequence: 0,
      errorCode: null
    });

    store.acceptDelta({ generationId: 'generation-1' as never, sequence: 2, delta: 'ignored' });
    store.acceptDelta({ generationId: 'other' as never, sequence: 1, delta: 'ignored' });
    store.acceptDelta({ generationId: 'generation-1' as never, sequence: 1, delta: 'B' });

    expect(store.generation?.result).toBe('AB');
    expect(store.generation?.lastSequence).toBe(1);
  });
});
