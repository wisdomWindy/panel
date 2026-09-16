<script setup lang="ts">
defineProps<{ pending: boolean; errorMessage: string | null }>();
defineEmits<{ retry: [] }>();
</script>

<template>
  <section aria-labelledby="membership-plans-title">
    <h3 id="membership-plans-title" class="mb-3 mt-0 text-base font-semibold">
      选择套餐
    </h3>
    <el-alert
      type="warning"
      :closable="false"
      :title="errorMessage ?? '会员服务暂不可用，请稍后重试'"
      show-icon
    >
      <el-button size="small" :loading="pending" @click="$emit('retry')">
        重试
      </el-button>
    </el-alert>
  </section>
</template>
