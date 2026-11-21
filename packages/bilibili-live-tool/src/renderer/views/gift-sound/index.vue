<script setup lang="ts">
import { computed, onMounted, ref } from 'vue';
import { useI18n } from 'vue-i18n';
import { Download, Loading, RefreshLeft, Search, Upload } from '@element-plus/icons-vue';
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
  <section class="mx-auto flex w-full max-w-[1180px] flex-col gap-4">
    <header class="flex flex-wrap items-center gap-2">
      <div class="mr-auto min-w-0">
        <h2 class="app-text m-0 text-lg font-semibold">{{ t('giftSound.title') }}</h2>
        <p class="app-muted mt-1 mb-0 text-sm">{{ t('giftSound.description') }}</p>
      </div>
      <button
        type="button"
        class="app-chip disabled:cursor-not-allowed disabled:opacity-50"
        :aria-busy="store.pendingOperations.has('importSounds')"
        :disabled="store.pendingOperations.has('importSounds')"
        @click="importSounds"
      >
        <el-icon
          v-if="store.pendingOperations.has('importSounds')"
          class="is-loading"
        ><Loading /></el-icon>
        <el-icon v-else><Upload /></el-icon>
        {{ t('common.import') }}
      </button>
      <button
        type="button"
        class="app-chip disabled:cursor-not-allowed disabled:opacity-50"
        :aria-busy="store.pendingOperations.has('exportSounds')"
        :disabled="store.pendingOperations.has('exportSounds')"
        @click="exportSounds"
      >
        <el-icon
          v-if="store.pendingOperations.has('exportSounds')"
          class="is-loading"
        ><Loading /></el-icon>
        <el-icon v-else><Download /></el-icon>
        {{ t('common.export') }}
      </button>
      <button
        type="button"
        class="app-chip disabled:cursor-not-allowed disabled:opacity-50"
        :disabled="savePending"
        @click="resetDefaults"
      >
        <el-icon><RefreshLeft /></el-icon>
        {{ t('common.resetDefaults') }}
      </button>
      <button
        type="button"
        class="app-btn-primary inline-flex h-8 items-center gap-1.5 px-4 text-sm disabled:cursor-not-allowed disabled:opacity-60"
        :aria-busy="savePending"
        :disabled="savePending"
        @click="save"
      >
        <el-icon v-if="savePending" class="is-loading"><Loading /></el-icon>
        {{ t('common.save') }}
      </button>
    </header>

    <el-alert
      v-if="store.error"
      type="error"
      :closable="false"
      :title="store.error.message"
      show-icon
    />

    <section class="app-card p-4" aria-labelledby="gift-sound-bindings-title">
      <div class="flex flex-wrap items-center gap-2">
        <h3 id="gift-sound-bindings-title" class="app-text m-0 text-base font-semibold">
          {{ t('giftSound.bindings') }}
        </h3>
        <span class="app-faint text-xs tabular-nums">
          {{ filteredRows.length }} / {{ drafts.length }}
        </span>
        <div class="ml-auto flex flex-wrap items-center gap-2">
          <el-input
            v-model="search"
            :prefix-icon="Search"
            clearable
            :placeholder="t('giftSound.searchPlaceholder')"
            :aria-label="t('giftSound.searchPlaceholder')"
            class="!w-56"
          />
          <button
            type="button"
            class="app-chip"
            :class="{ 'app-chip--active': boundOnly }"
            :aria-pressed="boundOnly"
            @click="boundOnly = !boundOnly"
          >
            {{ t('giftSound.boundOnly') }}
          </button>
        </div>
      </div>
      <p class="app-faint mt-1 mb-0 text-xs">{{ t('giftSound.bindingsHint') }}</p>
      <div v-loading="store.pending" class="mt-3">
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
    </section>

    <section class="app-card p-4" aria-labelledby="gift-sound-playback-title">
      <h3 id="gift-sound-playback-title" class="app-text m-0 text-base font-semibold">
        {{ t('giftSound.playback') }}
      </h3>
      <p class="app-muted mt-1 mb-0 text-sm">{{ t('giftSound.playbackHint') }}</p>
      <div class="mt-3 flex flex-wrap items-center gap-4 border-t app-hairline pt-3">
        <span class="app-muted shrink-0 text-sm">{{ t('giftSound.globalVolume') }}</span>
        <span class="app-text text-sm tabular-nums">{{ Math.round(globalVolume * 100) }}%</span>
        <el-slider
          v-model="globalVolume"
          :min="0"
          :max="1"
          :step="0.1"
          show-input
          class="!w-72"
          :aria-label="t('giftSound.globalVolume')"
        />
        <span class="app-faint ml-auto text-sm">
          {{ t('giftSound.queue', { count: store.soundSnapshot?.queueCapacityPerSecond ?? 10 }) }}
        </span>
      </div>
    </section>
  </section>
</template>
