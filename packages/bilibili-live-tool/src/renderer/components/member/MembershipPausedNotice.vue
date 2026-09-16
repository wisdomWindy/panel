<script setup lang="ts">
import { WarningFilled } from '@element-plus/icons-vue';

defineProps<{
    reason: 'expired' | 'missingModule' | 'unavailable';
  }>();

defineEmits<{
    renew: [];
  }>();
</script>

<template>
  <section
    class="flex min-h-72 flex-col items-center justify-center border-y border-gray-200 bg-white px-6 text-center dark:border-gray-700 dark:bg-gray-900"
  >
    <el-icon :size="36" class="text-amber-500">
      <WarningFilled />
    </el-icon>
    <h3 class="mb-2 mt-4 text-lg font-semibold">
      {{
        reason === 'expired'
          ? '会员功能已暂停'
          : reason === 'missingModule'
            ? '会员模块未下载'
            : '会员服务暂不可用'
      }}
    </h3>
    <p v-if="reason === 'expired'" class="mb-5 mt-0 max-w-xl text-sm text-gray-500">
      已下载模块保留，续费后立即恢复。
    </p>
    <el-button v-if="reason === 'expired'" type="primary" @click="$emit('renew')">
      续费会员
    </el-button>
    <el-button v-else type="primary" @click="$emit('renew')">
      前往会员中心
    </el-button>
  </section>
</template>
