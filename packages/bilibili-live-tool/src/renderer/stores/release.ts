import { ref } from 'vue';
import { defineStore } from 'pinia';
import {
  invokeDesktop,
  normalizeDesktopError,
  type AboutSnapshot,
  type DesktopError,
  type PluginSnapshot,
  type ResilienceSnapshot,
  type UpdateSnapshot
} from '@/platform/desktop';
import {
  adaptAboutSnapshot,
  adaptPluginSnapshot,
  adaptResilienceSnapshot,
  adaptUpdateSnapshot
} from '@/modules/release/adapters';

export const useReleaseStore = defineStore('release', () => {
  const update = ref<UpdateSnapshot | null>(null);
  const plugins = ref<readonly PluginSnapshot[]>([]);
  const about = ref<AboutSnapshot | null>(null);
  const resilience = ref<ResilienceSnapshot | null>(null);
  const pending = ref<ReadonlySet<string>>(new Set());
  const error = ref<DesktopError | null>(null);
  const operations = new Map<string, Promise<unknown>>();
  let generation = 0;

  function setPending(key: string, isPending: boolean): void {
    const next = new Set(pending.value);
    if (isPending) next.add(key);
    else next.delete(key);
    pending.value = next;
  }

  function captureError(value: unknown): DesktopError {
    const mapped = normalizeDesktopError(value);
    error.value = mapped;
    return mapped;
  }

  function run<T>(key: string, effect: () => Promise<T>): Promise<T> {
    const existing = operations.get(key) as Promise<T> | undefined;
    if (existing) return existing;
    error.value = null;
    setPending(key, true);
    const current = effect()
      .catch(value => { throw captureError(value); })
      .finally(() => {
        if (operations.get(key) === current) operations.delete(key);
        setPending(key, false);
      });
    operations.set(key, current);
    return current;
  }

  async function bootstrap(): Promise<void> {
    const localGeneration = ++generation;
    await run('bootstrap', async () => {
      const [updateValue, pluginValues, aboutValue, resilienceValue] = await Promise.all([
        invokeDesktop('update_status', undefined),
        invokeDesktop('plugin_list', undefined),
        invokeDesktop('about_info', undefined),
        invokeDesktop('resilience_status', undefined)
      ]);
      if (generation !== localGeneration) return;
      update.value = adaptUpdateSnapshot(updateValue);
      plugins.value = Object.freeze(pluginValues.map(adaptPluginSnapshot));
      about.value = adaptAboutSnapshot(aboutValue);
      resilience.value = adaptResilienceSnapshot(resilienceValue);
    });
  }

  async function checkUpdate(): Promise<void> {
    await run('checkUpdate', async () => {
      update.value = adaptUpdateSnapshot(await invokeDesktop('update_check', undefined));
    });
  }

  async function downloadUpdate(): Promise<void> {
    await run('downloadUpdate', async () => {
      update.value = adaptUpdateSnapshot(await invokeDesktop('update_download', undefined));
    });
  }

  async function installUpdate(): Promise<void> {
    await run('installUpdate', async () => {
      update.value = adaptUpdateSnapshot(await invokeDesktop('update_install', undefined));
    });
  }

  async function skipUpdate(version: string): Promise<void> {
    await run('skipUpdate', async () => {
      update.value = adaptUpdateSnapshot(await invokeDesktop('update_skip', { version }));
    });
  }

  async function refreshPlugins(): Promise<void> {
    await run('refreshPlugins', async () => {
      plugins.value = Object.freeze((await invokeDesktop('plugin_list', undefined)).map(adaptPluginSnapshot));
    });
  }

  async function selectPluginInstall(): Promise<PluginSnapshot> {
    return run('selectPluginInstall', async () =>
      adaptPluginSnapshot(await invokeDesktop('plugin_select_install', undefined))
    );
  }

  async function installPlugin(pluginId: string): Promise<void> {
    await run(`plugin:${pluginId}`, async () => {
      replacePlugin(adaptPluginSnapshot(await invokeDesktop('plugin_install', { pluginId })));
    });
  }

  async function togglePlugin(plugin: PluginSnapshot): Promise<void> {
    await run(`plugin:${plugin.id}`, async () => {
      replacePlugin(adaptPluginSnapshot(await invokeDesktop('plugin_toggle', {
        pluginId: plugin.id,
        isEnabled: plugin.status !== 'enabled'
      })));
    });
  }

  async function uninstallPlugin(pluginId: string): Promise<void> {
    await run(`plugin:${pluginId}`, async () => {
      if (await invokeDesktop('plugin_uninstall', { pluginId })) {
        plugins.value = Object.freeze(plugins.value.filter(plugin => plugin.id !== pluginId));
      }
    });
  }

  function replaceUpdate(value: unknown): void {
    update.value = adaptUpdateSnapshot(value);
  }

  function replacePlugin(value: unknown): void {
    const adapted = adaptPluginSnapshot(value);
    const index = plugins.value.findIndex(plugin => plugin.id === adapted.id);
    plugins.value = Object.freeze(index < 0
      ? [...plugins.value, adapted]
      : plugins.value.map(plugin => plugin.id === adapted.id ? adapted : plugin));
  }

  function dispose(): void {
    generation += 1;
    operations.clear();
    pending.value = new Set();
  }

  return {
    update, plugins, about, resilience, pending, error,
    bootstrap, checkUpdate, downloadUpdate, installUpdate, skipUpdate,
    refreshPlugins, selectPluginInstall, installPlugin, togglePlugin, uninstallPlugin,
    replaceUpdate, replacePlugin, dispose
  };
});
