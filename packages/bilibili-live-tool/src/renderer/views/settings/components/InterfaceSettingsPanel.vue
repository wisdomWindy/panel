<script setup lang="ts">
import { computed, ref, watch } from 'vue';
import { Loading } from '@element-plus/icons-vue';
import { ElMessage } from 'element-plus';
import { useI18n } from 'vue-i18n';
import type { FourRegionLayout, UiPreferences } from '@/platform/desktop';
import { cloneUiPreferences } from '@/modules/application-shell/adapters';
import { useApplicationShellStore } from '@/stores/application-shell';
import LayoutEditorDialog from './LayoutEditorDialog.vue';

const defaultUi = (): UiPreferences => ({
  theme: 'system',
  accentColor: '#FB7299',
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
    if (value) draft.value = cloneUiPreferences(value);
  },
  { immediate: true }
);

async function preview(): Promise<void> {
  if (!shell.savedUi) return;
  try {
    await shell.previewPreferences(cloneUiPreferences(draft.value));
  } catch {
    if (shell.savedUi) draft.value = cloneUiPreferences(shell.savedUi);
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
    await shell.saveUi(cloneUiPreferences(draft.value));
    ElMessage.success(t('settings.feedback.saved'));
  } catch {
    if (shell.savedUi) draft.value = cloneUiPreferences(shell.savedUi);
    ElMessage.error(t('settings.feedback.operationFailed'));
  }
}

async function reset(): Promise<void> {
  try {
    const value = await shell.resetUi();
    draft.value = cloneUiPreferences(value);
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

    <div class="app-hairline my-5 border-t" />
    <h3 class="app-text m-0 mb-4 text-sm font-semibold">
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
      <button
        v-if="draft.layoutMode === 'custom'"
        type="button"
        class="app-chip ml-3 disabled:cursor-not-allowed disabled:opacity-50"
        :disabled="!shell.savedUi || isPending"
        @click="layoutVisible = true"
      >
        {{ t('settings.interface.editLayout') }}
      </button>
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

    <div class="app-hairline mt-5 flex flex-wrap items-center gap-2 border-t pt-5">
      <button
        type="button"
        class="app-btn-primary inline-flex h-9 items-center gap-1.5 px-4 text-sm disabled:cursor-not-allowed disabled:opacity-60"
        :aria-busy="shell.pendingOperations.has('saveUi')"
        :disabled="!shell.savedUi || isPending"
        @click="save"
      >
        <el-icon v-if="shell.pendingOperations.has('saveUi')" class="is-loading">
          <Loading />
        </el-icon>
        {{ t('common.save') }}
      </button>
      <button
        type="button"
        class="app-chip disabled:cursor-not-allowed disabled:opacity-50"
        :aria-busy="shell.pendingOperations.has('resetUi')"
        :disabled="!shell.savedUi || isPending"
        @click="reset"
      >
        <el-icon v-if="shell.pendingOperations.has('resetUi')" class="is-loading">
          <Loading />
        </el-icon>
        {{ t('settings.interface.restoreDefaults') }}
      </button>
    </div>
  </el-form>

  <LayoutEditorDialog
    v-model:visible="layoutVisible"
    :model-value="draft.customLayout ?? defaultLayout()"
    @save="saveLayout"
  />
</template>
