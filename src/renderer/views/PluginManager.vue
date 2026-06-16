<script setup lang="ts">
import { ref, computed, onMounted } from 'vue';
import { usePluginStore } from '../stores/plugin';
import { PluginState } from '../plugins/types';

const pluginStore = usePluginStore();

// 状态
const showInstallDialog = ref(false);
const showDetailDialog = ref(false);
const showConfigDialog = ref(false);
const selectedPlugin = ref<PluginState | null>(null);
const installFile = ref<File | null>(null);
const installing = ref(false);

// 计算属性
const plugins = computed(() => pluginStore.installedPlugins);
const loading = computed(() => pluginStore.loading);
const error = computed(() => pluginStore.error);

// 生命周期
onMounted(async () => {
  await loadPlugins();
});

// 加载插件列表
async function loadPlugins() {
  pluginStore.setLoading(true);
  pluginStore.setError(null);

  try {
    // TODO: 从插件管理器加载插件列表
    // const pluginManager = getPluginManager();
    // const plugins = pluginManager.getPlugins();
    // pluginStore.setPlugins(plugins);
  } catch (err) {
    pluginStore.setError(err instanceof Error ? err.message : '加载插件失败');
  } finally {
    pluginStore.setLoading(false);
  }
}

// 启用/禁用插件
async function togglePlugin(plugin: PluginState) {
  try {
    // TODO: 调用插件管理器启用/禁用插件
    // const pluginManager = getPluginManager();
    // if (plugin.enabled) {
    //   await pluginManager.disablePlugin(plugin.id);
    //   pluginStore.disablePlugin(plugin.id);
    // } else {
    //   await pluginManager.enablePlugin(plugin.id);
    //   pluginStore.enablePlugin(plugin.id);
    // }
  } catch (err) {
    ElMessage.error(err instanceof Error ? err.message : '操作失败');
  }
}

// 查看插件详情
function viewDetail(plugin: PluginState) {
  selectedPlugin.value = plugin;
  showDetailDialog.value = true;
}

// 打开配置对话框
function openConfig(plugin: PluginState) {
  selectedPlugin.value = plugin;
  showConfigDialog.value = true;
}

// 卸载插件
async function uninstallPlugin(plugin: PluginState) {
  try {
    await ElMessageBox.confirm(
      `确定要卸载插件 "${plugin.name}" 吗？此操作不可恢复。`,
      '卸载确认',
      { type: 'warning' }
    );

    // TODO: 调用插件管理器卸载插件
    // const pluginManager = getPluginManager();
    // await pluginManager.uninstallPlugin(plugin.id);
    // pluginStore.removePlugin(plugin.id);

    ElMessage.success('插件已卸载');
  } catch (err) {
    if (err !== 'cancel') {
      ElMessage.error(err instanceof Error ? err.message : '卸载失败');
    }
  }
}

// 安装插件
async function handleInstall() {
  if (!installFile.value) {
    ElMessage.warning('请选择插件文件');
    return;
  }

  installing.value = true;

  try {
    // TODO: 调用插件管理器安装插件
    // const pluginManager = getPluginManager();
    // await pluginManager.installPlugin(installFile.value.path);

    ElMessage.success('插件安装成功');
    showInstallDialog.value = false;
    installFile.value = null;
    await loadPlugins();
  } catch (err) {
    ElMessage.error(err instanceof Error ? err.message : '安装失败');
  } finally {
    installing.value = false;
  }
}

// 文件选择处理
function handleFileChange(file: File) {
  installFile.value = file;
}

// 格式化日期
function formatDate(dateString: string): string {
  return new Date(dateString).toLocaleString('zh-CN');
}

// 获取权限标签类型
function getPermissionTagType(permission: string): string {
  const types: Record<string, string> = {
    danmaku: 'primary',
    gift: 'success',
    captain: 'warning',
    config: 'info',
    effect: 'danger',
    ui: '',
  };
  return types[permission] || '';
}

// 获取权限标签文本
function getPermissionLabel(permission: string): string {
  const labels: Record<string, string> = {
    danmaku: '弹幕',
    gift: '礼物',
    captain: '舰长',
    config: '配置',
    effect: '特效',
    ui: 'UI',
  };
  return labels[permission] || permission;
}
</script>

<template>
  <div class="plugin-manager">
    <!-- 头部操作栏 -->
    <div class="plugin-header">
      <h2>插件管理</h2>
      <div class="plugin-actions">
        <el-button type="primary" @click="showInstallDialog = true">
          <el-icon><Upload /></el-icon>
          安装插件
        </el-button>
        <el-button @click="loadPlugins">
          <el-icon><Refresh /></el-icon>
          刷新
        </el-button>
      </div>
    </div>

    <!-- 统计信息 -->
    <div class="plugin-stats">
      <el-statistic title="已安装" :value="pluginStore.pluginCount" />
      <el-statistic title="已启用" :value="pluginStore.enabledCount" />
      <el-statistic title="已禁用" :value="pluginStore.disabledPlugins.length" />
    </div>

    <!-- 错误提示 -->
    <el-alert
      v-if="error"
      :title="error"
      type="error"
      show-icon
      closable
      @close="pluginStore.setError(null)"
    />

    <!-- 插件列表 -->
    <el-table
      :data="plugins"
      :loading="loading"
      empty-text="暂无已安装的插件"
      stripe
      style="width: 100%"
    >
      <el-table-column prop="name" label="插件名称" min-width="150">
        <template #default="{ row }">
          <div class="plugin-name">
            <span class="name">{{ row.name }}</span>
            <span class="version">v{{ row.version }}</span>
          </div>
        </template>
      </el-table-column>

      <el-table-column prop="description" label="描述" min-width="200" show-overflow-tooltip />

      <el-table-column prop="author" label="作者" width="120" />

      <el-table-column label="权限" width="200">
        <template #default="{ row }">
          <el-tag
            v-for="permission in row.permissions"
            :key="permission"
            :type="getPermissionTagType(permission)"
            size="small"
            class="permission-tag"
          >
            {{ getPermissionLabel(permission) }}
          </el-tag>
        </template>
      </el-table-column>

      <el-table-column label="状态" width="100">
        <template #default="{ row }">
          <el-switch
            v-model="row.enabled"
            @change="togglePlugin(row)"
            :active-text="row.enabled ? '已启用' : '已禁用'"
          />
        </template>
      </el-table-column>

      <el-table-column label="安装时间" width="180">
        <template #default="{ row }">
          {{ formatDate(row.installedAt) }}
        </template>
      </el-table-column>

      <el-table-column label="操作" width="200" fixed="right">
        <template #default="{ row }">
          <el-button size="small" @click="viewDetail(row)">
            详情
          </el-button>
          <el-button size="small" @click="openConfig(row)">
            配置
          </el-button>
          <el-button size="small" type="danger" @click="uninstallPlugin(row)">
            卸载
          </el-button>
        </template>
      </el-table-column>
    </el-table>

    <!-- 安装对话框 -->
    <el-dialog v-model="showInstallDialog" title="安装插件" width="500px">
      <div class="install-content">
        <el-upload
          drag
          :auto-upload="false"
          :on-change="handleFileChange"
          accept=".zip,.js"
        >
          <el-icon class="el-icon--upload"><Upload /></el-icon>
          <div class="el-upload__text">
            将插件文件拖到此处，或<em>点击选择</em>
          </div>
          <template #tip>
            <div class="el-upload__tip">
              支持 .zip 和 .js 格式的插件文件
            </div>
          </template>
        </el-upload>
      </div>

      <template #footer>
        <span class="dialog-footer">
          <el-button @click="showInstallDialog = false">取消</el-button>
          <el-button type="primary" @click="handleInstall" :loading="installing">
            安装
          </el-button>
        </span>
      </template>
    </el-dialog>

    <!-- 详情对话框 -->
    <el-dialog v-model="showDetailDialog" title="插件详情" width="600px">
      <div v-if="selectedPlugin" class="detail-content">
        <el-descriptions :column="2" border>
          <el-descriptions-item label="插件名称">
            {{ selectedPlugin.name }}
          </el-descriptions-item>
          <el-descriptions-item label="版本">
            {{ selectedPlugin.version }}
          </el-descriptions-item>
          <el-descriptions-item label="作者">
            {{ selectedPlugin.author }}
          </el-descriptions-item>
          <el-descriptions-item label="状态">
            <el-tag :type="selectedPlugin.enabled ? 'success' : 'info'">
              {{ selectedPlugin.enabled ? '已启用' : '已禁用' }}
            </el-tag>
          </el-descriptions-item>
          <el-descriptions-item label="描述" :span="2">
            {{ selectedPlugin.description }}
          </el-descriptions-item>
          <el-descriptions-item label="权限" :span="2">
            <el-tag
              v-for="permission in selectedPlugin.permissions"
              :key="permission"
              :type="getPermissionTagType(permission)"
              class="permission-tag"
            >
              {{ getPermissionLabel(permission) }}
            </el-tag>
          </el-descriptions-item>
          <el-descriptions-item label="安装时间">
            {{ formatDate(selectedPlugin.installedAt) }}
          </el-descriptions-item>
          <el-descriptions-item label="更新时间">
            {{ formatDate(selectedPlugin.updatedAt) }}
          </el-descriptions-item>
        </el-descriptions>
      </div>
    </el-dialog>

    <!-- 配置对话框 -->
    <el-dialog v-model="showConfigDialog" title="插件配置" width="600px">
      <div v-if="selectedPlugin" class="config-content">
        <p>插件配置功能开发中...</p>
        <p>当前配置：</p>
        <pre>{{ JSON.stringify(selectedPlugin.config, null, 2) }}</pre>
      </div>
    </el-dialog>
  </div>
</template>

<style scoped>
.plugin-manager {
  padding: 20px;
}

.plugin-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 20px;
}

.plugin-header h2 {
  margin: 0;
  font-size: 20px;
  font-weight: 600;
}

.plugin-actions {
  display: flex;
  gap: 10px;
}

.plugin-stats {
  display: flex;
  gap: 40px;
  margin-bottom: 20px;
  padding: 20px;
  background: #f5f7fa;
  border-radius: 8px;
}

.plugin-name {
  display: flex;
  align-items: center;
  gap: 8px;
}

.plugin-name .name {
  font-weight: 600;
}

.plugin-name .version {
  color: #909399;
  font-size: 12px;
}

.permission-tag {
  margin-right: 4px;
}

.install-content {
  padding: 20px 0;
}

.detail-content {
  padding: 10px 0;
}

.config-content {
  padding: 10px 0;
}

.config-content pre {
  background: #f5f7fa;
  padding: 10px;
  border-radius: 4px;
  overflow-x: auto;
}
</style>
