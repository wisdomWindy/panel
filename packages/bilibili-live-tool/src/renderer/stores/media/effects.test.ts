import { beforeEach, describe, expect, it, vi } from 'vitest';
import { createPinia, setActivePinia } from 'pinia';
import { invokeDesktop, type CaptainEffectSnapshot, type DesktopError } from '@/platform/desktop';
import { useMediaEffectsStore } from './effects';

vi.mock('@/platform/desktop', async () => {
  const actual = await vi.importActual<typeof import('@/platform/desktop')>('@/platform/desktop');
  return { ...actual, invokeDesktop: vi.fn() };
});

function snapshot(updatedAt: number): CaptainEffectSnapshot {
  return {
    config: {
      presetId: 'gold',
      durationSeconds: 5,
      position: 'center',
      windowWidth: 560,
      windowHeight: 280,
      primaryColor: '#FFD700',
      secondaryColor: '#FFA500',
      firstSound: null,
      renewalSound: null,
      comboSound: null,
      volume: 0.7,
      comboEnabled: true,
      comboWindowSeconds: 10
    },
    presets: [],
    updatedAt
  };
}

describe('media effects store', () => {
  beforeEach(() => {
    setActivePinia(createPinia());
    vi.mocked(invokeDesktop).mockReset();
  });

  it('deduplicates ordinary loads and keeps the latest forced refresh', async () => {
    let resolveFirst!: (value: CaptainEffectSnapshot) => void;
    let resolveSecond!: (value: CaptainEffectSnapshot) => void;
    vi.mocked(invokeDesktop)
      .mockReturnValueOnce(
        new Promise(done => {
          resolveFirst = done;
        })
      )
      .mockReturnValueOnce(
        new Promise(done => {
          resolveSecond = done;
        })
      );
    const store = useMediaEffectsStore();
    const first = store.load();
    const duplicate = store.load();
    expect(invokeDesktop).toHaveBeenCalledTimes(1);
    const latest = store.refresh();
    expect(invokeDesktop).toHaveBeenCalledTimes(2);
    resolveSecond(snapshot(2));
    await latest;
    resolveFirst(snapshot(1));
    await Promise.all([first, duplicate]);
    expect(store.snapshot?.updatedAt).toBe(2);
    expect(store.pending).toBe(false);
  });

  it('maps failures and always releases mutation pending state', async () => {
    const failure: DesktopError = { code: 'RECORD_INVALID_PARAM', message: 'invalid', retryable: false };
    vi.mocked(invokeDesktop).mockRejectedValueOnce(failure);
    const store = useMediaEffectsStore();
    await expect(store.save(snapshot(1).config)).rejects.toEqual(failure);
    expect(store.error).toEqual(failure);
    expect(store.savePending).toBe(false);
  });
});
