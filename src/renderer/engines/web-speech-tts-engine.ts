/**
 * Web Speech API 引擎实现
 * 使用浏览器内置的 SpeechSynthesis API
 */

import type { TTSEngine, TTSSpeakOptions } from './tts-engine.js';

export class WebSpeechTTSEngine implements TTSEngine {
  readonly name = 'web-speech';

  isAvailable(): boolean {
    return typeof window !== 'undefined' && !!window.speechSynthesis;
  }

  async speak(text: string, options: TTSSpeakOptions): Promise<void> {
    if (!this.isAvailable()) {
      throw new Error('Web Speech API 不可用');
    }

    return new Promise((resolve, reject) => {
      const utterance = new SpeechSynthesisUtterance(text);
      utterance.rate = options.rate ?? 1;
      utterance.pitch = options.pitch ?? 1;
      utterance.volume = options.volume ?? 1;
      utterance.lang = options.lang ?? 'zh-CN';

      if (options.voice) {
        const voices = window.speechSynthesis.getVoices();
        const selectedVoice = voices.find(v => v.name === options.voice);
        if (selectedVoice) {
          utterance.voice = selectedVoice;
        }
      }

      utterance.onend = () => resolve();
      utterance.onerror = (event) => reject(event);

      window.speechSynthesis.speak(utterance);
    });
  }

  async stop(): Promise<void> {
    if (this.isAvailable()) {
      window.speechSynthesis.cancel();
    }
  }

  pause(): void {
    if (this.isAvailable()) {
      window.speechSynthesis.pause();
    }
  }

  resume(): void {
    if (this.isAvailable()) {
      window.speechSynthesis.resume();
    }
  }
}

export function createWebSpeechTTSEngine(): WebSpeechTTSEngine {
  return new WebSpeechTTSEngine();
}
