<script setup lang="ts">
import { computed, ref } from 'vue';
import { Brush, Files, InfoFilled, Location, Refresh } from '@element-plus/icons-vue';
import { ElMessage } from 'element-plus';
import { useI18n } from 'vue-i18n';
import { useReleaseStore } from '@/stores/release';
import ConfigManagementPanel from './components/ConfigManagementPanel.vue';
import InterfaceSettingsPanel from './components/InterfaceSettingsPanel.vue';
import LanguageSettings from './components/LanguageSettings.vue';

type SettingsCategory = 'config' | 'language' | 'interface' | 'about';

const { t } = useI18n();
const release = useReleaseStore();
const activeCategory = ref<SettingsCategory>('config');
const title = computed(() => t(`settings.categories.${activeCategory.value}`));

async function handleCheckUpdate(): Promise<void> {
  try {
    await release.checkUpdate();
    if (release.update?.candidate) {
      ElMessage.success(t('update.hasUpdate'));
      return;
    }
    if (release.update?.status === 'current') {
      ElMessage.success(t('update.noUpdate'));
      return;
    }
    ElMessage.error(t('update.unavailable'));
  } catch {
    ElMessage.error(release.error?.message ?? t('settings.feedback.operationFailed'));
  }
}
</script>

<template>
  <section class="flex h-full min-h-[520px] min-w-0 overflow-hidden bg-white dark:bg-gray-900">
    <aside class="w-52 shrink-0 border-r border-gray-200 py-3 dark:border-gray-700">
      <el-menu
        :default-active="activeCategory"
        class="border-r-0"
        @select="activeCategory = $event as SettingsCategory"
      >
        <el-menu-item index="config">
          <el-icon><Files /></el-icon>
          <span>{{ t('settings.categories.config') }}</span>
        </el-menu-item>
        <el-menu-item index="language">
          <el-icon><Location /></el-icon>
          <span>{{ t('settings.categories.language') }}</span>
        </el-menu-item>
        <el-menu-item index="interface">
          <el-icon><Brush /></el-icon>
          <span>{{ t('settings.categories.interface') }}</span>
        </el-menu-item>
        <el-menu-item index="about">
          <el-icon><InfoFilled /></el-icon>
          <span>{{ t('settings.categories.about') }}</span>
        </el-menu-item>
      </el-menu>
    </aside>
    <div class="min-w-0 flex-1 overflow-y-auto px-7 py-6">
      <h2 class="m-0 mb-6 text-xl font-semibold text-gray-900 dark:text-gray-100">
        {{ title }}
      </h2>
      <ConfigManagementPanel v-if="activeCategory === 'config'" />
      <LanguageSettings v-else-if="activeCategory === 'language'" />
      <InterfaceSettingsPanel v-else-if="activeCategory === 'interface'" />
      <div v-else class="max-w-3xl">
        <div class="mb-5 flex justify-end">
          <el-button
            :icon="Refresh"
            :loading="release.pending.has('checkUpdate')"
            @click="handleCheckUpdate"
          >
            {{ t('update.checkUpdate') }}
          </el-button>
        </div>
        <dl class="grid grid-cols-[9rem_minmax(0,1fr)] gap-x-4 gap-y-3 text-sm">
          <dt class="text-gray-500">
            {{ t('settings.aboutInfo.version') }}
          </dt>
          <dd class="m-0 font-semibold">
            v{{ release.about?.applicationVersion ?? '--' }}
          </dd>
        </dl>
        <div class="my-6 border-t border-gray-200 dark:border-gray-700" />
        <h3 class="mb-4 text-sm font-semibold">
          {{ t('settings.aboutInfo.licenses') }}
        </h3>
        <el-table
          v-if="release.about?.licenses.length"
          :data="release.about.licenses"
          class="w-full"
        >
          <el-table-column prop="name" :label="t('settings.aboutInfo.component')" />
          <el-table-column prop="license" :label="t('settings.aboutInfo.license')" />
          <el-table-column prop="sourceUrl" :label="t('settings.aboutInfo.source')" min-width="240" />
        </el-table>
        <el-alert
          v-else
          type="info"
          show-icon
          :closable="false"
          :title="t('settings.aboutInfo.unavailable')"
        />
      </div>
    </div>
  </section>
</template>
