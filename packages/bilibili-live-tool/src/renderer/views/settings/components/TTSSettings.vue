<script setup lang="ts">
import { reactive } from 'vue';
import { useTTSStore } from '../../../stores/tts';
import { EDGE_TTS_VOICES } from '../../../modules/tts/types';

const ttsStore = useTTSStore();
const edgeTTSVoices = EDGE_TTS_VOICES;

const form = reactive({
  enabled: ttsStore.config.enabled,
  engine: ttsStore.config.engine || 'edge-tts',
  speed: ttsStore.config.speed,
  volume: ttsStore.config.volume,
  minInterval: ttsStore.config.minInterval,
  formatTemplate: ttsStore.config.formatTemplate,
  broadcastGifts: ttsStore.config.broadcastGifts,
  giftValueThreshold: ttsStore.config.giftValueThreshold,
  broadcastCaptain: ttsStore.config.broadcastCaptain,
  filterKeywordsText: ttsStore.config.filterKeywords.join('\n'),
  edgeTTSVoice: ttsStore.config.edgeTTSVoice || 'zh-CN-XiaoxiaoNeural'
});

function getValues() {
  return {
    enabled: form.enabled,
    engine: form.engine,
    speed: form.speed,
    volume: form.volume,
    minInterval: form.minInterval,
    formatTemplate: form.formatTemplate,
    broadcastGifts: form.broadcastGifts,
    giftValueThreshold: form.giftValueThreshold,
    broadcastCaptain: form.broadcastCaptain,
    filterKeywords: form.filterKeywordsText.split('\n').filter((word: string) => word.trim()),
    edgeTTSVoice: form.edgeTTSVoice
  };
}

defineExpose({ getValues });
</script>

<template>
  <el-form :model="form" label-width="150px">
    <el-form-item label="启用语音播报">
      <el-switch v-model="form.enabled" />
      <span class="ml-2 text-gray-500">开启后将自动播报弹幕内容</span>
    </el-form-item>
    <el-form-item label="TTS 引擎">
      <el-select v-model="form.engine">
        <el-option label="Edge TTS（推荐，高质量）" value="edge-tts" />
        <el-option label="Web Speech API（浏览器内置）" value="local" />
      </el-select>
    </el-form-item>
    <el-form-item v-if="form.engine === 'edge-tts'" label="语音选择">
      <el-select v-model="form.edgeTTSVoice">
        <el-option
          v-for="voice in edgeTTSVoices"
          :key="voice.name"
          :label="voice.label"
          :value="voice.name"
        />
      </el-select>
    </el-form-item>
    <el-form-item label="播报语速">
      <el-slider
        v-model="form.speed"
        :min="0.5"
        :max="2"
        :step="0.1"
        show-input
      />
    </el-form-item>
    <el-form-item label="播报音量">
      <el-slider
        v-model="form.volume"
        :min="0"
        :max="1"
        :step="0.1"
        show-input
      />
    </el-form-item>
    <el-form-item label="最小播报间隔">
      <el-input-number
        v-model="form.minInterval"
        :min="500"
        :max="10000"
        :step="500"
      />
      <span class="ml-2">毫秒</span>
    </el-form-item>
    <el-form-item label="播报格式">
      <el-input v-model="form.formatTemplate" placeholder="{username}说：{content}" />
      <span class="ml-2 text-gray-500">可用变量: {username}, {content}</span>
    </el-form-item>
    <el-form-item label="礼物播报">
      <el-switch v-model="form.broadcastGifts" />
    </el-form-item>
    <el-form-item v-if="form.broadcastGifts" label="礼物价值阈值">
      <el-input-number
        v-model="form.giftValueThreshold"
        :min="0"
        :max="10000"
        :step="100"
      />
      <span class="ml-2">舰长数</span>
    </el-form-item>
    <el-form-item label="舰长播报">
      <el-switch v-model="form.broadcastCaptain" />
    </el-form-item>
    <el-form-item label="过滤关键词">
      <el-input
        v-model="form.filterKeywordsText"
        type="textarea"
        :rows="3"
        placeholder="每行一个关键词，包含这些词的弹幕将不会被播报"
      />
    </el-form-item>
  </el-form>
</template>
