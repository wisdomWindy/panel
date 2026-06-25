export interface PluginInstallTarget {
  sourcePath: string;
  pluginId: string;
  targetDirectoryName: string;
}

export interface PluginRuntimeStateSnapshot {
  initialized: boolean;
  plugins: Array<{ id: string; enabled: boolean }>;
}

export function resolvePluginInstallTarget(
  sourcePath: string,
  manifest: { id: string }
): PluginInstallTarget {
  return {
    sourcePath,
    pluginId: manifest.id,
    targetDirectoryName: manifest.id
  };
}

export function toPluginDirectoryPath(pluginRoot: string, pluginId: string): string {
  return `${pluginRoot}/${pluginId}`;
}

export function createPluginRuntimeState(): PluginRuntimeStateSnapshot {
  return {
    initialized: false,
    plugins: []
  };
}
