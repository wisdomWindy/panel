<script setup lang="ts">
import { reactive } from 'vue';
import { useConfigStore } from '../../../stores/config';

const configStore = useConfigStore();

const form = reactive({
  enableAutoSend: configStore.config.enableAutoSend,
  autoSendInterval: configStore.config.autoSendInterval,
  enableTriggerReply: configStore.config.enableTriggerReply,
  enableAutoRecord: configStore.config.enableAutoRecord
});

function getValues() {
  return { ...form };
}

defineExpose({ getValues });
</script>

<template>
  <el-form :model="form" label-width="150px">
    <el-form-item label="启用自动发送">
      <el-switch v-model="form.enableAutoSend" />
    </el-form-item>
    <el-form-item label="默认间隔">
      <el-input-number
        v-model="form.autoSendInterval"
        :min="1000"
        :max="300000"
        :step="1000"
      />
      <span class="ml-2">毫秒</span>
    </el-form-item>
    <el-form-item label="触发回复">
      <el-switch v-model="form.enableTriggerReply" />
    </el-form-item>
    <el-divider content-position="left">
      自动录制
    </el-divider>
    <el-form-item label="启用自动录制">
      <el-switch v-model="form.enableAutoRecord" />
    </el-form-item>
    <el-form-item>
      <el-text type="info" size="small">
        连接直播间且开播时自动开始录制，下播时自动停止录制
      </el-text>
    </el-form-item>
  </el-form>
</template>
