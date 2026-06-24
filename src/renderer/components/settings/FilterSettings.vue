<script setup lang="ts">
import { reactive } from 'vue';
import { useConfigStore } from '../../stores/config';

const configStore = useConfigStore();

const form = reactive({
  enableSensitiveWordFilter: configStore.config.enableSensitiveWordFilter,
  replaceChar: configStore.config.replaceChar,
  sensitiveWords: configStore.config.sensitiveWords.join('\n')
});

function getValues() {
  return {
    enableSensitiveWordFilter: form.enableSensitiveWordFilter,
    replaceChar: form.replaceChar,
    sensitiveWords: form.sensitiveWords.split('\n').filter((word: string) => word.trim())
  };
}

defineExpose({ getValues });
</script>

<template>
  <el-form :model="form" label-width="150px">
    <el-form-item label="敏感词过滤">
      <el-switch v-model="form.enableSensitiveWordFilter" />
    </el-form-item>
    <el-form-item label="替换字符">
      <el-input v-model="form.replaceChar" placeholder="替换敏感词的字符" />
    </el-form-item>
    <el-form-item label="敏感词列表">
      <el-input
        v-model="form.sensitiveWords"
        type="textarea"
        :rows="5"
        placeholder="每行一个敏感词"
      />
    </el-form-item>
  </el-form>
</template>
