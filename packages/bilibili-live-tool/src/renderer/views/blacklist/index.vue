<script setup lang="ts">
import { ref, onMounted } from 'vue';
import { blacklistManager } from '../../modules/danmaku/blacklist';
import { BlacklistItem } from '../../modules/danmaku/types';

const dialogVisible = ref(false);
const form = ref({
  userId: '',
  username: '',
  reason: '',
  duration: 0
});

const list = ref<BlacklistItem[]>([]);

function formatTime(timestamp: number): string {
  return new Date(timestamp).toLocaleString('zh-CN', {
    hour12: false,
    year: 'numeric',
    month: '2-digit',
    day: '2-digit',
    hour: '2-digit',
    minute: '2-digit'
  });
}

function formatBannedUntil(bannedUntil: number): string {
  if (bannedUntil === 0) return '永久';
  return formatTime(bannedUntil);
}

function handleAdd() {
  if (!form.value.userId.trim()) {
    ElMessage.warning('请输入用户ID');
    return;
  }
  if (!form.value.username.trim()) {
    ElMessage.warning('请输入用户名');
    return;
  }
  blacklistManager.addToBlacklist(
    form.value.userId.trim(),
    form.value.username.trim(),
    form.value.reason.trim(),
    form.value.duration
  );
  ElMessage.success('已添加到黑名单');
  dialogVisible.value = false;
  form.value = { userId: '', username: '', reason: '', duration: 0 };
  refreshList();
}

function handleRemove(userId: string) {
  blacklistManager.removeFromBlacklist(userId);
  ElMessage.success('已移除');
  refreshList();
}

function refreshList() {
  list.value = blacklistManager.getBlacklist();
}

onMounted(() => {
  refreshList();
  blacklistManager.on('banned', refreshList);
  blacklistManager.on('unbanned', refreshList);
});
</script>

<template>
  <div class="p-8">
    <el-card>
      <template #header>
        <div class="flex justify-between items-center">
          <span class="font-semibold text-gray-700">黑名单管理</span>
          <el-button type="primary" @click="dialogVisible = true">
            添加
          </el-button>
        </div>
      </template>

      <el-table
        :data="list"
        class="w-full"
        stripe
        highlight-current-row
      >
        <el-table-column prop="userId" label="用户ID" width="150" />
        <el-table-column prop="username" label="用户名" width="120" />
        <el-table-column prop="reason" label="原因" min-width="150" />
        <el-table-column label="封禁时间" width="180">
          <template #default="{ row }">
            {{ formatTime(row.bannedAt) }}
          </template>
        </el-table-column>
        <el-table-column label="解封时间" width="180">
          <template #default="{ row }">
            <el-tag :type="row.bannedUntil === 0 ? 'danger' : 'warning'">
              {{ formatBannedUntil(row.bannedUntil) }}
            </el-tag>
          </template>
        </el-table-column>
        <el-table-column label="操作" width="100">
          <template #default="{ row }">
            <el-button size="small" type="danger" @click="handleRemove(row.userId)">
              移除
            </el-button>
          </template>
        </el-table-column>
      </el-table>

      <el-empty v-if="list.length === 0" description="暂无黑名单记录" />
    </el-card>

    <el-dialog v-model="dialogVisible" title="添加黑名单" width="500px">
      <el-form :model="form" label-width="80px">
        <el-form-item label="用户ID">
          <el-input v-model="form.userId" placeholder="B站用户ID" />
        </el-form-item>
        <el-form-item label="用户名">
          <el-input v-model="form.username" placeholder="用户名" />
        </el-form-item>
        <el-form-item label="原因">
          <el-input v-model="form.reason" placeholder="封禁原因" />
        </el-form-item>
        <el-form-item label="时长(分)">
          <el-input-number
            v-model="form.duration"
            :min="0"
            :max="10080"
            placeholder="0=永久"
          />
          <span class="ml-2 text-sm text-gray-500">0=永久</span>
        </el-form-item>
      </el-form>
      <template #footer>
        <el-button @click="dialogVisible = false">
          取消
        </el-button>
        <el-button type="primary" @click="handleAdd">
          确定
        </el-button>
      </template>
    </el-dialog>
  </div>
</template>
