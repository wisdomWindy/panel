/**
 * 插件加载器
 * 负责从本地目录加载插件，验证插件格式和兼容性
 */

import {
  PluginManifest,
  PluginLoadResult,
  PluginValidationResult,
  PluginLoadError,
  PluginValidationError,
  PLUGIN_MANIFEST_FILE,
  PLUGIN_ENTRY_FILE,
} from './types';

// 插件加载器配置
interface PluginLoaderConfig {
  pluginDir: string;
  appVersion: string;
}

// 插件文件信息
interface PluginFileInfo {
  path: string;
  manifest: PluginManifest;
  entryPath: string;
}

/**
 * 插件加载器类
 */
export class PluginLoader {
  private pluginDir: string;
  private appVersion: string;

  constructor(config: PluginLoaderConfig) {
    this.pluginDir = config.pluginDir;
    this.appVersion = config.appVersion;
  }

  /**
   * 加载单个插件
   */
  async loadPlugin(pluginId: string): Promise<PluginLoadResult> {
    try {
      // 1. 获取插件目录路径
      const pluginPath = this.getPluginPath(pluginId);

      // 2. 验证插件目录存在
      const dirExists = await this.checkDirectoryExists(pluginPath);
      if (!dirExists) {
        return {
          success: false,
          error: `插件目录不存在: ${pluginPath}`,
        };
      }

      // 3. 读取并验证 manifest.json
      const manifestResult = await this.loadManifest(pluginPath);
      if (!manifestResult.success) {
        return {
          success: false,
          error: manifestResult.error,
        };
      }

      const manifest = manifestResult.manifest!;

      // 4. 验证插件格式
      const validationResult = this.validateManifest(manifest);
      if (!validationResult.valid) {
        return {
          success: false,
          error: `插件格式验证失败: ${validationResult.errors.join(', ')}`,
        };
      }

      // 5. 检查版本兼容性
      const versionResult = this.checkVersionCompatibility(manifest);
      if (!versionResult.compatible) {
        return {
          success: false,
          error: versionResult.error,
        };
      }

      // 6. 验证入口文件存在
      const entryPath = this.getEntryPath(pluginPath, manifest.main);
      const entryExists = await this.checkFileExists(entryPath);
      if (!entryExists) {
        return {
          success: false,
          error: `插件入口文件不存在: ${entryPath}`,
        };
      }

      // 7. 读取插件代码
      const code = await this.readPluginCode(entryPath);
      if (!code.success) {
        return {
          success: false,
          error: code.error,
        };
      }

      return {
        success: true,
        plugin: {
          id: manifest.id,
          name: manifest.name,
          version: manifest.version,
          description: manifest.description,
          author: manifest.author,
        },
        warnings: versionResult.warnings,
      };
    } catch (error) {
      return {
        success: false,
        error: error instanceof Error ? error.message : String(error),
      };
    }
  }

  /**
   * 加载所有插件
   */
  async loadAllPlugins(): Promise<PluginLoadResult[]> {
    try {
      // 1. 获取插件目录下的所有子目录
      const pluginDirs = await this.getPluginDirectories();

      // 2. 加载每个插件
      const results: PluginLoadResult[] = [];
      for (const pluginDir of pluginDirs) {
        const result = await this.loadPlugin(pluginDir);
        results.push(result);
      }

      return results;
    } catch (error) {
      return [
        {
          success: false,
          error: error instanceof Error ? error.message : String(error),
        },
      ];
    }
  }

  /**
   * 验证插件清单
   */
  validateManifest(manifest: any): PluginValidationResult {
    const errors: string[] = [];
    const warnings: string[] = [];

    // 检查必填字段
    if (!manifest.id || typeof manifest.id !== 'string') {
      errors.push('缺少必填字段: id');
    }

    if (!manifest.name || typeof manifest.name !== 'string') {
      errors.push('缺少必填字段: name');
    }

    if (!manifest.version || typeof manifest.version !== 'string') {
      errors.push('缺少必填字段: version');
    }

    if (!manifest.description || typeof manifest.description !== 'string') {
      errors.push('缺少必填字段: description');
    }

    if (!manifest.author || typeof manifest.author !== 'string') {
      errors.push('缺少必填字段: author');
    }

    if (!manifest.main || typeof manifest.main !== 'string') {
      errors.push('缺少必填字段: main');
    }

    if (!manifest.minAppVersion || typeof manifest.minAppVersion !== 'string') {
      errors.push('缺少必填字段: minAppVersion');
    }

    // 检查权限字段
    if (manifest.permissions) {
      if (!Array.isArray(manifest.permissions)) {
        errors.push('permissions 必须是数组');
      } else {
        const validPermissions = ['danmaku', 'gift', 'captain', 'config', 'effect', 'ui'];
        for (const permission of manifest.permissions) {
          if (!validPermissions.includes(permission)) {
            errors.push(`无效的权限: ${permission}`);
          }
        }
      }
    }

    // 检查版本格式
    if (manifest.version && !this.isValidVersion(manifest.version)) {
      warnings.push('版本号格式不规范，建议使用 Semantic Versioning');
    }

    // 检查 ID 格式
    if (manifest.id && !/^[a-z0-9-]+$/.test(manifest.id)) {
      warnings.push('ID 格式不规范，建议使用小写字母、数字和连字符');
    }

    return {
      valid: errors.length === 0,
      errors,
      warnings,
    };
  }

  /**
   * 检查版本兼容性
   */
  checkVersionCompatibility(manifest: PluginManifest): {
    compatible: boolean;
    error?: string;
    warnings?: string[];
  } {
    const warnings: string[] = [];

    // 比较版本
    const comparison = this.compareVersions(manifest.minAppVersion, this.appVersion);

    if (comparison > 0) {
      return {
        compatible: false,
        error: `插件要求最低应用版本 ${manifest.minAppVersion}，当前版本 ${this.appVersion}`,
      };
    }

    // 如果插件要求的版本低于当前版本，添加警告
    if (comparison < 0) {
      warnings.push(
        `插件针对较低版本开发 (${manifest.minAppVersion})，可能存在兼容性问题`
      );
    }

    return {
      compatible: true,
      warnings,
    };
  }

  /**
   * 获取插件目录路径
   */
  private getPluginPath(pluginId: string): string {
    return `${this.pluginDir}/${pluginId}`;
  }

  /**
   * 获取入口文件路径
   */
  private getEntryPath(pluginPath: string, main: string): string {
    return `${pluginPath}/${main}`;
  }

  /**
   * 加载 manifest.json
   */
  private async loadManifest(pluginPath: string): Promise<{
    success: boolean;
    manifest?: PluginManifest;
    error?: string;
  }> {
    try {
      const manifestPath = `${pluginPath}/${PLUGIN_MANIFEST_FILE}`;
      const content = await this.readFile(manifestPath);

      if (!content.success) {
        return {
          success: false,
          error: `无法读取 manifest.json: ${content.error}`,
        };
      }

      try {
        const manifest = JSON.parse(content.content!);
        return {
          success: true,
          manifest,
        };
      } catch (parseError) {
        return {
          success: false,
          error: `manifest.json 格式错误: ${parseError}`,
        };
      }
    } catch (error) {
      return {
        success: false,
        error: error instanceof Error ? error.message : String(error),
      };
    }
  }

  /**
   * 读取插件代码
   */
  private async readPluginCode(entryPath: string): Promise<{
    success: boolean;
    code?: string;
    error?: string;
  }> {
    try {
      const content = await this.readFile(entryPath);

      if (!content.success) {
        return {
          success: false,
          error: `无法读取插件入口文件: ${content.error}`,
        };
      }

      return {
        success: true,
        code: content.content,
      };
    } catch (error) {
      return {
        success: false,
        error: error instanceof Error ? error.message : String(error),
      };
    }
  }

  /**
   * 检查目录是否存在
   */
  private async checkDirectoryExists(path: string): Promise<boolean> {
    try {
      // 在 Electron 环境中使用 fs 模块
      if (typeof window !== 'undefined' && window.electronAPI) {
        return await window.electronAPI.checkDirectoryExists(path);
      }
      // 在浏览器环境中模拟
      return true;
    } catch {
      return false;
    }
  }

  /**
   * 检查文件是否存在
   */
  private async checkFileExists(path: string): Promise<boolean> {
    try {
      // 在 Electron 环境中使用 fs 模块
      if (typeof window !== 'undefined' && window.electronAPI) {
        return await window.electronAPI.checkFileExists(path);
      }
      // 在浏览器环境中模拟
      return true;
    } catch {
      return false;
    }
  }

  /**
   * 读取文件内容
   */
  private async readFile(path: string): Promise<{
    success: boolean;
    content?: string;
    error?: string;
  }> {
    try {
      // 在 Electron 环境中使用 fs 模块
      if (typeof window !== 'undefined' && window.electronAPI) {
        const content = await window.electronAPI.readFile(path);
        return {
          success: true,
          content,
        };
      }
      // 在浏览器环境中模拟
      return {
        success: false,
        error: '文件系统访问不可用',
      };
    } catch (error) {
      return {
        success: false,
        error: error instanceof Error ? error.message : String(error),
      };
    }
  }

  /**
   * 获取插件目录下的所有子目录
   */
  private async getPluginDirectories(): Promise<string[]> {
    try {
      // 在 Electron 环境中使用 fs 模块
      if (typeof window !== 'undefined' && window.electronAPI) {
        return await window.electronAPI.getDirectories(this.pluginDir);
      }
      // 在浏览器环境中模拟
      return [];
    } catch {
      return [];
    }
  }

  /**
   * 验证版本号格式
   */
  private isValidVersion(version: string): boolean {
    // 简单的版本号格式验证
    return /^\d+\.\d+\.\d+(-[a-zA-Z0-9.]+)?(\+[a-zA-Z0-9.]+)?$/.test(version);
  }

  /**
   * 比较版本号
   * @returns -1 if v1 < v2, 0 if v1 === v2, 1 if v1 > v2
   */
  private compareVersions(v1: string, v2: string): number {
    const parts1 = v1.split('.').map(Number);
    const parts2 = v2.split('.').map(Number);

    for (let i = 0; i < 3; i++) {
      const p1 = parts1[i] || 0;
      const p2 = parts2[i] || 0;

      if (p1 > p2) return 1;
      if (p1 < p2) return -1;
    }

    return 0;
  }

  /**
   * 获取插件目录路径
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
}

/**
 * 创建插件加载器
 */
export function createPluginLoader(config: PluginLoaderConfig): PluginLoader {
  return new PluginLoader(config);
}
