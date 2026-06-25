/**
 * 插件系统入口文件
 * 导出所有类型、接口和核心模块
 */

// 导出类型定义
export * from './types';

// 导出核心模块（后续任务实现）
// export { PluginManager } from './plugin-manager';
// export { PluginLoader } from './plugin-loader';
// export { PluginAPIImpl } from './plugin-api';
// export { PluginSandbox } from './plugin-sandbox';

// 插件系统版本
export const PLUGIN_SYSTEM_VERSION = '1.0.0';

// 插件目录名称
export const PLUGIN_DIR_NAME = 'plugins';

// 插件清单文件名
export const PLUGIN_MANIFEST_FILE = 'manifest.json';

// 插件入口文件名
export const PLUGIN_ENTRY_FILE = 'index.js';

// 插件配置前缀
export const PLUGIN_CONFIG_PREFIX = 'plugin';

// 插件权限列表
export const PLUGIN_PERMISSIONS: Record<string, string> = {
  danmaku: '弹幕读写',
  gift: '礼物监听',
  captain: '舰长监听',
  config: '配置读写',
  effect: '特效注册',
  ui: 'UI 扩展'
};

// 插件状态枚举
export enum PluginStatus {
  INSTALLED = 'installed',
  ENABLED = 'enabled',
  DISABLED = 'disabled',
  ERROR = 'error',
}

// 插件事件枚举
export enum PluginEvent {
  LOADED = 'loaded',
  ENABLED = 'enabled',
  DISABLED = 'disabled',
  UNLOADED = 'unloaded',
  ERROR = 'error',
  CONFIG_CHANGED = 'config_changed',
}

// 插件错误类型
export class PluginError extends Error {
  constructor(
    message: string,
    public pluginId: string,
    public code: string
  ) {
    super(message);
    this.name = 'PluginError';
  }
}

// 插件加载错误
export class PluginLoadError extends PluginError {
  constructor(message: string, pluginId: string) {
    super(message, pluginId, 'PLUGIN_LOAD_ERROR');
    this.name = 'PluginLoadError';
  }
}

// 插件验证错误
export class PluginValidationError extends PluginError {
  constructor(message: string, pluginId: string) {
    super(message, pluginId, 'PLUGIN_VALIDATION_ERROR');
    this.name = 'PluginValidationError';
  }
}

// 插件权限错误
export class PluginPermissionError extends PluginError {
  constructor(message: string, pluginId: string) {
    super(message, pluginId, 'PLUGIN_PERMISSION_ERROR');
    this.name = 'PluginPermissionError';
  }
}
