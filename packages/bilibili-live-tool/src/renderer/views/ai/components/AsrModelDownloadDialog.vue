<script setup lang="ts">
import { ref, watch } from 'vue';
import type { AsrModelManifest, AsrStatus } from '@/platform/desktop';

const props = defineProps<{
    visible: boolean;
    manifest: AsrModelManifest | null;
    status: AsrStatus | null;
    pending: boolean;
  }>();

const emit = defineEmits<{
    'update:visible': [value: boolean];
    download: [manifest: AsrModelManifest, accepted: boolean];
  }>();

const accepted = ref(false);
watch(
  () => props.visible,
  value => {
    if (value) accepted.value = false;
  }
);

function download(): void {
  if (props.manifest && accepted.value) emit('download', props.manifest, true);
}
</script>

<template>
  <el-dialog
    :model-value="visible"
    title="ASR 模型"
    width="min(520px, 94vw)"
    :close-on-click-modal="!pending"
    @update:model-value="emit('update:visible', $event)"
  >
    <el-alert
      v-if="!manifest"
      type="warning"
      :closable="false"
      title="模型资源暂不可用"
      show-icon
    />
    <template v-else>
      <dl class="grid grid-cols-[auto_1fr] gap-x-4 gap-y-2 text-sm">
        <dt class="text-gray-500">
          模型
        </dt>
        <dd class="m-0 break-all">
          {{ manifest.modelName }}
        </dd>
        <dt class="text-gray-500">
          大小
        </dt>
        <dd class="m-0">
          {{ Math.ceil(manifest.sizeBytes / 1024 / 1024) }} MB
        </dd>
        <dt class="text-gray-500">
          许可证
        </dt>
        <dd class="m-0">
          {{ manifest.license }}
        </dd>
        <dt class="text-gray-500">
          来源
        </dt>
        <dd class="m-0 break-all">
          {{ manifest.sourceLabel }}
        </dd>
      </dl>
      <el-checkbox v-model="accepted" class="mt-4">
        我已阅读并同意模型许可证
      </el-checkbox>
      <el-progress
        v-if="status?.state === 'downloading'"
        class="mt-4"
        :percentage="status.progressPercent"
      />
    </template>
    <template #footer>
      <el-button :disabled="pending" @click="emit('update:visible', false)">
        取消
      </el-button>
      <el-button
        type="primary"
        :loading="pending"
        :disabled="!manifest || !accepted"
        @click="download"
      >
        开始下载
      </el-button>
    </template>
  </el-dialog>
</template>
