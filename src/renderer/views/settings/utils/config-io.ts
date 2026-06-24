export interface CoreConfigShape {
  [key: string]: unknown;
}

export interface RecordingConfigShape {
  [key: string]: unknown;
}

export interface SettingsExportConfig {
  core: CoreConfigShape;
  recording?: RecordingConfigShape;
  tts?: Record<string, unknown>;
  asr?: Record<string, unknown>;
  ai?: Record<string, unknown>;
  moderation?: Record<string, unknown>;
}

export interface SettingsImportPatches {
  core?: CoreConfigShape;
  recording?: RecordingConfigShape;
  tts?: Record<string, unknown>;
  asr?: Record<string, unknown>;
  ai?: Record<string, unknown>;
  moderation?: Record<string, unknown>;
}

export function buildSettingsExportConfig(config: SettingsExportConfig): SettingsExportConfig {
  return {
    core: { ...config.core },
    recording: config.recording ? { ...config.recording } : undefined,
    tts: config.tts ? { ...config.tts } : undefined,
    asr: config.asr ? { ...config.asr } : undefined,
    ai: config.ai ? { ...config.ai } : undefined,
    moderation: config.moderation ? { ...config.moderation } : undefined
  };
}

export function createSettingsImportPatches(
  raw: Partial<SettingsExportConfig>
): SettingsImportPatches {
  const patches: SettingsImportPatches = {};

  if (raw.core) {
    patches.core = { ...raw.core };
  }
  if (raw.recording) {
    patches.recording = { ...raw.recording };
  }
  if (raw.tts) {
    patches.tts = { ...raw.tts };
  }
  if (raw.asr) {
    patches.asr = { ...raw.asr };
  }
  if (raw.ai) {
    patches.ai = { ...raw.ai };
  }
  if (raw.moderation) {
    patches.moderation = { ...raw.moderation };
  }

  return patches;
}
