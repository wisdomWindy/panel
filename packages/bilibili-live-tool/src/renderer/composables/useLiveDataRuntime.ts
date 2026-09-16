import {
  listenDesktop,
  type EventName,
  type EventPayloadMap,
  type LiveEvent
} from '@/platform/desktop';
import { useLiveAnalyticsStore } from '@/stores/live-data/analytics';
import { useLiveFeedStore } from '@/stores/live-data/feed';
import { useLiveRoomStore } from '@/stores/live-data/room';
import { useLiveSafetyStore } from '@/stores/live-data/safety';

type Unlisten = () => void;
type Listen = <K extends EventName>(
  name: K,
  listener: (payload: EventPayloadMap[K]) => void
) => Promise<Unlisten>;

export interface LiveDataRuntimeTargets {
  roomStatus(payload: EventPayloadMap['room_status_changed']): void;
  liveEvent(payload: LiveEvent): void;
  compileProgress(payload: EventPayloadMap['sensitive_word_compile_progress']): void;
  statistics(payload: EventPayloadMap['stat_updated']): void;
  rankings(payload: EventPayloadMap['rank_updated']): void;
  exportDone(payload: EventPayloadMap['export_done']): void;
  dispose?(): void;
}

export function createLiveDataRuntime(listen: Listen, targets: LiveDataRuntimeTargets) {
  let generation = 0;
  let active = false;
  let unlisteners: Unlisten[] = [];
  let pending: { generation: number; promise: Promise<void> } | null = null;

  function forwardEvent(payload: LiveEvent): void {
    targets.liveEvent(payload);
  }

  const bindings: ReadonlyArray<() => Promise<Unlisten>> = [
    () => listen('room_status_changed', targets.roomStatus),
    () => listen('danmu_received', forwardEvent),
    () => listen('gift_received', forwardEvent),
    () => listen('captain_received', forwardEvent),
    () => listen('enter_received', forwardEvent),
    () => listen('sensitive_word_compile_progress', targets.compileProgress),
    () => listen('stat_updated', targets.statistics),
    () => listen('rank_updated', targets.rankings),
    () => listen('export_done', targets.exportDone)
  ];

  async function register(localGeneration: number): Promise<void> {
    const results = await Promise.allSettled(bindings.map(bind => bind()));
    const registered = results.flatMap(result =>
      result.status === 'fulfilled' ? [result.value] : []
    );
    const rejected = results.find(
      (result): result is PromiseRejectedResult => result.status === 'rejected'
    );
    if (rejected || generation !== localGeneration) {
      registered.forEach(unlisten => unlisten());
      if (rejected) throw rejected.reason;
      return;
    }
    unlisteners = registered;
    active = true;
  }

  function start(): Promise<void> {
    if (active) return Promise.resolve();
    if (pending) return pending.promise;
    const localGeneration = ++generation;
    const promise = register(localGeneration).finally(() => {
      if (pending?.generation === localGeneration) pending = null;
    });
    pending = { generation: localGeneration, promise };
    return promise;
  }

  function stop(): void {
    generation += 1;
    active = false;
    unlisteners.splice(0).forEach(unlisten => unlisten());
    targets.dispose?.();
  }

  return { start, stop, isActive: () => active };
}

let rootRuntime: ReturnType<typeof createLiveDataRuntime> | null = null;

export function useLiveDataRuntime() {
  if (rootRuntime) return rootRuntime;
  const room = useLiveRoomStore();
  const feed = useLiveFeedStore();
  const safety = useLiveSafetyStore();
  const analytics = useLiveAnalyticsStore();

  rootRuntime = createLiveDataRuntime(listenDesktop, {
    roomStatus: room.applyStatus,
    liveEvent: feed.accept,
    compileProgress: safety.setCompileProgress,
    statistics: analytics.replaceStatistics,
    rankings: analytics.replaceRankings,
    exportDone: safety.setLastExport,
    dispose: analytics.dispose
  });
  return rootRuntime;
}
