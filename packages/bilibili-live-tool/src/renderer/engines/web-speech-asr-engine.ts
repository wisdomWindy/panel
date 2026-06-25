/**
 * Web Speech API ASR 引擎实现
 * 使用浏览器内置的 SpeechRecognition API
 */

import type { ASREngine, ASREngineConfig } from './asr-engine.js';
import type { ASRResult } from '../modules/asr/types.js';

export class WebSpeechASREngine implements ASREngine {
  readonly name = 'web-speech';
  private recognition: any = null;
  private resultCallback: ((result: ASRResult) => void) | null = null;
  private errorCallback: ((error: { code: string; message: string }) => void) | null = null;
  private endCallback: (() => void) | null = null;

  async isAvailable(): Promise<boolean> {
    if (typeof window === 'undefined') return false;
    return !!(window as any).SpeechRecognition || !!(window as any).webkitSpeechRecognition;
  }

  async start(config: ASREngineConfig): Promise<void> {
    const SpeechRecognition = (window as any).SpeechRecognition || (window as any).webkitSpeechRecognition;
    if (!SpeechRecognition) {
      throw new Error('Web Speech API 不可用');
    }

    this.recognition = new SpeechRecognition();
    this.recognition.continuous = config.continuous;
    this.recognition.interimResults = config.interimResults;
    this.recognition.maxAlternatives = config.maxAlternatives;
    this.recognition.lang = config.language === 'auto' ? '' : config.language;

    this.recognition.onresult = (event: any) => {
      if (!this.resultCallback) return;
      const result = event.results[event.results.length - 1];
      const asrResult: ASRResult = {
        text: result[0].transcript,
        confidence: result[0].confidence,
        isFinal: result.isFinal,
        timestamp: Date.now(),
        alternatives: Array.from(result).map((alt: any) => ({
          text: alt.transcript,
          confidence: alt.confidence
        }))
      };
      this.resultCallback(asrResult);
    };

    this.recognition.onerror = (event: any) => {
      if (!this.errorCallback) return;
      const errorMessages: Record<string, string> = {
        'no-speech': '未检测到语音',
        'audio-capture': '无法捕获音频',
        'not-allowed': '麦克风权限被拒绝',
        'network': '网络错误',
        'aborted': '识别被中止',
        'language-not-supported': '不支持的语言'
      };
      this.errorCallback({
        code: event.error,
        message: errorMessages[event.error] || `识别错误: ${event.error}`
      });
    };

    this.recognition.onend = () => {
      if (this.endCallback) {
        this.endCallback();
      }
    };

    this.recognition.start();
  }

  async stop(): Promise<void> {
    if (this.recognition) {
      try {
        this.recognition.stop();
      } catch {
        // 忽略已经在运行的错误
      }
    }
  }

  pause(): void {
    this.stop();
  }

  resume(): void {
    if (this.recognition) {
      try {
        this.recognition.start();
      } catch {
        // 忽略错误
      }
    }
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

export function createWebSpeechASREngine(): WebSpeechASREngine {
  return new WebSpeechASREngine();
}
