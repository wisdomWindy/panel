export type ReleaseChannel = 'free' | 'member';
export type UpdateStatus = 'unavailable' | 'current' | 'available' | 'downloading' | 'downloaded' | 'failed';
export type PluginPermission = 'sanitizedEvent' | 'currentRoomUid' | 'effect' | 'panel' | 'dataSource' | 'network';
export type PluginStatus = 'disabled' | 'enabled' | 'updateAvailable' | 'failed' | 'unavailable';

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

export interface PluginSnapshot {
  readonly id: string;
  readonly name: string;
  readonly version: string;
  readonly status: PluginStatus;
  readonly permissions: readonly PluginPermission[];
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
  plugin_list: undefined;
  plugin_select_install: undefined;
  plugin_install: { pluginId: string };
  plugin_toggle: { pluginId: string; isEnabled: boolean };
  plugin_uninstall: { pluginId: string };
  plugin_config_get: { pluginId: string };
  plugin_config_save: { pluginId: string };
  about_info: undefined;
  resilience_status: undefined;
}

export interface ReleaseCommandResultMap {
  update_status: UpdateSnapshot;
  update_check: UpdateSnapshot;
  update_download: UpdateSnapshot;
  update_install: UpdateSnapshot;
  update_skip: UpdateSnapshot;
  plugin_list: readonly PluginSnapshot[];
  plugin_select_install: PluginSnapshot;
  plugin_install: PluginSnapshot;
  plugin_toggle: PluginSnapshot;
  plugin_uninstall: boolean;
  plugin_config_get: boolean;
  plugin_config_save: boolean;
  about_info: AboutSnapshot;
  resilience_status: ResilienceSnapshot;
}

export interface ReleaseEventPayloadMap {
  'update://status': UpdateSnapshot;
  'plugin://status': PluginSnapshot;
  'plugin://crashed': PluginSnapshot;
}
