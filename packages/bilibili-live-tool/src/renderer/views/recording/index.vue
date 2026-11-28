<script setup lang="ts">
import { computed, nextTick, onMounted, onUnmounted, ref, watch } from 'vue';
import { useI18n } from 'vue-i18n';
import { Delete, Loading, Setting, Star, VideoCamera, VideoPause } from '@element-plus/icons-vue';
import type {
  HighlightSummary,
  PreviewSession,
  RecordingConfig,
  RecordingSummary
} from '@/platform/desktop';
import { formatBytes, formatDuration } from '@/modules/media/formatters';
import { errorKey, isUserCancellation } from '@/modules/common/error-text';
import { validateFileNameStem } from '@/modules/media/validation';
import { useLiveRoomStore } from '@/stores/live-data/room';
import { useMediaRecordingStore } from '@/stores/media/recording';
import HighlightList from './components/HighlightList.vue';
import RecordingList from './components/RecordingList.vue';
import RecordingSettingsDialog from './components/RecordingSettingsDialog.vue';

const store = useMediaRecordingStore();
const { t } = useI18n();
const room = useLiveRoomStore();
const activeTab = ref<'recordings' | 'highlights'>('recordings');
const settingsVisible = ref(false);
const settingsDraft = ref<RecordingConfig | null>(null);
const preview = ref<PreviewSession | null>(null);
const previewTitle = ref('');
const primaryButton = ref<HTMLButtonElement | null>(null);

const config = computed(() => store.config);
const status = computed(() => store.status);
const isActive = computed(() => status.value?.state === 'recording');
/** Every concurrent recording; the operator can watch several rooms at once. */
const tasks = computed(() => status.value?.tasks ?? []);
const hasActiveTasks = computed(() =>
  tasks.value.some(task =>
    ['starting', 'recording', 'stopping', 'finalizing'].includes(task.state)
  )
);
const isTransitioning = computed(() =>
  ['starting', 'stopping', 'finalizing'].includes(status.value?.state ?? '')
);
/** PRD 9.12 records a room; a live connection is not required to start. */
const manualRoomId = ref('');

const targetRoomId = computed(() => room.snapshot.roomId ?? manualRoomId.value.trim());

const startDisabledReason = computed(() => {
  if (!targetRoomId.value) return t('recording.reasons.roomIdRequired');
  if (!/^\d{1,20}$/.test(targetRoomId.value)) return t('recording.reasons.roomIdInvalid');
  if (
    status.value &&
      status.value.diskRemainingBytes > 0 &&
      status.value.diskRemainingBytes < config.value.waterlineBytes
  ) {
    return t('recording.reasons.lowDisk');
  }
  // A missing save folder or FFmpeg no longer blocks the click: the backend
  // falls back to the app-owned `download` folder, and a missing FFmpeg is
  // reported with an install prompt when the operator actually starts.
  return null;
});
const primaryText = computed(() =>
  status.value?.state === 'error' && !hasActiveTasks.value
    ? t('recording.retryRecording')
    : t('recording.startRecording')
);
const primaryPending = computed(
  () => store.pendingOperations.has('start') || isTransitioning.value
);
const stopAllPending = computed(() => store.pendingOperations.has('stop'));
const markPending = computed(() => store.pendingOperations.has('markHighlight'));
const primaryDisabled = computed(
  () => primaryPending.value || !!startDisabledReason.value
);

const stateText = computed(() => {
  if (!status.value) return '';
  if (tasks.value.length > 1) return `${t('recording.recording')} ×${tasks.value.length}`;
  return t(`recording.${status.value.state}`);
});

/** Localized label for one task's state. */
function taskStateText(state: string): string {
  return t(`recording.${state}`);
}
const stateDotClass = computed(() => {
  switch (status.value?.state) {
    case 'recording':
    case 'error':
      return 'app-status-dot--danger';
    case 'starting':
    case 'stopping':
    case 'finalizing':
      return 'app-status-dot--warning';
    default:
      return 'app-status-dot--success';
  }
});

const elapsedText = computed(() => formatDuration(status.value?.elapsedSeconds ?? 0));
/**
 * Only a running task writes bytes. The runtime keeps the last session's size
 * after it ends, so counting it while idle makes the gauge (and 大小) stick at
 * the size of a file the operator already deleted.
 */
const isSessionActive = computed(() =>
  ['starting', 'recording', 'stopping', 'finalizing'].includes(status.value?.state ?? 'idle')
);
const sessionSizeBytes = computed(() =>
  isSessionActive.value ? (status.value?.fileSizeBytes ?? 0) : 0
);
const fileSizeText = computed(() => formatBytes(sessionSizeBytes.value));
const remainingText = computed(() => {
  const remaining = status.value?.diskRemainingBytes ?? 0;
  return remaining > 0 ? formatBytes(remaining) : '--';
});
const quotaText = computed(() => formatBytes(config.value?.quotaBytes ?? 0));
// The backend reports free volume space instead of quota consumption, so the
// quota gauge is derived from the sizes of the listed recording files.
const storedBytes = computed(() => {
  const finished = store.recordings.reduce((total, recording) => total + recording.sizeBytes, 0);
  // The active recording is still being written, so its size comes from the
  // live status instead of the listed (not yet finalised) row.
  return finished + sessionSizeBytes.value;
});
const quotaPercent = computed(() => {
  const quota = config.value?.quotaBytes ?? 0;
  if (quota <= 0) return 0;
  return Math.max(0, Math.min(100, Math.round((storedBytes.value / quota) * 100)));
});
const diskUsageText = computed(() => `${formatBytes(storedBytes.value)} / ${quotaText.value}`);
const lowDisk = computed(
  () =>
    (status.value?.diskRemainingBytes ?? 0) > 0 &&
    !!config.value &&
    (status.value?.diskRemainingBytes ?? 0) < config.value.waterlineBytes
);

/** Localised text for a backend error, falling back to its own message. */
function describeError(code: string | null | undefined, fallback: string): string {
  const key = errorKey(code);
  return key ? t(key) : fallback;
}

const qualityText = computed(() => {
  const quality = config.value?.quality ?? 'original';
  const qualityKey =
    quality === 'original' ? 'qualityOriginal' : quality === 'high' ? 'qualityHigh' : 'qualityStandard';
  return t(`recording.settings.${qualityKey}`);
});
const ffmpegSourceText = computed(() => {
  const ffmpegSource = config.value?.ffmpegSource ?? 'bundled';
  return t(`recording.settings.${ffmpegSource}`);
});

const tabs = computed(() => [
  { id: 'recordings' as const, label: t('recording.fileList'), count: store.recordings.length },
  { id: 'highlights' as const, label: t('recording.highlightList'), count: store.highlights.length }
]);
const metricCards = computed(() => [
  { id: 'duration', label: t('recording.duration'), value: elapsedText.value },
  { id: 'fileSize', label: t('recording.fileSize'), value: fileSizeText.value },
  { id: 'remaining', label: t('recording.remainingSpace'), value: remainingText.value },
  { id: 'quota', label: t('recording.quota'), value: quotaText.value },
  {
    id: 'highlights',
    label: t('recording.highlightCount'),
    value: String(status.value?.highlightCount ?? 0)
  }
]);

function cloneConfig(value: RecordingConfig): RecordingConfig {
  return { ...value, directory: value.directory ? { ...value.directory } : null };
}

function openSettings(): void {
  if (!config.value) return;
  settingsDraft.value = cloneConfig(config.value);
  settingsVisible.value = true;
}

async function handlePrimary(): Promise<void> {
  try {
    if (startDisabledReason.value || !targetRoomId.value) return;
    await store.start(targetRoomId.value);
    ElMessage.success(t('recording.messages.started'));
  } catch (error) {
    const value = error as { code?: string; message?: string };
    if (value.code === 'RECORD_FFMPEG_MISSING') {
      await promptInstallFfmpeg();
      return;
    }
    ElMessage.error(
      describeError(value.code, value.message ?? t('recording.messages.operationFailed'))
    );
  }
}

/**
 * PRD 9.12: recording needs FFmpeg. Missing it is an install problem, so the
 * prompt offers the one-click install that stays inside the app's own folder.
 */
async function promptInstallFfmpeg(): Promise<void> {
  try {
    await ElMessageBox.confirm(
      t('recording.ffmpegPrompt.body'),
      t('recording.ffmpegPrompt.title'),
      {
        type: 'warning',
        confirmButtonText: t('recording.ffmpegPrompt.installNow'),
        cancelButtonText: t('recording.ffmpegPrompt.openSettings'),
        distinguishCancelAndClose: true
      }
    );
    await installFfmpeg();
  } catch (action) {
    // Cancel means "take me to the settings"; close means "not now".
    if (action === 'cancel') openSettings();
  }
}

/** Downloads FFmpeg into the app data directory and switches the source to it. */
async function installFfmpeg(): Promise<void> {
  const pending = ElMessage({ message: t('recording.settings.installing'), duration: 0 });
  try {
    await store.installFfmpeg();
    pending.close();
    ElMessage.success(t('recording.messages.ffmpegInstalled'));
    if (settingsDraft.value) {
      settingsDraft.value = { ...settingsDraft.value, ffmpegSource: 'installed' };
    }
  } catch (error) {
    pending.close();
    const value = error as { code?: string; message?: string };
    ElMessage.error(
      describeError(value.code, value.message ?? t('recording.messages.ffmpegInstallFailed'))
    );
  }
}

async function handleStopAll(): Promise<void> {
  try {
    await store.stop();
    await store.refresh();
    ElMessage.success(t('recording.messages.stopped'));
  } catch (error) {
    const value = error as { code?: string; message?: string };
    ElMessage.error(
      describeError(value.code, value.message ?? t('recording.messages.operationFailed'))
    );
  }
}
async function handleMark(): Promise<void> {
  try {
    await store.markHighlight();
    activeTab.value = 'highlights';
    ElMessage.success(t('recording.messages.highlightMarked'));
  } catch (error) {
    ElMessage.error(
      (error as { message?: string }).message ?? t('recording.messages.markFailed')
    );
  }
}

/** Localized text for a backend error code (unknown codes stay readable). */
function describeCode(code: string | null | undefined): string {
  if (!code) return '';
  const key = errorKey(code);
  return key ? t(key) : code;
}

async function stopOneTask(taskId: number): Promise<void> {
  try {
    await store.stopTask(taskId);
    ElMessage.success(t('recording.messages.stopped'));
  } catch (error) {
    const value = error as { code?: string; message?: string };
    ElMessage.error(
      describeError(value.code, value.message ?? t('recording.messages.operationFailed'))
    );
  }
}

async function markTask(taskId: number): Promise<void> {
  try {
    await store.markHighlightTask(taskId);
    activeTab.value = 'highlights';
    ElMessage.success(t('recording.messages.highlightMarked'));
  } catch (error) {
    const value = error as { code?: string; message?: string };
    ElMessage.error(
      describeError(value.code, value.message ?? t('recording.messages.markFailed'))
    );
  }
}
const cleanupPending = computed(() => store.pendingOperations.has('cleanup'));

/** Removes records/files the operator cannot use any more. */
async function handleCleanup(): Promise<void> {
  try {
    await ElMessageBox.confirm(t('recording.cleanupBody'), t('recording.cleanupTitle'), {
      type: 'warning',
      confirmButtonText: t('recording.cleanupConfirm'),
      cancelButtonText: t('common.cancel')
    });
  } catch {
    return;
  }
  try {
    const report = await store.cleanupLeftovers();
    const removed = report.removedRecords + report.removedFiles + report.removedHighlights;
    if (removed === 0) {
      ElMessage.success(t('recording.cleanupNone'));
      return;
    }
    ElMessage.success(
      t('recording.cleanupDone', {
        records: report.removedRecords,
        files: report.removedFiles,
        size: formatBytes(report.freedBytes)
      })
    );
  } catch (error) {
    const value = error as { code?: string; message?: string };
    ElMessage.error(
      describeError(value.code, value.message ?? t('recording.messages.operationFailed'))
    );
  }
}
/** Inline picker: grants the folder and persists it without opening the dialog. */
async function pickDirectoryInline(): Promise<void> {
  const current = config.value;
  if (!current) return;
  try {
    const directory = await store.selectDirectory();
    await store.saveConfig({ ...current, directory });
    ElMessage.success(t('recording.messages.directorySaved'));
  } catch (error) {
    const value = error as { code?: string; message?: string };
    if (value.code !== 'SYS_DIALOG_CANCELLED')
      ElMessage.error(value.message ?? t('recording.messages.directoryFailed'));
  }
}

async function selectDirectory(): Promise<void> {
  if (!settingsDraft.value) return;
  try {
    const directory = await store.selectDirectory();
    settingsDraft.value = { ...settingsDraft.value, directory };
  } catch (error) {
    const value = error as { code?: string; message?: string };
    if (value.code !== 'SYS_DIALOG_CANCELLED')
      ElMessage.error(value.message ?? t('recording.messages.directoryFailed'));
  }
}

async function setFfmpegSource(source: RecordingConfig['ffmpegSource']): Promise<void> {
  if (!settingsDraft.value) return;
  try {
    const saved = await store.setFfmpegSource(source);
    settingsDraft.value = { ...settingsDraft.value, ffmpegSource: saved.ffmpegSource };
  } catch (error) {
    const value = error as { code?: string; message?: string };
    if (value.code !== 'SYS_DIALOG_CANCELLED')
      ElMessage.error(value.message ?? t('recording.messages.ffmpegFailed'));
  }
}

async function saveSettings(): Promise<void> {
  if (!settingsDraft.value) return;
  try {
    const saved = await store.saveConfig(cloneConfig(settingsDraft.value));
    settingsDraft.value = cloneConfig(saved);
    settingsVisible.value = false;
    ElMessage.success(t('recording.messages.saved'));
    await nextTick();
    primaryButton.value?.focus();
  } catch (error) {
    ElMessage.error((error as { message?: string }).message ?? t('common.saveFailed'));
  }
}

function handleSettingsVisibility(visible: boolean): void {
  settingsVisible.value = visible;
  if (!visible) {
    void store.discardDirectorySelection();
    if (config.value) settingsDraft.value = cloneConfig(config.value);
  }
}

async function openRecordingFolder(recording: RecordingSummary): Promise<void> {
  try {
    await store.openRecordingFolder(recording.id);
  } catch (error) {
    ElMessage.error(
      (error as { message?: string }).message ?? t('recording.messages.folderFailed')
    );
  }
}

async function showHighlightPreview(highlight: HighlightSummary): Promise<void> {
  try {
    preview.value = await store.previewHighlight(highlight.id);
    previewTitle.value = `${t('recording.highlightList')} ${highlight.positionSeconds}s`;
  } catch (error) {
    ElMessage.error(
      (error as { message?: string }).message ?? t('recording.messages.previewFailed')
    );
  }
}

async function renameRecording(recording: RecordingSummary): Promise<void> {
  const extension = `.${recording.format}`;
  const initial = recording.fileName.endsWith(extension)
    ? recording.fileName.slice(0, -extension.length)
    : recording.fileName;
  try {
    const result = await ElMessageBox.prompt(
      t('recording.dialogs.renamePrompt'),
      t('recording.dialogs.renameTitle', { fileName: recording.fileName }),
      {
        inputValue: initial,
        inputValidator: value => validateFileNameStem(value).message ?? true,
        confirmButtonText: t('recording.rename'),
        cancelButtonText: t('common.cancel')
      }
    );
    await store.renameRecording(recording.id, result.value.trim());
    ElMessage.success(t('recording.messages.renamed'));
  } catch (error) {
    if (error !== 'cancel' && error !== 'close')
      ElMessage.error(
        (error as { message?: string }).message ?? t('recording.messages.renameFailed')
      );
  }
}

async function deleteRecording(recording: RecordingSummary): Promise<void> {
  try {
    await ElMessageBox.confirm(
      t('recording.dialogs.deleteRecordingConfirm', { fileName: recording.fileName }),
      t('recording.dialogs.deleteRecordingTitle'),
      {
        type: 'warning',
        confirmButtonText: t('common.delete'),
        cancelButtonText: t('common.cancel')
      }
    );
    await store.deleteRecording(recording.id);
    ElMessage.success(t('recording.messages.deleted'));
  } catch (error) {
    if (error !== 'cancel' && error !== 'close')
      ElMessage.error(
        (error as { message?: string }).message ?? t('recording.messages.deleteFailed')
      );
  }
}

async function exportHighlight(highlight: HighlightSummary): Promise<void> {
  try {
    await store.exportHighlight(highlight.id);
    ElMessage.success(t('recording.messages.highlightExported'));
  } catch (error) {
    const value = error as { code?: string; message?: string };
    if (value.code !== 'SYS_DIALOG_CANCELLED')
      ElMessage.error(value.message ?? t('recording.messages.exportFailed'));
  }
}

async function deleteHighlight(highlight: HighlightSummary): Promise<void> {
  try {
    await ElMessageBox.confirm(
      t('recording.dialogs.deleteHighlightConfirm', { seconds: highlight.positionSeconds }),
      t('recording.dialogs.deleteHighlightTitle'),
      {
        type: 'warning',
        confirmButtonText: t('common.delete'),
        cancelButtonText: t('common.cancel')
      }
    );
    await store.deleteHighlight(highlight.id);
    ElMessage.success(t('recording.messages.highlightDeleted'));
  } catch (error) {
    if (error !== 'cancel' && error !== 'close')
      ElMessage.error(
        (error as { message?: string }).message ?? t('recording.messages.deleteFailed')
      );
  }
}

function closePreview(): void {
  preview.value = null;
  previewTitle.value = '';
}

onMounted(async () => {
  try {
    await store.load();
  } catch {
    // The stable error region below owns retry feedback.
  }
});

/**
 * The backend only pushes status on transitions, so a running recording would
 * otherwise show a frozen 00:00:00 and 0 B. Poll once per second while active.
 */
let statusTimer: ReturnType<typeof setInterval> | null = null;

function stopStatusPolling(): void {
  if (statusTimer !== null) {
    clearInterval(statusTimer);
    statusTimer = null;
  }
}

function startStatusPolling(): void {
  if (statusTimer !== null) return;
  statusTimer = setInterval(() => {
    if (!isActive.value) {
      stopStatusPolling();
      return;
    }
    void store.refreshStatus().catch(() => undefined);
  }, 1000);
}

watch(isActive, active => (active ? startStatusPolling() : stopStatusPolling()), {
  immediate: true
});

onUnmounted(stopStatusPolling);
</script>

<template>
  <section class="mx-auto flex w-full max-w-[1180px] flex-col gap-4">
    <header class="flex flex-wrap items-center gap-2">
      <div class="mr-auto min-w-0">
        <h2 class="app-text m-0 text-lg font-semibold">{{ t('recording.liveTitle') }}</h2>
        <p class="app-muted mt-1 mb-0 text-sm">{{ t('recording.pageHint') }}</p>
      </div>
    </header>

    <el-alert
      v-if="store.error && !isUserCancellation(store.error.code)"
      type="error"
      :closable="false"
      :title="describeError(store.error.code, store.error.message)"
      show-icon
    >
      <el-button size="small" @click="store.refresh()">{{ t('common.retry') }}</el-button>
    </el-alert>

    <el-skeleton v-if="store.pending && (!config || !status)" :rows="8" animated />
    <template v-else-if="config && status">
      <div class="grid grid-cols-1 gap-4 xl:grid-cols-[minmax(0,2fr)_minmax(0,1fr)]">
        <section class="app-card p-5" :aria-label="t('recording.recordingStatus')">
          <h3 class="app-text m-0 text-sm font-semibold">{{ t('recording.recordingStatus') }}</h3>
          <div class="mt-3" aria-live="polite">
            <div class="flex items-center gap-2">
              <span class="app-status-dot" :class="stateDotClass" aria-hidden="true" />
              <p class="app-text m-0 text-3xl font-bold leading-tight">{{ stateText }}</p>
            </div>
            <p class="app-muted mt-2 mb-0 text-sm">
              {{ t('recording.sessionDuration') }}
              <span class="app-text ml-1 font-mono tabular-nums">{{ elapsedText }}</span>
            </p>
          </div>
          <div class="mt-5">
            <div class="flex flex-wrap items-baseline justify-between gap-2">
              <span class="app-muted text-sm">{{ t('recording.diskUsage') }}</span>
              <span class="app-text text-sm tabular-nums">{{ diskUsageText }}</span>
            </div>
            <div
              class="mt-2 h-2 w-full overflow-hidden rounded-full"
              style="background: var(--app-panel-2)"
              role="progressbar"
              :aria-label="t('recording.diskUsage')"
              aria-valuemin="0"
              aria-valuemax="100"
              :aria-valuenow="quotaPercent"
            >
              <div
                class="h-full rounded-full"
                :style="{ width: `${quotaPercent}%`, background: 'var(--app-accent)' }"
              />
            </div>
            <p v-if="lowDisk" class="mt-2 mb-0 text-xs" style="color: var(--app-gold)">
              {{ t('recording.reasons.lowDisk') }}
            </p>
          </div>
          <div v-if="!room.isConnected" class="mt-5">
            <label class="app-muted mb-1 block text-xs" for="recording-room-id">
              {{ t('recording.manualRoomId') }}
            </label>
            <el-input
              id="recording-room-id"
              v-model="manualRoomId"
              class="!w-56"
              :maxlength="20"
              :placeholder="t('recording.manualRoomIdHint')"
              :aria-label="t('recording.manualRoomId')"
            />
          </div>
          <div class="mt-5 flex flex-wrap items-center gap-2">
            <el-tooltip
              :content="!isActive && startDisabledReason ? startDisabledReason : primaryText"
            >
              <span class="inline-flex">
                <button
                  ref="primaryButton"
                  type="button"
                  class="app-btn-primary inline-flex h-9 items-center gap-1.5 px-4 text-sm disabled:cursor-not-allowed disabled:opacity-60"
                  :aria-busy="primaryPending"
                  :disabled="primaryDisabled"
                  @click="handlePrimary"
                >
                  <el-icon v-if="primaryPending" class="is-loading"><Loading /></el-icon>
                  <el-icon v-else><VideoCamera /></el-icon>
                  {{ primaryText }}
                </button>
              </span>
            </el-tooltip>
            <button
              v-if="hasActiveTasks"
              type="button"
              class="app-chip disabled:cursor-not-allowed disabled:opacity-50"
              :aria-busy="stopAllPending"
              :disabled="stopAllPending"
              @click="handleStopAll"
            >
              <el-icon v-if="stopAllPending" class="is-loading"><Loading /></el-icon>
              <el-icon v-else><VideoPause /></el-icon>
              {{ t('recording.stopAll') }}
            </button>
            <button
              type="button"
              class="app-chip disabled:cursor-not-allowed disabled:opacity-50"
              :aria-busy="markPending"
              :disabled="!isActive || markPending"
              @click="handleMark"
            >
              <el-icon v-if="markPending" class="is-loading"><Loading /></el-icon>
              <el-icon v-else><Star /></el-icon>
              {{ t('recording.manualMark') }} · {{ config.shortcut }}
            </button>
          </div>
          <p
            v-if="startDisabledReason && !isActive"
            class="mt-3 mb-0 text-xs"
            style="color: var(--app-gold)"
          >
            {{ startDisabledReason }}
          </p>
          <p v-if="status.lastErrorCode" class="mt-2 mb-0 text-xs" style="color: var(--app-red)">
            {{ t('recording.lastError') }}: {{ describeCode(status.lastErrorCode) }}
          </p>
        </section>

        <section class="app-card p-5">
          <h3 class="app-text m-0 text-base font-semibold">{{ t('recording.config') }}</h3>
          <dl class="m-0 mt-3 grid grid-cols-1 gap-2.5 text-sm">
            <div class="flex min-w-0 items-center justify-between gap-3">
              <dt class="app-muted shrink-0">{{ t('recording.settings.directory') }}</dt>
              <dd class="m-0 flex min-w-0 items-center gap-2">
                <span
                  class="app-text min-w-0 truncate text-right"
                  :title="config.directory?.displayPath ?? ''"
                >
                  {{ config.directory?.displayPath ?? t('common.notSelected') }}
                </span>
                <!-- PRD 9.12: the folder must be pickable without opening the dialog. -->
                <button
                  type="button"
                  class="app-chip h-7 shrink-0 px-2 text-xs"
                  :disabled="store.pendingOperations.has('selectDirectory')"
                  @click="pickDirectoryInline"
                >
                  {{ t('recording.settings.selectDirectory') }}
                </button>
              </dd>
            </div>
            <div class="flex items-center justify-between gap-3">
              <dt class="app-muted">{{ t('recording.format') }}</dt>
              <dd class="app-text m-0 font-medium">{{ config.format.toUpperCase() }}</dd>
            </div>
            <div class="flex items-center justify-between gap-3">
              <dt class="app-muted">{{ t('recording.quality') }}</dt>
              <dd class="app-text m-0 font-medium">{{ qualityText }}</dd>
            </div>
            <div class="flex items-center justify-between gap-3">
              <dt class="app-muted">{{ t('recording.settings.ffmpegSource') }}</dt>
              <dd class="app-text m-0 font-medium">{{ ffmpegSourceText }}</dd>
            </div>
            <div class="flex items-center justify-between gap-3">
              <dt class="app-muted">{{ t('recording.settings.stopWaterline') }}</dt>
              <dd class="app-text m-0 font-medium tabular-nums">
                {{ formatBytes(config.waterlineBytes) }}
              </dd>
            </div>
          </dl>
          <p class="app-faint mt-3 mb-0 text-xs">{{ t('recording.ffmpegLicenseHint') }}</p>
          <div class="mt-4 flex flex-wrap items-center gap-2">
            <button
              type="button"
              class="app-chip"
              @click="openSettings"
            >
              <el-icon><Setting /></el-icon>
              {{ t('recording.settingsButton') }}
            </button>
          </div>
        </section>
      </div>

      <div class="grid grid-cols-2 gap-4 md:grid-cols-3 xl:grid-cols-5">
        <article v-for="card in metricCards" :key="card.id" class="app-metric">
          <p class="app-metric-label m-0">{{ card.label }}</p>
          <p class="app-metric-value mt-1.5 mb-0">{{ card.value }}</p>
        </article>
      </div>

      <!-- Each 开始录制 adds one task; every task records its own room. -->
      <section v-if="tasks.length" class="app-card p-4">
        <h3 class="app-text m-0 text-base font-semibold">
          {{ t('recording.tasks') }}
          <span class="app-faint tabular-nums">{{ tasks.length }}</span>
        </h3>
        <div class="mt-3 flex flex-col gap-2">
          <div
            v-for="task in tasks"
            :key="task.id"
            class="flex flex-wrap items-center gap-x-4 gap-y-2 rounded-lg px-3 py-2"
            style="background: var(--app-panel-2)"
          >
            <span class="app-text text-sm font-semibold tabular-nums">
              {{ t('recording.room') }} {{ task.roomId ?? '--' }}
            </span>
            <span class="app-muted text-xs">{{ taskStateText(task.state) }}</span>
            <span class="app-muted text-xs tabular-nums">
              {{ formatDuration(task.elapsedSeconds) }}
            </span>
            <span class="app-muted text-xs tabular-nums">
              {{ formatBytes(task.fileSizeBytes) }}
            </span>
            <span v-if="task.lastErrorCode" class="text-xs" style="color: var(--app-red)">
              {{ describeCode(task.lastErrorCode) }}
            </span>
            <div class="ml-auto flex flex-wrap items-center gap-2">
              <button
                type="button"
                class="app-chip disabled:cursor-not-allowed disabled:opacity-50"
                :disabled="task.state !== 'recording' || markPending"
                @click="markTask(task.id)"
              >
                <el-icon><Star /></el-icon>
                {{ t('recording.manualMark') }}
              </button>
              <button
                type="button"
                class="app-chip disabled:cursor-not-allowed disabled:opacity-50"
                :disabled="!['starting', 'recording'].includes(task.state) || primaryPending"
                @click="stopOneTask(task.id)"
              >
                {{ t('recording.stopTask') }}
              </button>
            </div>
          </div>
        </div>
      </section>

      <section class="app-card p-4">
        <div class="flex flex-wrap items-center gap-1.5">
          <button
            v-for="tab in tabs"
            :key="tab.id"
            type="button"
            class="app-chip"
            :class="{ 'app-chip--active': activeTab === tab.id }"
            :aria-pressed="activeTab === tab.id"
            @click="activeTab = tab.id"
          >
            {{ tab.label }}
            <span class="app-faint tabular-nums">{{ tab.count }}</span>
          </button>
          <button
            type="button"
            class="app-chip ml-auto disabled:cursor-not-allowed disabled:opacity-50"
            :aria-busy="cleanupPending"
            :disabled="cleanupPending"
            @click="handleCleanup"
          >
            <el-icon v-if="cleanupPending" class="is-loading"><Loading /></el-icon>
            <el-icon v-else><Delete /></el-icon>
            {{ t('recording.cleanup') }}
          </button>
        </div>
        <RecordingList
          v-if="activeTab === 'recordings'"
          class="mt-3"
          :recordings="store.recordings"
          :loading="store.pending"
          :pending-operations="store.pendingOperations"
          @open-folder="openRecordingFolder"
          @rename="renameRecording"
          @delete="deleteRecording"
        />
        <HighlightList
          v-else
          class="mt-3"
          :highlights="store.highlights"
          :loading="store.pending"
          :pending-operations="store.pendingOperations"
          @preview="showHighlightPreview"
          @export="exportHighlight"
          @delete="deleteHighlight"
        />
      </section>
    </template>

    <RecordingSettingsDialog
      v-if="settingsDraft"
      v-model="settingsDraft"
      :visible="settingsVisible"
      :pending="store.pendingOperations.has('saveConfig')"
      :path-pending="store.pendingOperations.has('selectDirectory')"
      :ffmpeg-pending="store.pendingOperations.has('setFfmpegSource')"
      :install-pending="store.pendingOperations.has('installFfmpeg')"
      :locked="isActive"
      @update:visible="handleSettingsVisibility"
      @select-directory="selectDirectory"
      @set-ffmpeg-source="setFfmpegSource"
      @install-ffmpeg="installFfmpeg"
      @save="saveSettings"
    />

    <el-dialog
      :model-value="!!preview"
      :title="previewTitle"
      width="min(860px, calc(100vw - 32px))"
      destroy-on-close
      @close="closePreview"
    >
      <video
        v-if="preview"
        :src="preview.url"
        controls
        autoplay
        class="max-h-[65vh] w-full rounded-lg bg-black"
      />
    </el-dialog>
  </section>
</template>
