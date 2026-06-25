/**
 * Web Speech API 语音识别 composable
 * 封装浏览器内置的语音识别功能
 */

import { ref, onUnmounted } from 'vue';
import type { ASRResult } from '../modules/asr/types.js';

export function useWebSpeechRecognition() {
  const isListening = ref(false);
  const isAvailable = ref(false);
  const recognition = ref<any>(null);

  function init(config: {
    continuous: boolean;
    interimResults: boolean;
    maxAlternatives: number;
    language: string;
  }): void {
    if (typeof window === 'undefined') return;

    const SpeechRecognition = (window as any).SpeechRecognition || (window as any).webkitSpeechRecognition;
    if (!SpeechRecognition) return;

    isAvailable.value = true;
    recognition.value = new SpeechRecognition();
    recognition.value.continuous = config.continuous;
    recognition.value.interimResults = config.interimResults;
    recognition.value.maxAlternatives = config.maxAlternatives;
    recognition.value.lang = config.language === 'auto' ? '' : config.language;
  }

  function onResult(callback: (result: ASRResult) => void): void {
    if (!recognition.value) return;

    recognition.value.onresult = (event: any) => {
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
      callback(asrResult);
    };
  }

  function onError(callback: (error: { code: string; message: string }) => void): void {
    if (!recognition.value) return;

    recognition.value.onerror = (event: any) => {
      const errorMessages: Record<string, string> = {
        'no-speech': '未检测到语音',
        'audio-capture': '无法捕获音频',
        'not-allowed': '麦克风权限被拒绝',
        'network': '网络错误',
        'aborted': '识别被中止',
        'language-not-supported': '不支持的语言'
      };
      callback({
        code: event.error,
        message: errorMessages[event.error] || `识别错误: ${event.error}`
      });
    };
  }

  function onEnd(callback: () => void): void {
    if (!recognition.value) return;
    recognition.value.onend = callback;
  }

  function start(): void {
    if (!recognition.value) return;
    recognition.value.start();
    isListening.value = true;
  }

  function stop(): void {
    if (!recognition.value) return;
    try {
      recognition.value.stop();
    } catch {
      // 忽略已经在运行的错误
    }
    isListening.value = false;
  }

  function updateConfig(config: {
    continuous: boolean;
    interimResults: boolean;
    maxAlternatives: number;
    language: string;
  }): void {
    if (!recognition.value) return;
    recognition.value.continuous = config.continuous;
    recognition.value.interimResults = config.interimResults;
    recognition.value.maxAlternatives = config.maxAlternatives;
    recognition.value.lang = config.language === 'auto' ? '' : config.language;
  }

  onUnmounted(() => {
    stop();
  });

  return {
    isListening,
    isAvailable,
    init,
    onResult,
    onError,
    onEnd,
    start,
    stop,
    updateConfig
  };
}
