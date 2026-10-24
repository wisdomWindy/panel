<script setup lang="ts">
import { computed, nextTick, ref, watch } from 'vue';
import { useI18n } from 'vue-i18n';
import { Delete, VideoPause, VideoPlay } from '@element-plus/icons-vue';
import type { LiveEvent, UserUid } from '@/platform/desktop';
import {
  formatCount,
  formatDanmuRate,
  formatMoneyMinor,
  formatTimestamp
} from '@/modules/live-data/formatters';
import { useLiveAnalyticsStore } from '@/stores/live-data/analytics';
import { useLiveFeedStore } from '@/stores/live-data/feed';
import { useLiveRoomStore } from '@/stores/live-data/room';
import AudienceDetailDrawer from '@/components/live-data/AudienceDetailDrawer.vue';
import AudienceIdentityView from '@/components/live-data/AudienceIdentityView.vue';

  type Filter = 'all' | 'captain' | 'gift' | 'highlight';

const feed = useLiveFeedStore();
const room = useLiveRoomStore();
const analytics = useLiveAnalyticsStore();
const { t } = useI18n();
const filter = ref<Filter>('all');
const search = ref('');
const stream = ref<HTMLElement | null>(null);
const selectedUid = ref<UserUid | null>(null);
const filterOptions = computed(() => [
  { label: t('danmakuPanel.all'), value: 'all' },
  { label: t('danmakuPanel.captainsOnly'), value: 'captain' },
  { label: t('danmakuPanel.giftsOnly'), value: 'gift' },
  { label: t('danmakuPanel.highlightsOnly'), value: 'highlight' }
]);

const filteredEvents = computed(() => {
  const query = search.value.trim().toLocaleLowerCase();
  return feed.events.filter(event => {
    const captain =
        event.kind === 'captain' ||
        event.audience.tags.some(tag => ['governor', 'admiral', 'captain'].includes(tag));
    const highlighted =
        captain || event.audience.tags.includes('admin') || event.audience.tags.includes('vip');
    if (filter.value === 'captain' && !captain) return false;
    if (filter.value === 'gift' && event.kind !== 'gift') return false;
    if (filter.value === 'highlight' && !highlighted) return false;
    if (!query) return true;
    return (
      event.audience.username.toLocaleLowerCase().includes(query) ||
        eventText(event).toLocaleLowerCase().includes(query)
    );
  });
});

const metrics = computed(() => analytics.statistics?.metrics ?? null);

watch(
  () => feed.events.length,
  async () => {
    if (feed.isPaused) return;
    await nextTick();
    if (stream.value) stream.value.scrollTop = stream.value.scrollHeight;
  }
);

function eventText(event: LiveEvent): string {
  if (event.kind === 'danmu') return event.content;
  if (event.kind === 'gift') return `${event.giftName} x${event.quantity}`;
  if (event.kind === 'captain')
    return `${t(`danmakuPanel.${event.action}`)}${t(`liveData.tags.${event.level}`)}`;
  return t('danmakuPanel.entry');
}

function clearPanel(): void {
  feed.clear();
  ElMessage.success(t('danmakuPanel.cleared'));
}

function handleBlacklisted(uid: UserUid): void {
  feed.removeAudience(uid);
}
</script>

<template>
  <section class="mx-auto flex h-full min-h-[560px] max-w-7xl flex-col bg-white dark:bg-zinc-900">
    <header class="flex min-h-14 flex-wrap items-center gap-3 border-b border-gray-200 py-2 dark:border-zinc-800">
      <h2 class="mr-auto m-0 text-lg font-semibold text-gray-900 dark:text-zinc-100">
        {{ t('danmakuPanel.title') }}
      </h2>
      <el-segmented v-model="filter" :options="filterOptions" />
      <el-input
        v-model="search"
        clearable
        class="!w-56"
        :placeholder="t('danmakuPanel.searchPlaceholder')"
        :aria-label="t('danmakuPanel.searchPlaceholder')"
      />
      <el-button
        :icon="feed.isPaused ? VideoPlay : VideoPause"
        @click="feed.setPaused(!feed.isPaused)"
      >
        {{ feed.isPaused ? t('danmakuPanel.resume') : t('danmakuPanel.pause') }}
      </el-button>
      <el-button :icon="Delete" @click="clearPanel">
        {{ t('danmakuPanel.clear') }}
      </el-button>
    </header>

    <el-alert
      v-if="room.snapshot.status === 'reconnecting'"
      class="mt-3"
      type="warning"
      :closable="false"
      :title="t('danmakuPanel.reconnecting', { attempt: room.snapshot.reconnectAttempt })"
      show-icon
    />
    <el-alert
      v-else-if="room.snapshot.status === 'error'"
      class="mt-3"
      type="error"
      :closable="false"
      :title="t('danmakuPanel.reconnectFailed')"
      show-icon
    />

    <div
      ref="stream"
      class="min-h-0 flex-1 overflow-y-auto"
      tabindex="0"
      :aria-label="t('danmakuPanel.eventList')"
    >
      <div
        v-if="filteredEvents.length === 0"
        class="flex h-full min-h-72 items-center justify-center text-sm text-gray-400 dark:text-zinc-400"
      >
        {{ t('liveData.empty') }}
      </div>
      <article
        v-for="event in filteredEvents"
        :key="event.eventId"
        class="grid min-h-16 grid-cols-[minmax(260px,0.9fr)_minmax(280px,1.5fr)_88px] items-center gap-4 border-b border-gray-100 px-3 py-2 text-sm dark:border-zinc-800 max-lg:grid-cols-[1fr_80px]"
      >
        <AudienceIdentityView
          :identity="event.audience"
          interactive
          class="max-lg:col-span-1"
          @select="selectedUid = event.audience.userUid"
        />
        <div class="min-w-0 text-gray-800 dark:text-zinc-200 max-lg:col-span-2 max-lg:row-start-2">
          <span v-if="event.kind === 'danmu'" class="break-words">{{ event.content }}</span>
          <span v-else-if="event.kind === 'gift'">
            {{
              t('danmakuPanel.giftEvent', {
                giftName: event.giftName,
                quantity: event.quantity,
                value: formatMoneyMinor(event.valueMinor)
              })
            }}
          </span>
          <span v-else-if="event.kind === 'captain'">
            {{
              t('danmakuPanel.captainEvent', {
                action: t(`danmakuPanel.${event.action}`),
                level: t(`liveData.tags.${event.level}`),
                value: formatMoneyMinor(event.valueMinor)
              })
            }}
          </span>
          <span v-else>{{ t('danmakuPanel.entry') }}</span>
        </div>
        <time class="justify-self-end tabular-nums text-gray-500 dark:text-zinc-400">
          {{ formatTimestamp(event.occurredAt) }}
        </time>
      </article>
    </div>

    <footer
      class="flex min-h-11 flex-wrap items-center gap-x-6 gap-y-1 border-t border-gray-200 px-3 text-sm text-gray-600 dark:border-zinc-800 dark:text-zinc-400"
    >
      <span>
        {{ t('danmakuPanel.total') }}
        <strong class="tabular-nums text-gray-900 dark:text-zinc-100">
          {{ formatCount(metrics?.danmuCount ?? 0) }}
        </strong>
      </span>
      <span>
        {{ t('danmakuPanel.rate') }}
        <strong class="tabular-nums text-gray-900 dark:text-zinc-100">
          {{ formatDanmuRate(metrics?.danmuRate ?? 0) }}
        </strong>
      </span>
      <router-link to="/logs" class="text-pink-600 hover:underline">
        {{ t('danmakuPanel.filtered', { count: formatCount(metrics?.filteredCount ?? 0) }) }}
      </router-link>
    </footer>

    <AudienceDetailDrawer
      :visible="selectedUid !== null"
      :room-id="room.snapshot.roomId"
      :user-uid="selectedUid"
      @close="selectedUid = null"
      @blacklisted="handleBlacklisted"
    />
  </section>
</template>
