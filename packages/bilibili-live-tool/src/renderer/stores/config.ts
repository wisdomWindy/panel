/**
 * 配置状态管理
 */

import { defineStore } from 'pinia';
import { ref } from 'vue';

export interface AppConfig {
  // 开放平台配置
  accessKeyId: string;
  accessKeySecret: string;
  appId: number;

  // 直播间配置
  autoReconnect: boolean;
  reconnectInterval: number;
  maxReconnectAttempts: number;
  heartbeatInterval: number;

  // 自动录制配置
  enableAutoRecord: boolean;

  // 敏感词过滤
  enableSensitiveWordFilter: boolean;
  sensitiveWords: string[];
  replaceChar: string;

  // UI 配置
  theme: 'light' | 'dark' | 'auto';
  language: 'zh-CN' | 'en-US';
  fontSize: number;
  enableNotifications: boolean;

  // 日志配置
  enableLog: boolean;
  logLevel: 'debug' | 'info' | 'warn' | 'error';
  maxLogLines: number;

  // 其他
  checkUpdates: boolean;
  analyticsEnabled: boolean;
}

function mergeAppConfig(current: AppConfig, updates: unknown): AppConfig {
  if (!updates || typeof updates !== 'object') {
    return current;
  }

  const source = updates as Record<string, unknown>;
  const readString = (key: keyof AppConfig, fallback: string) =>
    typeof source[key] === 'string' ? source[key] : fallback;
  const readNumber = (key: keyof AppConfig, fallback: number) =>
    typeof source[key] === 'number' ? source[key] : fallback;
  const readBoolean = (key: keyof AppConfig, fallback: boolean) =>
    typeof source[key] === 'boolean' ? source[key] : fallback;
  const theme = source.theme;
  const language = source.language;
  const logLevel = source.logLevel;

  return {
    accessKeyId: readString('accessKeyId', current.accessKeyId),
    accessKeySecret: readString('accessKeySecret', current.accessKeySecret),
    appId: readNumber('appId', current.appId),
    autoReconnect: readBoolean('autoReconnect', current.autoReconnect),
    reconnectInterval: readNumber('reconnectInterval', current.reconnectInterval),
    maxReconnectAttempts: readNumber('maxReconnectAttempts', current.maxReconnectAttempts),
    heartbeatInterval: readNumber('heartbeatInterval', current.heartbeatInterval),
    enableAutoRecord: readBoolean('enableAutoRecord', current.enableAutoRecord),
    enableSensitiveWordFilter: readBoolean(
      'enableSensitiveWordFilter',
      current.enableSensitiveWordFilter
    ),
    sensitiveWords: Array.isArray(source.sensitiveWords)
      ? source.sensitiveWords.filter((word): word is string => typeof word === 'string')
      : current.sensitiveWords,
    replaceChar: readString('replaceChar', current.replaceChar),
    theme: theme === 'light' || theme === 'dark' || theme === 'auto' ? theme : current.theme,
    language: language === 'zh-CN' || language === 'en-US' ? language : current.language,
    fontSize: readNumber('fontSize', current.fontSize),
    enableNotifications: readBoolean('enableNotifications', current.enableNotifications),
    enableLog: readBoolean('enableLog', current.enableLog),
    logLevel:
      logLevel === 'debug' || logLevel === 'info' || logLevel === 'warn' || logLevel === 'error'
        ? logLevel
        : current.logLevel,
    maxLogLines: readNumber('maxLogLines', current.maxLogLines),
    checkUpdates: readBoolean('checkUpdates', current.checkUpdates),
    analyticsEnabled: readBoolean('analyticsEnabled', current.analyticsEnabled)
  };
}

export const useConfigStore = defineStore('config', () => {
  // State
  const config = ref<AppConfig>({
    // 开放平台配置
    accessKeyId: '92yCCLAhXtGfRzWKMgSXruqN',
    accessKeySecret: '9lIaEErbeC62RJ1ApCNrx47ao3v0wd',
    appId: 1716751199246,

    // 直播间配置
    autoReconnect: true,
    reconnectInterval: 3000,
    maxReconnectAttempts: 10,
    heartbeatInterval: 30,

    // 自动录制配置
    enableAutoRecord: false,

    // 敏感词过滤
    enableSensitiveWordFilter: true,
    sensitiveWords: [],
    replaceChar: '*',

    // UI 配置
    theme: 'auto',
    language: 'zh-CN',
    fontSize: 14,
    enableNotifications: true,

    // 日志配置
    enableLog: true,
    logLevel: 'info',
    maxLogLines: 1000,

    // 其他
    checkUpdates: true,
    analyticsEnabled: false
  });

  // Actions
  function updateConfig(updates: Partial<AppConfig> | Record<string, unknown>) {
    config.value = mergeAppConfig(config.value, updates);
    saveConfig();
  }

  function resetConfig() {
    config.value = {
      accessKeyId: '',
      accessKeySecret: '',
      appId: 0,
      autoReconnect: true,
      reconnectInterval: 3000,
      maxReconnectAttempts: 10,
      heartbeatInterval: 30,
      enableAutoRecord: false,
      enableSensitiveWordFilter: true,
      sensitiveWords: [],
      replaceChar: '*',
      theme: 'auto',
      language: 'zh-CN',
      fontSize: 14,
      enableNotifications: true,
      enableLog: true,
      logLevel: 'info',
      maxLogLines: 1000,
      checkUpdates: true,
      analyticsEnabled: false
    };
    saveConfig();
  }

  function saveConfig() {
    try {
      localStorage.setItem('bilibili-live-config', JSON.stringify(config.value));
    } catch (error) {
      console.error('Failed to save config:', error);
    }
  }

  function loadConfig() {
    try {
      const saved = localStorage.getItem('bilibili-live-config');
      if (saved) {
        config.value = mergeAppConfig(config.value, JSON.parse(saved));
      }
    } catch (error) {
      console.error('Failed to load config:', error);
    }
  }

  function addSensitiveWord(word: string) {
    if (!config.value.sensitiveWords.includes(word)) {
      config.value.sensitiveWords.push(word);
      saveConfig();
    }
  }

  function removeSensitiveWord(word: string) {
    config.value.sensitiveWords = config.value.sensitiveWords.filter(w => w !== word);
    saveConfig();
  }

  function clearSensitiveWords() {
    config.value.sensitiveWords = [];
    saveConfig();
  }

  function exportConfig(): AppConfig {
    return { ...config.value };
  }

  function importConfig(newConfig: AppConfig) {
    config.value = mergeAppConfig(config.value, newConfig);
    saveConfig();
  }

  // 初始化时加载配置
  loadConfig();

  return {
    // State
    config,

    // Actions
    updateConfig,
    resetConfig,
    saveConfig,
    loadConfig,
    addSensitiveWord,
    removeSensitiveWord,
    clearSensitiveWords,
    exportConfig,
    importConfig
  };
});
