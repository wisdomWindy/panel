/**
 * 插件管理器
 * 负责插件的安装、启用、禁用、卸载和生命周期管理
 */

import {
  Plugin,
  PluginManifest,
  PluginState,
  PluginLoadResult,
  PluginInstallOptions,
  PluginPermission,
  PluginLoadError
} from './types';
import { PluginLoader, createPluginLoader } from './plugin-loader';
import { PluginSandbox, createPluginSandbox } from './plugin-sandbox';
import { PluginAPIImpl, createPluginAPI } from './plugin-api';
import { resolvePluginInstallTarget, toPluginDirectoryPath } from './runtime/helpers.js';

// 插件管理器配置
interface PluginManagerConfig {
  pluginDir: string;
  appVersion: string;
  dependencies: {
    danmakuService?: any;
    configStore?: any;
    effectManager?: any;
    uiRegistry?: any;
  };
}

// 插件实例信息
interface PluginInstance {
  manifest: PluginManifest;
  sandbox: PluginSandbox;
  api: PluginAPIImpl;
  plugin: Plugin;
  state: PluginState;
}

/**
 * 插件管理器类
 */
export class PluginManager {
  private pluginDir: string;
  private appVersion: string;
  private dependencies: PluginManagerConfig['dependencies'];
  private loader: PluginLoader;
  private plugins: Map<string, PluginInstance> = new Map();
  private eventListeners: Map<string, Set<(pluginId: string, event: any) => void>> = new Map();

  constructor(config: PluginManagerConfig) {
    this.pluginDir = config.pluginDir;
    this.appVersion = config.appVersion;
    this.dependencies = config.dependencies;
    this.loader = createPluginLoader({
      pluginDir: config.pluginDir,
      appVersion: config.appVersion
    });
  }

  /**
   * 初始化插件管理器
   */
  async initialize(): Promise<void> {
    try {
      // 加载所有插件
      const results = await this.loader.loadAllPlugins();

      // 处理加载结果
      for (const result of results) {
        if (result.success && result.plugin) {
          // 插件加载成功，但不自动启用
          await this.registerPlugin(result.plugin.id, result);
        } else {
          console.error(`插件加载失败: ${result.error}`);
        }
      }

      this.emitEvent('initialized', null);
    } catch (error) {
      console.error('插件管理器初始化失败:', error);
      throw error;
    }
  }

  /**
   * 注册插件
   */
  private async registerPlugin(pluginId: string, loadResult: PluginLoadResult): Promise<void> {
    try {
      // 1. 加载插件清单
      const manifestResult = await this.loader.loadPlugin(pluginId);
      if (!manifestResult.success) {
        throw new PluginLoadError(manifestResult.error || '加载插件失败', pluginId);
      }

      // 2. 创建插件 API
      const api = createPluginAPI(
        pluginId,
        loadResult.plugin?.permissions || [],
        this.dependencies
      );

      // 3. 创建沙箱
      const sandbox = createPluginSandbox({
        pluginId,
        permissions: loadResult.plugin?.permissions || [],
        api
      });

      // 4. 加载插件模块
      const pluginPath = `${this.pluginDir}/${pluginId}`;
      const entryPath = `${pluginPath}/${manifestResult.plugin?.main || 'index.js'}`;

      // 读取插件代码
      let pluginCode = '';
      if (typeof window !== 'undefined' && window.electronAPI) {
        pluginCode = await window.electronAPI.readFile(entryPath);
      }

      // 在沙箱中执行插件代码
      const moduleResult = await sandbox.loadPluginModule(pluginCode);
      if (!moduleResult.success) {
        throw new PluginLoadError(moduleResult.error || '执行插件代码失败', pluginId);
      }

      // 5. 创建插件实例
      const plugin: Plugin = {
        id: pluginId,
        name: loadResult.plugin?.name || pluginId,
        version: loadResult.plugin?.version || '1.0.0',
        description: loadResult.plugin?.description || '',
        author: loadResult.plugin?.author || '',
        ...moduleResult.result
      };

      // 6. 创建插件状态
      const state: PluginState = {
        id: pluginId,
        name: plugin.name,
        version: plugin.version,
        description: plugin.description,
        author: plugin.author,
        enabled: false,
        installedAt: new Date().toISOString(),
        updatedAt: new Date().toISOString(),
        config: {},
        permissions: loadResult.plugin?.permissions || []
      };

      // 7. 保存插件实例
      this.plugins.set(pluginId, {
        manifest: manifestResult.plugin! as PluginManifest,
        sandbox,
        api,
        plugin,
        state
      });

      this.emitEvent('registered', pluginId);
    } catch (error) {
      console.error(`注册插件 ${pluginId} 失败:`, error);
      throw error;
    }
  }

  /**
   * 安装插件
   */
  async installPlugin(pluginPath: string, options?: PluginInstallOptions): Promise<void> {
    try {
      // 1. 验证插件路径
      if (!pluginPath) {
        throw new Error('插件路径不能为空');
      }

      // 2. 加载插件
      const loadResult = await this.loader.loadPluginFromPath(pluginPath);
      if (!loadResult.success) {
        throw new PluginLoadError(loadResult.error || '加载插件失败', pluginPath);
      }

      // 3. 检查版本兼容性
      if (!options?.skipVersionCheck) {
        const versionResult = this.loader.checkVersionCompatibility(loadResult.plugin! as PluginManifest);
        if (!versionResult.compatible) {
          if (!options?.force) {
            throw new Error(versionResult.error);
          }
          console.warn(`强制安装插件: ${versionResult.error}`);
        }
      }

      // 4. 复制插件到插件目录
      const installTarget = resolvePluginInstallTarget(pluginPath, {
        id: loadResult.plugin!.id
      });
      await this.copyPluginToDirectory(installTarget.sourcePath, installTarget.pluginId);

      // 5. 注册插件
      await this.registerPlugin(loadResult.plugin!.id, loadResult);

      this.emitEvent('installed', loadResult.plugin!.id);
    } catch (error) {
      console.error('安装插件失败:', error);
      throw error;
    }
  }

  /**
   * 启用插件
   */
  async enablePlugin(pluginId: string): Promise<void> {
    const instance = this.plugins.get(pluginId);
    if (!instance) {
      throw new Error(`插件 ${pluginId} 未安装`);
    }

    if (instance.state.enabled) {
      return; // 已经启用
    }

    try {
      // 调用 onEnable 钩子
      if (instance.plugin.onEnable) {
        await instance.plugin.onEnable();
      }

      // 更新状态
      instance.state.enabled = true;
      instance.state.updatedAt = new Date().toISOString();

      this.emitEvent('enabled', pluginId);
    } catch (error) {
      console.error(`启用插件 ${pluginId} 失败:`, error);
      throw error;
    }
  }

  /**
   * 禁用插件
   */
  async disablePlugin(pluginId: string): Promise<void> {
    const instance = this.plugins.get(pluginId);
    if (!instance) {
      throw new Error(`插件 ${pluginId} 未安装`);
    }

    if (!instance.state.enabled) {
      return; // 已经禁用
    }

    try {
      // 调用 onDisable 钩子
      if (instance.plugin.onDisable) {
        await instance.plugin.onDisable();
      }

      // 更新状态
      instance.state.enabled = false;
      instance.state.updatedAt = new Date().toISOString();

      this.emitEvent('disabled', pluginId);
    } catch (error) {
      console.error(`禁用插件 ${pluginId} 失败:`, error);
      throw error;
    }
  }

  /**
   * 卸载插件
   */
  async uninstallPlugin(pluginId: string): Promise<void> {
    const instance = this.plugins.get(pluginId);
    if (!instance) {
      throw new Error(`插件 ${pluginId} 未安装`);
    }

    try {
      // 1. 如果启用，先禁用
      if (instance.state.enabled) {
        await this.disablePlugin(pluginId);
      }

      // 2. 调用 onUnload 钩子
      if (instance.plugin.onUnload) {
        await instance.plugin.onUnload();
      }

      // 3. 清理资源
      instance.api.cleanup();

      // 4. 删除插件目录
      await this.deletePluginDirectory(pluginId);

      // 5. 移除插件实例
      this.plugins.delete(pluginId);

      this.emitEvent('uninstalled', pluginId);
    } catch (error) {
      console.error(`卸载插件 ${pluginId} 失败:`, error);
      throw error;
    }
  }

  /**
   * 获取所有插件状态
   */
  getPlugins(): PluginState[] {
    return Array.from(this.plugins.values()).map((instance) => instance.state);
  }

  /**
   * 获取指定插件状态
   */
  getPlugin(pluginId: string): PluginState | undefined {
    return this.plugins.get(pluginId)?.state;
  }

  /**
   * 检查插件是否已安装
   */
  isPluginInstalled(pluginId: string): boolean {
    return this.plugins.has(pluginId);
  }

  /**
   * 检查插件是否已启用
   */
  isPluginEnabled(pluginId: string): boolean {
    return this.plugins.get(pluginId)?.state.enabled ?? false;
  }

  /**
   * 获取插件配置
   */
  getPluginConfig(pluginId: string): Record<string, any> {
    const instance = this.plugins.get(pluginId);
    if (!instance) {
      throw new Error(`插件 ${pluginId} 未安装`);
    }
    return instance.state.config;
  }

  /**
   * 更新插件配置
   */
  updatePluginConfig(pluginId: string, config: Record<string, any>): void {
    const instance = this.plugins.get(pluginId);
    if (!instance) {
      throw new Error(`插件 ${pluginId} 未安装`);
    }

    instance.state.config = { ...instance.state.config, ...config };
    instance.state.updatedAt = new Date().toISOString();

    this.emitEvent('config_changed', pluginId);
  }

  /**
   * 获取插件权限
   */
  getPluginPermissions(pluginId: string): PluginPermission[] {
    const instance = this.plugins.get(pluginId);
    if (!instance) {
      throw new Error(`插件 ${pluginId} 未安装`);
    }
    return instance.state.permissions;
  }

  /**
   * 注册事件监听器
   */
  on(event: string, listener: (pluginId: string, event: any) => void): () => void {
    if (!this.eventListeners.has(event)) {
      this.eventListeners.set(event, new Set());
    }
    this.eventListeners.get(event)!.add(listener);

    return () => {
      this.eventListeners.get(event)?.delete(listener);
    };
  }

  /**
   * 触发事件
   */
  private emitEvent(event: string, pluginId: string | null): void {
    const listeners = this.eventListeners.get(event);
    if (listeners) {
      listeners.forEach((listener) => {
        try {
          listener(pluginId!, event);
        } catch (error) {
          console.error(`事件监听器执行失败: ${error}`);
        }
      });
    }
  }

  /**
   * 复制插件到插件目录
   */
  private async copyPluginToDirectory(sourcePath: string, pluginId: string): Promise<void> {
    // 在 Electron 环境中使用 fs 模块
    if (typeof window !== 'undefined' && window.electronAPI) {
      const targetPath = toPluginDirectoryPath(this.pluginDir, pluginId);
      await window.electronAPI.copyDirectory(sourcePath, targetPath);
    }
  }

  /**
   * 删除插件目录
   */
  private async deletePluginDirectory(pluginId: string): Promise<void> {
    // 在 Electron 环境中使用 fs 模块
    if (typeof window !== 'undefined' && window.electronAPI) {
      const pluginPath = toPluginDirectoryPath(this.pluginDir, pluginId);
      await window.electronAPI.deleteDirectory(pluginPath);
    }
  }

  /**
   * 获取插件目录
   */
  getPluginDir(): string {
    return this.pluginDir;
  }

  /**
   * 获取应用版本
   */
  getAppVersion(): string {
    return this.appVersion;
  }

  /**
   * 获取插件数量
   */
  getPluginCount(): number {
    return this.plugins.size;
  }

  /**
   * 获取已启用插件数量
   */
  getEnabledPluginCount(): number {
    return Array.from(this.plugins.values()).filter((instance) => instance.state.enabled).length;
  }
}

/**
 * 创建插件管理器实例
 */
export function createPluginManager(config: PluginManagerConfig): PluginManager {
  return new PluginManager(config);
}
