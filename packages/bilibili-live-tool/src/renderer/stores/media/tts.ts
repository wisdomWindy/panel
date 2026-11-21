import { ref } from 'vue';
import { defineStore } from 'pinia';
import {
  invokeDesktop,
  normalizeDesktopError,
  type DesktopError,
  type Qwen3DownloadState,
  type Qwen3ModelCatalog,
  type Qwen3TtsConfig,
  type Qwen3TtsSnapshot,
  type TtsConfig,
  type TtsSnapshot,
  type TtsStatus
} from '@/platform/desktop';
import {
  adaptQwen3DownloadState,
  adaptQwen3ModelCatalog,
  adaptQwen3TtsSnapshot,
  adaptTtsSnapshot,
  adaptTtsStatus
} from '@/modules/media/adapters';

export const useMediaTtsStore = defineStore('media-tts', () => {
  const snapshot = ref<TtsSnapshot | null>(null);
  const qwen3 = ref<Qwen3TtsSnapshot | null>(null);
  const qwen3Catalog = ref<Qwen3ModelCatalog | null>(null);
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

  async function loadQwen3(): Promise<Qwen3TtsSnapshot | null> {
    error.value = null;
    try {
      const adapted = adaptQwen3TtsSnapshot(await invokeDesktop('tts_qwen3_get', undefined));
      qwen3.value = adapted;
      return adapted;
    } catch (value) {
      const mapped = normalizeDesktopError(value);
      error.value = mapped;
      throw mapped;
    }
  }

  function saveQwen3(config: Qwen3TtsConfig): Promise<Qwen3TtsSnapshot> {
    return runOperation('qwen3Save', async () => {
      const adapted = adaptQwen3TtsSnapshot(await invokeDesktop('tts_qwen3_save', { config }));
      qwen3.value = adapted;
      return adapted;
    });
  }

  function testQwen3(): Promise<Qwen3TtsSnapshot> {
    return runOperation('qwen3Test', async () => {
      const adapted = adaptQwen3TtsSnapshot(await invokeDesktop('tts_qwen3_test', undefined));
      qwen3.value = adapted;
      return adapted;
    });
  }

  let modelPoll: ReturnType<typeof setInterval> | null = null;

  function stopModelPolling(): void {
    if (modelPoll !== null) {
      clearInterval(modelPoll);
      modelPoll = null;
    }
  }

  function startModelPolling(): void {
    if (modelPoll !== null) return;
    modelPoll = setInterval(() => {
      void loadQwen3Models().catch(() => stopModelPolling());
    }, 2000);
  }

  async function loadQwen3Models(): Promise<Qwen3ModelCatalog> {
    const adapted = adaptQwen3ModelCatalog(await invokeDesktop('tts_qwen3_models', undefined));
    qwen3Catalog.value = adapted;
    if (adapted.download.status !== 'downloading') stopModelPolling();
    return adapted;
  }

  /** Downloads the 1.7B CustomVoice checkpoint unless a model is named. */
  function downloadQwen3Model(model: string | null = null): Promise<Qwen3DownloadState> {
    return runOperation('qwen3Download', async () => {
      const state = adaptQwen3DownloadState(
        await invokeDesktop('tts_qwen3_download_model', { model })
      );
      if (qwen3Catalog.value) {
        qwen3Catalog.value = Object.freeze({ ...qwen3Catalog.value, download: state });
      }
      if (state.status === 'downloading') startModelPolling();
      return state;
    });
  }

  function dispose(): void {
    generation += 1;
    loadPromise = null;
    operations.clear();
    pending.value = false;
    pendingOperations.value = new Set();
    stopModelPolling();
  }

  return { snapshot, qwen3, qwen3Catalog, pending, pendingOperations, error, load, refresh, save, setEnabled, test, pause, clear, replaceStatus, loadQwen3, saveQwen3, testQwen3, loadQwen3Models, downloadQwen3Model, stopModelPolling, dispose };
});
