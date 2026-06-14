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

  // 弹幕发送配置
  minSendInterval: number;
  enableAutoSend: boolean;
  autoSendInterval: number;
  enableTriggerReply: boolean;
  enableGiftThank: boolean;

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

    // 弹幕发送配置
    minSendInterval: 3000,
    enableAutoSend: false,
    autoSendInterval: 5000,
    enableTriggerReply: true,
    enableGiftThank: true,

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
  function updateConfig(updates: Partial<AppConfig>) {
    config.value = {
      ...config.value,
      ...updates
    };
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
      minSendInterval: 3000,
      enableAutoSend: false,
      autoSendInterval: 5000,
      enableTriggerReply: true,
      enableGiftThank: true,
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
        const parsed = JSON.parse(saved);
        config.value = {
          ...config.value,
          ...parsed
        };
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
    config.value = { ...newConfig };
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
