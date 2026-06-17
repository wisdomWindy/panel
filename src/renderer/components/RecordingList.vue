<script setup lang="ts">
import { VideoPlay, FolderOpened, Delete, Download } from '@element-plus/icons-vue';
import { ElMessageBox, ElMessage } from 'element-plus';
import type { Recording } from '../modules/recording/types.js';
import { useRecordingStore } from '../stores/recording';

defineProps<{
  recordings: Recording[];
  loading?: boolean;
}>();

const recordingStore = useRecordingStore();

const statusTagType = (status: string) => {
  switch (status) {
    case 'recording':
      return 'danger';
    case 'paused':
      return 'warning';
    case 'completed':
      return 'success';
    case 'failed':
      return 'info';
    default:
      return '';
  }
};

const statusText = (status: string) => {
  switch (status) {
    case 'recording':
      return '录制中';
    case 'paused':
      return '已暂停';
    case 'completed':
      return '已完成';
    case 'failed':
      return '失败';
    default:
      return status;
  }
};

const formatDuration = (seconds: number) => {
  const hours = Math.floor(seconds / 3600);
  const minutes = Math.floor((seconds % 3600) / 60);
  const secs = seconds % 60;
  return `${hours.toString().padStart(2, '0')}:${minutes.toString().padStart(2, '0')}:${secs.toString().padStart(2, '0')}`;
};

const formatFileSize = (bytes: number) => {
  if (bytes === 0) return '0 B';
  const units = ['B', 'KB', 'MB', 'GB', 'TB'];
  const k = 1024;
  const i = Math.floor(Math.log(bytes) / Math.log(k));
  return parseFloat((bytes / Math.pow(k, i)).toFixed(2)) + ' ' + units[i];
};

const formatDate = (date: Date | string) => {
  const d = new Date(date);
  return d.toLocaleString('zh-CN');
};

async function handlePlay(recording: Recording) {
  // 使用系统默认播放器打开文件
  await recordingStore.openRecordingFolder(recording.id);
}

async function handleOpenFolder(recording: Recording) {
  await recordingStore.openRecordingFolder(recording.id);
}

async function handleDelete(recording: Recording) {
  try {
    await ElMessageBox.confirm(
      `确定要删除录制文件 "${recording.fileName}" 吗？此操作不可恢复。`,
      '确认删除',
      {
        confirmButtonText: '删除',
        cancelButtonText: '取消',
        type: 'warning'
      }
    );

    await recordingStore.deleteRecording(recording.id);
    ElMessage.success('删除成功');
  } catch (error) {
    if (error !== 'cancel') {
      ElMessage.error('删除失败');
    }
  }
}

function handleDownload(recording: Recording) {
  // 创建下载链接
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

      <el-table-column
        label="时长"
        width="100"
        align="center"
      >
        <template #default="{ row }">
          {{ formatDuration(row.duration) }}
        </template>
      </el-table-column>

      <el-table-column
        label="大小"
        width="100"
        align="center"
      >
        <template #default="{ row }">
          {{ formatFileSize(row.fileSize) }}
        </template>
      </el-table-column>

      <el-table-column
        label="录制时间"
        width="180"
        align="center"
      >
        <template #default="{ row }">
          {{ formatDate(row.startedAt) }}
        </template>
      </el-table-column>

      <el-table-column
        label="状态"
        width="100"
        align="center"
      >
        <template #default="{ row }">
          <el-tag :type="statusTagType(row.status)">
            {{ statusText(row.status) }}
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

    <!-- 空状态 -->
    <el-empty
      v-if="!loading && recordings.length === 0"
      description="暂无录制文件"
    />
  </div>
</template>
