/**
 * 插件沙箱
 * 提供插件隔离执行环境，防止插件异常影响主应用
 */

import { PluginPermission, PluginAPI, PluginPermissionError } from './types';

// 沙箱配置接口
interface SandboxConfig {
  pluginId: string;
  permissions: PluginPermission[];
  api: PluginAPI;
}

// 沙箱执行结果
interface SandboxResult<T = any> {
  success: boolean;
  result?: T;
  error?: string;
}

/**
 * 插件沙箱类
 * 使用 Function 构造函数创建受限的执行环境
 */
export class PluginSandbox {
  private pluginId: string;
  private permissions: Set<PluginPermission>;
  private api: PluginAPI;
  private sandboxEnv: Record<string, any>;

  constructor(config: SandboxConfig) {
    this.pluginId = config.pluginId;
    this.permissions = new Set(config.permissions);
    this.api = config.api;
    this.sandboxEnv = this.createSandboxEnvironment();
  }

  /**
   * 创建沙箱环境
   * 只暴露授权的 API 和有限的全局对象
   */
  private createSandboxEnvironment(): Record<string, any> {
    const env: Record<string, any> = {
      // 基础 JavaScript 对象
      console: this.createConsole(),
      setTimeout: setTimeout.bind(globalThis),
      clearTimeout: clearTimeout.bind(globalThis),
      setInterval: setInterval.bind(globalThis),
      clearInterval: clearInterval.bind(globalThis),
      Promise: Promise,
      Array: Array,
      Object: Object,
      String: String,
      Number: Number,
      Boolean: Boolean,
      Date: Date,
      Math: Math,
      JSON: JSON,
      RegExp: RegExp,
      Error: Error,
      TypeError: TypeError,
      RangeError: RangeError,
      SyntaxError: SyntaxError,
      parseInt: parseInt,
      parseFloat: parseFloat,
      isNaN: isNaN,
      isFinite: isFinite,
      encodeURIComponent: encodeURIComponent,
      decodeURIComponent: decodeURIComponent,
      encodeURI: encodeURI,
      decodeURI: decodeURI,
    };

    // 根据权限添加 API
    if (this.permissions.has('danmaku')) {
      env.sendDanmaku = this.api.sendDanmaku.bind(this.api);
      env.onDanmaku = this.api.onDanmaku.bind(this.api);
    }

    if (this.permissions.has('gift')) {
      env.onGift = this.api.onGift.bind(this.api);
    }

    if (this.permissions.has('captain')) {
      env.onCaptain = this.api.onCaptain.bind(this.api);
    }

    if (this.permissions.has('config')) {
      env.getConfig = this.api.getConfig.bind(this.api);
      env.setConfig = this.api.setConfig.bind(this.api);
      env.onConfigChange = this.api.onConfigChange.bind(this.api);
    }

    if (this.permissions.has('effect')) {
      env.registerEffect = this.api.registerEffect.bind(this.api);
    }

    if (this.permissions.has('ui')) {
      env.registerMenuItem = this.api.registerMenuItem.bind(this.api);
      env.registerSettingPanel = this.api.registerSettingPanel.bind(this.api);
    }

    // 日志 API 始终可用
    env.log = this.api.log.bind(this.api);

    return env;
  }

  /**
   * 创建受限的 console 对象
   */
  private createConsole(): Console {
    return {
      log: (...args: any[]) => {
        this.api.log('info', args.map(String).join(' '));
      },
      warn: (...args: any[]) => {
        this.api.log('warn', args.map(String).join(' '));
      },
      error: (...args: any[]) => {
        this.api.log('error', args.map(String).join(' '));
      },
      info: (...args: any[]) => {
        this.api.log('info', args.map(String).join(' '));
      },
      debug: (...args: any[]) => {
        this.api.log('info', args.map(String).join(' '));
      },
      trace: (...args: any[]) => {
        this.api.log('info', args.map(String).join(' '));
      },
      dir: (...args: any[]) => {
        this.api.log('info', args.map(String).join(' '));
      },
      time: () => {},
      timeEnd: () => {},
      timeLog: () => {},
      clear: () => {},
      count: () => {},
      countReset: () => {},
      group: () => {},
      groupCollapsed: () => {},
      groupEnd: () => {},
      table: (...args: any[]) => {
        this.api.log('info', args.map(String).join(' '));
      },
      assert: () => {},
      profile: () => {},
      profileEnd: () => {},
      timeStamp: () => {},
      dirxml: (...args: any[]) => {
        this.api.log('info', args.map(String).join(' '));
      },
      Console: null as any,
    } as Console;
  }

  /**
   * 检查权限
   */
  private checkPermission(permission: PluginPermission): void {
    if (!this.permissions.has(permission)) {
      throw new PluginPermissionError(
        `插件 ${this.pluginId} 未声明 ${permission} 权限`,
        this.pluginId
      );
    }
  }

  /**
   * 在沙箱中执行代码
   */
  execute<T = any>(code: string): SandboxResult<T> {
    try {
      // 创建受限的 Function 构造函数
      const sandboxKeys = Object.keys(this.sandboxEnv);
      const sandboxValues = Object.values(this.sandboxEnv);

      // 使用 Function 构造函数创建沙箱函数
      const sandboxFunction = new Function(
        ...sandboxKeys,
        `"use strict"; ${code}`
      );

      // 在沙箱环境中执行
      const result = sandboxFunction(...sandboxValues);

      return {
        success: true,
        result,
      };
    } catch (error) {
      return {
        success: false,
        error: error instanceof Error ? error.message : String(error),
      };
    }
  }

  /**
   * 在沙箱中执行异步代码
   */
  async executeAsync<T = any>(code: string): Promise<SandboxResult<T>> {
    try {
      // 创建受限的 Function 构造函数
      const sandboxKeys = Object.keys(this.sandboxEnv);
      const sandboxValues = Object.values(this.sandboxEnv);

      // 使用 Function 构造函数创建沙箱函数
      const sandboxFunction = new Function(
        ...sandboxKeys,
        `"use strict"; return (async () => { ${code} })()`
      );

      // 在沙箱环境中执行
      const result = await sandboxFunction(...sandboxValues);

      return {
        success: true,
        result,
      };
    } catch (error) {
      return {
        success: false,
        error: error instanceof Error ? error.message : String(error),
      };
    }
  }

  /**
   * 加载并执行插件模块
   */
  async loadPluginModule(moduleCode: string): Promise<SandboxResult<any>> {
    try {
      // 创建模块包装器
      const moduleWrapper = `
        const module = { exports: {} };
        const exports = module.exports;
        
        ${moduleCode}
        
        return module.exports;
      `;

      // 在沙箱中执行
      const result = await this.executeAsync(moduleWrapper);

      if (!result.success) {
        return result;
      }

      // 验证模块导出
      const pluginModule = result.result;
      if (!pluginModule || typeof pluginModule !== 'object') {
        return {
          success: false,
          error: '插件模块必须导出一个对象',
        };
      }

      return {
        success: true,
        result: pluginModule,
      };
    } catch (error) {
      return {
        success: false,
        error: error instanceof Error ? error.message : String(error),
      };
    }
  }

  /**
   * 获取沙箱环境（用于测试）
   */
  getSandboxEnvironment(): Record<string, any> {
    return { ...this.sandboxEnv };
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
 * 创建插件沙箱
 */
export function createPluginSandbox(config: SandboxConfig): PluginSandbox {
  return new PluginSandbox(config);
}

/**
 * 验证插件权限
 */
export function validatePluginPermissions(
  requestedPermissions: PluginPermission[],
  manifestPermissions: PluginPermission[]
): { valid: boolean; missing: PluginPermission[] } {
  const manifestSet = new Set(manifestPermissions);
  const missing: PluginPermission[] = [];

  for (const permission of requestedPermissions) {
    if (!manifestSet.has(permission)) {
      missing.push(permission);
    }
  }

  return {
    valid: missing.length === 0,
    missing,
  };
}
