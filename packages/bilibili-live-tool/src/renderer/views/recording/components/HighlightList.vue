<script setup lang="ts">
import { useI18n } from 'vue-i18n';
import { Delete, Download, VideoPlay } from '@element-plus/icons-vue';
import type { HighlightId, HighlightSummary } from '@/platform/desktop';
import { formatClockTime, formatDuration } from '@/modules/media/formatters';

defineProps<{
    highlights: readonly HighlightSummary[];
    loading?: boolean;
    pendingOperations: ReadonlySet<string>;
  }>();

const emit = defineEmits<{
    preview: [highlight: HighlightSummary];
    export: [highlight: HighlightSummary];
    delete: [highlight: HighlightSummary];
  }>();
const { t } = useI18n();

function operationPending(
  action: string,
  id: HighlightId,
  pending: ReadonlySet<string>
): boolean {
  return pending.has(`${action}:${id}`);
}
</script>

<template>
  <div class="min-h-64 w-full overflow-x-auto">
    <el-table :data="highlights" :loading="loading" class="min-w-[680px] w-full">
      <el-table-column :label="t('recording.timestamp')" width="110">
        <template #default="{ row }">
          {{ formatDuration(row.positionSeconds) }}
        </template>
      </el-table-column>
      <el-table-column :label="t('recording.triggerReason')" width="120">
        <template #default="{ row }">
          <el-tag :type="row.triggerType === 'manual' ? 'primary' : 'warning'">
            {{
              row.triggerType === 'manual' ? t('recording.manualMark') : t('recording.danmakuSurge')
            }}
          </el-tag>
        </template>
      </el-table-column>
      <el-table-column :label="t('recording.clipDuration')" width="110">
        <template #default="{ row }">
          {{ formatDuration(row.clipDurationSeconds) }}
        </template>
      </el-table-column>
      <el-table-column :label="t('recording.markTime')" min-width="130">
        <template #default="{ row }">
          {{ formatClockTime(row.createdAt) }}
        </template>
      </el-table-column>
      <el-table-column :label="t('common.actions')" width="150" fixed="right">
        <template #default="{ row }">
          <div class="flex items-center gap-1">
            <el-tooltip
              :content="
                row.isClipAvailable ? t('recording.preview') : t('recording.clipUnavailable')
              "
            >
              <el-button
                :icon="VideoPlay"
                circle
                :disabled="!row.isClipAvailable"
                :loading="operationPending('previewHighlight', row.id, pendingOperations)"
                :aria-label="t('recording.preview')"
                @click="emit('preview', row)"
              />
            </el-tooltip>
            <el-tooltip
              :content="row.isClipAvailable ? t('common.export') : t('recording.clipUnavailable')"
            >
              <el-button
                :icon="Download"
                circle
                :disabled="!row.isClipAvailable"
                :loading="operationPending('exportHighlight', row.id, pendingOperations)"
                :aria-label="t('common.export')"
                @click="emit('export', row)"
              />
            </el-tooltip>
            <el-tooltip :content="t('common.delete')">
              <el-button
                :icon="Delete"
                circle
                type="danger"
                plain
                :loading="operationPending('deleteHighlight', row.id, pendingOperations)"
                :aria-label="t('common.delete')"
                @click="emit('delete', row)"
              />
            </el-tooltip>
          </div>
        </template>
      </el-table-column>
      <template #empty>
        <el-empty :description="t('recording.noHighlights')" />
      </template>
    </el-table>
  </div>
</template>
