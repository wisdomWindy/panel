import { invoke as tauriInvoke } from '@tauri-apps/api/core';
import { listen as tauriListen, type UnlistenFn } from '@tauri-apps/api/event';
import {
  isDesktopError,
  type CommandName,
  type CommandRequestMap,
  type CommandResultMap,
  type DesktopError,
  type EventName,
  type EventPayloadMap
} from './contracts';

export interface DesktopTransportDependencies {
  invoke<T>(command: string, args?: Record<string, unknown>): Promise<T>;
  listen<T>(event: string, handler: (event: { payload: T }) => void): Promise<UnlistenFn>;
}

export interface DesktopTransport {
  invoke<K extends CommandName>(
    name: K,
    request: CommandRequestMap[K]
  ): Promise<CommandResultMap[K]>;
  listen<K extends EventName>(
    name: K,
    listener: (payload: EventPayloadMap[K]) => void
  ): Promise<UnlistenFn>;
}

/**
 * Normalizes unknown failures before stores or views persist and display them.
 *
 * @example
 * error.value = normalizeDesktopError(reason);
 */
export function normalizeDesktopError(error: unknown): DesktopError {
  if (isDesktopError(error)) {
    return error;
  }

  return {
    code: 'desktop_transport_error',
    message:
      error instanceof Error
        ? error.message
        : typeof error === 'string'
          ? error
          : 'Desktop transport failed',
    retryable: true
  };
}

function desktopRuntimeUnavailable(): DesktopError {
  return {
    code: 'desktop_runtime_unavailable',
    message: '桌面运行时不可用，请在桌面应用中使用此功能',
    retryable: false
  };
}

function hasTauriBridge(): boolean {
  return '__TAURI_INTERNALS__' in globalThis;
}

/**
 * Creates the WebView's only native transport boundary.
 *
 * @see recording and configuration services migrate through this facade.
 */
export function createDesktopTransport(
  dependencies: DesktopTransportDependencies
): DesktopTransport {
  return {
    async invoke<K extends CommandName>(
      name: K,
      request: CommandRequestMap[K]
    ): Promise<CommandResultMap[K]> {
      try {
        if (request === undefined) {
          return await dependencies.invoke<CommandResultMap[K]>(name);
        }

        return await dependencies.invoke<CommandResultMap[K]>(name, { request });
      } catch (error) {
        throw normalizeDesktopError(error);
      }
    },
    async listen<K extends EventName>(
      name: K,
      listener: (payload: EventPayloadMap[K]) => void
    ): Promise<UnlistenFn> {
      try {
        return await dependencies.listen<EventPayloadMap[K]>(name, event =>
          listener(event.payload)
        );
      } catch (error) {
        throw normalizeDesktopError(error);
      }
    }
  };
}

const desktopTransport = createDesktopTransport({
  invoke: <T>(command: string, args?: Record<string, unknown>) => {
    if (!hasTauriBridge()) return Promise.reject(desktopRuntimeUnavailable());
    return tauriInvoke<T>(command, args);
  },
  listen: <T>(event: string, handler: (event: { payload: T }) => void) => {
    if (!hasTauriBridge()) return Promise.reject(desktopRuntimeUnavailable());
    return tauriListen<T>(event, tauriEvent => handler({ payload: tauriEvent.payload }));
  }
});

/**
 * Invokes a command from the closed desktop command map.
 *
 * @example
 * const status = await invokeDesktop('app_status', undefined);
 */
export const invokeDesktop = desktopTransport.invoke;

/**
 * Subscribes to a typed desktop event and returns its lifecycle cleanup function.
 *
 * @example
 * const unlisten = await listenDesktop('app_status_changed', handleStatus);
 * unlisten();
 */
export const listenDesktop = desktopTransport.listen;
