<script setup lang="ts">
import { reactive } from 'vue';
import { useAIStore } from '../../../stores/ai';

const aiStore = useAIStore();

const form = reactive({
  enabled: aiStore.config.enabled,
  provider: aiStore.config.provider || 'openai',
  apiKey: aiStore.config.apiKey || '',
  model: aiStore.config.model || 'gpt-3.5-turbo',
  endpoint: aiStore.config.endpoint || '',
  personalityKey: 'friendly',
  systemPrompt: aiStore.config.personality?.systemPrompt || '',
  contextWindow: aiStore.config.contextWindow || 10,
  maxResponseLength: aiStore.config.maxResponseLength || 100,
  sensitiveWordsFilter: aiStore.config.sensitiveWordsFilter ?? true
});

function handlePersonalityChange(key: string) {
  aiStore.setPersonality(key);
  form.systemPrompt = aiStore.config.personality.systemPrompt;
}

function getValues() {
  return {
    enabled: form.enabled,
    provider: form.provider,
    apiKey: form.apiKey,
    model: form.model,
    endpoint: form.endpoint,
    personality: {
      ...aiStore.config.personality,
      systemPrompt: form.systemPrompt
    },
    contextWindow: form.contextWindow,
    maxResponseLength: form.maxResponseLength,
    sensitiveWordsFilter: form.sensitiveWordsFilter
  };
}

defineExpose({ getValues });
</script>

<template>
  <el-form :model="form" label-width="150px">
    <el-form-item label="启用 AI 对话">
      <el-switch v-model="form.enabled" />
    </el-form-item>
    <el-form-item label="AI 提供商">
      <el-select v-model="form.provider">
        <el-option label="OpenAI" value="openai" />
        <el-option label="Claude" value="claude" />
        <el-option label="Ollama（本地）" value="ollama" />
        <el-option label="自定义 API" value="custom" />
      </el-select>
    </el-form-item>
    <el-form-item v-if="form.provider !== 'ollama'" label="API Key">
      <el-input
        v-model="form.apiKey"
        type="password"
        placeholder="输入 API Key"
        show-password
      />
    </el-form-item>
    <el-form-item v-if="form.provider === 'custom'" label="API 端点">
      <el-input v-model="form.endpoint" placeholder="https://your-api.com/v1/chat/completions" />
    </el-form-item>
    <el-form-item label="模型">
      <el-select v-if="form.provider === 'openai'" v-model="form.model">
        <el-option label="GPT-3.5 Turbo" value="gpt-3.5-turbo" />
        <el-option label="GPT-4" value="gpt-4" />
        <el-option label="GPT-4 Turbo" value="gpt-4-turbo" />
      </el-select>
      <el-select v-else-if="form.provider === 'claude'" v-model="form.model">
        <el-option label="Claude 3 Haiku" value="claude-3-haiku-20240307" />
        <el-option label="Claude 3 Sonnet" value="claude-3-sonnet-20240229" />
        <el-option label="Claude 3 Opus" value="claude-3-opus-20240229" />
      </el-select>
      <el-input v-else v-model="form.model" placeholder="模型名称" />
    </el-form-item>
    <el-form-item label="人设风格">
      <el-select v-model="form.personalityKey" @change="handlePersonalityChange">
        <el-option label="友好助手" value="friendly" />
        <el-option label="幽默达人" value="humorous" />
        <el-option label="二次元" value="anime" />
        <el-option label="专业主播" value="professional" />
      </el-select>
    </el-form-item>
    <el-form-item label="系统提示词">
      <el-input
        v-model="form.systemPrompt"
        type="textarea"
        :rows="3"
        placeholder="设定 AI 的性格和行为"
      />
    </el-form-item>
    <el-form-item label="上下文窗口">
      <el-input-number v-model="form.contextWindow" :min="1" :max="50" />
      <span class="ml-2">条弹幕</span>
    </el-form-item>
    <el-form-item label="最大回复长度">
      <el-input-number v-model="form.maxResponseLength" :min="50" :max="500" />
      <span class="ml-2">字符</span>
    </el-form-item>
    <el-form-item label="敏感词过滤">
      <el-switch v-model="form.sensitiveWordsFilter" />
      <span class="ml-2 text-gray-500">过滤 AI 回复中的敏感词</span>
    </el-form-item>
  </el-form>
</template>
