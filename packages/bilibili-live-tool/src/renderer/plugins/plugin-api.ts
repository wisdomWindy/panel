/**
 * 插件 API 暂露
 * 为插件提供访问核心功能的接口
 */

import {
  PluginAPI,
  DanmakuEvent,
  GiftEvent,
  CaptainEvent,
  EffectDefinition,
  MenuItemDefinition,
  SettingPanelDefinition,
  PluginPermission
} from './types';

// API 事件监听器类型
type EventListener<T> = (event: T) => void;
type ConfigChangeListener = (key: string, value: any) => void;

/**
 * 插件 API 实现类
 */
export class PluginAPIImpl implements PluginAPI {
  private pluginId: string;
  private permissions: Set<PluginPermission>;
  private eventListeners: Map<string, Set<EventListener<any>>> = new Map();
  private configListeners: Set<ConfigChangeListener> = new Set();
  private cleanupFunctions: Set<() => void> = new Set();

  // 外部依赖（需要注入）
  private configStore: any;
  private effectManager: any;
  private uiRegistry: any;

  constructor(
    pluginId: string,
    permissions: PluginPermission[],
    dependencies: {
      configStore?: any;
      effectManager?: any;
      uiRegistry?: any;
    }
  ) {
    this.pluginId = pluginId;
    this.permissions = new Set(permissions);
    this.configStore = dependencies.configStore;
    this.effectManager = dependencies.effectManager;
    this.uiRegistry = dependencies.uiRegistry;
  }

  /**
   * 检查权限
   */
  private checkPermission(permission: PluginPermission): void {
    if (!this.permissions.has(permission)) {
      throw new Error(`插件 ${this.pluginId} 未声明 ${permission} 权限`);
    }
  }

  /**
   * 监听弹幕事件
   */
  onDanmaku(callback: (danmaku: DanmakuEvent) => void): () => void {
    this.checkPermission('danmaku');

    const listener = (danmaku: DanmakuEvent) => {
      try {
        callback(danmaku);
      } catch (error) {
        this.log('error', `弹幕事件处理异常: ${error}`);
      }
    };

    // 注册监听器
    if (!this.eventListeners.has('danmaku')) {
      this.eventListeners.set('danmaku', new Set());
    }
    this.eventListeners.get('danmaku')!.add(listener);

    // 返回取消监听函数
    const cleanup = () => {
      this.eventListeners.get('danmaku')?.delete(listener);
    };
    this.cleanupFunctions.add(cleanup);

    return cleanup;
  }

  /**
   * 监听礼物事件
   */
  onGift(callback: (gift: GiftEvent) => void): () => void {
    this.checkPermission('gift');

    const listener = (gift: GiftEvent) => {
      try {
        callback(gift);
      } catch (error) {
        this.log('error', `礼物事件处理异常: ${error}`);
      }
    };

    // 注册监听器
    if (!this.eventListeners.has('gift')) {
      this.eventListeners.set('gift', new Set());
    }
    this.eventListeners.get('gift')!.add(listener);

    // 返回取消监听函数
    const cleanup = () => {
      this.eventListeners.get('gift')?.delete(listener);
    };
    this.cleanupFunctions.add(cleanup);

    return cleanup;
  }

  /**
   * 监听舰长事件
   */
  onCaptain(callback: (captain: CaptainEvent) => void): () => void {
    this.checkPermission('captain');

    const listener = (captain: CaptainEvent) => {
      try {
        callback(captain);
      } catch (error) {
        this.log('error', `舰长事件处理异常: ${error}`);
      }
    };

    // 注册监听器
    if (!this.eventListeners.has('captain')) {
      this.eventListeners.set('captain', new Set());
    }
    this.eventListeners.get('captain')!.add(listener);

    // 返回取消监听函数
    const cleanup = () => {
      this.eventListeners.get('captain')?.delete(listener);
    };
    this.cleanupFunctions.add(cleanup);

    return cleanup;
  }

  /**
   * 获取配置
   */
  getConfig<T>(key: string): T {
    this.checkPermission('config');

    if (!this.configStore) {
      throw new Error('配置服务不可用');
    }

    const fullKey = `plugin.${this.pluginId}.${key}`;
    return this.configStore.getConfig(fullKey);
  }

  /**
   * 设置配置
   */
  setConfig<T>(key: string, value: T): void {
    this.checkPermission('config');

    if (!this.configStore) {
      throw new Error('配置服务不可用');
    }

    const fullKey = `plugin.${this.pluginId}.${key}`;
    this.configStore.setConfig(fullKey, value);

    // 通知配置变更监听器
    this.configListeners.forEach((listener) => {
      try {
        listener(key, value);
      } catch (error) {
        this.log('error', `配置变更监听异常: ${error}`);
      }
    });
  }

  /**
   * 监听配置变更
   */
  onConfigChange(callback: (key: string, value: any) => void): () => void {
    this.checkPermission('config');

    this.configListeners.add(callback);

    // 返回取消监听函数
    const cleanup = () => {
      this.configListeners.delete(callback);
    };
    this.cleanupFunctions.add(cleanup);

    return cleanup;
  }

  /**
   * 注册自定义特效
   */
  registerEffect(effect: EffectDefinition): () => void {
    this.checkPermission('effect');

    if (!this.effectManager) {
      throw new Error('特效管理器不可用');
    }

    try {
      this.effectManager.registerEffect(effect);

      // 返回取消注册函数
      const cleanup = () => {
        this.effectManager.unregisterEffect(effect.id);
      };
      this.cleanupFunctions.add(cleanup);

      return cleanup;
    } catch (error) {
      this.log('error', `注册特效失败: ${error}`);
      throw error;
    }
  }

  /**
   * 注册菜单项
   */
  registerMenuItem(menu: MenuItemDefinition): () => void {
    this.checkPermission('ui');

    if (!this.uiRegistry) {
      throw new Error('UI 注册表不可用');
    }

    try {
      this.uiRegistry.registerMenuItem(this.pluginId, menu);

      // 返回取消注册函数
      const cleanup = () => {
        this.uiRegistry.unregisterMenuItem(this.pluginId, menu.id);
      };
      this.cleanupFunctions.add(cleanup);

      return cleanup;
    } catch (error) {
      this.log('error', `注册菜单项失败: ${error}`);
      throw error;
    }
  }

  /**
   * 注册设置面板
   */
  registerSettingPanel(panel: SettingPanelDefinition): () => void {
    this.checkPermission('ui');

    if (!this.uiRegistry) {
      throw new Error('UI 注册表不可用');
    }

    try {
      this.uiRegistry.registerSettingPanel(this.pluginId, panel);

      // 返回取消注册函数
      const cleanup = () => {
        this.uiRegistry.unregisterSettingPanel(this.pluginId, panel.id);
      };
      this.cleanupFunctions.add(cleanup);

      return cleanup;
    } catch (error) {
      this.log('error', `注册设置面板失败: ${error}`);
      throw error;
    }
  }

  /**
   * 记录日志
   */
  log(level: 'info' | 'warn' | 'error', message: string): void {
    const logMessage = `[插件:${this.pluginId}] ${message}`;

    switch (level) {
      case 'info':
        console.log(logMessage);
        break;
      case 'warn':
        console.warn(logMessage);
        break;
      case 'error':
        console.error(logMessage);
        break;
    }
  }

  /**
   * 触发事件（供插件管理器调用）
   */
  emitEvent<T>(eventType: string, event: T): void {
    const listeners = this.eventListeners.get(eventType);
    if (listeners) {
      listeners.forEach((listener) => {
        try {
          listener(event);
        } catch (error) {
          this.log('error', `事件处理异常: ${error}`);
        }
      });
    }
  }

  /**
   * 清理所有资源
   */
  cleanup(): void {
    // 清理所有事件监听器
    this.eventListeners.clear();
    this.configListeners.clear();

    // 执行所有清理函数
    this.cleanupFunctions.forEach((cleanup) => {
      try {
        cleanup();
      } catch (error) {
        console.error(`清理函数执行失败: ${error}`);
      }
    });
    this.cleanupFunctions.clear();
  }

  /**
   * 获取插件 ID
   */
  getPluginId(): string {
    return this.pluginId;
  }

  /**
   * 获取权限列表
   */
  getPermissions(): PluginPermission[] {
    return Array.from(this.permissions);
  }

  /**
   * 检查是否有指定权限
   */
  hasPermission(permission: PluginPermission): boolean {
    return this.permissions.has(permission);
  }
}

/**
 * 创建插件 API 实例
 */
export function createPluginAPI(
  pluginId: string,
  permissions: PluginPermission[],
  dependencies: {
    configStore?: any;
    effectManager?: any;
    uiRegistry?: any;
  }
): PluginAPIImpl {
  return new PluginAPIImpl(pluginId, permissions, dependencies);
}
