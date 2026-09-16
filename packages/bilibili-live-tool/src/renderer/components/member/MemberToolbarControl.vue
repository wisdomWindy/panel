<script setup lang="ts">
import { computed } from 'vue';
import { User } from '@element-plus/icons-vue';
import { useRouter } from 'vue-router';
import { useMembershipStore } from '@/stores/membership';

const router = useRouter();
const store = useMembershipStore();
const label = computed(() => {
  if (store.status.shellState === 'active') {
    const expiry = store.status.entitlement?.expiresAt;
    return expiry ? new Intl.DateTimeFormat(undefined, { dateStyle: 'short' }).format(expiry) : '会员中心';
  }
  if (store.status.shellState === 'expired') return '续费';
  if (store.status.shellState === 'unpaid') return '开通会员';
  return '登录';
});
</script>

<template>
  <el-button :icon="User" @click="router.push('/membership')">
    {{ label }}
  </el-button>
</template>
