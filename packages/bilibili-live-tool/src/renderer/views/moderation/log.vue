<script setup lang="ts">
import { onMounted, ref } from 'vue';
import { useI18n } from 'vue-i18n';
import { ArrowLeft, Refresh, Search } from '@element-plus/icons-vue';
import { useRouter } from 'vue-router';
import type { UserUid } from '@/platform/desktop';
import { useModerationStore } from '@/stores/moderation';

const router = useRouter();
const { t } = useI18n();
const store = useModerationStore();
const userUid = ref('');
const startedAt = ref<number | null>(null);
const endedAt = ref<number | null>(null);

function query(): void {
  const normalizedUid = userUid.value.trim();
  void store
    .loadLogs({
      userUid: normalizedUid ? (normalizedUid as UserUid) : null,
      startedAt: startedAt.value,
      endedAt: endedAt.value
    })
    .catch(() => undefined);
}

async function markFalsePositive(logId: number): Promise<void> {
  await store.markFalsePositive(logId).catch(() => undefined);
}

function dateTime(value: number): string {
  return new Intl.DateTimeFormat(undefined, { dateStyle: 'medium', timeStyle: 'medium' }).format(
    value
  );
}

onMounted(query);
</script>

<template>
  <section class="mx-auto flex w-full max-w-[1180px] flex-col gap-4">
    <header class="flex flex-wrap items-center gap-2">
      <button
        type="button"
        class="app-icon-btn"
        :aria-label="t('moderation.logTitle')"
        :title="t('moderation.logTitle')"
        @click="router.push('/moderation')"
      >
        <el-icon><ArrowLeft /></el-icon>
      </button>
      <div class="mr-auto min-w-0">
        <h2 class="app-text m-0 text-lg font-semibold">
          {{ t('moderation.logTitle') }}
        </h2>
      </div>
      <button
        type="button"
        class="app-chip disabled:cursor-not-allowed disabled:opacity-50"
        :disabled="store.pendingOperations.has('loadLogs')"
        :aria-busy="store.pendingOperations.has('loadLogs')"
        @click="query"
      >
        <el-icon><Refresh /></el-icon>
        {{ t('common.refresh') }}
      </button>
    </header>

    <section class="app-card flex flex-wrap items-end gap-3 px-4 py-3">
      <label class="text-sm">
        <span class="app-muted mb-1 block text-xs">{{ t('moderation.filterUid') }}</span>
        <el-input v-model="userUid" clearable class="w-44" />
      </label>
      <label class="text-sm">
        <span class="app-muted mb-1 block text-xs">{{ t('moderation.filterStart') }}</span>
        <el-date-picker v-model="startedAt" type="datetime" />
      </label>
      <label class="text-sm">
        <span class="app-muted mb-1 block text-xs">{{ t('moderation.filterEnd') }}</span>
        <el-date-picker v-model="endedAt" type="datetime" />
      </label>
      <button
        type="button"
        class="app-btn-primary inline-flex h-8 items-center gap-1.5 px-3.5 text-sm"
        @click="query"
      >
        <el-icon><Search /></el-icon>
        {{ t('common.search') }}
      </button>
    </section>

    <el-alert
      v-if="store.error"
      type="error"
      :closable="false"
      :title="store.error.message"
      show-icon
    />

    <section class="app-card flex min-h-0 flex-1 flex-col overflow-hidden">
      <div
        v-loading="store.pendingOperations.has('loadLogs')"
        class="min-h-0 flex-1 overflow-x-auto"
      >
        <el-table :data="store.logs" class="min-w-[760px]" header-cell-class-name="app-faint">
          <el-table-column :label="t('common.time')" width="190">
            <template #default="{ row }">
              <span class="app-muted tabular-nums">{{ dateTime(row.occurredAt) }}</span>
            </template>
          </el-table-column>
          <el-table-column
            prop="username"
            :label="t('moderation.columnUser')"
            min-width="150"
            show-overflow-tooltip
          />
          <el-table-column
            prop="userUid"
            :label="t('moderation.filterUid')"
            width="140"
            show-overflow-tooltip
          />
          <el-table-column :label="t('common.operation')" width="130">
            <template #default="{ row }">
              <span class="app-badge app-badge--plain">
                {{ t(`moderation.logAction.${row.action}`) }}
              </span>
            </template>
          </el-table-column>
          <el-table-column :label="t('moderation.columnFalsePositive')" width="100">
            <template #default="{ row }">
              <span :style="row.isFalsePositive ? { color: 'var(--app-gold)' } : undefined">
                {{ row.isFalsePositive ? t('moderation.yes') : t('moderation.no') }}
              </span>
            </template>
          </el-table-column>
          <el-table-column :label="t('common.actions')" width="150" fixed="right">
            <template #default="{ row }">
              <button
                type="button"
                class="app-chip h-7 px-2.5 text-xs disabled:cursor-not-allowed disabled:opacity-50"
                :disabled="row.isFalsePositive || store.pendingOperations.has(`falsePositive:${row.id}`)"
                :aria-busy="store.pendingOperations.has(`falsePositive:${row.id}`)"
                @click="markFalsePositive(row.id)"
              >
                {{ t('moderation.markFalsePositive') }}
              </button>
            </template>
          </el-table-column>
        </el-table>
      </div>
    </section>
  </section>
</template>

<style scoped>
.app-card :deep(.el-table) {
  --el-table-border-color: var(--app-border);
  --el-table-bg-color: transparent;
  --el-table-tr-bg-color: transparent;
  --el-table-header-bg-color: transparent;
  --el-table-header-text-color: var(--app-text-3);
  --el-table-text-color: var(--app-text);
  --el-table-row-hover-bg-color: var(--app-panel-2);
}
.app-card :deep(.el-table th.el-table__cell) {
  font-weight: 500;
}
</style>
