/**
 * ASR 服务
 * 纯 IPC 调用封装，Web Speech API 逻辑已提取到 composables
 */

import { invokeIPC, invokeIPCVoid } from '../utils/ipc.js';
import type { ASRConfig } from '../modules/asr/types.js';
import { DEFAULT_ASR_CONFIG } from '../modules/asr/types.js';

export class ASRService {
  private config: ASRConfig = { ...DEFAULT_ASR_CONFIG };

  updateConfig(config: Partial<ASRConfig>): void {
    this.config = { ...this.config, ...config };
  }

  getConfig(): ASRConfig {
    return { ...this.config };
  }

  async start(whisperConfig: {
    engine: string;
    model: string;
    language: string;
  }): Promise<void> {
    await invokeIPC('asr:start', whisperConfig);
  }

  async processAudio(audioBase64: string, language: string): Promise<{
    text: string;
    confidence: number;
  }> {
    const data = await invokeIPC<{
      success: boolean;
      text: string;
      confidence: number;
      error?: string;
    }>('asr:process', { audio: audioBase64, language });
    return data as { text: string; confidence: number };
  }

  async stop(): Promise<void> {
    await invokeIPCVoid('asr:stop');
  }

  async checkAvailability(): Promise<{ available: boolean; version?: string }> {
    try {
      const result = await window.electronAPI.invoke('asr:checkAvailability');
      return result as { available: boolean; version?: string };
    } catch {
      return { available: false };
    }
  }
}

let asrServiceInstance: ASRService | null = null;

export function getASRService(): ASRService {
  if (!asrServiceInstance) {
    asrServiceInstance = new ASRService();
  }
  return asrServiceInstance;
}
