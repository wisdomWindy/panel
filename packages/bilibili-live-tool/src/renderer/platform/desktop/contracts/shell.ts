import type { PathGrantId } from './base';

export type LocaleCode = 'zh-CN' | 'en-US';
export type ThemeMode = 'light' | 'dark' | 'system';
export type FontSize = 'small' | 'medium' | 'large';
export type LayoutMode = 'default' | 'custom';
export type MemberShellStatus = 'unavailable' | 'loggedOut' | 'unpaid' | 'active' | 'expired';
export type ShellConfigScope =
  | 'room'
  | 'sensitiveWords'
  | 'blacklist'
  | 'giftSounds'
  | 'interfaceLayout'
  | 'recording'
  | 'plugins'
  | 'ai'
  | 'moderation';

export interface SystemMetrics {
  memoryBytes: number | null;
  cpuPercent: number | null;
  observedAt: number;
}

export interface LocalePreferences {
  locale: LocaleCode;
  followSystem: boolean;
}

export interface FourRegionLayout {
  version: 1;
  navigationWidth: number;
  toolbarHeight: number;
  statusHeight: number;
}

export interface UiPreferences {
  theme: ThemeMode;
  accentColor: string;
  opacity: number;
  fontSize: FontSize;
  layoutMode: LayoutMode;
  customLayout: FourRegionLayout | null;
  showDanmakuLevel: boolean;
  showDanmakuMedal: boolean;
  showDanmakuTime: boolean;
  maxDanmakuEntries: number;
}

export interface ConfigScopeOption {
  id: ShellConfigScope;
  isAvailable: boolean;
}

export interface ConfigOverview {
  displayPath: string;
  lastBackupAt: number | null;
  scopes: readonly ConfigScopeOption[];
}

export interface ConfigExportResult {
  revision: string;
  displayPath: string;
}

export interface ConfigImportResult {
  revision: string;
  importedScopes: readonly ShellConfigScope[];
}

export interface ConfigResetResult {
  revision: string;
  resetScopes: readonly ShellConfigScope[];
}

export interface ShellCommandRequestMap {
  shell_status: undefined;
  config_overview: undefined;
  config_open_directory: undefined;
  config_export: { grantId: PathGrantId; scopes: readonly ShellConfigScope[] };
  config_import: { grantId: PathGrantId };
  config_reset: undefined;
  locale_get: undefined;
  locale_set_auto: { isEnabled: boolean };
  locale_save: { preferences: LocalePreferences };
  ui_get: undefined;
  ui_preview: { preferences: UiPreferences };
  ui_save: { preferences: UiPreferences };
  ui_reset: undefined;
}

export interface ShellCommandResultMap {
  shell_status: SystemMetrics;
  config_overview: ConfigOverview;
  config_open_directory: { opened: true };
  config_export: ConfigExportResult;
  config_import: ConfigImportResult;
  config_reset: ConfigResetResult;
  locale_get: LocalePreferences;
  locale_set_auto: LocalePreferences;
  locale_save: LocalePreferences;
  ui_get: UiPreferences;
  ui_preview: UiPreferences;
  ui_save: UiPreferences;
  ui_reset: UiPreferences;
}

export interface ShellEventPayloadMap {
  shell_status_changed: SystemMetrics;
  locale_changed: LocalePreferences;
  ui_preferences_changed: UiPreferences;
}

export const SHELL_COMMAND_NAMES = [
  'shell_status',
  'config_overview',
  'config_open_directory',
  'config_export',
  'config_import',
  'config_reset',
  'locale_get',
  'locale_set_auto',
  'locale_save',
  'ui_get',
  'ui_preview',
  'ui_save',
  'ui_reset'
] as const satisfies readonly (keyof ShellCommandRequestMap)[];

export const SHELL_EVENT_NAMES = [
  'shell_status_changed',
  'locale_changed',
  'ui_preferences_changed'
] as const satisfies readonly (keyof ShellEventPayloadMap)[];
