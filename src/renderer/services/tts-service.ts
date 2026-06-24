/**
 * TTS 服务
 * 纯 IPC 调用和配置管理，队列逻辑已提取到 composables
 */

import { invokeIPC, invokeIPCVoid } from '../utils/ipc.js';
import type { TTSConfig } from '../modules/tts/types.js';
import { DEFAULT_TTS_CONFIG } from '../modules/tts/types.js';

export class TTSService {
  private config: TTSConfig = { ...DEFAULT_TTS_CONFIG };

  updateConfig(config: Partial<TTSConfig>): void {
    this.config = { ...this.config, ...config };
  }

  getConfig(): TTSConfig {
    return { ...this.config };
  }

  async speak(text: string): Promise<void> {
    if (this.config.engine === 'edge-tts') {
      await this.speakWithEdgeTTS(text);
    }
  }

  async stop(): Promise<void> {
    if (this.config.engine === 'edge-tts') {
      await invokeIPCVoid('tts:stop');
    }
  }

  private async speakWithEdgeTTS(text: string): Promise<void> {
    try {
      await invokeIPC('tts:speak', {
        text,
        voice: this.config.edgeTTSVoice,
        rate: this.config.speed,
        volume: this.config.volume
      });
    } catch (error) {
      console.error('Edge TTS error:', error);
      throw error;
    }
  }
}

let ttsServiceInstance: TTSService | null = null;

export function getTTSService(): TTSService {
  if (!ttsServiceInstance) {
    ttsServiceInstance = new TTSService();
  }
  return ttsServiceInstance;
}
