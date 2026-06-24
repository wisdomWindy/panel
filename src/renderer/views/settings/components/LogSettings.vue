<script setup lang="ts">
import { reactive } from 'vue';
import { useConfigStore } from '../../stores/config';

const configStore = useConfigStore();

const form = reactive({
  enableLog: configStore.config.enableLog,
  logLevel: configStore.config.logLevel,
  maxLogLines: configStore.config.maxLogLines
});

function getValues() {
  return { ...form };
}

defineExpose({ getValues });
</script>

<template>
  <el-form :model="form" label-width="150px">
    <el-form-item label="启用日志">
      <el-switch v-model="form.enableLog" />
    </el-form-item>
    <el-form-item label="日志级别">
      <el-select v-model="form.logLevel" width="150">
        <el-option label="Debug" value="debug" />
        <el-option label="Info" value="info" />
        <el-option label="Warning" value="warn" />
        <el-option label="Error" value="error" />
      </el-select>
    </el-form-item>
    <el-form-item label="最大日志行数">
      <el-input-number
        v-model="form.maxLogLines"
        :min="100"
        :max="10000"
        :step="100"
      />
    </el-form-item>
  </el-form>
</template>
