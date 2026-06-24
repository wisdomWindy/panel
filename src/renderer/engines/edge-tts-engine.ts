/**
 * Edge TTS 引擎实现
 * 通过 IPC 调用主进程的 edge-tts 服务
 */

import type { TTSEngine, TTSSpeakOptions } from './tts-engine.js';
import { invokeIPC, invokeIPCVoid } from '../utils/ipc.js';

export class EdgeTTSEngine implements TTSEngine {
  readonly name = 'edge-tts';

  isAvailable(): boolean {
    return true; // Edge TTS 通过 IPC 调用，始终可用
  }

  async speak(text: string, options: TTSSpeakOptions): Promise<void> {
    await invokeIPC('tts:speak', {
      text,
      voice: options.voice,
      rate: options.rate,
      volume: options.volume
    });
  }

  async stop(): Promise<void> {
    await invokeIPCVoid('tts:stop');
  }

  pause(): void {
    // Edge TTS 不支持暂停，需要停止后重新开始
  }

  resume(): void {
    // Edge TTS 不支持恢复
  }
}

export function createEdgeTTSEngine(): EdgeTTSEngine {
  return new EdgeTTSEngine();
}
