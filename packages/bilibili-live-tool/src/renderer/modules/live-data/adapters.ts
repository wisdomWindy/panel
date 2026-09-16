import type {
  AudienceIdentity,
  LiveEvent,
  RankingSnapshot,
  RoomSnapshot,
  StatSnapshot
} from '@/platform/desktop';

function copyAudience(value: AudienceIdentity): AudienceIdentity {
  return {
    ...value,
    medal: value.medal ? { ...value.medal } : null,
    tags: [...value.tags]
  };
}

export function adaptLiveEvent(value: LiveEvent): LiveEvent {
  return { ...value, audience: copyAudience(value.audience) };
}

export function adaptRoomSnapshot(value: RoomSnapshot): RoomSnapshot {
  return { ...value };
}

export function adaptStatSnapshot(value: StatSnapshot): StatSnapshot {
  return {
    ...value,
    range: { ...value.range },
    metrics: { ...value.metrics },
    trendPoints: value.trendPoints.map(point => ({ ...point }))
  };
}

export function adaptRankingSnapshot(value: RankingSnapshot): RankingSnapshot {
  return {
    ...value,
    range: { ...value.range },
    gift: value.gift.map(row => ({ ...row })),
    danmu: value.danmu.map(row => ({ ...row })),
    captain: value.captain.map(row => ({ ...row }))
  };
}
