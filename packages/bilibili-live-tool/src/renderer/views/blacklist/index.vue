<script setup lang="ts">
import { computed, onMounted, reactive, ref } from 'vue';
import { Delete, Download, Plus, Upload } from '@element-plus/icons-vue';
import {
  normalizeDesktopError,
  type BlacklistEntry,
  type UserUid
} from '@/platform/desktop';
import { useLiveSafetyStore } from '@/stores/live-data/safety';
import { userUidError, usernameError } from '@/modules/live-data/validation';

const safety = useLiveSafetyStore();
const search = ref('');
const dialogVisible = ref(false);
const importResult = ref('');
const errors = reactive({ uid: '', username: '' });
const form = reactive({ uid: '', username: '', reason: '' });

const filteredRows = computed(() => {
  const query = search.value.trim().toLocaleLowerCase();
  return safety.blacklist.items.filter(
    row => !query || row.uid.includes(query) || row.username.toLocaleLowerCase().includes(query)
  );
});

const duplicate = computed(() => safety.blacklist.items.find(row => row.uid === form.uid));

function showError(value: unknown): void {
  const error = normalizeDesktopError(value);
  if (error.code !== 'dialog_cancelled') ElMessage.error(error.message ?? '操作失败，请稍后重试');
}

function openDialog(): void {
  Object.assign(form, { uid: '', username: '', reason: '' });
  Object.assign(errors, { uid: '', username: '' });
  dialogVisible.value = true;
}

function validate(): boolean {
  errors.uid = userUidError(form.uid);
  errors.username = usernameError(form.username);
  return !errors.uid && !errors.username;
}

async function submit(): Promise<void> {
  if (!validate()) return;
  try {
    await safety.addBlacklist({
      uid: form.uid.trim() as UserUid,
      username: form.username.trim(),
      reason: form.reason.trim() || null
    });
    dialogVisible.value = false;
  } catch (error) {
    showError(error);
  }
}

async function remove(row: BlacklistEntry): Promise<void> {
  try {
    await ElMessageBox.confirm(`${row.username} · ${row.uid}`, '移除黑名单', {
      type: 'warning',
      confirmButtonText: '移除',
      cancelButtonText: '取消'
    });
    await safety.removeBlacklist(row.uid);
  } catch (error) {
    if (error !== 'cancel' && error !== 'close') showError(error);
  }
}

async function importRows(): Promise<void> {
  try {
    const result = await safety.importBlacklist();
    if (result)
      importResult.value = `导入 ${result.importedCount}，重复 ${result.duplicateCount}，失败 ${result.errorCount}`;
  } catch (error) {
    showError(error);
  }
}

async function exportRows(): Promise<void> {
  try {
    await safety.exportBlacklist();
  } catch (error) {
    showError(error);
  }
}

function formatDate(value: number): string {
  return new Intl.DateTimeFormat('zh-CN', {
    year: 'numeric',
    month: '2-digit',
    day: '2-digit',
    hour: '2-digit',
    minute: '2-digit',
    second: '2-digit',
    hour12: false
  }).format(value);
}

onMounted(() => {
  void safety.load().catch(() => undefined);
});
</script>

<template>
  <section class="mx-auto flex h-full min-h-[560px] max-w-7xl flex-col bg-white">
    <header class="flex min-h-14 flex-wrap items-center gap-3 border-b border-gray-200 py-2">
      <h2 class="mr-auto m-0 text-lg font-semibold text-gray-900">
        黑名单管理
      </h2>
      <el-input
        v-model="search"
        clearable
        class="!w-56"
        placeholder="搜索用户名或 UID"
        aria-label="搜索用户名或 UID"
      />
      <el-button :icon="Upload" :loading="safety.mutationPending" @click="importRows">
        导入
      </el-button>
      <el-button :icon="Download" :loading="safety.mutationPending" @click="exportRows">
        导出
      </el-button>
      <el-button type="primary" :icon="Plus" @click="openDialog">
        手动添加
      </el-button>
    </header>

    <p v-if="importResult" class="my-2 text-sm text-gray-600">
      {{ importResult }}
    </p>
    <el-alert
      v-if="safety.error"
      class="my-3"
      :title="safety.error.message"
      type="error"
      :closable="false"
    />

    <div class="min-h-0 flex-1 overflow-x-auto">
      <el-table
        v-loading="safety.pending"
        :data="filteredRows"
        height="100%"
        class="min-w-[840px]"
        row-key="uid"
      >
        <el-table-column prop="uid" label="UID" width="180" />
        <el-table-column prop="username" label="用户名" min-width="160" />
        <el-table-column label="原因" min-width="220">
          <template #default="{ row }">
            {{ row.reason ?? '-' }}
          </template>
        </el-table-column>
        <el-table-column label="加入时间" width="190">
          <template #default="{ row }">
            {{ formatDate(row.createdAt) }}
          </template>
        </el-table-column>
        <el-table-column label="操作" width="90" fixed="right">
          <template #default="{ row }">
            <el-button
              :icon="Delete"
              link
              type="danger"
              aria-label="移除黑名单"
              @click="remove(row)"
            />
          </template>
        </el-table-column>
      </el-table>
    </div>

    <footer
      class="flex min-h-11 items-center gap-6 border-t border-gray-200 px-3 text-sm text-gray-600"
    >
      <span>总数 {{ safety.blacklist.items.length }}</span>
      <span>今日新增 {{ safety.blacklist.todayAddedCount }}</span>
    </footer>

    <el-dialog
      v-model="dialogVisible"
      title="手动添加"
      width="min(480px, 92vw)"
      @keydown.enter="submit"
    >
      <el-form label-position="top">
        <el-form-item label="UID" :error="errors.uid">
          <el-input
            v-model="form.uid"
            maxlength="20"
            inputmode="numeric"
            autofocus
            @blur="validate"
          />
        </el-form-item>
        <el-form-item label="用户名" :error="errors.username">
          <el-input v-model="form.username" maxlength="50" @blur="validate" />
        </el-form-item>
        <el-form-item label="原因">
          <el-input v-model="form.reason" maxlength="200" />
        </el-form-item>
      </el-form>
      <el-alert
        v-if="duplicate"
        :title="`UID 已存在：${duplicate.username}`"
        type="warning"
        :closable="false"
      />
      <template #footer>
        <el-button @click="dialogVisible = false">
          取消
        </el-button>
        <el-button type="primary" :loading="safety.mutationPending" @click="submit">
          {{ duplicate ? '更新原因' : '添加' }}
        </el-button>
      </template>
    </el-dialog>
  </section>
</template>
