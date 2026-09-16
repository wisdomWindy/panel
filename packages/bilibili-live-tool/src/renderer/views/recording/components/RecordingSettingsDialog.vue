<script setup lang="ts">
import { computed } from 'vue';
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
    title="录制设置"
    width="min(720px, calc(100vw - 32px))"
    destroy-on-close
    @close="close"
  >
    <div class="max-h-[calc(100vh-190px)] overflow-y-auto pr-2">
      <el-form label-position="top" @submit.prevent="emit('save')">
        <el-divider content-position="left">
          编码与输出
        </el-divider>
        <div class="grid grid-cols-1 gap-x-5 md:grid-cols-2">
          <el-form-item label="录制格式">
            <el-select :model-value="config.format" class="w-full" @update:model-value="update('format', $event)">
              <el-option label="MP4" value="mp4" />
              <el-option label="FLV" value="flv" />
            </el-select>
          </el-form-item>
          <el-form-item label="录制画质">
            <el-select :model-value="config.quality" class="w-full" @update:model-value="update('quality', $event)">
              <el-option label="原画" value="original" />
              <el-option label="高清" value="high" />
              <el-option label="标清" value="standard" />
            </el-select>
          </el-form-item>
        </div>
        <el-form-item label="存储路径">
          <div class="flex w-full min-w-0 items-center gap-2">
            <el-input :model-value="config.directory?.displayPath ?? ''" readonly placeholder="未选择" />
            <el-tooltip content="选择存储目录">
              <el-button
                :icon="FolderOpened"
                circle
                :loading="pathPending"
                aria-label="选择存储目录"
                @click="emit('selectDirectory')"
              />
            </el-tooltip>
          </div>
        </el-form-item>

        <el-divider content-position="left">
          分段与自动录制
        </el-divider>
        <div class="grid grid-cols-1 gap-x-5 md:grid-cols-2">
          <el-form-item label="自动分段">
            <el-switch :model-value="config.segmentationEnabled" @update:model-value="update('segmentationEnabled', $event)" />
          </el-form-item>
          <el-form-item label="分段时长（秒）">
            <el-input-number
              :model-value="config.segmentDurationSeconds"
              :min="60"
              :max="86400"
              :disabled="!config.segmentationEnabled"
              @update:model-value="update('segmentDurationSeconds', $event ?? 3600)"
            />
          </el-form-item>
          <el-form-item label="开播自动录制">
            <el-switch :model-value="config.autoRecord" @update:model-value="update('autoRecord', $event)" />
          </el-form-item>
        </div>

        <el-divider content-position="left">
          高光
        </el-divider>
        <div class="grid grid-cols-1 gap-x-5 md:grid-cols-2">
          <el-form-item label="弹幕阈值">
            <el-input-number
              :model-value="config.danmakuThreshold"
              :min="1"
              :max="10000"
              @update:model-value="update('danmakuThreshold', $event ?? 20)"
            />
          </el-form-item>
          <el-form-item label="高光时长">
            <span class="text-sm text-gray-700">前 30 秒 / 后 30 秒</span>
          </el-form-item>
        </div>

        <el-divider content-position="left">
          磁盘保护
        </el-divider>
        <div class="grid grid-cols-1 gap-x-5 md:grid-cols-2">
          <el-form-item label="磁盘配额">
            <el-select :model-value="config.quotaBytes" class="w-full" @update:model-value="update('quotaBytes', $event)">
              <el-option :label="formatBytes(10 * 1024 ** 3)" :value="10 * 1024 ** 3" />
              <el-option :label="formatBytes(20 * 1024 ** 3)" :value="20 * 1024 ** 3" />
              <el-option :label="formatBytes(50 * 1024 ** 3)" :value="50 * 1024 ** 3" />
            </el-select>
          </el-form-item>
          <el-form-item label="停止录制水位">
            <el-select :model-value="config.waterlineBytes" class="w-full" @update:model-value="update('waterlineBytes', $event)">
              <el-option :label="formatBytes(2 * 1024 ** 3)" :value="2 * 1024 ** 3" />
              <el-option :label="formatBytes(5 * 1024 ** 3)" :value="5 * 1024 ** 3" />
              <el-option :label="formatBytes(10 * 1024 ** 3)" :value="10 * 1024 ** 3" />
            </el-select>
          </el-form-item>
        </div>

        <el-divider content-position="left">
          FFmpeg 与快捷键
        </el-divider>
        <div class="grid grid-cols-1 gap-x-5 md:grid-cols-2">
          <el-form-item label="FFmpeg 来源">
            <el-segmented
              :model-value="config.ffmpegSource"
              :options="[{ label: '内置', value: 'bundled', disabled: true }, { label: '本机', value: 'local' }]"
              :disabled="ffmpegPending"
              @change="emit('setFfmpegSource', $event)"
            />
            <span v-if="config.ffmpegSource === 'bundled'" class="mt-1 block text-xs text-amber-700">内置 FFmpeg 尚未随安装包提供</span>
          </el-form-item>
          <el-form-item label="高光快捷键">
            <el-input :model-value="config.shortcut" maxlength="32" @update:model-value="update('shortcut', $event)" />
          </el-form-item>
        </div>
      </el-form>
    </div>
    <template #footer>
      <el-button :disabled="pending" @click="close">
        取消
      </el-button>
      <el-button type="primary" :loading="pending" @click="emit('save')">
        保存
      </el-button>
    </template>
  </el-dialog>
</template>
