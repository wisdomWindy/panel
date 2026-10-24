import { describe, expect, it, vi } from 'vitest';
import type { EventName, EventPayloadMap } from '@/platform/desktop';
import { createAiRuntime, type AiRuntimeTargets } from './useAiRuntime';

function targets(): AiRuntimeTargets {
  return {
    delta: vi.fn(),
    finished: vi.fn(),
    asr: vi.fn(),
    transcript: vi.fn(),
    bootstrap: vi.fn().mockResolvedValue(undefined),
    dispose: vi.fn()
  };
}

describe('shared AI runtime', () => {
  it('registers the AI listeners once and disposes all of them', async () => {
    const listeners = new Map<string, (payload: unknown) => void>();
    const unlisten = vi.fn();
    const listen = vi.fn(async <K extends EventName>(
      name: K,
      listener: (payload: EventPayloadMap[K]) => void
    ) => {
      listeners.set(name, listener as (payload: unknown) => void);
      return unlisten;
    });
    const target = targets();
    const runtime = createAiRuntime(listen, target);

    await Promise.all([runtime.start(), runtime.start()]);

    expect([...listeners.keys()]).toEqual([
      'ai_generation_delta',
      'ai_generation_finished',
      'asr_status_changed',
      'asr_transcript'
    ]);
    expect(target.bootstrap).toHaveBeenCalledTimes(1);
    listeners.get('asr_transcript')?.('你好，直播间');
    expect(target.transcript).toHaveBeenCalledWith('你好，直播间');

    runtime.stop();
    expect(unlisten).toHaveBeenCalledTimes(4);
    expect(target.dispose).toHaveBeenCalledTimes(1);
    listeners.get('asr_transcript')?.('stale');
    expect(target.transcript).toHaveBeenCalledTimes(1);
  });

  it('removes partial listeners when registration fails', async () => {
    const unlisten = vi.fn();
    let call = 0;
    const listen = vi.fn(async () => {
      call += 1;
      if (call === 2) throw new Error('listener failed');
      return unlisten;
    });
    const runtime = createAiRuntime(listen, targets());

    await expect(runtime.start()).rejects.toThrow('listener failed');
    expect(unlisten).toHaveBeenCalledTimes(3);
    expect(runtime.isActive()).toBe(false);
  });
});
