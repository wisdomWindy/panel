<script setup lang="ts">
import { reactive, ref } from 'vue';
import { useModerationStore } from '../../../stores/moderation';

const moderationStore = useModerationStore();
const newProhibitedWord = ref('');

const form = reactive({
  enabled: moderationStore.config.enabled,
  autoMuteEnabled: moderationStore.config.autoMuteEnabled,
  floodProtectionEnabled: moderationStore.config.floodProtectionEnabled,
  floodThreshold: moderationStore.config.floodThreshold,
  floodTimeWindow: moderationStore.config.floodTimeWindow,
  muteDuration: moderationStore.config.muteDuration,
  warningBeforeMute: moderationStore.config.warningBeforeMute,
  prohibitedWords: [...moderationStore.config.prohibitedWords]
});

function handleAddProhibitedWord() {
  if (newProhibitedWord.value.trim()) {
    moderationStore.addProhibitedWord(newProhibitedWord.value.trim());
    form.prohibitedWords = [...moderationStore.config.prohibitedWords];
    newProhibitedWord.value = '';
  }
}

function handleRemoveProhibitedWord(word: string) {
  moderationStore.removeProhibitedWord(word);
  form.prohibitedWords = [...moderationStore.config.prohibitedWords];
}

function getValues() {
  return {
    enabled: form.enabled,
    autoMuteEnabled: form.autoMuteEnabled,
    floodProtectionEnabled: form.floodProtectionEnabled,
    floodThreshold: form.floodThreshold,
    floodTimeWindow: form.floodTimeWindow,
    muteDuration: form.muteDuration,
    warningBeforeMute: form.warningBeforeMute
  };
}

defineExpose({ getValues });
</script>

<template>
  <el-form :model="form" label-width="150px">
    <el-form-item label="启用禁言防护">
      <el-switch v-model="form.enabled" />
      <span class="ml-2 text-gray-500">开启后自动检测违规行为</span>
    </el-form-item>
    <el-form-item label="自动禁言">
      <el-switch v-model="form.autoMuteEnabled" />
      <span class="ml-2 text-gray-500">检测到违禁词自动禁言</span>
    </el-form-item>
    <el-form-item label="刷屏防护">
      <el-switch v-model="form.floodProtectionEnabled" />
      <span class="ml-2 text-gray-500">检测刷屏行为自动禁言</span>
    </el-form-item>
    <el-form-item label="刷屏阈值">
      <el-input-number v-model="form.floodThreshold" :min="3" :max="20" />
      <span class="ml-2">条/{{ form.floodTimeWindow }}秒</span>
    </el-form-item>
    <el-form-item label="检测时间窗口">
      <el-input-number v-model="form.floodTimeWindow" :min="5" :max="60" />
      <span class="ml-2">秒</span>
    </el-form-item>
    <el-form-item label="禁言时长">
      <el-input-number v-model="form.muteDuration" :min="1" :max="1440" />
      <span class="ml-2">分钟（0=永久）</span>
    </el-form-item>
    <el-form-item label="禁言前警告">
      <el-switch v-model="form.warningBeforeMute" />
      <span class="ml-2 text-gray-500">首次违规先警告再禁言</span>
    </el-form-item>
    <el-form-item label="违禁词列表">
      <div class="w-full">
        <div class="flex gap-2 mb-2">
          <el-input
            v-model="newProhibitedWord"
            placeholder="输入违禁词"
            @keyup.enter="handleAddProhibitedWord"
          />
          <el-button type="primary" @click="handleAddProhibitedWord">
            添加
          </el-button>
        </div>
        <div class="flex flex-wrap gap-2">
          <el-tag
            v-for="word in form.prohibitedWords"
            :key="word"
            closable
            @close="handleRemoveProhibitedWord(word)"
          >
            {{ word }}
          </el-tag>
        </div>
      </div>
    </el-form-item>
  </el-form>
</template>
