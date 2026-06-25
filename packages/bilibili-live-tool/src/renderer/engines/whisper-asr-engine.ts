/**
 * Whisper ASR 引擎实现
 * 通过 IPC 调用主进程的 whisper.cpp 服务
 */

import type { ASREngine, ASREngineConfig } from './asr-engine.js';
import type { ASRResult } from '../modules/asr/types.js';
import { invokeIPC, invokeIPCVoid } from '../utils/ipc.js';
import { useAudioRecording } from '../composables/useAudioRecording.js';

export class WhisperASREngine implements ASREngine {
  readonly name = 'whisper-cpp';
  private audioRecording = useAudioRecording();
  private resultCallback: ((result: ASRResult) => void) | null = null;
  private errorCallback: ((error: { code: string; message: string }) => void) | null = null;
  private endCallback: (() => void) | null = null;
  private language = 'zh';

  async isAvailable(): Promise<boolean> {
    try {
      const result = await window.electronAPI.invoke('asr:checkAvailability');
      return (result as { available: boolean }).available;
    } catch {
      return false;
    }
  }

  async start(config: ASREngineConfig): Promise<void> {
    this.language = config.language;

    const hasMic = await this.audioRecording.requestMicrophone();
    if (!hasMic) {
      throw new Error('无法访问麦克风');
    }

    await invokeIPC('asr:start', {
      engine: 'whisper-cpp',
      model: config.model || 'base',
      language: config.language
    });

    this.audioRecording.startRecording(async (blob) => {
      const reader = new FileReader();
      reader.onloadend = async () => {
        const base64 = (reader.result as string).split(',')[1];
        try {
          const data = await invokeIPC<{
            success: boolean;
            text: string;
            confidence: number;
            error?: string;
          }>('asr:process', { audio: base64, language: this.language });

          if (this.resultCallback) {
            this.resultCallback({
              text: (data as { text: string }).text,
              confidence: (data as { confidence: number }).confidence || 0.9,
              isFinal: true,
              timestamp: Date.now()
            });
          }
        } catch (error) {
          if (this.errorCallback) {
            this.errorCallback({
              code: 'process-error',
              message: (error as Error).message
            });
          }
        }
      };
      reader.readAsDataURL(blob);
    });
  }

  async stop(): Promise<void> {
    this.audioRecording.stopRecording();
    this.audioRecording.releaseMicrophone();
    await invokeIPCVoid('asr:stop');
    if (this.endCallback) {
      this.endCallback();
    }
  }

  pause(): void {
    this.audioRecording.pauseRecording();
  }

  resume(): void {
    // Whisper 不支持暂停恢复，需要重新开始
  }

  onResult(callback: (result: ASRResult) => void): void {
    this.resultCallback = callback;
  }

  onError(callback: (error: { code: string; message: string }) => void): void {
    this.errorCallback = callback;
  }

  onEnd(callback: () => void): void {
    this.endCallback = callback;
  }
}

export function createWhisperASREngine(): WhisperASREngine {
  return new WhisperASREngine();
}
