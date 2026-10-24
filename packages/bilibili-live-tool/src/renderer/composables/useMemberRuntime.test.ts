import { describe, expect, it, vi } from 'vitest';
import type { EventName, EventPayloadMap } from '@/platform/desktop';
import { createMemberRuntime, type MemberRuntimeTargets } from './useMemberRuntime';

function targets(): MemberRuntimeTargets {
  return {
    qr: vi.fn(),
    loggedIn: vi.fn(),
    status: vi.fn(),
    modules: vi.fn(),
    installed: vi.fn(),
    alert: vi.fn(),
    action: vi.fn(),
    bootstrap: vi.fn().mockResolvedValue(undefined),
    dispose: vi.fn()
  };
}

describe('member runtime', () => {
  it('registers only member listeners once and disposes all of them', async () => {
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
    const runtime = createMemberRuntime(listen, target);

    await Promise.all([runtime.start(), runtime.start()]);

    expect([...listeners.keys()]).toEqual([
      'auth_qrcode_status',
      'auth_logged_in',
      'membership_expired',
      'membership_renewed',
      'module_download_progress',
      'module_installed',
      'moderation_alert',
      'moderation_action_result'
    ]);
    expect(listen).toHaveBeenCalledTimes(8);
    expect(target.bootstrap).toHaveBeenCalledTimes(1);

    runtime.stop();
    expect(unlisten).toHaveBeenCalledTimes(8);
    expect(target.dispose).toHaveBeenCalledTimes(1);
  });

  it('removes partial listeners when one registration fails', async () => {
    const unlisten = vi.fn();
    let call = 0;
    const listen = vi.fn(async () => {
      call += 1;
      if (call === 4) throw new Error('listener failed');
      return unlisten;
    });
    const runtime = createMemberRuntime(listen, targets());

    await expect(runtime.start()).rejects.toThrow('listener failed');
    expect(unlisten).toHaveBeenCalledTimes(7);
    expect(runtime.isActive()).toBe(false);
  });
});
