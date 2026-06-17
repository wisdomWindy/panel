<script setup lang="ts">
import { onMounted } from 'vue';
import { useUpdateStore } from '../stores/update';
import { Bell, Download, Document } from '@element-plus/icons-vue';
import { ChangelogEntry } from '../services/update-service';

const updateStore = useUpdateStore();

const changeTypeLabels: Record<string, string> = {
  added: '新功能',
  changed: '改进',
  fixed: '修复',
  removed: '移除',
  security: '安全'
};

const changeTypeColors: Record<string, string> = {
  added: '#67C23A',
  changed: '#409EFF',
  fixed: '#E6A23C',
  removed: '#909399',
  security: '#F56C6C'
};

onMounted(async () => {
  await updateStore.parseChangelog();
  await updateStore.checkForUpdates();
});

function handleDownload() {
  if (updateStore.releaseUrl) {
    window.open(updateStore.releaseUrl, '_blank');
  }
}
</script>

<template>
  <div class="p-8">
    <el-card>
      <template #header>
        <div class="flex items-center">
          <el-icon class="mr-2">
            <Document />
          </el-icon>
          <span>更新日志</span>
        </div>
      </template>

      <!-- 头部信息 -->
      <div class="mb-6 p-4 bg-gray-50 rounded-lg">
        <div class="flex items-center justify-between">
          <div>
            <div class="text-sm text-gray-500 mb-1">
              当前版本
            </div>
            <div class="text-lg font-semibold text-blue-600">
              v{{ updateStore.currentVersion }}
            </div>
          </div>

          <div v-if="updateStore.hasUpdate" class="text-right">
            <div class="text-sm text-gray-500 mb-1">
              最新版本
            </div>
            <div class="text-lg font-semibold text-green-600">
              v{{ updateStore.latestVersion }}
            </div>
          </div>

          <el-button
            v-if="updateStore.hasUpdate"
            type="primary"
            :icon="Download"
            @click="handleDownload"
          >
            前往下载
          </el-button>
        </div>
      </div>

      <!-- 加载状态 -->
      <div
        v-if="updateStore.isChecking"
        class="text-center py-8 text-gray-500"
      >
        <el-icon class="is-loading mr-2">
          <Bell />
        </el-icon>
        正在检查更新...
      </div>

      <!-- 更新日志列表 -->
      <div v-else-if="updateStore.hasChangelog" class="space-y-6">
        <el-card
          v-for="entry in updateStore.changelog"
          :key="entry.version"
          shadow="hover"
          class="changelog-card"
        >
          <div class="flex items-center justify-between mb-4">
            <div class="flex items-center">
              <el-tag effect="dark" type="primary" class="mr-3">
                v{{ entry.version }}
              </el-tag>
              <span class="text-sm text-gray-500">
                {{ entry.date }}
              </span>
            </div>
          </div>

          <div class="space-y-4">
            <div
              v-for="change in entry.changes"
              :key="change.type"
            >
              <div class="flex items-center mb-2">
                <el-tag
                  :color="changeTypeColors[change.type]"
                  effect="dark"
                  size="small"
                >
                  {{ changeTypeLabels[change.type] || change.type }}
                </el-tag>
              </div>

              <ul class="pl-5 space-y-1">
                <li
                  v-for="(item, index) in change.items"
                  :key="index"
                  class="text-sm text-gray-700"
                >
                  {{ item }}
                </li>
              </ul>
            </div>
          </div>
        </el-card>
      </div>

      <!-- 空状态 -->
      <div v-else class="text-center py-12 text-gray-500">
        <el-icon class="text-4xl mb-4">
          <Document />
        </el-icon>
        <div>暂无更新日志</div>
      </div>
    </el-card>
  </div>
</template>
