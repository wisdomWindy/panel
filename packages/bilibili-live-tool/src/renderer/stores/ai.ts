import { computed, ref } from 'vue';
import { defineStore } from 'pinia';
import {
  invokeDesktop,
  normalizeDesktopError,
  type AiConfig,
  type AiConfigSaveRequest,
  type AiContextItem,
  type AiGeneration,
  type AiLogEntry,
  type AsrModelManifest,
  type AsrStatus,
  type DesktopError,
  type GenerationId,
  type UserUid
} from '@/platform/desktop';
import {
  adaptAiConfig,
  adaptAiGeneration,
  adaptAiLogs,
  adaptAsrStatus
} from '@/modules/member/adapters';

export const useAIStore = defineStore('member-ai', () => {
  const config = ref<AiConfig | null>(null);
  const generation = ref<AiGeneration | null>(null);
  const asr = ref<AsrStatus | null>(null);
  const logs = ref<readonly AiLogEntry[]>([]);
  const pendingOperations = ref<ReadonlySet<string>>(new Set());
  const error = ref<DesktopError | null>(null);
  const operations = new Map<string, Promise<unknown>>();
  let revision = 0;

  const isGenerating = computed(
    () => generation.value?.status === 'assembling' || generation.value?.status === 'streaming'
  );

  function run<T>(key: string, effect: () => Promise<T>): Promise<T> {
    const existing = operations.get(key) as Promise<T> | undefined;
    if (existing) return existing;
    const localRevision = revision;
    error.value = null;
    pendingOperations.value = new Set([...pendingOperations.value, key]);
    const current = effect()
      .catch(reason => {
        const mapped = normalizeDesktopError(reason);
        if (localRevision === revision) error.value = mapped;
        throw mapped;
      })
      .finally(() => {
        if (operations.get(key) === current) operations.delete(key);
        if (localRevision === revision) {
          const next = new Set(pendingOperations.value);
          next.delete(key);
          pendingOperations.value = next;
        }
      });
    operations.set(key, current);
    return current;
  }

  function bootstrap(): Promise<void> {
    return run('bootstrap', async () => {
      const [nextConfig, nextAsr] = await Promise.all([
        invokeDesktop('ai_config_get', undefined),
        invokeDesktop('asr_status', undefined)
      ]);
      config.value = adaptAiConfig(nextConfig);
      asr.value = adaptAsrStatus(nextAsr);
    });
  }

  function saveConfig(request: AiConfigSaveRequest): Promise<AiConfig> {
    return run('saveConfig', async () => {
      const next = adaptAiConfig(await invokeDesktop('ai_config_save', request));
      config.value = next;
      return next;
    });
  }

  function generate(prompt: string, context: readonly AiContextItem[]): Promise<AiGeneration> {
    return run('generate', async () => {
      const next = adaptAiGeneration(await invokeDesktop('ai_generate', { prompt, context }));
      generation.value = next;
      return next;
    });
  }

  function regenerate(generationId: GenerationId): Promise<AiGeneration> {
    return run('generate', async () => {
      const next = adaptAiGeneration(await invokeDesktop('ai_regenerate', { generationId }));
      generation.value = next;
      return next;
    });
  }

  function loadLogs(filters: {
    userUid: UserUid | null;
    startedAt: number | null;
    endedAt: number | null;
  }): Promise<readonly AiLogEntry[]> {
    return run('loadLogs', async () => {
      const next = adaptAiLogs(await invokeDesktop('ai_log_query', filters));
      logs.value = next;
      return next;
    });
  }

  function downloadModel(
    manifest: AsrModelManifest,
    isLicenseAccepted: boolean
  ): Promise<AsrStatus> {
    return run('downloadModel', async () => {
      const next = adaptAsrStatus(
        await invokeDesktop('asr_model_download', { manifest, isLicenseAccepted })
      );
      asr.value = next;
      return next;
    });
  }

  function startAsr(): Promise<AsrStatus> {
    return run('asr', async () => {
      const next = adaptAsrStatus(await invokeDesktop('asr_start', undefined));
      asr.value = next;
      return next;
    });
  }

  function stopAsr(): Promise<AsrStatus> {
    return run('asr', async () => {
      const next = adaptAsrStatus(await invokeDesktop('asr_stop', undefined));
      asr.value = next;
      return next;
    });
  }

  function acceptDelta(payload: {
    readonly generationId: GenerationId;
    readonly sequence: number;
    readonly delta: string;
  }): void {
    const current = generation.value;
    if (!current || current.generationId !== payload.generationId) return;
    if (payload.sequence !== current.lastSequence + 1) return;
    generation.value = Object.freeze({
      ...current,
      status: 'streaming',
      result: current.result + payload.delta,
      lastSequence: payload.sequence
    });
  }

  function acceptGeneration(value: unknown): void {
    const next = adaptAiGeneration(value);
    if (generation.value && generation.value.generationId !== next.generationId) return;
    generation.value = next;
  }

  function acceptAsr(value: unknown): void {
    asr.value = adaptAsrStatus(value);
  }

  function clear(): void {
    generation.value = null;
    logs.value = [];
    error.value = null;
  }

  function dispose(): void {
    revision += 1;
    operations.clear();
    pendingOperations.value = new Set();
    generation.value = null;
    error.value = null;
  }

  return {
    config,
    generation,
    asr,
    logs,
    pendingOperations,
    error,
    isGenerating,
    bootstrap,
    saveConfig,
    generate,
    regenerate,
    loadLogs,
    downloadModel,
    startAsr,
    stopAsr,
    acceptDelta,
    acceptGeneration,
    acceptAsr,
    clear,
    dispose
  };
});
