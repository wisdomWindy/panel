import { describe, expect, it, vi } from 'vitest';
import {
  createDesktopTransport,
  invokeDesktop,
  normalizeDesktopError,
  type DesktopTransportDependencies
} from './transport';
import { isDesktopError } from './contracts';

describe('desktop transport', () => {
  it('invokes a foundation command with its request envelope', async () => {
    const invoke = vi.fn().mockResolvedValue({
      buildChannel: 'free',
      schemaVersion: 1,
      status: 'ready'
    });
    const transport = createDesktopTransport({
      invoke,
      listen: vi.fn()
    } as DesktopTransportDependencies);

    const result = await transport.invoke('app_bootstrap', undefined);

    expect(result).toEqual({
      buildChannel: 'free',
      schemaVersion: 1,
      status: 'ready'
    });
    expect(invoke).toHaveBeenCalledWith('app_bootstrap');
  });

  it('normalizes rejected transport errors without hiding the failure', async () => {
    const transport = createDesktopTransport({
      invoke: vi.fn().mockRejectedValue(new Error('bridge unavailable')),
      listen: vi.fn()
    } as DesktopTransportDependencies);

    await expect(transport.invoke('app_status', undefined)).rejects.toMatchObject({
      code: 'desktop_transport_error',
      message: 'bridge unavailable',
      retryable: true
    });

    try {
      await transport.invoke('app_status', undefined);
    } catch (error) {
      expect(isDesktopError(error)).toBe(true);
    }
  });

  it('normalizes unknown values at consuming error boundaries', () => {
    expect(normalizeDesktopError({ reason: 'unknown' })).toEqual({
      code: 'desktop_transport_error',
      message: 'Desktop transport failed',
      retryable: true
    });
  });

  it('forwards event payloads and returns the real unsubscribe function', async () => {
    const unsubscribe = vi.fn();
    const listener = vi.fn();
    const listen = vi.fn(async (_name, handler) => {
      handler({ payload: { status: 'degraded', retryable: true } });
      return unsubscribe;
    });
    const transport = createDesktopTransport({
      invoke: vi.fn(),
      listen
    } as DesktopTransportDependencies);

    const unlisten = await transport.listen('app_status_changed', listener);

    expect(listener).toHaveBeenCalledWith({
      status: 'degraded',
      retryable: true
    });
    unlisten();
    expect(unsubscribe).toHaveBeenCalledOnce();
  });

  it('fails closed with a structured error when the desktop bridge is absent', async () => {
    await expect(invokeDesktop('app_status', undefined)).rejects.toMatchObject({
      code: 'desktop_runtime_unavailable',
      message: '桌面运行时不可用，请在桌面应用中使用此功能',
      retryable: false
    });
  });
});
