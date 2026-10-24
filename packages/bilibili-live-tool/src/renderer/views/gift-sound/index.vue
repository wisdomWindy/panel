<script setup lang="ts">
import { computed, onMounted, ref } from 'vue';
import { useI18n } from 'vue-i18n';
import { Download, RefreshLeft, Search, Upload } from '@element-plus/icons-vue';
import type { GiftId, GiftSoundBinding } from '@/platform/desktop';
import { useMediaGiftsStore } from '@/stores/media/gifts';
import GiftSoundTable from '@/components/media/GiftSoundTable.vue';

const store = useMediaGiftsStore();
const { t } = useI18n();
const search = ref('');
const boundOnly = ref(false);
const globalVolume = ref(0.8);
const drafts = ref<GiftSoundBinding[]>([]);

const filteredRows = computed(() => {
  const needle = search.value.trim().toLocaleLowerCase();
  return drafts.value.filter(
    row =>
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
  drafts.value = drafts.value.map(row => (row.giftId === giftId ? { ...row, ...patch } : row));
}

async function replaceSound(row: GiftSoundBinding): Promise<void> {
  try {
    await store.bindSound(row.giftId, row.giftName);
    buildDrafts();
  } catch {
    ElMessage.error(store.error?.message ?? t('giftSound.selectFailed'));
  }
}

async function previewSound(row: GiftSoundBinding): Promise<void> {
  try {
    await store.previewSound({ kind: 'binding', bindingId: row.giftId }, row.volume);
  } catch {
    ElMessage.error(store.error?.message ?? t('giftSound.previewFailed'));
  }
}

async function removeSound(row: GiftSoundBinding): Promise<void> {
  try {
    await ElMessageBox.confirm(
      t('giftSound.removeConfirm', { giftName: row.giftName }),
      t('giftSound.removeTitle'),
      {
        confirmButtonText: t('common.remove'),
        cancelButtonText: t('common.cancel'),
        type: 'warning'
      }
    );
    await store.removeSound(row.giftId);
    buildDrafts();
  } catch (error) {
    if (error !== 'cancel' && error !== 'close') {
      ElMessage.error(store.error?.message ?? t('giftSound.removeFailed'));
    }
  }
}

async function save(): Promise<void> {
  try {
    await store.saveSounds(globalVolume.value, drafts.value);
    buildDrafts();
    ElMessage.success(t('giftSound.saved'));
  } catch {
    ElMessage.error(store.error?.message ?? t('common.saveFailed'));
  }
}

async function resetDefaults(): Promise<void> {
  try {
    await ElMessageBox.confirm(t('giftSound.resetConfirm'), t('giftSound.resetTitle'), {
      confirmButtonText: t('common.reset'),
      cancelButtonText: t('common.cancel'),
      type: 'warning'
    });
    const reset = drafts.value.map(row => ({
      ...row,
      volume: 0.8,
      isEnabled: row.soundFileName !== null
    }));
    await store.saveSounds(0.8, reset);
    buildDrafts();
    ElMessage.success(t('giftSound.resetSuccess'));
  } catch (error) {
    if (error !== 'cancel' && error !== 'close') {
      ElMessage.error(store.error?.message ?? t('captainEffect.resetFailed'));
    }
  }
}

async function importSounds(): Promise<void> {
  try {
    const count = await store.importSounds();
    buildDrafts();
    ElMessage.success(t('giftSound.imported', { count }));
  } catch {
    ElMessage.error(store.error?.message ?? t('giftSound.importFailed'));
  }
}

async function exportSounds(): Promise<void> {
  try {
    await store.exportSounds();
    ElMessage.success(t('giftSound.exported'));
  } catch {
    ElMessage.error(store.error?.message ?? t('giftSound.exportFailed'));
  }
}

onMounted(load);
</script>

<template>
  <section class="min-w-0">
    <header class="mb-4 flex flex-wrap items-center gap-3 border-b border-gray-200 pb-4 dark:border-zinc-800">
      <div class="min-w-56 flex-1">
        <h2 class="m-0 text-xl font-semibold text-gray-900 dark:text-zinc-100">
          {{ t('giftSound.title') }}
        </h2>
        <p class="mb-0 mt-1 text-xs text-gray-500 dark:text-zinc-400">
          {{ t('giftSound.description') }}
        </p>
      </div>
      <el-button
        :icon="Upload"
        :loading="store.pendingOperations.has('importSounds')"
        @click="importSounds"
      >
        {{ t('common.import') }}
      </el-button>
      <el-button
        :icon="Download"
        :loading="store.pendingOperations.has('exportSounds')"
        @click="exportSounds"
      >
        {{ t('common.export') }}
      </el-button>
      <el-button :icon="RefreshLeft" :disabled="savePending" @click="resetDefaults">
        {{ t('common.resetDefaults') }}
      </el-button>
      <el-button type="primary" :loading="savePending" @click="save">
        {{ t('common.save') }}
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
        :placeholder="t('giftSound.searchPlaceholder')"
        class="!w-64"
      />
      <el-checkbox v-model="boundOnly">
        {{ t('giftSound.boundOnly') }}
      </el-checkbox>
      <span class="ml-auto text-xs text-gray-500 dark:text-zinc-400">
        {{ filteredRows.length }} / {{ drafts.length }}
      </span>
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

    <footer class="mt-4 flex flex-wrap items-center gap-5 border-t border-gray-200 pt-4 dark:border-zinc-800">
      <span class="text-sm font-medium text-gray-800 dark:text-zinc-200">{{ t('giftSound.globalVolume') }}</span>
      <el-slider
        v-model="globalVolume"
        :min="0"
        :max="1"
        :step="0.1"
        show-input
        class="!w-72"
      />
      <span class="ml-auto text-sm text-gray-500 dark:text-zinc-400">
        {{ t('giftSound.queue', { count: store.soundSnapshot?.queueCapacityPerSecond ?? 10 }) }}
      </span>
    </footer>
  </section>
</template>
