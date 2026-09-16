import { ref } from 'vue';
import { defineStore } from 'pinia';
import {
  invokeDesktop,
  normalizeDesktopError,
  type ConfigExportResult,
  type ConfigImportResult,
  type ConfigOverview,
  type ConfigResetResult,
  type DesktopError,
  type LocalePreferences,
  type ShellConfigScope,
  type SystemMetrics,
  type UiPreferences
} from '@/platform/desktop';
import {
  adaptConfigOverview,
  adaptLocalePreferences,
  adaptSystemMetrics,
  adaptUiPreferences
} from '@/modules/application-shell/adapters';

export const useApplicationShellStore = defineStore('application-shell', () => {
  const metrics = ref<SystemMetrics | null>(null);
  const statusObservedAt = ref(Date.now());
  const locale = ref<LocalePreferences | null>(null);
  const savedUi = ref<UiPreferences | null>(null);
  const previewUi = ref<UiPreferences | null>(null);
  const configOverview = ref<ConfigOverview | null>(null);
  const pending = ref(true);
  const pendingOperations = ref<ReadonlySet<string>>(new Set());
  const error = ref<DesktopError | null>(null);
  let generation = 0;
  let previewGeneration = 0;
  let bootstrapPromise: Promise<void> | null = null;
  const operations = new Map<string, Promise<unknown>>();

  function replaceMetrics(value: unknown): void {
    metrics.value = adaptSystemMetrics(value);
    statusObservedAt.value = Date.now();
  }

  function replaceLocale(value: unknown): void {
    locale.value = adaptLocalePreferences(value);
  }

  function replaceUi(value: unknown): void {
    const adapted = adaptUiPreferences(value);
    savedUi.value = adapted;
    previewUi.value = adapted;
  }

  function replacePreviewUi(value: unknown): void {
    previewUi.value = adaptUiPreferences(value);
  }

  function captureError(value: unknown): DesktopError {
    const mapped = normalizeDesktopError(value);
    error.value = mapped;
    return mapped;
  }

  function runOperation<T>(key: string, effect: () => Promise<T>): Promise<T> {
    const existing = operations.get(key) as Promise<T> | undefined;
    if (existing) return existing;
    error.value = null;
    pendingOperations.value = new Set([...pendingOperations.value, key]);
    const current = effect()
      .catch(value => { throw captureError(value); })
      .finally(() => {
        if (operations.get(key) === current) operations.delete(key);
        const next = new Set(pendingOperations.value);
        next.delete(key);
        pendingOperations.value = next;
      });
    operations.set(key, current);
    return current;
  }

  function bootstrap(force = false): Promise<void> {
    if (!force && bootstrapPromise) return bootstrapPromise;
    const localGeneration = ++generation;
    pending.value = true;
    error.value = null;
    const current = Promise.all([
      invokeDesktop('app_bootstrap', undefined),
      invokeDesktop('shell_status', undefined),
      invokeDesktop('locale_get', undefined),
      invokeDesktop('ui_get', undefined)
    ])
      .then(([, metricValue, localeValue, uiValue]) => {
        const adapted = {
          metrics: adaptSystemMetrics(metricValue),
          locale: adaptLocalePreferences(localeValue),
          ui: adaptUiPreferences(uiValue)
        };
        if (generation === localGeneration) {
          metrics.value = adapted.metrics;
          locale.value = adapted.locale;
          savedUi.value = adapted.ui;
          previewUi.value = adapted.ui;
        }
      })
      .catch(value => {
        if (generation === localGeneration) captureError(value);
        throw value;
      })
      .finally(() => {
        if (bootstrapPromise === current) bootstrapPromise = null;
        if (generation === localGeneration) pending.value = false;
      });
    bootstrapPromise = current;
    return current;
  }

  async function refreshMetrics(): Promise<void> {
    try {
      replaceMetrics(await invokeDesktop('shell_status', undefined));
    } catch (value) {
      captureError(value);
    } finally {
      statusObservedAt.value = Date.now();
    }
  }

  function loadConfigOverview(): Promise<ConfigOverview> {
    return runOperation('configOverview', async () => {
      const value = adaptConfigOverview(await invokeDesktop('config_overview', undefined));
      configOverview.value = value;
      return value;
    });
  }

  function openConfigDirectory(): Promise<{ opened: true }> {
    return runOperation('openConfigDirectory', () => invokeDesktop('config_open_directory', undefined));
  }

  async function selectConfigGrant(purpose: 'configExport' | 'configImport') {
    try {
      return await invokeDesktop('path_select', { purpose });
    } catch (value) {
      const mapped = normalizeDesktopError(value);
      if (mapped.code === 'dialog_cancelled') return null;
      throw mapped;
    }
  }

  function exportConfig(scopes: readonly ShellConfigScope[]): Promise<ConfigExportResult | null> {
    return runOperation('exportConfig', async () => {
      const grant = await selectConfigGrant('configExport');
      if (!grant) return null;
      try {
        return await invokeDesktop('config_export', { grantId: grant.grantId, scopes });
      } finally {
        await invokeDesktop('path_revoke', { grantId: grant.grantId }).catch(() => undefined);
      }
    });
  }

  async function refreshPreferencesAfterConfigMutation(): Promise<void> {
    const [localeResult, uiResult] = await Promise.allSettled([
      invokeDesktop('locale_get', undefined),
      invokeDesktop('ui_get', undefined)
    ]);
    try {
      if (localeResult.status === 'fulfilled') replaceLocale(localeResult.value);
      else captureError(localeResult.reason);
    } catch (value) {
      captureError(value);
    }
    try {
      if (uiResult.status === 'fulfilled') replaceUi(uiResult.value);
      else if (!error.value) captureError(uiResult.reason);
    } catch (value) {
      if (!error.value) captureError(value);
    }
  }

  function importConfig(): Promise<ConfigImportResult | null> {
    return runOperation('importConfig', async () => {
      const grant = await selectConfigGrant('configImport');
      if (!grant) return null;
      try {
        const result = await invokeDesktop('config_import', { grantId: grant.grantId });
        await refreshPreferencesAfterConfigMutation();
        return result;
      } finally {
        await invokeDesktop('path_revoke', { grantId: grant.grantId }).catch(() => undefined);
      }
    });
  }

  function resetConfig(): Promise<ConfigResetResult> {
    return runOperation('resetConfig', async () => {
      const result = await invokeDesktop('config_reset', undefined);
      await refreshPreferencesAfterConfigMutation();
      return result;
    });
  }

  function setLocaleAuto(isEnabled: boolean): Promise<LocalePreferences> {
    return runOperation('locale', async () => {
      const value = adaptLocalePreferences(await invokeDesktop('locale_set_auto', { isEnabled }));
      locale.value = value;
      return value;
    });
  }

  function saveLocale(preferences: LocalePreferences): Promise<LocalePreferences> {
    return runOperation('locale', async () => {
      const value = adaptLocalePreferences(await invokeDesktop('locale_save', { preferences }));
      locale.value = value;
      return value;
    });
  }

  function previewPreferences(preferences: UiPreferences): Promise<UiPreferences> {
    const localGeneration = ++previewGeneration;
    error.value = null;
    pendingOperations.value = new Set([...pendingOperations.value, 'previewUi']);
    return invokeDesktop('ui_preview', { preferences })
      .then(value => {
        const adapted = adaptUiPreferences(value);
        if (localGeneration === previewGeneration) previewUi.value = adapted;
        return adapted;
      })
      .catch(value => {
        if (localGeneration !== previewGeneration) {
          return previewUi.value ?? savedUi.value ?? preferences;
        }
        previewUi.value = savedUi.value;
        throw captureError(value);
      })
      .finally(() => {
        if (localGeneration !== previewGeneration) return;
        const next = new Set(pendingOperations.value);
        next.delete('previewUi');
        pendingOperations.value = next;
      });
  }

  function invalidatePreview(): void {
    previewGeneration += 1;
    const next = new Set(pendingOperations.value);
    next.delete('previewUi');
    pendingOperations.value = next;
  }

  function saveUi(preferences: UiPreferences): Promise<UiPreferences> {
    invalidatePreview();
    return runOperation('saveUi', async () => {
      try {
        const value = adaptUiPreferences(await invokeDesktop('ui_save', { preferences }));
        savedUi.value = value;
        previewUi.value = value;
        return value;
      } catch (value) {
        previewUi.value = savedUi.value;
        throw value;
      }
    });
  }

  function resetUi(): Promise<UiPreferences> {
    invalidatePreview();
    return runOperation('resetUi', async () => {
      const value = adaptUiPreferences(await invokeDesktop('ui_reset', undefined));
      savedUi.value = value;
      previewUi.value = value;
      return value;
    });
  }

  function dispose(): void {
    generation += 1;
    invalidatePreview();
    bootstrapPromise = null;
    operations.clear();
    pending.value = false;
    pendingOperations.value = new Set();
  }

  return {
    metrics, statusObservedAt, locale, savedUi, previewUi, configOverview, pending, pendingOperations, error,
    bootstrap, refreshMetrics, loadConfigOverview, openConfigDirectory, exportConfig,
    importConfig, resetConfig, setLocaleAuto, saveLocale, previewPreferences, saveUi,
    resetUi, replaceMetrics, replaceLocale, replaceUi, replacePreviewUi, dispose
  };
});
