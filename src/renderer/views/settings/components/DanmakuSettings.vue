<script setup lang="ts">
import { reactive } from 'vue';
import { useConfigStore } from '../../stores/config';

const configStore = useConfigStore();

const form = reactive({
  minSendInterval: configStore.config.minSendInterval,
  enableGiftThank: configStore.config.enableGiftThank,
  giftThankMessage: '感谢 {username} 送的 {giftName}!'
});

function getValues() {
  return {
    minSendInterval: form.minSendInterval,
    enableGiftThank: form.enableGiftThank
  };
}

defineExpose({ getValues });
</script>

<template>
  <el-form :model="form" label-width="150px">
    <el-form-item label="最小发送间隔">
      <el-input-number
        v-model="form.minSendInterval"
        :min="1000"
        :max="60000"
        :step="1000"
      />
      <span class="ml-2">毫秒</span>
    </el-form-item>
    <el-form-item label="感谢礼物">
      <el-switch v-model="form.enableGiftThank" />
    </el-form-item>
    <el-form-item label="感谢模板">
      <el-input
        v-model="form.giftThankMessage"
        type="textarea"
        :rows="3"
        placeholder="感谢礼物模板，可使用 {username} 和 {giftName} 变量"
      />
    </el-form-item>
  </el-form>
</template>
