<script setup lang="ts">
import { computed, onMounted } from 'vue';
import { useI18n } from 'vue-i18n';
import type { LiveEvent } from '@/platform/desktop';
import { formatCount, formatMoneyMinor } from '@/modules/live-data/formatters';
import { useLiveAnalyticsStore } from '@/stores/live-data/analytics';
import { useLiveFeedStore } from '@/stores/live-data/feed';
import { useLiveRoomStore } from '@/stores/live-data/room';

const { t } = useI18n();
const room = useLiveRoomStore();
const feed = useLiveFeedStore();
const analytics = useLiveAnalyticsStore();

const metrics = computed(() => analytics.statistics?.metrics ?? null);
const isConnected = computed(() => room.snapshot.status === 'connected');
/**
 * The long connection deliberately outlives the end of a stream, so the badge
 * must never read the connection status: it reads the broadcast state the
 * platform reports.
 */
const liveBadge = computed(() => {
  if (room.snapshot.isLive === true) {
    return { label: t('liveRoom.live'), tone: 'app-badge--admin' };
  }
  if (room.snapshot.isLive === false) {
    return { label: t('liveRoom.offline'), tone: 'app-badge--plain' };
  }
  return { label: t('liveRoom.detecting'), tone: 'app-badge--plain' };
});
const roomTitle = computed(() => {
  const name = room.snapshot.roomName;
  if (name) return name;
  if (isConnected.value && room.snapshot.roomId) {
    return t('liveRoom.untitledConnected', { roomId: room.snapshot.roomId });
  }
  return t('liveRoom.untitled');
});

const metricCards = computed(() => [
  {
    id: 'danmu',
    label: t('liveRoom.metrics.danmu'),
    value: formatCount(metrics.value?.danmuCount ?? 0),
    hint: t('liveRoom.metrics.danmuHint')
  },
  {
    id: 'gift',
    label: t('liveRoom.metrics.gift'),
    value: formatMoneyMinor(metrics.value?.giftValueMinor ?? 0),
    hint: t('liveRoom.metrics.giftHint')
  },
  {
    id: 'captain',
    label: t('liveRoom.metrics.captain'),
    value: formatCount(metrics.value?.captainCount ?? 0),
    hint: t('liveRoom.metrics.captainHint')
  },
  {
    id: 'online',
    label: t('liveRoom.metrics.online'),
    value: formatCount(metrics.value?.onlineCount ?? room.snapshot.onlineCount ?? 0),
    hint: t('liveRoom.metrics.onlineHint')
  }
]);

/** Newest first, mirroring the recent-event rail in the design. */
const recentEvents = computed(() => [...feed.events].reverse().slice(0, 8));

type BadgeTone = 'admin' | 'governor' | 'admiral' | 'captain' | 'vip' | 'plain';

function badgeOf(event: LiveEvent): { tone: BadgeTone; labelKey: string } {
  const tags = event.audience.tags;
  if (tags.includes('admin')) return { tone: 'admin', labelKey: 'liveData.tags.admin' };
  if (tags.includes('governor')) return { tone: 'governor', labelKey: 'liveData.tags.governor' };
  if (tags.includes('admiral')) return { tone: 'admiral', labelKey: 'liveData.tags.admiral' };
  if (tags.includes('captain')) return { tone: 'captain', labelKey: 'liveData.tags.captain' };
  if (tags.includes('vip')) return { tone: 'vip', labelKey: 'liveData.tags.vip' };
  return { tone: 'plain', labelKey: 'liveData.tags.ordinary' };
}

function summaryOf(event: LiveEvent): string {
  switch (event.kind) {
    case 'danmu':
      return event.content;
    case 'gift':
      return t('liveRoom.event.gift', { name: event.giftName, count: event.quantity });
    case 'captain':
      return t('liveRoom.event.captain', {
        action: t(`danmakuPanel.${event.action}`),
        level: t(`liveData.tags.${event.level}`)
      });
    case 'enter':
      return t('liveRoom.event.enter');
    default:
      return '';
  }
}

function clockOf(timestamp: number): string {
  const date = new Date(timestamp);
  return `${String(date.getHours()).padStart(2, '0')}:${String(date.getMinutes()).padStart(2, '0')}:${String(
    date.getSeconds()
  ).padStart(2, '0')}`;
}

onMounted(() => {
  void analytics.queryStatistics({ kind: 'liveSession' }).catch(() => undefined);
});
</script>

<template>
  <section class="mx-auto flex w-full max-w-[1180px] flex-col gap-4">
    <div class="app-card p-4">
      <div class="flex flex-wrap items-start justify-between gap-3">
        <div class="min-w-0">
          <h2 class="app-text m-0 truncate text-lg font-semibold">
            {{ roomTitle }}
          </h2>
          <p class="app-muted mt-1.5 mb-0 text-sm">
            {{ t('liveRoom.anchor') }}
            <span class="app-text">{{ room.snapshot.roomName ?? t('liveRoom.anchorUnknown') }}</span>
            <span class="mx-2 app-faint">·</span>
            {{ t('liveRoom.roomId') }}
            <span class="app-text tabular-nums">{{ room.snapshot.roomId ?? t('liveRoom.notConnected') }}</span>
          </p>
        </div>
        <span class="app-badge" :class="liveBadge.tone">
          {{ liveBadge.label }}
        </span>
      </div>
      <p class="app-faint mt-2 mb-0 text-xs">
        {{ t('liveRoom.readOnly') }}
      </p>
    </div>

    <div class="grid grid-cols-2 gap-4 xl:grid-cols-4">
      <article v-for="card in metricCards" :key="card.id" class="app-metric">
        <p class="app-metric-label m-0">{{ card.label }}</p>
        <p class="app-metric-value mt-1.5 mb-0">{{ card.value }}</p>
        <p class="app-faint mt-1 mb-0 text-xs">{{ card.hint }}</p>
      </article>
    </div>

    <div class="app-card flex min-h-0 flex-col p-4">
      <h3 class="app-text m-0 text-base font-semibold">{{ t('liveRoom.recent') }}</h3>
      <p v-if="recentEvents.length === 0" class="app-faint mt-6 mb-0 text-sm">
        {{ t('liveRoom.empty') }}
      </p>
      <ul v-else class="m-0 mt-3 list-none divide-y p-0" style="--tw-divide-opacity: 1">
        <li
          v-for="event in recentEvents"
          :key="event.eventId"
          class="flex items-start gap-3 border-b py-2.5 last:border-b-0"
          style="border-color: var(--app-border)"
        >
          <span class="app-badge shrink-0" :class="`app-badge--${badgeOf(event).tone}`">
            {{ t(badgeOf(event).labelKey) }}
          </span>
          <span class="app-text shrink-0 text-sm font-medium">{{ event.audience.username }}</span>
          <span class="app-faint shrink-0 text-xs tabular-nums">{{ clockOf(event.occurredAt) }}</span>
          <span class="app-muted min-w-0 flex-1 truncate text-sm">{{ summaryOf(event) }}</span>
        </li>
      </ul>
    </div>
  </section>
</template>
