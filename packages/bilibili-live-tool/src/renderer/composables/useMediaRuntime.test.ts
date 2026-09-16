import { describe, expect, it, vi } from 'vitest';
import type { EventName, EventPayloadMap } from '@/platform/desktop';
import { createMediaRuntime, type MediaRuntimeTargets } from './useMediaRuntime';

type Handler = (payload: never) => void;

function targets(): MediaRuntimeTargets {
  return {
    giftAlert: vi.fn(),
    giftNotify: vi.fn(),
    ttsStatus: vi.fn(),
    recordingStatus: vi.fn(),
    recordingCompleted: vi.fn(),
    highlightCreated: vi.fn(),
    dispose: vi.fn()
  };
}

describe('media root runtime', () => {
  it('registers exactly six listeners and deduplicates concurrent starts', async () => {
    const names: EventName[] = [];
    const listen = vi.fn(async <K extends EventName>(
      name: K,
      _listener: (payload: EventPayloadMap[K]) => void
    ) => {
      names.push(name);
      return vi.fn();
    });
    const runtime = createMediaRuntime(listen, targets());
    const first = runtime.start();
    const duplicate = runtime.start();
    await Promise.all([first, duplicate]);
    expect(listen).toHaveBeenCalledTimes(6);
    expect(names).toEqual([
      'gift_alert_trigger',
      'gift_notify_updated',
      'tts_status_changed',
      'recording_status_changed',
      'recording_completed',
      'highlight_created'
    ]);
    expect(runtime.isActive()).toBe(true);
  });

  it('cleans successful registrations when one listener rejects', async () => {
    const cleanup = Array.from({ length: 5 }, () => vi.fn());
    let index = 0;
    const listen = vi.fn(() => {
      const current = index++;
      return current === 2
        ? Promise.reject(new Error('registration failed'))
        : Promise.resolve(cleanup[current < 2 ? current : current - 1]);
    });
    const runtime = createMediaRuntime(listen as never, targets());
    await expect(runtime.start()).rejects.toThrow('registration failed');
    cleanup.forEach(unlisten => expect(unlisten).toHaveBeenCalledOnce());
    expect(runtime.isActive()).toBe(false);
  });

  it('disposes late registrations and blocks events after stop', async () => {
    const handlers = new Map<EventName, Handler>();
    const cleanups = Array.from({ length: 6 }, () => vi.fn());
    const resolvers: Array<(value: () => void) => void> = [];
    let index = 0;
    const listen = vi.fn(<K extends EventName>(
      name: K,
      listener: (payload: EventPayloadMap[K]) => void
    ) => {
      handlers.set(name, listener as Handler);
      const cleanup = cleanups[index++];
      return new Promise<() => void>(resolve => resolvers.push(() => resolve(cleanup)));
    });
    const target = targets();
    const runtime = createMediaRuntime(listen, target);
    const pending = runtime.start();
    runtime.stop();
    handlers.get('tts_status_changed')?.({
      state: 'idle', currentText: null, pendingCount: 0,
      activeEngine: 'system', fallbackReasonCode: null
    } as never);
    resolvers.forEach(resolve => resolve(() => undefined));
    await pending;
    cleanups.forEach(unlisten => expect(unlisten).toHaveBeenCalledOnce());
    expect(target.ttsStatus).not.toHaveBeenCalled();
    expect(target.dispose).toHaveBeenCalledOnce();
    expect(runtime.isActive()).toBe(false);
  });
});
