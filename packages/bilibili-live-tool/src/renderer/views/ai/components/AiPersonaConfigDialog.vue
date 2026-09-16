<script setup lang="ts">
import { reactive, watch } from 'vue';
import type { AiConfig, AiConfigSaveRequest, AiSpeakingStyle } from '@/platform/desktop';

const props = defineProps<{
    visible: boolean;
    config: AiConfig | null;
    pending: boolean;
    errorMessage: string | null;
  }>();

const emit = defineEmits<{
    'update:visible': [value: boolean];
    save: [request: AiConfigSaveRequest];
  }>();

const draft = reactive({
  identity: '',
  speakingStyle: 'short' as AiSpeakingStyle,
  systemPrompt: '',
  provider: '',
  model: '',
  temperature: 0.7,
  maxLength: 100,
  isSensitiveFilterEnabled: true,
  bannedWordsText: '',
  danmakuContextCount: 20,
  asrContextCount: 0,
  manualContextCount: 0,
  apiKeyReplacement: ''
});

watch(
  () => [props.visible, props.config] as const,
  () => {
    if (!props.visible || !props.config) return;
    Object.assign(draft, {
      ...props.config,
      bannedWordsText: props.config.bannedWords.join('\n'),
      apiKeyReplacement: ''
    });
  },
  { immediate: true }
);

function submit(): void {
  emit('save', {
    config: {
      identity: draft.identity.trim(),
      speakingStyle: draft.speakingStyle,
      systemPrompt: draft.systemPrompt.trim(),
      provider: draft.provider.trim(),
      model: draft.model.trim(),
      temperature: draft.temperature,
      maxLength: draft.maxLength,
      isSensitiveFilterEnabled: draft.isSensitiveFilterEnabled,
      bannedWords: draft.bannedWordsText
        .split(/\r?\n/)
        .map(value => value.trim())
        .filter(Boolean),
      danmakuContextCount: draft.danmakuContextCount,
      asrContextCount: draft.asrContextCount,
      manualContextCount: draft.manualContextCount
    },
    apiKeyReplacement: draft.apiKeyReplacement.trim() || null
  });
}
</script>

<template>
  <el-dialog
    :model-value="visible"
    title="AI 人设配置"
    width="min(680px, 94vw)"
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
        <el-form-item label="AI 身份设定">
          <el-input v-model="draft.identity" />
        </el-form-item>
        <el-form-item label="说话风格">
          <el-select v-model="draft.speakingStyle" class="w-full">
            <el-option label="简短短句" value="short" />
            <el-option label="幽默玩梗" value="humorous" />
            <el-option label="礼貌正式" value="formal" />
            <el-option label="随性闲聊" value="casual" />
          </el-select>
        </el-form-item>
      </div>
      <el-form-item label="系统提示词">
        <el-input v-model="draft.systemPrompt" type="textarea" :rows="3" />
      </el-form-item>
      <div class="grid gap-x-4 sm:grid-cols-2">
        <el-form-item label="Provider">
          <el-input v-model="draft.provider" />
        </el-form-item>
        <el-form-item label="模型">
          <el-input v-model="draft.model" />
        </el-form-item>
        <el-form-item label="API Key">
          <el-input
            v-model="draft.apiKeyReplacement"
            type="password"
            show-password
            :placeholder="config?.isApiKeyConfigured ? '已配置' : ''"
          />
        </el-form-item>
        <el-form-item label="温度">
          <el-input-number
            v-model="draft.temperature"
            :min="0"
            :max="2"
            :step="0.1"
            class="w-full"
          />
        </el-form-item>
        <el-form-item label="最大长度">
          <el-input-number v-model="draft.maxLength" :min="1" class="w-full" />
        </el-form-item>
        <el-form-item label="历史弹幕条数">
          <el-input-number v-model="draft.danmakuContextCount" :min="0" class="w-full" />
        </el-form-item>
        <el-form-item label="语音上下文条数">
          <el-input-number v-model="draft.asrContextCount" :min="0" class="w-full" />
        </el-form-item>
        <el-form-item label="人工输入条数">
          <el-input-number v-model="draft.manualContextCount" :min="0" class="w-full" />
        </el-form-item>
      </div>
      <el-form-item label="敏感词过滤">
        <el-switch v-model="draft.isSensitiveFilterEnabled" />
      </el-form-item>
      <el-form-item v-if="draft.isSensitiveFilterEnabled" label="敏感词库">
        <el-input v-model="draft.bannedWordsText" type="textarea" :rows="3" />
      </el-form-item>
    </el-form>
    <template #footer>
      <el-button :disabled="pending" @click="emit('update:visible', false)">
        取消
      </el-button>
      <el-button
        type="primary"
        :loading="pending"
        :disabled="!config"
        @click="submit"
      >
        保存
      </el-button>
    </template>
  </el-dialog>
</template>
