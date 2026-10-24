import {
  listenDesktop,
  type EventName,
  type EventPayloadMap
} from '@/platform/desktop';
import { useMembershipStore } from '@/stores/membership';
import { useModerationStore } from '@/stores/moderation';

type Unlisten = () => void;
type Listen = <K extends EventName>(
  name: K,
  listener: (payload: EventPayloadMap[K]) => void
) => Promise<Unlisten>;

export interface MemberRuntimeTargets {
  qr(payload: EventPayloadMap['auth_qrcode_status']): void;
  loggedIn(payload: EventPayloadMap['auth_logged_in']): void;
  status(payload: EventPayloadMap['membership_expired']): void;
  modules(payload: EventPayloadMap['module_download_progress']): void;
  installed(payload: EventPayloadMap['module_installed']): void;
  alert(payload: EventPayloadMap['moderation_alert']): void;
  action(payload: EventPayloadMap['moderation_action_result']): void;
  bootstrap(): Promise<void>;
  dispose(): void;
}

export function createMemberRuntime(listen: Listen, targets: MemberRuntimeTargets) {
  let generation = 0;
  let active = false;
  let unlisteners: Unlisten[] = [];
  let pending: Promise<void> | null = null;

  async function start(): Promise<void> {
    if (active) return;
    if (pending) return pending;
    const localGeneration = ++generation;
    const guarded = <K extends EventName>(handler: (payload: EventPayloadMap[K]) => void) =>
      (payload: EventPayloadMap[K]) => {
        if (generation === localGeneration) handler(payload);
      };
    const bindings: ReadonlyArray<() => Promise<Unlisten>> = [
      () => listen('auth_qrcode_status', guarded<'auth_qrcode_status'>(targets.qr)),
      () => listen('auth_logged_in', guarded<'auth_logged_in'>(targets.loggedIn)),
      () => listen('membership_expired', guarded<'membership_expired'>(targets.status)),
      () => listen('membership_renewed', guarded<'membership_renewed'>(targets.status)),
      () => listen('module_download_progress', guarded<'module_download_progress'>(targets.modules)),
      () => listen('module_installed', guarded<'module_installed'>(targets.installed)),
      () => listen('moderation_alert', guarded<'moderation_alert'>(targets.alert)),
      () => listen('moderation_action_result', guarded<'moderation_action_result'>(targets.action))
    ];
    const current = (async () => {
      const results = await Promise.allSettled(bindings.map(bind => bind()));
      const registered = results.flatMap(result => result.status === 'fulfilled' ? [result.value] : []);
      const rejected = results.find(
        (result): result is PromiseRejectedResult => result.status === 'rejected'
      );
      if (rejected || generation !== localGeneration) {
        registered.forEach(unlisten => unlisten());
        if (rejected) throw rejected.reason;
        return;
      }
      unlisteners = registered;
      await targets.bootstrap();
      if (generation !== localGeneration) {
        unlisteners.splice(0).forEach(unlisten => unlisten());
        return;
      }
      active = true;
    })().finally(() => {
      if (pending === current) pending = null;
    });
    pending = current;
    return current;
  }

  function stop(): void {
    generation += 1;
    active = false;
    unlisteners.splice(0).forEach(unlisten => unlisten());
    targets.dispose();
  }

  return { start, stop, isActive: () => active };
}

let rootRuntime: ReturnType<typeof createMemberRuntime> | null = null;

export function useMemberRuntime() {
  if (rootRuntime) return rootRuntime;
  const membership = useMembershipStore();
  const moderation = useModerationStore();
  rootRuntime = createMemberRuntime(listenDesktop, {
    qr: membership.replaceQrStatus,
    loggedIn: () => { void membership.bootstrap(); },
    status: membership.replaceStatus,
    modules: payload => membership.replaceModules(payload.modules),
    installed: membership.acceptModule,
    alert: moderation.replaceAlert,
    action: moderation.replaceAlert,
    bootstrap: async () => {
      await membership.bootstrap();
      await Promise.allSettled([
        membership.loadModules(),
        membership.loadOrders(),
        moderation.bootstrap()
      ]);
    },
    dispose: () => {
      membership.dispose();
      moderation.dispose();
    }
  });
  return rootRuntime;
}
