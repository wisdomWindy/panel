<script setup lang="ts">
import { ref, reactive, computed } from 'vue';
import { Connection, ChatDotRound, Timer, Filter, Document, UserFilled } from '@element-plus/icons-vue';
import { useConfigStore } from '../stores/config';
import { getDanmakuService } from '../services/danmaku-service';

const danmakuService = getDanmakuService();

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
    roles: '角色回复设置'
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
    enableTriggerReply: configStore.config.enableTriggerReply
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
    enableSensitiveWordFilter: form.filter.enableSensitiveWordFilter,
    replaceChar: form.filter.replaceChar,
    sensitiveWords: form.filter.sensitiveWords.split('\n').filter((word: string) => word.trim()),
    enableLog: form.log.enableLog,
    logLevel: form.log.logLevel,
    maxLogLines: form.log.maxLogLines
  });

  danmakuService.updateMinInterval(form.danmaku.minSendInterval);

  ElMessage.success('设置已保存');
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
