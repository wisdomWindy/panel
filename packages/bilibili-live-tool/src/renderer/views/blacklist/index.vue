<script setup lang="ts">
import { computed, onMounted, reactive, ref } from 'vue';
import { useI18n } from 'vue-i18n';
import { Delete, Download, Plus, Upload } from '@element-plus/icons-vue';
import { normalizeDesktopError, type BlacklistEntry, type UserUid } from '@/platform/desktop';
import { useLiveSafetyStore } from '@/stores/live-data/safety';
import { userUidError, usernameError } from '@/modules/live-data/validation';

const safety = useLiveSafetyStore();
const { locale, t } = useI18n();
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

const emptyText = computed(() =>
  safety.blacklist.items.length === 0 ? t('blacklist.noRecords') : t('blacklist.noMatches')
);

function showError(value: unknown): void {
  const error = normalizeDesktopError(value);
  if (error.code !== 'SYS_DIALOG_CANCELLED')
    ElMessage.error(error.message ?? t('common.operationFailed'));
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
    await ElMessageBox.confirm(`${row.username} · ${row.uid}`, t('blacklist.removeTitle'), {
      type: 'warning',
      confirmButtonText: t('common.remove'),
      cancelButtonText: t('common.cancel')
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
      importResult.value = t('blacklist.importSummary', {
        imported: result.importedCount,
        duplicates: result.duplicateCount,
        failed: result.errorCount
      });
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
  return new Intl.DateTimeFormat(locale.value, {
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
  <section class="mx-auto flex h-full min-h-[560px] w-full max-w-[1180px] flex-col gap-3">
    <header class="flex flex-wrap items-center gap-2">
      <div class="mr-auto min-w-0">
        <h2 class="app-text m-0 text-lg font-semibold">
          {{ t('blacklist.title') }}
        </h2>
        <p class="app-muted mt-1 mb-0 text-sm">
          {{ t('blacklist.description') }}
        </p>
      </div>
      <el-input
        v-model="search"
        clearable
        class="!w-56"
        :placeholder="t('blacklist.searchPlaceholder')"
        :aria-label="t('blacklist.searchPlaceholder')"
      />
      <button
        type="button"
        class="app-chip disabled:cursor-not-allowed disabled:opacity-50"
        :disabled="safety.mutationPending"
        :aria-busy="safety.mutationPending"
        @click="importRows"
      >
        <el-icon><Upload /></el-icon>
        {{ t('common.import') }}
      </button>
      <button
        type="button"
        class="app-chip disabled:cursor-not-allowed disabled:opacity-50"
        :disabled="safety.mutationPending"
        :aria-busy="safety.mutationPending"
        @click="exportRows"
      >
        <el-icon><Download /></el-icon>
        {{ t('common.export') }}
      </button>
      <button
        type="button"
        class="app-btn-primary inline-flex h-8 items-center gap-1.5 px-3 text-sm"
        @click="openDialog"
      >
        <el-icon><Plus /></el-icon>
        {{ t('blacklist.manualAdd') }}
      </button>
    </header>

    <p v-if="importResult" class="app-muted my-0 text-sm">
      {{ importResult }}
    </p>
    <el-alert
      v-if="safety.error"
      :title="safety.error.message"
      type="error"
      :closable="false"
      show-icon
    />

    <section class="app-card flex min-h-0 flex-1 flex-col overflow-hidden">
      <div class="min-h-0 flex-1 overflow-x-auto">
        <el-table
          v-loading="safety.pending"
          :data="filteredRows"
          :empty-text="emptyText"
          height="100%"
          class="min-w-[840px]"
          header-cell-class-name="app-faint"
          row-key="uid"
        >
          <el-table-column prop="uid" label="UID" width="180" />
          <el-table-column prop="username" :label="t('blacklist.username')" min-width="160" />
          <el-table-column :label="t('blacklist.reason')" min-width="220" show-overflow-tooltip>
            <template #default="{ row }">
              <span class="app-muted">{{ row.reason ?? '-' }}</span>
            </template>
          </el-table-column>
          <el-table-column :label="t('blacklist.createdAt')" width="190">
            <template #default="{ row }">
              <span class="app-faint tabular-nums">{{ formatDate(row.createdAt) }}</span>
            </template>
          </el-table-column>
          <el-table-column :label="t('common.actions')" width="90" fixed="right">
            <template #default="{ row }">
              <button
                type="button"
                class="app-icon-btn"
                :style="{ color: 'var(--app-red)' }"
                :aria-label="t('blacklist.removeTitle')"
                @click="remove(row)"
              >
                <el-icon><Delete /></el-icon>
              </button>
            </template>
          </el-table-column>
        </el-table>
      </div>

      <footer
        class="app-faint flex min-h-10 flex-wrap items-center gap-x-6 gap-y-1 border-t px-3 text-sm"
        style="border-color: var(--app-border)"
      >
        <span>
          {{ t('common.total') }}
          <strong class="app-text tabular-nums">{{ safety.blacklist.items.length }}</strong>
        </span>
        <span>
          {{ t('common.todayAdded') }}
          <strong class="app-text tabular-nums">{{ safety.blacklist.todayAddedCount }}</strong>
        </span>
      </footer>
    </section>

    <el-dialog
      v-model="dialogVisible"
      :title="t('blacklist.manualAdd')"
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
        <el-form-item :label="t('blacklist.username')" :error="errors.username">
          <el-input v-model="form.username" maxlength="50" @blur="validate" />
        </el-form-item>
        <el-form-item :label="t('blacklist.reason')">
          <el-input v-model="form.reason" maxlength="200" />
        </el-form-item>
      </el-form>
      <el-alert
        v-if="duplicate"
        :title="t('blacklist.duplicate', { username: duplicate.username })"
        type="warning"
        :closable="false"
      />
      <template #footer>
        <div class="flex justify-end gap-2">
          <button type="button" class="app-chip" @click="dialogVisible = false">
            {{ t('common.cancel') }}
          </button>
          <button
            type="button"
            class="app-btn-primary inline-flex h-8 items-center px-3 text-sm disabled:cursor-not-allowed disabled:opacity-60"
            :disabled="safety.mutationPending"
            :aria-busy="safety.mutationPending"
            @click="submit"
          >
            {{ duplicate ? t('blacklist.updateReason') : t('common.add') }}
          </button>
        </div>
      </template>
    </el-dialog>
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
