import { defineStore } from 'pinia';
import { ref } from 'vue';

export interface AppConfig {
  theme: 'light' | 'dark' | 'auto';
  language: 'zh-CN' | 'en-US';
  fontSize: number;
  enableNotifications: boolean;
  enableLog: boolean;
  logLevel: 'debug' | 'info' | 'warn' | 'error';
  maxLogLines: number;
  checkUpdates: boolean;
  analyticsEnabled: boolean;
}

const defaultConfig = (): AppConfig => ({
  theme: 'auto',
  language: 'zh-CN',
  fontSize: 14,
  enableNotifications: true,
  enableLog: true,
  logLevel: 'info',
  maxLogLines: 1000,
  checkUpdates: true,
  analyticsEnabled: false
});

function mergeAppConfig(current: AppConfig, updates: unknown): AppConfig {
  if (!updates || typeof updates !== 'object') return current;
  const source = updates as Record<string, unknown>;
  const numberValue = (key: keyof AppConfig, fallback: number) =>
    typeof source[key] === 'number' ? source[key] : fallback;
  const booleanValue = (key: keyof AppConfig, fallback: boolean) =>
    typeof source[key] === 'boolean' ? source[key] : fallback;
  const theme = source.theme;
  const language = source.language;
  const logLevel = source.logLevel;

  return {
    theme: theme === 'light' || theme === 'dark' || theme === 'auto' ? theme : current.theme,
    language: language === 'zh-CN' || language === 'en-US' ? language : current.language,
    fontSize: numberValue('fontSize', current.fontSize),
    enableNotifications: booleanValue('enableNotifications', current.enableNotifications),
    enableLog: booleanValue('enableLog', current.enableLog),
    logLevel:
      logLevel === 'debug' || logLevel === 'info' || logLevel === 'warn' || logLevel === 'error'
        ? logLevel
        : current.logLevel,
    maxLogLines: numberValue('maxLogLines', current.maxLogLines),
    checkUpdates: booleanValue('checkUpdates', current.checkUpdates),
    analyticsEnabled: booleanValue('analyticsEnabled', current.analyticsEnabled)
  };
}

export const useConfigStore = defineStore('config', () => {
  const config = ref<AppConfig>(defaultConfig());

  function saveConfig(): void {
    try {
      localStorage.setItem('bilibili-live-config', JSON.stringify(config.value));
    } catch (error) {
      console.error('Failed to save config:', error);
    }
  }

  function updateConfig(updates: Partial<AppConfig> | Record<string, unknown>): void {
    config.value = mergeAppConfig(config.value, updates);
    saveConfig();
  }

  function resetConfig(): void {
    config.value = defaultConfig();
    saveConfig();
  }

  function loadConfig(): void {
    try {
      const saved = localStorage.getItem('bilibili-live-config');
      if (saved) config.value = mergeAppConfig(config.value, JSON.parse(saved));
    } catch (error) {
      console.error('Failed to load config:', error);
    }
  }

  function exportConfig(): AppConfig {
    return { ...config.value };
  }

  function importConfig(value: AppConfig): void {
    config.value = mergeAppConfig(config.value, value);
    saveConfig();
  }

  loadConfig();

  return {
    config,
    updateConfig,
    resetConfig,
    saveConfig,
    loadConfig,
    exportConfig,
    importConfig
  };
});
