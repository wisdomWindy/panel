import { ref } from 'vue';
import { defineStore } from 'pinia';
import {
  invokeDesktop,
  normalizeDesktopError,
  type AboutSnapshot,
  type DesktopError,
  type ResilienceSnapshot,
  type UpdateSnapshot
} from '@/platform/desktop';
import {
  adaptAboutSnapshot,
  adaptResilienceSnapshot,
  adaptUpdateSnapshot
} from '@/modules/release/adapters';

export const useReleaseStore = defineStore('release', () => {
  const update = ref<UpdateSnapshot | null>(null);
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

  function run<T>(key: string, effect: () => Promise<T>): Promise<T> {
    const existing = operations.get(key) as Promise<T> | undefined;
    if (existing) return existing;
    const localGeneration = generation;
    error.value = null;
    setPending(key, true);
    const current = effect()
      .catch(value => {
        const mapped = normalizeDesktopError(value);
        if (generation === localGeneration) error.value = mapped;
        throw mapped;
      })
      .finally(() => {
        if (operations.get(key) !== current) return;
        operations.delete(key);
        setPending(key, false);
      });
    operations.set(key, current);
    return current;
  }

  async function bootstrap(): Promise<void> {
    const localGeneration = ++generation;
    await run('bootstrap', async () => {
      const [updateValue, aboutValue, resilienceValue] = await Promise.all([
        invokeDesktop('update_status', undefined),
        invokeDesktop('about_info', undefined),
        invokeDesktop('resilience_status', undefined)
      ]);
      if (generation !== localGeneration) return;
      update.value = adaptUpdateSnapshot(updateValue);
      about.value = adaptAboutSnapshot(aboutValue);
      resilience.value = adaptResilienceSnapshot(resilienceValue);
    });
  }

  async function checkUpdate(): Promise<void> {
    const localGeneration = generation;
    await run('checkUpdate', async () => {
      const nextUpdate = adaptUpdateSnapshot(await invokeDesktop('update_check', undefined));
      if (generation === localGeneration) update.value = nextUpdate;
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

  function replaceUpdate(value: unknown): void {
    update.value = adaptUpdateSnapshot(value);
  }

  function dispose(): void {
    generation += 1;
    operations.clear();
    pending.value = new Set();
  }

  return {
    update, about, resilience, pending, error,
    bootstrap, checkUpdate, downloadUpdate, installUpdate, skipUpdate,
    replaceUpdate, dispose
  };
});
