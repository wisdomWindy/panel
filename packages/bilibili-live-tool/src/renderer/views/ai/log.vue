<script setup lang="ts">
import { onMounted, ref } from 'vue';
import { ArrowLeft, Refresh, Search } from '@element-plus/icons-vue';
import { useRouter } from 'vue-router';
import type { UserUid } from '@/platform/desktop';
import { useAIStore } from '@/stores/ai';

const router = useRouter();
const store = useAIStore();
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

function dateTime(value: number): string {
  return new Intl.DateTimeFormat(undefined, { dateStyle: 'medium', timeStyle: 'medium' }).format(
    value
  );
}

onMounted(query);
</script>

<template>
  <div class="mx-auto w-full max-w-6xl">
    <header class="mb-5 flex flex-wrap items-center justify-between gap-3">
      <div class="flex items-center gap-3">
        <el-button
          circle
          :icon="ArrowLeft"
          title="返回 AI 助手"
          @click="router.push('/ai')"
        />
        <h2 class="m-0 text-xl font-semibold text-gray-900 dark:text-gray-100">
          AI 日志
        </h2>
      </div>
      <el-button :icon="Refresh" :loading="store.pendingOperations.has('loadLogs')" @click="query">
        刷新
      </el-button>
    </header>

    <section
      class="mb-4 flex flex-wrap items-end gap-3 border-y border-gray-200 bg-white px-4 py-3 dark:border-gray-700 dark:bg-gray-900"
    >
      <label class="text-sm">
        <span class="mb-1 block text-gray-500">UID</span>
        <el-input v-model="userUid" clearable class="w-48" />
      </label>
      <label class="text-sm">
        <span class="mb-1 block text-gray-500">开始时间</span>
        <el-date-picker v-model="startedAt" type="datetime" />
      </label>
      <label class="text-sm">
        <span class="mb-1 block text-gray-500">结束时间</span>
        <el-date-picker v-model="endedAt" type="datetime" />
      </label>
      <el-button type="primary" :icon="Search" @click="query">
        查询
      </el-button>
    </section>

    <el-alert
      v-if="store.error"
      class="mb-4"
      type="error"
      :closable="false"
      :title="store.error.message"
      show-icon
    />
    <div v-loading="store.pendingOperations.has('loadLogs')" class="overflow-x-auto">
      <el-table :data="store.logs" class="min-w-[920px]">
        <el-table-column label="时间" width="190">
          <template #default="{ row }">
            {{ dateTime(row.createdAt) }}
          </template>
        </el-table-column>
        <el-table-column prop="operationType" label="类型" width="120" />
        <el-table-column
          prop="username"
          label="用户"
          width="140"
          show-overflow-tooltip
        />
        <el-table-column
          prop="userUid"
          label="UID"
          width="130"
          show-overflow-tooltip
        />
        <el-table-column
          prop="originalContent"
          label="原始内容"
          min-width="220"
          show-overflow-tooltip
        />
        <el-table-column
          prop="aiContent"
          label="AI 内容"
          min-width="240"
          show-overflow-tooltip
        />
        <el-table-column prop="generationStatus" label="状态" width="110" />
        <el-table-column
          prop="rejectReason"
          label="失败原因"
          min-width="160"
          show-overflow-tooltip
        />
      </el-table>
    </div>
  </div>
</template>
