import type {
  AudienceIdentity,
  CaptainEffectConfig,
  CaptainEffectPayload,
  CaptainEffectSnapshot,
  GiftAlertPayload,
  GiftEvent,
  GiftNotifyConfig,
  GiftNotifySnapshot,
  GiftSoundBinding,
  GiftSoundSnapshot,
  HighlightSummary,
  RecordingConfig,
  RecordingStatus,
  RecordingSummary,
  SoundBindingSummary,
  TtsConfig,
  TtsSnapshot,
  TtsStatus,
  TtsVoice
} from '@/platform/desktop';

export class MalformedMediaPayloadError extends Error {
  constructor(readonly field: string) {
    super(`Malformed media payload: ${field}`);
    this.name = 'MalformedMediaPayloadError';
  }
}

type RecordValue = Record<string, unknown>;

function fail(field: string): never {
  throw new MalformedMediaPayloadError(field);
}

function record(value: unknown, field: string): RecordValue {
  if (typeof value !== 'object' || value === null || Array.isArray(value)) fail(field);
  return value as RecordValue;
}

function string(value: unknown, field: string, allowEmpty = false): string {
  if (typeof value !== 'string' || (!allowEmpty && value.length === 0)) fail(field);
  return value;
}

function boolean(value: unknown, field: string): boolean {
  if (typeof value !== 'boolean') fail(field);
  return value;
}

function number(value: unknown, field: string, minimum = 0): number {
  if (typeof value !== 'number' || !Number.isFinite(value) || value < minimum) fail(field);
  return value;
}

function integer(value: unknown, field: string, minimum = 0): number {
  const result = number(value, field, minimum);
  if (!Number.isSafeInteger(result)) fail(field);
  return result;
}

function nullableString(value: unknown, field: string): string | null {
  return value === null ? null : string(value, field);
}

function enumeration<T extends string>(value: unknown, allowed: readonly T[], field: string): T {
  const result = string(value, field);
  if (!allowed.includes(result as T)) fail(field);
  return result as T;
}

function numberChoice<T extends number>(value: unknown, allowed: readonly T[], field: string): T {
  const result = number(value, field);
  if (!allowed.includes(result as T)) fail(field);
  return result as T;
}

function array(value: unknown, field: string): unknown[] {
  if (!Array.isArray(value)) fail(field);
  return value;
}

function frozen<T extends object>(value: T): T {
  return Object.freeze(value);
}

function soundBinding(value: unknown, field: string): SoundBindingSummary | null {
  if (value === null) return null;
  const source = record(value, field);
  return frozen({
    bindingId: string(source.bindingId, `${field}.bindingId`),
    fileName: string(source.fileName, `${field}.fileName`),
    isMissing: boolean(source.isMissing, `${field}.isMissing`)
  });
}

export function adaptCaptainEffectConfig(value: unknown): CaptainEffectConfig {
  const source = record(value, 'captainEffectConfig');
  const durationSeconds = numberChoice(source.durationSeconds, [3, 5, 8, 10], 'durationSeconds');
  const comboWindowSeconds = numberChoice(source.comboWindowSeconds, [5, 10, 30], 'comboWindowSeconds');
  const volume = number(source.volume, 'volume');
  if (volume > 1) fail('volume');
  const primaryColor = string(source.primaryColor, 'primaryColor');
  const secondaryColor = string(source.secondaryColor, 'secondaryColor');
  if (!/^#[0-9a-fA-F]{6}$/.test(primaryColor)) fail('primaryColor');
  if (!/^#[0-9a-fA-F]{6}$/.test(secondaryColor)) fail('secondaryColor');
  return frozen({
    presetId: string(source.presetId, 'presetId'),
    durationSeconds,
    position: enumeration(source.position, ['topLeft', 'topRight', 'bottomLeft', 'bottomRight', 'center'], 'position'),
    primaryColor,
    secondaryColor,
    firstSound: soundBinding(source.firstSound, 'firstSound'),
    renewalSound: soundBinding(source.renewalSound, 'renewalSound'),
    comboSound: soundBinding(source.comboSound, 'comboSound'),
    volume,
    comboEnabled: boolean(source.comboEnabled, 'comboEnabled'),
    comboWindowSeconds
  });
}

function audience(value: unknown): AudienceIdentity {
  const source = record(value, 'audience');
  const medalSource = source.medal === null ? null : record(source.medal, 'audience.medal');
  return frozen({
    userUid: string(source.userUid, 'audience.userUid') as AudienceIdentity['userUid'],
    username: string(source.username, 'audience.username'),
    userLevel: source.userLevel === null ? null : integer(source.userLevel, 'audience.userLevel'),
    medal: medalSource === null
      ? null
      : frozen({
        name: string(medalSource.name, 'audience.medal.name'),
        level: medalSource.level === null ? null : integer(medalSource.level, 'audience.medal.level')
      }),
    tags: frozen(array(source.tags, 'audience.tags').map((tag, index) =>
      enumeration(tag, ['admin', 'governor', 'admiral', 'captain', 'vip'], `audience.tags.${index}`)
    ))
  });
}

export function adaptGiftEvent(value: unknown): GiftEvent {
  const source = record(value, 'giftEvent');
  if (source.kind !== 'gift') fail('giftEvent.kind');
  const roomId = string(source.roomId, 'giftEvent.roomId');
  if (!/^\d{1,20}$/.test(roomId)) fail('giftEvent.roomId');
  return frozen({
    kind: 'gift',
    eventId: string(source.eventId, 'giftEvent.eventId') as GiftEvent['eventId'],
    roomId: roomId as GiftEvent['roomId'],
    occurredAt: integer(source.occurredAt, 'giftEvent.occurredAt'),
    audience: audience(source.audience),
    giftId: string(source.giftId, 'giftEvent.giftId') as GiftEvent['giftId'],
    giftName: string(source.giftName, 'giftEvent.giftName'),
    quantity: integer(source.quantity, 'giftEvent.quantity', 1),
    valueMinor: integer(source.valueMinor, 'giftEvent.valueMinor')
  });
}

function giftNotifyConfig(value: unknown): GiftNotifyConfig {
  const source = record(value, 'giftNotifyConfig');
  const methods = record(source.methods, 'giftNotifyConfig.methods');
  return frozen({
    isEnabled: boolean(source.isEnabled, 'giftNotifyConfig.isEnabled'),
    highValueThresholdMinor: integer(source.highValueThresholdMinor, 'giftNotifyConfig.highValueThresholdMinor'),
    methods: frozen({
      systemNotification: boolean(methods.systemNotification, 'methods.systemNotification'),
      localPopup: boolean(methods.localPopup, 'methods.localPopup'),
      sound: boolean(methods.sound, 'methods.sound')
    }),
    comboEnabled: boolean(source.comboEnabled, 'giftNotifyConfig.comboEnabled'),
    comboWindowSeconds: numberChoice(source.comboWindowSeconds, [5, 10, 30], 'giftNotifyConfig.comboWindowSeconds'),
    popupPosition: enumeration(source.popupPosition, ['topLeft', 'topRight', 'bottomLeft', 'bottomRight', 'center'], 'giftNotifyConfig.popupPosition'),
    popupDurationSeconds: integer(source.popupDurationSeconds, 'giftNotifyConfig.popupDurationSeconds', 1)
  });
}

export function adaptCaptainEffectSnapshot(value: unknown): CaptainEffectSnapshot {
  const source = record(value, 'captainEffectSnapshot');
  const presets = array(source.presets, 'captainEffectSnapshot.presets').map((item, index) => {
    const preset = record(item, `presets.${index}`);
    return frozen({
      presetId: string(preset.presetId, `presets.${index}.presetId`),
      displayName: string(preset.displayName, `presets.${index}.displayName`)
    });
  });
  return frozen({
    config: adaptCaptainEffectConfig(source.config),
    presets: frozen(presets),
    updatedAt: integer(source.updatedAt, 'captainEffectSnapshot.updatedAt')
  });
}

export function adaptCaptainEffectPayload(value: unknown): CaptainEffectPayload {
  const source = record(value, 'captainEffectPayload');
  return frozen({
    eventId: string(source.eventId, 'captainEffectPayload.eventId') as CaptainEffectPayload['eventId'],
    audience: audience(source.audience),
    action: enumeration(source.action, ['purchase', 'renewal'], 'captainEffectPayload.action'),
    level: enumeration(source.level, ['governor', 'admiral', 'captain'], 'captainEffectPayload.level'),
    config: adaptCaptainEffectConfig(source.config),
    isPreview: boolean(source.isPreview, 'captainEffectPayload.isPreview')
  });
}

export function adaptGiftNotifySnapshot(value: unknown): GiftNotifySnapshot {
  const source = record(value, 'giftNotifySnapshot');
  return frozen({
    config: giftNotifyConfig(source.config),
    totalValueMinor: integer(source.totalValueMinor, 'giftNotifySnapshot.totalValueMinor'),
    donorCount: integer(source.donorCount, 'giftNotifySnapshot.donorCount'),
    giftCount: integer(source.giftCount, 'giftNotifySnapshot.giftCount'),
    latestGifts: frozen(array(source.latestGifts, 'giftNotifySnapshot.latestGifts').map(adaptGiftEvent)),
    updatedAt: integer(source.updatedAt, 'giftNotifySnapshot.updatedAt')
  });
}

export function adaptGiftAlertPayload(value: unknown): GiftAlertPayload {
  const source = record(value, 'giftAlertPayload');
  return frozen({
    event: adaptGiftEvent(source.event),
    popupPosition: enumeration(source.popupPosition, ['topLeft', 'topRight', 'bottomLeft', 'bottomRight', 'center'], 'giftAlertPayload.popupPosition'),
    popupDurationSeconds: integer(source.popupDurationSeconds, 'giftAlertPayload.popupDurationSeconds', 1),
    isPreview: boolean(source.isPreview, 'giftAlertPayload.isPreview')
  });
}

function giftSoundBinding(value: unknown, field: string): GiftSoundBinding {
  const source = record(value, field);
  const volume = number(source.volume, `${field}.volume`);
  if (volume > 1) fail(`${field}.volume`);
  return frozen({
    giftId: string(source.giftId, `${field}.giftId`) as GiftSoundBinding['giftId'],
    giftName: string(source.giftName, `${field}.giftName`),
    soundFileName: nullableString(source.soundFileName, `${field}.soundFileName`),
    volume,
    isEnabled: boolean(source.isEnabled, `${field}.isEnabled`),
    isMissing: boolean(source.isMissing, `${field}.isMissing`)
  });
}

export function adaptGiftSoundBinding(value: unknown): GiftSoundBinding {
  return giftSoundBinding(value, 'giftSoundBinding');
}

export function adaptGiftSoundSnapshot(value: unknown): GiftSoundSnapshot {
  const source = record(value, 'giftSoundSnapshot');
  const globalVolume = number(source.globalVolume, 'giftSoundSnapshot.globalVolume');
  if (globalVolume > 1) fail('giftSoundSnapshot.globalVolume');
  return frozen({
    bindings: frozen(array(source.bindings, 'giftSoundSnapshot.bindings').map((item, index) => giftSoundBinding(item, `bindings.${index}`))),
    globalVolume,
    queueCapacityPerSecond: integer(source.queueCapacityPerSecond, 'giftSoundSnapshot.queueCapacityPerSecond', 1),
    updatedAt: integer(source.updatedAt, 'giftSoundSnapshot.updatedAt')
  });
}

function ttsStatus(value: unknown): TtsStatus {
  const source = record(value, 'ttsStatus');
  return frozen({
    state: enumeration(source.state, ['idle', 'speaking', 'paused', 'degraded', 'unavailable'], 'ttsStatus.state'),
    currentText: nullableString(source.currentText, 'ttsStatus.currentText'),
    pendingCount: integer(source.pendingCount, 'ttsStatus.pendingCount'),
    activeEngine: enumeration(source.activeEngine, ['system', 'cloud', 'localFallback'], 'ttsStatus.activeEngine'),
    fallbackReasonCode: nullableString(source.fallbackReasonCode, 'ttsStatus.fallbackReasonCode')
  });
}

function ttsConfig(value: unknown): TtsConfig {
  const source = record(value, 'ttsConfig');
  const templates = record(source.templates, 'ttsConfig.templates');
  const rate = number(source.rate, 'ttsConfig.rate');
  const volume = number(source.volume, 'ttsConfig.volume');
  if (volume > 1) fail('ttsConfig.volume');
  return frozen({
    isEnabled: boolean(source.isEnabled, 'ttsConfig.isEnabled'),
    selectedEngine: enumeration(source.selectedEngine, ['system', 'cloud'], 'ttsConfig.selectedEngine'),
    selectedVoiceId: nullableString(source.selectedVoiceId, 'ttsConfig.selectedVoiceId'),
    rate,
    volume,
    eventKinds: frozen(array(source.eventKinds, 'ttsConfig.eventKinds').map((item, index) => enumeration(item, ['danmu', 'gift', 'captain', 'entry'], `ttsConfig.eventKinds.${index}`))),
    identityKinds: frozen(array(source.identityKinds, 'ttsConfig.identityKinds').map((item, index) => enumeration(item, ['admin', 'captain', 'vip', 'ordinary'], `ttsConfig.identityKinds.${index}`))),
    templates: frozen({
      danmu: string(templates.danmu, 'ttsConfig.templates.danmu', true),
      gift: string(templates.gift, 'ttsConfig.templates.gift', true),
      captain: string(templates.captain, 'ttsConfig.templates.captain', true)
    })
  });
}

function ttsVoice(value: unknown, field: string): TtsVoice {
  const source = record(value, field);
  return frozen({
    voiceId: string(source.voiceId, `${field}.voiceId`),
    displayName: string(source.displayName, `${field}.displayName`),
    engine: enumeration(source.engine, ['system', 'cloud'], `${field}.engine`),
    isAvailable: boolean(source.isAvailable, `${field}.isAvailable`),
    unavailableReasonCode: nullableString(source.unavailableReasonCode, `${field}.unavailableReasonCode`)
  });
}

export function adaptTtsStatus(value: unknown): TtsStatus {
  return ttsStatus(value);
}

export function adaptTtsSnapshot(value: unknown): TtsSnapshot {
  const source = record(value, 'ttsSnapshot');
  return frozen({
    config: ttsConfig(source.config),
    voices: frozen(array(source.voices, 'ttsSnapshot.voices').map((item, index) => ttsVoice(item, `voices.${index}`))),
    status: ttsStatus(source.status),
    updatedAt: integer(source.updatedAt, 'ttsSnapshot.updatedAt')
  });
}

export function adaptRecordingConfig(value: unknown): RecordingConfig {
  const source = record(value, 'recordingConfig');
  const directorySource = source.directory === null ? null : record(source.directory, 'recordingConfig.directory');
  return frozen({
    format: enumeration(source.format, ['mp4', 'flv'], 'recordingConfig.format'),
    quality: enumeration(source.quality, ['original', 'high', 'standard'], 'recordingConfig.quality'),
    directory: directorySource === null
      ? null
      : frozen({
        grantId: string(directorySource.grantId, 'recordingConfig.directory.grantId') as NonNullable<RecordingConfig['directory']>['grantId'],
        displayPath: string(directorySource.displayPath, 'recordingConfig.directory.displayPath')
      }) as RecordingConfig['directory'],
    segmentationEnabled: boolean(source.segmentationEnabled, 'recordingConfig.segmentationEnabled'),
    segmentDurationSeconds: integer(source.segmentDurationSeconds, 'recordingConfig.segmentDurationSeconds', 1),
    autoRecord: boolean(source.autoRecord, 'recordingConfig.autoRecord'),
    danmakuThreshold: integer(source.danmakuThreshold, 'recordingConfig.danmakuThreshold', 1),
    highlightBeforeSeconds: numberChoice(source.highlightBeforeSeconds, [30], 'recordingConfig.highlightBeforeSeconds'),
    highlightAfterSeconds: numberChoice(source.highlightAfterSeconds, [30], 'recordingConfig.highlightAfterSeconds'),
    quotaBytes: integer(source.quotaBytes, 'recordingConfig.quotaBytes', 1),
    waterlineBytes: integer(source.waterlineBytes, 'recordingConfig.waterlineBytes', 1),
    ffmpegSource: enumeration(source.ffmpegSource, ['bundled', 'local'], 'recordingConfig.ffmpegSource'),
    shortcut: string(source.shortcut, 'recordingConfig.shortcut')
  });
}

export function adaptRecordingStatus(value: unknown): RecordingStatus {
  const source = record(value, 'recordingStatus');
  const roomId = nullableString(source.roomId, 'recordingStatus.roomId');
  if (roomId !== null && !/^\d{1,20}$/.test(roomId)) fail('recordingStatus.roomId');
  return frozen({
    state: enumeration(source.state, ['idle', 'starting', 'recording', 'stopping', 'finalizing', 'error'], 'recordingStatus.state'),
    sessionId: nullableString(source.sessionId, 'recordingStatus.sessionId'),
    roomId: roomId as RecordingStatus['roomId'],
    elapsedSeconds: integer(source.elapsedSeconds, 'recordingStatus.elapsedSeconds'),
    fileSizeBytes: integer(source.fileSizeBytes, 'recordingStatus.fileSizeBytes'),
    diskRemainingBytes: integer(source.diskRemainingBytes, 'recordingStatus.diskRemainingBytes'),
    highlightCount: integer(source.highlightCount, 'recordingStatus.highlightCount'),
    lastErrorCode: nullableString(source.lastErrorCode, 'recordingStatus.lastErrorCode')
  });
}

export function adaptRecordingSummary(value: unknown): RecordingSummary {
  const source = record(value, 'recordingSummary');
  return frozen({
    id: string(source.id, 'recordingSummary.id') as RecordingSummary['id'],
    fileName: string(source.fileName, 'recordingSummary.fileName'),
    format: enumeration(source.format, ['mp4', 'flv'], 'recordingSummary.format'),
    sizeBytes: integer(source.sizeBytes, 'recordingSummary.sizeBytes'),
    durationSeconds: integer(source.durationSeconds, 'recordingSummary.durationSeconds'),
    startedAt: integer(source.startedAt, 'recordingSummary.startedAt'),
    endedAt: source.endedAt === null ? null : integer(source.endedAt, 'recordingSummary.endedAt'),
    status: enumeration(source.status, ['recording', 'completed', 'partial', 'failed'], 'recordingSummary.status')
  });
}

export function adaptRecordingSummaries(value: unknown): readonly RecordingSummary[] {
  return frozen(array(value, 'recordingSummaries').map(adaptRecordingSummary));
}

export function adaptHighlightSummary(value: unknown): HighlightSummary {
  const source = record(value, 'highlightSummary');
  return frozen({
    id: string(source.id, 'highlightSummary.id') as HighlightSummary['id'],
    recordingId: string(source.recordingId, 'highlightSummary.recordingId') as HighlightSummary['recordingId'],
    positionSeconds: integer(source.positionSeconds, 'highlightSummary.positionSeconds'),
    triggerType: enumeration(source.triggerType, ['danmakuSurge', 'manual'], 'highlightSummary.triggerType'),
    clipDurationSeconds: integer(source.clipDurationSeconds, 'highlightSummary.clipDurationSeconds'),
    createdAt: integer(source.createdAt, 'highlightSummary.createdAt'),
    isClipAvailable: boolean(source.isClipAvailable, 'highlightSummary.isClipAvailable')
  });
}

export function adaptHighlightSummaries(value: unknown): readonly HighlightSummary[] {
  return frozen(array(value, 'highlightSummaries').map(adaptHighlightSummary));
}
