/**
 * 插件系统类型定义
 * 定义插件接口、API 接口、清单接口等
 */

// 插件权限类型
export type PluginPermission =
  | 'danmaku'    // 弹幕读写
  | 'gift'       // 礼物监听
  | 'captain'    // 舰长监听
  | 'config'     // 配置读写
  | 'effect'     // 特效注册
  | 'ui';        // UI 扩展

// 配置字段类型
export interface ConfigField {
  type: 'string' | 'number' | 'boolean' | 'select' | 'color';
  default: any;
  description: string;
  options?: { label: string; value: any }[];
  min?: number;
  max?: number;
}

// 插件清单接口
export interface PluginManifest {
  id: string;
  name: string;
  version: string;
  description: string;
  author: string;
  main: string;
  minAppVersion: string;
  permissions: PluginPermission[];
  configSchema?: Record<string, ConfigField>;
}

// 弹幕事件接口
export interface DanmakuEvent {
  id: string;
  userId: string;
  username: string;
  content: string;
  timestamp: number;
  userType: 'normal' | 'captain' | 'admin' | 'owner';
  badge?: {
    level: number;
    name: string;
  };
  medal?: {
    level: number;
    name: string;
    color: string;
  };
}

// 礼物事件接口
export interface GiftEvent {
  id: string;
  giftId: string;
  giftName: string;
  userId: string;
  username: string;
  count: number;
  price: number;
  timestamp: number;
  isCombo: boolean;
  comboId?: string;
}

// 舰长事件接口
export interface CaptainEvent {
  userId: string;
  username: string;
  level: number;
  giftName: string;
  giftId: string;
  count: number;
  timestamp: number;
  isFirstTime: boolean;
  isRenewal: boolean;
}

// 弹幕发送选项
export interface DanmakuOptions {
  color?: string;
  fontSize?: number;
}

// 特效定义接口
export interface EffectDefinition {
  id: string;
  name: string;
  type: string;
  template: {
    text: string;
    animation: string;
    colors: {
      primary: string;
      secondary: string;
    };
    duration: number;
    fontSize: number;
  };
  sound?: {
    enabled: boolean;
    file: string;
    volume: number;
  };
}

// 菜单项定义接口
export interface MenuItemDefinition {
  id: string;
  label: string;
  icon?: string;
  path: string;
  order?: number;
}

// 设置面板定义接口
export interface SettingPanelDefinition {
  id: string;
  label: string;
  icon?: string;
  component: any;
  order?: number;
}

// 插件 API 接口
export interface PluginAPI {
  // 弹幕相关
  sendDanmaku(content: string, options?: DanmakuOptions): Promise<void>;
  onDanmaku(callback: (danmaku: DanmakuEvent) => void): () => void;

  // 礼物相关
  onGift(callback: (gift: GiftEvent) => void): () => void;

  // 舰长相关
  onCaptain(callback: (captain: CaptainEvent) => void): () => void;

  // 配置相关
  getConfig<T>(key: string): T;
  setConfig<T>(key: string, value: T): void;
  onConfigChange(callback: (key: string, value: any) => void): () => void;

  // 特效相关
  registerEffect(effect: EffectDefinition): () => void;

  // UI 相关
  registerMenuItem(menu: MenuItemDefinition): () => void;
  registerSettingPanel(panel: SettingPanelDefinition): () => void;

  // 日志
  log(level: 'info' | 'warn' | 'error', message: string): void;
}

// 插件接口
export interface Plugin {
  // 插件元信息
  id: string;
  name: string;
  version: string;
  description: string;
  author: string;
  main?: string;
  minAppVersion?: string;
  permissions?: PluginPermission[];

  // 生命周期钩子
  onLoad?: () => Promise<void> | void;
  onEnable?: () => Promise<void> | void;
  onDisable?: () => Promise<void> | void;
  onUnload?: () => Promise<void> | void;

  // 事件监听
  onDanmaku?: (danmaku: DanmakuEvent) => void;
  onGift?: (gift: GiftEvent) => void;
  onCaptain?: (captain: CaptainEvent) => void;

  // 配置
  config?: Record<string, any>;
  configSchema?: Record<string, ConfigField>;
}

// 插件状态接口
export interface PluginState {
  id: string;
  name: string;
  version: string;
  description: string;
  author: string;
  enabled: boolean;
  installedAt: string;
  updatedAt: string;
  config: Record<string, any>;
  permissions: PluginPermission[];
}

// 插件安装选项
export interface PluginInstallOptions {
  force?: boolean;
  skipVersionCheck?: boolean;
}

// 插件加载结果
export interface PluginLoadResult {
  success: boolean;
  plugin?: Plugin;
  error?: string;
  warnings?: string[];
}

// 插件验证结果
export interface PluginValidationResult {
  valid: boolean;
  errors: string[];
  warnings: string[];
}

// 插件常量
export const PLUGIN_MANIFEST_FILE = 'manifest.json';
export const PLUGIN_ENTRY_FILE = 'index.js';

// 插件事件类型
export type PluginEvent =
  | 'registered'
  | 'unregistered'
  | 'enabled'
  | 'disabled'
  | 'installed'
  | 'uninstalled'
  | 'config_changed'
  | 'initialized';

// 插件错误基类
export class PluginError extends Error {
  constructor(message: string) {
    super(message);
    this.name = 'PluginError';
  }
}

// 插件加载错误
export class PluginLoadError extends PluginError {
  pluginId: string;

  constructor(message: string, pluginId: string) {
    super(message);
    this.name = 'PluginLoadError';
    this.pluginId = pluginId;
  }
}

// 插件验证错误
export class PluginValidationError extends PluginError {
  errors: string[];

  constructor(message: string, errors: string[]) {
    super(message);
    this.name = 'PluginValidationError';
    this.errors = errors;
  }
}

// 插件权限错误
export class PluginPermissionError extends PluginError {
  pluginId: string;

  constructor(message: string, pluginId: string) {
    super(message);
    this.name = 'PluginPermissionError';
    this.pluginId = pluginId;
  }
}
