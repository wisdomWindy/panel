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
  lastErrorCode: null,
  isLive: null
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
    roomName: string | null;
    onlineCount: number | null;
    isLive: boolean | null;
  }): void {
    // The event carries the whole room state: the connection completes
    // asynchronously, so this is the only place the page learns the anchor
    // name and the broadcast state.
    snapshot.value = {
      status: value.status,
      roomId: value.roomId,
      roomName: value.roomName,
      onlineCount: value.onlineCount,
      reconnectAttempt: value.attempt,
      lastErrorCode: value.errorCode,
      isLive: value.isLive
    };
  }

  async function connect(identityCode: string): Promise<void> {
    if (pending.value) return;
    const requestGeneration = ++generation.value;
    pending.value = true;
    error.value = null;
    try {
      const result = await invokeDesktop('room_connect', { identityCode });
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
