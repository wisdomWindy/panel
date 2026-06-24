<script setup lang="ts">
import { reactive } from 'vue';
import { useConfigStore } from '../../../stores/config';

const configStore = useConfigStore();

const form = reactive({
  autoReconnect: configStore.config.autoReconnect,
  reconnectInterval: configStore.config.reconnectInterval,
  maxReconnectAttempts: configStore.config.maxReconnectAttempts
});

function getValues() {
  return {
    autoReconnect: form.autoReconnect,
    reconnectInterval: form.reconnectInterval,
    maxReconnectAttempts: form.maxReconnectAttempts
  };
}

defineExpose({ getValues });
</script>

<template>
  <el-form :model="form" label-width="150px">
    <el-form-item label="自动重连">
      <el-switch v-model="form.autoReconnect" />
    </el-form-item>
    <el-form-item label="重连间隔">
      <el-input-number
        v-model="form.reconnectInterval"
        :min="1000"
        :max="60000"
        :step="1000"
      />
      <span class="ml-2">毫秒</span>
    </el-form-item>
    <el-form-item label="最大重连次数">
      <el-input-number
        v-model="form.maxReconnectAttempts"
        :min="1"
        :max="20"
      />
    </el-form-item>
  </el-form>
</template>
