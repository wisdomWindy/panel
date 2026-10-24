import type { UserUid } from './base';

export type AuthSessionId = string & { readonly __authSessionId: unique symbol };
export type OrderId = string & { readonly __orderId: unique symbol };
export type ModuleId = string & { readonly __moduleId: unique symbol };
export type MemberOperationId = string & { readonly __memberOperationId: unique symbol };
export type ModerationAlertId = string & { readonly __moderationAlertId: unique symbol };

export type MemberShellState = 'unavailable' | 'loggedOut' | 'unpaid' | 'active' | 'expired';
export type MemberCapabilityState = 'unavailable' | 'missingModule' | 'ready' | 'expired';

export interface MemberAccount {
  readonly userUid: UserUid;
  readonly username: string;
  readonly avatarUrl: string | null;
  readonly loggedInAt: number;
}

export interface MemberEntitlement {
  readonly status: 'active' | 'expired';
  readonly planId: string;
  readonly expiresAt: number;
  readonly remainingDays: number;
  readonly lastVerifiedAt: number;
  readonly isOfflineValid: boolean;
}

export interface MemberCapabilities {
  readonly moderation: MemberCapabilityState;
}

export interface MemberStatus {
  readonly shellState: MemberShellState;
  readonly account: MemberAccount | null;
  readonly entitlement: MemberEntitlement | null;
  readonly capabilities: MemberCapabilities;
  readonly serviceErrorCode: string | null;
}

export type AuthQrStatus =
  | { readonly state: 'pending'; readonly sessionId: AuthSessionId; readonly imageBase64: string; readonly expiresInSeconds: number }
  | { readonly state: 'scanned'; readonly sessionId: AuthSessionId; readonly imageBase64: string; readonly expiresInSeconds: number }
  | { readonly state: 'success'; readonly sessionId: AuthSessionId; readonly account: MemberAccount }
  | { readonly state: 'expired'; readonly sessionId: AuthSessionId }
  | { readonly state: 'error'; readonly sessionId: AuthSessionId; readonly errorCode: string };

export interface MembershipPlan {
  readonly planId: string;
  readonly label: string;
  readonly amountMinor: number;
  readonly currency: string;
  readonly savingsLabel: string | null;
}

export type MembershipOrderStatus = 'pending' | 'paid' | 'cancelled' | 'failed' | 'timedOut';

export interface MembershipOrder {
  readonly orderId: OrderId;
  readonly planId: string;
  readonly planLabel: string;
  readonly amountMinor: number;
  readonly currency: string;
  readonly status: MembershipOrderStatus;
  readonly paymentQrBase64: string | null;
  readonly paymentUrl: string | null;
  readonly createdAt: number;
  readonly paidAt: number | null;
}

export type MemberModuleStatus = 'missing' | 'downloading' | 'verifying' | 'installed' | 'failed';

export interface MemberModule {
  readonly moduleId: ModuleId;
  readonly name: string;
  readonly version: string;
  readonly status: MemberModuleStatus;
  readonly downloadedBytes: number;
  readonly totalBytes: number | null;
  readonly progressPercent: number;
  readonly errorCode: string | null;
}

export interface ModuleOperation {
  readonly operationId: MemberOperationId;
  readonly modules: readonly MemberModule[];
  readonly totalProgressPercent: number;
  readonly isTerminal: boolean;
}

export type MuteDuration = '10m' | '1h' | 'permanent';

export interface ModerationRules {
  readonly frequencyWindowSeconds: number;
  readonly frequencyThreshold: number;
  readonly repeatWindowSeconds: number;
  readonly repeatThreshold: number;
  readonly similarityWindowSeconds: number;
  readonly similarityThreshold: number;
  readonly similarityRatio: number;
  readonly defaultMuteDuration: MuteDuration;
  readonly isRiskConfirmed: boolean;
  readonly isAutoEnabled: boolean;
}

export interface ModerationAlert {
  readonly alertId: ModerationAlertId;
  readonly userUid: UserUid;
  readonly username: string;
  readonly violationDescription: string;
  readonly contentExample: string;
  readonly occurredAt: number;
  readonly status: 'pending' | 'executing' | 'muted' | 'ignored' | 'failed';
  readonly errorCode: string | null;
}

export interface ModerationSummary {
  readonly rules: ModerationRules;
  readonly alerts: readonly ModerationAlert[];
  readonly recognizedToday: number;
  readonly autoMutedToday: number;
  readonly manualMutedToday: number;
  readonly isCredentialAvailable: boolean;
}

export interface ModerationLogEntry {
  readonly id: number;
  readonly userUid: UserUid;
  readonly username: string;
  readonly action: 'autoMute' | 'manualMute' | 'ignore' | 'falsePositive';
  readonly occurredAt: number;
  readonly isFalsePositive: boolean;
}

export const MEMBER_COMMAND_NAMES = [
  'member_status',
  'auth_qrcode_generate',
  'auth_qrcode_cancel',
  'auth_query_uid',
  'auth_bind_uid',
  'auth_logout',
  'membership_purchase',
  'membership_cancel_order',
  'membership_orders',
  'module_list',
  'module_update',
  'module_cleanup',
  'moderate_status',
  'moderate_set_auto',
  'moderate_update_rules',
  'moderate_mute',
  'moderate_ignore',
  'moderate_log_query',
  'moderate_mark_false_positive'
] as const;

export interface MemberCommandRequestMap {
  member_status: undefined;
  auth_qrcode_generate: undefined;
  auth_qrcode_cancel: { sessionId: AuthSessionId };
  auth_query_uid: { userUid: UserUid };
  auth_bind_uid: { account: MemberAccount };
  auth_logout: undefined;
  membership_purchase: { planId: string };
  membership_cancel_order: { orderId: OrderId };
  membership_orders: undefined;
  module_list: undefined;
  module_update: { moduleId: ModuleId };
  module_cleanup: { moduleId: ModuleId };
  moderate_status: undefined;
  moderate_set_auto: { isEnabled: boolean; isRiskAccepted: boolean };
  moderate_update_rules: { rules: ModerationRules };
  moderate_mute: { alertId: ModerationAlertId; duration: MuteDuration };
  moderate_ignore: { alertId: ModerationAlertId };
  moderate_log_query: { userUid: UserUid | null; startedAt: number | null; endedAt: number | null };
  moderate_mark_false_positive: { logId: number };
}

export interface MemberCommandResultMap {
  member_status: MemberStatus;
  auth_qrcode_generate: AuthQrStatus;
  auth_qrcode_cancel: { cancelled: boolean };
  auth_query_uid: MemberAccount;
  auth_bind_uid: MemberStatus;
  auth_logout: MemberStatus;
  membership_purchase: MembershipOrder;
  membership_cancel_order: MembershipOrder;
  membership_orders: readonly MembershipOrder[];
  module_list: readonly MemberModule[];
  module_update: ModuleOperation;
  module_cleanup: { removed: boolean; modules: readonly MemberModule[] };
  moderate_status: ModerationSummary;
  moderate_set_auto: ModerationSummary;
  moderate_update_rules: ModerationSummary;
  moderate_mute: ModerationAlert;
  moderate_ignore: ModerationSummary;
  moderate_log_query: readonly ModerationLogEntry[];
  moderate_mark_false_positive: readonly ModerationLogEntry[];
}

export interface MemberEventPayloadMap {
  auth_qrcode_status: AuthQrStatus;
  auth_logged_in: MemberAccount;
  membership_expired: MemberStatus;
  membership_renewed: MemberStatus;
  module_download_progress: ModuleOperation;
  module_installed: MemberModule;
  moderation_alert: ModerationAlert;
  moderation_action_result: ModerationAlert;
}
