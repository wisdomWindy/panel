<script setup lang="ts">
import { onMounted, onUnmounted, ref } from 'vue';
import { useI18n } from 'vue-i18n';
import { useRoomStore } from '@/stores/room';
import { useUpdateStore } from '@/stores/update';
import { getDanmakuService } from '@/services/danmaku-service';
import {
  House,
  Connection,
  ChatDotRound,
  Document,
  Present,
  Setting,
  DocumentCopy,
  CircleClose,
  MagicStick,
  Bell,
  Box,
  VideoCamera
} from '@element-plus/icons-vue';
import LoginPage from '@/views/Login.vue';
import WhatsNewModal from '@/components/WhatsNewModal.vue';
import RecordingStatusBar from '@/components/RecordingStatusBar.vue';

const { t } = useI18n();
const roomStore = useRoomStore();
const updateStore = useUpdateStore();
const danmakuService = getDanmakuService();

const showWhatsNew = ref(false);
const whatsNewVersion = ref('');

function handleBeforeUnload() {
  if (roomStore.isConnected) {
    danmakuService.disconnect();
  }
}

async function handleDisconnect() {
  try {
    await danmakuService.disconnect();
    ElMessage.info(t('connection.disconnected'));
  } catch (error) {
    ElMessage.error(`${t('connection.disconnect')}: ${error}`);
  }
}

function checkWhatsNew() {
  const currentVersion = updateStore.currentVersion;
  const lastWhatsNewVersion = localStorage.getItem('lastWhatsNewVersion');
  const dontRemindList = JSON.parse(localStorage.getItem('whatsNewDontRemind') || '[]');

  if (lastWhatsNewVersion !== currentVersion && !dontRemindList.includes(currentVersion)) {
    whatsNewVersion.value = currentVersion;
    showWhatsNew.value = true;
  }
}

function handleWhatsNewClose() {
  showWhatsNew.value = false;
  localStorage.setItem('lastWhatsNewVersion', whatsNewVersion.value);
}

function handleDontRemind() {
  const dontRemindList = JSON.parse(localStorage.getItem('whatsNewDontRemind') || '[]');
  if (!dontRemindList.includes(whatsNewVersion.value)) {
    dontRemindList.push(whatsNewVersion.value);
    localStorage.setItem('whatsNewDontRemind', JSON.stringify(dontRemindList));
  }
}

onMounted(async () => {
  window.addEventListener('beforeunload', handleBeforeUnload);

  // 异步检查更新（不阻塞主界面）
  await updateStore.checkForUpdates();

  // 检查是否需要显示新版本弹窗
  await updateStore.parseChangelog();
  checkWhatsNew();
});

onUnmounted(() => {
  window.removeEventListener('beforeunload', handleBeforeUnload);
});
</script>

<template>
  <div class="h-screen flex flex-col bg-[#F5F6FA] overflow-hidden">
    <!-- 未连接：全屏显示身份码输入页 -->
    <LoginPage v-if="!roomStore.isConnected" />

    <!-- 已连接：显示完整布局 -->
    <el-container v-else class="flex flex-col h-full">
      <el-header class="flex items-center justify-between shrink-0 h-14 bg-gradient-to-r from-[#1D3557] to-[#457B9D] shadow-lg">
        <h1 class="m-0 text-lg font-semibold tracking-wider text-white">
          {{ t('app.title') }}
        </h1>
        <div class="flex items-center">
          <el-badge :is-dot="updateStore.hasUpdate" class="mr-4">
            <el-button
              type="info"
              size="small"
              :icon="Bell"
              circle
              @click="$router.push('/update-log')"
            />
          </el-badge>
          <el-button
            type="danger"
            size="small"
            @click="handleDisconnect"
          >
            {{ t('connection.disconnect') }}
          </el-button>
        </div>
      </el-header>

      <el-container class="flex flex-1 min-h-0">
        <el-aside width="220px" class="shrink-0 bg-[#304156] overflow-y-auto">
          <el-menu
            :default-active="$route.path"
            router
            background-color="#304156"
            text-color="#bfcbd9"
            active-text-color="#409EFF"
            class="border-r-0"
          >
            <el-menu-item index="/dashboard">
              <el-icon><House /></el-icon>
              <span>{{ t('nav.dashboard') }}</span>
            </el-menu-item>
            <el-menu-item index="/connection">
              <el-icon><Connection /></el-icon>
              <span>{{ t('nav.connection') }}</span>
            </el-menu-item>
            <el-menu-item index="/danmaku-pool">
              <el-icon><ChatDotRound /></el-icon>
              <span>{{ t('nav.danmakuPool') }}</span>
            </el-menu-item>
            <el-menu-item index="/danmaku-list">
              <el-icon><Document /></el-icon>
              <span>{{ t('nav.danmakuList') }}</span>
            </el-menu-item>
            <el-menu-item index="/gift-list">
              <el-icon><Present /></el-icon>
              <span>{{ t('nav.giftList') }}</span>
            </el-menu-item>
            <el-menu-item index="/settings">
              <el-icon><Setting /></el-icon>
              <span>{{ t('nav.settings') }}</span>
            </el-menu-item>
            <el-menu-item index="/logs">
              <el-icon><DocumentCopy /></el-icon>
              <span>{{ t('nav.logs') }}</span>
            </el-menu-item>
            <el-menu-item index="/blacklist">
              <el-icon><CircleClose /></el-icon>
              <span>{{ t('nav.blacklist') }}</span>
            </el-menu-item>
            <el-menu-item index="/effect-preview">
              <el-icon><MagicStick /></el-icon>
              <span>特效预览</span>
            </el-menu-item>
            <el-menu-item index="/update-log">
              <el-icon><Bell /></el-icon>
              <span>{{ t('update.title') }}</span>
            </el-menu-item>
            <el-menu-item index="/plugins">
              <el-icon><Box /></el-icon>
              <span>{{ t('plugins.title') }}</span>
            </el-menu-item>
            <el-menu-item index="/recording">
              <el-icon><VideoCamera /></el-icon>
              <span>{{ t('recording.title') }}</span>
            </el-menu-item>
            <el-menu-item index="/gift-sound">
              <el-icon><Present /></el-icon>
              <span>礼物音效</span>
            </el-menu-item>
          </el-menu>
        </el-aside>

        <el-main class="p-8 overflow-y-auto flex-1 bg-[#F5F6FA]">
          <router-view />
        </el-main>
      </el-container>

      <!-- 录制状态栏 -->
      <RecordingStatusBar class="shrink-0" />
    </el-container>

    <!-- 新版本特性弹窗 -->
    <WhatsNewModal
      :visible="showWhatsNew"
      :version="whatsNewVersion"
      :changelog="updateStore.latestChangelogEntry"
      @close="handleWhatsNewClose"
      @dont-remind="handleDontRemind"
    />
  </div>
</template>
