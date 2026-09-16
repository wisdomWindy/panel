import { ref } from 'vue';
import { defineStore } from 'pinia';
import {
  invokeDesktop,
  normalizeDesktopError,
  type BlacklistSnapshot,
  type DesktopError,
  type EventPayloadMap,
  type BlacklistEntry,
  type ImportResult,
  type PathGrantId,
  type PathPurpose,
  type SensitiveFileFormat,
  type SensitiveMatchType,
  type SensitiveWord,
  type SensitiveWordCategory,
  type SensitiveWordSnapshot
} from '@/platform/desktop';

export const useLiveSafetyStore = defineStore('live-safety', () => {
  const sensitiveWords = ref<SensitiveWordSnapshot>({ items: [], todayHitCount: 0 });
  const blacklist = ref<BlacklistSnapshot>({ items: [], todayAddedCount: 0 });
  const pending = ref(false);
  const mutationPending = ref(false);
  const error = ref<DesktopError | null>(null);
  const compileProgress = ref({ percentage: 0, stage: '' });
  const lastExport = ref<EventPayloadMap['export_done'] | null>(null);

  async function load(): Promise<void> {
    if (pending.value) return;
    pending.value = true;
    error.value = null;
    try {
      const [words, blocked] = await Promise.all([
        invokeDesktop('sensitive_word_list', {}),
        invokeDesktop('blacklist_list', undefined)
      ]);
      sensitiveWords.value = {
        items: words.items.map(item => ({ ...item })),
        todayHitCount: words.todayHitCount
      };
      blacklist.value = {
        items: blocked.items.map(item => ({ ...item })),
        todayAddedCount: blocked.todayAddedCount
      };
    } catch (value) {
      error.value = normalizeDesktopError(value);
      throw value;
    } finally {
      pending.value = false;
    }
  }

  async function refreshSensitiveWords(): Promise<void> {
    const value = await invokeDesktop('sensitive_word_list', {});
    sensitiveWords.value = {
      items: value.items.map(item => ({ ...item })),
      todayHitCount: value.todayHitCount
    };
  }

  async function refreshBlacklist(): Promise<void> {
    const value = await invokeDesktop('blacklist_list', undefined);
    blacklist.value = {
      items: value.items.map(item => ({ ...item })),
      todayAddedCount: value.todayAddedCount
    };
  }

  async function mutate<T>(effect: () => Promise<T>): Promise<T> {
    if (mutationPending.value) {
      throw {
        code: 'operation_pending',
        message: '操作正在进行，请稍后重试',
        retryable: true
      } satisfies DesktopError;
    }
    mutationPending.value = true;
    error.value = null;
    try {
      return await effect();
    } catch (value) {
      error.value = normalizeDesktopError(value);
      throw value;
    } finally {
      mutationPending.value = false;
    }
  }

  async function withPathGrant<T>(
    purpose: PathPurpose,
    effect: (grantId: PathGrantId) => Promise<T>
  ): Promise<T> {
    const grant = await invokeDesktop('path_select', { purpose });
    try {
      return await effect(grant.grantId);
    } finally {
      await invokeDesktop('path_revoke', { grantId: grant.grantId }).catch(() => undefined);
    }
  }

  type SensitiveForm = {
    word: string;
    category: SensitiveWordCategory;
    matchType: SensitiveMatchType;
    isEnabled: boolean;
  };

  async function saveSensitiveWord(form: SensitiveForm, id?: number): Promise<SensitiveWord> {
    return mutate(async () => {
      const value =
        id === undefined
          ? await invokeDesktop('sensitive_word_add', form)
          : await invokeDesktop('sensitive_word_update', { id, ...form });
      await refreshSensitiveWords();
      return value;
    });
  }

  async function toggleSensitiveWord(id: number, isEnabled: boolean): Promise<void> {
    await mutate(async () => {
      await invokeDesktop('sensitive_word_toggle', { id, isEnabled });
      await refreshSensitiveWords();
    });
  }

  async function deleteSensitiveWord(id: number): Promise<void> {
    await mutate(async () => {
      await invokeDesktop('sensitive_word_delete', { id });
      await refreshSensitiveWords();
    });
  }

  async function importSensitiveWords(format: SensitiveFileFormat): Promise<ImportResult | null> {
    return mutate(() =>
      withPathGrant('sensitiveWordImport', async grantId => {
        const result = await invokeDesktop('sensitive_word_import', { grantId, format });
        await refreshSensitiveWords();
        return result;
      })
    );
  }

  async function exportSensitiveWords(format: SensitiveFileFormat): Promise<void> {
    await mutate(() =>
      withPathGrant('sensitiveWordExport', grantId =>
        invokeDesktop('sensitive_word_export', { grantId, format })
      )
    );
  }

  async function addBlacklist(
    entry: Pick<BlacklistEntry, 'uid' | 'username' | 'reason'>
  ): Promise<void> {
    await mutate(async () => {
      await invokeDesktop('blacklist_add', entry);
      await refreshBlacklist();
    });
  }

  async function removeBlacklist(uid: BlacklistEntry['uid']): Promise<void> {
    await mutate(async () => {
      await invokeDesktop('blacklist_remove', { uid });
      await refreshBlacklist();
    });
  }

  async function importBlacklist(): Promise<ImportResult | null> {
    return mutate(() =>
      withPathGrant('blacklistImport', async grantId => {
        const result = await invokeDesktop('blacklist_import', { grantId });
        await refreshBlacklist();
        return result;
      })
    );
  }

  async function exportBlacklist(): Promise<void> {
    await mutate(() =>
      withPathGrant('blacklistExport', grantId => invokeDesktop('blacklist_export', { grantId }))
    );
  }

  function setCompileProgress(value: { percentage: number; stage: string }): void {
    compileProgress.value = { ...value };
  }

  function setLastExport(value: EventPayloadMap['export_done']): void {
    lastExport.value = { ...value };
  }

  return {
    sensitiveWords,
    blacklist,
    pending,
    mutationPending,
    error,
    compileProgress,
    lastExport,
    load,
    saveSensitiveWord,
    toggleSensitiveWord,
    deleteSensitiveWord,
    importSensitiveWords,
    exportSensitiveWords,
    addBlacklist,
    removeBlacklist,
    importBlacklist,
    exportBlacklist,
    setCompileProgress,
    setLastExport
  };
});
