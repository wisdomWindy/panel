<script setup lang="ts">
import { computed, onMounted, ref, watch } from 'vue';
import { FolderOpened, Upload, Download, RefreshLeft, Loading } from '@element-plus/icons-vue';
import { ElMessage, ElMessageBox } from 'element-plus';
import { useI18n } from 'vue-i18n';
import type { ShellConfigScope } from '@/platform/desktop';
import { useApplicationShellStore } from '@/stores/application-shell';
import { useLiveSafetyStore } from '@/stores/live-data/safety';
import { useMediaGiftsStore } from '@/stores/media/gifts';
import { useMediaRecordingStore } from '@/stores/media/recording';

const { t, locale } = useI18n();
const shell = useApplicationShellStore();
const safety = useLiveSafetyStore();
const gifts = useMediaGiftsStore();
const recording = useMediaRecordingStore();
const selectedScopes = ref<ShellConfigScope[]>([]);
const availableScopes = computed(() => shell.configOverview?.scopes.filter(scope => scope.isAvailable) ?? []);
const isPending = computed(() => shell.pendingOperations.size > 0);

watch(
  () => shell.configOverview,
  overview => {
    if (overview) selectedScopes.value = overview.scopes.filter(scope => scope.isAvailable).map(scope => scope.id);
  },
  { immediate: true }
);

function formatBackupTime(value: number | null): string {
  return value === null ? t('settings.config.neverBackedUp') : new Intl.DateTimeFormat(locale.value, {
    year: 'numeric', month: 'short', day: 'numeric',
    hour: '2-digit', minute: '2-digit', second: '2-digit'
  }).format(value);
}

function showFailure(): void {
  ElMessage.error(t('settings.feedback.operationFailed'));
}

function refreshAffectedSnapshots(scopes: readonly ShellConfigScope[]): Promise<void[]> {
  const selected = new Set(scopes);
  const refreshes: Promise<void>[] = [];
  if (selected.has('sensitiveWords') || selected.has('blacklist')) refreshes.push(safety.load());
  if (selected.has('giftSounds')) refreshes.push(gifts.refresh());
  if (selected.has('recording')) refreshes.push(recording.refresh());
  return Promise.all(refreshes);
}

async function load(): Promise<void> {
  try {
    await shell.loadConfigOverview();
  } catch {
    // The stable degraded state below keeps native-only actions unavailable on Web.
  }
}

async function openDirectory(): Promise<void> {
  try {
    await shell.openConfigDirectory();
  } catch {
    showFailure();
  }
}

async function exportSelected(): Promise<void> {
  if (selectedScopes.value.length === 0) {
    ElMessage.warning(t('settings.config.selectAtLeastOne'));
    return;
  }
  try {
    const result = await shell.exportConfig(selectedScopes.value);
    if (result) ElMessage.success(t('settings.feedback.exported'));
  } catch {
    showFailure();
  }
}

async function importSelected(): Promise<void> {
  try {
    const result = await shell.importConfig();
    if (result) {
      await Promise.allSettled([
        shell.loadConfigOverview(),
        refreshAffectedSnapshots(result.importedScopes)
      ]);
      ElMessage.success(t('settings.feedback.imported'));
    }
  } catch {
    showFailure();
  }
}

async function resetAll(): Promise<void> {
  try {
    await ElMessageBox.confirm(
      t('settings.config.resetConfirmation'),
      t('settings.config.resetTitle'),
      { type: 'warning', confirmButtonText: t('common.confirm'), cancelButtonText: t('common.cancel') }
    );
  } catch {
    return;
  }
  try {
    const result = await shell.resetConfig();
    await Promise.allSettled([
      shell.loadConfigOverview(),
      refreshAffectedSnapshots(result.resetScopes)
    ]);
    ElMessage.success(t('settings.feedback.reset'));
  } catch {
    showFailure();
  }
}

onMounted(() => { void load(); });
</script>

<template>
  <div class="max-w-4xl">
    <el-alert
      v-if="shell.error && !shell.configOverview"
      :title="t('settings.feedback.desktopUnavailable')"
      type="warning"
      show-icon
      :closable="false"
      class="mb-5"
    />
    <dl class="m-0 grid grid-cols-[9rem_minmax(0,1fr)] gap-x-4 gap-y-3 text-sm">
      <dt class="app-muted">
        {{ t('settings.config.location') }}
      </dt>
      <dd class="app-text m-0 break-all font-medium">
        {{ shell.configOverview?.displayPath ?? '--' }}
      </dd>
      <dt class="app-muted">
        {{ t('settings.config.lastBackup') }}
      </dt>
      <dd class="app-text m-0 font-medium tabular-nums">
        {{ formatBackupTime(shell.configOverview?.lastBackupAt ?? null) }}
      </dd>
    </dl>

    <div class="app-hairline my-5 border-t" />

    <section aria-labelledby="settings-config-export-title">
      <h3 id="settings-config-export-title" class="app-text m-0 text-sm font-semibold">
        {{ t('settings.config.exportScope') }}
      </h3>
      <div class="app-surface-2 app-hairline mt-3 rounded-lg border p-4">
        <el-checkbox-group v-model="selectedScopes" class="grid grid-cols-2 gap-x-6 gap-y-2 lg:grid-cols-3">
          <el-checkbox v-for="scope in availableScopes" :key="scope.id" :value="scope.id">
            {{ t(`settings.config.scopes.${scope.id}`) }}
          </el-checkbox>
        </el-checkbox-group>
      </div>
    </section>

    <div class="app-hairline mt-6 flex flex-wrap items-center gap-2 border-t pt-5">
      <button
        type="button"
        class="app-chip disabled:cursor-not-allowed disabled:opacity-50"
        :disabled="!shell.configOverview || isPending"
        @click="openDirectory"
      >
        <el-icon>
          <FolderOpened />
        </el-icon>
        {{ t('settings.config.openDirectory') }}
      </button>
      <button
        type="button"
        class="app-chip disabled:cursor-not-allowed disabled:opacity-50"
        :aria-busy="shell.pendingOperations.has('exportConfig')"
        :disabled="!shell.configOverview || shell.pendingOperations.has('exportConfig')"
        @click="exportSelected"
      >
        <el-icon v-if="shell.pendingOperations.has('exportConfig')" class="is-loading">
          <Loading />
        </el-icon>
        <el-icon v-else>
          <Download />
        </el-icon>
        {{ t('common.export') }}
      </button>
      <button
        type="button"
        class="app-chip disabled:cursor-not-allowed disabled:opacity-50"
        :aria-busy="shell.pendingOperations.has('importConfig')"
        :disabled="!shell.configOverview || shell.pendingOperations.has('importConfig')"
        @click="importSelected"
      >
        <el-icon v-if="shell.pendingOperations.has('importConfig')" class="is-loading">
          <Loading />
        </el-icon>
        <el-icon v-else>
          <Upload />
        </el-icon>
        {{ t('common.import') }}
      </button>
      <button
        type="button"
        class="app-chip ml-auto disabled:cursor-not-allowed disabled:opacity-50"
        style="color: var(--app-red); border-color: color-mix(in srgb, var(--app-red) 45%, transparent)"
        :aria-busy="shell.pendingOperations.has('resetConfig')"
        :disabled="!shell.configOverview || shell.pendingOperations.has('resetConfig')"
        @click="resetAll"
      >
        <el-icon v-if="shell.pendingOperations.has('resetConfig')" class="is-loading">
          <Loading />
        </el-icon>
        <el-icon v-else>
          <RefreshLeft />
        </el-icon>
        {{ t('settings.config.restoreDefaults') }}
      </button>
    </div>
  </div>
</template>
