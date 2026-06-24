<script setup lang="ts">
import { reactive } from 'vue';
import { useRouter } from 'vue-router';
import { useConfigStore } from '../../../stores/config';

const router = useRouter();
const configStore = useConfigStore();

const form = reactive({
  checkUpdates: configStore.config.checkUpdates
});

function getValues() {
  return { checkUpdates: form.checkUpdates };
}

defineExpose({ getValues });
</script>

<template>
  <el-form :model="form" label-width="150px">
    <el-form-item label="自动检查更新">
      <el-switch v-model="form.checkUpdates" />
    </el-form-item>
    <el-form-item>
      <el-button type="primary" @click="router.push('/update-log')">
        查看更新日志
      </el-button>
    </el-form-item>
  </el-form>
</template>
