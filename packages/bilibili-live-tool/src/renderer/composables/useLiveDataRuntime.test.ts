import { describe, expect, it, vi } from 'vitest';
import type { EventName, EventPayloadMap } from '@/platform/desktop';
import { createLiveDataRuntime, type LiveDataRuntimeTargets } from './useLiveDataRuntime';

const targets = (): LiveDataRuntimeTargets => ({
  roomStatus: vi.fn(),
  liveEvent: vi.fn(),
  compileProgress: vi.fn(),
  statistics: vi.fn(),
  rankings: vi.fn(),
  exportDone: vi.fn(),
  dispose: vi.fn()
});

describe('live data root runtime', () => {
  it('owns one set of nine listeners and cleans each one once', async () => {
    const stops = Array.from({ length: 9 }, () => vi.fn());
    let index = 0;
    const listen = vi.fn(
      async <K extends EventName>(_name: K, _listener: (payload: EventPayloadMap[K]) => void) =>
        stops[index++]
    );
    const runtime = createLiveDataRuntime(listen, targets());

    await Promise.all([runtime.start(), runtime.start()]);
    expect(listen).toHaveBeenCalledTimes(9);
    expect(runtime.isActive()).toBe(true);
    runtime.stop();
    stops.forEach(stop => expect(stop).toHaveBeenCalledOnce());
  });

  it('disposes subscriptions that resolve after stop', async () => {
    const stops = Array.from({ length: 9 }, () => vi.fn());
    let release!: () => void;
    const gate = new Promise<void>(resolve => {
      release = resolve;
    });
    let index = 0;
    const listen = vi.fn(
      async <K extends EventName>(_name: K, _listener: (payload: EventPayloadMap[K]) => void) => {
        const stop = stops[index++];
        await gate;
        return stop;
      }
    );
    const runtime = createLiveDataRuntime(listen, targets());
    const starting = runtime.start();
    runtime.stop();
    release();
    await starting;
    expect(runtime.isActive()).toBe(false);
    stops.forEach(stop => expect(stop).toHaveBeenCalledOnce());
  });
});
