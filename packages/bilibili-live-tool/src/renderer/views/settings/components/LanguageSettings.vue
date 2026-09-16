<script setup lang="ts">
import { computed, ref, watch } from 'vue';
import { ElMessage } from 'element-plus';
import { useI18n } from 'vue-i18n';
import type { LocaleCode } from '@/platform/desktop';
import { setAppLocale } from '@/i18n-runtime';
import { useApplicationShellStore } from '@/stores/application-shell';

const shell = useApplicationShellStore();
const { t, locale } = useI18n();
const draftLocale = ref<LocaleCode>('zh-CN');
const followSystem = ref(true);
const isPending = computed(() => shell.pendingOperations.has('locale'));

watch(
  () => shell.locale,
  value => {
    if (!value) return;
    draftLocale.value = value.locale;
    followSystem.value = value.followSystem;
    previewLocale(value.locale);
  },
  { immediate: true }
);

function previewLocale(value: LocaleCode): void {
  locale.value = value;
  setAppLocale(value);
}

async function changeAuto(value: boolean): Promise<void> {
  try {
    const saved = await shell.setLocaleAuto(value);
    followSystem.value = saved.followSystem;
    draftLocale.value = saved.locale;
    previewLocale(saved.locale);
  } catch {
    followSystem.value = shell.locale?.followSystem ?? true;
    ElMessage.error(t('settings.feedback.operationFailed'));
  }
}

async function save(): Promise<void> {
  try {
    await shell.saveLocale({ locale: draftLocale.value, followSystem: followSystem.value });
    ElMessage.success(t('settings.feedback.saved'));
  } catch {
    if (shell.locale) {
      draftLocale.value = shell.locale.locale;
      previewLocale(shell.locale.locale);
    }
    ElMessage.error(t('settings.feedback.operationFailed'));
  }
}
</script>

<template>
  <el-form label-position="top" class="max-w-xl">
    <el-form-item :label="t('settings.locale.followSystem')">
      <el-switch
        v-model="followSystem"
        :disabled="!shell.locale || isPending"
        @change="changeAuto(Boolean($event))"
      />
    </el-form-item>
    <el-form-item :label="t('settings.locale.interfaceLanguage')">
      <el-select
        v-model="draftLocale"
        class="!w-64"
        :disabled="!shell.locale || followSystem || isPending"
        @change="previewLocale($event as LocaleCode)"
      >
        <el-option label="简体中文" value="zh-CN" />
        <el-option label="English" value="en-US" />
      </el-select>
    </el-form-item>
    <div class="mt-6 border-t border-gray-200 pt-5 dark:border-gray-700">
      <el-button
        type="primary"
        :loading="isPending"
        :disabled="!shell.locale"
        @click="save"
      >
        {{ t('common.save') }}
      </el-button>
    </div>
  </el-form>
</template>
