import { computed, ref } from 'vue';
import { defineStore } from 'pinia';
import {
  invokeDesktop,
  normalizeDesktopError,
  type DesktopError,
  type RoomId,
  type RoomSnapshot
} from '@/platform/desktop';
import { adaptRoomSnapshot } from '@/modules/live-data/adapters';

const emptyRoom = (): RoomSnapshot => ({
  status: 'disconnected',
  roomId: null,
  roomName: null,
  onlineCount: null,
  reconnectAttempt: 0,
  lastErrorCode: null
});

export const useLiveRoomStore = defineStore('live-room', () => {
  const snapshot = ref<RoomSnapshot>(emptyRoom());
  const pending = ref(false);
  const error = ref<DesktopError | null>(null);
  const generation = ref(0);

  const isConnected = computed(() => snapshot.value.status === 'connected');

  function replaceSnapshot(value: RoomSnapshot): void {
    snapshot.value = adaptRoomSnapshot(value);
  }

  function applyStatus(value: {
    status: RoomSnapshot['status'];
    roomId: RoomId | null;
    attempt: number;
    errorCode: string | null;
  }): void {
    snapshot.value = {
      ...snapshot.value,
      status: value.status,
      roomId: value.roomId,
      reconnectAttempt: value.attempt,
      lastErrorCode: value.errorCode,
      ...(value.roomId === snapshot.value.roomId ? {} : { roomName: null, onlineCount: null })
    };
  }

  async function connect(roomId: RoomId): Promise<void> {
    if (pending.value) return;
    const requestGeneration = ++generation.value;
    pending.value = true;
    error.value = null;
    try {
      const result = await invokeDesktop('room_connect', { roomId });
      if (requestGeneration === generation.value) replaceSnapshot(result);
    } catch (value) {
      if (requestGeneration === generation.value) error.value = normalizeDesktopError(value);
      throw value;
    } finally {
      if (requestGeneration === generation.value) pending.value = false;
    }
  }

  async function disconnect(): Promise<void> {
    const requestGeneration = ++generation.value;
    pending.value = true;
    error.value = null;
    try {
      const result = await invokeDesktop('room_disconnect', undefined);
      if (requestGeneration === generation.value) replaceSnapshot(result);
    } catch (value) {
      if (requestGeneration === generation.value) error.value = normalizeDesktopError(value);
      throw value;
    } finally {
      if (requestGeneration === generation.value) pending.value = false;
    }
  }

  return {
    snapshot,
    pending,
    error,
    generation,
    isConnected,
    replaceSnapshot,
    applyStatus,
    connect,
    disconnect
  };
});
