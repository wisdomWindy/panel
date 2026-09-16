<script setup lang="ts">
import { computed, nextTick, ref, watch } from 'vue';
import { Delete, VideoPause, VideoPlay } from '@element-plus/icons-vue';
import type { LiveEvent, UserUid } from '@/platform/desktop';
import { formatCount, formatDanmuRate, formatMoneyMinor, formatTimestamp } from '@/modules/live-data/formatters';
import { useLiveAnalyticsStore } from '@/stores/live-data/analytics';
import { useLiveFeedStore } from '@/stores/live-data/feed';
import { useLiveRoomStore } from '@/stores/live-data/room';
import AudienceDetailDrawer from '@/components/live-data/AudienceDetailDrawer.vue';
import AudienceIdentityView from '@/components/live-data/AudienceIdentityView.vue';

type Filter = 'all' | 'captain' | 'gift' | 'highlight';

const feed = useLiveFeedStore();
const room = useLiveRoomStore();
const analytics = useLiveAnalyticsStore();
const filter = ref<Filter>('all');
const search = ref('');
const stream = ref<HTMLElement | null>(null);
const selectedUid = ref<UserUid | null>(null);

const filteredEvents = computed(() => {
  const query = search.value.trim().toLocaleLowerCase();
  return feed.events.filter((event) => {
    const captain = event.kind === 'captain' || event.audience.tags.some((tag) => ['governor', 'admiral', 'captain'].includes(tag));
    const highlighted = captain || event.audience.tags.includes('admin') || event.audience.tags.includes('vip');
    if (filter.value === 'captain' && !captain) return false;
    if (filter.value === 'gift' && event.kind !== 'gift') return false;
    if (filter.value === 'highlight' && !highlighted) return false;
    if (!query) return true;
    return event.audience.username.toLocaleLowerCase().includes(query) || eventText(event).toLocaleLowerCase().includes(query);
  });
});

const metrics = computed(() => analytics.statistics?.metrics ?? null);

watch(() => feed.events.length, async () => {
  if (feed.isPaused) return;
  await nextTick();
  if (stream.value) stream.value.scrollTop = stream.value.scrollHeight;
});

function eventText(event: LiveEvent): string {
  if (event.kind === 'danmu') return event.content;
  if (event.kind === 'gift') return `${event.giftName} x${event.quantity}`;
  if (event.kind === 'captain') return `${event.action === 'purchase' ? '首开' : '续费'}${event.level === 'governor' ? '总督' : event.level === 'admiral' ? '提督' : '舰长'}`;
  return '进入直播间';
}

function clearPanel(): void {
  feed.clear();
  ElMessage.success('已清屏，日志仍保留');
}

function handleBlacklisted(uid: UserUid): void {
  feed.removeAudience(uid);
}
</script>

<template>
  <section class="mx-auto flex h-full min-h-[560px] max-w-7xl flex-col bg-white">
    <header class="flex min-h-14 flex-wrap items-center gap-3 border-b border-gray-200 py-2">
      <h2 class="mr-auto m-0 text-lg font-semibold text-gray-900">
        弹幕面板
      </h2>
      <el-segmented
        v-model="filter"
        :options="[
          { label: '全部', value: 'all' },
          { label: '仅舰长', value: 'captain' },
          { label: '仅礼物', value: 'gift' },
          { label: '仅高亮', value: 'highlight' }
        ]"
      />
      <el-input
        v-model="search"
        clearable
        class="!w-56"
        placeholder="搜索用户名或内容"
        aria-label="搜索用户名或内容"
      />
      <el-button :icon="feed.isPaused ? VideoPlay : VideoPause" @click="feed.setPaused(!feed.isPaused)">
        {{ feed.isPaused ? '继续滚动' : '暂停滚动' }}
      </el-button>
      <el-button :icon="Delete" @click="clearPanel">
        清屏
      </el-button>
    </header>

    <el-alert
      v-if="room.snapshot.status === 'reconnecting'"
      class="mt-3"
      type="warning"
      :closable="false"
      :title="`重连中 ${room.snapshot.reconnectAttempt}/10`"
      show-icon
    />
    <el-alert
      v-else-if="room.snapshot.status === 'error'"
      class="mt-3"
      type="error"
      :closable="false"
      title="重连失败，请手动重试"
      show-icon
    />

    <div
      ref="stream"
      class="min-h-0 flex-1 overflow-y-auto"
      tabindex="0"
      aria-label="直播事件列表"
    >
      <div v-if="filteredEvents.length === 0" class="flex h-full min-h-72 items-center justify-center text-sm text-gray-400">
        暂无数据
      </div>
      <article v-for="event in filteredEvents" :key="event.eventId" class="grid min-h-16 grid-cols-[minmax(260px,0.9fr)_minmax(280px,1.5fr)_88px] items-center gap-4 border-b border-gray-100 px-3 py-2 text-sm max-lg:grid-cols-[1fr_80px]">
        <AudienceIdentityView
          :identity="event.audience"
          interactive
          class="max-lg:col-span-1"
          @select="selectedUid = event.audience.userUid"
        />
        <div class="min-w-0 text-gray-800 max-lg:col-span-2 max-lg:row-start-2">
          <span v-if="event.kind === 'danmu'" class="break-words">{{ event.content }}</span>
          <span v-else-if="event.kind === 'gift'">赠送 {{ event.giftName }} x{{ event.quantity }} · {{ formatMoneyMinor(event.valueMinor) }}</span>
          <span v-else-if="event.kind === 'captain'">{{ event.action === 'purchase' ? '首开' : '续费' }} · {{ event.level === 'governor' ? '总督' : event.level === 'admiral' ? '提督' : '舰长' }} · {{ formatMoneyMinor(event.valueMinor) }}</span>
          <span v-else>进入直播间</span>
        </div>
        <time class="justify-self-end tabular-nums text-gray-500">{{ formatTimestamp(event.occurredAt) }}</time>
      </article>
    </div>

    <footer class="flex min-h-11 flex-wrap items-center gap-x-6 gap-y-1 border-t border-gray-200 px-3 text-sm text-gray-600">
      <span>弹幕总量 <strong class="tabular-nums text-gray-900">{{ formatCount(metrics?.danmuCount ?? 0) }}</strong></span>
      <span>弹幕速率 <strong class="tabular-nums text-gray-900">{{ formatDanmuRate(metrics?.danmuRate ?? 0) }}</strong></span>
      <router-link to="/logs" class="text-pink-600 hover:underline">
        已过滤 {{ formatCount(metrics?.filteredCount ?? 0) }} 条
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
