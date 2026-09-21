<script setup lang="ts">
import { computed, onMounted, onUnmounted, ref } from 'vue';
import { useI18n } from 'vue-i18n';
import { useRoute, useRouter } from 'vue-router';
import type { RoomId } from '@/platform/desktop';
import { elementPlusLocale } from '@/i18n-runtime';
import AppShellNavigation from '@/components/application-shell/AppShellNavigation.vue';
import AppShellStatusBar from '@/components/application-shell/AppShellStatusBar.vue';
import AppShellToolbar from '@/components/application-shell/AppShellToolbar.vue';
import { useApplicationShellRuntime } from '@/composables/useApplicationShellRuntime';
import { useLiveDataRuntime } from '@/composables/useLiveDataRuntime';
import { useMediaRuntime } from '@/composables/useMediaRuntime';
import { useReleaseRuntime } from '@/composables/useReleaseRuntime';
import {
  buildNavigationTree,
  type ShellNavigationItem
} from '@/modules/application-shell/navigation';
import { buildStatusBarView, normalizeRoomInput } from '@/modules/application-shell/view-model';
import { useApplicationShellStore } from '@/stores/application-shell';
import { useLiveFeedStore } from '@/stores/live-data/feed';
import { useLiveRoomStore } from '@/stores/live-data/room';
import { useMediaRecordingStore } from '@/stores/media/recording';
import { memberChannel } from '@member-channel';

const { t } = useI18n();
const route = useRoute();
const router = useRouter();
const shell = useApplicationShellStore();
const room = useLiveRoomStore();
const feed = useLiveFeedStore();
const recording = useMediaRecordingStore();
const applicationShellRuntime = useApplicationShellRuntime();
const liveDataRuntime = useLiveDataRuntime();
const mediaRuntime = useMediaRuntime();
const releaseRuntime = useReleaseRuntime();

const roomInput = ref('');
const roomError = ref<string | null>(null);
const notificationOpen = ref(false);
const navigation = computed(() => memberChannel.buildNavigation(buildNavigationTree()));
const recentRooms = computed(() => room.snapshot.roomId ? [room.snapshot.roomId] : []);
const acceptedDanmakuCount = computed(() => {
  const cutoff = shell.statusObservedAt - 1_000;
  return feed.events.filter(event => event.kind === 'danmu' && event.occurredAt >= cutoff).length;
});
const statusItems = computed(() => buildStatusBarView({
  room: room.snapshot,
  acceptedDanmakuCount: acceptedDanmakuCount.value,
  recordingState: recording.status?.state ?? 'idle',
  metrics: shell.metrics,
  locale: shell.locale?.locale ?? 'zh-CN'
}));

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
    await room.connect(normalized.value as RoomId);
  } catch {
    roomError.value = t('shell.errors.operationFailed');
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

onMounted(() => {
  void Promise.allSettled([
    applicationShellRuntime.start(),
    liveDataRuntime.start(),
    mediaRuntime.start(),
    releaseRuntime.start(),
    memberChannel.start()
  ]);
});

onUnmounted(() => {
  applicationShellRuntime.stop();
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
      class="grid h-screen min-h-0 min-w-0 grid-rows-[var(--shell-toolbar-height,64px)_minmax(0,1fr)_var(--shell-status-height,36px)] overflow-hidden bg-zinc-100 text-[length:var(--shell-font-size,14px)] text-zinc-950 opacity-[var(--shell-opacity,0.98)] dark:bg-zinc-950 dark:text-zinc-100"
    >
      <AppShellToolbar
        v-model="roomInput"
        :rooms="recentRooms"
        :room="room.snapshot"
        :is-pending="room.pending"
        :room-error="roomError"
        :notification-open="notificationOpen"
        :member-toolbar-component="memberChannel.toolbarComponent"
        @update:notification-open="notificationOpen = $event"
        @toggle-connection="toggleConnection"
        @open-notifications="openNotifications"
        @open-settings="openSettings"
      />
      <div class="flex min-h-0 min-w-0">
        <AppShellNavigation
          :items="navigation"
          :active-route="route.path"
          @navigate="navigate"
        />
        <main class="min-w-0 flex-1 overflow-y-auto bg-zinc-100 p-3 sm:p-5 dark:bg-zinc-950">
          <router-view />
        </main>
      </div>
      <AppShellStatusBar :items="statusItems" />
    </div>
  </el-config-provider>
</template>
