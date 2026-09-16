<script setup lang="ts">
import { computed, onMounted, ref } from 'vue';
import { Download, RefreshLeft, Search, Upload } from '@element-plus/icons-vue';
import type { GiftId, GiftSoundBinding } from '@/platform/desktop';
import { useMediaGiftsStore } from '@/stores/media/gifts';
import GiftSoundTable from '@/components/media/GiftSoundTable.vue';

const store = useMediaGiftsStore();
const search = ref('');
const boundOnly = ref(false);
const globalVolume = ref(0.8);
const drafts = ref<GiftSoundBinding[]>([]);

const filteredRows = computed(() => {
  const needle = search.value.trim().toLocaleLowerCase();
  return drafts.value.filter(row =>
    (!boundOnly.value || row.soundFileName !== null) &&
    (!needle || row.giftName.toLocaleLowerCase().includes(needle))
  );
});
const savePending = computed(() => store.pendingOperations.has('saveSounds'));

function buildDrafts(): void {
  const persisted = store.soundSnapshot?.bindings.map(item => ({ ...item })) ?? [];
  const byId = new Map(persisted.map(item => [item.giftId, item]));
  store.notifySnapshot?.latestGifts.forEach(event => {
    if (!byId.has(event.giftId)) {
      byId.set(event.giftId, {
        giftId: event.giftId,
        giftName: event.giftName,
        soundFileName: null,
        volume: 0.8,
        isEnabled: false,
        isMissing: false
      });
    }
  });
  drafts.value = [...byId.values()];
  globalVolume.value = store.soundSnapshot?.globalVolume ?? 0.8;
}

async function load(): Promise<void> {
  try {
    await store.load();
    buildDrafts();
  } catch {
    // The stable page error region owns load feedback.
  }
}

function updateRow(giftId: GiftId, patch: Partial<GiftSoundBinding>): void {
  drafts.value = drafts.value.map(row => row.giftId === giftId ? { ...row, ...patch } : row);
}

async function replaceSound(row: GiftSoundBinding): Promise<void> {
  try {
    await store.bindSound(row.giftId, row.giftName);
    buildDrafts();
  } catch {
    ElMessage.error(store.error?.message ?? '选择音效失败');
  }
}

async function previewSound(row: GiftSoundBinding): Promise<void> {
  try {
    await store.previewSound({ kind: 'binding', bindingId: row.giftId }, row.volume);
  } catch {
    ElMessage.error(store.error?.message ?? '音效预览失败');
  }
}

async function removeSound(row: GiftSoundBinding): Promise<void> {
  try {
    await ElMessageBox.confirm(`移除“${row.giftName}”的音效绑定？`, '移除音效', {
      confirmButtonText: '移除',
      cancelButtonText: '取消',
      type: 'warning'
    });
    await store.removeSound(row.giftId);
    buildDrafts();
  } catch (error) {
    if (error !== 'cancel' && error !== 'close') {
      ElMessage.error(store.error?.message ?? '移除失败');
    }
  }
}

async function save(): Promise<void> {
  try {
    await store.saveSounds(globalVolume.value, drafts.value);
    buildDrafts();
    ElMessage.success('礼物音效配置已保存');
  } catch {
    ElMessage.error(store.error?.message ?? '保存失败');
  }
}

async function resetDefaults(): Promise<void> {
  try {
    await ElMessageBox.confirm('将全部礼物音量恢复为 0.8 并启用已有绑定？', '重置礼物音效', {
      confirmButtonText: '重置',
      cancelButtonText: '取消',
      type: 'warning'
    });
    const reset = drafts.value.map(row => ({
      ...row,
      volume: 0.8,
      isEnabled: row.soundFileName !== null
    }));
    await store.saveSounds(0.8, reset);
    buildDrafts();
    ElMessage.success('已恢复默认配置');
  } catch (error) {
    if (error !== 'cancel' && error !== 'close') {
      ElMessage.error(store.error?.message ?? '重置失败');
    }
  }
}

async function importSounds(): Promise<void> {
  try {
    const count = await store.importSounds();
    buildDrafts();
    ElMessage.success(`已导入 ${count} 条配置`);
  } catch {
    ElMessage.error(store.error?.message ?? '导入失败');
  }
}

async function exportSounds(): Promise<void> {
  try {
    await store.exportSounds();
    ElMessage.success('礼物音效配置已导出');
  } catch {
    ElMessage.error(store.error?.message ?? '导出失败');
  }
}

onMounted(load);
</script>

<template>
  <section class="min-w-0 p-[18px] xl:p-6">
    <header class="mb-4 flex flex-wrap items-center gap-3 border-b border-gray-200 pb-4">
      <div class="min-w-56 flex-1">
        <h2 class="m-0 text-xl font-semibold text-gray-900">
          礼物音效
        </h2>
        <p class="mb-0 mt-1 text-xs text-gray-500">
          为本场出现的礼物绑定 MP3/WAV 音效
        </p>
      </div>
      <el-button :icon="Upload" :loading="store.pendingOperations.has('importSounds')" @click="importSounds">
        导入
      </el-button>
      <el-button :icon="Download" :loading="store.pendingOperations.has('exportSounds')" @click="exportSounds">
        导出
      </el-button>
      <el-button :icon="RefreshLeft" :disabled="savePending" @click="resetDefaults">
        重置默认
      </el-button>
      <el-button type="primary" :loading="savePending" @click="save">
        保存
      </el-button>
    </header>

    <el-alert
      v-if="store.error"
      class="mb-4"
      type="error"
      :closable="false"
      :title="store.error.message"
      show-icon
    />

    <div class="mb-3 flex flex-wrap items-center gap-3">
      <el-input
        v-model="search"
        :prefix-icon="Search"
        clearable
        placeholder="搜索礼物名称"
        class="!w-64"
      />
      <el-checkbox v-model="boundOnly">
        仅显示已绑定
      </el-checkbox>
      <span class="ml-auto text-xs text-gray-500">{{ filteredRows.length }} / {{ drafts.length }}</span>
    </div>

    <div v-loading="store.pending">
      <GiftSoundTable
        :rows="filteredRows"
        :pending-operations="store.pendingOperations"
        @preview="previewSound"
        @replace="replaceSound"
        @remove="removeSound"
        @volume="(giftId, value) => updateRow(giftId, { volume: value })"
        @enabled="(giftId, value) => updateRow(giftId, { isEnabled: value })"
      />
    </div>

    <footer class="mt-4 flex flex-wrap items-center gap-5 border-t border-gray-200 pt-4">
      <span class="text-sm font-medium text-gray-800">全局音量</span>
      <el-slider
        v-model="globalVolume"
        :min="0"
        :max="1"
        :step="0.1"
        show-input
        class="!w-72"
      />
      <span class="ml-auto text-sm text-gray-500">播放队列：{{ store.soundSnapshot?.queueCapacityPerSecond ?? 10 }} 个/秒</span>
    </footer>
  </section>
</template>
