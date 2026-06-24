<script setup lang="ts">
import { useRouter } from 'vue-router';

const router = useRouter();
</script>

<template>
  <div>
    <el-alert
      title="插件管理"
      description="安装和管理第三方插件，扩展应用功能"
      type="info"
      show-icon
      :closable="false"
      class="mb-4"
    />
    <el-button type="primary" @click="router.push('/plugins')">
      打开插件管理
    </el-button>
  </div>
</template>
