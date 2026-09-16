<script setup lang="ts">
import { reactive, watch } from 'vue';
import type { ModerationRules, MuteDuration } from '@/platform/desktop';

const props = defineProps<{
    visible: boolean;
    rules: ModerationRules | null;
    pending: boolean;
    errorMessage: string | null;
  }>();

const emit = defineEmits<{
    'update:visible': [value: boolean];
    save: [rules: ModerationRules];
  }>();

const draft = reactive({
  frequencyWindowSeconds: 10,
  frequencyThreshold: 5,
  repeatWindowSeconds: 60,
  repeatThreshold: 3,
  similarityWindowSeconds: 60,
  similarityThreshold: 5,
  similarityRatio: 0.8,
  defaultMuteDuration: '1h' as MuteDuration,
  isRiskConfirmed: false,
  isAutoEnabled: false
});

watch(
  () => [props.visible, props.rules] as const,
  () => {
    if (props.visible && props.rules) Object.assign(draft, props.rules);
  },
  { immediate: true }
);

function submit(): void {
  if (props.rules) emit('save', Object.freeze({ ...draft }));
}
</script>

<template>
  <el-dialog
    :model-value="visible"
    title="规则配置"
    width="min(640px, 94vw)"
    :close-on-click-modal="!pending"
    @update:model-value="emit('update:visible', $event)"
  >
    <el-alert
      v-if="errorMessage"
      class="mb-4"
      type="error"
      :closable="false"
      :title="errorMessage"
      show-icon
    />
    <el-form label-position="top" @submit.prevent="submit">
      <div class="grid gap-x-4 sm:grid-cols-2">
        <el-form-item label="频率窗口（秒）">
          <el-input-number v-model="draft.frequencyWindowSeconds" :min="1" class="w-full" />
        </el-form-item>
        <el-form-item label="频率条数">
          <el-input-number v-model="draft.frequencyThreshold" :min="1" class="w-full" />
        </el-form-item>
        <el-form-item label="重复窗口（秒）">
          <el-input-number v-model="draft.repeatWindowSeconds" :min="1" class="w-full" />
        </el-form-item>
        <el-form-item label="重复条数">
          <el-input-number v-model="draft.repeatThreshold" :min="1" class="w-full" />
        </el-form-item>
        <el-form-item label="相似窗口（秒）">
          <el-input-number v-model="draft.similarityWindowSeconds" :min="1" class="w-full" />
        </el-form-item>
        <el-form-item label="相似条数">
          <el-input-number v-model="draft.similarityThreshold" :min="1" class="w-full" />
        </el-form-item>
        <el-form-item label="相似度">
          <el-input-number
            v-model="draft.similarityRatio"
            :min="0"
            :max="1"
            :step="0.05"
            class="w-full"
          />
        </el-form-item>
        <el-form-item label="默认禁言时长">
          <el-select v-model="draft.defaultMuteDuration" class="w-full">
            <el-option label="10 分钟" value="10m" />
            <el-option label="1 小时" value="1h" />
            <el-option label="永久" value="permanent" />
          </el-select>
        </el-form-item>
      </div>
    </el-form>
    <template #footer>
      <el-button :disabled="pending" @click="emit('update:visible', false)">
        取消
      </el-button>
      <el-button
        type="primary"
        :loading="pending"
        :disabled="!rules"
        @click="submit"
      >
        保存
      </el-button>
    </template>
  </el-dialog>
</template>
