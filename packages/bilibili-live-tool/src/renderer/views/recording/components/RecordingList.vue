<script setup lang="ts">
import { useI18n } from 'vue-i18n';
import { Delete, Edit, VideoPlay } from '@element-plus/icons-vue';
import type { RecordingId, RecordingSummary } from '@/platform/desktop';
import { formatBytes, formatClockTime, formatDuration } from '@/modules/media/formatters';

defineProps<{
    recordings: readonly RecordingSummary[];
    loading?: boolean;
    pendingOperations: ReadonlySet<string>;
  }>();

const emit = defineEmits<{
    preview: [recording: RecordingSummary];
    rename: [recording: RecordingSummary];
    delete: [recording: RecordingSummary];
  }>();
const { t } = useI18n();

function operationPending(
  action: string,
  id: RecordingId,
  pending: ReadonlySet<string>
): boolean {
  return pending.has(`${action}:${id}`);
}

function statusText(status: RecordingSummary['status']): string {
  return t(`recording.${status}`);
}
</script>

<template>
  <div class="min-h-64 w-full overflow-x-auto">
    <el-table :data="recordings" :loading="loading" class="min-w-[780px] w-full">
      <el-table-column
        prop="fileName"
        :label="t('recording.fileName')"
        min-width="220"
        show-overflow-tooltip
      />
      <el-table-column :label="t('recording.duration')" width="105" align="right">
        <template #default="{ row }">
          {{ formatDuration(row.durationSeconds) }}
        </template>
      </el-table-column>
      <el-table-column :label="t('recording.size')" width="105" align="right">
        <template #default="{ row }">
          {{ formatBytes(row.sizeBytes) }}
        </template>
      </el-table-column>
      <el-table-column :label="t('recording.recordingTime')" width="120">
        <template #default="{ row }">
          {{ formatClockTime(row.startedAt) }}
        </template>
      </el-table-column>
      <el-table-column :label="t('common.status')" width="100">
        <template #default="{ row }">
          <el-tag
            :type="
              row.status === 'completed'
                ? 'success'
                : row.status === 'failed'
                  ? 'danger'
                  : row.status === 'partial'
                    ? 'warning'
                    : 'primary'
            "
          >
            {{ statusText(row.status) }}
          </el-tag>
        </template>
      </el-table-column>
      <el-table-column :label="t('common.actions')" width="150" fixed="right">
        <template #default="{ row }">
          <div class="flex items-center gap-1">
            <el-tooltip :content="t('recording.preview')">
              <el-button
                :icon="VideoPlay"
                circle
                :loading="operationPending('previewRecording', row.id, pendingOperations)"
                :aria-label="t('recording.preview')"
                @click="emit('preview', row)"
              />
            </el-tooltip>
            <el-tooltip :content="t('recording.rename')">
              <el-button
                :icon="Edit"
                circle
                :aria-label="t('recording.rename')"
                @click="emit('rename', row)"
              />
            </el-tooltip>
            <el-tooltip :content="t('common.delete')">
              <el-button
                :icon="Delete"
                circle
                type="danger"
                plain
                :loading="operationPending('deleteRecording', row.id, pendingOperations)"
                :aria-label="t('common.delete')"
                @click="emit('delete', row)"
              />
            </el-tooltip>
          </div>
        </template>
      </el-table-column>
      <template #empty>
        <el-empty :description="t('recording.noRecordings')" />
      </template>
    </el-table>
  </div>
</template>
