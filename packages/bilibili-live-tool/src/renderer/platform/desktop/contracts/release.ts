export type ReleaseChannel = 'free' | 'member';
export type UpdateStatus = 'unavailable' | 'current' | 'available' | 'downloading' | 'downloaded' | 'failed';

export interface UpdateCandidate {
  readonly version: string;
  readonly releaseNotes: readonly string[];
  readonly rolloutPercent: number;
  readonly isPaused: boolean;
  readonly isForced: boolean;
  readonly blacklistedVersions: readonly string[];
}

export interface UpdateSnapshot {
  readonly currentVersion: string;
  readonly channel: ReleaseChannel;
  readonly status: UpdateStatus;
  readonly candidate: UpdateCandidate | null;
  readonly progressPercent: number | null;
  readonly errorCode: string | null;
}

export interface LicenseDescriptor {
  readonly name: string;
  readonly license: string;
  readonly sourceUrl: string;
}

export interface AboutSnapshot {
  readonly applicationVersion: string;
  readonly channel: ReleaseChannel;
  readonly licenses: readonly LicenseDescriptor[];
}

export interface ResilienceSnapshot {
  readonly migrationBackupCount: number;
  readonly lastMigrationBackupAt: number | null;
  readonly retentionPolicyAvailable: boolean;
  readonly ffmpegArtifactAvailable: boolean;
  readonly whisperArtifactAvailable: boolean;
}

export interface ReleaseCommandRequestMap {
  update_status: undefined;
  update_check: undefined;
  update_download: undefined;
  update_install: undefined;
  update_skip: { version: string };
  about_info: undefined;
  resilience_status: undefined;
}

export interface ReleaseCommandResultMap {
  update_status: UpdateSnapshot;
  update_check: UpdateSnapshot;
  update_download: UpdateSnapshot;
  update_install: UpdateSnapshot;
  update_skip: UpdateSnapshot;
  about_info: AboutSnapshot;
  resilience_status: ResilienceSnapshot;
}

export interface ReleaseEventPayloadMap {
  'update://status': UpdateSnapshot;
}
