import { invokeDesktop, type ConfigKey } from '@/platform/desktop';

/**
 * The operator connects once; the identity code and the auto-connect
 * preference live in the local key/value store, so the next launch
 * re-establishes the long connection by itself. The connection itself already
 * survives 下播 (the platform only sends `INTERACTION_END` on heartbeat expiry
 * or `app/end`), so once restored it keeps monitoring across stream sessions.
 */
const IDENTITY_KEY = 'live.identity' as ConfigKey;
const AUTO_CONNECT_KEY = 'live.auto_connect' as ConfigKey;
const SCOPE = 'global' as const;

export interface StoredRoomSession {
  readonly identityCode: string | null;
  readonly autoConnect: boolean;
}

const EMPTY_SESSION: StoredRoomSession = { identityCode: null, autoConnect: false };

function trimmedOrNull(value: string | undefined): string | null {
  const text = value?.trim() ?? '';
  return text.length > 0 ? text : null;
}

/** Never throws: a missing or unreadable entry simply means "not remembered". */
export async function loadRoomSession(): Promise<StoredRoomSession> {
  try {
    const [identity, autoConnect] = await Promise.all([
      invokeDesktop('config_get', { key: IDENTITY_KEY, scope: SCOPE }),
      invokeDesktop('config_get', { key: AUTO_CONNECT_KEY, scope: SCOPE })
    ]);
    return {
      identityCode: trimmedOrNull(identity.value),
      autoConnect: autoConnect.value === '1'
    };
  } catch {
    return EMPTY_SESSION;
  }
}

/** Remembers the code so the next launch reconnects without any click. */
export async function rememberRoomSession(identityCode: string): Promise<void> {
  await invokeDesktop('config_set', { key: IDENTITY_KEY, scope: SCOPE, value: identityCode });
  await invokeDesktop('config_set', { key: AUTO_CONNECT_KEY, scope: SCOPE, value: '1' });
}

/** Stops auto-connecting and drops the stored code. */
export async function forgetRoomSession(): Promise<void> {
  await invokeDesktop('config_set', { key: IDENTITY_KEY, scope: SCOPE, value: '' });
  await invokeDesktop('config_set', { key: AUTO_CONNECT_KEY, scope: SCOPE, value: '0' });
}
