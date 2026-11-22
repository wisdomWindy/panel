<script setup lang="ts">
import { computed } from 'vue';
import { useI18n } from 'vue-i18n';
import { FolderOpened, Loading } from '@element-plus/icons-vue';
import type { RecordingConfig } from '@/platform/desktop';
import { formatBytes } from '@/modules/media/formatters';

const props = defineProps<{
  visible: boolean;
  modelValue: RecordingConfig;
  pending?: boolean;
  pathPending?: boolean;
  ffmpegPending?: boolean;
  installPending?: boolean;
}>();
const { t } = useI18n();

const emit = defineEmits<{
  'update:visible': [visible: boolean];
  'update:modelValue': [config: RecordingConfig];
  save: [];
  selectDirectory: [];
  setFfmpegSource: [source: RecordingConfig['ffmpegSource']];
  installFfmpeg: [];
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
    width="min(720px, calc(100vw - 32px))"
    destroy-on-close
    @close="close"
  >
    <template #header>
      <h3 class="app-text m-0 text-base font-semibold">{{ t('recording.settings.title') }}</h3>
    </template>
    <div class="max-h-[calc(100vh-190px)] overflow-y-auto pr-2">
      <el-form label-position="top" @submit.prevent="emit('save')">
        <h4 class="app-text m-0 text-sm font-semibold">{{ t('recording.settings.encoding') }}</h4>
        <div class="mt-3 grid grid-cols-1 gap-x-5 md:grid-cols-2">
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
              <span class="inline-flex">
                <button
                  type="button"
                  class="app-icon-btn shrink-0 disabled:cursor-not-allowed disabled:opacity-50"
                  :aria-label="t('recording.settings.selectDirectory')"
                  :aria-busy="pathPending"
                  :disabled="pathPending"
                  @click="emit('selectDirectory')"
                >
                  <el-icon v-if="pathPending" class="is-loading"><Loading /></el-icon>
                  <el-icon v-else><FolderOpened /></el-icon>
                </button>
              </span>
            </el-tooltip>
          </div>
        </el-form-item>

        <h4 class="app-text mt-5 mb-0 text-sm font-semibold">
          {{ t('recording.settings.segmentation') }}
        </h4>
        <div class="mt-3 grid grid-cols-1 gap-x-5 md:grid-cols-2">
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

        <h4 class="app-text mt-5 mb-0 text-sm font-semibold">
          {{ t('recording.settings.highlight') }}
        </h4>
        <div class="mt-3 grid grid-cols-1 gap-x-5 md:grid-cols-2">
          <el-form-item :label="t('recording.danmakuThreshold')">
            <el-input-number
              :model-value="config.danmakuThreshold"
              :min="1"
              :max="10000"
              @update:model-value="update('danmakuThreshold', $event ?? 20)"
            />
          </el-form-item>
          <el-form-item :label="t('recording.settings.highlightDuration')">
            <span class="app-muted text-sm">{{ t('recording.settings.highlightWindow') }}</span>
          </el-form-item>
        </div>

        <h4 class="app-text mt-5 mb-0 text-sm font-semibold">
          {{ t('recording.settings.diskProtection') }}
        </h4>
        <div class="mt-3 grid grid-cols-1 gap-x-5 md:grid-cols-2">
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

        <h4 class="app-text mt-5 mb-0 text-sm font-semibold">
          {{ t('recording.settings.ffmpegAndShortcut') }}
        </h4>
        <div class="mt-3 grid grid-cols-1 gap-x-5 md:grid-cols-2">
          <el-form-item :label="t('recording.settings.ffmpegSource')">
            <el-segmented
              :model-value="config.ffmpegSource"
              :options="[
                { label: t('recording.settings.bundled'), value: 'bundled' },
                { label: t('recording.settings.system'), value: 'system' },
                { label: t('recording.settings.installed'), value: 'installed' },
                { label: t('recording.settings.local'), value: 'local' }
              ]"
              :disabled="ffmpegPending"
              @change="emit('setFfmpegSource', $event)"
            />
            <button
              type="button"
              class="app-chip mt-1 inline-flex h-7 items-center gap-1 px-2 text-xs"
              :disabled="installPending"
              @click="emit('installFfmpeg')"
            >
              {{ installPending ? t('recording.settings.installing') : t('recording.settings.installFfmpeg') }}
            </button>
            <span
              v-if="config.ffmpegSource === 'bundled'"
              class="mt-1 block text-xs"
              style="color: var(--app-gold)"
            >
              {{ t('recording.settings.bundledUnavailable') }}
            </span>
            <span
              v-else-if="config.ffmpegSource === 'system'"
              class="app-muted mt-1 block text-xs"
            >
              {{ t('recording.settings.systemHint') }}
            </span>
            <span
              v-else-if="config.ffmpegSource === 'installed'"
              class="app-muted mt-1 block text-xs"
            >
              {{ t('recording.settings.installedHint') }}
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
      <div class="flex items-center justify-end gap-2">
        <button type="button" class="app-chip h-8 px-4" @click="close">
          {{ t('common.cancel') }}
        </button>
        <button
          type="button"
          class="app-btn-primary inline-flex h-8 items-center gap-1.5 px-4 text-sm disabled:cursor-not-allowed disabled:opacity-60"
          :disabled="pending"
          @click="emit('save')"
        >
          <el-icon v-if="pending" class="is-loading"><Loading /></el-icon>
          {{ pending ? t('recording.settings.saving') : t('common.save') }}
        </button>
      </div>
    </template>
  </el-dialog>
</template>
