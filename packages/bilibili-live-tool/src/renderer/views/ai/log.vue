<script setup lang="ts">
import { onMounted, ref } from 'vue';
import { ArrowLeft, Refresh, Search } from '@element-plus/icons-vue';
import { useRouter } from 'vue-router';
import { useI18n } from 'vue-i18n';
import type { UserUid } from '@/platform/desktop';
import { useAIStore } from '@/stores/ai';

const router = useRouter();
const { t } = useI18n();
const store = useAIStore();
const userUid = ref('');
const startedAt = ref<string | null>(null);
const endedAt = ref<string | null>(null);

function query(): void {
  const normalizedUid = userUid.value.trim();
  void store
    .loadLogs({
      userUid: normalizedUid ? (normalizedUid as UserUid) : null,
      startedAt: startedAt.value ? Number(startedAt.value) : null,
      endedAt: endedAt.value ? Number(endedAt.value) : null
    })
    .catch(() => undefined);
}

function dateTime(value: number): string {
  return new Intl.DateTimeFormat(undefined, { dateStyle: 'medium', timeStyle: 'medium' }).format(
    value
  );
}

onMounted(query);
</script>

<template>
  <section class="mx-auto flex w-full max-w-6xl min-w-0 flex-col gap-4">
    <header class="flex flex-wrap items-center justify-between gap-3">
      <div class="flex min-w-0 items-center gap-3">
        <button
          type="button"
          class="app-icon-btn shrink-0"
          :aria-label="t('aiLog.backToAssistant')"
          :title="t('aiLog.backToAssistant')"
          @click="router.push('/ai')"
        >
          <el-icon><ArrowLeft /></el-icon>
        </button>
        <h2 class="app-text m-0 text-lg font-semibold">
          {{ t('aiLog.title') }}
        </h2>
      </div>
      <el-button
        :icon="Refresh"
        :loading="store.pendingOperations.has('loadLogs')"
        @click="query"
      >
        {{ t('aiLog.refresh') }}
      </el-button>
    </header>

    <section class="app-card flex flex-wrap items-end gap-3 px-4 py-3">
      <label class="text-sm">
        <span class="app-muted mb-1 block">{{ t('aiLog.uid') }}</span>
        <el-input
          v-model="userUid"
          clearable
          class="w-48"
          :aria-label="t('aiLog.uid')"
        />
      </label>
      <label class="text-sm">
        <span class="app-muted mb-1 block">{{ t('aiLog.startedAt') }}</span>
        <el-date-picker v-model="startedAt" type="datetime" value-format="x" />
      </label>
      <label class="text-sm">
        <span class="app-muted mb-1 block">{{ t('aiLog.endedAt') }}</span>
        <el-date-picker v-model="endedAt" type="datetime" value-format="x" />
      </label>
      <button
        type="button"
        class="app-btn-primary inline-flex h-8 items-center gap-1.5 px-3.5 text-sm"
        @click="query"
      >
        <el-icon><Search /></el-icon>
        {{ t('aiLog.query') }}
      </button>
    </section>

    <el-alert
      v-if="store.error"
      type="error"
      :closable="false"
      :title="store.error.message"
      show-icon
    />
    <div v-loading="store.pendingOperations.has('loadLogs')" class="app-card overflow-x-auto p-2">
      <el-table :data="store.logs" class="min-w-[920px]" :empty-text="t('common.noData')">
        <el-table-column :label="t('aiLog.columns.time')" width="190">
          <template #default="{ row }">
            {{ dateTime(row.createdAt) }}
          </template>
        </el-table-column>
        <el-table-column prop="operationType" :label="t('aiLog.columns.type')" width="120" />
        <el-table-column
          prop="username"
          :label="t('aiLog.columns.user')"
          width="140"
          show-overflow-tooltip
        />
        <el-table-column
          prop="userUid"
          :label="t('aiLog.columns.uid')"
          width="130"
          show-overflow-tooltip
        />
        <el-table-column
          prop="originalContent"
          :label="t('aiLog.columns.originalContent')"
          min-width="220"
          show-overflow-tooltip
        />
        <el-table-column
          prop="aiContent"
          :label="t('aiLog.columns.aiContent')"
          min-width="240"
          show-overflow-tooltip
        />
        <el-table-column
          prop="generationStatus"
          :label="t('aiLog.columns.status')"
          width="110"
        />
        <el-table-column
          prop="rejectReason"
          :label="t('aiLog.columns.rejectReason')"
          min-width="160"
          show-overflow-tooltip
        />
      </el-table>
    </div>
  </section>
</template>
