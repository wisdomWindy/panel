<script setup lang="ts">
import { ref, reactive, computed, onMounted } from 'vue';
import { Connection, ChatDotRound, Timer, Filter, Document, UserFilled, Bell, Setting, VideoCamera } from '@element-plus/icons-vue';
import { useConfigStore } from '../stores/config';
import { getDanmakuService } from '../services/danmaku-service';
import { useRouter } from 'vue-router';
import { useI18n } from 'vue-i18n';
import { useTTSStore } from '../stores/tts';
import { useASRStore } from '../stores/asr';
import { EDGE_TTS_VOICES } from '../modules/tts/types';

const danmakuService = getDanmakuService();
const router = useRouter();
const { locale } = useI18n();
const ttsStore = useTTSStore();
const asrStore = useASRStore();
const edgeTTSVoices = EDGE_TTS_VOICES;

const configStore = useConfigStore();

const activeTab = ref('connection');
const importDialogVisible = ref(false);
const importConfig = ref('');

const tabTitle = computed(() => {
  const titles: Record<string, string> = {
    connection: '连接设置',
    danmaku: '弹幕设置',
    auto: '自动发送',
    filter: '过滤设置',
    log: '日志设置',
    roles: '角色回复设置',
    update: '更新设置',
    plugins: '插件管理',
    recording: '录制设置',
    language: '语言设置',
    tts: '语音播报',
    asr: '语音识别'
  };
  return titles[activeTab.value] || '设置';
});

const form = reactive({
  connection: {
    autoReconnect: configStore.config.autoReconnect,
    reconnectInterval: configStore.config.reconnectInterval,
    maxReconnectAttempts: configStore.config.maxReconnectAttempts
  },
  danmaku: {
    minSendInterval: configStore.config.minSendInterval,
    enableGiftThank: configStore.config.enableGiftThank,
    giftThankMessage: '感谢 {username} 送的 {giftName}!'
  },
  auto: {
    enableAutoSend: configStore.config.enableAutoSend,
    autoSendInterval: configStore.config.autoSendInterval,
    enableTriggerReply: configStore.config.enableTriggerReply,
    enableAutoRecord: configStore.config.enableAutoRecord
  },
  filter: {
    enableSensitiveWordFilter: configStore.config.enableSensitiveWordFilter,
    replaceChar: configStore.config.replaceChar,
    sensitiveWords: configStore.config.sensitiveWords.join('\n')
  },
  log: {
    enableLog: configStore.config.enableLog,
    logLevel: configStore.config.logLevel,
    maxLogLines: configStore.config.maxLogLines
  },
  roles: {
    ownerEnabled: true,
    ownerTemplate: '欢迎 {username} 主播！',
    ownerCooldown: 300,
    adminEnabled: true,
    adminTemplate: '房管 {username} 来了！',
    adminCooldown: 120,
    captainEnabled: true,
    captainTemplate: '感谢舰长 {username} 的支持！',
    captainCooldown: 60
  },
  update: {
    checkUpdates: configStore.config.checkUpdates
  },
  recording: {
    enabled: false,
    autoStart: false,
    format: 'mp4',
    quality: 'original',
    outputPath: '',
    segmentEnabled: false,
    segmentSize: 2048,
    autoCleanup: false,
    cleanupDays: 30,
    highlightEnabled: true,
    danmakuThreshold: 30,
    giftValueThreshold: 100,
    captainTrigger: true,
    clipDuration: 30
  },
  language: {
    locale: configStore.config.language || 'zh-CN'
  },
  tts: {
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
  },
  asr: {
    enabled: asrStore.config.enabled,
    engine: asrStore.config.engine || 'whisper-cpp',
    language: asrStore.config.language || 'zh',
    model: asrStore.config.model || 'base',
    continuous: asrStore.config.continuous,
    interimResults: asrStore.config.interimResults,
    noiseReduction: asrStore.config.noiseReduction,
    autoStart: asrStore.config.autoStart
  }
});

function handleTabSelect(tab: string) {
  activeTab.value = tab;
}

function handleSave() {
  configStore.updateConfig({
    autoReconnect: form.connection.autoReconnect,
    reconnectInterval: form.connection.reconnectInterval,
    maxReconnectAttempts: form.connection.maxReconnectAttempts,
    minSendInterval: form.danmaku.minSendInterval,
    enableGiftThank: form.danmaku.enableGiftThank,
    enableAutoSend: form.auto.enableAutoSend,
    autoSendInterval: form.auto.autoSendInterval,
    enableTriggerReply: form.auto.enableTriggerReply,
    enableAutoRecord: form.auto.enableAutoRecord,
    enableSensitiveWordFilter: form.filter.enableSensitiveWordFilter,
    replaceChar: form.filter.replaceChar,
    sensitiveWords: form.filter.sensitiveWords.split('\n').filter((word: string) => word.trim()),
    enableLog: form.log.enableLog,
    logLevel: form.log.logLevel,
    maxLogLines: form.log.maxLogLines,
    checkUpdates: form.update.checkUpdates,
    language: form.language.locale as 'zh-CN' | 'en-US'
  });

  // 保存 TTS 配置
  ttsStore.updateConfig({
    enabled: form.tts.enabled,
    engine: form.tts.engine,
    speed: form.tts.speed,
    volume: form.tts.volume,
    minInterval: form.tts.minInterval,
    formatTemplate: form.tts.formatTemplate,
    broadcastGifts: form.tts.broadcastGifts,
    giftValueThreshold: form.tts.giftValueThreshold,
    broadcastCaptain: form.tts.broadcastCaptain,
    filterKeywords: form.tts.filterKeywordsText.split('\n').filter((word: string) => word.trim()),
    edgeTTSVoice: form.tts.edgeTTSVoice
  });

  // 保存 ASR 配置
  asrStore.updateConfig({
    enabled: form.asr.enabled,
    engine: form.asr.engine,
    language: form.asr.language,
    model: form.asr.model,
    continuous: form.asr.continuous,
    interimResults: form.asr.interimResults,
    noiseReduction: form.asr.noiseReduction,
    autoStart: form.asr.autoStart
  });

  danmakuService.updateMinInterval(form.danmaku.minSendInterval);

  ElMessage.success('设置已保存');
}

function handleLanguageChange(lang: string) {
  locale.value = lang;
  localStorage.setItem('locale', lang);
  configStore.updateConfig({ language: lang as 'zh-CN' | 'en-US' });
}

function handleReset() {
  location.reload();
}

function handleExport() {
  const config = {
    connection: form.connection,
    danmaku: form.danmaku,
    auto: form.auto,
    filter: form.filter,
    log: form.log,
    roles: form.roles
  };

  const configText = JSON.stringify(config, null, 2);
  const dataBlob = new Blob([configText], { type: 'application/json' });
  const url = URL.createObjectURL(dataBlob);
  const link = document.createElement('a');
  link.href = url;
  link.download = `bilibili-live-config-${Date.now()}.json`;
  link.click();
  URL.revokeObjectURL(url);
  ElMessage.success('配置已导出');
}

function handleImport() {
  importDialogVisible.value = true;
}

function confirmImport() {
  try {
    const config = JSON.parse(importConfig.value);

    Object.assign(form.connection, config.connection || {});
    Object.assign(form.danmaku, config.danmaku || {});
    Object.assign(form.auto, config.auto || {});
    Object.assign(form.filter, config.filter || {});
    Object.assign(form.log, config.log || {});
    Object.assign(form.roles, config.roles || {});

    handleSave();
    importDialogVisible.value = false;
    ElMessage.success('配置已导入');
  } catch (error) {
    ElMessage.error('配置文件格式错误');
  }
}
</script>

<template>
  <div class="p-8 h-[calc(100vh-100px)]">
    <el-row :gutter="20">
      <el-col :span="6">
        <el-card>
          <template #header>
            <span>设置菜单</span>
          </template>

          <el-menu :default-active="activeTab" @select="handleTabSelect">
            <el-menu-item index="connection">
              <el-icon><Connection /></el-icon>
              <span>连接设置</span>
            </el-menu-item>
            <el-menu-item index="danmaku">
              <el-icon><ChatDotRound /></el-icon>
              <span>弹幕设置</span>
            </el-menu-item>
            <el-menu-item index="auto">
              <el-icon><Timer /></el-icon>
              <span>自动发送</span>
            </el-menu-item>
            <el-menu-item index="filter">
              <el-icon><Filter /></el-icon>
              <span>过滤设置</span>
            </el-menu-item>
            <el-menu-item index="roles">
              <el-icon><UserFilled /></el-icon>
              <span>角色回复</span>
            </el-menu-item>
            <el-menu-item index="log">
              <el-icon><Document /></el-icon>
              <span>日志设置</span>
            </el-menu-item>
            <el-menu-item index="update">
              <el-icon><Bell /></el-icon>
              <span>更新设置</span>
            </el-menu-item>
            <el-menu-item index="plugins">
              <el-icon><Setting /></el-icon>
              <span>插件管理</span>
            </el-menu-item>
            <el-menu-item index="recording">
              <el-icon><VideoCamera /></el-icon>
              <span>录制设置</span>
            </el-menu-item>
            <el-menu-item index="language">
              <el-icon><Setting /></el-icon>
              <span>语言设置</span>
            </el-menu-item>
            <el-menu-item index="tts">
              <el-icon><Setting /></el-icon>
              <span>语音播报</span>
            </el-menu-item>
            <el-menu-item index="asr">
              <el-icon><Setting /></el-icon>
              <span>语音识别</span>
            </el-menu-item>
          </el-menu>
        </el-card>
      </el-col>
      <el-col :span="18">
        <el-card class="min-h-[500px]">
          <template #header>
            <span>{{ tabTitle }}</span>
          </template>

          <!-- 连接设置 -->
          <div v-show="activeTab === 'connection'">
            <el-form :model="form.connection" label-width="150px">
              <el-form-item label="自动重连">
                <el-switch v-model="form.connection.autoReconnect" />
              </el-form-item>
              <el-form-item label="重连间隔">
                <el-input-number
                  v-model="form.connection.reconnectInterval"
                  :min="1000"
                  :max="60000"
                  :step="1000"
                />
                <span class="ml-2">毫秒</span>
              </el-form-item>
              <el-form-item label="最大重连次数">
                <el-input-number
                  v-model="form.connection.maxReconnectAttempts"
                  :min="1"
                  :max="20"
                />
              </el-form-item>
            </el-form>
          </div>

          <!-- 弹幕设置 -->
          <div v-show="activeTab === 'danmaku'">
            <el-form :model="form.danmaku" label-width="150px">
              <el-form-item label="最小发送间隔">
                <el-input-number
                  v-model="form.danmaku.minSendInterval"
                  :min="1000"
                  :max="60000"
                  :step="1000"
                />
                <span class="ml-2">毫秒</span>
              </el-form-item>
              <el-form-item label="感谢礼物">
                <el-switch v-model="form.danmaku.enableGiftThank" />
              </el-form-item>
              <el-form-item label="感谢模板">
                <el-input
                  v-model="form.danmaku.giftThankMessage"
                  type="textarea"
                  :rows="3"
                  placeholder="感谢礼物模板，可使用 {username} 和 {giftName} 变量"
                />
              </el-form-item>
            </el-form>
          </div>

          <!-- 自动发送 -->
          <div v-show="activeTab === 'auto'">
            <el-form :model="form.auto" label-width="150px">
              <el-form-item label="启用自动发送">
                <el-switch v-model="form.auto.enableAutoSend" />
              </el-form-item>
              <el-form-item label="默认间隔">
                <el-input-number
                  v-model="form.auto.autoSendInterval"
                  :min="1000"
                  :max="300000"
                  :step="1000"
                />
                <span class="ml-2">毫秒</span>
              </el-form-item>
              <el-form-item label="触发回复">
                <el-switch v-model="form.auto.enableTriggerReply" />
              </el-form-item>
              <el-divider content-position="left">
                自动录制
              </el-divider>
              <el-form-item label="启用自动录制">
                <el-switch v-model="form.auto.enableAutoRecord" />
              </el-form-item>
              <el-form-item>
                <el-text type="info" size="small">
                  连接直播间且开播时自动开始录制，下播时自动停止录制
                </el-text>
              </el-form-item>
            </el-form>
          </div>

          <!-- 过滤设置 -->
          <div v-show="activeTab === 'filter'">
            <el-form :model="form.filter" label-width="150px">
              <el-form-item label="敏感词过滤">
                <el-switch v-model="form.filter.enableSensitiveWordFilter" />
              </el-form-item>
              <el-form-item label="替换字符">
                <el-input v-model="form.filter.replaceChar" placeholder="替换敏感词的字符" />
              </el-form-item>
              <el-form-item label="敏感词列表">
                <el-input
                  v-model="form.filter.sensitiveWords"
                  type="textarea"
                  :rows="5"
                  placeholder="每行一个敏感词"
                />
              </el-form-item>
            </el-form>
          </div>

          <!-- 角色回复设置 -->
          <div v-show="activeTab === 'roles'">
            <el-form :model="form.roles" label-width="150px">
              <el-divider content-position="left">
                主播回复
              </el-divider>
              <el-form-item label="启用">
                <el-switch v-model="form.roles.ownerEnabled" />
              </el-form-item>
              <el-form-item label="回复模板">
                <el-input v-model="form.roles.ownerTemplate" placeholder="支持 {username}, {userType} 变量" />
              </el-form-item>
              <el-form-item label="冷却时间">
                <el-input-number
                  v-model="form.roles.ownerCooldown"
                  :min="10"
                  :max="3600"
                  :step="10"
                />
                <span class="ml-2">秒</span>
              </el-form-item>

              <el-divider content-position="left">
                房管回复
              </el-divider>
              <el-form-item label="启用">
                <el-switch v-model="form.roles.adminEnabled" />
              </el-form-item>
              <el-form-item label="回复模板">
                <el-input v-model="form.roles.adminTemplate" placeholder="支持 {username}, {userType} 变量" />
              </el-form-item>
              <el-form-item label="冷却时间">
                <el-input-number
                  v-model="form.roles.adminCooldown"
                  :min="10"
                  :max="3600"
                  :step="10"
                />
                <span class="ml-2">秒</span>
              </el-form-item>

              <el-divider content-position="left">
                舰长回复
              </el-divider>
              <el-form-item label="启用">
                <el-switch v-model="form.roles.captainEnabled" />
              </el-form-item>
              <el-form-item label="回复模板">
                <el-input v-model="form.roles.captainTemplate" placeholder="支持 {username}, {userType} 变量" />
              </el-form-item>
              <el-form-item label="冷却时间">
                <el-input-number
                  v-model="form.roles.captainCooldown"
                  :min="10"
                  :max="3600"
                  :step="10"
                />
                <span class="ml-2">秒</span>
              </el-form-item>
            </el-form>
          </div>

          <!-- 日志设置 -->
          <div v-show="activeTab === 'log'">
            <el-form :model="form.log" label-width="150px">
              <el-form-item label="启用日志">
                <el-switch v-model="form.log.enableLog" />
              </el-form-item>
              <el-form-item label="日志级别">
                <el-select v-model="form.log.logLevel" width="150">
                  <el-option label="Debug" value="debug" />
                  <el-option label="Info" value="info" />
                  <el-option label="Warning" value="warn" />
                  <el-option label="Error" value="error" />
                </el-select>
              </el-form-item>
              <el-form-item label="最大日志行数">
                <el-input-number
                  v-model="form.log.maxLogLines"
                  :min="100"
                  :max="10000"
                  :step="100"
                />
              </el-form-item>
            </el-form>
          </div>

          <!-- 更新设置 -->
          <div v-show="activeTab === 'update'">
            <el-form :model="form.update" label-width="150px">
              <el-form-item label="自动检查更新">
                <el-switch v-model="form.update.checkUpdates" />
              </el-form-item>
              <el-form-item>
                <el-button type="primary" @click="router.push('/update-log')">
                  查看更新日志
                </el-button>
              </el-form-item>
            </el-form>
          </div>

          <!-- 插件管理 -->
          <div v-show="activeTab === 'plugins'">
            <div class="plugin-management">
              <el-alert
                title="插件管理"
                description="安装和管理第三方插件，扩展应用功能"
                type="info"
                show-icon
                :closable="false"
                class="mb-4"
              />
              <el-button type="primary" @click="router.push('/plugins')">
                打开插件管理
              </el-button>
            </div>
          </div>

          <!-- 录制设置 -->
          <div v-show="activeTab === 'recording'">
            <el-form :model="form.recording" label-width="150px">
              <el-form-item label="启用录制功能">
                <el-switch v-model="form.recording.enabled" />
              </el-form-item>
              <el-form-item label="自动开始录制">
                <el-switch v-model="form.recording.autoStart" />
                <span class="ml-2 text-gray-500">连接直播间后自动开始录制</span>
              </el-form-item>
              <el-form-item label="录制格式">
                <el-select v-model="form.recording.format">
                  <el-option label="MP4" value="mp4" />
                  <el-option label="FLV" value="flv" />
                </el-select>
              </el-form-item>
              <el-form-item label="录制质量">
                <el-select v-model="form.recording.quality">
                  <el-option label="原始质量" value="original" />
                  <el-option label="720p" value="720p" />
                  <el-option label="1080p" value="1080p" />
                </el-select>
              </el-form-item>
              <el-form-item label="存储路径">
                <el-input v-model="form.recording.outputPath" placeholder="留空使用默认路径" />
              </el-form-item>
              <el-form-item label="自动分段">
                <el-switch v-model="form.recording.segmentEnabled" />
                <span class="ml-2 text-gray-500">按文件大小自动分段</span>
              </el-form-item>
              <el-form-item v-if="form.recording.segmentEnabled" label="分段大小">
                <el-input-number
                  v-model="form.recording.segmentSize"
                  :min="100"
                  :max="4096"
                  :step="100"
                />
                <span class="ml-2">MB</span>
              </el-form-item>
              <el-form-item label="自动清理">
                <el-switch v-model="form.recording.autoCleanup" />
                <span class="ml-2 text-gray-500">自动删除旧录制文件</span>
              </el-form-item>
              <el-form-item v-if="form.recording.autoCleanup" label="保留天数">
                <el-input-number
                  v-model="form.recording.cleanupDays"
                  :min="1"
                  :max="365"
                  :step="1"
                />
                <span class="ml-2">天</span>
              </el-form-item>

              <el-divider content-position="left">高光配置</el-divider>

              <el-form-item label="启用高光标记">
                <el-switch v-model="form.recording.highlightEnabled" />
              </el-form-item>
              <el-form-item v-if="form.recording.highlightEnabled" label="弹幕激增阈值">
                <el-input-number
                  v-model="form.recording.danmakuThreshold"
                  :min="10"
                  :max="100"
                  :step="5"
                />
                <span class="ml-2">条/10秒</span>
              </el-form-item>
              <el-form-item v-if="form.recording.highlightEnabled" label="礼物价值阈值">
                <el-input-number
                  v-model="form.recording.giftValueThreshold"
                  :min="10"
                  :max="10000"
                  :step="100"
                />
                <span class="ml-2">舰长数</span>
              </el-form-item>
              <el-form-item v-if="form.recording.highlightEnabled" label="舰长触发">
                <el-switch v-model="form.recording.captainTrigger" />
              </el-form-item>
              <el-form-item v-if="form.recording.highlightEnabled" label="高光片段时长">
                <el-input-number
                  v-model="form.recording.clipDuration"
                  :min="10"
                  :max="120"
                  :step="5"
                />
                <span class="ml-2">秒</span>
              </el-form-item>
            </el-form>
          </div>

          <!-- 语言设置 -->
          <div v-show="activeTab === 'language'">
            <el-form :model="form.language" label-width="150px">
              <el-form-item label="界面语言">
                <el-select v-model="form.language.locale" @change="handleLanguageChange">
                  <el-option label="中文" value="zh-CN" />
                  <el-option label="English" value="en-US" />
                </el-select>
              </el-form-item>
              <el-form-item>
                <el-alert
                  title="切换语言后界面将立即更新"
                  type="info"
                  show-icon
                  :closable="false"
                />
              </el-form-item>
            </el-form>
          </div>

          <!-- 语音播报设置 -->
          <div v-show="activeTab === 'tts'">
            <el-form :model="form.tts" label-width="150px">
              <el-form-item label="启用语音播报">
                <el-switch v-model="form.tts.enabled" />
                <span class="ml-2 text-gray-500">开启后将自动播报弹幕内容</span>
              </el-form-item>
              <el-form-item label="TTS 引擎">
                <el-select v-model="form.tts.engine">
                  <el-option label="Edge TTS（推荐，高质量）" value="edge-tts" />
                  <el-option label="Web Speech API（浏览器内置）" value="local" />
                </el-select>
              </el-form-item>
              <el-form-item v-if="form.tts.engine === 'edge-tts'" label="语音选择">
                <el-select v-model="form.tts.edgeTTSVoice">
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
                  v-model="form.tts.speed"
                  :min="0.5"
                  :max="2"
                  :step="0.1"
                  show-input
                />
              </el-form-item>
              <el-form-item label="播报音量">
                <el-slider
                  v-model="form.tts.volume"
                  :min="0"
                  :max="1"
                  :step="0.1"
                  show-input
                />
              </el-form-item>
              <el-form-item label="最小播报间隔">
                <el-input-number
                  v-model="form.tts.minInterval"
                  :min="500"
                  :max="10000"
                  :step="500"
                />
                <span class="ml-2">毫秒</span>
              </el-form-item>
              <el-form-item label="播报格式">
                <el-input
                  v-model="form.tts.formatTemplate"
                  placeholder="{username}说：{content}"
                />
                <span class="ml-2 text-gray-500">可用变量: {username}, {content}</span>
              </el-form-item>
              <el-form-item label="礼物播报">
                <el-switch v-model="form.tts.broadcastGifts" />
              </el-form-item>
              <el-form-item v-if="form.tts.broadcastGifts" label="礼物价值阈值">
                <el-input-number
                  v-model="form.tts.giftValueThreshold"
                  :min="0"
                  :max="10000"
                  :step="100"
                />
                <span class="ml-2">舰长数</span>
              </el-form-item>
              <el-form-item label="舰长播报">
                <el-switch v-model="form.tts.broadcastCaptain" />
              </el-form-item>
              <el-form-item label="过滤关键词">
                <el-input
                  v-model="form.tts.filterKeywordsText"
                  type="textarea"
                  :rows="3"
                  placeholder="每行一个关键词，包含这些词的弹幕将不会被播报"
                />
              </el-form-item>
            </el-form>
          </div>

          <!-- 语音识别设置 -->
          <div v-show="activeTab === 'asr'">
            <el-form :model="form.asr" label-width="150px">
              <el-form-item label="启用语音识别">
                <el-switch v-model="form.asr.enabled" />
                <span class="ml-2 text-gray-500">开启后可将语音转换为文字</span>
              </el-form-item>
              <el-form-item label="识别引擎">
                <el-select v-model="form.asr.engine">
                  <el-option label="whisper.cpp（推荐，高质量）" value="whisper-cpp" />
                  <el-option label="Web Speech API（浏览器内置）" value="web-speech" />
                </el-select>
              </el-form-item>
              <el-form-item label="识别语言">
                <el-select v-model="form.asr.language">
                  <el-option label="中文" value="zh" />
                  <el-option label="English" value="en" />
                  <el-option label="日本語" value="ja" />
                  <el-option label="한국어" value="ko" />
                  <el-option label="自动检测" value="auto" />
                </el-select>
              </el-form-item>
              <el-form-item v-if="form.asr.engine === 'whisper-cpp'" label="模型大小">
                <el-select v-model="form.asr.model">
                  <el-option label="tiny（最快，准确率较低）" value="tiny" />
                  <el-option label="base（平衡速度和准确率）" value="base" />
                  <el-option label="small（较好的准确率）" value="small" />
                  <el-option label="medium（高准确率）" value="medium" />
                  <el-option label="large（最高准确率）" value="large" />
                </el-select>
              </el-form-item>
              <el-form-item label="连续识别">
                <el-switch v-model="form.asr.continuous" />
                <span class="ml-2 text-gray-500">持续监听语音输入</span>
              </el-form-item>
              <el-form-item label="显示中间结果">
                <el-switch v-model="form.asr.interimResults" />
                <span class="ml-2 text-gray-500">实时显示识别过程</span>
              </el-form-item>
              <el-form-item label="降噪处理">
                <el-switch v-model="form.asr.noiseReduction" />
                <span class="ml-2 text-gray-500">自动过滤背景噪音</span>
              </el-form-item>
              <el-form-item label="自动开始">
                <el-switch v-model="form.asr.autoStart" />
                <span class="ml-2 text-gray-500">连接直播间后自动开始识别</span>
              </el-form-item>
              <el-form-item>
                <el-alert
                  v-if="form.asr.engine === 'whisper-cpp'"
                  title="whisper.cpp 需要单独安装"
                  description="请运行: brew install whisper-cpp 或访问 https://github.com/ggerganov/whisper.cpp"
                  type="info"
                  show-icon
                  :closable="false"
                />
              </el-form-item>
            </el-form>
          </div>

          <div class="mt-5 flex justify-end gap-2">
            <el-button type="primary" @click="handleSave">
              保存设置
            </el-button>
            <el-button @click="handleReset">
              重置
            </el-button>
            <el-button @click="handleExport">
              导出配置
            </el-button>
            <el-button @click="handleImport">
              导入配置
            </el-button>
          </div>
        </el-card>
      </el-col>
    </el-row>

    <!-- 导入配置对话框 -->
    <el-dialog v-model="importDialogVisible" title="导入配置" width="500px">
      <el-form>
        <el-form-item label="配置文件">
          <el-input
            v-model="importConfig"
            type="textarea"
            :rows="10"
            placeholder="粘贴配置文件内容"
          />
        </el-form-item>
      </el-form>
      <template #footer>
        <span class="dialog-footer">
          <el-button @click="importDialogVisible = false">
            取消
          </el-button>
          <el-button type="primary" @click="confirmImport">
            导入
          </el-button>
        </span>
      </template>
    </el-dialog>
  </div>
</template>
