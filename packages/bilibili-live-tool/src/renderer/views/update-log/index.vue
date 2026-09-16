<script setup lang="ts">
import { computed, onMounted, ref } from 'vue';
import { Refresh } from '@element-plus/icons-vue';
import { useI18n } from 'vue-i18n';
import { useReleaseStore } from '@/stores/release';
import UpdateAvailableDialog from './UpdateAvailableDialog.vue';

const { t } = useI18n();
const store = useReleaseStore();
const isDialogOpen = ref(false);
const isPending = computed(() => store.pending.size > 0);

async function check(): Promise<void> {
  try {
    await store.checkUpdate();
    isDialogOpen.value = ['available', 'downloading', 'downloaded'].includes(store.update?.status ?? '');
  } catch {
    // The page alert owns the native failure.
  }
}

async function download(): Promise<void> {
  try { await store.downloadUpdate(); } catch { /* The page alert owns the native failure. */ }
}

async function install(): Promise<void> {
  try { await store.installUpdate(); } catch { /* The page alert owns the native failure. */ }
}

async function skip(): Promise<void> {
  if (!store.update?.candidate) return;
  try {
    await store.skipUpdate(store.update.candidate.version);
    isDialogOpen.value = false;
  } catch { /* The page alert owns the native failure. */ }
}

onMounted(() => {
  if (!store.update && !store.pending.has('bootstrap')) {
    void store.bootstrap().then(check).catch(() => undefined);
  }
});
</script>

<template>
  <section class="min-w-0 bg-white px-7 py-6 dark:bg-gray-900">
    <header class="flex flex-wrap items-center justify-between gap-3 border-b border-gray-200 pb-5 dark:border-gray-700">
      <div>
        <h2 class="m-0 text-xl font-semibold">
          {{ t('update.title') }}
        </h2>
        <p class="mb-0 mt-1 text-sm text-gray-500">
          {{ t('update.currentVersion') }}:
          <span class="font-semibold text-gray-900 dark:text-gray-100">v{{ store.update?.currentVersion ?? '--' }}</span>
        </p>
      </div>
      <el-button :icon="Refresh" :loading="store.pending.has('checkUpdate')" @click="check">
        {{ t('update.checkUpdate') }}
      </el-button>
    </header>
    <el-alert
      v-if="store.error"
      class="mt-5"
      type="warning"
      show-icon
      :closable="false"
      :title="store.error.message"
    />
    <el-alert
      v-else-if="store.update?.status === 'unavailable'"
      class="mt-5"
      type="info"
      show-icon
      :closable="false"
      :title="t('update.unavailable')"
    />
    <div v-if="store.update?.candidate" class="mt-6">
      <div class="flex items-center justify-between border-b border-gray-200 pb-3 dark:border-gray-700">
        <h3 class="m-0 text-sm font-semibold">
          v{{ store.update.candidate.version }}
        </h3>
        <el-button type="primary" link @click="isDialogOpen = true">
          {{ t('update.whatsNew') }}
        </el-button>
      </div>
      <ul class="space-y-2 pl-5 text-sm">
        <li v-for="note in store.update.candidate.releaseNotes" :key="note">
          {{ note }}
        </li>
      </ul>
    </div>
    <el-empty v-else-if="!store.error" :description="t('update.noChangelog')" />
    <UpdateAvailableDialog
      v-if="store.update"
      v-model="isDialogOpen"
      :snapshot="store.update"
      :is-pending="isPending"
      @download="download"
      @install="install"
      @skip="skip"
    />
  </section>
</template>
