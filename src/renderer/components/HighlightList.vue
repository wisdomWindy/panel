<script setup lang="ts">
import { VideoPlay, Delete } from '@element-plus/icons-vue';
import type { Highlight } from '../modules/recording/types.js';
import { useRecordingStore } from '../stores/recording';
import { formatTimestamp, formatDate } from '../utils/format.js';
import { getHighlightReasonTagType, getHighlightReasonText, confirmDelete, showSuccess, showError, showWarning } from '../utils/ui.js';

defineProps<{
  highlights: Highlight[];
  loading?: boolean;
}>();

const recordingStore = useRecordingStore();

async function handlePlay(highlight: Highlight) {
  if (highlight.clipPath) {
    await recordingStore.openRecordingFolder(highlight.recordingId);
  } else {
    showWarning('该高光片段未生成独立文件');
  }
}

async function handleDelete(highlight: Highlight) {
  const confirmed = await confirmDelete('确定要删除该高光标记吗？');
  if (!confirmed) return;

  try {
    await recordingStore.deleteHighlight(highlight.id);
    showSuccess('删除成功');
  } catch {
    showError('删除失败');
  }
}
</script>

<template>
  <div class="w-full">
    <el-table
      :data="highlights"
      :loading="loading"
      stripe
      class="w-full"
    >
      <el-table-column label="时间点" width="100" align="center">
        <template #default="{ row }">
          {{ formatTimestamp(row.timestamp) }}
        </template>
      </el-table-column>

      <el-table-column label="触发原因" width="120" align="center">
        <template #default="{ row }">
          <el-tag :type="getHighlightReasonTagType(row.reason)">
            {{ getHighlightReasonText(row.reason) }}
          </el-tag>
        </template>
      </el-table-column>

      <el-table-column
        prop="description"
        label="描述"
        min-width="200"
        show-overflow-tooltip
      />

      <el-table-column label="片段时长" width="100" align="center">
        <template #default="{ row }">
          {{ row.clipDuration }}秒
        </template>
      </el-table-column>

      <el-table-column label="标记时间" width="180" align="center">
        <template #default="{ row }">
          {{ formatDate(row.createdAt) }}
        </template>
      </el-table-column>

      <el-table-column
        label="操作"
        width="120"
        align="center"
        fixed="right"
      >
        <template #default="{ row }">
          <el-button
            type="primary"
            :icon="VideoPlay"
            size="small"
            :disabled="!row.clipPath"
            @click="handlePlay(row)"
          />
          <el-button
            type="danger"
            :icon="Delete"
            size="small"
            @click="handleDelete(row)"
          />
        </template>
      </el-table-column>
    </el-table>

    <el-empty v-if="!loading && highlights.length === 0" description="暂无高光标记" />
  </div>
</template>
