import type {
  ConfigOverview,
  FourRegionLayout,
  LocalePreferences,
  SystemMetrics,
  UiPreferences
} from '@/platform/desktop';

function record(value: unknown, name: string): Record<string, unknown> {
  if (typeof value !== 'object' || value === null || Array.isArray(value)) {
    throw new TypeError(`${name} must be an object`);
  }
  return value as Record<string, unknown>;
}

function finite(value: unknown, name: string): number {
  if (typeof value !== 'number' || !Number.isFinite(value)) {
    throw new TypeError(`${name} must be finite`);
  }
  return value;
}

function boolean(value: unknown, name: string): boolean {
  if (typeof value !== 'boolean') throw new TypeError(`${name} must be boolean`);
  return value;
}

function layout(value: unknown): FourRegionLayout {
  const source = record(value, 'customLayout');
  if (
    source.version !== 1 ||
    finite(source.navigationWidth, 'navigationWidth') < 176 ||
    finite(source.navigationWidth, 'navigationWidth') > 280 ||
    finite(source.toolbarHeight, 'toolbarHeight') < 48 ||
    finite(source.toolbarHeight, 'toolbarHeight') > 64 ||
    finite(source.statusHeight, 'statusHeight') < 28 ||
    finite(source.statusHeight, 'statusHeight') > 40
  ) {
    throw new TypeError('customLayout is outside the supported four-region bounds');
  }
  return {
    version: 1,
    navigationWidth: source.navigationWidth as number,
    toolbarHeight: source.toolbarHeight as number,
    statusHeight: source.statusHeight as number
  };
}

export function adaptSystemMetrics(value: unknown): SystemMetrics {
  const source = record(value, 'systemMetrics');
  const memoryBytes = source.memoryBytes === null ? null : finite(source.memoryBytes, 'memoryBytes');
  const cpuPercent = source.cpuPercent === null ? null : finite(source.cpuPercent, 'cpuPercent');
  const observedAt = finite(source.observedAt, 'observedAt');
  if (memoryBytes !== null && memoryBytes < 0) throw new TypeError('memoryBytes must be non-negative');
  if (cpuPercent !== null && (cpuPercent < 0 || cpuPercent > 100)) {
    throw new TypeError('cpuPercent must be between zero and one hundred');
  }
  if (observedAt < 0) throw new TypeError('observedAt must be non-negative');
  return { memoryBytes, cpuPercent, observedAt };
}

export function adaptLocalePreferences(value: unknown): LocalePreferences {
  const source = record(value, 'localePreferences');
  if (source.locale !== 'zh-CN' && source.locale !== 'en-US') {
    throw new TypeError('locale is unsupported');
  }
  return { locale: source.locale, followSystem: boolean(source.followSystem, 'followSystem') };
}

export function adaptUiPreferences(value: unknown): UiPreferences {
  const source = record(value, 'uiPreferences');
  if (!['light', 'dark', 'system'].includes(source.theme as string)) {
    throw new TypeError('theme is invalid');
  }
  if (typeof source.accentColor !== 'string' || !/^#[0-9A-F]{6}$/.test(source.accentColor)) {
    throw new TypeError('accentColor is invalid');
  }
  const accentColor = source.accentColor === '#FF6699' ? '#FB7299' : source.accentColor;
  const opacity = finite(source.opacity, 'opacity');
  if (opacity < 0 || opacity > 1) throw new TypeError('opacity is invalid');
  if (!['small', 'medium', 'large'].includes(source.fontSize as string)) {
    throw new TypeError('fontSize is invalid');
  }
  if (source.layoutMode !== 'default' && source.layoutMode !== 'custom') {
    throw new TypeError('layoutMode is invalid');
  }
  const customLayout = source.customLayout === null ? null : layout(source.customLayout);
  if ((source.layoutMode === 'default') !== (customLayout === null)) {
    throw new TypeError('layoutMode and customLayout disagree');
  }
  const maxDanmakuEntries = finite(source.maxDanmakuEntries, 'maxDanmakuEntries');
  if (!Number.isInteger(maxDanmakuEntries) || maxDanmakuEntries < 1 || maxDanmakuEntries > 10_000) {
    throw new TypeError('maxDanmakuEntries is invalid');
  }
  return {
    theme: source.theme as UiPreferences['theme'],
    accentColor,
    opacity,
    fontSize: source.fontSize as UiPreferences['fontSize'],
    layoutMode: source.layoutMode,
    customLayout,
    showDanmakuLevel: boolean(source.showDanmakuLevel, 'showDanmakuLevel'),
    showDanmakuMedal: boolean(source.showDanmakuMedal, 'showDanmakuMedal'),
    showDanmakuTime: boolean(source.showDanmakuTime, 'showDanmakuTime'),
    maxDanmakuEntries
  };
}

export function cloneUiPreferences(value: UiPreferences): UiPreferences {
  return {
    theme: value.theme,
    accentColor: value.accentColor,
    opacity: value.opacity,
    fontSize: value.fontSize,
    layoutMode: value.layoutMode,
    customLayout: value.customLayout === null ? null : { ...value.customLayout },
    showDanmakuLevel: value.showDanmakuLevel,
    showDanmakuMedal: value.showDanmakuMedal,
    showDanmakuTime: value.showDanmakuTime,
    maxDanmakuEntries: value.maxDanmakuEntries
  };
}

export function adaptConfigOverview(value: unknown): ConfigOverview {
  const source = record(value, 'configOverview');
  if (typeof source.displayPath !== 'string' || !Array.isArray(source.scopes)) {
    throw new TypeError('configOverview fields are invalid');
  }
  const knownScopes = import.meta.env.VITE_BUILD_CHANNEL === 'member'
    ? [
      'room', 'sensitiveWords', 'blacklist', 'giftSounds', 'interfaceLayout',
      'recording', 'ai', 'moderation'
    ]
    : [
      'room', 'sensitiveWords', 'blacklist', 'giftSounds', 'interfaceLayout',
      'recording'
    ];
  const scopes = source.scopes.map(item => {
    const option = record(item, 'configScope');
    if (!knownScopes.includes(option.id as string)) {
      throw new TypeError('config scope is invalid');
    }
    return { id: option.id as ConfigOverview['scopes'][number]['id'], isAvailable: boolean(option.isAvailable, 'isAvailable') };
  });
  const lastBackupAt = source.lastBackupAt === null ? null : finite(source.lastBackupAt, 'lastBackupAt');
  if (lastBackupAt !== null && lastBackupAt < 0) throw new TypeError('lastBackupAt must be non-negative');
  return { displayPath: source.displayPath, lastBackupAt, scopes: Object.freeze(scopes) };
}
