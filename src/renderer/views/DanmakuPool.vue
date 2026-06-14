<script setup lang="ts">
import { computed, ref } from 'vue';
import { useDanmakuStore } from '../stores/danmaku';
import { getDanmakuService } from '../services/danmaku-service';

const danmakuService = getDanmakuService();

const danmakuStore = useDanmakuStore();

const isAutoSending = computed({
  get: () => danmakuService.isAutoSending,
  set: (enabled: boolean) => {
    toggleAutoSend(enabled);
  }
});

const autoSendInterval = computed({
  get: () => danmakuStore.autoSendInterval,
  set: (interval: number) => {
    updateAutoSendInterval(interval);
  }
});

const addDialogVisible = ref(false);
const addForm = ref({
  content: '',
  type: 'random' as 'timed' | 'random' | 'trigger',
  triggerWord: '',
  intervalSeconds: 5
});

const randomCount = computed(() => {
  return danmakuStore.danmakuPool.filter(item => item.type === 'random' && item.enabled).length;
});

function handleAdd() {
  addForm.value = {
    content: '',
    type: 'random',
    triggerWord: '',
    intervalSeconds: 5
  };
  addDialogVisible.value = true;
}

function confirmAdd() {
  if (!addForm.value.content.trim()) {
    ElMessage.warning('请输入弹幕内容');
    return;
  }
  if (addForm.value.type === 'trigger' && !addForm.value.triggerWord.trim()) {
    ElMessage.warning('请输入触发词');
    return;
  }

  danmakuService.addToPool({
    content: addForm.value.content.trim(),
    type: addForm.value.type,
    triggerWord: addForm.value.type === 'trigger' ? addForm.value.triggerWord.trim() : undefined,
    enabled: true,
    priority: 0,
    interval: (addForm.value.intervalSeconds || 5) * 1000 // 秒转毫秒
  });
  ElMessage.success('添加成功');
  addDialogVisible.value = false;
}

function handleUpdate(id: number, updates: Record<string, unknown>) {
  danmakuService.updatePoolItem(id, updates);
  ElMessage.success('更新成功');
}

function getIntervalSeconds(interval?: number): number {
  return Math.max(1, Math.round((interval ?? 5000) / 1000));
}

function handleIntervalChange(id: number, intervalSeconds: number | undefined) {
  if (intervalSeconds === undefined) {
    return;
  }

  handleUpdate(id, { interval: intervalSeconds * 1000 });
}

function handleDelete(id: number) {
  ElMessageBox.confirm('确定要删除这条弹幕吗？', '提示', {
    confirmButtonText: '确定',
    cancelButtonText: '取消',
    type: 'warning'
  })
    .then(() => {
      danmakuService.removeFromPool(id);
      ElMessage.success('删除成功');
    })
    .catch(() => {});
}

function handleTest(content: string) {
  if (!danmakuService.isConnected) {
    ElMessage.warning('请先连接直播间');
    return;
  }

  danmakuService
    .sendDanmaku(content)
    .then(() => {
      ElMessage.success('测试发送成功');
    })
    .catch(error => {
      ElMessage.error(`测试发送失败: ${error}`);
    });
}

function handleRefresh() {
  // 刷新数据
  ElMessage.success('已刷新');
}

function toggleAutoSend(enabled: boolean) {
  if (enabled) {
    danmakuService.startAutoSend(autoSendInterval.value);
  } else {
    danmakuService.stopAutoSend();
  }
}

function updateAutoSendInterval(interval: number) {
  danmakuStore.setAutoSendInterval(interval);
  if (isAutoSending.value) {
    danmakuService.startAutoSend(interval);
  }
}
</script>

<template>
  <div class="p-8">
    <el-row :gutter="20">
      <el-col :span="16">
        <el-card>
          <template #header>
            <div class="flex justify-between items-center">
              <span class="font-semibold text-gray-700">弹幕池</span>
              <div class="flex gap-1">
                <el-button type="primary" @click="handleAdd">
                  添加弹幕
                </el-button>
                <el-button @click="handleRefresh">
                  刷新
                </el-button>
              </div>
            </div>
          </template>

          <el-table :data="danmakuStore.danmakuPool" class="w-full">
            <el-table-column prop="content" label="内容" min-width="180">
              <template #default="{ row }">
                <el-input
                  v-model="row.content"
                  size="small"
                  @change="handleUpdate(row.id, { content: row.content })"
                />
              </template>
            </el-table-column>
            <el-table-column prop="type" label="类型" width="100">
              <template #default="{ row }">
                <el-select
                  v-model="row.type"
                  size="small"
                  @change="handleUpdate(row.id, { type: row.type })"
                >
                  <el-option label="定时" value="timed" />
                  <el-option label="随机" value="random" />
                  <el-option label="触发" value="trigger" />
                </el-select>
              </template>
            </el-table-column>
            <el-table-column prop="triggerWord" label="触发词" width="120">
              <template #default="{ row }">
                <el-input
                  v-if="row.type === 'trigger'"
                  v-model="row.triggerWord"
                  size="small"
                  placeholder="触发词"
                  @change="handleUpdate(row.id, { triggerWord: row.triggerWord })"
                />
                <span v-else>-</span>
              </template>
            </el-table-column>
            <el-table-column prop="interval" label="间隔(秒)" width="120">
              <template #default="{ row }">
                <el-input-number
                  v-if="row.type !== 'trigger'"
                  :model-value="getIntervalSeconds(row.interval)"
                  :min="1"
                  :max="3600"
                  size="small"
                  @update:model-value="handleIntervalChange(row.id, $event)"
                />
                <span v-else>-</span>
              </template>
            </el-table-column>
            <el-table-column prop="enabled" label="启用" width="80">
              <template #default="{ row }">
                <el-switch
                  v-model="row.enabled"
                  size="small"
                  @change="handleUpdate(row.id, { enabled: $event })"
                />
              </template>
            </el-table-column>
            <el-table-column label="操作" width="120">
              <template #default="{ row }">
                <el-button size="small" type="primary" @click="handleTest(row.content)">
                  测试
                </el-button>
                <el-button size="small" type="danger" @click="handleDelete(row.id)">
                  删除
                </el-button>
              </template>
            </el-table-column>
          </el-table>
        </el-card>
      </el-col>

      <el-col :span="8">
        <el-card>
          <template #header>
            <div class="flex justify-between items-center">
              <span>自动发送控制</span>
            </div>
          </template>

          <div class="p-4">
            <el-form label-width="100px">
              <el-form-item label="状态">
                <el-switch v-model="isAutoSending" />
              </el-form-item>
              <el-form-item label="间隔">
                <el-input-number
                  v-model="autoSendInterval"
                  :min="1000"
                  :max="300000"
                  :step="1000"
                />
                <span class="ml-2">毫秒</span>
              </el-form-item>
              <el-form-item label="随机弹幕">
                <div class="flex justify-between">
                  <span>共 {{ randomCount }} 条</span>
                </div>
              </el-form-item>
            </el-form>
          </div>
        </el-card>
      </el-col>
    </el-row>

    <!-- 添加弹幕对话框 -->
    <el-dialog v-model="addDialogVisible" title="添加弹幕" width="480px">
      <el-form :model="addForm" label-width="80px">
        <el-form-item label="类型">
          <el-select v-model="addForm.type" class="w-full">
            <el-option label="定时发送" value="timed" />
            <el-option label="随机发送" value="random" />
            <el-option label="触发回复" value="trigger" />
          </el-select>
        </el-form-item>
        <el-form-item label="内容">
          <el-input
            v-model="addForm.content"
            type="textarea"
            :rows="2"
            placeholder="请输入弹幕内容"
          />
        </el-form-item>
        <el-form-item v-if="addForm.type === 'trigger'" label="触发词">
          <el-input v-model="addForm.triggerWord" placeholder="请输入触发词" />
        </el-form-item>
        <el-form-item v-if="addForm.type !== 'trigger'" label="间隔(秒)">
          <el-input-number
            v-model="addForm.intervalSeconds"
            :min="1"
            :max="3600"
            class="w-full"
          />
        </el-form-item>
      </el-form>
      <template #footer>
        <el-button @click="addDialogVisible = false">
          取消
        </el-button>
        <el-button type="primary" @click="confirmAdd">
          确定
        </el-button>
      </template>
    </el-dialog>
  </div>
</template>
