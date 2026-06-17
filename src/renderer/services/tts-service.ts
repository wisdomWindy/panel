/**
 * TTS 服务
 * 使用 Web Speech API 实现语音播报
 */

import type {
  TTSConfig,
  BroadcastMessage,
  TTSStatus,
  TTSEngineInfo
} from '../modules/tts/types.js';
import { DEFAULT_TTS_CONFIG, MAX_QUEUE_SIZE } from '../modules/tts/types.js';

export class TTSService {
  private config: TTSConfig = { ...DEFAULT_TTS_CONFIG };
  private status: TTSStatus = 'idle';
  private queue: BroadcastMessage[] = [];
  private isProcessing = false;
  private lastBroadcastTime = 0;
  private statusListeners: Array<(status: TTSStatus) => void> = [];
  private messageListeners: Array<(message: BroadcastMessage) => void> = [];

  constructor() {
    this.initSpeechSynthesis();
  }

  /**
   * 初始化 SpeechSynthesis
   */
  private initSpeechSynthesis(): void {
    if (typeof window !== 'undefined' && window.speechSynthesis) {
      // Chrome 需要先调用一次 getVoices() 来触发 voicesloaded 事件
      window.speechSynthesis.getVoices();
    }
  }

  /**
   * 获取可用语音列表
   */
  getVoices(): TTSEngineInfo[] {
    if (typeof window === 'undefined' || !window.speechSynthesis) {
      return [];
    }

    const voices = window.speechSynthesis.getVoices();
    return voices.map(voice => ({
      name: voice.name,
      lang: voice.lang,
      localService: voice.localService,
      voiceURI: voice.voiceURI
    }));
  }

  /**
   * 获取中文语音列表
   */
  getChineseVoices(): TTSEngineInfo[] {
    return this.getVoices().filter(voice =>
      voice.lang.startsWith('zh') || voice.lang.startsWith('cmn')
    );
  }

  /**
   * 更新配置
   */
  updateConfig(config: Partial<TTSConfig>): void {
    this.config = { ...this.config, ...config };
  }

  /**
   * 获取配置
   */
  getConfig(): TTSConfig {
    return { ...this.config };
  }

  /**
   * 获取状态
   */
  getStatus(): TTSStatus {
    return this.status;
  }

  /**
   * 添加状态监听器
   */
  onStatusChange(listener: (status: TTSStatus) => void): void {
    this.statusListeners.push(listener);
  }

  /**
   * 添加消息监听器
   */
  onMessage(listener: (message: BroadcastMessage) => void): void {
    this.messageListeners.push(listener);
  }

  /**
   * 更新状态
   */
  private setStatus(newStatus: TTSStatus): void {
    this.status = newStatus;
    this.statusListeners.forEach(listener => listener(newStatus));
  }

  /**
   * 处理弹幕消息
   */
  handleDanmaku(username: string, content: string, _userId: string): void {
    if (!this.config.enabled) return;

    // 检查过滤关键词
    if (this.shouldFilter(content)) return;

    const message: BroadcastMessage = {
      id: `danmaku_${Date.now()}_${Math.random().toString(36).substr(2, 9)}`,
      type: 'danmaku',
      username,
      content,
      timestamp: Date.now()
    };

    this.addToQueue(message);
  }

  /**
   * 处理礼物消息
   */
  handleGift(username: string, giftName: string, giftCount: number, giftValue: number): void {
    if (!this.config.enabled || !this.config.broadcastGifts) return;

    // 检查礼物价值阈值
    if (giftValue < this.config.giftValueThreshold) return;

    const message: BroadcastMessage = {
      id: `gift_${Date.now()}_${Math.random().toString(36).substr(2, 9)}`,
      type: 'gift',
      username,
      content: `${giftName} x${giftCount}`,
      timestamp: Date.now(),
      giftName,
      giftCount,
      giftValue
    };

    this.addToQueue(message);
  }

  /**
   * 处理舰长消息
   */
  handleCaptain(username: string, level: number): void {
    if (!this.config.enabled || !this.config.broadcastCaptain) return;

    const levelNames = ['舰长', '提督', '总督'];
    const levelName = levelNames[level - 1] || '舰长';

    const message: BroadcastMessage = {
      id: `captain_${Date.now()}_${Math.random().toString(36).substr(2, 9)}`,
      type: 'captain',
      username,
      content: levelName,
      timestamp: Date.now(),
      captainLevel: level
    };

    this.addToQueue(message);
  }

  /**
   * 检查是否应该过滤
   */
  private shouldFilter(content: string): boolean {
    const lowerContent = content.toLowerCase();
    return this.config.filterKeywords.some(keyword =>
      lowerContent.includes(keyword.toLowerCase())
    );
  }

  /**
   * 添加到播报队列
   */
  private addToQueue(message: BroadcastMessage): void {
    // 队列满时丢弃最旧的消息
    if (this.queue.length >= MAX_QUEUE_SIZE) {
      this.queue.shift();
    }

    this.queue.push(message);
    this.messageListeners.forEach(listener => listener(message));

    // 开始处理队列
    if (!this.isProcessing) {
      this.processQueue();
    }
  }

  /**
   * 处理播报队列
   */
  private async processQueue(): Promise<void> {
    if (this.isProcessing || this.queue.length === 0) return;

    this.isProcessing = true;
    this.setStatus('speaking');

    while (this.queue.length > 0) {
      // 检查最小播报间隔
      const now = Date.now();
      const timeSinceLastBroadcast = now - this.lastBroadcastTime;
      if (timeSinceLastBroadcast < this.config.minInterval) {
        await this.delay(this.config.minInterval - timeSinceLastBroadcast);
      }

      const message = this.queue.shift();
      if (message) {
        await this.speakMessage(message);
        this.lastBroadcastTime = Date.now();
      }
    }

    this.isProcessing = false;
    this.setStatus('idle');
  }

  /**
   * 播报单条消息
   */
  private async speakMessage(message: BroadcastMessage): Promise<void> {
    const text = this.formatMessage(message);
    await this.speak(text);
  }

  /**
   * 格式化播报文本
   */
  private formatMessage(message: BroadcastMessage): string {
    let template = this.config.formatTemplate;

    // 根据消息类型使用不同的默认模板
    if (message.type === 'gift') {
      template = '感谢{username}送出的{giftName}';
    } else if (message.type === 'captain') {
      template = '欢迎{username}开通{captainLevel}';
    }

    return template
      .replace('{username}', message.username)
      .replace('{content}', message.content)
      .replace('{giftName}', message.giftName || '')
      .replace('{giftCount}', String(message.giftCount || ''))
      .replace('{captainLevel}', message.content);
  }

  /**
   * 播报文本
   */
  async speak(text: string): Promise<void> {
    if (typeof window === 'undefined' || !window.speechSynthesis) {
      console.warn('SpeechSynthesis not available');
      return;
    }

    return new Promise((resolve, reject) => {
      const utterance = new SpeechSynthesisUtterance(text);

      // 设置语音参数
      utterance.rate = this.config.speed;
      utterance.pitch = this.config.pitch;
      utterance.volume = this.config.volume;

      // 设置语音
      const voices = window.speechSynthesis.getVoices();
      const selectedVoice = voices.find(v => v.name === this.config.voice);
      if (selectedVoice) {
        utterance.voice = selectedVoice;
      }

      // 设置语言
      utterance.lang = 'zh-CN';

      utterance.onend = () => resolve();
      utterance.onerror = (event) => {
        console.error('TTS error:', event);
        reject(event);
      };

      window.speechSynthesis.speak(utterance);
    });
  }

  /**
   * 停止播报
   */
  stop(): void {
    if (typeof window !== 'undefined' && window.speechSynthesis) {
      window.speechSynthesis.cancel();
    }
    this.queue = [];
    this.isProcessing = false;
    this.setStatus('idle');
  }

  /**
   * 暂停播报
   */
  pause(): void {
    if (typeof window !== 'undefined' && window.speechSynthesis) {
      window.speechSynthesis.pause();
    }
    this.setStatus('paused');
  }

  /**
   * 恢复播报
   */
  resume(): void {
    if (typeof window !== 'undefined' && window.speechSynthesis) {
      window.speechSynthesis.resume();
    }
    this.setStatus('speaking');
  }

  /**
   * 获取队列长度
   */
  getQueueLength(): number {
    return this.queue.length;
  }

  /**
   * 延迟
   */
  private delay(ms: number): Promise<void> {
    return new Promise(resolve => setTimeout(resolve, ms));
  }

  /**
   * 销毁服务
   */
  destroy(): void {
    this.stop();
    this.statusListeners = [];
    this.messageListeners = [];
  }
}

// 单例
let ttsServiceInstance: TTSService | null = null;

export function getTTSService(): TTSService {
  if (!ttsServiceInstance) {
    ttsServiceInstance = new TTSService();
  }
  return ttsServiceInstance;
}
