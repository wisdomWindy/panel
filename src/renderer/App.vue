<script setup lang="ts">
import { onMounted, onUnmounted, ref } from 'vue';
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
  Bell
} from '@element-plus/icons-vue';
import LoginPage from '@/views/Login.vue';
import WhatsNewModal from '@/components/WhatsNewModal.vue';

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
    ElMessage.info('已断开连接');
  } catch (error) {
    ElMessage.error(`断开连接失败: ${error}`);
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
  <div class="min-h-screen flex flex-col bg-[#F5F6FA]">
    <!-- 未连接：全屏显示身份码输入页 -->
    <LoginPage v-if="!roomStore.isConnected" />

    <!-- 已连接：显示完整布局 -->
    <el-container v-else class="flex flex-col h-full">
      <el-header class="flex items-center justify-between shrink-0 h-14 bg-gradient-to-r from-[#1D3557] to-[#457B9D] shadow-lg">
        <h1 class="m-0 text-lg font-semibold tracking-wider text-white">
          Bilibili 直播助手
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
            断开连接
          </el-button>
        </div>
      </el-header>

      <el-container class="flex flex-1 min-h-0">
        <el-aside width="220px" class="shrink-0 bg-[#304156]">
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
              <span>仪表盘</span>
            </el-menu-item>
            <el-menu-item index="/connection">
              <el-icon><Connection /></el-icon>
              <span>连接配置</span>
            </el-menu-item>
            <el-menu-item index="/danmaku-pool">
              <el-icon><ChatDotRound /></el-icon>
              <span>弹幕池管理</span>
            </el-menu-item>
            <el-menu-item index="/danmaku-list">
              <el-icon><Document /></el-icon>
              <span>弹幕列表</span>
            </el-menu-item>
            <el-menu-item index="/gift-list">
              <el-icon><Present /></el-icon>
              <span>礼物列表</span>
            </el-menu-item>
            <el-menu-item index="/settings">
              <el-icon><Setting /></el-icon>
              <span>设置</span>
            </el-menu-item>
            <el-menu-item index="/logs">
              <el-icon><DocumentCopy /></el-icon>
              <span>日志监控</span>
            </el-menu-item>
            <el-menu-item index="/blacklist">
              <el-icon><CircleClose /></el-icon>
              <span>黑名单管理</span>
            </el-menu-item>
            <el-menu-item index="/effect-preview">
              <el-icon><MagicStick /></el-icon>
              <span>特效预览</span>
            </el-menu-item>
            <el-menu-item index="/update-log">
              <el-icon><Bell /></el-icon>
              <span>更新日志</span>
            </el-menu-item>
          </el-menu>
        </el-aside>

        <el-main class="p-8 overflow-y-auto flex-1 bg-[#F5F6FA]">
          <router-view />
        </el-main>
      </el-container>
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
