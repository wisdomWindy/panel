import { watch } from 'vue';
import { listenDesktop, type EventName, type EventPayloadMap } from '@/platform/desktop';
import { applyLocaleProjection, applyUiProjection } from '@/modules/application-shell/projection';
import { useApplicationShellStore } from '@/stores/application-shell';

type Unlisten = () => void;
type Listen = <K extends EventName>(name: K, listener: (payload: EventPayloadMap[K]) => void) => Promise<Unlisten>;

export function createApplicationShellRuntime(
  listen: Listen,
  store: ReturnType<typeof useApplicationShellStore>
) {
  let generation = 0;
  let active = false;
  let unlisteners: Unlisten[] = [];
  let timer: ReturnType<typeof setInterval> | null = null;
  let stopProjectionWatches: Array<() => void> = [];
  let pending: Promise<void> | null = null;

  async function startGeneration(localGeneration: number): Promise<void> {
    await store.bootstrap();
    if (generation !== localGeneration) return;
    if (store.locale) applyLocaleProjection(store.locale);
    if (store.previewUi) applyUiProjection(store.previewUi);
    stopProjectionWatches = [
      watch(() => store.locale, value => { if (value) applyLocaleProjection(value); }),
      watch(() => store.previewUi, value => { if (value) applyUiProjection(value); })
    ];
    const guarded = <K extends EventName>(handler: (payload: EventPayloadMap[K]) => void) =>
      (payload: EventPayloadMap[K]) => { if (generation === localGeneration) handler(payload); };
    const registrations = await Promise.allSettled([
      listen('shell_status_changed', guarded<'shell_status_changed'>(store.replaceMetrics)),
      listen('locale_changed', guarded<'locale_changed'>(store.replaceLocale)),
      listen('ui_preferences_changed', guarded<'ui_preferences_changed'>(store.replaceUi))
    ]);
    const completed = registrations.flatMap(result => result.status === 'fulfilled' ? [result.value] : []);
    const failed = registrations.find((result): result is PromiseRejectedResult => result.status === 'rejected');
    if (failed || generation !== localGeneration) {
      completed.forEach(unlisten => unlisten());
      if (failed) throw failed.reason;
      return;
    }
    unlisteners = completed;
    timer = setInterval(() => { void store.refreshMetrics(); }, 1_000);
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
    stopProjectionWatches.splice(0).forEach(stopWatch => stopWatch());
    if (timer) clearInterval(timer);
    timer = null;
    store.dispose();
  }

  return { start, stop, isActive: () => active };
}

let rootRuntime: ReturnType<typeof createApplicationShellRuntime> | null = null;

export function useApplicationShellRuntime() {
  if (rootRuntime) return rootRuntime;
  rootRuntime = createApplicationShellRuntime(listenDesktop, useApplicationShellStore());
  return rootRuntime;
}
