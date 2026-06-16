<script setup lang="ts">
import { Play, Delete } from '@element-plus/icons-vue';
import { ElMessageBox, ElMessage } from 'element-plus';
import type { Highlight, HighlightReason } from '../modules/recording/types.js';
import { useRecordingStore } from '../stores/recording';

defineProps<{
  highlights: Highlight[];
  loading?: boolean;
}>();

const recordingStore = useRecordingStore();

const reasonTagType = (reason: HighlightReason) => {
  switch (reason) {
    case 'danmaku_surge':
      return 'danger';
    case 'high_value_gift':
      return 'warning';
    case 'captain':
      return 'success';
    case 'manual':
      return 'info';
    default:
      return '';
  }
};

const reasonText = (reason: HighlightReason) => {
  switch (reason) {
    case 'danmaku_surge':
      return '弹幕激增';
    case 'high_value_gift':
      return '高价值礼物';
    case 'captain':
      return '舰长赠送';
    case 'manual':
      return '手动标记';
    default:
      return reason;
  }
};

const formatTimestamp = (seconds: number) => {
  const hours = Math.floor(seconds / 3600);
  const minutes = Math.floor((seconds % 3600) / 60);
  const secs = seconds % 60;

  if (hours > 0) {
    return `${hours}:${minutes.toString().padStart(2, '0')}:${secs.toString().padStart(2, '0')}`;
  }
  return `${minutes}:${secs.toString().padStart(2, '0')}`;
};

const formatDuration = (seconds: number) => {
  return `${seconds}秒`;
};

const formatDate = (date: Date | string) => {
  const d = new Date(date);
  return d.toLocaleString('zh-CN');
};

async function handlePlay(highlight: Highlight) {
  if (highlight.clipPath) {
    // 打开高光片段所在目录
    await recordingStore.openRecordingFolder(highlight.recordingId);
  } else {
    ElMessage.warning('该高光片段未生成独立文件');
  }
}

async function handleDelete(highlight: Highlight) {
  try {
    await ElMessageBox.confirm(
      '确定要删除该高光标记吗？',
      '确认删除',
      {
        confirmButtonText: '删除',
        cancelButtonText: '取消',
        type: 'warning'
      }
    );

    await recordingStore.deleteHighlight(highlight.id);
    ElMessage.success('删除成功');
  } catch (error) {
    if (error !== 'cancel') {
      ElMessage.error('删除失败');
    }
  }
}
</script>

<template>
  <div class="highlight-list">
    <el-table
      :data="highlights"
      :loading="loading"
      stripe
      style="width: 100%"
    >
      <el-table-column
        label="时间点"
        width="100"
        align="center"
      >
        <template #default="{ row }">
          {{ formatTimestamp(row.timestamp) }}
        </template>
      </el-table-column>

      <el-table-column
        label="触发原因"
        width="120"
        align="center"
      >
        <template #default="{ row }">
          <el-tag :type="reasonTagType(row.reason)">
            {{ reasonText(row.reason) }}
          </el-tag>
        </template>
      </el-table-column>

      <el-table-column
        prop="description"
        label="描述"
        min-width="200"
        show-overflow-tooltip
      />

      <el-table-column
        label="片段时长"
        width="100"
        align="center"
      >
        <template #default="{ row }">
          {{ formatDuration(row.clipDuration) }}
        </template>
      </el-table-column>

      <el-table-column
        label="标记时间"
        width="180"
        align="center"
      >
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
            :icon="Play"
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

    <!-- 空状态 -->
    <el-empty
      v-if="!loading && highlights.length === 0"
      description="暂无高光标记"
    />
  </div>
</template>

<style scoped>
.highlight-list {
  width: 100%;
}
</style>
