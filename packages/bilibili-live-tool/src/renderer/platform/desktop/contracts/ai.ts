import type { UserUid } from './base';

export type GenerationId = string & { readonly __generationId: unique symbol };
export type AiModelProfileId = string & { readonly __aiModelProfileId: unique symbol };
export type AiOutputDeviceId = string & { readonly __aiOutputDeviceId: unique symbol };
export type AiVoiceId = string & { readonly __aiVoiceId: unique symbol };
export type AiVoiceQueueItemId = string & { readonly __aiVoiceQueueItemId: unique symbol };
export type AiSpeakingStyle = 'concise' | 'natural' | 'detailed';
export type AiProviderKind = 'cloud' | 'aggregator' | 'local' | 'customOpenAi';
export type AiReasoningEffort =
  | 'none'
  | 'minimal'
  | 'low'
  | 'medium'
  | 'high'
  | 'xhigh'
  | 'max';
export type AiModelConnectionState = 'untested' | 'testing' | 'available' | 'unavailable';
export type AiTriggerScope = 'all' | 'questions' | 'captains' | 'keywords' | 'highValue';

export interface AiConfig {
  readonly identity: string;
  readonly speakingStyle: AiSpeakingStyle;
  readonly systemPrompt: string;
  readonly maxLength: number;
  readonly maxSentences: number;
  readonly isSensitiveFilterEnabled: boolean;
  readonly bannedWords: readonly string[];
  readonly danmakuContextCount: number;
  readonly manualContextCount: number;
}

export interface AiConfigSaveRequest {
  readonly config: AiConfig;
}

export interface AiModelProfile {
  readonly profileId: AiModelProfileId;
  readonly name: string;
  readonly providerKind: AiProviderKind;
  readonly baseUrl: string;
  readonly model: string;
  readonly reasoningEffort: AiReasoningEffort;
  readonly maxTokens: number;
  readonly timeoutSeconds: number;
  readonly proxyUrl: string | null;
  readonly isEnabled: boolean;
  readonly isDefault: boolean;
  readonly connectionState: AiModelConnectionState;
  readonly isApiKeyConfigured: boolean;
  readonly updatedAt: number;
}

export interface AiModelProfileWritable {
  readonly profileId: AiModelProfileId | null;
  readonly name: string;
  readonly providerKind: AiProviderKind;
  readonly baseUrl: string;
  readonly model: string;
  readonly reasoningEffort: AiReasoningEffort;
  readonly maxTokens: number;
  readonly timeoutSeconds: number;
  readonly proxyUrl: string | null;
  readonly isEnabled: boolean;
}

export interface AiModelProfileSaveRequest {
  readonly profile: AiModelProfileWritable;
  readonly apiKeyReplacement: string | null;
}

export interface AiVoiceReplyRules {
  readonly triggerScope: AiTriggerScope;
  readonly keywords: readonly string[];
  readonly globalCooldownSeconds: number;
  readonly userCooldownSeconds: number;
  readonly queueCapacity: number;
  readonly isDuplicateFilterEnabled: boolean;
}

export interface AiOutputDevice {
  readonly deviceId: AiOutputDeviceId;
  readonly displayName: string;
  readonly isAvailable: boolean;
  readonly isDefault: boolean;
}

export interface AiVoice {
  readonly voiceId: AiVoiceId;
  readonly displayName: string;
  readonly isAvailable: boolean;
  readonly isDefault: boolean;
}

export interface AiVoiceQueueItem {
  readonly itemId: AiVoiceQueueItemId;
  readonly text: string;
  readonly priority: number;
  readonly createdAt: number;
}

export interface AiVoiceQueueSnapshot {
  readonly state: 'idle' | 'speaking' | 'paused' | 'unavailable';
  readonly current: AiVoiceQueueItem | null;
  readonly pending: readonly AiVoiceQueueItem[];
  readonly capacity: number;
  readonly volume: number;
  readonly selectedDeviceId: AiOutputDeviceId | null;
  readonly selectedVoiceId: AiVoiceId;
  readonly isAudioRouteConfirmed: boolean;
  readonly lastErrorCode: string | null;
}

export interface AiVoiceReplySnapshot {
  readonly isEnabled: boolean;
  readonly defaultProfileId: AiModelProfileId | null;
  readonly rules: AiVoiceReplyRules;
  readonly devices: readonly AiOutputDevice[];
  readonly voices: readonly AiVoice[];
  readonly queue: AiVoiceQueueSnapshot;
}

export interface AiContextItem {
  readonly id: string;
  readonly source: 'danmaku' | 'manual';
  readonly content: string;
}

export interface AiGenerateRequest {
  readonly prompt: string;
  readonly context: readonly AiContextItem[];
}

export interface AiGeneration {
  readonly generationId: GenerationId;
  readonly status: 'assembling' | 'streaming' | 'completed' | 'failed' | 'rejected' | 'cancelled';
  readonly result: string;
  readonly lastSequence: number;
  readonly errorCode: string | null;
}

export interface AiLogEntry {
  readonly id: number;
  readonly operationType: string;
  readonly username: string | null;
  readonly userUid: UserUid | null;
  readonly originalContent: string | null;
  readonly aiContent: string | null;
  readonly generationStatus: string;
  readonly rejectReason: string | null;
  readonly roomId: string;
  readonly createdAt: number;
}

export const AI_COMMAND_NAMES = [
  'ai_config_get',
  'ai_config_save',
  'ai_model_profile_list',
  'ai_model_profile_save',
  'ai_model_profile_delete',
  'ai_model_set_default',
  'ai_model_test',
  'ai_voice_reply_get',
  'ai_voice_reply_set_enabled',
  'ai_voice_reply_update_rules',
  'ai_tts_list_output_devices',
  'ai_tts_set_output_device',
  'ai_tts_test',
  'ai_tts_pause',
  'ai_tts_skip',
  'ai_tts_clear_queue',
  'ai_generate',
  'ai_regenerate',
  'ai_log_query'
] as const;

export interface AiCommandRequestMap {
  ai_config_get: undefined;
  ai_config_save: AiConfigSaveRequest;
  ai_model_profile_list: undefined;
  ai_model_profile_save: AiModelProfileSaveRequest;
  ai_model_profile_delete: { profileId: AiModelProfileId };
  ai_model_set_default: { profileId: AiModelProfileId };
  ai_model_test: { profileId: AiModelProfileId };
  ai_voice_reply_get: undefined;
  ai_voice_reply_set_enabled: { isEnabled: boolean; isAudioRouteConfirmed: boolean };
  ai_voice_reply_update_rules: { rules: AiVoiceReplyRules };
  ai_tts_list_output_devices: undefined;
  ai_tts_set_output_device: {
    deviceId: AiOutputDeviceId;
    voiceId: AiVoiceId;
    volume: number;
    isAudioRouteConfirmed: boolean;
  };
  ai_tts_test: { deviceId: AiOutputDeviceId; voiceId: AiVoiceId; volume: number };
  ai_tts_pause: { isPaused: boolean };
  ai_tts_skip: undefined;
  ai_tts_clear_queue: undefined;
  ai_generate: AiGenerateRequest;
  ai_regenerate: { generationId: GenerationId };
  ai_log_query: { userUid: UserUid | null; startedAt: number | null; endedAt: number | null };
}

export interface AiCommandResultMap {
  ai_config_get: AiConfig;
  ai_config_save: AiConfig;
  ai_model_profile_list: readonly AiModelProfile[];
  ai_model_profile_save: AiModelProfile;
  ai_model_profile_delete: { deleted: boolean };
  ai_model_set_default: readonly AiModelProfile[];
  ai_model_test: AiModelProfile;
  ai_voice_reply_get: AiVoiceReplySnapshot;
  ai_voice_reply_set_enabled: AiVoiceReplySnapshot;
  ai_voice_reply_update_rules: AiVoiceReplySnapshot;
  ai_tts_list_output_devices: readonly AiOutputDevice[];
  ai_tts_set_output_device: AiVoiceReplySnapshot;
  ai_tts_test: AiVoiceReplySnapshot;
  ai_tts_pause: AiVoiceReplySnapshot;
  ai_tts_skip: AiVoiceReplySnapshot;
  ai_tts_clear_queue: AiVoiceReplySnapshot;
  ai_generate: AiGeneration;
  ai_regenerate: AiGeneration;
  ai_log_query: readonly AiLogEntry[];
}

export interface AiEventPayloadMap {
  ai_generation_delta: { generationId: GenerationId; sequence: number; delta: string };
  ai_generation_finished: AiGeneration;
}
