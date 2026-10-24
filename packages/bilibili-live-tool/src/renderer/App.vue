<script setup lang="ts">
import { computed, onMounted, onUnmounted, ref, watch } from 'vue';
import { ElMessage } from 'element-plus';
import { useI18n } from 'vue-i18n';
import { useRoute, useRouter } from 'vue-router';
import { elementPlusLocale } from '@/i18n-runtime';
import AppShellNavigation from '@/components/application-shell/AppShellNavigation.vue';
import AppShellStatusBar from '@/components/application-shell/AppShellStatusBar.vue';
import AppShellToolbar from '@/components/application-shell/AppShellToolbar.vue';
import GiftAlertOverlay from '@/components/media/GiftAlertOverlay.vue';
import { useApplicationShellRuntime } from '@/composables/useApplicationShellRuntime';
import { useAiRuntime } from '@/composables/useAiRuntime';
import { useLiveDataRuntime } from '@/composables/useLiveDataRuntime';
import { useMediaRuntime } from '@/composables/useMediaRuntime';
import { useReleaseRuntime } from '@/composables/useReleaseRuntime';
import UpdateAvailableDialog from '@/views/update-log/UpdateAvailableDialog.vue';
import {
  buildNavigationTree,
  type ShellNavigationItem
} from '@/modules/application-shell/navigation';
import {
  buildStatusBarView,
  connectionErrorMessageKey,
  normalizeRoomInput
} from '@/modules/application-shell/view-model';
import { useApplicationShellStore } from '@/stores/application-shell';
import { useLiveFeedStore } from '@/stores/live-data/feed';
import { useLiveRoomStore } from '@/stores/live-data/room';
import { useMediaRecordingStore } from '@/stores/media/recording';
import { useMediaGiftsStore } from '@/stores/media/gifts';
import { useReleaseStore } from '@/stores/release';
import { memberChannel } from '@member-channel';

const { locale, t } = useI18n();
const route = useRoute();
const router = useRouter();
const shell = useApplicationShellStore();
const room = useLiveRoomStore();
const feed = useLiveFeedStore();
const recording = useMediaRecordingStore();
const gifts = useMediaGiftsStore();
const release = useReleaseStore();
const applicationShellRuntime = useApplicationShellRuntime();
const aiRuntime = useAiRuntime();
const liveDataRuntime = useLiveDataRuntime();
const mediaRuntime = useMediaRuntime();
const releaseRuntime = useReleaseRuntime();

const roomInput = ref('');
const roomError = ref<string | null>(null);
const notificationOpen = ref(false);
const isUpdateDialogOpen = ref(false);
const isReleasePending = computed(() => release.pending.size > 0);
const navigation = computed(() => memberChannel.buildNavigation(buildNavigationTree()));
const acceptedDanmakuCount = computed(() => {
  const cutoff = shell.statusObservedAt - 1_000;
  return feed.events.filter(event => event.kind === 'danmu' && event.occurredAt >= cutoff).length;
});
const statusItems = computed(() =>
  buildStatusBarView({
    room: room.snapshot,
    acceptedDanmakuCount: acceptedDanmakuCount.value,
    recordingState: recording.status?.state ?? 'idle',
    metrics: shell.metrics,
    locale: shell.locale?.locale ?? 'zh-CN'
  })
);
const displayedRoomError = computed(() => {
  if (roomError.value) return roomError.value;
  const key = connectionErrorMessageKey(room.snapshot.lastErrorCode);
  return key ? t(`shell.errors.${key}`) : null;
});

watch(
  () => release.update,
  snapshot => {
    const isActionable =
        snapshot?.candidate && ['available', 'downloading', 'downloaded'].includes(snapshot.status);
    if (!isActionable) {
      isUpdateDialogOpen.value = false;
    } else if (route.path !== '/update-log') {
      isUpdateDialogOpen.value = true;
    }
  }
);

watch(
  [() => route.meta.titleKey, locale],
  ([titleKey]) => {
    document.title = titleKey ? `${t(String(titleKey))} - biliLiveHelper` : 'biliLiveHelper';
  },
  { immediate: true }
);

async function toggleConnection(): Promise<void> {
  roomError.value = null;
  try {
    if (room.snapshot.status === 'connected' || room.snapshot.status === 'reconnecting') {
      await room.disconnect();
      return;
    }
    const normalized = normalizeRoomInput(roomInput.value);
    if (!normalized.ok) {
      roomError.value = t(`shell.errors.${normalized.code}`);
      return;
    }
    await room.connect(normalized.value);
  } catch {
    roomError.value = room.error?.message ?? t('shell.errors.operationFailed');
  }
}

function navigate(item: ShellNavigationItem): void {
  if (item.route && !item.isDisabled) void router.push(item.route);
}

function openNotifications(): void {
  notificationOpen.value = !notificationOpen.value;
}

function openSettings(): void {
  void router.push('/settings');
}

async function handleDownloadUpdate(): Promise<void> {
  if (!release.update?.candidate) return;
  try {
    await release.downloadUpdate();
  } catch {
    ElMessage.error(release.error?.message ?? t('settings.feedback.operationFailed'));
  }
}

async function handleInstallUpdate(): Promise<void> {
  if (!release.update?.candidate) return;
  try {
    await release.installUpdate();
  } catch {
    ElMessage.error(release.error?.message ?? t('settings.feedback.operationFailed'));
  }
}

async function handleSkipUpdate(): Promise<void> {
  const candidate = release.update?.candidate;
  if (!candidate) return;
  try {
    await release.skipUpdate(candidate.version);
    isUpdateDialogOpen.value = false;
  } catch {
    ElMessage.error(release.error?.message ?? t('settings.feedback.operationFailed'));
  }
}

onMounted(() => {
  void Promise.allSettled([
    applicationShellRuntime.start(),
    aiRuntime.start(),
    liveDataRuntime.start(),
    mediaRuntime.start(),
    releaseRuntime.start(),
    memberChannel.start()
  ]);
});

onUnmounted(() => {
  applicationShellRuntime.stop();
  aiRuntime.stop();
  liveDataRuntime.stop();
  mediaRuntime.stop();
  releaseRuntime.stop();
  memberChannel.stop();
});
</script>

<template>
  <el-config-provider :locale="elementPlusLocale">
    <div
      :aria-busy="shell.pending"
      class="grid h-screen min-h-0 min-w-0 grid-rows-[var(--shell-toolbar-height,64px)_minmax(0,1fr)_var(--shell-status-height,36px)] overflow-hidden bg-white text-[length:var(--shell-font-size,14px)] text-zinc-950 opacity-[var(--shell-opacity,0.98)] dark:bg-zinc-950 dark:text-zinc-100"
    >
      <AppShellToolbar
        v-model="roomInput"
        :room="room.snapshot"
        :is-pending="room.pending"
        :room-error="displayedRoomError"
        :notification-open="notificationOpen"
        :member-toolbar-component="memberChannel.toolbarComponent"
        @update:notification-open="notificationOpen = $event"
        @toggle-connection="toggleConnection"
        @open-notifications="openNotifications"
        @open-settings="openSettings"
      />
      <div class="flex min-h-0 min-w-0">
        <AppShellNavigation :items="navigation" :active-route="route.path" @navigate="navigate" />
        <main class="min-w-0 flex-1 overflow-y-auto bg-white p-3 sm:p-5 dark:bg-zinc-950">
          <router-view />
        </main>
      </div>
      <AppShellStatusBar :items="statusItems" />
    </div>
    <UpdateAvailableDialog
      v-if="release.update && release.update.candidate && route.path !== '/update-log'"
      v-model="isUpdateDialogOpen"
      :snapshot="release.update"
      :is-pending="isReleasePending"
      @download="handleDownloadUpdate"
      @install="handleInstallUpdate"
      @skip="handleSkipUpdate"
    />
    <GiftAlertOverlay :payload="gifts.latestAlert" @dismiss="gifts.dismissAlert" />
  </el-config-provider>
</template>
