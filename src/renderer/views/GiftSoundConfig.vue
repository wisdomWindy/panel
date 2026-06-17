<script setup lang="ts">
import { ref, reactive, computed, onMounted, watch } from 'vue';
import { ElMessage, ElMessageBox } from 'element-plus';
import { Plus, Delete, Edit, Search, Upload, Download, VideoPlay, VideoPause, Refresh } from '@element-plus/icons-vue';
import { giftSoundService } from '../services/gift-sound-service';
import type { GiftSoundConfig, GiftSoundExportData } from '../modules/gift-sound/types';

// 响应式数据
const giftSounds = ref<GiftSoundConfig[]>([]);
const loading = ref(false);
const searchQuery = ref('');
const selectedGiftSound = ref<GiftSoundConfig | null>(null);
const dialogVisible = ref(false);
const isEditing = ref(false);
const importDialogVisible = ref(false);
const importData = ref('');
const previewSoundFile = ref('');
const previewVolume = ref(0.8);
const isPreviewPlaying = ref(false);

// 表单数据
const formData = reactive<GiftSoundConfig>({
  id: '',
  giftId: '',
  giftName: '',
  soundFile: '',
  soundName: '',
  volume: 0.8,
  duration: 0,
  playMode: 'once',
  cooldown: 1000,
  priority: 0,
  enabled: true,
  createdAt: new Date(),
  updatedAt: new Date()
});

// 计算属性
const filteredGiftSounds = computed(() => {
  if (!searchQuery.value) {
    return giftSounds.value;
  }
  const query = searchQuery.value.toLowerCase();
  return giftSounds.value.filter(
    gs =>
      gs.giftName.toLowerCase().includes(query) ||
      gs.giftId.toLowerCase().includes(query) ||
      gs.soundName.toLowerCase().includes(query)
  );
});

const totalGiftSounds = computed(() => giftSounds.value.length);
const enabledGiftSounds = computed(() => giftSounds.value.filter(gs => gs.enabled).length);

// 生命周期
onMounted(() => {
  loadGiftSounds();
});

// 方法
async function loadGiftSounds() {
  loading.value = true;
  try {
    giftSounds.value = await giftSoundService.getAllGiftSounds();
  } catch (error) {
    ElMessage.error('加载礼物音效配置失败');
    console.error('加载礼物音效配置失败:', error);
  } finally {
    loading.value = false;
  }
}

function handleAdd() {
  isEditing.value = false;
  resetFormData();
  dialogVisible.value = true;
}

function handleEdit(giftSound: GiftSoundConfig) {
  isEditing.value = true;
  Object.assign(formData, giftSound);
  dialogVisible.value = true;
}

async function handleDelete(giftSound: GiftSoundConfig) {
  try {
    await ElMessageBox.confirm(
      `确定要删除礼物 "${giftSound.giftName}" 的音效配置吗？`,
      '确认删除',
      {
        confirmButtonText: '确定',
        cancelButtonText: '取消',
        type: 'warning'
      }
    );

    await giftSoundService.deleteGiftSound(giftSound.id);
    ElMessage.success('删除成功');
    await loadGiftSounds();
  } catch (error) {
    if (error !== 'cancel') {
      ElMessage.error('删除失败');
      console.error('删除失败:', error);
    }
  }
}

async function handleSave() {
  try {
    // 验证表单
    if (!formData.giftId || !formData.giftName) {
      ElMessage.warning('请填写礼物ID和名称');
      return;
    }

    if (!formData.soundFile) {
      ElMessage.warning('请选择音效文件');
      return;
    }

    // 检查是否已存在相同礼物ID的配置（仅在添加时检查）
    if (!isEditing.value) {
      const existingConfig = giftSounds.value.find(gs => gs.giftId === formData.giftId);
      if (existingConfig) {
        ElMessage.warning(`礼物 "${formData.giftName}" 已有音效配置，每个礼物最多只能有一个音效文件`);
        return;
      }
    }

    if (isEditing.value) {
      await giftSoundService.updateGiftSound(formData.id, formData);
      ElMessage.success('更新成功');
    } else {
      await giftSoundService.addGiftSound(formData);
      ElMessage.success('添加成功');
    }

    dialogVisible.value = false;
    await loadGiftSounds();
  } catch (error) {
    ElMessage.error('保存失败');
    console.error('保存失败:', error);
  }
}

async function handleToggleEnabled(giftSound: GiftSoundConfig) {
  try {
    await giftSoundService.updateGiftSound(giftSound.id, {
      enabled: !giftSound.enabled
    });
    ElMessage.success(giftSound.enabled ? '已禁用' : '已启用');
    await loadGiftSounds();
  } catch (error) {
    ElMessage.error('操作失败');
    console.error('操作失败:', error);
  }
}

async function handlePreview(giftSound: GiftSoundConfig) {
  try {
    previewSoundFile.value = giftSound.soundFile;
    previewVolume.value = giftSound.volume;
    isPreviewPlaying.value = true;
    await giftSoundService.previewSound(giftSound.soundFile, giftSound.volume);

    // 模拟播放结束（实际应该通过事件监听）
    setTimeout(() => {
      isPreviewPlaying.value = false;
    }, 3000);
  } catch (error) {
    ElMessage.error('预览失败');
    console.error('预览失败:', error);
    isPreviewPlaying.value = false;
  }
}

function handleStopPreview() {
  giftSoundService.stopAllSounds();
  isPreviewPlaying.value = false;
}

function handleFileSelect() {
  // 检查当前礼物是否已经有音效配置
  const existingConfig = giftSounds.value.find(gs => gs.giftId === formData.giftId);
  if (existingConfig) {
    ElMessage.warning(`礼物 "${formData.giftName}" 已有音效配置，请先删除现有配置后再上传新文件`);
    return;
  }

  // 这里应该打开文件选择对话框
  // 由于Electron环境限制，这里使用模拟数据
  ElMessage.info('请选择MP3格式的音效文件');
}

async function handleExport() {
  try {
    const exportData = await giftSoundService.exportGiftSounds();
    const jsonStr = JSON.stringify(exportData, null, 2);

    // 创建下载链接
    const blob = new Blob([jsonStr], { type: 'application/json' });
    const url = URL.createObjectURL(blob);
    const a = document.createElement('a');
    a.href = url;
    a.download = `gift-sounds-${new Date().toISOString().slice(0, 10)}.json`;
    document.body.appendChild(a);
    a.click();
    document.body.removeChild(a);
    URL.revokeObjectURL(url);

    ElMessage.success('导出成功');
  } catch (error) {
    ElMessage.error('导出失败');
    console.error('导出失败:', error);
  }
}

function handleImport() {
  importDialogVisible.value = true;
  importData.value = '';
}

async function handleImportConfirm() {
  try {
    if (!importData.value.trim()) {
      ElMessage.warning('请输入导入数据');
      return;
    }

    const data = JSON.parse(importData.value) as GiftSoundExportData;
    if (!data.giftSounds || !Array.isArray(data.giftSounds)) {
      ElMessage.warning('导入数据格式错误');
      return;
    }

    await giftSoundService.importGiftSounds(data.giftSounds);
    ElMessage.success('导入成功');
    importDialogVisible.value = false;
    await loadGiftSounds();
  } catch (error) {
    ElMessage.error('导入失败');
    console.error('导入失败:', error);
  }
}

function resetFormData() {
  Object.assign(formData, {
    id: Date.now().toString(),
    giftId: '',
    giftName: '',
    soundFile: '',
    soundName: '',
    volume: 0.8,
    duration: 0,
    playMode: 'once',
    cooldown: 1000,
    priority: 0,
    enabled: true,
    createdAt: new Date(),
    updatedAt: new Date()
  });
}

function handleCloseDialog() {
  dialogVisible.value = false;
}
</script>

<template>
  <div class="p-5">
    <!-- 页面标题 -->
    <div class="mb-5">
      <h2 class="m-0 mb-2 text-2xl text-[#303133]">
        礼物音效配置
      </h2>
      <p class="m-0 text-[#909399] text-sm">
        为直播间礼物设置自定义音效，增强互动体验
      </p>
    </div>

    <!-- 统计信息 -->
    <div class="bg-[#f5f7fa] p-5 rounded-lg mb-5">
      <el-row :gutter="20">
        <el-col :span="8">
          <el-statistic title="总配置数" :value="totalGiftSounds" />
        </el-col>
        <el-col :span="8">
          <el-statistic title="已启用" :value="enabledGiftSounds" />
        </el-col>
        <el-col :span="8">
          <el-statistic title="已禁用" :value="totalGiftSounds - enabledGiftSounds" />
        </el-col>
      </el-row>
    </div>

    <!-- 操作栏 -->
    <div class="flex justify-between items-center mb-5">
      <div class="flex gap-2.5">
        <el-button type="primary" :icon="Plus" @click="handleAdd">
          添加配置
        </el-button>
        <el-button :icon="Upload" @click="handleImport">
          导入
        </el-button>
        <el-button :icon="Download" @click="handleExport">
          导出
        </el-button>
      </div>
      <div class="flex gap-2.5">
        <el-input
          v-model="searchQuery"
          placeholder="搜索礼物名称或ID"
          :prefix-icon="Search"
          clearable
          class="w-[300px]"
        />
      </div>
    </div>

    <!-- 礼物音效列表 -->
    <el-table
      v-loading="loading"
      :data="filteredGiftSounds"
      border
      stripe
      class="w-full"
    >
      <el-table-column prop="giftName" label="礼物名称" width="150" />
      <el-table-column prop="giftId" label="礼物ID" width="120" />
      <el-table-column prop="soundName" label="音效名称" width="150" />
      <el-table-column label="音量" width="100">
        <template #default="{ row }">
          <el-progress
            :percentage="row.volume * 100"
            :stroke-width="10"
            :text-inside="false"
          />
        </template>
      </el-table-column>
      <el-table-column label="播放模式" width="100">
        <template #default="{ row }">
          <el-tag :type="row.playMode === 'loop' ? 'success' : 'info'">
            {{ row.playMode === 'loop' ? '循环' : '单次' }}
          </el-tag>
        </template>
      </el-table-column>
      <el-table-column label="冷却时间" width="100">
        <template #default="{ row }">
          {{ row.cooldown }}ms
        </template>
      </el-table-column>
      <el-table-column label="优先级" width="80">
        <template #default="{ row }">
          {{ row.priority }}
        </template>
      </el-table-column>
      <el-table-column label="状态" width="80">
        <template #default="{ row }">
          <el-switch
            v-model="row.enabled"
            @change="handleToggleEnabled(row)"
          />
        </template>
      </el-table-column>
      <el-table-column label="操作" width="200" fixed="right">
        <template #default="{ row }">
          <el-button
            type="primary"
            :icon="VideoPlay"
            size="small"
            :disabled="!row.enabled"
            @click="handlePreview(row)"
          >
            预览
          </el-button>
          <el-button
            type="warning"
            :icon="Edit"
            size="small"
            @click="handleEdit(row)"
          >
            编辑
          </el-button>
          <el-button
            type="danger"
            :icon="Delete"
            size="small"
            @click="handleDelete(row)"
          >
            删除
          </el-button>
        </template>
      </el-table-column>
    </el-table>

    <!-- 空状态 -->
    <el-empty
      v-if="!loading && filteredGiftSounds.length === 0"
      description="暂无礼物音效配置"
    >
      <el-button type="primary" @click="handleAdd">
        添加配置
      </el-button>
    </el-empty>

    <!-- 添加/编辑对话框 -->
    <el-dialog
      v-model="dialogVisible"
      :title="isEditing ? '编辑礼物音效' : '添加礼物音效'"
      width="600px"
      @close="handleCloseDialog"
    >
      <el-form :model="formData" label-width="100px">
        <el-form-item label="礼物ID" required>
          <el-input v-model="formData.giftId" placeholder="请输入礼物ID" />
        </el-form-item>
        <el-form-item label="礼物名称" required>
          <el-input v-model="formData.giftName" placeholder="请输入礼物名称" />
        </el-form-item>
        <el-form-item label="音效文件" required>
          <el-input v-model="formData.soundFile" placeholder="请选择音效文件">
            <template #append>
              <el-button @click="handleFileSelect">
                选择
              </el-button>
            </template>
          </el-input>
        </el-form-item>
        <el-form-item label="音效名称">
          <el-input v-model="formData.soundName" placeholder="请输入音效名称" />
        </el-form-item>
        <el-form-item label="音量">
          <el-slider
            v-model="formData.volume"
            :min="0"
            :max="1"
            :step="0.1"
            show-input
          />
        </el-form-item>
        <el-form-item label="播放时长">
          <el-input-number
            v-model="formData.duration"
            :min="0"
            :step="1000"
            placeholder="0表示完整播放"
          />
          <span class="ml-2.5 text-[#909399] text-xs">毫秒，0表示完整播放</span>
        </el-form-item>
        <el-form-item label="播放模式">
          <el-radio-group v-model="formData.playMode">
            <el-radio value="once">
              单次播放
            </el-radio>
            <el-radio value="loop">
              循环播放
            </el-radio>
          </el-radio-group>
        </el-form-item>
        <el-form-item label="冷却时间">
          <el-input-number
            v-model="formData.cooldown"
            :min="0"
            :step="100"
          />
          <span class="ml-2.5 text-[#909399] text-xs">毫秒</span>
        </el-form-item>
        <el-form-item label="优先级">
          <el-input-number
            v-model="formData.priority"
            :min="0"
            :max="100"
          />
          <span class="ml-2.5 text-[#909399] text-xs">数字越大优先级越高</span>
        </el-form-item>
        <el-form-item label="启用状态">
          <el-switch v-model="formData.enabled" />
        </el-form-item>
      </el-form>
      <template #footer>
        <span class="flex justify-end gap-2.5">
          <el-button @click="handleCloseDialog">取消</el-button>
          <el-button type="primary" @click="handleSave">保存</el-button>
        </span>
      </template>
    </el-dialog>

    <!-- 导入对话框 -->
    <el-dialog
      v-model="importDialogVisible"
      title="导入音效配置"
      width="500px"
    >
      <el-form>
        <el-form-item label="导入数据">
          <el-input
            v-model="importData"
            type="textarea"
            :rows="10"
            placeholder="请粘贴导出的JSON数据"
          />
        </el-form-item>
      </el-form>
      <template #footer>
        <span class="flex justify-end gap-2.5">
          <el-button @click="importDialogVisible = false">取消</el-button>
          <el-button type="primary" @click="handleImportConfirm">导入</el-button>
        </span>
      </template>
    </el-dialog>

    <!-- 预览面板 -->
    <div v-if="isPreviewPlaying" class="fixed bottom-5 right-5 bg-white p-4 rounded-lg shadow-lg z-[1000]">
      <div class="flex items-center gap-2.5">
        <span>正在预览: {{ previewSoundFile }}</span>
        <el-button
          type="danger"
          :icon="VideoPause"
          size="small"
          @click="handleStopPreview"
        >
          停止
        </el-button>
      </div>
    </div>
  </div>
</template>
