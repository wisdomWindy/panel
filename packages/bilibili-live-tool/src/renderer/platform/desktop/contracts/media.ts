import type { EventId, GiftId, PathGrant, PathGrantId, RoomId } from './base';
import type { AudienceIdentity, CaptainEvent, GiftEvent } from './live-data';

export type RecordingId = string & { readonly __recordingId: unique symbol };
export type HighlightId = string & { readonly __highlightId: unique symbol };
export type PreviewSessionId = string & { readonly __previewSessionId: unique symbol };
export type CaptainEffectPosition = 'topLeft' | 'topRight' | 'bottomLeft' | 'bottomRight' | 'center';
export type GiftPopupPosition = CaptainEffectPosition;

export interface SoundBindingSummary { bindingId: string; fileName: string; isMissing: boolean }
export interface CaptainEffectPreset {
  presetId: string;
  displayName: string;
  primaryColor: string;
  secondaryColor: string;
}
export interface CaptainEffectConfig {
  presetId: string;
  durationSeconds: 3 | 5 | 8 | 10;
  position: CaptainEffectPosition;
  windowWidth: number;
  windowHeight: number;
  primaryColor: string;
  secondaryColor: string;
  firstSound: SoundBindingSummary | null;
  renewalSound: SoundBindingSummary | null;
  comboSound: SoundBindingSummary | null;
  volume: number;
  comboEnabled: boolean;
  comboWindowSeconds: 5 | 10 | 30;
}
export interface CaptainEffectSnapshot {
  config: CaptainEffectConfig;
  presets: readonly CaptainEffectPreset[];
  updatedAt: number;
}
export interface CaptainEffectPayload {
  eventId: EventId;
  audience: AudienceIdentity;
  action: CaptainEvent['action'];
  level: CaptainEvent['level'];
  config: CaptainEffectConfig;
  isPreview: boolean;
}

export interface GiftNotifyMethods { systemNotification: boolean; localPopup: boolean; sound: boolean }
export interface GiftNotifyConfig {
  isEnabled: boolean;
  highValueThresholdMinor: number;
  methods: GiftNotifyMethods;
  comboEnabled: boolean;
  comboWindowSeconds: 5 | 10 | 30;
  popupPosition: GiftPopupPosition;
  popupDurationSeconds: number;
}
export interface GiftNotifySnapshot {
  config: GiftNotifyConfig;
  totalValueMinor: number;
  donorCount: number;
  giftCount: number;
  latestGifts: readonly GiftEvent[];
  updatedAt: number;
}
export interface GiftAlertPayload {
  event: GiftEvent;
  popupPosition: GiftPopupPosition;
  popupDurationSeconds: number;
  isPreview: boolean;
}
export interface GiftSoundBinding {
  giftId: GiftId;
  giftName: string;
  soundFileName: string | null;
  volume: number;
  isEnabled: boolean;
  isMissing: boolean;
}
export interface GiftSoundSnapshot {
  bindings: readonly GiftSoundBinding[];
  globalVolume: number;
  queueCapacityPerSecond: number;
  updatedAt: number;
}

export type TtsEngine = 'system' | 'cloud';
export type ActiveTtsEngine = TtsEngine | 'localFallback';
export type TtsEventKind = 'danmu' | 'gift' | 'captain' | 'entry';
export type TtsIdentityKind = 'admin' | 'captain' | 'vip' | 'ordinary';
export type TtsState = 'idle' | 'speaking' | 'paused' | 'degraded' | 'unavailable';
export interface TtsTemplates { danmu: string; gift: string; captain: string }
export interface TtsVoice {
  voiceId: string;
  displayName: string;
  engine: TtsEngine;
  isAvailable: boolean;
  unavailableReasonCode: string | null;
}
export interface TtsConfig {
  isEnabled: boolean;
  selectedEngine: TtsEngine;
  selectedVoiceId: string | null;
  rate: number;
  volume: number;
  eventKinds: readonly TtsEventKind[];
  identityKinds: readonly TtsIdentityKind[];
  templates: TtsTemplates;
}
export interface TtsStatus {
  state: TtsState;
  currentText: string | null;
  pendingCount: number;
  activeEngine: ActiveTtsEngine;
  fallbackReasonCode: string | null;
}
export interface TtsSnapshot { config: TtsConfig; voices: readonly TtsVoice[]; status: TtsStatus; updatedAt: number }

/** Languages advertised by Qwen3-TTS-12Hz checkpoints. */
export type Qwen3Language =
  | 'Auto'
  | 'Chinese'
  | 'English'
  | 'Japanese'
  | 'Korean'
  | 'German'
  | 'French'
  | 'Russian'
  | 'Portuguese'
  | 'Spanish'
  | 'Italian';

export interface Qwen3TtsConfig {
  isEnabled: boolean;
  baseUrl: string;
  voiceId: string | null;
  language: Qwen3Language;
  instruct: string;
  timeoutMs: number;
  fallbackToSystem: boolean;
  aiVoiceId: string | null;
  aiInstruct: string | null;
  pipMirror: Qwen3PipMirror;
}

export type Qwen3PipMirror = 'tsinghua' | 'aliyun' | 'official';
export interface Qwen3TtsHealth {
  ready: boolean;
  /** `loading` while the model weights are still warming up. */
  status: string;
  /** `cpu` or `cuda` once the engine reports it. */
  device: string | null;
  engine: string;
  model: string;
  sampleRate: number;
  error: string | null;
}
export interface Qwen3EnvironmentStatus {
  readonly ready: boolean;
  readonly venvPython: string | null;
  readonly detail: string;
}

export interface Qwen3TtsSnapshot {
  config: Qwen3TtsConfig;
  health: Qwen3TtsHealth | null;
  voiceIds: readonly string[];
}

export interface Qwen3ModelEntry {
  modelId: string;
  displayName: string;
  task: string;
  isDefault: boolean;
  isInstalled: boolean;
}

export type Qwen3DownloadStatus = 'idle' | 'downloading' | 'ready' | 'error';

export interface Qwen3DownloadState {
  modelId: string | null;
  status: Qwen3DownloadStatus;
  progressPercent: number | null;
  bytesDownloaded: number;
  bytesTotal: number | null;
  path: string | null;
  error: string | null;
}

export interface Qwen3ModelCatalog {
  models: readonly Qwen3ModelEntry[];
  download: Qwen3DownloadState;
}

export type RecordingFormat = 'mp4' | 'flv';
export type RecordingQuality = 'original' | 'high' | 'standard';
export type FfmpegSource = 'bundled' | 'system' | 'installed' | 'local';
export type RecordingState = 'idle' | 'starting' | 'recording' | 'stopping' | 'finalizing' | 'error';
export type RecordingItemState = 'recording' | 'completed' | 'partial' | 'failed';
export type HighlightTriggerType = 'danmakuSurge' | 'manual';
export interface RecordingConfig {
  format: RecordingFormat;
  quality: RecordingQuality;
  directory: PathGrant | null;
  segmentationEnabled: boolean;
  segmentDurationSeconds: number;
  autoRecord: boolean;
  danmakuThreshold: number;
  highlightBeforeSeconds: 30;
  highlightAfterSeconds: 30;
  quotaBytes: number;
  waterlineBytes: number;
  ffmpegSource: FfmpegSource;
  shortcut: string;
}
export interface RecordingStatus {
  state: RecordingState;
  sessionId: string | null;
  roomId: RoomId | null;
  elapsedSeconds: number;
  fileSizeBytes: number;
  diskRemainingBytes: number;
  highlightCount: number;
  lastErrorCode: string | null;
  /** One entry per concurrent recording; the primary task is id 1. */
  tasks: readonly RecordingTask[];
}
export interface MediaCleanupReport {
  removedRecords: number;
  removedHighlights: number;
  removedFiles: number;
  freedBytes: number;
}

export interface RecordingTask {
  id: number;
  roomId: RoomId | null;
  state: RecordingState;
  elapsedSeconds: number;
  fileSizeBytes: number;
  lastErrorCode: string | null;
}
export interface RecordingSummary {
  id: RecordingId;
  fileName: string;
  format: RecordingFormat;
  sizeBytes: number;
  durationSeconds: number;
  startedAt: number;
  endedAt: number | null;
  status: RecordingItemState;
}
export interface HighlightSummary {
  id: HighlightId;
  recordingId: RecordingId;
  positionSeconds: number;
  triggerType: HighlightTriggerType;
  clipDurationSeconds: number;
  createdAt: number;
  isClipAvailable: boolean;
}
export interface PreviewSession { sessionId: PreviewSessionId; url: string; expiresAt: number }

export const MEDIA_COMMAND_NAMES = [
  'captain_effect_get', 'captain_effect_current', 'captain_effect_save', 'captain_effect_preview',
  'gift_notify_get', 'gift_notify_save', 'gift_notify_test',
  'gift_sound_list', 'gift_sound_set', 'gift_sound_remove', 'gift_sound_save',
  'gift_sound_import', 'gift_sound_export', 'sound_preview',
  'tts_get', 'tts_save', 'tts_set_enabled', 'tts_test', 'tts_pause', 'tts_clear',
  'ffmpeg_install',
  'recording_get_config',
  'recording_save_config',
  'recording_set_ffmpeg_source',
  'recording_status', 'recording_start', 'recording_stop', 'recording_list',
  'recording_preview', 'recording_open_folder', 'recording_rename', 'recording_delete',
  'highlight_list', 'highlight_mark', 'highlight_preview', 'highlight_export', 'highlight_delete',
  'tts_qwen3_get', 'tts_qwen3_save', 'tts_qwen3_test',
  'tts_qwen3_models',
  'tts_qwen3_environment_status',
  'tts_qwen3_prepare_environment',
  'tts_qwen3_start',
  'tts_qwen3_download_model',
  'recording_stop_task', 'recording_mark_highlight_task', 'media_cleanup'
] as const;
export const MEDIA_EVENT_NAMES = [
  'captain_effect_trigger', 'gift_alert_trigger', 'gift_notify_updated',
  'tts_status_changed', 'recording_status_changed', 'recording_completed', 'highlight_created'
] as const;

export type SoundPreviewTarget =
  | { kind: 'binding'; bindingId: string }
  | { kind: 'grant'; grantId: PathGrantId };

export interface MediaCommandRequestMap {
  captain_effect_get: undefined;
  captain_effect_current: undefined;
  captain_effect_save: { config: CaptainEffectConfig };
  captain_effect_preview: { config: CaptainEffectConfig };
  gift_notify_get: undefined;
  gift_notify_save: { config: GiftNotifyConfig };
  gift_notify_test: { config: GiftNotifyConfig };
  gift_sound_list: undefined;
  gift_sound_set: { giftId: GiftId; giftName: string; grantId: PathGrantId };
  gift_sound_remove: { giftId: GiftId };
  gift_sound_save: { globalVolume: number; bindings: readonly GiftSoundBinding[] };
  gift_sound_import: { grantId: PathGrantId };
  gift_sound_export: { grantId: PathGrantId };
  sound_preview: { target: SoundPreviewTarget; volume: number };
  tts_get: undefined;
  tts_save: { config: TtsConfig };
  tts_set_enabled: { isEnabled: boolean };
  tts_test: { config: TtsConfig };
  tts_pause: { isPaused: boolean };
  tts_clear: undefined;
  tts_qwen3_get: undefined;
  tts_qwen3_save: { config: Qwen3TtsConfig };
  tts_qwen3_test: undefined;
  tts_qwen3_models: undefined;
  tts_qwen3_environment_status: undefined;
  tts_qwen3_prepare_environment: undefined;
  tts_qwen3_start: undefined;
  tts_qwen3_download_model: { model: string | null };
  recording_get_config: undefined;
  recording_save_config: { config: RecordingConfig };
  ffmpeg_install: undefined;
  recording_set_ffmpeg_source: { source: FfmpegSource; grantId: PathGrantId | null };
  recording_status: undefined;
  recording_start: { roomId: RoomId };
  recording_stop: undefined;
  recording_stop_task: { taskId: number };
  recording_mark_highlight_task: { taskId: number };
  media_cleanup: undefined;
  recording_list: undefined;
  recording_preview: { recordingId: RecordingId };
  recording_open_folder: { recordingId: RecordingId };
  recording_rename: { recordingId: RecordingId; fileNameStem: string };
  recording_delete: { recordingId: RecordingId };
  highlight_list: undefined;
  highlight_mark: undefined;
  highlight_preview: { highlightId: HighlightId };
  highlight_export: { highlightId: HighlightId; grantId: PathGrantId };
  highlight_delete: { highlightId: HighlightId };
}

export interface MediaCommandResultMap {
  captain_effect_get: CaptainEffectSnapshot;
  captain_effect_current: CaptainEffectPayload | null;
  captain_effect_save: CaptainEffectSnapshot;
  captain_effect_preview: { queued: boolean };
  gift_notify_get: GiftNotifySnapshot;
  gift_notify_save: GiftNotifySnapshot;
  gift_notify_test: GiftAlertPayload;
  gift_sound_list: GiftSoundSnapshot;
  gift_sound_set: GiftSoundBinding;
  gift_sound_remove: { deleted: boolean };
  gift_sound_save: GiftSoundSnapshot;
  gift_sound_import: { importedCount: number };
  gift_sound_export: { exportedCount: number; displayPath: string };
  sound_preview: { started: boolean };
  tts_get: TtsSnapshot;
  tts_save: TtsSnapshot;
  tts_set_enabled: TtsStatus;
  tts_test: TtsStatus;
  tts_pause: TtsStatus;
  tts_clear: TtsStatus;
  tts_qwen3_get: Qwen3TtsSnapshot;
  tts_qwen3_save: Qwen3TtsSnapshot;
  tts_qwen3_test: Qwen3TtsSnapshot;
  tts_qwen3_models: Qwen3ModelCatalog;
  tts_qwen3_environment_status: Qwen3EnvironmentStatus;
  tts_qwen3_prepare_environment: Qwen3EnvironmentStatus;
  tts_qwen3_start: Qwen3TtsSnapshot;
  tts_qwen3_download_model: Qwen3DownloadState;
  recording_get_config: RecordingConfig;
  recording_save_config: RecordingConfig;
  ffmpeg_install: string;
  recording_set_ffmpeg_source: RecordingConfig;
  recording_status: RecordingStatus;
  recording_start: RecordingStatus;
  recording_stop: RecordingStatus;
  recording_stop_task: RecordingStatus;
  recording_mark_highlight_task: HighlightSummary;
  media_cleanup: MediaCleanupReport;
  recording_list: readonly RecordingSummary[];
  recording_preview: PreviewSession;
  recording_open_folder: { opened: boolean };
  recording_rename: RecordingSummary;
  recording_delete: { deleted: boolean };
  highlight_list: readonly HighlightSummary[];
  highlight_mark: HighlightSummary;
  highlight_preview: PreviewSession;
  highlight_export: { exported: boolean; displayPath: string };
  highlight_delete: { deleted: boolean };
}

export interface MediaEventPayloadMap {
  captain_effect_trigger: CaptainEffectPayload;
  gift_alert_trigger: GiftAlertPayload;
  gift_notify_updated: GiftNotifySnapshot;
  tts_status_changed: TtsStatus;
  recording_status_changed: RecordingStatus;
  recording_completed: RecordingSummary;
  highlight_created: HighlightSummary;
  tts_environment_progress: { stage: string; message: string };
}
