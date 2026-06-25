/**
 * 引擎工厂
 * 根据配置创建和选择合适的引擎
 */

import type { TTSEngine } from './tts-engine.js';
import type { ASREngine } from './asr-engine.js';
import { createEdgeTTSEngine } from './edge-tts-engine.js';
import { createWebSpeechTTSEngine } from './web-speech-tts-engine.js';
import { createWebSpeechASREngine } from './web-speech-asr-engine.js';
import { createWhisperASREngine } from './whisper-asr-engine.js';

/** TTS 引擎类型 */
export type TTSEngineType = 'edge-tts' | 'web-speech';

/** ASR 引擎类型 */
export type ASREngineType = 'whisper-cpp' | 'web-speech';

/** 创建 TTS 引擎 */
export function createTTSEngine(type: TTSEngineType): TTSEngine {
  switch (type) {
    case 'edge-tts':
      return createEdgeTTSEngine();
    case 'web-speech':
      return createWebSpeechTTSEngine();
    default:
      return createEdgeTTSEngine();
  }
}

/** 创建 ASR 引擎 */
export function createASREngine(type: ASREngineType): ASREngine {
  switch (type) {
    case 'web-speech':
      return createWebSpeechASREngine();
    case 'whisper-cpp':
      return createWhisperASREngine();
    default:
      return createWhisperASREngine();
  }
}
