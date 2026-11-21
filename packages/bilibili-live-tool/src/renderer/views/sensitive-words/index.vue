<script setup lang="ts">
import { computed, onMounted, reactive, ref } from 'vue';
import { useI18n } from 'vue-i18n';
import { Delete, Download, Edit, Plus, Upload } from '@element-plus/icons-vue';
import {
  normalizeDesktopError,
  type SensitiveFileFormat,
  type SensitiveWord,
  type SensitiveWordCategory
} from '@/platform/desktop';
import { useLiveSafetyStore } from '@/stores/live-data/safety';
import { normalizeSensitiveWord, sensitiveWordError } from '@/modules/live-data/validation';

const safety = useLiveSafetyStore();
const { t } = useI18n();
const search = ref('');
const category = ref<SensitiveWordCategory | 'all'>('all');
const format = ref<SensitiveFileFormat>('csv');
const dialogVisible = ref(false);
const editingId = ref<number | null>(null);
const formError = ref('');
const importResult = ref('');
const form = reactive({
  word: '',
  category: 'custom' as SensitiveWordCategory,
  matchType: 'exact' as 'exact' | 'fuzzy',
  isEnabled: true
});

const categoryLabels = computed<Record<SensitiveWordCategory, string>>(() => ({
  illegal: t('sensitiveWords.categories.illegal'),
  advertising: t('sensitiveWords.categories.advertising'),
  personal_attack: t('sensitiveWords.categories.personal_attack'),
  custom: t('sensitiveWords.categories.custom')
}));

const categoryOptions = computed(() =>
  (['all', 'illegal', 'advertising', 'personal_attack', 'custom'] as const).map(value => ({
    value,
    label: value === 'all' ? t('sensitiveWords.allCategories') : categoryLabels.value[value]
  }))
);

const filteredRows = computed(() => {
  const query = search.value.trim();
  return safety.sensitiveWords.items.filter(
    row =>
      (category.value === 'all' || row.category === category.value) &&
        (!query || row.word.includes(query))
  );
});

const enabledCount = computed(
  () => safety.sensitiveWords.items.filter(row => row.isEnabled).length
);

const emptyText = computed(() =>
  safety.sensitiveWords.items.length === 0
    ? t('sensitiveWords.empty')
    : t('sensitiveWords.noMatches')
);

function categoryTone(value: SensitiveWordCategory): string {
  switch (value) {
    case 'illegal':
      return 'app-badge--admin';
    case 'advertising':
      return 'app-badge--captain';
    case 'personal_attack':
      return 'app-badge--governor';
    default:
      return 'app-badge--plain';
  }
}

function showError(value: unknown): void {
  const error = normalizeDesktopError(value);
  if (error.code !== 'dialog_cancelled')
    ElMessage.error(error.message ?? t('common.operationFailed'));
}

function openCreate(): void {
  editingId.value = null;
  Object.assign(form, { word: '', category: 'custom', matchType: 'exact', isEnabled: true });
  formError.value = '';
  dialogVisible.value = true;
}

function openEdit(row: SensitiveWord): void {
  editingId.value = row.id;
  Object.assign(form, row);
  formError.value = '';
  dialogVisible.value = true;
}

async function submit(): Promise<void> {
  const word = normalizeSensitiveWord(form.word);
  formError.value = sensitiveWordError(word);
  if (formError.value) {
    return;
  }
  try {
    await safety.saveSensitiveWord({ ...form, word }, editingId.value ?? undefined);
    dialogVisible.value = false;
  } catch (error) {
    showError(error);
  }
}

async function remove(row: SensitiveWord): Promise<void> {
  try {
    await ElMessageBox.confirm(row.word, t('sensitiveWords.deleteTitle'), {
      type: 'warning',
      confirmButtonText: t('common.delete'),
      cancelButtonText: t('common.cancel')
    });
    await safety.deleteSensitiveWord(row.id);
  } catch (error) {
    if (error !== 'cancel' && error !== 'close') showError(error);
  }
}

async function importWords(): Promise<void> {
  try {
    const result = await safety.importSensitiveWords(format.value);
    if (result)
      importResult.value = t('sensitiveWords.importSummary', {
        imported: result.importedCount,
        duplicates: result.duplicateCount,
        failed: result.errorCount
      });
  } catch (error) {
    showError(error);
  }
}

async function exportWords(): Promise<void> {
  try {
    await safety.exportSensitiveWords(format.value);
  } catch (error) {
    showError(error);
  }
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
          {{ t('sensitiveWords.title') }}
        </h2>
        <p class="app-muted mt-1 mb-0 text-sm">
          {{ t('sensitiveWords.description') }}
        </p>
      </div>
      <el-input
        v-model="search"
        clearable
        class="!w-48"
        :placeholder="t('sensitiveWords.searchPlaceholder')"
        :aria-label="t('sensitiveWords.searchPlaceholder')"
      />
      <el-select v-model="format" class="!w-24" :aria-label="t('sensitiveWords.fileFormat')">
        <el-option label="CSV" value="csv" />
        <el-option label="JSON" value="json" />
        <el-option label="TXT" value="txt" />
      </el-select>
      <button
        type="button"
        class="app-chip disabled:cursor-not-allowed disabled:opacity-50"
        :disabled="safety.mutationPending"
        :aria-busy="safety.mutationPending"
        @click="importWords"
      >
        <el-icon><Upload /></el-icon>
        {{ t('common.import') }}
      </button>
      <button
        type="button"
        class="app-chip disabled:cursor-not-allowed disabled:opacity-50"
        :disabled="safety.mutationPending"
        :aria-busy="safety.mutationPending"
        @click="exportWords"
      >
        <el-icon><Download /></el-icon>
        {{ t('common.export') }}
      </button>
      <button
        type="button"
        class="app-btn-primary inline-flex h-8 items-center gap-1.5 px-3 text-sm"
        @click="openCreate"
      >
        <el-icon><Plus /></el-icon>
        {{ t('sensitiveWords.create') }}
      </button>
    </header>

    <div
      class="flex flex-wrap items-center gap-1.5"
      role="group"
      :aria-label="t('sensitiveWords.categoryLabel')"
    >
      <button
        v-for="option in categoryOptions"
        :key="option.value"
        type="button"
        class="app-chip"
        :class="{ 'app-chip--active': category === option.value }"
        :aria-pressed="category === option.value"
        @click="category = option.value"
      >
        {{ option.label }}
      </button>
    </div>

    <el-progress
      v-if="safety.compileProgress.percentage > 0 && safety.compileProgress.percentage < 100"
      :percentage="safety.compileProgress.percentage"
    />
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
          class="min-w-[760px]"
          header-cell-class-name="app-faint"
          row-key="id"
        >
          <el-table-column
            prop="word"
            :label="t('sensitiveWords.word')"
            min-width="220"
            show-overflow-tooltip
          />
          <el-table-column :label="t('sensitiveWords.category')" width="130">
            <template #default="{ row }">
              <span class="app-badge" :class="categoryTone(row.category as SensitiveWordCategory)">
                {{ categoryLabels[row.category as SensitiveWordCategory] }}
              </span>
            </template>
          </el-table-column>
          <el-table-column :label="t('sensitiveWords.matchType')" width="120">
            <template #default="{ row }">
              <span class="app-faint">
                {{
                  row.matchType === 'exact' ? t('sensitiveWords.exact') : t('sensitiveWords.fuzzy')
                }}
              </span>
            </template>
          </el-table-column>
          <el-table-column :label="t('common.enable')" width="100">
            <template #default="{ row }">
              <el-switch
                :model-value="row.isEnabled"
                :loading="safety.mutationPending"
                @change="
                  (value: string | number | boolean) =>
                    safety.toggleSensitiveWord(row.id, Boolean(value)).catch(showError)
                "
              />
            </template>
          </el-table-column>
          <el-table-column :label="t('common.actions')" width="130" fixed="right">
            <template #default="{ row }">
              <button
                type="button"
                class="app-icon-btn"
                :aria-label="t('sensitiveWords.editLabel')"
                @click="openEdit(row)"
              >
                <el-icon><Edit /></el-icon>
              </button>
              <button
                type="button"
                class="app-icon-btn"
                :style="{ color: 'var(--app-red)' }"
                :aria-label="t('sensitiveWords.deleteLabel')"
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
          <strong class="app-text tabular-nums">{{ safety.sensitiveWords.items.length }}</strong>
        </span>
        <span>
          {{ t('sensitiveWords.enabledCount', { count: enabledCount }) }}
        </span>
        <span>
          {{ t('common.todayHits') }}
          <strong class="app-text tabular-nums">{{ safety.sensitiveWords.todayHitCount }}</strong>
        </span>
      </footer>
    </section>

    <el-dialog
      v-model="dialogVisible"
      :title="editingId === null ? t('sensitiveWords.createTitle') : t('sensitiveWords.editTitle')"
      width="min(480px, 92vw)"
      @keydown.enter="submit"
    >
      <el-form label-position="top">
        <el-form-item :label="t('sensitiveWords.word')" :error="formError">
          <el-input
            v-model="form.word"
            maxlength="100"
            autofocus
            @blur="form.word = normalizeSensitiveWord(form.word)"
          />
        </el-form-item>
        <el-form-item :label="t('sensitiveWords.category')">
          <el-select v-model="form.category" class="w-full">
            <el-option
              v-for="(label, value) in categoryLabels"
              :key="value"
              :label="label"
              :value="value"
            />
          </el-select>
        </el-form-item>
        <el-form-item :label="t('sensitiveWords.matchType')">
          <el-radio-group v-model="form.matchType">
            <el-radio-button value="exact">
              {{ t('sensitiveWords.exact') }}
            </el-radio-button>
            <el-radio-button value="fuzzy">
              {{ t('sensitiveWords.fuzzy') }}
            </el-radio-button>
          </el-radio-group>
        </el-form-item>
        <el-form-item>
          <el-checkbox v-model="form.isEnabled">
            {{ t('common.enable') }}
          </el-checkbox>
        </el-form-item>
      </el-form>
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
            {{ t('common.save') }}
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
