import type {
  AuthQrStatus,
  MemberAccount,
  MemberModule,
  MemberStatus,
  MembershipOrder,
  ModerationAlert,
  ModerationLogEntry,
  ModerationRules,
  ModerationSummary
} from '@/platform/desktop';

type UnknownRecord = Record<string, unknown>;

const MEMBER_SHELL_STATES = ['unavailable', 'loggedOut', 'unpaid', 'active', 'expired'] as const;
const CAPABILITY_STATES = ['unavailable', 'missingModule', 'ready', 'expired'] as const;
const MODULE_STATES = ['missing', 'downloading', 'verifying', 'installed', 'failed'] as const;
const ORDER_STATES = ['pending', 'paid', 'cancelled', 'failed', 'timedOut'] as const;
const ALERT_STATES = ['pending', 'executing', 'muted', 'ignored', 'failed'] as const;
const MUTE_DURATIONS = ['10m', '1h', 'permanent'] as const;
const MODERATION_ACTIONS = ['autoMute', 'manualMute', 'ignore', 'falsePositive'] as const;

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

function nullableInteger(value: unknown, label: string): number | null {
  return value === null ? null : integer(value, label);
}

export function adaptMemberAccount(value: unknown): MemberAccount {
  const source = record(value, 'member account');
  return Object.freeze({
    userUid: string(source.userUid, 'userUid') as MemberAccount['userUid'],
    username: string(source.username, 'username'),
    avatarUrl: nullableString(source.avatarUrl, 'avatarUrl'),
    loggedInAt: integer(source.loggedInAt, 'loggedInAt', 1)
  });
}

export function adaptAuthQrStatus(value: unknown): AuthQrStatus {
  const source = record(value, 'authentication QR status');
  const state = enumeration(
    source.state,
    ['pending', 'scanned', 'success', 'expired', 'error'] as const,
    'state'
  );
  const sessionId = string(source.sessionId, 'sessionId') as AuthQrStatus['sessionId'];
  if (state === 'pending' || state === 'scanned') {
    return Object.freeze({
      state,
      sessionId,
      imageBase64: string(source.imageBase64, 'imageBase64'),
      expiresInSeconds: integer(source.expiresInSeconds, 'expiresInSeconds')
    });
  }
  if (state === 'success') {
    return Object.freeze({ state, sessionId, account: adaptMemberAccount(source.account) });
  }
  if (state === 'error') {
    return Object.freeze({ state, sessionId, errorCode: string(source.errorCode, 'errorCode') });
  }
  return Object.freeze({ state, sessionId });
}

export function adaptMemberStatus(value: unknown): MemberStatus {
  const source = record(value, 'member status');
  const capabilities = record(source.capabilities, 'capabilities');
  const entitlement =
    source.entitlement === null ? null : record(source.entitlement, 'entitlement');
  return Object.freeze({
    shellState: enumeration(source.shellState, MEMBER_SHELL_STATES, 'shellState'),
    account: source.account === null ? null : adaptMemberAccount(source.account),
    entitlement:
      entitlement === null
        ? null
        : Object.freeze({
          status: enumeration(
            entitlement.status,
              ['active', 'expired'] as const,
              'entitlement.status'
          ),
          planId: string(entitlement.planId, 'entitlement.planId'),
          expiresAt: integer(entitlement.expiresAt, 'entitlement.expiresAt', 1),
          remainingDays: integer(entitlement.remainingDays, 'entitlement.remainingDays'),
          lastVerifiedAt: integer(entitlement.lastVerifiedAt, 'entitlement.lastVerifiedAt', 1),
          isOfflineValid: boolean(entitlement.isOfflineValid, 'entitlement.isOfflineValid')
        }),
    capabilities: Object.freeze({
      moderation: enumeration(capabilities.moderation, CAPABILITY_STATES, 'capabilities.moderation')
    }),
    serviceErrorCode: nullableString(source.serviceErrorCode, 'serviceErrorCode')
  });
}

export function adaptMemberModule(value: unknown): MemberModule {
  const source = record(value, 'member module');
  const progressPercent = integer(source.progressPercent, 'progressPercent');
  if (progressPercent > 100) throw new TypeError('progressPercent is invalid');
  return Object.freeze({
    moduleId: string(source.moduleId, 'moduleId') as MemberModule['moduleId'],
    name: string(source.name, 'name'),
    version: string(source.version, 'version'),
    status: enumeration(source.status, MODULE_STATES, 'status'),
    downloadedBytes: integer(source.downloadedBytes, 'downloadedBytes'),
    totalBytes: nullableInteger(source.totalBytes, 'totalBytes'),
    progressPercent,
    errorCode: nullableString(source.errorCode, 'errorCode')
  });
}

export function adaptMemberModules(value: unknown): readonly MemberModule[] {
  if (!Array.isArray(value)) throw new TypeError('modules must be an array');
  return Object.freeze(value.map(adaptMemberModule));
}

export function adaptMembershipOrder(value: unknown): MembershipOrder {
  const source = record(value, 'membership order');
  return Object.freeze({
    orderId: string(source.orderId, 'orderId') as MembershipOrder['orderId'],
    planId: string(source.planId, 'planId'),
    planLabel: string(source.planLabel, 'planLabel'),
    amountMinor: integer(source.amountMinor, 'amountMinor'),
    currency: string(source.currency, 'currency'),
    status: enumeration(source.status, ORDER_STATES, 'status'),
    paymentQrBase64: nullableString(source.paymentQrBase64, 'paymentQrBase64'),
    paymentUrl: nullableString(source.paymentUrl, 'paymentUrl'),
    createdAt: integer(source.createdAt, 'createdAt', 1),
    paidAt: nullableInteger(source.paidAt, 'paidAt')
  });
}

export function adaptMembershipOrders(value: unknown): readonly MembershipOrder[] {
  if (!Array.isArray(value)) throw new TypeError('orders must be an array');
  return Object.freeze(value.map(adaptMembershipOrder));
}

export function adaptModerationRules(value: unknown): ModerationRules {
  const source = record(value, 'moderation rules');
  const ratio = number(source.similarityRatio, 'similarityRatio');
  if (ratio > 1) throw new TypeError('similarityRatio is invalid');
  return Object.freeze({
    frequencyWindowSeconds: integer(source.frequencyWindowSeconds, 'frequencyWindowSeconds', 1),
    frequencyThreshold: integer(source.frequencyThreshold, 'frequencyThreshold', 1),
    repeatWindowSeconds: integer(source.repeatWindowSeconds, 'repeatWindowSeconds', 1),
    repeatThreshold: integer(source.repeatThreshold, 'repeatThreshold', 1),
    similarityWindowSeconds: integer(source.similarityWindowSeconds, 'similarityWindowSeconds', 1),
    similarityThreshold: integer(source.similarityThreshold, 'similarityThreshold', 1),
    similarityRatio: ratio,
    defaultMuteDuration: enumeration(
      source.defaultMuteDuration,
      MUTE_DURATIONS,
      'defaultMuteDuration'
    ),
    isRiskConfirmed: boolean(source.isRiskConfirmed, 'isRiskConfirmed'),
    isAutoEnabled: boolean(source.isAutoEnabled, 'isAutoEnabled')
  });
}

export function adaptModerationAlert(value: unknown): ModerationAlert {
  const source = record(value, 'moderation alert');
  return Object.freeze({
    alertId: string(source.alertId, 'alertId') as ModerationAlert['alertId'],
    userUid: string(source.userUid, 'userUid') as ModerationAlert['userUid'],
    username: string(source.username, 'username'),
    violationDescription: string(source.violationDescription, 'violationDescription'),
    contentExample: string(source.contentExample, 'contentExample'),
    occurredAt: integer(source.occurredAt, 'occurredAt', 1),
    status: enumeration(source.status, ALERT_STATES, 'status'),
    errorCode: nullableString(source.errorCode, 'errorCode')
  });
}

export function adaptModerationSummary(value: unknown): ModerationSummary {
  const source = record(value, 'moderation summary');
  if (!Array.isArray(source.alerts)) throw new TypeError('alerts must be an array');
  return Object.freeze({
    rules: adaptModerationRules(source.rules),
    alerts: Object.freeze(source.alerts.map(adaptModerationAlert)),
    recognizedToday: integer(source.recognizedToday, 'recognizedToday'),
    autoMutedToday: integer(source.autoMutedToday, 'autoMutedToday'),
    manualMutedToday: integer(source.manualMutedToday, 'manualMutedToday'),
    isCredentialAvailable: boolean(source.isCredentialAvailable, 'isCredentialAvailable')
  });
}

export function adaptModerationLogEntry(value: unknown): ModerationLogEntry {
  const source = record(value, 'moderation log entry');
  return Object.freeze({
    id: integer(source.id, 'id'),
    userUid: string(source.userUid, 'userUid') as ModerationLogEntry['userUid'],
    username: string(source.username, 'username'),
    action: enumeration(source.action, MODERATION_ACTIONS, 'action'),
    occurredAt: integer(source.occurredAt, 'occurredAt', 1),
    isFalsePositive: boolean(source.isFalsePositive, 'isFalsePositive')
  });
}

export function adaptModerationLogs(value: unknown): readonly ModerationLogEntry[] {
  if (!Array.isArray(value)) throw new TypeError('moderation logs must be an array');
  return Object.freeze(value.map(adaptModerationLogEntry));
}
