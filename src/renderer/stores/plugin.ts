/**
 * 插件状态管理
 * 使用 Pinia 管理插件相关状态
 */

import { defineStore } from 'pinia';
import { ref, computed } from 'vue';
import { PluginState, PluginPermission } from '../plugins/types';

export const usePluginStore = defineStore('plugin', () => {
  // 状态
  const plugins = ref<PluginState[]>([]);
  const loading = ref(false);
  const error = ref<string | null>(null);

  // 计算属性
  const installedPlugins = computed(() => plugins.value);
  const enabledPlugins = computed(() => plugins.value.filter((p) => p.enabled));
  const disabledPlugins = computed(() => plugins.value.filter((p) => !p.enabled));
  const pluginCount = computed(() => plugins.value.length);
  const enabledCount = computed(() => plugins.value.filter((p) => p.enabled).length);

  // 获取插件
  function getPlugin(pluginId: string): PluginState | undefined {
    return plugins.value.find((p) => p.id === pluginId);
  }

  // 检查插件是否已安装
  function isPluginInstalled(pluginId: string): boolean {
    return plugins.value.some((p) => p.id === pluginId);
  }

  // 检查插件是否已启用
  function isPluginEnabled(pluginId: string): boolean {
    return plugins.value.find((p) => p.id === pluginId)?.enabled ?? false;
  }

  // 添加插件
  function addPlugin(plugin: PluginState): void {
    const existingIndex = plugins.value.findIndex((p) => p.id === plugin.id);
    if (existingIndex >= 0) {
      // 更新现有插件
      plugins.value[existingIndex] = plugin;
    } else {
      // 添加新插件
      plugins.value.push(plugin);
    }
  }

  // 移除插件
  function removePlugin(pluginId: string): void {
    const index = plugins.value.findIndex((p) => p.id === pluginId);
    if (index >= 0) {
      plugins.value.splice(index, 1);
    }
  }

  // 更新插件状态
  function updatePluginState(pluginId: string, updates: Partial<PluginState>): void {
    const plugin = plugins.value.find((p) => p.id === pluginId);
    if (plugin) {
      Object.assign(plugin, updates);
    }
  }

  // 启用插件
  function enablePlugin(pluginId: string): void {
    const plugin = plugins.value.find((p) => p.id === pluginId);
    if (plugin) {
      plugin.enabled = true;
      plugin.updatedAt = new Date().toISOString();
    }
  }

  // 禁用插件
  function disablePlugin(pluginId: string): void {
    const plugin = plugins.value.find((p) => p.id === pluginId);
    if (plugin) {
      plugin.enabled = false;
      plugin.updatedAt = new Date().toISOString();
    }
  }

  // 更新插件配置
  function updatePluginConfig(pluginId: string, config: Record<string, any>): void {
    const plugin = plugins.value.find((p) => p.id === pluginId);
    if (plugin) {
      plugin.config = { ...plugin.config, ...config };
      plugin.updatedAt = new Date().toISOString();
    }
  }

  // 获取插件配置
  function getPluginConfig(pluginId: string): Record<string, any> {
    return plugins.value.find((p) => p.id === pluginId)?.config ?? {};
  }

  // 获取插件权限
  function getPluginPermissions(pluginId: string): PluginPermission[] {
    return plugins.value.find((p) => p.id === pluginId)?.permissions ?? [];
  }

  // 检查插件是否有指定权限
  function hasPluginPermission(pluginId: string, permission: PluginPermission): boolean {
    const plugin = plugins.value.find((p) => p.id === pluginId);
    return plugin?.permissions.includes(permission) ?? false;
  }

  // 设置加载状态
  function setLoading(value: boolean): void {
    loading.value = value;
  }

  // 设置错误信息
  function setError(message: string | null): void {
    error.value = message;
  }

  // 清空插件列表
  function clearPlugins(): void {
    plugins.value = [];
  }

  // 批量设置插件
  function setPlugins(newPlugins: PluginState[]): void {
    plugins.value = newPlugins;
  }

  return {
    // 状态
    plugins,
    loading,
    error,

    // 计算属性
    installedPlugins,
    enabledPlugins,
    disabledPlugins,
    pluginCount,
    enabledCount,

    // 方法
    getPlugin,
    isPluginInstalled,
    isPluginEnabled,
    addPlugin,
    removePlugin,
    updatePluginState,
    enablePlugin,
    disablePlugin,
    updatePluginConfig,
    getPluginConfig,
    getPluginPermissions,
    hasPluginPermission,
    setLoading,
    setError,
    clearPlugins,
    setPlugins,
  };
});
