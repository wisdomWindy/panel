<script setup lang="ts">
import { ref, watch } from 'vue';
import type { AsrModelId, AsrModelManifest, AsrStatus } from '@/platform/desktop';

const props = defineProps<{
    visible: boolean;
    models: readonly AsrModelManifest[];
    status: AsrStatus | null;
    pending: boolean;
    errorMessage: string | null;
  }>();

const emit = defineEmits<{
    'update:visible': [value: boolean];
    download: [modelId: AsrModelId, accepted: boolean];
  }>();

const accepted = ref(false);
const selectedModelId = ref<AsrModelId | null>(null);
watch(
  () => [props.visible, props.models] as const,
  ([visible]) => {
    if (visible) {
      accepted.value = false;
      selectedModelId.value =
        props.models.find(model => !model.isInstalled)?.modelId ?? props.models[0]?.modelId ?? null;
    }
  }
);

function download(): void {
  if (selectedModelId.value && accepted.value) emit('download', selectedModelId.value, true);
}
</script>

<template>
  <el-dialog
    :model-value="visible"
    title="选择识别模型"
    width="min(520px, 94vw)"
    :close-on-click-modal="!pending"
    @update:model-value="emit('update:visible', $event)"
  >
    <el-alert
      v-if="errorMessage"
      class="mb-3"
      type="error"
      :closable="false"
      :title="errorMessage"
      show-icon
    />
    <el-empty
      v-if="models.length === 0"
      description="模型列表加载失败，请稍后重试"
      :image-size="72"
    />
    <template v-else>
      <el-radio-group v-model="selectedModelId" class="grid w-full gap-2">
        <el-radio
          v-for="model in models"
          :key="model.modelId"
          :value="model.modelId"
          border
          class="!m-0 !h-auto w-full !items-start !px-4 !py-3"
          :disabled="pending"
        >
          <span class="flex min-w-0 flex-col gap-1 whitespace-normal">
            <span class="font-medium text-gray-900 dark:text-gray-100">
              {{ model.modelName }}
              <el-tag
                v-if="model.isInstalled"
                class="ml-2"
                size="small"
                type="success"
              >
                已下载
              </el-tag>
            </span>
            <span class="text-xs text-gray-500">
              {{ Math.ceil(model.sizeBytes / 1024 / 1024) }} MB · {{ model.sourceLabel }} ·
              {{ model.license }}
            </span>
          </span>
        </el-radio>
      </el-radio-group>
      <el-checkbox v-model="accepted" class="mt-4">
        我已阅读并同意模型许可证
      </el-checkbox>
      <el-progress
        v-if="status?.state === 'downloading'"
        class="mt-4"
        :percentage="status.progressPercent"
        :indeterminate="pending && status.progressPercent === 0"
      />
    </template>
    <template #footer>
      <el-button :disabled="pending" @click="emit('update:visible', false)">
        取消
      </el-button>
      <el-button
        type="primary"
        :loading="pending"
        :disabled="!selectedModelId || !accepted"
        @click="download"
      >
        开始下载
      </el-button>
    </template>
  </el-dialog>
</template>
