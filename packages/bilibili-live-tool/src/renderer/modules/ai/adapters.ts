import type {
  AiConfig,
  AiGeneration,
  AiLogEntry,
  AiModelProfile,
  AiOutputDevice,
  AiVoice,
  AiVoiceQueueItem,
  AiVoiceReplyRules,
  AiVoiceReplySnapshot
} from '@/platform/desktop';

type UnknownRecord = Record<string, unknown>;

const AI_STYLES = ['concise', 'natural', 'detailed'] as const;
const PROVIDER_KINDS = ['cloud', 'aggregator', 'local', 'customOpenAi'] as const;
const REASONING_EFFORTS = [
  'none',
  'minimal',
  'low',
  'medium',
  'high',
  'xhigh',
  'max'
] as const;
const CONNECTION_STATES = ['untested', 'testing', 'available', 'unavailable'] as const;
const TRIGGER_SCOPES = ['all', 'questions', 'captains', 'keywords', 'highValue'] as const;
const QUEUE_STATES = ['idle', 'speaking', 'paused', 'unavailable'] as const;
const GENERATION_STATES = [
  'assembling',
  'streaming',
  'completed',
  'failed',
  'rejected',
  'cancelled'
] as const;

function record(value: unknown, label: string): UnknownRecord {
  if (value === null || typeof value !== 'object' || Array.isArray(value)) {
    throw new TypeError(`${label} must be an object`);
  }
  return value as UnknownRecord;
}

function string(value: unknown, label: string, allowEmpty = false): string {
  if (typeof value !== 'string' || (!allowEmpty && value.length === 0)) {
    throw new TypeError(`${label} must be a string`);
  }
  return value;
}

function nullableString(value: unknown, label: string): string | null {
  return value === null ? null : string(value, label);
}

function number(value: unknown, label: string, minimum = 0): number {
  if (typeof value !== 'number' || !Number.isFinite(value) || value < minimum) {
    throw new TypeError(`${label} must be a finite number`);
  }
  return value;
}

function integer(value: unknown, label: string, minimum = 0): number {
  const result = number(value, label, minimum);
  if (!Number.isInteger(result)) throw new TypeError(`${label} must be an integer`);
  return result;
}

function boolean(value: unknown, label: string): boolean {
  if (typeof value !== 'boolean') throw new TypeError(`${label} must be a boolean`);
  return value;
}

function enumeration<const T extends readonly string[]>(
  value: unknown,
  values: T,
  label: string
): T[number] {
  if (typeof value !== 'string' || !values.includes(value)) {
    throw new TypeError(`${label} is invalid`);
  }
  return value as T[number];
}

function stringArray(value: unknown, label: string): readonly string[] {
  if (!Array.isArray(value)) throw new TypeError(`${label} must be an array`);
  return Object.freeze(value.map((item, index) => string(item, `${label}[${index}]`)));
}

function nullableRecord(value: unknown, label: string): UnknownRecord | null {
  return value === null ? null : record(value, label);
}

export function adaptAiConfig(value: unknown): AiConfig {
  const source = record(value, 'AI config');
  return Object.freeze({
    identity: string(source.identity, 'identity', true),
    speakingStyle: enumeration(source.speakingStyle, AI_STYLES, 'speakingStyle'),
    systemPrompt: string(source.systemPrompt, 'systemPrompt', true),
    maxLength: integer(source.maxLength, 'maxLength', 1),
    maxSentences: integer(source.maxSentences, 'maxSentences', 1),
    isSensitiveFilterEnabled: boolean(source.isSensitiveFilterEnabled, 'isSensitiveFilterEnabled'),
    bannedWords: stringArray(source.bannedWords, 'bannedWords'),
    danmakuContextCount: integer(source.danmakuContextCount, 'danmakuContextCount'),
    manualContextCount: integer(source.manualContextCount, 'manualContextCount')
  });
}

export function adaptAiModelProfile(value: unknown): AiModelProfile {
  const source = record(value, 'AI model profile');
  return Object.freeze({
    profileId: string(source.profileId, 'profileId') as AiModelProfile['profileId'],
    name: string(source.name, 'name'),
    providerKind: enumeration(source.providerKind, PROVIDER_KINDS, 'providerKind'),
    baseUrl: string(source.baseUrl, 'baseUrl'),
    model: string(source.model, 'model'),
    reasoningEffort: enumeration(
      source.reasoningEffort,
      REASONING_EFFORTS,
      'reasoningEffort'
    ),
    maxTokens: integer(source.maxTokens, 'maxTokens', 1),
    timeoutSeconds: integer(source.timeoutSeconds, 'timeoutSeconds', 1),
    proxyUrl: nullableString(source.proxyUrl, 'proxyUrl'),
    isEnabled: boolean(source.isEnabled, 'isEnabled'),
    isDefault: boolean(source.isDefault, 'isDefault'),
    connectionState: enumeration(source.connectionState, CONNECTION_STATES, 'connectionState'),
    isApiKeyConfigured: boolean(source.isApiKeyConfigured, 'isApiKeyConfigured'),
    updatedAt: integer(source.updatedAt, 'updatedAt', 1)
  });
}

export function adaptAiModelProfiles(value: unknown): readonly AiModelProfile[] {
  if (!Array.isArray(value)) throw new TypeError('AI model profiles must be an array');
  return Object.freeze(value.map(adaptAiModelProfile));
}

function adaptAiVoiceRules(value: unknown): AiVoiceReplyRules {
  const source = record(value, 'AI voice rules');
  return Object.freeze({
    triggerScope: enumeration(source.triggerScope, TRIGGER_SCOPES, 'triggerScope'),
    keywords: stringArray(source.keywords, 'keywords'),
    globalCooldownSeconds: integer(source.globalCooldownSeconds, 'globalCooldownSeconds', 1),
    userCooldownSeconds: integer(source.userCooldownSeconds, 'userCooldownSeconds', 1),
    queueCapacity: integer(source.queueCapacity, 'queueCapacity', 1),
    isDuplicateFilterEnabled: boolean(source.isDuplicateFilterEnabled, 'isDuplicateFilterEnabled')
  });
}

export function adaptAiOutputDevice(value: unknown): AiOutputDevice {
  const source = record(value, 'AI output device');
  return Object.freeze({
    deviceId: string(source.deviceId, 'deviceId') as AiOutputDevice['deviceId'],
    displayName: string(source.displayName, 'displayName'),
    isAvailable: boolean(source.isAvailable, 'isAvailable'),
    isDefault: boolean(source.isDefault, 'isDefault')
  });
}

function adaptAiVoice(value: unknown): AiVoice {
  const source = record(value, 'AI voice');
  return Object.freeze({
    voiceId: string(source.voiceId, 'voiceId') as AiVoice['voiceId'],
    displayName: string(source.displayName, 'displayName'),
    isAvailable: boolean(source.isAvailable, 'isAvailable'),
    isDefault: boolean(source.isDefault, 'isDefault')
  });
}

function adaptAiVoiceQueueItem(value: unknown): AiVoiceQueueItem {
  const source = record(value, 'AI voice queue item');
  return Object.freeze({
    itemId: string(source.itemId, 'itemId') as AiVoiceQueueItem['itemId'],
    text: string(source.text, 'text'),
    priority: integer(source.priority, 'priority'),
    createdAt: integer(source.createdAt, 'createdAt', 1)
  });
}

export function adaptAiVoiceReplySnapshot(value: unknown): AiVoiceReplySnapshot {
  const source = record(value, 'AI voice reply snapshot');
  const queue = record(source.queue, 'AI voice queue');
  const current = nullableRecord(queue.current, 'AI voice queue current');
  if (!Array.isArray(source.devices) || !Array.isArray(source.voices) || !Array.isArray(queue.pending)) {
    throw new TypeError('AI voice reply collections are invalid');
  }
  const volume = number(queue.volume, 'volume');
  if (volume > 1) throw new TypeError('volume is invalid');
  return Object.freeze({
    isEnabled: boolean(source.isEnabled, 'isEnabled'),
    defaultProfileId: nullableString(source.defaultProfileId, 'defaultProfileId') as AiVoiceReplySnapshot['defaultProfileId'],
    rules: adaptAiVoiceRules(source.rules),
    devices: Object.freeze(source.devices.map(adaptAiOutputDevice)),
    voices: Object.freeze(source.voices.map(adaptAiVoice)),
    queue: Object.freeze({
      state: enumeration(queue.state, QUEUE_STATES, 'queue.state'),
      current: current ? adaptAiVoiceQueueItem(current) : null,
      pending: Object.freeze(queue.pending.map(adaptAiVoiceQueueItem)),
      capacity: integer(queue.capacity, 'queue.capacity', 1),
      volume,
      selectedDeviceId: nullableString(queue.selectedDeviceId, 'queue.selectedDeviceId') as AiVoiceReplySnapshot['queue']['selectedDeviceId'],
      selectedVoiceId: string(queue.selectedVoiceId, 'queue.selectedVoiceId') as AiVoiceReplySnapshot['queue']['selectedVoiceId'],
      isAudioRouteConfirmed: boolean(queue.isAudioRouteConfirmed, 'queue.isAudioRouteConfirmed'),
      lastErrorCode: nullableString(queue.lastErrorCode, 'queue.lastErrorCode')
    })
  });
}

export function adaptAiGeneration(value: unknown): AiGeneration {
  const source = record(value, 'AI generation');
  return Object.freeze({
    generationId: string(source.generationId, 'generationId') as AiGeneration['generationId'],
    status: enumeration(source.status, GENERATION_STATES, 'status'),
    result: string(source.result, 'result', true),
    lastSequence: integer(source.lastSequence, 'lastSequence'),
    errorCode: nullableString(source.errorCode, 'errorCode')
  });
}

export function adaptAiLogEntry(value: unknown): AiLogEntry {
  const source = record(value, 'AI log entry');
  return Object.freeze({
    id: integer(source.id, 'id'),
    operationType: string(source.operationType, 'operationType'),
    username: nullableString(source.username, 'username'),
    userUid: nullableString(source.userUid, 'userUid') as AiLogEntry['userUid'],
    originalContent: nullableString(source.originalContent, 'originalContent'),
    aiContent: nullableString(source.aiContent, 'aiContent'),
    generationStatus: string(source.generationStatus, 'generationStatus'),
    rejectReason: nullableString(source.rejectReason, 'rejectReason'),
    roomId: string(source.roomId, 'roomId'),
    createdAt: integer(source.createdAt, 'createdAt', 1)
  });
}

export function adaptAiLogs(value: unknown): readonly AiLogEntry[] {
  if (!Array.isArray(value)) throw new TypeError('AI logs must be an array');
  return Object.freeze(value.map(adaptAiLogEntry));
}
