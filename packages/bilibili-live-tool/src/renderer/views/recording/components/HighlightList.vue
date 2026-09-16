<script setup lang="ts">
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

function operationPending(action: string, id: HighlightId, pending: ReadonlySet<string>): boolean {
  return pending.has(`${action}:${id}`);
}
</script>

<template>
  <div class="min-h-64 w-full overflow-x-auto">
    <el-table :data="highlights" :loading="loading" class="min-w-[680px] w-full">
      <el-table-column label="时间点" width="110">
        <template #default="{ row }">
          {{ formatDuration(row.positionSeconds) }}
        </template>
      </el-table-column>
      <el-table-column label="触发类型" width="120">
        <template #default="{ row }">
          <el-tag :type="row.triggerType === 'manual' ? 'primary' : 'warning'">
            {{ row.triggerType === 'manual' ? '手动标记' : '弹幕激增' }}
          </el-tag>
        </template>
      </el-table-column>
      <el-table-column label="片段时长" width="110">
        <template #default="{ row }">
          {{ formatDuration(row.clipDurationSeconds) }}
        </template>
      </el-table-column>
      <el-table-column label="标记时间" min-width="130">
        <template #default="{ row }">
          {{ formatClockTime(row.createdAt) }}
        </template>
      </el-table-column>
      <el-table-column label="操作" width="150" fixed="right">
        <template #default="{ row }">
          <div class="flex items-center gap-1">
            <el-tooltip :content="row.isClipAvailable ? '预览' : '片段未生成'">
              <el-button
                :icon="VideoPlay"
                circle
                :disabled="!row.isClipAvailable"
                :loading="operationPending('previewHighlight', row.id, pendingOperations)"
                aria-label="预览高光"
                @click="emit('preview', row)"
              />
            </el-tooltip>
            <el-tooltip :content="row.isClipAvailable ? '导出' : '片段未生成'">
              <el-button
                :icon="Download"
                circle
                :disabled="!row.isClipAvailable"
                :loading="operationPending('exportHighlight', row.id, pendingOperations)"
                aria-label="导出高光"
                @click="emit('export', row)"
              />
            </el-tooltip>
            <el-tooltip content="删除">
              <el-button
                :icon="Delete"
                circle
                type="danger"
                plain
                :loading="operationPending('deleteHighlight', row.id, pendingOperations)"
                aria-label="删除高光"
                @click="emit('delete', row)"
              />
            </el-tooltip>
          </div>
        </template>
      </el-table-column>
      <template #empty>
        <el-empty description="暂无高光标记" />
      </template>
    </el-table>
  </div>
</template>
