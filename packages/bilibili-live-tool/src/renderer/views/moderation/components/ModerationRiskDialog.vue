<script setup lang="ts">
import { ref, watch } from 'vue';

const props = defineProps<{
    visible: boolean;
    pending: boolean;
  }>();

const emit = defineEmits<{
    'update:visible': [value: boolean];
    confirm: [];
  }>();

const accepted = ref(false);
watch(
  () => props.visible,
  value => {
    if (value) accepted.value = false;
  }
);
</script>

<template>
  <el-dialog
    :model-value="visible"
    title="自动禁言风险确认"
    width="min(560px, 94vw)"
    :close-on-click-modal="!pending"
    @update:model-value="emit('update:visible', $event)"
  >
    <ol class="m-0 space-y-3 pl-5 text-sm leading-6 text-gray-700 dark:text-gray-200">
      <li>本功能依赖本地账号 Cookie 能力，非 B 站开放平台标准 API。</li>
      <li>不承诺可用性，接口变动随时下线。</li>
      <li>使用本功能可能导致账号被平台风控。</li>
      <li>请确保你已了解并自行承担相关风险。</li>
    </ol>
    <el-checkbox v-model="accepted" class="mt-5">
      我已阅读并同意自行承担风险
    </el-checkbox>
    <template #footer>
      <el-button :disabled="pending" @click="emit('update:visible', false)">
        取消
      </el-button>
      <el-button
        type="danger"
        :loading="pending"
        :disabled="!accepted"
        @click="emit('confirm')"
      >
        确认启用
      </el-button>
    </template>
  </el-dialog>
</template>
