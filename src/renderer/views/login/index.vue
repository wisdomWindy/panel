<script setup lang="ts">
import { reactive } from 'vue';
import { useRouter } from 'vue-router';
import { useRoomStore } from '../../stores/room';
import { useConfigStore } from '../../stores/config';
import { getDanmakuService } from '../../services/danmaku-service';

const danmakuService = getDanmakuService();

const router = useRouter();
const roomStore = useRoomStore();
const configStore = useConfigStore();

const form = reactive({
  code: roomStore.code || ''
});

async function handleConnect() {
  if (!form.code) {
    ElMessage.warning('请输入身份码');
    return;
  }
  if (!configStore.config.appId) {
    ElMessage.warning('应用 ID 未配置，请在设置页面配置');
    return;
  }

  roomStore.setCode(form.code);
  roomStore.setAppId(configStore.config.appId);

  try {
    await danmakuService.connect();
    ElMessage.success('连接成功');
    router.push('/dashboard');
  } catch (error) {
    roomStore.setStatus('error');
    ElMessage.error(`连接失败: ${error}`);
  }
}
</script>

<template>
  <div class="flex items-center justify-center h-screen w-screen bg-[#F0F2F5]">
    <div class="flex flex-col items-center">
      <div class="mb-6 text-center">
        <h1 class="text-2xl font-bold mb-2 bg-gradient-to-r from-[#1D3557] to-[#457B9D] bg-clip-text text-transparent">
          Bilibili 直播助手
        </h1>
        <p class="text-sm text-gray-500 m-0">
          请输入主播身份码以开始连接
        </p>
      </div>

      <div class="w-80 max-w-[90vw] bg-white rounded-xl shadow-lg p-8">
        <el-form :model="form" label-position="top">
          <el-form-item label="身份码" size="large">
            <el-input
              v-model="form.code"
              placeholder="请输入主播身份码"
              size="large"
            />
          </el-form-item>

          <el-form-item>
            <el-button
              type="primary"
              size="large"
              class="w-full"
              :loading="roomStore.isConnecting"
              @click="handleConnect"
            >
              {{ roomStore.isConnecting ? '连接中...' : '开始连接' }}
            </el-button>
          </el-form-item>
        </el-form>

        <div class="text-center mt-5 text-sm text-gray-500">
          在
          <a
            href="https://play-live.bilibili.com/"
            target="_blank"
            class="text-blue-500 underline"
          >互动应用中心</a>
          右下角获取身份码
        </div>
      </div>
    </div>
  </div>
</template>
