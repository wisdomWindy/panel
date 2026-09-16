import { listenDesktop, type EventName, type EventPayloadMap } from '@/platform/desktop';
import { useReleaseStore } from '@/stores/release';

type Unlisten = () => void;

export function createReleaseRuntime(
  listen: <K extends EventName>(name: K, listener: (payload: EventPayloadMap[K]) => void) => Promise<Unlisten>,
  store: ReturnType<typeof useReleaseStore>
) {
  let generation = 0;
  let active = false;
  let pending: Promise<void> | null = null;
  let unlisteners: Unlisten[] = [];

  async function startGeneration(localGeneration: number): Promise<void> {
    await store.bootstrap();
    if (generation !== localGeneration) return;
    const guarded = <K extends EventName>(handler: (value: EventPayloadMap[K]) => void) =>
      (value: EventPayloadMap[K]) => { if (generation === localGeneration) handler(value); };
    const registrations = await Promise.allSettled([
      listen('update://status', guarded<'update://status'>(store.replaceUpdate)),
      listen('plugin://status', guarded<'plugin://status'>(store.replacePlugin)),
      listen('plugin://crashed', guarded<'plugin://crashed'>(store.replacePlugin))
    ]);
    const completed = registrations.flatMap(result => result.status === 'fulfilled' ? [result.value] : []);
    const failure = registrations.find((result): result is PromiseRejectedResult => result.status === 'rejected');
    if (failure || generation !== localGeneration) {
      completed.forEach(unlisten => unlisten());
      if (failure) throw failure.reason;
      return;
    }
    unlisteners = completed;
    active = true;
  }

  function start(): Promise<void> {
    if (active) return Promise.resolve();
    if (pending) return pending;
    const localGeneration = ++generation;
    const current = startGeneration(localGeneration).finally(() => {
      if (pending === current) pending = null;
    });
    pending = current;
    return current;
  }

  function stop(): void {
    generation += 1;
    active = false;
    unlisteners.splice(0).forEach(unlisten => unlisten());
    store.dispose();
  }

  return { start, stop, isActive: () => active };
}

let runtime: ReturnType<typeof createReleaseRuntime> | null = null;

export function useReleaseRuntime() {
  if (!runtime) runtime = createReleaseRuntime(listenDesktop, useReleaseStore());
  return runtime;
}
