/**
 * TTS 服务
 * 使用 Strategy 模式管理不同的 TTS 引擎
 */

import type { TTSEngine, TTSSpeakOptions } from '../engines/tts-engine.js';
import type { TTSConfig } from '../modules/tts/types.js';
import { DEFAULT_TTS_CONFIG } from '../modules/tts/types.js';
import { createTTSEngine, type TTSEngineType } from '../engines/engine-factory.js';

export class TTSService {
  private config: TTSConfig = { ...DEFAULT_TTS_CONFIG };
  private engine: TTSEngine;

  constructor() {
    this.engine = createTTSEngine(this.config.engine as TTSEngineType);
  }

  updateConfig(config: Partial<TTSConfig>): void {
    const oldEngine = this.config.engine;
    this.config = { ...this.config, ...config };

    // 引擎类型变化时切换引擎
    if (config.engine && config.engine !== oldEngine) {
      this.engine = createTTSEngine(config.engine as TTSEngineType);
    }
  }

  getConfig(): TTSConfig {
    return { ...this.config };
  }

  getEngine(): TTSEngine {
    return this.engine;
  }

  async speak(text: string): Promise<void> {
    const options: TTSSpeakOptions = {
      voice: this.config.edgeTTSVoice,
      rate: this.config.speed,
      pitch: this.config.pitch,
      volume: this.config.volume,
      lang: 'zh-CN'
    };
    await this.engine.speak(text, options);
  }

  async stop(): Promise<void> {
    await this.engine.stop();
  }

  pause(): void {
    this.engine.pause();
  }

  resume(): void {
    this.engine.resume();
  }
}

let ttsServiceInstance: TTSService | null = null;

export function getTTSService(): TTSService {
  if (!ttsServiceInstance) {
    ttsServiceInstance = new TTSService();
  }
  return ttsServiceInstance;
}
