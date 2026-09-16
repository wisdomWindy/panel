<script setup lang="ts">
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

function operationPending(action: string, id: RecordingId, pending: ReadonlySet<string>): boolean {
  return pending.has(`${action}:${id}`);
}

function statusText(status: RecordingSummary['status']): string {
  return { recording: '录制中', completed: '已完成', partial: '部分文件', failed: '失败' }[status];
}
</script>

<template>
  <div class="min-h-64 w-full overflow-x-auto">
    <el-table :data="recordings" :loading="loading" class="min-w-[780px] w-full">
      <el-table-column
        prop="fileName"
        label="文件名"
        min-width="220"
        show-overflow-tooltip
      />
      <el-table-column label="时长" width="105" align="right">
        <template #default="{ row }">
          {{ formatDuration(row.durationSeconds) }}
        </template>
      </el-table-column>
      <el-table-column label="大小" width="105" align="right">
        <template #default="{ row }">
          {{ formatBytes(row.sizeBytes) }}
        </template>
      </el-table-column>
      <el-table-column label="录制时间" width="120">
        <template #default="{ row }">
          {{ formatClockTime(row.startedAt) }}
        </template>
      </el-table-column>
      <el-table-column label="状态" width="100">
        <template #default="{ row }">
          <el-tag :type="row.status === 'completed' ? 'success' : row.status === 'failed' ? 'danger' : row.status === 'partial' ? 'warning' : 'primary'">
            {{ statusText(row.status) }}
          </el-tag>
        </template>
      </el-table-column>
      <el-table-column label="操作" width="150" fixed="right">
        <template #default="{ row }">
          <div class="flex items-center gap-1">
            <el-tooltip content="预览">
              <el-button
                :icon="VideoPlay"
                circle
                :loading="operationPending('previewRecording', row.id, pendingOperations)"
                aria-label="预览录制"
                @click="emit('preview', row)"
              />
            </el-tooltip>
            <el-tooltip content="重命名">
              <el-button
                :icon="Edit"
                circle
                aria-label="重命名录制"
                @click="emit('rename', row)"
              />
            </el-tooltip>
            <el-tooltip content="删除">
              <el-button
                :icon="Delete"
                circle
                type="danger"
                plain
                :loading="operationPending('deleteRecording', row.id, pendingOperations)"
                aria-label="删除录制"
                @click="emit('delete', row)"
              />
            </el-tooltip>
          </div>
        </template>
      </el-table-column>
      <template #empty>
        <el-empty description="暂无录制文件" />
      </template>
    </el-table>
  </div>
</template>
