export type BuildChannel = 'free' | 'member';
export type BootstrapStatus = 'uninitialized' | 'initializing' | 'ready' | 'degraded';
export type ConfigScope = 'global' | 'room';

export type ConfigKey = string & { readonly __configKey: unique symbol };
export type PathGrantId = string & { readonly __pathGrantId: unique symbol };
export type RoomId = string & { readonly __roomId: unique symbol };
export type UserUid = string & { readonly __userUid: unique symbol };
export type EventId = string & { readonly __eventId: unique symbol };
export type GiftId = string & { readonly __giftId: unique symbol };

export type PathPurpose =
  | 'recordingDirectory'
  | 'exportDirectory'
  | 'giftSoundFile'
  | 'giftSoundImport'
  | 'giftSoundExport'
  | 'highlightExport'
  | 'localFfmpegExecutable'
  | 'sensitiveWordImport'
  | 'sensitiveWordExport'
  | 'blacklistImport'
  | 'blacklistExport'
  | 'statExport'
  | 'rankExport'
  | 'configImport'
  | 'configExport';

export interface DesktopError {
  code: string;
  message: string;
  details?: Readonly<Record<string, string>>;
  retryable: boolean;
}

export interface AppStatus {
  buildChannel: BuildChannel;
  schemaVersion: number;
  status: BootstrapStatus;
}

export interface ConfigValue {
  value?: string;
  updatedAt?: string;
}

export interface ConfigRevision {
  revision: string;
}

export interface PathGrant {
  grantId: PathGrantId;
  displayPath: string;
}

export interface BaseCommandRequestMap {
  app_bootstrap: undefined;
  app_status: undefined;
  config_get: { key: ConfigKey; scope: ConfigScope };
  config_set: { key: ConfigKey; scope: ConfigScope; value: string };
  path_select: { purpose: PathPurpose };
  path_revoke: { grantId: PathGrantId };
}

export interface BaseCommandResultMap {
  app_bootstrap: AppStatus;
  app_status: AppStatus;
  config_get: ConfigValue;
  config_set: ConfigRevision;
  path_select: PathGrant;
  path_revoke: { revoked: boolean };
}

export interface BaseEventPayloadMap {
  app_status_changed: { status: BootstrapStatus; retryable: boolean };
  migration_progress: {
    stage: 'preflight' | 'backup' | 'schema' | 'data' | 'complete';
    percentage: number;
  };
}

export function isDesktopError(value: unknown): value is DesktopError {
  if (typeof value !== 'object' || value === null) return false;
  const candidate = value as Partial<DesktopError>;
  return (
    typeof candidate.code === 'string' &&
    typeof candidate.message === 'string' &&
    typeof candidate.retryable === 'boolean'
  );
}
