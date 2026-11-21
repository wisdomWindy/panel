<script setup lang="ts">
import { useI18n } from 'vue-i18n';
import { Loading } from '@element-plus/icons-vue';
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

const { t } = useI18n();
const headerCellStyle = {
  background: 'transparent',
  color: 'var(--app-text-3)',
  fontWeight: '500'
};

function statusStyle(status: MemberModule['status']): Record<string, string> {
  if (status === 'installed')
    return {
      background: 'color-mix(in srgb, var(--app-green) 20%, transparent)',
      color: 'var(--app-green)'
    };
  if (status === 'failed')
    return {
      background: 'color-mix(in srgb, var(--app-red) 20%, transparent)',
      color: 'var(--app-red)'
    };
  return { background: 'var(--app-panel-2)', color: 'var(--app-text-3)' };
}
</script>

<template>
  <el-dialog
    :model-value="visible"
    :title="t('membership.modules.title')"
    width="min(680px, calc(100vw - 32px))"
    @update:model-value="$emit('update:visible', $event)"
  >
    <div class="app-card overflow-hidden">
      <el-table
        v-if="modules.length"
        :data="modules"
        class="w-full"
        :header-cell-style="headerCellStyle"
      >
        <el-table-column prop="name" :label="t('membership.modules.name')" min-width="160" />
        <el-table-column prop="version" :label="t('membership.modules.version')" width="110" />
        <el-table-column :label="t('common.status')" width="130">
          <template #default="scope">
            <span class="app-badge" :style="statusStyle(scope.row.status)">
              {{ t(`membership.moduleStatus.${scope.row.status}`) }}
            </span>
          </template>
        </el-table-column>
        <el-table-column :label="t('membership.modules.progress')" min-width="150">
          <template #default="scope">
            <el-progress :percentage="scope.row.progressPercent" />
          </template>
        </el-table-column>
        <el-table-column :label="t('common.actions')" width="100" fixed="right">
          <template #default="scope">
            <button
              type="button"
              class="app-chip h-7 px-2.5 text-xs disabled:cursor-not-allowed disabled:opacity-50"
              style="color: var(--app-red); border-color: color-mix(in srgb, var(--app-red) 45%, transparent)"
              :disabled="pending.has(`cleanup:${scope.row.moduleId}`)"
              :aria-busy="pending.has(`cleanup:${scope.row.moduleId}`)"
              @click="$emit('cleanup', scope.row.moduleId)"
            >
              <el-icon
                v-if="pending.has(`cleanup:${scope.row.moduleId}`)"
                class="is-loading"
              >
                <Loading />
              </el-icon>
              {{ t('membership.modules.cleanup') }}
            </button>
          </template>
        </el-table-column>
      </el-table>
      <el-empty v-else :image-size="72" :description="t('common.noData')" />
    </div>
  </el-dialog>
</template>
