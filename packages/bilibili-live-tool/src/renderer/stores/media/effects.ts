import { ref } from 'vue';
import { defineStore } from 'pinia';
import {
  invokeDesktop,
  normalizeDesktopError,
  type CaptainEffectConfig,
  type CaptainEffectSnapshot,
  type DesktopError,
  type PathGrantId,
  type SoundBindingSummary
} from '@/platform/desktop';
import { adaptCaptainEffectSnapshot } from '@/modules/media/adapters';

export const useMediaEffectsStore = defineStore('media-effects', () => {
  const snapshot = ref<CaptainEffectSnapshot | null>(null);
  const pending = ref(false);
  const savePending = ref(false);
  const previewPending = ref(false);
  const error = ref<DesktopError | null>(null);
  let generation = 0;
  let loadPromise: Promise<CaptainEffectSnapshot | null> | null = null;
  let savePromise: Promise<CaptainEffectSnapshot> | null = null;
  let previewPromise: Promise<void> | null = null;
  const temporaryGrants = new Set<PathGrantId>();

  function read(force: boolean): Promise<CaptainEffectSnapshot | null> {
    if (!force && loadPromise) return loadPromise;
    const localGeneration = ++generation;
    pending.value = true;
    error.value = null;
    const current = invokeDesktop('captain_effect_get', undefined)
      .then(value => {
        const adapted = adaptCaptainEffectSnapshot(value);
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

  function load(): Promise<CaptainEffectSnapshot | null> {
    return read(false);
  }

  function refresh(): Promise<CaptainEffectSnapshot | null> {
    return read(true);
  }

  function save(config: CaptainEffectConfig): Promise<CaptainEffectSnapshot> {
    if (savePromise) return savePromise;
    const localGeneration = generation;
    savePending.value = true;
    error.value = null;
    const current = invokeDesktop('captain_effect_save', { config })
      .then(value => {
        const adapted = adaptCaptainEffectSnapshot(value);
        if (localGeneration === generation) snapshot.value = adapted;
        revokeTemporaryGrants();
        return adapted;
      })
      .catch(value => {
        const mapped = normalizeDesktopError(value);
        if (localGeneration === generation) error.value = mapped;
        throw mapped;
      })
      .finally(() => {
        if (savePromise === current) savePromise = null;
        if (localGeneration === generation) savePending.value = false;
      });
    savePromise = current;
    return current;
  }

  function preview(config: CaptainEffectConfig): Promise<void> {
    if (previewPromise) return previewPromise;
    const localGeneration = generation;
    previewPending.value = true;
    error.value = null;
    const current = invokeDesktop('captain_effect_preview', { config })
      .then(() => undefined)
      .catch(value => {
        const mapped = normalizeDesktopError(value);
        if (localGeneration === generation) error.value = mapped;
        throw mapped;
      })
      .finally(() => {
        if (previewPromise === current) previewPromise = null;
        if (localGeneration === generation) previewPending.value = false;
      });
    previewPromise = current;
    return current;
  }

  async function selectSound(): Promise<SoundBindingSummary> {
    const grant = await invokeDesktop('path_select', { purpose: 'giftSoundFile' });
    temporaryGrants.add(grant.grantId);
    const segments = grant.displayPath.replace(/\\/g, '/').split('/');
    return Object.freeze({
      bindingId: grant.grantId,
      fileName: segments.at(-1) || grant.displayPath,
      isMissing: false
    });
  }

  function previewSound(binding: SoundBindingSummary, volume: number): Promise<void> {
    const target = temporaryGrants.has(binding.bindingId as PathGrantId)
      ? { kind: 'grant' as const, grantId: binding.bindingId as PathGrantId }
      : { kind: 'binding' as const, bindingId: binding.bindingId };
    return invokeDesktop('sound_preview', { target, volume }).then(() => undefined);
  }

  function revokeTemporaryGrants(): void {
    const grants = [...temporaryGrants];
    temporaryGrants.clear();
    grants.forEach(grantId => {
      void invokeDesktop('path_revoke', { grantId }).catch(() => undefined);
    });
  }

  function dispose(): void {
    generation += 1;
    loadPromise = null;
    savePromise = null;
    previewPromise = null;
    pending.value = false;
    savePending.value = false;
    previewPending.value = false;
    revokeTemporaryGrants();
  }

  return {
    snapshot,
    pending,
    savePending,
    previewPending,
    error,
    load,
    refresh,
    save,
    preview,
    selectSound,
    previewSound,
    dispose
  };
});
