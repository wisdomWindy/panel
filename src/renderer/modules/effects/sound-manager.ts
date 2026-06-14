/**
 * 音效管理器
 * 负责播放和管理特效音效
 */

import { SoundManager } from './types';

/**
 * HTML5 Audio 封装类
 */
class AudioPlayer {
  private audio: HTMLAudioElement;
  private volume: number = 0.7; // 默认音量

  constructor() {
    this.audio = new Audio();
    this.audio.volume = this.volume;
  }

  /**
   * 播放音频
   * @param filePath 音频文件路径
   * @param volume 音量 (0-1)
   */
  async play(filePath: string, volume?: number): Promise<void> {
    return new Promise((resolve, reject) => {
      try {
        this.audio.src = filePath;

        if (volume !== undefined) {
          this.audio.volume = Math.max(0, Math.min(1, volume));
        } else {
          this.audio.volume = this.volume;
        }

        this.audio.onended = () => {
          resolve();
        };

        this.audio.onerror = error => {
          console.error('音频播放错误:', error);
          reject(new Error(`音频播放失败: ${filePath}`));
        };

        this.audio.play().catch(error => {
          console.error('音频播放失败:', error);
          reject(error);
        });
      } catch (error) {
        reject(error);
      }
    });
  }

  /**
   * 停止播放
   */
  stop(): void {
    this.audio.pause();
    this.audio.currentTime = 0;
  }

  /**
   * 设置音量
   * @param volume 音量 (0-1)
   */
  setVolume(volume: number): void {
    this.volume = Math.max(0, Math.min(1, volume));
    this.audio.volume = this.volume;
  }
}

/**
 * 默认音效配置
 */
const DEFAULT_SOUNDS = {
  captain_first: '/sounds/captain_first.mp3',
  captain_renew: '/sounds/captain_renew.mp3',
  gift_combo: '/sounds/gift_combo.mp3'
};

/**
 * 音效管理器实现
 */
export class SoundManagerImpl implements SoundManager {
  private audioPlayer: AudioPlayer;
  private soundMap: Map<string, string> = new Map();
  private isInitialized: boolean = false;

  constructor() {
    this.audioPlayer = new AudioPlayer();
  }

  /**
   * 初始化音效管理器
   * @param customSounds 自定义音效配置
   */
  async initialize(customSounds?: Record<string, string>): Promise<void> {
    try {
      // 加载默认音效
      Object.entries(DEFAULT_SOUNDS).forEach(([key, path]) => {
        this.soundMap.set(key, path);
      });

      // 加载自定义音效
      if (customSounds) {
        Object.entries(customSounds).forEach(([key, path]) => {
          this.soundMap.set(key, path);
        });
      }

      this.isInitialized = true;
      console.warn('音效管理器初始化成功');
    } catch (error) {
      console.error('音效管理器初始化失败:', error);
      throw error;
    }
  }

  /**
   * 播放音效
   * @param key 音效键名
   * @param volume 音量 (0-1)
   */
  async playSound(key: string, volume?: number): Promise<void> {
    if (!this.isInitialized) {
      console.warn('音效管理器未初始化');
      return;
    }

    const soundPath = this.soundMap.get(key);
    if (!soundPath) {
      console.warn(`未找到音效: ${key}`);
      return;
    }

    try {
      await this.audioPlayer.play(soundPath, volume);
      console.warn(`播放音效成功: ${key}`);
    } catch (error) {
      console.error(`播放音效失败: ${key}`, error);
    }
  }

  /**
   * 停止当前播放的音效
   */
  stopSound(): void {
    this.audioPlayer.stop();
  }

  /**
   * 设置全局音量
   * @param volume 音量 (0-1)
   */
  setVolume(volume: number): void {
    this.audioPlayer.setVolume(volume);
  }

  /**
   * 添加自定义音效
   * @param key 音效键名
   * @param path 音效文件路径
   */
  addSound(key: string, path: string): void {
    this.soundMap.set(key, path);
  }

  /**
   * 移除音效
   * @param key 音效键名
   */
  removeSound(key: string): void {
    this.soundMap.delete(key);
  }

  /**
   * 获取所有音效键名
   * @returns 音效键名列表
   */
  getSoundKeys(): string[] {
    return Array.from(this.soundMap.keys());
  }

  /**
   * 检查音效是否存在
   * @param key 音效键名
   * @returns 是否存在
   */
  hasSound(key: string): boolean {
    return this.soundMap.has(key);
  }
}

// 创建全局音效管理器实例
export const soundManager = new SoundManagerImpl();
