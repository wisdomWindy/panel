/**
 * 更新状态管理
 * 管理更新相关的状态和操作
 */

import { defineStore } from 'pinia';
import { ref, computed } from 'vue';
import { ChangelogEntry, UpdateCheckResult, getUpdateService } from '../services/update-service';

export const useUpdateStore = defineStore('update', () => {
  // State
  const currentVersion = ref<string>('0.1.0');
  const latestVersion = ref<string | null>(null);
  const hasUpdate = ref<boolean>(false);
  const releaseUrl = ref<string>('');
  const isChecking = ref<boolean>(false);
  const lastCheckTime = ref<Date | null>(null);
  const changelog = ref<ChangelogEntry[]>([]);

  // Getters
  const hasChangelog = computed(() => changelog.value.length > 0);
  const latestChangelogEntry = computed(() => {
    if (changelog.value.length === 0) return null;
    return changelog.value[0];
  });

  // Actions
  async function checkForUpdates(): Promise<void> {
    if (isChecking.value) return;

    isChecking.value = true;
    try {
      const updateService = getUpdateService();
      const result: UpdateCheckResult = await updateService.checkForUpdates();

      currentVersion.value = result.currentVersion;
      latestVersion.value = result.latestVersion;
      hasUpdate.value = result.hasUpdate;
      releaseUrl.value = result.releaseUrl;
      lastCheckTime.value = new Date();
    } catch (error) {
      console.error('Failed to check for updates:', error);
    } finally {
      isChecking.value = false;
    }
  }

  async function parseChangelog(): Promise<void> {
    try {
      const updateService = getUpdateService();
      changelog.value = await updateService.parseChangelog();
    } catch (error) {
      console.error('Failed to parse changelog:', error);
      changelog.value = [];
    }
  }

  function setCurrentVersion(version: string): void {
    currentVersion.value = version;
  }

  function clearUpdateStatus(): void {
    hasUpdate.value = false;
    latestVersion.value = null;
    releaseUrl.value = '';
  }

  return {
    // State
    currentVersion,
    latestVersion,
    hasUpdate,
    releaseUrl,
    isChecking,
    lastCheckTime,
    changelog,

    // Getters
    hasChangelog,
    latestChangelogEntry,

    // Actions
    checkForUpdates,
    parseChangelog,
    setCurrentVersion,
    clearUpdateStatus
  };
});
