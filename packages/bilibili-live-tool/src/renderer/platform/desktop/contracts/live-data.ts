import type { EventId, GiftId, PathGrantId, RoomId, UserUid } from './base';

export type ConnectionStatus =
  | 'disconnected'
  | 'connecting'
  | 'connected'
  | 'reconnecting'
  | 'error';

export type DataRange =
  | { kind: 'liveSession' }
  | { kind: 'today' }
  | { kind: 'last7Days' }
  | { kind: 'custom'; startAt: number; endAt: number };

export type IdentityTag = 'admin' | 'governor' | 'admiral' | 'captain' | 'vip';

export interface FanMedal {
  name: string;
  level: number | null;
}
export interface AudienceIdentity {
  userUid: UserUid;
  username: string;
  userLevel: number | null;
  medal: FanMedal | null;
  tags: readonly IdentityTag[];
}

interface LiveEventBase {
  eventId: EventId;
  roomId: RoomId;
  occurredAt: number;
  audience: AudienceIdentity;
}

export interface DanmuEvent extends LiveEventBase {
  kind: 'danmu';
  content: string;
}
export interface GiftEvent extends LiveEventBase {
  kind: 'gift';
  giftId: GiftId;
  giftName: string;
  giftIconUrl: string | null;
  quantity: number;
  valueMinor: number;
}
export interface CaptainEvent extends LiveEventBase {
  kind: 'captain';
  action: 'purchase' | 'renewal';
  level: 'governor' | 'admiral' | 'captain';
  valueMinor: number;
}
export interface EnterEvent extends LiveEventBase {
  kind: 'enter';
}
/** PRD 9.1.1: room session boundary; carries no viewer action of its own. */
export interface SessionEvent extends LiveEventBase {
  kind: 'sessionStarted' | 'sessionEnded';
}
export type LiveEvent = DanmuEvent | GiftEvent | CaptainEvent | EnterEvent | SessionEvent;

export interface RoomSnapshot {
  status: ConnectionStatus;
  roomId: RoomId | null;
  roomName: string | null;
  onlineCount: number | null;
  reconnectAttempt: number;
  lastErrorCode: string | null;
}

export type SensitiveWordCategory = 'illegal' | 'advertising' | 'personal_attack' | 'custom';
export type SensitiveMatchType = 'exact' | 'fuzzy';
export type SensitiveFileFormat = 'csv' | 'json' | 'txt';
export interface SensitiveWord {
  id: number;
  word: string;
  category: SensitiveWordCategory;
  matchType: SensitiveMatchType;
  isEnabled: boolean;
}
export interface SensitiveWordSnapshot {
  items: readonly SensitiveWord[];
  todayHitCount: number;
}
export interface ImportResult {
  importedCount: number;
  duplicateCount: number;
  errorCount: number;
  errors: readonly { line: number; code: string }[];
  isRolledBack: boolean;
}
export interface BlacklistEntry {
  uid: UserUid;
  username: string;
  reason: string | null;
  createdAt: number;
}
export interface BlacklistSnapshot {
  items: readonly BlacklistEntry[];
  todayAddedCount: number;
}
export interface AudienceDetail {
  identity: AudienceIdentity;
  sessionDanmuCount: number;
  giftValueMinor: number;
  isBlacklisted: boolean;
}
export interface StatMetrics {
  onlineCount: number;
  danmuCount: number;
  danmuRate: number;
  giftValueMinor: number;
  captainCount: number;
  filteredCount: number;
}
export interface StatTrendPoint extends StatMetrics {
  occurredAt: number;
}
export interface StatSnapshot {
  range: DataRange;
  metrics: StatMetrics;
  trendPoints: readonly StatTrendPoint[];
  generatedAt: number;
  isHistoricalDataAvailable: boolean;
  isStale: boolean;
}
export type RankChange = 'new' | 'up' | 'down' | 'same';
export interface MoneyRankingEntry {
  rank: number;
  userUid: UserUid;
  username: string;
  amountMinor: number;
  previousRank: number | null;
  change: RankChange;
}
export interface DanmuRankingEntry {
  rank: number;
  userUid: UserUid;
  username: string;
  danmuCount: number;
  previousRank: number | null;
  change: RankChange;
}
export interface RankingSnapshot {
  range: DataRange;
  gift: readonly MoneyRankingEntry[];
  danmu: readonly DanmuRankingEntry[];
  captain: readonly MoneyRankingEntry[];
  giftTotalUsers: number;
  danmuTotalUsers: number;
  captainTotalUsers: number;
  generatedAt: number;
  isHistoricalDataAvailable: boolean;
  isStale: boolean;
}
export type ExportType = 'sensitiveWords' | 'blacklist' | 'statistics' | 'rankings';
export interface ExportResult {
  exportType: ExportType;
  displayPath: string;
  exportedCount: number;
}

export const LIVE_DATA_COMMAND_NAMES = [
  'room_connect',
  'room_disconnect',
  'audience_detail_query',
  'sensitive_word_list',
  'sensitive_word_add',
  'sensitive_word_update',
  'sensitive_word_delete',
  'sensitive_word_toggle',
  'sensitive_word_import',
  'sensitive_word_export',
  'blacklist_list',
  'blacklist_add',
  'blacklist_remove',
  'blacklist_export',
  'blacklist_import',
  'stat_query',
  'stat_refresh',
  'stat_export',
  'rank_query',
  'rank_export'
] as const;
export const LIVE_DATA_EVENT_NAMES = [
  'room_status_changed',
  'danmu_received',
  'gift_received',
  'captain_received',
  'enter_received',
  'sensitive_word_compile_progress',
  'stat_updated',
  'rank_updated',
  'export_done'
] as const;

export interface LiveDataCommandRequestMap {
  room_connect: { identityCode: string };
  room_disconnect: undefined;
  audience_detail_query: { roomId: RoomId; userUid: UserUid };
  sensitive_word_list: { category?: SensitiveWordCategory; search?: string };
  sensitive_word_add: {
    word: string;
    category: SensitiveWordCategory;
    matchType: SensitiveMatchType;
    isEnabled: boolean;
  };
  sensitive_word_update: {
    id: number;
    word: string;
    category: SensitiveWordCategory;
    matchType: SensitiveMatchType;
    isEnabled: boolean;
  };
  sensitive_word_delete: { id: number };
  sensitive_word_toggle: { id: number; isEnabled: boolean };
  sensitive_word_import: { grantId: PathGrantId; format: SensitiveFileFormat };
  sensitive_word_export: { grantId: PathGrantId; format: SensitiveFileFormat };
  blacklist_list: undefined;
  blacklist_add: { uid: UserUid; username: string; reason: string | null };
  blacklist_remove: { uid: UserUid };
  blacklist_export: { grantId: PathGrantId };
  blacklist_import: { grantId: PathGrantId };
  stat_query: { range: DataRange };
  stat_refresh: { range: DataRange };
  stat_export: { range: DataRange; grantId: PathGrantId };
  rank_query: { range: DataRange };
  rank_export: { range: DataRange; grantId: PathGrantId };
}

export interface LiveDataCommandResultMap {
  room_connect: RoomSnapshot;
  room_disconnect: RoomSnapshot;
  audience_detail_query: AudienceDetail;
  sensitive_word_list: SensitiveWordSnapshot;
  sensitive_word_add: SensitiveWord;
  sensitive_word_update: SensitiveWord;
  sensitive_word_delete: { deleted: boolean };
  sensitive_word_toggle: SensitiveWord;
  sensitive_word_import: ImportResult;
  sensitive_word_export: ExportResult;
  blacklist_list: BlacklistSnapshot;
  blacklist_add: BlacklistEntry;
  blacklist_remove: { deleted: boolean };
  blacklist_export: ExportResult;
  blacklist_import: ImportResult;
  stat_query: StatSnapshot;
  stat_refresh: StatSnapshot;
  stat_export: ExportResult;
  rank_query: RankingSnapshot;
  rank_export: ExportResult;
}

export interface LiveDataEventPayloadMap {
  room_status_changed: {
    status: ConnectionStatus;
    roomId: RoomId | null;
    attempt: number;
    errorCode: string | null;
  };
  danmu_received: DanmuEvent;
  gift_received: GiftEvent;
  captain_received: CaptainEvent;
  enter_received: EnterEvent;
  sensitive_word_compile_progress: { percentage: number; stage: string };
  stat_updated: StatSnapshot;
  rank_updated: RankingSnapshot;
  export_done: { exportType: ExportType; displayPath: string };
}
