<script setup lang="ts">
import { computed, ref } from 'vue';
import { Delete, Refresh, Setting, Upload } from '@element-plus/icons-vue';
import { ElMessageBox } from 'element-plus';
import { useI18n } from 'vue-i18n';
import type { PluginSnapshot } from '@/platform/desktop';
import { useReleaseStore } from '@/stores/release';
import PluginInstallDialog from './PluginInstallDialog.vue';

type PluginTab = 'installed' | 'updateAvailable' | 'disabled';
const { t } = useI18n();
const store = useReleaseStore();
const activeTab = ref<PluginTab>('installed');
const isInstallOpen = ref(false);
const installManifest = ref<PluginSnapshot | null>(null);
const visiblePlugins = computed(() => store.plugins.filter(plugin => {
  if (activeTab.value === 'updateAvailable') return plugin.status === 'updateAvailable';
  if (activeTab.value === 'disabled') return plugin.status === 'disabled' || plugin.status === 'failed';
  return plugin.status !== 'updateAvailable';
}));

async function refresh(): Promise<void> {
  try { await store.refreshPlugins(); } catch { /* The page alert owns the native failure. */ }
}

async function selectInstall(): Promise<void> {
  try { installManifest.value = await store.selectPluginInstall(); } catch { /* The page alert owns the native failure. */ }
}

async function install(pluginId: string): Promise<void> {
  try {
    await store.installPlugin(pluginId);
    isInstallOpen.value = false;
    installManifest.value = null;
  } catch { /* The page alert owns the native failure. */ }
}

async function toggle(plugin: PluginSnapshot): Promise<void> {
  try { await store.togglePlugin(plugin); } catch { /* Native snapshot remains unchanged. */ }
}

async function uninstall(plugin: PluginSnapshot): Promise<void> {
  try {
    await ElMessageBox.confirm(t('plugins.uninstallConfirm'), t('plugins.uninstall'), {
      type: 'warning',
      confirmButtonText: t('common.confirm'),
      cancelButtonText: t('common.cancel')
    });
  } catch {
    return;
  }
  try { await store.uninstallPlugin(plugin.id); } catch { /* The page alert owns the native failure. */ }
}
</script>

<template>
  <section class="min-w-0 bg-white px-7 py-6 dark:bg-gray-900">
    <header class="flex flex-wrap items-center justify-between gap-3">
      <h2 class="m-0 text-xl font-semibold">
        {{ t('plugins.title') }}
      </h2>
      <div class="flex flex-wrap gap-2">
        <el-button :icon="Upload" type="primary" @click="isInstallOpen = true">
          {{ t('plugins.install') }}
        </el-button>
        <el-button :icon="Refresh" :loading="store.pending.has('refreshPlugins')" @click="refresh">
          {{ t('plugins.refresh') }}
        </el-button>
      </div>
    </header>
    <el-alert
      class="mt-5"
      type="info"
      show-icon
      :closable="false"
      :title="t('plugins.safety')"
    />
    <el-alert
      v-if="store.error"
      class="mt-3"
      type="warning"
      show-icon
      :closable="false"
      :title="store.error.message"
    />
    <el-tabs v-model="activeTab" class="mt-5">
      <el-tab-pane :label="t('plugins.installed')" name="installed" />
      <el-tab-pane :label="t('plugins.updateAvailable')" name="updateAvailable" />
      <el-tab-pane :label="t('plugins.disabled')" name="disabled" />
    </el-tabs>
    <div class="min-w-0 overflow-x-auto">
      <el-table :data="visiblePlugins" :empty-text="t('plugins.noPlugins')" class="min-w-[760px]">
        <el-table-column prop="name" :label="t('plugins.name')" min-width="180">
          <template #default="{ row }">
            <span class="font-semibold">{{ row.name }}</span>
          </template>
        </el-table-column>
        <el-table-column prop="version" :label="t('plugins.version')" width="110" />
        <el-table-column :label="t('plugins.enabled')" width="140">
          <template #default="{ row }">
            <el-tag :type="row.status === 'enabled' ? 'success' : row.status === 'failed' ? 'danger' : 'info'">
              {{ t(`plugins.status.${row.status}`) }}
            </el-tag>
          </template>
        </el-table-column>
        <el-table-column :label="t('plugins.permissionsLabel')" min-width="220">
          <template #default="{ row }">
            <div class="flex flex-wrap gap-1">
              <el-tag v-for="permission in row.permissions" :key="permission" size="small">
                {{ t(`plugins.permissions.${permission}`) }}
              </el-tag>
            </div>
          </template>
        </el-table-column>
        <el-table-column :label="t('plugins.actions')" width="230" fixed="right">
          <template #default="{ row }">
            <el-button size="small" :loading="store.pending.has(`plugin:${row.id}`)" @click="toggle(row)">
              {{ row.status === 'enabled' ? t('plugins.disable') : t('plugins.enable') }}
            </el-button>
            <el-button size="small" :icon="Setting" disabled>
              {{ t('plugins.configure') }}
            </el-button>
            <el-button
              size="small"
              type="danger"
              :icon="Delete"
              @click="uninstall(row)"
            />
          </template>
        </el-table-column>
      </el-table>
    </div>
    <PluginInstallDialog
      v-model="isInstallOpen"
      :manifest="installManifest"
      :is-pending="store.pending.has('selectPluginInstall')"
      @select="selectInstall"
      @install="install"
    />
  </section>
</template>
