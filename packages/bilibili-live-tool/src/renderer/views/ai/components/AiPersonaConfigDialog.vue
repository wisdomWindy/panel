<script setup lang="ts">
import { reactive, watch } from 'vue';
import { useI18n } from 'vue-i18n';
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

const { t } = useI18n();
const draft = reactive({
  identity: '',
  speakingStyle: 'natural' as AiSpeakingStyle,
  systemPrompt: '',
  maxLength: 50,
  maxSentences: 2,
  isSensitiveFilterEnabled: true,
  bannedWordsText: '',
  danmakuContextCount: 20,
  manualContextCount: 0
});

watch(
  () => [props.visible, props.config] as const,
  () => {
    if (!props.visible || !props.config) return;
    Object.assign(draft, {
      ...props.config,
      bannedWordsText: props.config.bannedWords.join('\n')
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
      maxLength: draft.maxLength,
      maxSentences: draft.maxSentences,
      isSensitiveFilterEnabled: draft.isSensitiveFilterEnabled,
      bannedWords: draft.bannedWordsText
        .split(/\r?\n/)
        .map(value => value.trim())
        .filter(Boolean),
      danmakuContextCount: draft.danmakuContextCount,
      manualContextCount: draft.manualContextCount
    }
  });
}
</script>

<template>
  <el-dialog
    :model-value="visible"
    width="min(680px, 94vw)"
    :close-on-click-modal="!pending"
    @update:model-value="emit('update:visible', $event)"
  >
    <template #header>
      <h3 class="app-text m-0 text-base font-semibold">
        {{ t('ai.persona.title') }}
      </h3>
      <p class="app-muted mt-1 mb-0 text-sm">
        {{ t('ai.persona.hint') }}
      </p>
    </template>
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
        <el-form-item :label="t('ai.persona.identity')">
          <el-input v-model="draft.identity" />
        </el-form-item>
        <el-form-item :label="t('ai.persona.speakingStyle')">
          <el-select v-model="draft.speakingStyle" class="w-full">
            <el-option :label="t('ai.persona.style.concise')" value="concise" />
            <el-option :label="t('ai.persona.style.natural')" value="natural" />
            <el-option :label="t('ai.persona.style.detailed')" value="detailed" />
          </el-select>
        </el-form-item>
      </div>
      <el-form-item :label="t('ai.persona.systemPrompt')">
        <el-input v-model="draft.systemPrompt" type="textarea" :rows="3" />
      </el-form-item>
      <div class="grid gap-x-4 sm:grid-cols-2">
        <el-form-item :label="t('ai.persona.maxLength')">
          <el-input-number
            v-model="draft.maxLength"
            :min="20"
            :max="80"
            class="w-full"
          />
        </el-form-item>
        <el-form-item :label="t('ai.persona.maxSentences')">
          <el-input-number
            v-model="draft.maxSentences"
            :min="1"
            :max="3"
            class="w-full"
          />
        </el-form-item>
        <el-form-item :label="t('ai.persona.danmakuContext')">
          <el-input-number v-model="draft.danmakuContextCount" :min="0" class="w-full" />
        </el-form-item>
        <el-form-item :label="t('ai.persona.manualContext')">
          <el-input-number v-model="draft.manualContextCount" :min="0" class="w-full" />
        </el-form-item>
      </div>
      <el-form-item :label="t('ai.persona.sensitiveFilter')">
        <el-switch v-model="draft.isSensitiveFilterEnabled" />
      </el-form-item>
      <el-form-item v-if="draft.isSensitiveFilterEnabled" :label="t('ai.persona.bannedWords')">
        <el-input v-model="draft.bannedWordsText" type="textarea" :rows="3" />
      </el-form-item>
    </el-form>
    <template #footer>
      <div class="flex flex-wrap items-center justify-end gap-2">
        <button type="button" class="app-chip" :disabled="pending" @click="emit('update:visible', false)">
          {{ t('common.cancel') }}
        </button>
        <button
          type="button"
          class="app-btn-primary h-8 px-4 text-sm disabled:cursor-not-allowed disabled:opacity-60"
          :disabled="pending || !config"
          @click="submit"
        >
          {{ t('common.save') }}
        </button>
      </div>
    </template>
  </el-dialog>
</template>
