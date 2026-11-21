<script setup lang="ts">
import { computed, onMounted, ref } from 'vue';
import { useI18n } from 'vue-i18n';
import { ElMessage, ElMessageBox } from 'element-plus';
import {
  invokeDesktop,
  normalizeDesktopError,
  type DataClearScope,
  type RetentionPolicy
} from '@/platform/desktop';

const { t } = useI18n();

const policy = ref<RetentionPolicy>({ eventLogDays: 90, aiLogDays: 30, statDays: 180 });
const isLoading = ref(false);
const isSaving = ref(false);
const clearingScope = ref<DataClearScope | null>(null);

/** `null` is the wire value for "keep forever" (PRD 12.3). */
const FOREVER = 0;

const eventOptions = computed(() => [
  { label: t('settings.data.days', { days: 7 }), value: 7 },
  { label: t('settings.data.days', { days: 30 }), value: 30 },
  { label: t('settings.data.days', { days: 90 }), value: 90 },
  { label: t('settings.data.days', { days: 180 }), value: 180 },
  { label: t('settings.data.forever'), value: FOREVER }
]);
const aiOptions = computed(() =>
  [7, 30, 90, FOREVER].map(days => ({
    label: days === FOREVER ? t('settings.data.forever') : t('settings.data.days', { days }),
    value: days
  }))
);
const statOptions = computed(() =>
  [30, 90, 180, FOREVER].map(days => ({
    label: days === FOREVER ? t('settings.data.forever') : t('settings.data.days', { days }),
    value: days
  }))
);

const clearActions = computed<readonly { scope: DataClearScope; labelKey: string; danger: boolean }[]>(
  () => [
    { scope: 'eventLogs', labelKey: 'settings.data.clearEventLogs', danger: false },
    { scope: 'aiLogs', labelKey: 'settings.data.clearAiLogs', danger: false },
    { scope: 'cache', labelKey: 'settings.data.clearCache', danger: false },
    { scope: 'all', labelKey: 'settings.data.clearAll', danger: true }
  ]
);

function toWire(value: number | null): number {
  return value ?? 0;
}

function fromWire(value: number): number | null {
  return value > 0 ? value : null;
}

async function load(): Promise<void> {
  isLoading.value = true;
  try {
    const loaded = await invokeDesktop('data_retention_get', undefined);
    policy.value = loaded;
  } catch (error) {
    ElMessage.error(normalizeDesktopError(error).message ?? t('settings.data.loadFailed'));
  } finally {
    isLoading.value = false;
  }
}

async function save(): Promise<void> {
  isSaving.value = true;
  try {
    policy.value = await invokeDesktop('data_retention_save', { policy: policy.value });
    ElMessage.success(t('settings.data.saved'));
  } catch (error) {
    ElMessage.error(normalizeDesktopError(error).message ?? t('settings.data.saveFailed'));
  } finally {
    isSaving.value = false;
  }
}

async function clearData(scope: DataClearScope, danger: boolean): Promise<void> {
  try {
    await ElMessageBox.confirm(
      danger ? t('settings.data.confirmAll') : t('settings.data.confirmScope'),
      t('settings.data.confirmTitle'),
      {
        type: danger ? 'error' : 'warning',
        confirmButtonText: t('settings.data.confirm'),
        cancelButtonText: t('common.cancel')
      }
    );
  } catch {
    return;
  }
  clearingScope.value = scope;
  try {
    await invokeDesktop('data_clear', { scope });
    ElMessage.success(t('settings.data.cleared'));
  } catch (error) {
    ElMessage.error(normalizeDesktopError(error).message ?? t('settings.data.clearFailed'));
  } finally {
    clearingScope.value = null;
  }
}

onMounted(() => {
  void load();
});
</script>

<template>
  <div class="flex flex-col gap-4">
    <section class="app-card p-4">
      <h3 class="app-text m-0 text-sm font-semibold">{{ t('settings.data.retention') }}</h3>
      <p class="app-muted mt-1 mb-3 text-sm">{{ t('settings.data.retentionHint') }}</p>
      <el-skeleton v-if="isLoading" :rows="3" animated />
      <el-form v-else label-position="top">
        <el-form-item :label="t('settings.data.eventLogs')">
          <el-select
            :model-value="toWire(policy.eventLogDays)"
            class="!w-48"
            @update:model-value="policy.eventLogDays = fromWire(Number($event))"
          >
            <el-option
              v-for="option in eventOptions"
              :key="option.value"
              :label="option.label"
              :value="option.value"
            />
          </el-select>
        </el-form-item>
        <el-form-item :label="t('settings.data.aiLogs')">
          <el-select
            :model-value="toWire(policy.aiLogDays)"
            class="!w-48"
            @update:model-value="policy.aiLogDays = fromWire(Number($event))"
          >
            <el-option
              v-for="option in aiOptions"
              :key="option.value"
              :label="option.label"
              :value="option.value"
            />
          </el-select>
        </el-form-item>
        <el-form-item :label="t('settings.data.stats')">
          <el-select
            :model-value="toWire(policy.statDays)"
            class="!w-48"
            @update:model-value="policy.statDays = fromWire(Number($event))"
          >
            <el-option
              v-for="option in statOptions"
              :key="option.value"
              :label="option.label"
              :value="option.value"
            />
          </el-select>
        </el-form-item>
      </el-form>
      <button
        type="button"
        class="app-btn-primary mt-2 h-8 px-4 text-sm"
        :disabled="isSaving || isLoading"
        @click="save"
      >
        {{ t('common.save') }}
      </button>
    </section>

    <section class="app-card p-4">
      <h3 class="app-text m-0 text-sm font-semibold">{{ t('settings.data.clear') }}</h3>
      <p class="app-muted mt-1 mb-3 text-sm">{{ t('settings.data.clearHint') }}</p>
      <div class="flex flex-wrap gap-2">
        <button
          v-for="action in clearActions"
          :key="action.scope"
          type="button"
          class="app-chip"
          :class="{ 'app-chip--danger': action.danger }"
          :disabled="clearingScope !== null"
          @click="clearData(action.scope, action.danger)"
        >
          {{ t(action.labelKey) }}
        </button>
      </div>
    </section>
  </div>
</template>

<style scoped>
.app-chip--danger {
  border-color: color-mix(in srgb, var(--app-red) 45%, transparent);
  color: var(--app-red);
}
</style>
