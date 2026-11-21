import { computed, ref } from 'vue';
import { defineStore } from 'pinia';
import type { LiveEvent } from '@/platform/desktop';
import { adaptLiveEvent } from '@/modules/live-data/adapters';

const MAX_EVENTS = 1000;

export const useLiveFeedStore = defineStore('live-feed', () => {
  const events = ref<LiveEvent[]>([]);
  const isPaused = ref(false);

  const danmu = computed(() => events.value.filter(event => event.kind === 'danmu'));
  const gifts = computed(() => events.value.filter(event => event.kind === 'gift'));

  function accept(value: LiveEvent): void {
    // Session boundaries are not feed rows: they reset or seal the session
    // instead of appearing as a danmaku line (PRD 9.1.1).
    if (value.kind === 'sessionStarted' || value.kind === 'sessionEnded') return;
    const next = [...events.value, adaptLiveEvent(value)];
    events.value = next.length > MAX_EVENTS ? next.slice(-MAX_EVENTS) : next;
  }

  function clear(): void {
    events.value = [];
  }

  function clearKind(kind: LiveEvent['kind']): void {
    events.value = events.value.filter(event => event.kind !== kind);
  }

  function removeAudience(userUid: string): void {
    events.value = events.value.filter(event => event.audience.userUid !== userUid);
  }

  function setPaused(value: boolean): void {
    isPaused.value = value;
  }

  return { events, danmu, gifts, isPaused, accept, clear, clearKind, removeAudience, setPaused };
});
