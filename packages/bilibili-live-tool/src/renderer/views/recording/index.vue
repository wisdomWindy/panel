<script setup lang="ts">
import { computed, nextTick, onMounted, ref } from 'vue';
import { useI18n } from 'vue-i18n';
import { Setting, Star, VideoCamera, VideoPause } from '@element-plus/icons-vue';
import type {
  HighlightSummary,
  PreviewSession,
  RecordingConfig,
  RecordingSummary
} from '@/platform/desktop';
import RecordingStatusBand from '@/components/media/RecordingStatusBand.vue';
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
const primaryButton = ref<{ $el?: HTMLElement } | null>(null);

const config = computed(() => store.config);
const status = computed(() => store.status);
const isActive = computed(() => status.value?.state === 'recording');
const isTransitioning = computed(() =>
  ['starting', 'stopping', 'finalizing'].includes(status.value?.state ?? '')
);
const startDisabledReason = computed(() => {
  if (!room.isConnected || !room.snapshot.roomId) return t('recording.reasons.connectRoom');
  if (!config.value?.directory) return t('recording.reasons.selectDirectory');
  if (config.value.ffmpegSource === 'bundled') return t('recording.reasons.selectLocalFfmpeg');
  if (
    status.value &&
      status.value.diskRemainingBytes > 0 &&
      status.value.diskRemainingBytes < config.value.waterlineBytes
  ) {
    return t('recording.reasons.lowDisk');
  }
  // An error state must stay retryable: the backend accepts a restart from
  // Error, and the primary button already offers "重试录制".
  return null;
});
const primaryText = computed(() =>
  isActive.value
    ? t('recording.stopRecording')
    : status.value?.state === 'error'
      ? t('recording.retryRecording')
      : t('recording.startRecording')
);

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
    if (isActive.value) {
      await store.stop();
      await store.refresh();
      ElMessage.success(t('recording.messages.stopped'));
      return;
    }
    if (startDisabledReason.value || !room.snapshot.roomId) return;
    await store.start(room.snapshot.roomId);
    ElMessage.success(t('recording.messages.started'));
  } catch (error) {
    ElMessage.error(
      (error as { message?: string }).message ?? t('recording.messages.operationFailed')
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

async function selectDirectory(): Promise<void> {
  if (!settingsDraft.value) return;
  try {
    const directory = await store.selectDirectory();
    settingsDraft.value = { ...settingsDraft.value, directory };
  } catch (error) {
    const value = error as { code?: string; message?: string };
    if (value.code !== 'dialog_cancelled')
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
    if (value.code !== 'dialog_cancelled')
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
    primaryButton.value?.$el?.focus();
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

async function showRecordingPreview(recording: RecordingSummary): Promise<void> {
  try {
    preview.value = await store.previewRecording(recording.id);
    previewTitle.value = recording.fileName;
  } catch (error) {
    ElMessage.error(
      (error as { message?: string }).message ?? t('recording.messages.previewFailed')
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
    if (value.code !== 'dialog_cancelled')
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
</script>

<template>
  <div class="mx-auto w-full max-w-6xl">
    <div class="mb-4 flex flex-wrap items-center justify-between gap-3">
      <h2 class="m-0 text-xl font-semibold text-gray-900 dark:text-zinc-100">
        {{ t('recording.liveTitle') }}
      </h2>
      <div class="flex flex-wrap items-center gap-2">
        <el-tooltip :content="!isActive && startDisabledReason ? startDisabledReason : primaryText">
          <span>
            <el-button
              ref="primaryButton"
              :type="isActive ? 'danger' : 'primary'"
              :icon="isActive ? VideoPause : VideoCamera"
              :loading="store.pendingOperations.has(isActive ? 'stop' : 'start') || isTransitioning"
              :disabled="!isActive && !!startDisabledReason"
              @click="handlePrimary"
            >
              {{ primaryText }}
            </el-button>
          </span>
        </el-tooltip>
        <el-button
          :icon="Star"
          :disabled="!isActive"
          :loading="store.pendingOperations.has('markHighlight')"
          @click="handleMark"
        >
          {{ t('recording.manualMark') }}
        </el-button>
        <el-button :icon="Setting" :disabled="!config || isActive" @click="openSettings">
          {{ t('recording.settingsButton') }}
        </el-button>
      </div>
    </div>

    <el-alert
      v-if="store.error"
      class="mb-4"
      type="error"
      :closable="false"
      :title="store.error.message"
      show-icon
    >
      <el-button size="small" @click="store.refresh()">
        {{ t('common.retry') }}
      </el-button>
    </el-alert>

    <el-skeleton v-if="store.pending && (!config || !status)" :rows="8" animated />
    <template v-else-if="config && status">
      <RecordingStatusBand :status="status" :config="config" />
      <div class="mt-3 flex flex-wrap items-center justify-between gap-2 text-sm text-gray-600 dark:text-zinc-300">
        <span>
          {{ t('recording.highlightShortcut') }}
          <kbd class="rounded border border-gray-300 bg-gray-50 px-2 py-1 font-mono dark:border-zinc-700 dark:bg-zinc-900">
            {{ config.shortcut }}
          </kbd>
        </span>
        <span v-if="startDisabledReason && !isActive" class="text-amber-700 dark:text-amber-400">
          {{ startDisabledReason }}
        </span>
      </div>

      <el-tabs v-model="activeTab" class="mt-4">
        <el-tab-pane :label="t('recording.fileList')" name="recordings">
          <RecordingList
            :recordings="store.recordings"
            :loading="store.pending"
            :pending-operations="store.pendingOperations"
            @preview="showRecordingPreview"
            @rename="renameRecording"
            @delete="deleteRecording"
          />
        </el-tab-pane>
        <el-tab-pane :label="t('recording.highlightClips')" name="highlights">
          <HighlightList
            :highlights="store.highlights"
            :loading="store.pending"
            :pending-operations="store.pendingOperations"
            @preview="showHighlightPreview"
            @export="exportHighlight"
            @delete="deleteHighlight"
          />
        </el-tab-pane>
      </el-tabs>
    </template>

    <RecordingSettingsDialog
      v-if="settingsDraft"
      v-model="settingsDraft"
      :visible="settingsVisible"
      :pending="store.pendingOperations.has('saveConfig')"
      :path-pending="store.pendingOperations.has('selectDirectory')"
      :ffmpeg-pending="store.pendingOperations.has('setFfmpegSource')"
      @update:visible="handleSettingsVisibility"
      @select-directory="selectDirectory"
      @set-ffmpeg-source="setFfmpegSource"
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
        class="max-h-[65vh] w-full bg-black"
      />
    </el-dialog>
  </div>
</template>
