/**
 * 音频播放器
 * 使用 Web Audio API 实现音效播放
 */

import type {
  GiftSoundConfig,
  AudioPlayerConfig,
  SoundPlaybackState,
  SoundPlayEvent
} from '../../../renderer/modules/gift-sound/types.js';

export class AudioPlayer {
  private audioContext: AudioContext | null = null;
  private gainNode: GainNode | null = null;
  private currentSource: AudioBufferSourceNode | null = null;
  private currentGainNode: GainNode | null = null;
  private audioBuffers: Map<string, AudioBuffer> = new Map();
  private playbackState: SoundPlaybackState = {
    isPlaying: false,
    currentSound: null,
    volume: 0.8,
    progress: 0,
    duration: 0
  };
  private config: AudioPlayerConfig = {
    masterVolume: 0.8,
    fadeInDuration: 100,
    fadeOutDuration: 100,
    maxConcurrentSounds: 5
  };
  private eventListeners: Map<string, Function[]> = new Map();
  private progressInterval: NodeJS.Timeout | null = null;

  constructor() {
    this.setupEventListeners();
  }

  /**
   * 初始化音频上下文
   */
  async initialize(): Promise<void> {
    try {
      this.audioContext = new AudioContext();
      this.gainNode = this.audioContext.createGain();
      this.gainNode.gain.value = this.config.masterVolume;
      this.gainNode.connect(this.audioContext.destination);
      console.log('音频播放器初始化成功');
    } catch (error) {
      console.error('音频播放器初始化失败:', error);
      throw error;
    }
  }

  /**
   * 加载音频文件
   */
  async loadAudio(soundFile: string): Promise<AudioBuffer> {
    if (!this.audioContext) {
      throw new Error('音频上下文未初始化');
    }

    // 检查缓存
    if (this.audioBuffers.has(soundFile)) {
      return this.audioBuffers.get(soundFile)!;
    }

    try {
      // 从文件路径加载音频
      const response = await fetch(soundFile);
      const arrayBuffer = await response.arrayBuffer();
      const audioBuffer = await this.audioContext.decodeAudioData(arrayBuffer);

      // 缓存音频缓冲区
      this.audioBuffers.set(soundFile, audioBuffer);
      return audioBuffer;
    } catch (error) {
      console.error('加载音频文件失败:', error);
      throw error;
    }
  }

  /**
   * 播放音效
   */
  async play(giftSound: GiftSoundConfig): Promise<void> {
    if (!this.audioContext || !this.gainNode) {
      throw new Error('音频上下文未初始化');
    }

    try {
      // 停止当前播放
      this.stop();

      // 加载音频文件
      const audioBuffer = await this.loadAudio(giftSound.soundFile);

      // 创建音频源
      this.currentSource = this.audioContext.createBufferSource();
      this.currentSource.buffer = audioBuffer;

      // 创建音量控制节点
      this.currentGainNode = this.audioContext.createGain();
      this.currentGainNode.gain.value = giftSound.volume;

      // 连接音频图
      this.currentSource.connect(this.currentGainNode);
      this.currentGainNode.connect(this.gainNode);

      // 设置循环播放
      if (giftSound.playMode === 'loop') {
        this.currentSource.loop = true;
      }

      // 设置播放时长
      if (giftSound.duration > 0) {
        this.currentSource.start(0, 0, giftSound.duration / 1000);
      } else {
        this.currentSource.start(0);
      }

      // 更新播放状态
      this.playbackState = {
        isPlaying: true,
        currentSound: giftSound.soundFile,
        volume: giftSound.volume,
        progress: 0,
        duration: audioBuffer.duration
      };

      // 开始进度更新
      this.startProgressUpdate();

      // 应用淡入效果
      if (this.config.fadeInDuration > 0) {
        this.applyFadeIn(this.config.fadeInDuration);
      }

      // 触发播放开始事件
      this.emitEvent({
        type: 'start',
        soundFile: giftSound.soundFile,
        timestamp: new Date()
      });

      // 设置播放结束监听
      this.currentSource.onended = () => {
        this.handlePlaybackEnd(giftSound.soundFile);
      };

      console.log('开始播放音效:', giftSound.soundFile);
    } catch (error) {
      console.error('播放音效失败:', error);
      this.emitEvent({
        type: 'error',
        soundFile: giftSound.soundFile,
        timestamp: new Date(),
        error: (error as Error).message
      });
      throw error;
    }
  }

  /**
   * 停止播放
   */
  stop(): void {
    if (this.currentSource) {
      // 应用淡出效果
      if (this.config.fadeOutDuration > 0 && this.currentGainNode) {
        this.applyFadeOut(this.config.fadeOutDuration);
      }

      // 停止音频源
      try {
        this.currentSource.stop();
      } catch (error) {
        // 忽略已经停止的错误
      }

      this.currentSource = null;
      this.currentGainNode = null;
    }

    // 停止进度更新
    this.stopProgressUpdate();

    // 更新播放状态
    this.playbackState.isPlaying = false;
    this.playbackState.currentSound = null;
    this.playbackState.progress = 0;
  }

  /**
   * 暂停播放
   */
  pause(): void {
    if (this.audioContext && this.audioContext.state === 'running') {
      this.audioContext.suspend();
      this.playbackState.isPlaying = false;
      this.emitEvent({
        type: 'pause',
        soundFile: this.playbackState.currentSound || '',
        timestamp: new Date()
      });
    }
  }

  /**
   * 恢复播放
   */
  resume(): void {
    if (this.audioContext && this.audioContext.state === 'suspended') {
      this.audioContext.resume();
      this.playbackState.isPlaying = true;
      this.emitEvent({
        type: 'resume',
        soundFile: this.playbackState.currentSound || '',
        timestamp: new Date()
      });
    }
  }

  /**
   * 设置主音量
   */
  setMasterVolume(volume: number): void {
    this.config.masterVolume = Math.max(0, Math.min(1, volume));
    if (this.gainNode) {
      this.gainNode.gain.value = this.config.masterVolume;
    }
  }

  /**
   * 获取播放状态
   */
  getPlaybackState(): SoundPlaybackState {
    return { ...this.playbackState };
  }

  /**
   * 更新配置
   */
  updateConfig(config: Partial<AudioPlayerConfig>): void {
    this.config = { ...this.config, ...config };
  }

  /**
   * 应用淡入效果
   */
  private applyFadeIn(duration: number): void {
    if (!this.currentGainNode || !this.audioContext) return;

    const currentTime = this.audioContext.currentTime;
    this.currentGainNode.gain.setValueAtTime(0, currentTime);
    this.currentGainNode.gain.linearRampToValueAtTime(
      this.playbackState.volume,
      currentTime + duration / 1000
    );
  }

  /**
   * 应用淡出效果
   */
  private applyFadeOut(duration: number): void {
    if (!this.currentGainNode || !this.audioContext) return;

    const currentTime = this.audioContext.currentTime;
    this.currentGainNode.gain.setValueAtTime(
      this.currentGainNode.gain.value,
      currentTime
    );
    this.currentGainNode.gain.linearRampToValueAtTime(0, currentTime + duration / 1000);
  }

  /**
   * 开始进度更新
   */
  private startProgressUpdate(): void {
    this.stopProgressUpdate();
    this.progressInterval = setInterval(() => {
      if (this.playbackState.isPlaying && this.audioContext) {
        this.playbackState.progress += 0.1;
        if (this.playbackState.progress >= this.playbackState.duration) {
          this.playbackState.progress = this.playbackState.duration;
        }
      }
    }, 100);
  }

  /**
   * 停止进度更新
   */
  private stopProgressUpdate(): void {
    if (this.progressInterval) {
      clearInterval(this.progressInterval);
      this.progressInterval = null;
    }
  }

  /**
   * 处理播放结束
   */
  private handlePlaybackEnd(soundFile: string): void {
    this.playbackState.isPlaying = false;
    this.playbackState.currentSound = null;
    this.playbackState.progress = 0;

    this.stopProgressUpdate();

    this.emitEvent({
      type: 'end',
      soundFile,
      timestamp: new Date()
    });

    console.log('音效播放完成:', soundFile);
  }

  /**
   * 设置事件监听器
   */
  private setupEventListeners(): void {
    this.eventListeners.set('start', []);
    this.eventListeners.set('end', []);
    this.eventListeners.set('error', []);
    this.eventListeners.set('pause', []);
    this.eventListeners.set('resume', []);
  }

  /**
   * 添加事件监听器
   */
  on(event: string, listener: Function): void {
    if (!this.eventListeners.has(event)) {
      this.eventListeners.set(event, []);
    }
    this.eventListeners.get(event)!.push(listener);
  }

  /**
   * 移除事件监听器
   */
  off(event: string, listener: Function): void {
    if (this.eventListeners.has(event)) {
      const listeners = this.eventListeners.get(event)!;
      const index = listeners.indexOf(listener);
      if (index > -1) {
        listeners.splice(index, 1);
      }
    }
  }

  /**
   * 触发事件
   */
  private emitEvent(event: SoundPlayEvent): void {
    if (this.eventListeners.has(event.type)) {
      this.eventListeners.get(event.type)!.forEach(listener => {
        try {
          listener(event);
        } catch (error) {
          console.error('事件监听器执行错误:', error);
        }
      });
    }
  }

  /**
   * 清理资源
   */
  destroy(): void {
    this.stop();

    // 清理音频缓冲区
    this.audioBuffers.clear();

    // 关闭音频上下文
    if (this.audioContext) {
      this.audioContext.close();
      this.audioContext = null;
    }

    this.gainNode = null;
    this.eventListeners.clear();

    console.log('音频播放器已销毁');
  }
}

// 创建单例实例
export const audioPlayer = new AudioPlayer();
