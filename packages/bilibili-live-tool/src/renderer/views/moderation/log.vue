<script setup lang="ts">
import { onMounted, ref } from 'vue';
import { ArrowLeft, Refresh, Search } from '@element-plus/icons-vue';
import { useRouter } from 'vue-router';
import type { UserUid } from '@/platform/desktop';
import { useModerationStore } from '@/stores/moderation';

const router = useRouter();
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
  <div class="mx-auto w-full max-w-6xl">
    <header class="mb-5 flex flex-wrap items-center justify-between gap-3">
      <div class="flex items-center gap-3">
        <el-button
          circle
          :icon="ArrowLeft"
          title="返回刷屏防护"
          @click="router.push('/moderation')"
        />
        <h2 class="m-0 text-xl font-semibold text-gray-900 dark:text-gray-100">
          禁言日志
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
      <el-table :data="store.logs" class="min-w-[760px]">
        <el-table-column label="时间" width="190">
          <template #default="{ row }">
            {{ dateTime(row.occurredAt) }}
          </template>
        </el-table-column>
        <el-table-column
          prop="username"
          label="用户"
          min-width="150"
          show-overflow-tooltip
        />
        <el-table-column
          prop="userUid"
          label="UID"
          width="140"
          show-overflow-tooltip
        />
        <el-table-column prop="action" label="操作" width="130" />
        <el-table-column label="误判" width="100">
          <template #default="{ row }">
            {{ row.isFalsePositive ? '是' : '否' }}
          </template>
        </el-table-column>
        <el-table-column label="操作" width="120" fixed="right">
          <template #default="{ row }">
            <el-button
              link
              type="primary"
              :disabled="row.isFalsePositive"
              :loading="store.pendingOperations.has(`falsePositive:${row.id}`)"
              @click="markFalsePositive(row.id)"
            >
              标记误判
            </el-button>
          </template>
        </el-table-column>
      </el-table>
    </div>
  </div>
</template>
