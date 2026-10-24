<script setup lang="ts">
import { computed } from 'vue';
import { useI18n } from 'vue-i18n';
import { FolderOpened } from '@element-plus/icons-vue';
import type { RecordingConfig } from '@/platform/desktop';
import { formatBytes } from '@/modules/media/formatters';

const props = defineProps<{
    visible: boolean;
    modelValue: RecordingConfig;
    pending?: boolean;
    pathPending?: boolean;
    ffmpegPending?: boolean;
  }>();
const { t } = useI18n();

const emit = defineEmits<{
    'update:visible': [visible: boolean];
    'update:modelValue': [config: RecordingConfig];
    save: [];
    selectDirectory: [];
    setFfmpegSource: [source: RecordingConfig['ffmpegSource']];
  }>();

const config = computed({
  get: () => props.modelValue,
  set: value => emit('update:modelValue', value)
});

function update<K extends keyof RecordingConfig>(key: K, value: RecordingConfig[K]): void {
  config.value = { ...config.value, [key]: value };
}

function close(): void {
  emit('update:visible', false);
}
</script>

<template>
  <el-dialog
    :model-value="visible"
    :title="t('recording.settings.title')"
    width="min(720px, calc(100vw - 32px))"
    destroy-on-close
    @close="close"
  >
    <div class="max-h-[calc(100vh-190px)] overflow-y-auto pr-2">
      <el-form label-position="top" @submit.prevent="emit('save')">
        <el-divider content-position="left">
          {{ t('recording.settings.encoding') }}
        </el-divider>
        <div class="grid grid-cols-1 gap-x-5 md:grid-cols-2">
          <el-form-item :label="t('recording.format')">
            <el-select
              :model-value="config.format"
              class="w-full"
              @update:model-value="update('format', $event)"
            >
              <el-option label="MP4" value="mp4" />
              <el-option label="FLV" value="flv" />
            </el-select>
          </el-form-item>
          <el-form-item :label="t('recording.quality')">
            <el-select
              :model-value="config.quality"
              class="w-full"
              @update:model-value="update('quality', $event)"
            >
              <el-option :label="t('recording.settings.qualityOriginal')" value="original" />
              <el-option :label="t('recording.settings.qualityHigh')" value="high" />
              <el-option :label="t('recording.settings.qualityStandard')" value="standard" />
            </el-select>
          </el-form-item>
        </div>
        <el-form-item :label="t('recording.settings.directory')">
          <div class="flex w-full min-w-0 items-center gap-2">
            <el-input
              :model-value="config.directory?.displayPath ?? ''"
              readonly
              :placeholder="t('common.notSelected')"
            />
            <el-tooltip :content="t('recording.settings.selectDirectory')">
              <el-button
                :icon="FolderOpened"
                circle
                :loading="pathPending"
                :aria-label="t('recording.settings.selectDirectory')"
                @click="emit('selectDirectory')"
              />
            </el-tooltip>
          </div>
        </el-form-item>

        <el-divider content-position="left">
          {{ t('recording.settings.segmentation') }}
        </el-divider>
        <div class="grid grid-cols-1 gap-x-5 md:grid-cols-2">
          <el-form-item :label="t('recording.segmentEnabled')">
            <el-switch
              :model-value="config.segmentationEnabled"
              @update:model-value="update('segmentationEnabled', $event)"
            />
          </el-form-item>
          <el-form-item :label="t('recording.settings.segmentDuration')">
            <el-input-number
              :model-value="config.segmentDurationSeconds"
              :min="60"
              :max="86400"
              :disabled="!config.segmentationEnabled"
              @update:model-value="update('segmentDurationSeconds', $event ?? 3600)"
            />
          </el-form-item>
          <el-form-item :label="t('recording.settings.autoRecord')">
            <el-switch
              :model-value="config.autoRecord"
              @update:model-value="update('autoRecord', $event)"
            />
          </el-form-item>
        </div>

        <el-divider content-position="left">
          {{ t('recording.settings.highlight') }}
        </el-divider>
        <div class="grid grid-cols-1 gap-x-5 md:grid-cols-2">
          <el-form-item :label="t('recording.danmakuThreshold')">
            <el-input-number
              :model-value="config.danmakuThreshold"
              :min="1"
              :max="10000"
              @update:model-value="update('danmakuThreshold', $event ?? 20)"
            />
          </el-form-item>
          <el-form-item :label="t('recording.settings.highlightDuration')">
            <span class="text-sm text-gray-700 dark:text-zinc-300">{{ t('recording.settings.highlightWindow') }}</span>
          </el-form-item>
        </div>

        <el-divider content-position="left">
          {{ t('recording.settings.diskProtection') }}
        </el-divider>
        <div class="grid grid-cols-1 gap-x-5 md:grid-cols-2">
          <el-form-item :label="t('recording.settings.diskQuota')">
            <el-select
              :model-value="config.quotaBytes"
              class="w-full"
              @update:model-value="update('quotaBytes', $event)"
            >
              <el-option :label="formatBytes(10 * 1024 ** 3)" :value="10 * 1024 ** 3" />
              <el-option :label="formatBytes(20 * 1024 ** 3)" :value="20 * 1024 ** 3" />
              <el-option :label="formatBytes(50 * 1024 ** 3)" :value="50 * 1024 ** 3" />
            </el-select>
          </el-form-item>
          <el-form-item :label="t('recording.settings.stopWaterline')">
            <el-select
              :model-value="config.waterlineBytes"
              class="w-full"
              @update:model-value="update('waterlineBytes', $event)"
            >
              <el-option :label="formatBytes(2 * 1024 ** 3)" :value="2 * 1024 ** 3" />
              <el-option :label="formatBytes(5 * 1024 ** 3)" :value="5 * 1024 ** 3" />
              <el-option :label="formatBytes(10 * 1024 ** 3)" :value="10 * 1024 ** 3" />
            </el-select>
          </el-form-item>
        </div>

        <el-divider content-position="left">
          {{ t('recording.settings.ffmpegAndShortcut') }}
        </el-divider>
        <div class="grid grid-cols-1 gap-x-5 md:grid-cols-2">
          <el-form-item :label="t('recording.settings.ffmpegSource')">
            <el-segmented
              :model-value="config.ffmpegSource"
              :options="[
                { label: t('recording.settings.bundled'), value: 'bundled', disabled: true },
                { label: t('recording.settings.local'), value: 'local' }
              ]"
              :disabled="ffmpegPending"
              @change="emit('setFfmpegSource', $event)"
            />
            <span
              v-if="config.ffmpegSource === 'bundled'"
              class="mt-1 block text-xs text-amber-700 dark:text-amber-400"
            >
              {{ t('recording.settings.bundledUnavailable') }}
            </span>
          </el-form-item>
          <el-form-item :label="t('recording.settings.shortcut')">
            <el-input
              :model-value="config.shortcut"
              maxlength="32"
              @update:model-value="update('shortcut', $event)"
            />
          </el-form-item>
        </div>
      </el-form>
    </div>
    <template #footer>
      <el-button :disabled="pending" @click="close">
        {{ t('common.cancel') }}
      </el-button>
      <el-button type="primary" :loading="pending" @click="emit('save')">
        {{ t('common.save') }}
      </el-button>
    </template>
  </el-dialog>
</template>
