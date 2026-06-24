/**
 * 音频录制 composable
 * 封装 MediaRecorder 相关逻辑
 */

import { ref, onUnmounted } from 'vue';

export function useAudioRecording() {
  const isRecording = ref(false);
  const mediaStream = ref<MediaStream | null>(null);
  const mediaRecorder = ref<MediaRecorder | null>(null);
  const audioChunks = ref<Blob[]>([]);

  async function requestMicrophone(): Promise<boolean> {
    try {
      mediaStream.value = await navigator.mediaDevices.getUserMedia({ audio: true });
      return true;
    } catch {
      return false;
    }
  }

  function startRecording(onData: (blob: Blob) => void): void {
    if (!mediaStream.value) return;

    audioChunks.value = [];
    mediaRecorder.value = new MediaRecorder(mediaStream.value);

    mediaRecorder.value.ondataavailable = (event) => {
      if (event.data.size > 0) {
        audioChunks.value.push(event.data);
      }
    };

    mediaRecorder.value.onstop = () => {
      if (audioChunks.value.length === 0) return;
      const audioBlob = new Blob(audioChunks.value, { type: 'audio/webm' });
      onData(audioBlob);
    };

    mediaRecorder.value.start(1000);
    isRecording.value = true;
  }

  function stopRecording(): void {
    if (mediaRecorder.value && mediaRecorder.value.state !== 'inactive') {
      mediaRecorder.value.stop();
    }
    isRecording.value = false;
  }

  function pauseRecording(): void {
    if (mediaRecorder.value && mediaRecorder.value.state === 'recording') {
      mediaRecorder.value.pause();
      isRecording.value = false;
    }
  }

  function releaseMicrophone(): void {
    if (mediaStream.value) {
      mediaStream.value.getTracks().forEach(track => track.stop());
      mediaStream.value = null;
    }
  }

  function destroy(): void {
    stopRecording();
    releaseMicrophone();
  }

  onUnmounted(destroy);

  return {
    isRecording,
    mediaStream,
    requestMicrophone,
    startRecording,
    stopRecording,
    pauseRecording,
    releaseMicrophone,
    destroy
  };
}
