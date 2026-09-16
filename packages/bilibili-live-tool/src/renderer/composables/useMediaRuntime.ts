import {
  listenDesktop,
  type EventName,
  type EventPayloadMap
} from '@/platform/desktop';
import { useMediaEffectsStore } from '@/stores/media/effects';
import { useMediaGiftsStore } from '@/stores/media/gifts';
import { useMediaRecordingStore } from '@/stores/media/recording';
import { useMediaTtsStore } from '@/stores/media/tts';

type Unlisten = () => void;
type Listen = <K extends EventName>(
  name: K,
  listener: (payload: EventPayloadMap[K]) => void
) => Promise<Unlisten>;

export interface MediaRuntimeTargets {
  giftAlert(payload: EventPayloadMap['gift_alert_trigger']): void;
  giftNotify(payload: EventPayloadMap['gift_notify_updated']): void;
  ttsStatus(payload: EventPayloadMap['tts_status_changed']): void;
  recordingStatus(payload: EventPayloadMap['recording_status_changed']): void;
  recordingCompleted(payload: EventPayloadMap['recording_completed']): void;
  highlightCreated(payload: EventPayloadMap['highlight_created']): void;
  dispose?(): void;
}

export function createMediaRuntime(listen: Listen, targets: MediaRuntimeTargets) {
  let generation = 0;
  let active = false;
  let unlisteners: Unlisten[] = [];
  let pending: { generation: number; promise: Promise<void> } | null = null;

  async function register(localGeneration: number): Promise<void> {
    const activeListener = <K extends EventName>(handler: (payload: EventPayloadMap[K]) => void) =>
      (payload: EventPayloadMap[K]) => {
        if (generation === localGeneration) handler(payload);
      };
    const bindings: ReadonlyArray<() => Promise<Unlisten>> = [
      () => listen('gift_alert_trigger', activeListener<'gift_alert_trigger'>(targets.giftAlert)),
      () => listen('gift_notify_updated', activeListener<'gift_notify_updated'>(targets.giftNotify)),
      () => listen('tts_status_changed', activeListener<'tts_status_changed'>(targets.ttsStatus)),
      () => listen('recording_status_changed', activeListener<'recording_status_changed'>(targets.recordingStatus)),
      () => listen('recording_completed', activeListener<'recording_completed'>(targets.recordingCompleted)),
      () => listen('highlight_created', activeListener<'highlight_created'>(targets.highlightCreated))
    ];
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

let rootRuntime: ReturnType<typeof createMediaRuntime> | null = null;

export function useMediaRuntime() {
  if (rootRuntime) return rootRuntime;
  const effects = useMediaEffectsStore();
  const gifts = useMediaGiftsStore();
  const tts = useMediaTtsStore();
  const recording = useMediaRecordingStore();
  rootRuntime = createMediaRuntime(listenDesktop, {
    giftAlert: gifts.acceptAlert,
    giftNotify: gifts.replaceNotify,
    ttsStatus: tts.replaceStatus,
    recordingStatus: recording.replaceStatus,
    recordingCompleted: recording.acceptCompleted,
    highlightCreated: recording.acceptHighlight,
    dispose: () => {
      effects.dispose();
      gifts.dispose();
      tts.dispose();
      recording.dispose();
    }
  });
  return rootRuntime;
}
