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
import { useApplicationShellStore } from '@/stores/application-shell';
import { useLiveFeedStore } from '@/stores/live-data/feed';
import { useLiveRoomStore } from '@/stores/live-data/room';
import { connectionErrorMessageKey } from '@/modules/application-shell/view-model';
import AudienceDetailDrawer from '@/components/live-data/AudienceDetailDrawer.vue';
import AudienceIdentityView from '@/components/live-data/AudienceIdentityView.vue';

/** PRD 8.1 弹幕面板 filters mirror the design's identity chips. */
type Filter = 'all' | 'admin' | 'captain' | 'gift' | 'ordinary';

const feed = useLiveFeedStore();
const shell = useApplicationShellStore();
const room = useLiveRoomStore();
const analytics = useLiveAnalyticsStore();
const { t } = useI18n();

/**
 * Reconnecting never gives up any more, so an `error` status is always a
 * permanent failure: show its real cause instead of a retry prompt.
 */
const roomErrorText = computed(() => {
  const key = connectionErrorMessageKey(room.snapshot.lastErrorCode);
  return t(`shell.errors.${key ?? 'operationFailed'}`);
});
const filter = ref<Filter>('all');
const search = ref('');
const stream = ref<HTMLElement | null>(null);
const selectedUid = ref<UserUid | null>(null);

const isCaptain = (event: LiveEvent): boolean =>
  event.kind === 'captain' ||
  event.audience.tags.some(tag => ['governor', 'admiral', 'captain'].includes(tag));

const matchesFilter = (event: LiveEvent, value: Filter): boolean => {
  switch (value) {
    case 'admin':
      return event.audience.tags.includes('admin');
    case 'captain':
      return isCaptain(event);
    case 'gift':
      return event.kind === 'gift';
    case 'ordinary':
      return (
        event.audience.tags.length === 0 &&
        (event.kind === 'danmu' || event.kind === 'enter')
      );
    default:
      return true;
  }
};

const filterOptions = computed<readonly { label: string; value: Filter; count: string | null }[]>(() => [
  {
    label: t('danmakuPanel.all'),
    value: 'all',
    count: formatCount(metrics.value?.danmuCount ?? feed.events.length)
  },
  { label: t('danmakuPanel.adminOnly'), value: 'admin', count: null },
  { label: t('danmakuPanel.captainsOnly'), value: 'captain', count: null },
  { label: t('danmakuPanel.giftsOnly'), value: 'gift', count: null },
  { label: t('danmakuPanel.ordinaryOnly'), value: 'ordinary', count: null }
]);

const filteredEvents = computed(() => {
  const query = search.value.trim().toLocaleLowerCase();
  return feed.events.filter(event => {
    if (!matchesFilter(event, filter.value)) return false;
    if (!query) return true;
    return (
      event.audience.username.toLocaleLowerCase().includes(query) ||
      eventText(event).toLocaleLowerCase().includes(query)
    );
  });
});

const metrics = computed(() => analytics.statistics?.metrics ?? null);

/**
 * PRD 9.14: the display preferences were previously stored but never applied.
 * `previewUi` is set while the settings panel previews, so it wins.
 */
const display = computed(() => shell.previewUi ?? shell.savedUi);
const shownEvents = computed(() => {
  const limit = display.value?.maxDanmakuEntries ?? 1000;
  return filteredEvents.value.slice(-limit);
});
const showTime = computed(() => display.value?.showDanmakuTime ?? true);

const legend = computed(() => [
  { tone: 'admin', label: t('liveData.tags.admin'), note: t('danmakuPanel.legend.admin') },
  { tone: 'governor', label: t('liveData.tags.governor'), note: t('danmakuPanel.legend.governor') },
  { tone: 'admiral', label: t('liveData.tags.admiral'), note: t('danmakuPanel.legend.admiral') },
  { tone: 'captain', label: t('liveData.tags.captain'), note: t('danmakuPanel.legend.captain') },
  // `vip` (老爷) is omitted on purpose: the long-link protocol carries no such
  // field, so the badge could never appear and the legend would be misleading.
  { tone: 'plain', label: t('liveData.tags.ordinary'), note: t('danmakuPanel.legend.ordinary') }
]);

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
  if (event.kind === 'superChat') return event.content;
  if (event.kind === 'superChatDeleted') return '';
  if (event.kind === 'captain')
    return `${t(`danmakuPanel.${event.action}`)}${t(`liveData.tags.${event.level}`)}`;
  return t('danmakuPanel.entry');
}

/** A single glyph stands in for the avatar, matching the design's rail. */
function avatarOf(event: LiveEvent): string {
  return [...event.audience.username][0] ?? '?';
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
  <section class="mx-auto flex h-full min-h-[560px] w-full max-w-[1180px] flex-col gap-3">
    <header class="flex flex-wrap items-center gap-2">
      <div class="mr-auto flex flex-wrap items-center gap-1.5">
        <button
          v-for="option in filterOptions"
          :key="option.value"
          type="button"
          class="app-chip"
          :class="{ 'app-chip--active': filter === option.value }"
          :aria-pressed="filter === option.value"
          @click="filter = option.value"
        >
          {{ option.label }}
          <span v-if="option.count !== null" class="tabular-nums opacity-70">{{ option.count }}</span>
        </button>
      </div>
      <el-input
        v-model="search"
        clearable
        class="!w-52"
        :placeholder="t('danmakuPanel.searchPlaceholder')"
        :aria-label="t('danmakuPanel.searchPlaceholder')"
      />
      <button
        type="button"
        class="app-icon-btn"
        :aria-label="feed.isPaused ? t('danmakuPanel.resume') : t('danmakuPanel.pause')"
        @click="feed.setPaused(!feed.isPaused)"
      >
        <el-icon><component :is="feed.isPaused ? VideoPlay : VideoPause" /></el-icon>
      </button>
      <button
        type="button"
        class="app-icon-btn"
        :aria-label="t('danmakuPanel.clear')"
        @click="clearPanel"
      >
        <el-icon><Delete /></el-icon>
      </button>
    </header>

    <el-alert
      v-if="room.snapshot.status === 'reconnecting'"
      type="warning"
      :closable="false"
      :title="t('danmakuPanel.reconnecting', { attempt: room.snapshot.reconnectAttempt })"
      show-icon
    />
    <el-alert
      v-else-if="room.snapshot.status === 'error'"
      type="error"
      :closable="false"
      :title="roomErrorText"
      show-icon
    />

    <div class="flex min-h-0 flex-1 gap-4">
      <div
        ref="stream"
        class="app-card min-h-0 flex-1 overflow-y-auto"
        tabindex="0"
        :aria-label="t('danmakuPanel.eventList')"
      >
        <div
          v-if="shownEvents.length === 0"
          class="app-faint flex h-full min-h-72 items-center justify-center text-sm"
        >
          {{ t('liveData.empty') }}
        </div>
        <article
          v-for="event in shownEvents"
          :key="event.eventId"
          class="flex gap-3 border-b px-3 py-2.5 last:border-b-0"
          :class="
            event.kind === 'superChat'
              ? 'app-row-super-chat'
              : event.kind === 'gift'
                ? 'app-row-gift'
                : ''
          "
          style="border-color: var(--app-border)"
        >
          <span class="app-avatar" aria-hidden="true">{{ avatarOf(event) }}</span>
          <div class="min-w-0 flex-1">
            <div class="flex min-w-0 items-center gap-2">
              <AudienceIdentityView
                :identity="event.audience"
                interactive
                :show-level="display?.showDanmakuLevel ?? true"
                :show-medal="display?.showDanmakuMedal ?? true"
                class="min-w-0"
                @select="selectedUid = event.audience.userUid"
              />
              <time v-if="showTime" class="app-faint ml-auto shrink-0 text-xs tabular-nums">
                {{ formatTimestamp(event.occurredAt) }}
              </time>
            </div>
            <p class="app-text mt-1 mb-0 text-sm break-words">
              <template v-if="event.kind === 'danmu'">{{ event.content }}</template>
              <template v-else-if="event.kind === 'superChat'">
                <span class="app-badge app-badge--captain mr-1.5">
                  {{ formatMoneyMinor(event.valueMinor) }}
                </span>
                {{ event.content }}
              </template>
              <template v-else-if="event.kind === 'gift'">
                {{
                  t('danmakuPanel.giftEvent', {
                    giftName: event.giftName,
                    quantity: event.quantity,
                    value: formatMoneyMinor(event.valueMinor)
                  })
                }}
              </template>
              <template v-else-if="event.kind === 'captain'">
                {{
                  t('danmakuPanel.captainEvent', {
                    action: t(`danmakuPanel.${event.action}`),
                    level: t(`liveData.tags.${event.level}`),
                    value: formatMoneyMinor(event.valueMinor)
                  })
                }}
              </template>
              <template v-else-if="event.kind === 'enter'">{{ t('danmakuPanel.entry') }}</template>
            </p>
          </div>
        </article>
      </div>

      <aside class="app-card hidden w-[240px] shrink-0 self-start p-4 xl:block">
        <h3 class="app-text m-0 text-sm font-semibold">{{ t('danmakuPanel.legendTitle') }}</h3>
        <ul class="m-0 mt-3 list-none p-0">
          <li
            v-for="entry in legend"
            :key="entry.tone"
            class="flex items-center justify-between gap-2 py-1.5"
          >
            <span class="app-badge" :class="`app-badge--${entry.tone}`">{{ entry.label }}</span>
            <span class="app-faint text-xs">{{ entry.note }}</span>
          </li>
        </ul>
      </aside>
    </div>

    <footer class="app-faint flex min-h-9 flex-wrap items-center gap-x-6 gap-y-1 text-xs">
      <span>
        {{ t('danmakuPanel.total') }}
        <strong class="app-text tabular-nums">{{ formatCount(metrics?.danmuCount ?? 0) }}</strong>
      </span>
      <span>
        {{ t('danmakuPanel.rate') }}
        <strong class="app-text tabular-nums">{{ formatDanmuRate(metrics?.danmuRate ?? 0) }}</strong>
      </span>
      <router-link to="/logs" class="app-accent-text hover:underline">
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

<style scoped>
.app-row-gift {
  background: color-mix(in srgb, var(--app-gold) 8%, transparent);
}
.app-row-super-chat {
  background: color-mix(in srgb, var(--app-gold) 16%, transparent);
}
.app-avatar {
  display: inline-flex;
  flex: none;
  align-items: center;
  justify-content: center;
  width: 28px;
  height: 28px;
  border-radius: 999px;
  background: var(--app-panel-2);
  color: var(--app-text-2);
  font-size: 13px;
  font-weight: 600;
}
</style>
