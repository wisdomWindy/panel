<script setup lang="ts">
import { useI18n } from 'vue-i18n';
import { Delete, Download, Loading, VideoPlay } from '@element-plus/icons-vue';
import type { HighlightId, HighlightSummary } from '@/platform/desktop';
import { formatClockTime, formatDuration } from '@/modules/media/formatters';

const props = defineProps<{
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

const headerCellStyle = {
  background: 'transparent',
  color: 'var(--app-text-3)',
  fontWeight: '500'
};

function isPending(action: string, id: HighlightId): boolean {
  return props.pendingOperations.has(`${action}:${id}`);
}

function triggerBadgeClass(triggerType: HighlightSummary['triggerType']): string {
  return triggerType === 'manual' ? 'app-badge--captain' : 'app-badge--vip';
}

function triggerText(triggerType: HighlightSummary['triggerType']): string {
  return triggerType === 'manual' ? t('recording.manualMark') : t('recording.danmakuSurge');
}
</script>

<template>
  <div class="min-h-64 w-full overflow-x-auto">
    <el-table
      :data="highlights"
      :loading="loading"
      :header-cell-style="headerCellStyle"
      class="w-full min-w-[780px]"
    >
      <el-table-column :label="t('recording.timestamp')" width="110">
        <template #default="{ row }">
          <span class="app-muted tabular-nums">{{ formatDuration(row.positionSeconds) }}</span>
        </template>
      </el-table-column>
      <el-table-column :label="t('recording.triggerReason')" width="140">
        <template #default="{ row }">
          <span class="app-badge" :class="triggerBadgeClass(row.triggerType)">
            {{ triggerText(row.triggerType) }}
          </span>
        </template>
      </el-table-column>
      <el-table-column :label="t('recording.clipDuration')" width="110">
        <template #default="{ row }">
          <span class="app-muted tabular-nums">{{ formatDuration(row.clipDurationSeconds) }}</span>
        </template>
      </el-table-column>
      <el-table-column :label="t('recording.markTime')" min-width="130">
        <template #default="{ row }">
          <span class="app-faint tabular-nums">{{ formatClockTime(row.createdAt) }}</span>
        </template>
      </el-table-column>
      <el-table-column :label="t('common.actions')" width="245" fixed="right">
        <template #default="{ row }">
          <div class="flex flex-wrap items-center gap-1">
            <el-tooltip
              :content="row.isClipAvailable ? t('recording.preview') : t('recording.clipUnavailable')"
            >
              <span class="inline-flex">
                <button
                  type="button"
                  class="app-chip h-7 px-2.5 text-xs disabled:cursor-not-allowed disabled:opacity-50"
                  :aria-label="t('recording.preview')"
                  :aria-busy="isPending('previewHighlight', row.id)"
                  :disabled="!row.isClipAvailable || isPending('previewHighlight', row.id)"
                  @click="emit('preview', row)"
                >
                  <el-icon v-if="isPending('previewHighlight', row.id)" class="is-loading">
                    <Loading />
                  </el-icon>
                  <el-icon v-else><VideoPlay /></el-icon>
                  {{ t('recording.preview') }}
                </button>
              </span>
            </el-tooltip>
            <el-tooltip
              :content="row.isClipAvailable ? t('common.export') : t('recording.clipUnavailable')"
            >
              <span class="inline-flex">
                <button
                  type="button"
                  class="app-chip h-7 px-2.5 text-xs disabled:cursor-not-allowed disabled:opacity-50"
                  :aria-label="t('common.export')"
                  :aria-busy="isPending('exportHighlight', row.id)"
                  :disabled="!row.isClipAvailable || isPending('exportHighlight', row.id)"
                  @click="emit('export', row)"
                >
                  <el-icon v-if="isPending('exportHighlight', row.id)" class="is-loading">
                    <Loading />
                  </el-icon>
                  <el-icon v-else><Download /></el-icon>
                  {{ t('common.export') }}
                </button>
              </span>
            </el-tooltip>
            <el-tooltip :content="t('common.delete')">
              <button
                type="button"
                class="app-chip h-7 px-2.5 text-xs disabled:cursor-not-allowed disabled:opacity-50"
                style="color: var(--app-red); border-color: color-mix(in srgb, var(--app-red) 45%, transparent)"
                :aria-label="t('common.delete')"
                :aria-busy="isPending('deleteHighlight', row.id)"
                :disabled="isPending('deleteHighlight', row.id)"
                @click="emit('delete', row)"
              >
                <el-icon v-if="isPending('deleteHighlight', row.id)" class="is-loading">
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
        <p class="app-faint m-0 py-10 text-center text-sm">{{ t('recording.noHighlights') }}</p>
      </template>
    </el-table>
  </div>
</template>
