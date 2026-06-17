/**
 * ASR 服务
 * 支持 whisper.cpp 和 Web Speech API 两种引擎
 * whisper.cpp 提供高质量的本地语音识别
 */

import type {
  ASRConfig,
  ASRResult,
  ASRStatus,
  ASRError
} from '../modules/asr/types.js';
import { DEFAULT_ASR_CONFIG } from '../modules/asr/types.js';

export class ASRService {
  private config: ASRConfig = { ...DEFAULT_ASR_CONFIG };
  private status: ASRStatus = 'idle';
  private mediaStream: MediaStream | null = null;
  private mediaRecorder: MediaRecorder | null = null;
  private audioChunks: Blob[] = [];
  private statusListeners: Array<(status: ASRStatus) => void> = [];
  private resultListeners: Array<(result: ASRResult) => void> = [];
  private errorListeners: Array<(error: ASRError) => void> = [];
  private recognition: any = null;  // Web Speech API Recognition

  constructor() {
    this.initWebSpeechAPI();
  }

  /**
   * 初始化 Web Speech API
   */
  private initWebSpeechAPI(): void {
    if (typeof window === 'undefined') return;

    const SpeechRecognition = (window as any).SpeechRecognition || (window as any).webkitSpeechRecognition;
    if (SpeechRecognition) {
      this.recognition = new SpeechRecognition();
      this.recognition.continuous = this.config.continuous;
      this.recognition.interimResults = this.config.interimResults;
      this.recognition.maxAlternatives = this.config.maxAlternatives;
      this.recognition.lang = this.config.language === 'auto' ? '' : this.config.language;

      this.recognition.onresult = (event: any) => {
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
        this.resultListeners.forEach(listener => listener(asrResult));
      };

      this.recognition.onerror = (event: any) => {
        const error: ASRError = {
          code: event.error,
          message: this.getErrorMessage(event.error),
          details: event.message
        };
        this.errorListeners.forEach(listener => listener(error));
        this.setStatus('error');
      };

      this.recognition.onend = () => {
        if (this.status === 'listening' && this.config.continuous) {
          // 连续模式下自动重新开始
          try {
            this.recognition.start();
          } catch (e) {
            // 忽略已经在运行的错误
          }
        } else {
          this.setStatus('idle');
        }
      };
    }
  }

  /**
   * 获取错误信息
   */
  private getErrorMessage(error: string): string {
    const messages: Record<string, string> = {
      'no-speech': '未检测到语音',
      'audio-capture': '无法捕获音频',
      'not-allowed': '麦克风权限被拒绝',
      'network': '网络错误',
      'aborted': '识别被中止',
      'language-not-supported': '不支持的语言'
    };
    return messages[error] || `识别错误: ${error}`;
  }

  /**
   * 更新配置
   */
  updateConfig(config: Partial<ASRConfig>): void {
    this.config = { ...this.config, ...config };

    // 更新 Web Speech API 配置
    if (this.recognition) {
      this.recognition.continuous = this.config.continuous;
      this.recognition.interimResults = this.config.interimResults;
      this.recognition.maxAlternatives = this.config.maxAlternatives;
      this.recognition.lang = this.config.language === 'auto' ? '' : this.config.language;
    }
  }

  /**
   * 获取配置
   */
  getConfig(): ASRConfig {
    return { ...this.config };
  }

  /**
   * 获取状态
   */
  getStatus(): ASRStatus {
    return this.status;
  }

  /**
   * 添加状态监听器
   */
  onStatusChange(listener: (status: ASRStatus) => void): void {
    this.statusListeners.push(listener);
  }

  /**
   * 添加结果监听器
   */
  onResult(listener: (result: ASRResult) => void): void {
    this.resultListeners.push(listener);
  }

  /**
   * 添加错误监听器
   */
  onError(listener: (error: ASRError) => void): void {
    this.errorListeners.push(listener);
  }

  /**
   * 更新状态
   */
  private setStatus(newStatus: ASRStatus): void {
    this.status = newStatus;
    this.statusListeners.forEach(listener => listener(newStatus));
  }

  /**
   * 开始识别
   */
  async start(): Promise<void> {
    if (!this.config.enabled) {
      throw new Error('ASR 未启用');
    }

    try {
      if (this.config.engine === 'web-speech') {
        await this.startWebSpeech();
      } else {
        // whisper.cpp 通过 IPC 调用主进程
        await this.startWhisper();
      }
    } catch (error) {
      const asrError: ASRError = {
        code: 'start-failed',
        message: '启动识别失败',
        details: (error as Error).message
      };
      this.errorListeners.forEach(listener => listener(asrError));
      this.setStatus('error');
      throw error;
    }
  }

  /**
   * 使用 Web Speech API 开始识别
   */
  private async startWebSpeech(): Promise<void> {
    if (!this.recognition) {
      throw new Error('Web Speech API 不可用');
    }

    // 请求麦克风权限
    try {
      this.mediaStream = await navigator.mediaDevices.getUserMedia({ audio: true });
    } catch (error) {
      throw new Error('无法访问麦克风');
    }

    this.recognition.start();
    this.setStatus('listening');
  }

  /**
   * 使用 whisper.cpp 开始识别
   */
  private async startWhisper(): Promise<void> {
    // 请求麦克风权限
    try {
      this.mediaStream = await navigator.mediaDevices.getUserMedia({ audio: true });
    } catch (error) {
      throw new Error('无法访问麦克风');
    }

    // 通过 IPC 调用主进程的 whisper 服务
    const result = await window.electron.ipcRenderer.invoke('asr:start', {
      engine: this.config.engine,
      model: this.config.model,
      language: this.config.language
    });

    if (!result.success) {
      throw new Error(result.error);
    }

    this.setStatus('listening');

    // 开始录音
    this.startRecording();
  }

  /**
   * 开始录音
   */
  private startRecording(): void {
    if (!this.mediaStream) return;

    this.audioChunks = [];
    this.mediaRecorder = new MediaRecorder(this.mediaStream);

    this.mediaRecorder.ondataavailable = (event) => {
      if (event.data.size > 0) {
        this.audioChunks.push(event.data);
      }
    };

    this.mediaRecorder.onstop = async () => {
      if (this.audioChunks.length === 0) return;

      const audioBlob = new Blob(this.audioChunks, { type: 'audio/webm' });
      await this.processAudio(audioBlob);
    };

    // 每秒收集一次音频数据
    this.mediaRecorder.start(1000);
  }

  /**
   * 处理音频数据
   */
  private async processAudio(audioBlob: Blob): Promise<void> {
    try {
      this.setStatus('processing');

      // 将音频转换为 base64
      const reader = new FileReader();
      reader.onloadend = async () => {
        const base64Audio = (reader.result as string).split(',')[1];

        // 通过 IPC 发送到主进程进行识别
        const result = await window.electron.ipcRenderer.invoke('asr:process', {
          audio: base64Audio,
          language: this.config.language
        });

        if (result.success && result.text) {
          const asrResult: ASRResult = {
            text: result.text,
            confidence: result.confidence || 0.9,
            isFinal: true,
            timestamp: Date.now()
          };
          this.resultListeners.forEach(listener => listener(asrResult));
        }

        this.setStatus('listening');
      };
      reader.readAsDataURL(audioBlob);
    } catch (error) {
      console.error('Audio processing error:', error);
      this.setStatus('listening');
    }
  }

  /**
   * 停止识别
   */
  stop(): void {
    // 停止 Web Speech API
    if (this.recognition) {
      try {
        this.recognition.stop();
      } catch (e) {
        // 忽略错误
      }
    }

    // 停止录音
    if (this.mediaRecorder && this.mediaRecorder.state !== 'inactive') {
      this.mediaRecorder.stop();
    }

    // 释放麦克风
    if (this.mediaStream) {
      this.mediaStream.getTracks().forEach(track => track.stop());
      this.mediaStream = null;
    }

    // 通知主进程停止 whisper
    if (this.config.engine !== 'web-speech') {
      window.electron.ipcRenderer.invoke('asr:stop').catch(console.error);
    }

    this.setStatus('idle');
  }

  /**
   * 暂停识别
   */
  pause(): void {
    if (this.recognition) {
      try {
        this.recognition.stop();
      } catch (e) {
        // 忽略错误
      }
    }

    if (this.mediaRecorder && this.mediaRecorder.state !== 'inactive') {
      this.mediaRecorder.pause();
    }

    this.setStatus('idle');
  }

  /**
   * 恢复识别
   */
  resume(): void {
    if (this.config.enabled && this.status !== 'listening') {
      this.start();
    }
  }

  /**
   * 检查是否可用
   */
  isAvailable(): boolean {
    if (this.config.engine === 'web-speech') {
      return !!(window as any).SpeechRecognition || !!(window as any).webkitSpeechRecognition;
    }
    return true;  // whisper.cpp 通过 IPC 检查
  }

  /**
   * 检查 whisper.cpp 是否可用
   */
  async checkWhisperAvailability(): Promise<{ available: boolean; version?: string }> {
    try {
      const result = await window.electron.ipcRenderer.invoke('asr:checkAvailability');
      return result;
    } catch (error) {
      return { available: false };
    }
  }

  /**
   * 销毁服务
   */
  destroy(): void {
    this.stop();
    this.statusListeners = [];
    this.resultListeners = [];
    this.errorListeners = [];
  }
}

// 单例
let asrServiceInstance: ASRService | null = null;

export function getASRService(): ASRService {
  if (!asrServiceInstance) {
    asrServiceInstance = new ASRService();
  }
  return asrServiceInstance;
}
