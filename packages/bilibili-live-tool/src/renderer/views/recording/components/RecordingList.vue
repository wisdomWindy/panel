<script setup lang="ts">
import { VideoPlay, FolderOpened, Delete, Download } from '@element-plus/icons-vue';
import type { Recording } from '../../../modules/recording/types.js';
import { useRecordingStore } from '../../../stores/recording';
import { formatDuration, formatFileSize, formatDate } from '../../../utils/format.js';
import { getRecordingStatusTagType, getRecordingStatusText, confirmDelete, showSuccess, showError } from '../../../utils/ui.js';

defineProps<{
  recordings: Recording[];
  loading?: boolean;
}>();

const recordingStore = useRecordingStore();

async function handlePlay(recording: Recording) {
  await recordingStore.openRecordingFolder(recording.id);
}

async function handleOpenFolder(recording: Recording) {
  await recordingStore.openRecordingFolder(recording.id);
}

async function handleDelete(recording: Recording) {
  const confirmed = await confirmDelete(
    `确定要删除录制文件 "${recording.fileName}" 吗？此操作不可恢复。`
  );
  if (!confirmed) return;

  try {
    await recordingStore.deleteRecording(recording.id);
    showSuccess('删除成功');
  } catch {
    showError('删除失败');
  }
}

function handleDownload(recording: Recording) {
  const link = document.createElement('a');
  link.href = recording.filePath;
  link.download = recording.fileName;
  link.click();
}
</script>

<template>
  <div class="w-full">
    <el-table
      :data="recordings"
      :loading="loading"
      stripe
      class="w-full"
    >
      <el-table-column
        prop="fileName"
        label="文件名"
        min-width="200"
        show-overflow-tooltip
      />

      <el-table-column label="时长" width="100" align="center">
        <template #default="{ row }">
          {{ formatDuration(row.duration) }}
        </template>
      </el-table-column>

      <el-table-column label="大小" width="100" align="center">
        <template #default="{ row }">
          {{ formatFileSize(row.fileSize) }}
        </template>
      </el-table-column>

      <el-table-column label="录制时间" width="180" align="center">
        <template #default="{ row }">
          {{ formatDate(row.startedAt) }}
        </template>
      </el-table-column>

      <el-table-column label="状态" width="100" align="center">
        <template #default="{ row }">
          <el-tag :type="getRecordingStatusTagType(row.status)">
            {{ getRecordingStatusText(row.status) }}
          </el-tag>
        </template>
      </el-table-column>

      <el-table-column
        label="操作"
        width="200"
        align="center"
        fixed="right"
      >
        <template #default="{ row }">
          <el-button
            type="primary"
            :icon="VideoPlay"
            size="small"
            @click="handlePlay(row)"
          />
          <el-button
            type="info"
            :icon="FolderOpened"
            size="small"
            @click="handleOpenFolder(row)"
          />
          <el-button
            type="success"
            :icon="Download"
            size="small"
            @click="handleDownload(row)"
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

    <el-empty v-if="!loading && recordings.length === 0" description="暂无录制文件" />
  </div>
</template>
