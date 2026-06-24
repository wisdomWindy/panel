<script setup lang="ts">
import { reactive } from 'vue';
import { useI18n } from 'vue-i18n';
import { useConfigStore } from '../../../stores/config';
import { setAppLocale } from '../../../i18n-runtime';

const { locale } = useI18n();
const configStore = useConfigStore();

const form = reactive({
  locale: configStore.config.language || 'zh-CN'
});

function handleLanguageChange(lang: string) {
  locale.value = lang;
  setAppLocale(lang as 'zh-CN' | 'en-US');
  configStore.updateConfig({ language: lang as 'zh-CN' | 'en-US' });
}

function getValues() {
  return { language: form.locale };
}

defineExpose({ getValues });
</script>

<template>
  <el-form :model="form" label-width="150px">
    <el-form-item label="界面语言">
      <el-select v-model="form.locale" @change="handleLanguageChange">
        <el-option label="中文" value="zh-CN" />
        <el-option label="English" value="en-US" />
      </el-select>
    </el-form-item>
    <el-form-item>
      <el-alert
        title="切换语言后界面将立即更新"
        type="info"
        show-icon
        :closable="false"
      />
    </el-form-item>
  </el-form>
</template>
