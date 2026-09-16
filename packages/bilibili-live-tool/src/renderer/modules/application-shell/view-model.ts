import type { LocaleCode, RoomSnapshot, SystemMetrics } from '@/platform/desktop';

export type RoomInputResult =
  | { readonly ok: true; readonly value: string }
  | { readonly ok: false; readonly code: 'roomRequired' | 'roomInvalid' };

export interface StatusBarItem {
  readonly id: 'connection' | 'room' | 'danmakuRate' | 'online' | 'recording' | 'memory' | 'cpu';
  readonly value: string;
  readonly tone: 'neutral' | 'success' | 'warning' | 'danger';
}

export function normalizeRoomInput(value: string): RoomInputResult {
  const normalized = value.trim();
  if (normalized.length === 0) return { ok: false, code: 'roomRequired' };
  if (!/^\d{1,20}$/.test(normalized)) return { ok: false, code: 'roomInvalid' };
  return { ok: true, value: normalized };
}

function formatMemory(value: number | null): string {
  if (value === null) return '--';
  const mebibytes = value / 1_048_576;
  return `${Number.isInteger(mebibytes) ? mebibytes.toFixed(0) : mebibytes.toFixed(1)} MiB`;
}

function connectionValue(status: RoomSnapshot['status'], locale: LocaleCode): string {
  const labels = locale === 'en-US'
    ? { disconnected: 'disconnected', connecting: 'connecting', connected: 'connected', reconnecting: 'reconnecting', error: 'error' }
    : { disconnected: '已断开', connecting: '连接中', connected: '已连接', reconnecting: '重连中', error: '连接错误' };
  return labels[status];
}

function recordingValue(state: string, locale: LocaleCode): string {
  const labels: Record<string, readonly [string, string]> = {
    idle: ['未录制', 'idle'], starting: ['启动中', 'starting'], recording: ['录制中', 'recording'],
    paused: ['暂停', 'paused'], stopping: ['停止中', 'stopping'], finalizing: ['处理中', 'finalizing'], error: ['录制异常', 'error']
  };
  const pair = labels[state] ?? ['不可用', 'unavailable'];
  return pair[locale === 'en-US' ? 1 : 0];
}

export function buildStatusBarView(input: {
  room: RoomSnapshot;
  acceptedDanmakuCount: number;
  recordingState: string;
  metrics: SystemMetrics | null;
  locale: LocaleCode;
}): readonly StatusBarItem[] {
  const status = input.room.status;
  const number = new Intl.NumberFormat(input.locale);
  const tone = status === 'connected' ? 'success' : status === 'reconnecting' || status === 'connecting' ? 'warning' : 'danger';
  return Object.freeze([
    { id: 'connection', value: connectionValue(status, input.locale), tone },
    { id: 'room', value: input.room.roomId ?? '--', tone: 'neutral' },
    { id: 'danmakuRate', value: `${Math.max(0, Math.trunc(input.acceptedDanmakuCount))} ${input.locale === 'en-US' ? 'msg/s' : '条/秒'}`, tone: 'neutral' },
    { id: 'online', value: input.room.onlineCount === null ? '--' : number.format(input.room.onlineCount), tone: 'neutral' },
    { id: 'recording', value: recordingValue(input.recordingState, input.locale), tone: input.recordingState === 'recording' ? 'success' : input.recordingState === 'error' ? 'danger' : 'neutral' },
    { id: 'memory', value: formatMemory(input.metrics?.memoryBytes ?? null), tone: 'neutral' },
    { id: 'cpu', value: input.metrics?.cpuPercent === null || input.metrics?.cpuPercent === undefined ? '--' : `${input.metrics.cpuPercent.toFixed(1)}%`, tone: 'neutral' }
  ]);
}
