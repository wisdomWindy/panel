<script setup lang="ts">
import { computed, ref } from 'vue';
import { useI18n } from 'vue-i18n';
import { Brush, Files, InfoFilled, Lock, Location, Refresh } from '@element-plus/icons-vue';
import { ElMessage } from 'element-plus';
import { useReleaseStore } from '@/stores/release';
import {
  detectHostPlatform,
  isFullyVerifiedPlatform
} from '@/modules/application-shell/platform-support';
import ConfigManagementPanel from './components/ConfigManagementPanel.vue';
import DataPrivacyPanel from './components/DataPrivacyPanel.vue';
import InterfaceSettingsPanel from './components/InterfaceSettingsPanel.vue';
import LanguageSettings from './components/LanguageSettings.vue';

type SettingsCategory = 'config' | 'language' | 'interface' | 'data' | 'about';

const { t } = useI18n();
const release = useReleaseStore();
const activeCategory = ref<SettingsCategory>('config');
const title = computed(() => t(`settings.categories.${activeCategory.value}`));
const hostPlatform = detectHostPlatform(
  typeof navigator === 'undefined' ? '' : navigator.userAgent
);
const platformIsVerified = isFullyVerifiedPlatform(hostPlatform);
const categories = [
  { id: 'config' as const, icon: Files },
  { id: 'language' as const, icon: Location },
  { id: 'interface' as const, icon: Brush },
  { id: 'data' as const, icon: Lock },
  { id: 'about' as const, icon: InfoFilled }
];

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
  <section class="mx-auto flex w-full max-w-[1180px] min-w-0 flex-col gap-4">
    <header class="flex flex-wrap items-center justify-between gap-3">
      <div class="mr-auto min-w-0">
        <h2 class="app-text m-0 text-lg font-semibold">
          {{ t('settings.title') }}
        </h2>
        <p class="app-muted mt-1 mb-0 text-sm">
          {{ t('settings.pageHint') }}
        </p>
      </div>
    </header>

    <div class="flex min-w-0 flex-col gap-4 lg:flex-row lg:items-start">
      <aside class="app-card w-full shrink-0 p-2 lg:w-52">
        <nav
          class="flex flex-col gap-0.5"
          role="tablist"
          aria-orientation="vertical"
          :aria-label="t('settings.title')"
        >
          <button
            v-for="category in categories"
            :id="`settings-tab-${category.id}`"
            :key="category.id"
            type="button"
            role="tab"
            class="app-nav-link"
            :aria-selected="activeCategory === category.id"
            :aria-current="activeCategory === category.id ? 'page' : undefined"
            @click="activeCategory = category.id"
          >
            <el-icon><component :is="category.icon" /></el-icon>
            <span>{{ t(`settings.categories.${category.id}`) }}</span>
          </button>
        </nav>
      </aside>

      <div class="min-w-0 flex-1">
        <section
          class="app-card p-5"
          role="tabpanel"
          :aria-labelledby="`settings-tab-${activeCategory}`"
        >
          <header class="mb-5 flex flex-wrap items-start justify-between gap-3">
            <div class="min-w-0">
              <h3 class="app-text m-0 text-base font-semibold">
                {{ title }}
              </h3>
              <p class="app-muted mt-1 mb-0 text-sm">
                {{ t(`settings.categoryHints.${activeCategory}`) }}
              </p>
            </div>
            <button
              v-if="activeCategory === 'about'"
              type="button"
              class="app-btn-primary inline-flex h-9 shrink-0 items-center gap-1.5 px-4 text-sm disabled:cursor-not-allowed disabled:opacity-60"
              :aria-busy="release.pending.has('checkUpdate')"
              :disabled="release.pending.has('checkUpdate')"
              @click="handleCheckUpdate"
            >
              <el-icon :class="{ 'is-loading': release.pending.has('checkUpdate') }">
                <Refresh />
              </el-icon>
              {{ t('update.checkUpdate') }}
            </button>
          </header>

          <ConfigManagementPanel v-if="activeCategory === 'config'" />
          <LanguageSettings v-else-if="activeCategory === 'language'" />
          <InterfaceSettingsPanel v-else-if="activeCategory === 'interface'" />
      <DataPrivacyPanel v-else-if="activeCategory === 'data'" />
          <div v-else class="max-w-4xl">
            <dl class="m-0 grid grid-cols-[9rem_minmax(0,1fr)] gap-x-4 gap-y-3 text-sm">
              <dt class="app-muted">
                {{ t('settings.aboutInfo.version') }}
              </dt>
              <dd class="app-text m-0 font-semibold tabular-nums">
                v{{ release.about?.applicationVersion ?? '--' }}
              </dd>
              <dt class="app-muted">
                {{ t('settings.aboutInfo.platform') }}
              </dt>
              <dd class="app-text m-0 font-semibold">
                {{ t(`settings.aboutInfo.platforms.${hostPlatform}`) }}
              </dd>
            </dl>
            <p
              v-if="!platformIsVerified"
              class="app-muted mt-3 mb-0 text-sm"
              role="note"
              style="color: var(--app-gold)"
            >
              {{ t('settings.aboutInfo.platformUnverified') }}
            </p>
            <div class="app-hairline my-5 border-t" />
            <h4 class="app-text m-0 mb-3 text-sm font-semibold">
              {{ t('settings.aboutInfo.licenses') }}
            </h4>
            <el-table
              v-if="release.about?.licenses.length"
              :data="release.about.licenses"
              class="w-full"
              size="small"
            >
              <el-table-column :label="t('settings.aboutInfo.component')">
                <template #default="{ row }">
                  <span class="app-text">{{ row.name }}</span>
                </template>
              </el-table-column>
              <el-table-column :label="t('settings.aboutInfo.license')">
                <template #default="{ row }">
                  <span class="app-muted">{{ row.license }}</span>
                </template>
              </el-table-column>
              <el-table-column :label="t('settings.aboutInfo.source')" min-width="240">
                <template #default="{ row }">
                  <span class="app-faint">{{ row.sourceUrl }}</span>
                </template>
              </el-table-column>
            </el-table>
            <p v-else class="app-faint m-0 text-sm">
              {{ t('settings.aboutInfo.unavailable') }}
            </p>
          </div>
        </section>
      </div>
    </div>
  </section>
</template>
