<script setup lang="ts">
import { ref, onMounted, computed } from 'vue';
import { VideoCamera, VideoPause, VideoPlay, Setting, Star } from '@element-plus/icons-vue';
import { useRecordingStore } from '../../stores/recording';
import { useRoomStore } from '../../stores/room';
import RecordingList from './components/RecordingList.vue';
import HighlightList from './components/HighlightList.vue';
import {
  buildRecordingStartOptions,
  canStartRecording
} from './utils/recording-context.js';

const recordingStore = useRecordingStore();
const roomStore = useRoomStore();

const activeTab = ref('recordings');
const configDialogVisible = ref(false);

// 计算属性
const isRecording = computed(() => recordingStore.isRecording);
const isPaused = computed(() => recordingStore.isPaused);
const isIdle = computed(() => recordingStore.isIdle);

// 方法
async function handleStartRecording() {
  try {
    // 检查 FFmpeg
    const ffmpegInfo = await recordingStore.checkFFmpeg();
    if (!ffmpegInfo.available) {
      ElMessage.error('FFmpeg 未安装，请先安装 FFmpeg');
      return;
    }

    const roomContext = roomStore.roomInfo
      ? {
          roomId: String(roomStore.roomInfo.roomId),
          roomName: roomStore.roomInfo.roomName
        }
      : null;
    const canStart = canStartRecording(roomContext);
    if (!canStart.ok) {
      ElMessage.error(canStart.message);
      return;
    }

    await recordingStore.startRecording(
      buildRecordingStartOptions(roomContext, recordingStore.config)
    );

    ElMessage.success('开始录制');
  } catch (error) {
    ElMessage.error('开始录制失败: ' + (error as Error).message);
  }
}

async function handlePauseRecording() {
  try {
    await recordingStore.pauseRecording();
    ElMessage.success('录制已暂停');
  } catch (error) {
    ElMessage.error('暂停录制失败: ' + (error as Error).message);
  }
}

async function handleResumeRecording() {
  try {
    await recordingStore.resumeRecording();
    ElMessage.success('录制已恢复');
  } catch (error) {
    ElMessage.error('恢复录制失败: ' + (error as Error).message);
  }
}

async function handleStopRecording() {
  try {
    await recordingStore.stopRecording();
    ElMessage.success('录制已停止');
  } catch (error) {
    ElMessage.error('停止录制失败: ' + (error as Error).message);
  }
}

async function handleMarkHighlight() {
  try {
    await recordingStore.markHighlight('manual', '手动标记');
    ElMessage.success('高光已标记');
  } catch (error) {
    ElMessage.error('标记高光失败: ' + (error as Error).message);
  }
}

function handleOpenConfig() {
  configDialogVisible.value = true;
}

// 生命周期
onMounted(async () => {
  await recordingStore.fetchRecordings();
  await recordingStore.fetchHighlights();
  await recordingStore.fetchConfig();

  // 监听 F9 快捷键
  window.electronAPI.on('recording:markHighlightHotkey', () => {
    if (recordingStore.isRecording) {
      handleMarkHighlight();
    }
  });
});
</script>

<template>
  <div class="recording-page p-8 h-[calc(100vh-100px)]">
    <!-- 顶部操作区 -->
    <div class="flex items-center justify-between mb-6">
      <div class="flex items-center gap-4">
        <h2 class="text-2xl font-bold">
          录制管理
        </h2>

        <!-- 录制控制按钮 -->
        <div class="flex items-center gap-2">
          <el-button
            v-if="isIdle"
            type="danger"
            :icon="VideoCamera"
            @click="handleStartRecording"
          >
            开始录制
          </el-button>

          <template v-if="isRecording || isPaused">
            <el-button
              v-if="isRecording"
              type="warning"
              :icon="VideoPause"
              @click="handlePauseRecording"
            >
              暂停
            </el-button>
            <el-button
              v-if="isPaused"
              type="success"
              :icon="VideoPlay"
              @click="handleResumeRecording"
            >
              继续
            </el-button>
            <el-button
              type="info"
              :icon="VideoCamera"
              @click="handleStopRecording"
            >
              停止
            </el-button>
            <el-button
              type="primary"
              :icon="Star"
              @click="handleMarkHighlight"
            >
              标记高光
            </el-button>
          </template>
        </div>
      </div>

      <!-- 配置按钮 -->
      <el-button
        :icon="Setting"
        @click="handleOpenConfig"
      >
        录制配置
      </el-button>
    </div>

    <!-- 录制状态信息 -->
    <el-card
      v-if="isRecording || isPaused"
      class="mb-6"
    >
      <div class="flex items-center justify-between">
        <div class="flex items-center gap-6">
          <div class="flex items-center gap-2">
            <el-icon
              :class="isRecording ? 'text-red-500 animate-pulse' : 'text-yellow-500'"
              class="text-xl"
            >
              <VideoCamera />
            </el-icon>
            <span class="font-medium">
              {{ isRecording ? '录制中' : '已暂停' }}
            </span>
          </div>

          <div class="flex items-center gap-2">
            <span class="text-gray-500">时长:</span>
            <span class="font-mono text-lg">
              {{ recordingStore.formattedDuration }}
            </span>
          </div>

          <div class="flex items-center gap-2">
            <span class="text-gray-500">大小:</span>
            <span class="font-mono text-lg">
              {{ recordingStore.formattedFileSize }}
            </span>
          </div>

          <div
            v-if="recordingStore.highlightCount > 0"
            class="flex items-center gap-2"
          >
            <span class="text-gray-500">高光:</span>
            <span class="font-mono text-lg text-yellow-500">
              {{ recordingStore.highlightCount }}
            </span>
          </div>
        </div>
      </div>
    </el-card>

    <!-- 标签页 -->
    <el-tabs v-model="activeTab">
      <el-tab-pane label="录制文件" name="recordings">
        <RecordingList
          :recordings="recordingStore.recordings"
          :loading="recordingStore.isLoading"
        />
      </el-tab-pane>

      <el-tab-pane label="高光时刻" name="highlights">
        <HighlightList
          :highlights="recordingStore.highlights"
          :loading="recordingStore.isLoading"
        />
      </el-tab-pane>
    </el-tabs>

    <!-- 配置对话框 -->
    <el-dialog
      v-model="configDialogVisible"
      title="录制配置"
      width="600px"
    >
      <el-form :model="recordingStore.config" label-width="120px">
        <el-form-item label="录制格式">
          <el-select v-model="recordingStore.config.format">
            <el-option label="MP4" value="mp4" />
            <el-option label="FLV" value="flv" />
          </el-select>
        </el-form-item>

        <el-form-item label="录制质量">
          <el-select v-model="recordingStore.config.quality">
            <el-option label="原始质量" value="original" />
            <el-option label="720p" value="720p" />
            <el-option label="1080p" value="1080p" />
          </el-select>
        </el-form-item>

        <el-form-item label="存储路径">
          <el-input v-model="recordingStore.config.outputPath" placeholder="留空使用默认路径" />
        </el-form-item>

        <el-form-item label="自动分段">
          <el-switch v-model="recordingStore.config.segmentSize" :active-value="2048" :inactive-value="0" />
          <span class="ml-2">按 2GB 分段</span>
        </el-form-item>

        <el-form-item label="自动清理">
          <el-switch v-model="recordingStore.config.autoCleanup" />
          <span class="ml-2">保留 {{ recordingStore.config.cleanupDays }} 天</span>
        </el-form-item>

        <el-divider content-position="left">
          高光配置
        </el-divider>

        <el-form-item label="启用高光">
          <el-switch v-model="recordingStore.config.highlightEnabled" />
        </el-form-item>

        <el-form-item label="弹幕阈值">
          <el-input-number
            v-model="recordingStore.config.highlightConfig.danmakuThreshold"
            :min="10"
            :max="100"
          />
          <span class="ml-2">条/{{ recordingStore.config.highlightConfig.danmakuWindow }}秒</span>
        </el-form-item>

        <el-form-item label="礼物阈值">
          <el-input-number
            v-model="recordingStore.config.highlightConfig.giftValueThreshold"
            :min="10"
            :max="10000"
          />
          <span class="ml-2">舰长数</span>
        </el-form-item>

        <el-form-item label="舰长触发">
          <el-switch v-model="recordingStore.config.highlightConfig.captainTrigger" />
        </el-form-item>
      </el-form>

      <template #footer>
        <el-button @click="configDialogVisible = false">
          取消
        </el-button>
        <el-button type="primary" @click="recordingStore.updateConfig(recordingStore.config); configDialogVisible = false">
          保存
        </el-button>
      </template>
    </el-dialog>
  </div>
</template>
