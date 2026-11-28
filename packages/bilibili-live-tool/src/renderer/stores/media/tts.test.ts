import { beforeEach, describe, expect, it, vi } from 'vitest';
import { createPinia, setActivePinia } from 'pinia';
import {
  invokeDesktop,
  listenDesktop,
  type TtsSnapshot,
  type TtsStatus
} from '@/platform/desktop';
import { useMediaTtsStore } from './tts';

vi.mock('@/platform/desktop', async () => {
  const actual = await vi.importActual<typeof import('@/platform/desktop')>('@/platform/desktop');
  return { ...actual, invokeDesktop: vi.fn(), listenDesktop: vi.fn() };
});

const status = (state: TtsStatus['state'] = 'idle'): TtsStatus => ({
  state, currentText: null, pendingCount: 0, activeEngine: 'system', fallbackReasonCode: null
});

const snapshot = (): TtsSnapshot => ({
  config: {
    isEnabled: true, selectedEngine: 'system', selectedVoiceId: null,
    rate: 1, volume: 0.8, eventKinds: ['danmu'], identityKinds: ['ordinary'],
    templates: { danmu: '{用户}说：{内容}', gift: '{用户}送出{礼物}×{数量}', captain: '{用户}开通{等级}' }
  },
  voices: [], status: status(), updatedAt: 1
});

describe('media TTS store', () => {
  beforeEach(() => {
    setActivePinia(createPinia());
    vi.mocked(invokeDesktop).mockReset();
    vi.mocked(listenDesktop).mockReset();
  });

  it('deduplicates the same operation and commits one status response', async () => {
    vi.mocked(invokeDesktop).mockResolvedValueOnce(snapshot());
    const store = useMediaTtsStore();
    await store.load();
    let resolve!: (value: TtsStatus) => void;
    vi.mocked(invokeDesktop).mockReturnValueOnce(new Promise(done => { resolve = done; }));
    const first = store.pause(true);
    const duplicate = store.pause(true);
    expect(invokeDesktop).toHaveBeenCalledTimes(2);
    resolve(status('paused'));
    await Promise.all([first, duplicate]);
    expect(store.snapshot?.status.state).toBe('paused');
    expect(store.pendingOperations.size).toBe(0);
  });

  it('ignores a late load response after dispose', async () => {
    let resolve!: (value: TtsSnapshot) => void;
    vi.mocked(invokeDesktop).mockReturnValueOnce(new Promise(done => { resolve = done; }));
    const store = useMediaTtsStore();
    const pending = store.load();
    store.dispose();
    resolve(snapshot());
    await pending;
    expect(store.snapshot).toBeNull();
    expect(store.pending).toBe(false);
  });

  it('subscribes to environment progress once, not once per preparation run', async () => {
    const stop = vi.fn();
    vi.mocked(listenDesktop).mockResolvedValue(stop as never);
    const store = useMediaTtsStore();

    // The shell attaches at start-up and every preparation run used to attach
    // again; stacked listeners each flipped `preparing`, which produced one
    // "语音服务已就绪" toast per listener.
    await store.attachEnvironmentEvents();
    await store.attachEnvironmentEvents();
    await store.listenEnvironmentProgress();

    expect(listenDesktop).toHaveBeenCalledTimes(1);

    store.dispose();
    expect(stop).toHaveBeenCalledTimes(1);
  });
});
