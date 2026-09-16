import { ref } from 'vue';
import { defineStore } from 'pinia';
import {
  invokeDesktop,
  normalizeDesktopError,
  type DesktopError,
  type TtsConfig,
  type TtsSnapshot,
  type TtsStatus
} from '@/platform/desktop';
import { adaptTtsSnapshot, adaptTtsStatus } from '@/modules/media/adapters';

export const useMediaTtsStore = defineStore('media-tts', () => {
  const snapshot = ref<TtsSnapshot | null>(null);
  const pending = ref(false);
  const pendingOperations = ref<ReadonlySet<string>>(new Set());
  const error = ref<DesktopError | null>(null);
  let generation = 0;
  let loadPromise: Promise<TtsSnapshot | null> | null = null;
  const operations = new Map<string, Promise<unknown>>();

  function runOperation<T>(key: string, effect: () => Promise<T>): Promise<T> {
    const existing = operations.get(key) as Promise<T> | undefined;
    if (existing) return existing;
    const localGeneration = generation;
    error.value = null;
    pendingOperations.value = new Set([...pendingOperations.value, key]);
    const current = effect()
      .catch(value => {
        const mapped = normalizeDesktopError(value);
        if (localGeneration === generation) error.value = mapped;
        throw mapped;
      })
      .finally(() => {
        if (operations.get(key) === current) operations.delete(key);
        if (localGeneration === generation) {
          const next = new Set(pendingOperations.value);
          next.delete(key);
          pendingOperations.value = next;
        }
      });
    operations.set(key, current);
    return current;
  }

  function read(force: boolean): Promise<TtsSnapshot | null> {
    if (!force && loadPromise) return loadPromise;
    const localGeneration = ++generation;
    pending.value = true;
    error.value = null;
    const current = invokeDesktop('tts_get', undefined)
      .then(value => {
        const adapted = adaptTtsSnapshot(value);
        if (localGeneration === generation) snapshot.value = adapted;
        return localGeneration === generation ? adapted : null;
      })
      .catch(value => {
        const mapped = normalizeDesktopError(value);
        if (localGeneration === generation) error.value = mapped;
        throw mapped;
      })
      .finally(() => {
        if (loadPromise === current) loadPromise = null;
        if (localGeneration === generation) pending.value = false;
      });
    loadPromise = current;
    return current;
  }

  function load(): Promise<TtsSnapshot | null> {
    return read(false);
  }

  function refresh(): Promise<TtsSnapshot | null> {
    return read(true);
  }

  function replaceStatus(value: unknown): void {
    try {
      const status = adaptTtsStatus(value);
      if (snapshot.value) snapshot.value = Object.freeze({ ...snapshot.value, status });
    } catch (value) {
      error.value = normalizeDesktopError(value);
    }
  }

  function save(config: TtsConfig): Promise<TtsSnapshot> {
    return runOperation('save', async () => {
      const adapted = adaptTtsSnapshot(await invokeDesktop('tts_save', { config }));
      snapshot.value = adapted;
      return adapted;
    });
  }

  function commitStatus(key: string, effect: () => Promise<TtsStatus>): Promise<TtsStatus> {
    return runOperation(key, async () => {
      const status = adaptTtsStatus(await effect());
      if (snapshot.value) snapshot.value = Object.freeze({ ...snapshot.value, status });
      return status;
    });
  }

  function setEnabled(isEnabled: boolean): Promise<TtsStatus> {
    return commitStatus('setEnabled', () => invokeDesktop('tts_set_enabled', { isEnabled }));
  }

  function test(config: TtsConfig): Promise<TtsStatus> {
    return commitStatus('test', () => invokeDesktop('tts_test', { config }));
  }

  function pause(isPaused: boolean): Promise<TtsStatus> {
    return commitStatus('pause', () => invokeDesktop('tts_pause', { isPaused }));
  }

  function clear(): Promise<TtsStatus> {
    return commitStatus('clear', () => invokeDesktop('tts_clear', undefined));
  }

  function dispose(): void {
    generation += 1;
    loadPromise = null;
    operations.clear();
    pending.value = false;
    pendingOperations.value = new Set();
  }

  return { snapshot, pending, pendingOperations, error, load, refresh, save, setEnabled, test, pause, clear, replaceStatus, dispose };
});
