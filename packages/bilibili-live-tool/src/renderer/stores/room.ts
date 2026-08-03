/**
 * 直播间状态管理
 */

import { defineStore } from 'pinia';
import { ref, computed } from 'vue';
import { ConnectionStatus, RoomInfo } from '../modules/danmaku/types';

export const useRoomStore = defineStore('room', () => {
  // State
  const status = ref<ConnectionStatus>('disconnected');
  const roomInfo = ref<RoomInfo | null>(null);
  const code = ref<string>('');       // 主播身份码
  const appId = ref<number>(0);       // 应用 ID
  const uid = ref<number>(0);
  const onlineCount = ref<number>(0);

  // Getters
  const isConnected = computed(() => status.value === 'connected');
  const isConnecting = computed(() => status.value === 'connecting');
  const isDisconnected = computed(() => status.value === 'disconnected');
  const hasError = computed(() => status.value === 'error');
  const currentRoomId = computed(() => roomInfo.value?.roomId || '');

  // Actions
  function setStatus(newStatus: ConnectionStatus) {
    status.value = newStatus;
  }

  function setRoomInfo(info: RoomInfo | null) {
    roomInfo.value = info;
    if (info) {
      onlineCount.value = info.onlineCount;
    }
  }

  function setUid(newUid: number) {
    uid.value = newUid;
  }

  function setOnlineCount(count: number) {
    onlineCount.value = count;
  }

  function setCode(newCode: string) {
    code.value = newCode;
  }

  function setAppId(newAppId: number) {
    appId.value = newAppId;
  }

  function reset() {
    status.value = 'disconnected';
    roomInfo.value = null;
    onlineCount.value = 0;
  }

  function exportState() {
    return {
      status: status.value,
      roomInfo: roomInfo.value,
      code: code.value,
      appId: appId.value,
      uid: uid.value,
      onlineCount: onlineCount.value
    };
  }

  function importState(state: ReturnType<typeof exportState>) {
    status.value = state.status;
    roomInfo.value = state.roomInfo;
    code.value = state.code;
    appId.value = state.appId;
    uid.value = state.uid;
    onlineCount.value = state.onlineCount;
  }

  return {
    // State
    status,
    roomInfo,
    code,
    appId,
    uid,
    onlineCount,

    // Getters
    isConnected,
    isConnecting,
    isDisconnected,
    hasError,
    currentRoomId,

    // Actions
    setStatus,
    setRoomInfo,
    setUid,
    setOnlineCount,
    setCode,
    setAppId,
    reset,
    exportState,
    importState
  };
});
