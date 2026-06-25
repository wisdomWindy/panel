<script setup lang="ts">
import { reactive } from 'vue';
import { useRecordingStore } from '../../../stores/recording';

const recordingStore = useRecordingStore();

const form = reactive({
  enabled: recordingStore.config.enabled,
  autoStart: recordingStore.config.autoStart,
  format: recordingStore.config.format,
  quality: recordingStore.config.quality,
  outputPath: recordingStore.config.outputPath,
  segmentEnabled: recordingStore.config.segmentSize > 0,
  segmentSize: recordingStore.config.segmentSize || 2048,
  autoCleanup: recordingStore.config.autoCleanup,
  cleanupDays: recordingStore.config.cleanupDays,
  highlightEnabled: recordingStore.config.highlightEnabled,
  danmakuThreshold: recordingStore.config.highlightConfig.danmakuThreshold,
  giftValueThreshold: recordingStore.config.highlightConfig.giftValueThreshold,
  captainTrigger: recordingStore.config.highlightConfig.captainTrigger,
  clipDuration: recordingStore.config.highlightConfig.clipDuration
});

function getValues() {
  return {
    enabled: form.enabled,
    autoStart: form.autoStart,
    format: form.format,
    quality: form.quality,
    outputPath: form.outputPath,
    segmentSize: form.segmentEnabled ? form.segmentSize : 0,
    autoCleanup: form.autoCleanup,
    cleanupDays: form.cleanupDays,
    highlightEnabled: form.highlightEnabled,
    highlightConfig: {
      danmakuThreshold: form.danmakuThreshold,
      giftValueThreshold: form.giftValueThreshold,
      captainTrigger: form.captainTrigger,
      clipDuration: form.clipDuration
    }
  };
}

defineExpose({ getValues });
</script>

<template>
  <el-form :model="form" label-width="150px">
    <el-form-item label="启用录制功能">
      <el-switch v-model="form.enabled" />
    </el-form-item>
    <el-form-item label="自动开始录制">
      <el-switch v-model="form.autoStart" />
      <span class="ml-2 text-gray-500">连接直播间后自动开始录制</span>
    </el-form-item>
    <el-form-item label="录制格式">
      <el-select v-model="form.format">
        <el-option label="MP4" value="mp4" />
        <el-option label="FLV" value="flv" />
      </el-select>
    </el-form-item>
    <el-form-item label="录制质量">
      <el-select v-model="form.quality">
        <el-option label="原始质量" value="original" />
        <el-option label="720p" value="720p" />
        <el-option label="1080p" value="1080p" />
      </el-select>
    </el-form-item>
    <el-form-item label="存储路径">
      <el-input v-model="form.outputPath" placeholder="留空使用默认路径" />
    </el-form-item>
    <el-form-item label="自动分段">
      <el-switch v-model="form.segmentEnabled" />
      <span class="ml-2 text-gray-500">按文件大小自动分段</span>
    </el-form-item>
    <el-form-item v-if="form.segmentEnabled" label="分段大小">
      <el-input-number
        v-model="form.segmentSize"
        :min="100"
        :max="4096"
        :step="100"
      />
      <span class="ml-2">MB</span>
    </el-form-item>
    <el-form-item label="自动清理">
      <el-switch v-model="form.autoCleanup" />
      <span class="ml-2 text-gray-500">自动删除旧录制文件</span>
    </el-form-item>
    <el-form-item v-if="form.autoCleanup" label="保留天数">
      <el-input-number
        v-model="form.cleanupDays"
        :min="1"
        :max="365"
        :step="1"
      />
      <span class="ml-2">天</span>
    </el-form-item>

    <el-divider content-position="left">
      高光配置
    </el-divider>

    <el-form-item label="启用高光标记">
      <el-switch v-model="form.highlightEnabled" />
    </el-form-item>
    <el-form-item v-if="form.highlightEnabled" label="弹幕激增阈值">
      <el-input-number
        v-model="form.danmakuThreshold"
        :min="10"
        :max="100"
        :step="5"
      />
      <span class="ml-2">条/10秒</span>
    </el-form-item>
    <el-form-item v-if="form.highlightEnabled" label="礼物价值阈值">
      <el-input-number
        v-model="form.giftValueThreshold"
        :min="10"
        :max="10000"
        :step="100"
      />
      <span class="ml-2">舰长数</span>
    </el-form-item>
    <el-form-item v-if="form.highlightEnabled" label="舰长触发">
      <el-switch v-model="form.captainTrigger" />
    </el-form-item>
    <el-form-item v-if="form.highlightEnabled" label="高光片段时长">
      <el-input-number
        v-model="form.clipDuration"
        :min="10"
        :max="120"
        :step="5"
      />
      <span class="ml-2">秒</span>
    </el-form-item>
  </el-form>
</template>
