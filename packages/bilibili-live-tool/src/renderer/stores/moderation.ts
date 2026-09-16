import { computed, ref } from 'vue';
import { defineStore } from 'pinia';
import {
  invokeDesktop,
  normalizeDesktopError,
  type DesktopError,
  type ModerationAlert,
  type ModerationAlertId,
  type ModerationLogEntry,
  type ModerationRules,
  type ModerationSummary,
  type MuteDuration,
  type UserUid
} from '@/platform/desktop';
import {
  adaptModerationAlert,
  adaptModerationLogs,
  adaptModerationSummary
} from '@/modules/member/adapters';

export const useModerationStore = defineStore('member-moderation', () => {
  const summary = ref<ModerationSummary | null>(null);
  const logs = ref<readonly ModerationLogEntry[]>([]);
  const pendingOperations = ref<ReadonlySet<string>>(new Set());
  const error = ref<DesktopError | null>(null);
  const operations = new Map<string, Promise<unknown>>();
  let generation = 0;

  const alerts = computed(() => summary.value?.alerts ?? Object.freeze([]));
  const rules = computed(() => summary.value?.rules ?? null);

  function run<T>(key: string, effect: () => Promise<T>): Promise<T> {
    const existing = operations.get(key) as Promise<T> | undefined;
    if (existing) return existing;
    const localGeneration = generation;
    error.value = null;
    pendingOperations.value = new Set([...pendingOperations.value, key]);
    const current = effect()
      .catch(reason => {
        const mapped = normalizeDesktopError(reason);
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

  function bootstrap(): Promise<ModerationSummary> {
    return run('bootstrap', async () => {
      const next = adaptModerationSummary(await invokeDesktop('moderate_status', undefined));
      summary.value = next;
      return next;
    });
  }

  function setAuto(isEnabled: boolean, isRiskAccepted: boolean): Promise<ModerationSummary> {
    return run('setAuto', async () => {
      const next = adaptModerationSummary(
        await invokeDesktop('moderate_set_auto', { isEnabled, isRiskAccepted })
      );
      summary.value = next;
      return next;
    });
  }

  function updateRules(value: ModerationRules): Promise<ModerationSummary> {
    return run('updateRules', async () => {
      const next = adaptModerationSummary(
        await invokeDesktop('moderate_update_rules', { rules: value })
      );
      summary.value = next;
      return next;
    });
  }

  function mute(alertId: ModerationAlertId, duration: MuteDuration): Promise<ModerationAlert> {
    return run(`mute:${alertId}`, async () => {
      const next = adaptModerationAlert(
        await invokeDesktop('moderate_mute', { alertId, duration })
      );
      replaceAlert(next);
      return next;
    });
  }

  function ignore(alertId: ModerationAlertId): Promise<ModerationSummary> {
    return run(`ignore:${alertId}`, async () => {
      const next = adaptModerationSummary(await invokeDesktop('moderate_ignore', { alertId }));
      summary.value = next;
      return next;
    });
  }

  function loadLogs(filters: {
    userUid: UserUid | null;
    startedAt: number | null;
    endedAt: number | null;
  }): Promise<readonly ModerationLogEntry[]> {
    return run('loadLogs', async () => {
      const next = adaptModerationLogs(await invokeDesktop('moderate_log_query', filters));
      logs.value = next;
      return next;
    });
  }

  function markFalsePositive(logId: number): Promise<readonly ModerationLogEntry[]> {
    return run(`falsePositive:${logId}`, async () => {
      const next = adaptModerationLogs(
        await invokeDesktop('moderate_mark_false_positive', { logId })
      );
      logs.value = next;
      return next;
    });
  }

  function replaceAlert(value: unknown): void {
    const alert = adaptModerationAlert(value);
    const current = summary.value;
    if (!current) return;
    summary.value = Object.freeze({
      ...current,
      alerts: Object.freeze([
        alert,
        ...current.alerts.filter(item => item.alertId !== alert.alertId)
      ])
    });
  }

  function replaceSummary(value: unknown): void {
    summary.value = adaptModerationSummary(value);
  }

  function dispose(): void {
    generation += 1;
    operations.clear();
    pendingOperations.value = new Set();
    error.value = null;
    logs.value = [];
  }

  return {
    summary,
    logs,
    alerts,
    rules,
    pendingOperations,
    error,
    bootstrap,
    setAuto,
    updateRules,
    mute,
    ignore,
    loadLogs,
    markFalsePositive,
    replaceAlert,
    replaceSummary,
    dispose
  };
});
