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
  type AiModelProfile,
  type AiModelProfileId,
  type AiModelProfileSaveRequest,
  type AiOutputDevice,
  type AiOutputDeviceId,
  type AiVoiceId,
  type AiVoiceReplyRules,
  type AiVoiceReplySnapshot,
  type DesktopError,
  type GenerationId,
  type UserUid
} from '@/platform/desktop';
import {
  adaptAiConfig,
  adaptAiGeneration,
  adaptAiLogs,
  adaptAiModelProfile,
  adaptAiModelProfiles,
  adaptAiOutputDevice,
  adaptAiVoiceReplySnapshot
} from '@/modules/ai/adapters';

export const useAIStore = defineStore('ai', () => {
  const config = ref<AiConfig | null>(null);
  const profiles = ref<readonly AiModelProfile[]>([]);
  const voiceReply = ref<AiVoiceReplySnapshot | null>(null);
  const generation = ref<AiGeneration | null>(null);
  const logs = ref<readonly AiLogEntry[]>([]);
  const pendingOperations = ref<ReadonlySet<string>>(new Set());
  const error = ref<DesktopError | null>(null);
  const operationErrors = ref<Readonly<Record<string, DesktopError>>>({});

  function errorFor(...keys: string[]): DesktopError | null {
    return Object.entries(operationErrors.value).find(([operation]) =>
      keys.some(key => operation === key || operation.startsWith(`${key}:`))
    )?.[1] ?? null;
  }
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
    const nextErrors = { ...operationErrors.value };
    delete nextErrors[key];
    operationErrors.value = nextErrors;
    pendingOperations.value = new Set([...pendingOperations.value, key]);
    const current = effect()
      .catch(reason => {
        const mapped = normalizeDesktopError(reason);
        if (localRevision === revision) {
          error.value = mapped;
          operationErrors.value = { ...operationErrors.value, [key]: mapped };
        }
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
      const [nextConfig, nextProfiles, nextVoiceReply] = await Promise.all([
        invokeDesktop('ai_config_get', undefined),
        invokeDesktop('ai_model_profile_list', undefined),
        invokeDesktop('ai_voice_reply_get', undefined)
      ]);
      config.value = adaptAiConfig(nextConfig);
      profiles.value = adaptAiModelProfiles(nextProfiles);
      voiceReply.value = adaptAiVoiceReplySnapshot(nextVoiceReply);
    });
  }

  function saveProfile(request: AiModelProfileSaveRequest): Promise<AiModelProfile> {
    return run('saveProfile', async () => {
      const next = adaptAiModelProfile(await invokeDesktop('ai_model_profile_save', request));
      profiles.value = Object.freeze([
        ...profiles.value.filter(profile => profile.profileId !== next.profileId),
        next
      ]);
      return next;
    });
  }

  function deleteProfile(profileId: AiModelProfileId): Promise<boolean> {
    return run(`deleteProfile:${profileId}`, async () => {
      const result = await invokeDesktop('ai_model_profile_delete', { profileId });
      if (result.deleted) {
        profiles.value = Object.freeze(profiles.value.filter(profile => profile.profileId !== profileId));
      }
      return result.deleted;
    });
  }

  function setDefaultProfile(profileId: AiModelProfileId): Promise<readonly AiModelProfile[]> {
    return run('setDefaultProfile', async () => {
      const next = adaptAiModelProfiles(await invokeDesktop('ai_model_set_default', { profileId }));
      profiles.value = next;
      if (voiceReply.value) {
        voiceReply.value = Object.freeze({ ...voiceReply.value, defaultProfileId: profileId });
      }
      return next;
    });
  }

  function testProfile(profileId: AiModelProfileId): Promise<AiModelProfile> {
    return run(`testProfile:${profileId}`, async () => {
      const next = adaptAiModelProfile(await invokeDesktop('ai_model_test', { profileId }));
      profiles.value = Object.freeze(profiles.value.map(profile =>
        profile.profileId === next.profileId ? next : profile
      ));
      return next;
    });
  }

  function commitVoiceReply(
    key: string,
    effect: () => Promise<unknown>
  ): Promise<AiVoiceReplySnapshot> {
    return run(key, async () => {
      const next = adaptAiVoiceReplySnapshot(await effect());
      voiceReply.value = next;
      return next;
    });
  }

  function setVoiceReplyEnabled(
    isEnabled: boolean,
    isAudioRouteConfirmed: boolean,
    isDataDisclosureAcknowledged: boolean
  ): Promise<AiVoiceReplySnapshot> {
    return commitVoiceReply('setVoiceReplyEnabled', () =>
      invokeDesktop('ai_voice_reply_set_enabled', {
        isEnabled,
        isAudioRouteConfirmed,
        isDataDisclosureAcknowledged
      })
    );
  }

  function refreshVoiceReply(): Promise<AiVoiceReplySnapshot> {
    return commitVoiceReply('refreshVoiceReply', () => invokeDesktop('ai_voice_reply_get', undefined));
  }

  function updateVoiceRules(rules: AiVoiceReplyRules): Promise<AiVoiceReplySnapshot> {
    return commitVoiceReply('updateVoiceRules', () =>
      invokeDesktop('ai_voice_reply_update_rules', { rules })
    );
  }

  function loadOutputDevices(): Promise<readonly AiOutputDevice[]> {
    return run('loadOutputDevices', async () => {
      const value = await invokeDesktop('ai_tts_list_output_devices', undefined);
      const devices = Object.freeze(value.map(adaptAiOutputDevice));
      if (voiceReply.value) voiceReply.value = Object.freeze({ ...voiceReply.value, devices });
      return devices;
    });
  }

  function setOutputDevice(
    deviceId: AiOutputDeviceId,
    voiceId: AiVoiceId,
    volume: number,
    isAudioRouteConfirmed: boolean
  ): Promise<AiVoiceReplySnapshot> {
    return commitVoiceReply('setOutputDevice', () =>
      invokeDesktop('ai_tts_set_output_device', {
        deviceId,
        voiceId,
        volume,
        isAudioRouteConfirmed
      })
    );
  }

  function testOutputDevice(
    deviceId: AiOutputDeviceId,
    voiceId: AiVoiceId,
    volume: number
  ): Promise<AiVoiceReplySnapshot> {
    return commitVoiceReply('testOutputDevice', () =>
      invokeDesktop('ai_tts_test', { deviceId, voiceId, volume })
    );
  }

  function pauseVoiceQueue(isPaused: boolean): Promise<AiVoiceReplySnapshot> {
    return commitVoiceReply('pauseVoiceQueue', () => invokeDesktop('ai_tts_pause', { isPaused }));
  }

  function skipVoiceQueue(): Promise<AiVoiceReplySnapshot> {
    return commitVoiceReply('skipVoiceQueue', () => invokeDesktop('ai_tts_skip', undefined));
  }

  function clearVoiceQueue(): Promise<AiVoiceReplySnapshot> {
    return commitVoiceReply('clearVoiceQueue', () => invokeDesktop('ai_tts_clear_queue', undefined));
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
    void refreshVoiceReply().catch(() => undefined);
  }

  function clear(): void {
    operationErrors.value = {};
    generation.value = null;
    logs.value = [];
    error.value = null;
  }

  function dispose(): void {
    operationErrors.value = {};
    revision += 1;
    operations.clear();
    pendingOperations.value = new Set();
    generation.value = null;
    error.value = null;
  }

  return {
    config,
    profiles,
    voiceReply,
    generation,
    logs,
    pendingOperations,
    error,
    errorFor,
    isGenerating,
    bootstrap,
    saveConfig,
    saveProfile,
    deleteProfile,
    setDefaultProfile,
    testProfile,
    setVoiceReplyEnabled,
    refreshVoiceReply,
    updateVoiceRules,
    loadOutputDevices,
    setOutputDevice,
    testOutputDevice,
    pauseVoiceQueue,
    skipVoiceQueue,
    clearVoiceQueue,
    generate,
    regenerate,
    loadLogs,
    acceptDelta,
    acceptGeneration,
    clear,
    dispose
  };
});
