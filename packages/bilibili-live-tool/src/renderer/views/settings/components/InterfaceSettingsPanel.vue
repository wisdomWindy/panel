<script setup lang="ts">
import { computed, ref, watch } from 'vue';
import { ElMessage } from 'element-plus';
import { useI18n } from 'vue-i18n';
import type { FourRegionLayout, UiPreferences } from '@/platform/desktop';
import { useApplicationShellStore } from '@/stores/application-shell';
import LayoutEditorDialog from './LayoutEditorDialog.vue';

const defaultUi = (): UiPreferences => ({
  theme: 'dark',
  accentColor: '#FF6699',
  opacity: 0.95,
  fontSize: 'medium',
  layoutMode: 'default',
  customLayout: null,
  showDanmakuLevel: true,
  showDanmakuMedal: true,
  showDanmakuTime: true,
  maxDanmakuEntries: 1000
});
const defaultLayout = (): FourRegionLayout => ({
  version: 1,
  navigationWidth: 208,
  toolbarHeight: 56,
  statusHeight: 32
});

const { t } = useI18n();
const shell = useApplicationShellStore();
const draft = ref<UiPreferences>(defaultUi());
const layoutVisible = ref(false);
const isPending = computed(() =>
  shell.pendingOperations.has('saveUi') ||
  shell.pendingOperations.has('resetUi')
);

watch(
  () => shell.savedUi,
  value => {
    if (value) draft.value = structuredClone(value);
  },
  { immediate: true }
);

async function preview(): Promise<void> {
  if (!shell.savedUi) return;
  try {
    await shell.previewPreferences(structuredClone(draft.value));
  } catch {
    if (shell.savedUi) draft.value = structuredClone(shell.savedUi);
    ElMessage.error(t('settings.feedback.operationFailed'));
  }
}

function selectLayoutMode(value: UiPreferences['layoutMode']): void {
  draft.value.layoutMode = value;
  draft.value.customLayout = value === 'custom'
    ? draft.value.customLayout ?? defaultLayout()
    : null;
  if (value === 'custom') layoutVisible.value = true;
  void preview();
}

function saveLayout(value: FourRegionLayout): void {
  draft.value = { ...draft.value, layoutMode: 'custom', customLayout: value };
  void preview();
}

async function save(): Promise<void> {
  try {
    await shell.saveUi(structuredClone(draft.value));
    ElMessage.success(t('settings.feedback.saved'));
  } catch {
    if (shell.savedUi) draft.value = structuredClone(shell.savedUi);
    ElMessage.error(t('settings.feedback.operationFailed'));
  }
}

async function reset(): Promise<void> {
  try {
    const value = await shell.resetUi();
    draft.value = structuredClone(value);
    ElMessage.success(t('settings.feedback.reset'));
  } catch {
    ElMessage.error(t('settings.feedback.operationFailed'));
  }
}
</script>

<template>
  <el-form label-position="top" class="max-w-3xl">
    <div class="grid grid-cols-2 gap-x-7">
      <el-form-item :label="t('settings.interface.theme')">
        <el-segmented
          v-model="draft.theme"
          :options="[
            { label: t('settings.interface.themeLight'), value: 'light' },
            { label: t('settings.interface.themeDark'), value: 'dark' },
            { label: t('settings.interface.themeSystem'), value: 'system' }
          ]"
          :disabled="!shell.savedUi || isPending"
          @change="preview"
        />
      </el-form-item>
      <el-form-item :label="t('settings.interface.accentColor')">
        <el-color-picker
          v-model="draft.accentColor"
          :disabled="!shell.savedUi || isPending"
          @change="preview"
        />
      </el-form-item>
      <el-form-item :label="t('settings.interface.opacity')">
        <el-slider
          v-model="draft.opacity"
          :min="0"
          :max="1"
          :step="0.05"
          show-input
          :disabled="!shell.savedUi || isPending"
          @change="preview"
        />
      </el-form-item>
      <el-form-item :label="t('settings.interface.fontSize')">
        <el-select v-model="draft.fontSize" :disabled="!shell.savedUi || isPending" @change="preview">
          <el-option :label="t('settings.interface.fontSmall')" value="small" />
          <el-option :label="t('settings.interface.fontMedium')" value="medium" />
          <el-option :label="t('settings.interface.fontLarge')" value="large" />
        </el-select>
      </el-form-item>
    </div>

    <div class="my-5 border-t border-gray-200 dark:border-gray-700" />
    <h3 class="mb-4 text-sm font-semibold text-gray-900 dark:text-gray-100">
      {{ t('settings.interface.layoutAndDisplay') }}
    </h3>
    <el-form-item :label="t('settings.interface.layout')">
      <el-segmented
        :model-value="draft.layoutMode"
        :options="[
          { label: t('settings.interface.layoutDefault'), value: 'default' },
          { label: t('settings.interface.layoutCustom'), value: 'custom' }
        ]"
        :disabled="!shell.savedUi || isPending"
        @change="selectLayoutMode($event as UiPreferences['layoutMode'])"
      />
      <el-button
        v-if="draft.layoutMode === 'custom'"
        class="ml-3"
        :disabled="!shell.savedUi || isPending"
        @click="layoutVisible = true"
      >
        {{ t('settings.interface.editLayout') }}
      </el-button>
    </el-form-item>
    <div class="grid grid-cols-2 gap-x-7">
      <el-form-item :label="t('settings.interface.danmakuFields')">
        <div class="flex flex-wrap gap-x-5">
          <el-checkbox v-model="draft.showDanmakuLevel" :disabled="!shell.savedUi || isPending" @change="preview">
            {{ t('settings.interface.showLevel') }}
          </el-checkbox>
          <el-checkbox v-model="draft.showDanmakuMedal" :disabled="!shell.savedUi || isPending" @change="preview">
            {{ t('settings.interface.showMedal') }}
          </el-checkbox>
          <el-checkbox v-model="draft.showDanmakuTime" :disabled="!shell.savedUi || isPending" @change="preview">
            {{ t('settings.interface.showTime') }}
          </el-checkbox>
        </div>
      </el-form-item>
      <el-form-item :label="t('settings.interface.maxDanmakuEntries')">
        <el-input-number
          v-model="draft.maxDanmakuEntries"
          :min="1"
          :max="10000"
          :step="100"
          :disabled="!shell.savedUi || isPending"
          @change="preview"
        />
      </el-form-item>
    </div>

    <div class="mt-5 flex gap-3 border-t border-gray-200 pt-5 dark:border-gray-700">
      <el-button
        type="primary"
        :loading="shell.pendingOperations.has('saveUi')"
        :disabled="!shell.savedUi"
        @click="save"
      >
        {{ t('common.save') }}
      </el-button>
      <el-button :loading="shell.pendingOperations.has('resetUi')" :disabled="!shell.savedUi" @click="reset">
        {{ t('settings.interface.restoreDefaults') }}
      </el-button>
    </div>
  </el-form>

  <LayoutEditorDialog
    v-model:visible="layoutVisible"
    :model-value="draft.customLayout ?? defaultLayout()"
    @save="saveLayout"
  />
</template>
