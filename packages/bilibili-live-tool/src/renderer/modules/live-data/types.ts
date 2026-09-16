import type { LiveEvent, RankingSnapshot, RoomSnapshot, StatSnapshot } from '@/platform/desktop';

export type LiveFeedEvent = LiveEvent;

export interface LiveDataSnapshots {
  room: RoomSnapshot;
  statistics: StatSnapshot | null;
  rankings: RankingSnapshot | null;
}
