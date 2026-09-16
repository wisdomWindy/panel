<script setup lang="ts">
import { computed, onMounted, reactive, ref } from 'vue';
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

const categoryLabels: Record<SensitiveWordCategory, string> = {
  illegal: '违规',
  advertising: '广告',
  personal_attack: '人身攻击',
  custom: '自定义'
};

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

function showError(value: unknown): void {
  const error = normalizeDesktopError(value);
  if (error.code !== 'dialog_cancelled') ElMessage.error(error.message ?? '操作失败，请稍后重试');
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
    await ElMessageBox.confirm(row.word, '删除敏感词', {
      type: 'warning',
      confirmButtonText: '删除',
      cancelButtonText: '取消'
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
      importResult.value = `导入 ${result.importedCount}，重复 ${result.duplicateCount}，失败 ${result.errorCount}`;
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
  <section class="mx-auto flex h-full min-h-[560px] max-w-7xl flex-col bg-white">
    <header class="flex min-h-14 flex-wrap items-center gap-3 border-b border-gray-200 py-2">
      <h2 class="mr-auto m-0 text-lg font-semibold text-gray-900">
        敏感词管理
      </h2>
      <el-select v-model="category" class="!w-32" aria-label="敏感词分类">
        <el-option label="全部分类" value="all" />
        <el-option
          v-for="(label, value) in categoryLabels"
          :key="value"
          :label="label"
          :value="value"
        />
      </el-select>
      <el-input
        v-model="search"
        clearable
        class="!w-48"
        placeholder="搜索敏感词"
        aria-label="搜索敏感词"
      />
      <el-select v-model="format" class="!w-24" aria-label="文件格式">
        <el-option label="CSV" value="csv" />
        <el-option label="JSON" value="json" />
        <el-option label="TXT" value="txt" />
      </el-select>
      <el-button :icon="Upload" :loading="safety.mutationPending" @click="importWords">
        导入
      </el-button>
      <el-button :icon="Download" :loading="safety.mutationPending" @click="exportWords">
        导出
      </el-button>
      <el-button type="primary" :icon="Plus" @click="openCreate">
        新增
      </el-button>
    </header>

    <el-progress
      v-if="safety.compileProgress.percentage > 0 && safety.compileProgress.percentage < 100"
      class="my-3"
      :percentage="safety.compileProgress.percentage"
    />
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
        class="min-w-[760px]"
        row-key="id"
      >
        <el-table-column prop="word" label="敏感词" min-width="220" />
        <el-table-column label="分类" width="130">
          <template #default="{ row }">
            {{ categoryLabels[row.category as SensitiveWordCategory] }}
          </template>
        </el-table-column>
        <el-table-column label="匹配方式" width="120">
          <template #default="{ row }">
            {{ row.matchType === 'exact' ? '精确' : '模糊' }}
          </template>
        </el-table-column>
        <el-table-column label="启用" width="100">
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
        <el-table-column label="操作" width="130" fixed="right">
          <template #default="{ row }">
            <el-button
              :icon="Edit"
              link
              aria-label="编辑敏感词"
              @click="openEdit(row)"
            />
            <el-button
              :icon="Delete"
              link
              type="danger"
              aria-label="删除敏感词"
              @click="remove(row)"
            />
          </template>
        </el-table-column>
      </el-table>
    </div>

    <footer
      class="flex min-h-11 items-center gap-6 border-t border-gray-200 px-3 text-sm text-gray-600"
    >
      <span>总数 {{ safety.sensitiveWords.items.length }}</span>
      <span>已启用 {{ enabledCount }}</span>
      <span>今日命中 {{ safety.sensitiveWords.todayHitCount }}</span>
    </footer>

    <el-dialog
      v-model="dialogVisible"
      :title="editingId === null ? '新增敏感词' : '编辑敏感词'"
      width="min(480px, 92vw)"
      @keydown.enter="submit"
    >
      <el-form label-position="top">
        <el-form-item label="敏感词" :error="formError">
          <el-input
            v-model="form.word"
            maxlength="100"
            autofocus
            @blur="form.word = normalizeSensitiveWord(form.word)"
          />
        </el-form-item>
        <el-form-item label="分类">
          <el-select v-model="form.category" class="w-full">
            <el-option
              v-for="(label, value) in categoryLabels"
              :key="value"
              :label="label"
              :value="value"
            />
          </el-select>
        </el-form-item>
        <el-form-item label="匹配方式">
          <el-radio-group v-model="form.matchType">
            <el-radio-button value="exact">
              精确
            </el-radio-button>
            <el-radio-button value="fuzzy">
              模糊
            </el-radio-button>
          </el-radio-group>
        </el-form-item>
        <el-form-item>
          <el-checkbox v-model="form.isEnabled">
            启用
          </el-checkbox>
        </el-form-item>
      </el-form>
      <template #footer>
        <el-button @click="dialogVisible = false">
          取消
        </el-button>
        <el-button type="primary" :loading="safety.mutationPending" @click="submit">
          保存
        </el-button>
      </template>
    </el-dialog>
  </section>
</template>
