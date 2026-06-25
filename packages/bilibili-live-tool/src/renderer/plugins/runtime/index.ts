import { createPluginManager, type PluginManager } from '../plugin-manager.js';
import { getDanmakuService } from '../../services/danmaku-service.js';
import { useConfigStore } from '../../stores/config.js';

const DEFAULT_PLUGIN_DIR = 'examples/plugins';
const APP_VERSION = '0.1.0';

let pluginManagerInstance: PluginManager | null = null;
let initialized = false;

function createConfigAdapter() {
  const configStore = useConfigStore();

  return {
    getConfig<T>(key: string): T {
      const normalizedKey = key.split('.').pop() as keyof typeof configStore.config;
      return configStore.config[normalizedKey] as T;
    },
    setConfig<T>(key: string, value: T): void {
      const normalizedKey = key.split('.').pop() as keyof typeof configStore.config;
      configStore.updateConfig({ [normalizedKey]: value } as never);
    }
  };
}

function createPluginManagerInstance(): PluginManager {
  return createPluginManager({
    pluginDir: DEFAULT_PLUGIN_DIR,
    appVersion: APP_VERSION,
    dependencies: {
      danmakuService: getDanmakuService(),
      configStore: createConfigAdapter()
    }
  });
}

export function getPluginManager(): PluginManager {
  if (!pluginManagerInstance) {
    pluginManagerInstance = createPluginManagerInstance();
  }

  return pluginManagerInstance;
}

export async function ensurePluginManagerInitialized(): Promise<PluginManager> {
  const manager = getPluginManager();

  if (!initialized) {
    await manager.initialize();
    initialized = true;
  }

  return manager;
}

export function resetPluginManagerForTests(): void {
  pluginManagerInstance = null;
  initialized = false;
}
