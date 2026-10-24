import {
  listenDesktop,
  type EventName,
  type EventPayloadMap
} from '@/platform/desktop';
import { useAIStore } from '@/stores/ai';

type Unlisten = () => void;
type Listen = <K extends EventName>(
  name: K,
  listener: (payload: EventPayloadMap[K]) => void
) => Promise<Unlisten>;

export interface AiRuntimeTargets {
  delta(payload: EventPayloadMap['ai_generation_delta']): void;
  finished(payload: EventPayloadMap['ai_generation_finished']): void;
  asr(payload: EventPayloadMap['asr_status_changed']): void;
  transcript(payload: EventPayloadMap['asr_transcript']): void;
  bootstrap(): Promise<void>;
  dispose(): void;
}

export function createAiRuntime(listen: Listen, targets: AiRuntimeTargets) {
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
      () => listen('ai_generation_delta', guarded<'ai_generation_delta'>(targets.delta)),
      () => listen('ai_generation_finished', guarded<'ai_generation_finished'>(targets.finished)),
      () => listen('asr_status_changed', guarded<'asr_status_changed'>(targets.asr)),
      () => listen('asr_transcript', guarded<'asr_transcript'>(targets.transcript))
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

let rootRuntime: ReturnType<typeof createAiRuntime> | null = null;

export function useAiRuntime() {
  if (rootRuntime) return rootRuntime;
  const ai = useAIStore();
  rootRuntime = createAiRuntime(listenDesktop, {
    delta: ai.acceptDelta,
    finished: ai.acceptGeneration,
    asr: ai.acceptAsr,
    transcript: ai.acceptAsrTranscript,
    bootstrap: ai.bootstrap,
    dispose: ai.dispose
  });
  return rootRuntime;
}
