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
  <section class="mx-auto flex w-full max-w-[1180px] min-w-0 flex-col gap-4">
    <header class="flex flex-wrap items-center justify-between gap-3">
      <div class="mr-auto min-w-0">
        <h2 class="app-text m-0 text-lg font-semibold">
          {{ t('update.title') }}
        </h2>
        <p class="app-muted mt-1 mb-0 text-sm">
          {{ t('update.pageHint') }}
        </p>
      </div>
      <button
        type="button"
        class="app-btn-primary inline-flex h-9 shrink-0 items-center gap-1.5 px-4 text-sm disabled:cursor-not-allowed disabled:opacity-60"
        :aria-busy="store.pending.has('checkUpdate')"
        :disabled="store.pending.has('checkUpdate')"
        @click="check"
      >
        <el-icon :class="{ 'is-loading': store.pending.has('checkUpdate') }">
          <Refresh />
        </el-icon>
        {{ t('update.checkUpdate') }}
      </button>
    </header>

    <el-alert
      v-if="store.error"
      type="warning"
      show-icon
      :closable="false"
      :title="store.error.message"
    />
    <el-alert
      v-else-if="store.update?.status === 'unavailable'"
      type="info"
      show-icon
      :closable="false"
      :title="t('update.unavailable')"
    />

    <section class="app-card p-5" :aria-label="t('update.versionHistory')">
      <div class="mb-4 flex flex-wrap items-center justify-between gap-2">
        <h3 class="app-text m-0 text-base font-semibold">
          {{ t('update.versionHistory') }}
        </h3>
        <span
          v-if="store.update?.candidate"
          class="app-badge app-badge--plain shrink-0"
        >
          {{ t('update.hasUpdate') }}
        </span>
      </div>

      <article
        v-if="store.update?.candidate"
        class="rounded-lg border p-4"
        style="border-color: var(--app-border); background: var(--app-panel-2)"
      >
        <div class="flex flex-wrap items-center justify-between gap-2">
          <div class="flex flex-wrap items-center gap-2">
            <span class="app-text text-sm font-semibold tabular-nums">
              v{{ store.update.candidate.version }}
            </span>
            <span
              v-if="store.update.candidate.isForced"
              class="app-badge app-badge--plain"
              style="color: var(--app-red)"
            >
              {{ t('update.forced') }}
            </span>
          </div>
          <button type="button" class="app-chip" @click="isDialogOpen = true">
            {{ t('update.whatsNew') }}
          </button>
        </div>
        <ul class="app-muted m-0 mt-3 list-disc space-y-1.5 pl-5 text-sm">
          <li v-for="note in store.update.candidate.releaseNotes" :key="note">
            {{ note }}
          </li>
        </ul>
      </article>

      <el-empty
        v-else-if="!store.error"
        :image-size="72"
        :description="t('update.noChangelog')"
      />

      <div class="app-hairline mt-4 flex flex-wrap items-center justify-between gap-2 border-t pt-4">
        <span class="app-faint text-xs">{{ t('update.currentVersion') }}</span>
        <span class="app-text text-sm font-semibold tabular-nums">
          v{{ store.update?.currentVersion ?? '--' }}
        </span>
      </div>
    </section>

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
