<script setup lang="ts">
import { computed, nextTick, onMounted, ref } from 'vue';
import { Setting, Star, VideoCamera, VideoPause } from '@element-plus/icons-vue';
import type { HighlightSummary, PreviewSession, RecordingConfig, RecordingSummary } from '@/platform/desktop';
import RecordingStatusBand from '@/components/media/RecordingStatusBand.vue';
import { validateFileNameStem } from '@/modules/media/validation';
import { useLiveRoomStore } from '@/stores/live-data/room';
import { useMediaRecordingStore } from '@/stores/media/recording';
import HighlightList from './components/HighlightList.vue';
import RecordingList from './components/RecordingList.vue';
import RecordingSettingsDialog from './components/RecordingSettingsDialog.vue';

const store = useMediaRecordingStore();
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
const isTransitioning = computed(() => ['starting', 'stopping', 'finalizing'].includes(status.value?.state ?? ''));
const startDisabledReason = computed(() => {
  if (!room.isConnected || !room.snapshot.roomId) return '请先连接已开播的直播间';
  if (!config.value?.directory) return '请先选择录制存储目录';
  if (config.value.ffmpegSource === 'bundled') return '内置 FFmpeg 尚未提供，请在设置中选择本机 FFmpeg';
  if (status.value && status.value.diskRemainingBytes > 0 && status.value.diskRemainingBytes < config.value.waterlineBytes) {
    return '磁盘剩余空间低于停止水位';
  }
  if (status.value?.state === 'error') return '录制状态异常，请检查 FFmpeg 路径';
  return null;
});
const primaryText = computed(() => isActive.value ? '停止录制' : status.value?.state === 'error' ? '重试录制' : '开始录制');

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
      ElMessage.success('录制已停止');
      return;
    }
    if (startDisabledReason.value || !room.snapshot.roomId) return;
    await store.start(room.snapshot.roomId);
    ElMessage.success('已开始录制');
  } catch (error) {
    ElMessage.error((error as { message?: string }).message ?? '录制操作失败');
  }
}

async function handleMark(): Promise<void> {
  try {
    await store.markHighlight();
    activeTab.value = 'highlights';
    ElMessage.success('高光已标记');
  } catch (error) {
    ElMessage.error((error as { message?: string }).message ?? '标记失败');
  }
}

async function selectDirectory(): Promise<void> {
  if (!settingsDraft.value) return;
  try {
    const directory = await store.selectDirectory();
    settingsDraft.value = { ...settingsDraft.value, directory };
  } catch (error) {
    const value = error as { code?: string; message?: string };
    if (value.code !== 'dialog_cancelled') ElMessage.error(value.message ?? '目录选择失败');
  }
}

async function setFfmpegSource(source: RecordingConfig['ffmpegSource']): Promise<void> {
  if (!settingsDraft.value) return;
  try {
    const saved = await store.setFfmpegSource(source);
    settingsDraft.value = { ...settingsDraft.value, ffmpegSource: saved.ffmpegSource };
  } catch (error) {
    const value = error as { code?: string; message?: string };
    if (value.code !== 'dialog_cancelled') ElMessage.error(value.message ?? 'FFmpeg 来源设置失败');
  }
}

async function saveSettings(): Promise<void> {
  if (!settingsDraft.value) return;
  try {
    const saved = await store.saveConfig(cloneConfig(settingsDraft.value));
    settingsDraft.value = cloneConfig(saved);
    settingsVisible.value = false;
    ElMessage.success('录制设置已保存');
    await nextTick();
    primaryButton.value?.$el?.focus();
  } catch (error) {
    ElMessage.error((error as { message?: string }).message ?? '保存失败');
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
    ElMessage.error((error as { message?: string }).message ?? '预览失败');
  }
}

async function showHighlightPreview(highlight: HighlightSummary): Promise<void> {
  try {
    preview.value = await store.previewHighlight(highlight.id);
    previewTitle.value = `高光 ${highlight.positionSeconds}s`;
  } catch (error) {
    ElMessage.error((error as { message?: string }).message ?? '预览失败');
  }
}

async function renameRecording(recording: RecordingSummary): Promise<void> {
  const extension = `.${recording.format}`;
  const initial = recording.fileName.endsWith(extension) ? recording.fileName.slice(0, -extension.length) : recording.fileName;
  try {
    const result = await ElMessageBox.prompt('输入新文件名', `重命名 ${recording.fileName}`, {
      inputValue: initial,
      inputValidator: value => validateFileNameStem(value).message ?? true,
      confirmButtonText: '重命名',
      cancelButtonText: '取消'
    });
    await store.renameRecording(recording.id, result.value.trim());
    ElMessage.success('录制文件已重命名');
  } catch (error) {
    if (error !== 'cancel' && error !== 'close') ElMessage.error((error as { message?: string }).message ?? '重命名失败');
  }
}

async function deleteRecording(recording: RecordingSummary): Promise<void> {
  try {
    await ElMessageBox.confirm(`确定删除录制文件“${recording.fileName}”？`, '删除录制', {
      type: 'warning', confirmButtonText: '删除', cancelButtonText: '取消'
    });
    await store.deleteRecording(recording.id);
    ElMessage.success('录制文件已删除');
  } catch (error) {
    if (error !== 'cancel' && error !== 'close') ElMessage.error((error as { message?: string }).message ?? '删除失败');
  }
}

async function exportHighlight(highlight: HighlightSummary): Promise<void> {
  try {
    await store.exportHighlight(highlight.id);
    ElMessage.success('高光片段已导出');
  } catch (error) {
    const value = error as { code?: string; message?: string };
    if (value.code !== 'dialog_cancelled') ElMessage.error(value.message ?? '导出失败');
  }
}

async function deleteHighlight(highlight: HighlightSummary): Promise<void> {
  try {
    await ElMessageBox.confirm(`确定删除 ${highlight.positionSeconds}s 处的高光标记？`, '删除高光', {
      type: 'warning', confirmButtonText: '删除', cancelButtonText: '取消'
    });
    await store.deleteHighlight(highlight.id);
    ElMessage.success('高光已删除');
  } catch (error) {
    if (error !== 'cancel' && error !== 'close') ElMessage.error((error as { message?: string }).message ?? '删除失败');
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
      <h2 class="m-0 text-xl font-semibold text-gray-900">
        直播录制
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
          手动标记
        </el-button>
        <el-button :icon="Setting" :disabled="!config || isActive" @click="openSettings">
          设置
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
        重试
      </el-button>
    </el-alert>

    <el-skeleton v-if="store.pending && (!config || !status)" :rows="8" animated />
    <template v-else-if="config && status">
      <RecordingStatusBand :status="status" :config="config" />
      <div class="mt-3 flex flex-wrap items-center justify-between gap-2 text-sm text-gray-600">
        <span>高光快捷键：<kbd class="rounded border border-gray-300 bg-gray-50 px-2 py-1 font-mono">{{ config.shortcut }}</kbd></span>
        <span v-if="startDisabledReason && !isActive" class="text-amber-700">{{ startDisabledReason }}</span>
      </div>

      <el-tabs v-model="activeTab" class="mt-4">
        <el-tab-pane label="录制文件" name="recordings">
          <RecordingList
            :recordings="store.recordings"
            :loading="store.pending"
            :pending-operations="store.pendingOperations"
            @preview="showRecordingPreview"
            @rename="renameRecording"
            @delete="deleteRecording"
          />
        </el-tab-pane>
        <el-tab-pane label="高光片段" name="highlights">
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
