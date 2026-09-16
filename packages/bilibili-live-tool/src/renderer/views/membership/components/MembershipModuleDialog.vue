<script setup lang="ts">
import type { MemberModule, ModuleId } from '@/platform/desktop';

defineProps<{
  visible: boolean;
  modules: readonly MemberModule[];
  pending: ReadonlySet<string>;
}>();
defineEmits<{
  'update:visible': [value: boolean];
  cleanup: [moduleId: ModuleId];
}>();
</script>

<template>
  <el-dialog
    :model-value="visible"
    title="管理模块"
    width="min(680px, calc(100vw - 32px))"
    @update:model-value="$emit('update:visible', $event)"
  >
    <el-table v-if="modules.length" :data="modules" class="w-full">
      <el-table-column prop="name" label="模块" min-width="160" />
      <el-table-column prop="version" label="版本" width="110" />
      <el-table-column label="状态" width="130">
        <template #default="scope">
          <el-tag :type="scope.row.status === 'installed' ? 'success' : scope.row.status === 'failed' ? 'danger' : 'info'">
            {{ scope.row.status }}
          </el-tag>
        </template>
      </el-table-column>
      <el-table-column label="进度" min-width="150">
        <template #default="scope">
          <el-progress :percentage="scope.row.progressPercent" />
        </template>
      </el-table-column>
      <el-table-column label="操作" width="90" fixed="right">
        <template #default="scope">
          <el-button
            link
            type="danger"
            :loading="pending.has(`cleanup:${scope.row.moduleId}`)"
            @click="$emit('cleanup', scope.row.moduleId)"
          >
            清理
          </el-button>
        </template>
      </el-table-column>
    </el-table>
  </el-dialog>
</template>
