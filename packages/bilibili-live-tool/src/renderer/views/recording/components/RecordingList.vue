<script setup lang="ts">
import { useI18n } from 'vue-i18n';
import { Delete, Edit, Loading, VideoPlay } from '@element-plus/icons-vue';
import type { RecordingId, RecordingSummary } from '@/platform/desktop';
import { formatBytes, formatClockTime, formatDuration } from '@/modules/media/formatters';

const props = defineProps<{
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

const headerCellStyle = {
  background: 'transparent',
  color: 'var(--app-text-3)',
  fontWeight: '500'
};

function isPending(action: string, id: RecordingId): boolean {
  return props.pendingOperations.has(`${action}:${id}`);
}

function statusText(status: RecordingSummary['status']): string {
  return t(`recording.${status}`);
}

function statusBadgeClass(status: RecordingSummary['status']): string {
  if (status === 'recording') return 'app-badge--admin';
  if (status === 'partial') return 'app-badge--captain';
  return 'app-badge--plain';
}

function statusBadgeStyle(status: RecordingSummary['status']): Record<string, string> | undefined {
  if (status === 'completed')
    return {
      background: 'color-mix(in srgb, var(--app-green) 22%, transparent)',
      color: 'var(--app-green)'
    };
  if (status === 'failed')
    return {
      background: 'color-mix(in srgb, var(--app-red) 22%, transparent)',
      color: 'var(--app-red)'
    };
  return undefined;
}
</script>

<template>
  <div class="min-h-64 w-full overflow-x-auto">
    <el-table
      :data="recordings"
      :loading="loading"
      :header-cell-style="headerCellStyle"
      class="w-full min-w-[820px]"
    >
      <el-table-column
        prop="fileName"
        :label="t('recording.fileName')"
        min-width="220"
        show-overflow-tooltip
      />
      <el-table-column :label="t('recording.duration')" width="100" align="right">
        <template #default="{ row }">
          <span class="app-muted tabular-nums">{{ formatDuration(row.durationSeconds) }}</span>
        </template>
      </el-table-column>
      <el-table-column :label="t('recording.size')" width="110" align="right">
        <template #default="{ row }">
          <span class="app-muted tabular-nums">{{ formatBytes(row.sizeBytes) }}</span>
        </template>
      </el-table-column>
      <el-table-column :label="t('recording.recordingTime')" width="120">
        <template #default="{ row }">
          <span class="app-faint tabular-nums">{{ formatClockTime(row.startedAt) }}</span>
        </template>
      </el-table-column>
      <el-table-column :label="t('common.status')" width="110">
        <template #default="{ row }">
          <span
            class="app-badge"
            :class="statusBadgeClass(row.status)"
            :style="statusBadgeStyle(row.status)"
          >
            {{ statusText(row.status) }}
          </span>
        </template>
      </el-table-column>
      <el-table-column :label="t('common.actions')" width="250" fixed="right">
        <template #default="{ row }">
          <div class="flex flex-wrap items-center gap-1">
            <el-tooltip :content="row.status === 'recording' ? t('recording.previewUnavailable') : t('recording.preview')">
              <button
                type="button"
                class="app-chip h-7 px-2.5 text-xs disabled:cursor-not-allowed disabled:opacity-50"
                :aria-label="t('recording.preview')"
                :aria-busy="isPending('previewRecording', row.id)"
                :disabled="isPending('previewRecording', row.id) || row.status === 'recording'"
                @click="emit('preview', row)"
              >
                <el-icon v-if="isPending('previewRecording', row.id)" class="is-loading">
                  <Loading />
                </el-icon>
                <el-icon v-else><VideoPlay /></el-icon>
                {{ t('recording.preview') }}
              </button>
            </el-tooltip>
            <el-tooltip :content="t('recording.rename')">
              <button
                type="button"
                class="app-chip h-7 px-2.5 text-xs"
                :aria-label="t('recording.rename')"
                @click="emit('rename', row)"
              >
                <el-icon><Edit /></el-icon>
                {{ t('recording.rename') }}
              </button>
            </el-tooltip>
            <el-tooltip :content="t('common.delete')">
              <button
                type="button"
                class="app-chip h-7 px-2.5 text-xs disabled:cursor-not-allowed disabled:opacity-50"
                style="color: var(--app-red); border-color: color-mix(in srgb, var(--app-red) 45%, transparent)"
                :aria-label="t('common.delete')"
                :aria-busy="isPending('deleteRecording', row.id)"
                :disabled="isPending('deleteRecording', row.id)"
                @click="emit('delete', row)"
              >
                <el-icon v-if="isPending('deleteRecording', row.id)" class="is-loading">
                  <Loading />
                </el-icon>
                <el-icon v-else><Delete /></el-icon>
                {{ t('common.delete') }}
              </button>
            </el-tooltip>
          </div>
        </template>
      </el-table-column>
      <template #empty>
        <p class="app-faint m-0 py-10 text-center text-sm">{{ t('recording.noRecordings') }}</p>
      </template>
    </el-table>
  </div>
</template>
