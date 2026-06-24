<script setup lang="ts">
import { reactive } from 'vue';
import { useASRStore } from '../../../stores/asr';

const asrStore = useASRStore();

const form = reactive({
  enabled: asrStore.config.enabled,
  engine: asrStore.config.engine || 'whisper-cpp',
  language: asrStore.config.language || 'zh',
  model: asrStore.config.model || 'base',
  continuous: asrStore.config.continuous,
  interimResults: asrStore.config.interimResults,
  noiseReduction: asrStore.config.noiseReduction,
  autoStart: asrStore.config.autoStart
});

function getValues() {
  return { ...form };
}

defineExpose({ getValues });
</script>

<template>
  <el-form :model="form" label-width="150px">
    <el-form-item label="启用语音识别">
      <el-switch v-model="form.enabled" />
      <span class="ml-2 text-gray-500">开启后可将语音转换为文字</span>
    </el-form-item>
    <el-form-item label="识别引擎">
      <el-select v-model="form.engine">
        <el-option label="whisper.cpp（推荐，高质量）" value="whisper-cpp" />
        <el-option label="Web Speech API（浏览器内置）" value="web-speech" />
      </el-select>
    </el-form-item>
    <el-form-item label="识别语言">
      <el-select v-model="form.language">
        <el-option label="中文" value="zh" />
        <el-option label="English" value="en" />
        <el-option label="日本語" value="ja" />
        <el-option label="한국어" value="ko" />
        <el-option label="自动检测" value="auto" />
      </el-select>
    </el-form-item>
    <el-form-item v-if="form.engine === 'whisper-cpp'" label="模型大小">
      <el-select v-model="form.model">
        <el-option label="tiny（最快，准确率较低）" value="tiny" />
        <el-option label="base（平衡速度和准确率）" value="base" />
        <el-option label="small（较好的准确率）" value="small" />
        <el-option label="medium（高准确率）" value="medium" />
        <el-option label="large（最高准确率）" value="large" />
      </el-select>
    </el-form-item>
    <el-form-item label="连续识别">
      <el-switch v-model="form.continuous" />
      <span class="ml-2 text-gray-500">持续监听语音输入</span>
    </el-form-item>
    <el-form-item label="显示中间结果">
      <el-switch v-model="form.interimResults" />
      <span class="ml-2 text-gray-500">实时显示识别过程</span>
    </el-form-item>
    <el-form-item label="降噪处理">
      <el-switch v-model="form.noiseReduction" />
      <span class="ml-2 text-gray-500">自动过滤背景噪音</span>
    </el-form-item>
    <el-form-item label="自动开始">
      <el-switch v-model="form.autoStart" />
      <span class="ml-2 text-gray-500">连接直播间后自动开始识别</span>
    </el-form-item>
    <el-form-item>
      <el-alert
        v-if="form.engine === 'whisper-cpp'"
        title="whisper.cpp 需要单独安装"
        description="请运行: brew install whisper-cpp 或访问 https://github.com/ggerganov/whisper.cpp"
        type="info"
        show-icon
        :closable="false"
      />
    </el-form-item>
  </el-form>
</template>
