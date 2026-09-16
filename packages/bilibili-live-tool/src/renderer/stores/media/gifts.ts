import { computed, ref } from 'vue';
import { defineStore } from 'pinia';
import {
  invokeDesktop,
  normalizeDesktopError,
  type DesktopError,
  type GiftAlertPayload,
  type GiftId,
  type GiftNotifyConfig,
  type GiftNotifySnapshot,
  type GiftSoundBinding,
  type GiftSoundSnapshot,
  type PathGrantId,
  type PathPurpose,
  type SoundPreviewTarget
} from '@/platform/desktop';
import {
  adaptGiftAlertPayload,
  adaptGiftNotifySnapshot,
  adaptGiftSoundBinding,
  adaptGiftSoundSnapshot
} from '@/modules/media/adapters';

export const useMediaGiftsStore = defineStore('media-gifts', () => {
  const notifySnapshot = ref<GiftNotifySnapshot | null>(null);
  const soundSnapshot = ref<GiftSoundSnapshot | null>(null);
  const latestAlert = ref<GiftAlertPayload | null>(null);
  const pending = ref(false);
  const pendingOperations = ref<ReadonlySet<string>>(new Set());
  const error = ref<DesktopError | null>(null);
  let generation = 0;
  let loadPromise: Promise<void> | null = null;
  const operations = new Map<string, Promise<unknown>>();

  const hasPendingMutation = computed(() => pendingOperations.value.size > 0);

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

  function read(force: boolean): Promise<void> {
    if (!force && loadPromise) return loadPromise;
    const localGeneration = ++generation;
    pending.value = true;
    error.value = null;
    const current = Promise.all([
      invokeDesktop('gift_notify_get', undefined),
      invokeDesktop('gift_sound_list', undefined)
    ])
      .then(([notify, sounds]) => {
        const adaptedNotify = adaptGiftNotifySnapshot(notify);
        const adaptedSounds = adaptGiftSoundSnapshot(sounds);
        if (localGeneration === generation) {
          notifySnapshot.value = adaptedNotify;
          soundSnapshot.value = adaptedSounds;
        }
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

  function load(): Promise<void> {
    return read(false);
  }

  function refresh(): Promise<void> {
    return read(true);
  }

  function saveNotify(config: GiftNotifyConfig): Promise<GiftNotifySnapshot> {
    return runOperation('saveNotify', async () => {
      const adapted = adaptGiftNotifySnapshot(await invokeDesktop('gift_notify_save', { config }));
      notifySnapshot.value = adapted;
      return adapted;
    });
  }

  function testNotify(config: GiftNotifyConfig): Promise<void> {
    return runOperation('testNotify', () =>
      invokeDesktop('gift_notify_test', { config }).then(() => undefined)
    );
  }

  async function withGrant<T>(purpose: PathPurpose, effect: (grantId: PathGrantId) => Promise<T>): Promise<T> {
    const grant = await invokeDesktop('path_select', { purpose });
    try {
      return await effect(grant.grantId);
    } finally {
      await invokeDesktop('path_revoke', { grantId: grant.grantId }).catch(() => undefined);
    }
  }

  function bindSound(giftId: GiftId, giftName: string): Promise<GiftSoundBinding> {
    return runOperation(`bindSound:${giftId}`, () =>
      withGrant('giftSoundFile', async grantId => {
        const binding = adaptGiftSoundBinding(
          await invokeDesktop('gift_sound_set', { giftId, giftName, grantId })
        );
        await refreshSounds();
        return binding;
      })
    );
  }

  function removeSound(giftId: GiftId): Promise<void> {
    return runOperation(`removeSound:${giftId}`, async () => {
      await invokeDesktop('gift_sound_remove', { giftId });
      await refreshSounds();
    });
  }

  function saveSounds(globalVolume: number, bindings: readonly GiftSoundBinding[]): Promise<GiftSoundSnapshot> {
    return runOperation('saveSounds', async () => {
      const adapted = adaptGiftSoundSnapshot(
        await invokeDesktop('gift_sound_save', { globalVolume, bindings })
      );
      soundSnapshot.value = adapted;
      return adapted;
    });
  }

  function importSounds(): Promise<number> {
    return runOperation('importSounds', () =>
      withGrant('giftSoundImport', async grantId => {
        const result = await invokeDesktop('gift_sound_import', { grantId });
        await refreshSounds();
        return result.importedCount;
      })
    );
  }

  function exportSounds(): Promise<string> {
    return runOperation('exportSounds', () =>
      withGrant('giftSoundExport', async grantId => {
        const result = await invokeDesktop('gift_sound_export', { grantId });
        return result.displayPath;
      })
    );
  }

  function previewSound(target: SoundPreviewTarget, volume: number): Promise<void> {
    return runOperation('previewSound', () =>
      invokeDesktop('sound_preview', { target, volume }).then(() => undefined)
    );
  }

  async function refreshSounds(): Promise<void> {
    soundSnapshot.value = adaptGiftSoundSnapshot(
      await invokeDesktop('gift_sound_list', undefined)
    );
  }

  function acceptAlert(value: unknown): void {
    try {
      latestAlert.value = adaptGiftAlertPayload(value);
    } catch (value) {
      error.value = normalizeDesktopError(value);
    }
  }

  function replaceNotify(value: unknown): void {
    try {
      notifySnapshot.value = adaptGiftNotifySnapshot(value);
    } catch (value) {
      error.value = normalizeDesktopError(value);
    }
  }

  function dispose(): void {
    generation += 1;
    loadPromise = null;
    operations.clear();
    pending.value = false;
    pendingOperations.value = new Set();
  }

  return {
    notifySnapshot,
    soundSnapshot,
    latestAlert,
    pending,
    pendingOperations,
    hasPendingMutation,
    error,
    load,
    refresh,
    saveNotify,
    testNotify,
    bindSound,
    removeSound,
    saveSounds,
    importSounds,
    exportSounds,
    previewSound,
    acceptAlert,
    replaceNotify,
    dispose
  };
});
