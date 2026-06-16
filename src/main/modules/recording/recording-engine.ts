/**
 * 录制引擎
 * 封装 FFmpeg 调用，支持全程录制和分段录制
 */

import { EventEmitter } from 'events';
import { ffmpegManager } from './ffmpeg-manager.js';
import type {
  RecordingFormat,
  RecordingOptions,
  RecordingSession,
  RecordingStatus
} from '../../../renderer/modules/recording/types.js';

export class RecordingEngine extends EventEmitter {
  private currentSession: RecordingSession | null = null;
  private statusTimer: NodeJS.Timeout | null = null;
  private startTime: number = 0;
  private pausedDuration: number = 0;
  private pauseStartTime: number = 0;

  constructor() {
    super();
  }

  /**
   * 开始录制
   */
  async startRecording(options: RecordingOptions): Promise<RecordingSession> {
    if (this.currentSession) {
      throw new Error('已有录制会话进行中，请先停止当前录制');
    }

    // 检查 FFmpeg 可用性
    const ffmpegInfo = await ffmpegManager.checkAvailability();
    if (!ffmpegInfo.available) {
      throw new Error('FFmpeg 未安装或不可用，请安装 FFmpeg 后重试');
    }

    // 生成会话 ID
    const sessionId = this.generateSessionId();
    const fileName = this.generateFileName(options.roomId, options.format);
    const filePath = `${options.outputPath}/${fileName}`;

    // 构建 FFmpeg 参数
    const args = this.buildFFmpegArgs(options, filePath);

    // 启动 FFmpeg 进程
    ffmpegManager.startProcess(args);

    // 创建录制会话
    this.currentSession = {
      id: sessionId,
      roomId: options.roomId,
      roomName: options.roomName || '',
      status: 'recording',
      filePath,
      fileName,
      fileSize: 0,
      duration: 0,
      format: options.format,
      quality: options.quality,
      startedAt: new Date(),
      highlightCount: 0
    };

    this.startTime = Date.now();
    this.pausedDuration = 0;

    // 启动状态更新定时器
    this.startStatusTimer();

    // 监听 FFmpeg 事件
    this.setupFFmpegListeners();

    this.emit('recordingStarted', this.currentSession);
    return this.currentSession;
  }

  /**
   * 暂停录制
   */
  pauseRecording(): void {
    if (!this.currentSession || this.currentSession.status !== 'recording') {
      throw new Error('没有正在进行的录制');
    }

    ffmpegManager.pauseProcess();
    this.currentSession.status = 'paused';
    this.pauseStartTime = Date.now();
    this.stopStatusTimer();

    this.emit('recordingPaused', this.currentSession);
  }

  /**
   * 恢复录制
   */
  resumeRecording(): void {
    if (!this.currentSession || this.currentSession.status !== 'paused') {
      throw new Error('没有暂停的录制');
    }

    ffmpegManager.resumeProcess();
    this.currentSession.status = 'recording';
    this.pausedDuration += Date.now() - this.pauseStartTime;
    this.startStatusTimer();

    this.emit('recordingResumed', this.currentSession);
  }

  /**
   * 停止录制
   */
  stopRecording(): RecordingSession | null {
    if (!this.currentSession) {
      return null;
    }

    ffmpegManager.stopProcess();
    this.stopStatusTimer();

    const session = { ...this.currentSession };
    session.status = 'completed';
    session.endedAt = new Date();
    session.duration = this.getCurrentDuration();

    this.emit('recordingStopped', session);

    this.currentSession = null;
    this.startTime = 0;
    this.pausedDuration = 0;

    return session;
  }

  /**
   * 获取当前录制状态
   */
  getRecordingStatus(): RecordingStatus | null {
    if (!this.currentSession) {
      return null;
    }

    return {
      sessionId: this.currentSession.id,
      status: this.currentSession.status,
      currentSize: this.currentSession.fileSize,
      currentDuration: this.getCurrentDuration(),
      highlightCount: this.currentSession.highlightCount
    };
  }

  /**
   * 获取当前录制会话
   */
  getCurrentSession(): RecordingSession | null {
    return this.currentSession;
  }

  /**
   * 增加高光计数
   */
  incrementHighlightCount(): void {
    if (this.currentSession) {
      this.currentSession.highlightCount++;
    }
  }

  /**
   * 构建 FFmpeg 参数
   */
  private buildFFmpegArgs(options: RecordingOptions, outputPath: string): string[] {
    const args: string[] = [];

    // 输入源（直播流 URL）
    args.push('-i', options.roomId);

    // 根据质量设置编码参数
    switch (options.quality) {
      case '720p':
        args.push('-vf', 'scale=-1:720');
        break;
      case '1080p':
        args.push('-vf', 'scale=-1:1080');
        break;
      case 'original':
      default:
        // 保持原始质量
        break;
    }

    // 根据格式设置编码器
    switch (options.format) {
      case 'mp4':
        args.push('-c:v', 'libx264');
        args.push('-c:a', 'aac');
        args.push('-f', 'mp4');
        break;
      case 'flv':
        args.push('-c:v', 'libx264');
        args.push('-c:a', 'aac');
        args.push('-f', 'flv');
        break;
    }

    // 分段设置
    if (options.segmentSize && options.segmentSize > 0) {
      // 按文件大小分段（使用 segment muxer）
      args.push('-f', 'segment');
      args.push('-segment_format', options.format);
      args.push('-segment_list', `${outputPath}.csv`);
      args.push('-segment_list_type', 'csv');
      // 每个分段的最大字节数
      const segmentSizeBytes = options.segmentSize * 1024 * 1024;
      args.push('-fs', segmentSizeBytes.toString());
      // 分段文件名格式
      const segmentOutputPath = outputPath.replace(`.${options.format}`, '_%03d.${options.format}');
      args.push(segmentOutputPath);
    } else if (options.segmentDuration && options.segmentDuration > 0) {
      // 按时长分段
      args.push('-f', 'segment');
      args.push('-segment_format', options.format);
      args.push('-segment_time', options.segmentDuration.toString());
      args.push('-segment_list', `${outputPath}.csv`);
      args.push('-segment_list_type', 'csv');
      args.push('-reset_timestamps', '1');
      const segmentOutputPath = outputPath.replace(`.${options.format}`, '_%03d.${options.format}');
      args.push(segmentOutputPath);
    } else {
      // 不分段
      args.push(outputPath);
    }

    // 覆盖输出文件
    args.push('-y');

    return args;
  }

  /**
   * 生成会话 ID
   */
  private generateSessionId(): string {
    return `rec_${Date.now()}_${Math.random().toString(36).substr(2, 9)}`;
  }

  /**
   * 生成文件名
   */
  private generateFileName(roomId: string, format: RecordingFormat): string {
    const now = new Date();
    const dateStr = now.toISOString().replace(/[:.]/g, '-').slice(0, 19);
    return `${roomId}_${dateStr}.${format}`;
  }

  /**
   * 获取当前录制时长（秒）
   */
  private getCurrentDuration(): number {
    if (!this.currentSession) {
      return 0;
    }

    if (this.currentSession.status === 'paused') {
      return Math.floor((this.pauseStartTime - this.startTime - this.pausedDuration) / 1000);
    }

    return Math.floor((Date.now() - this.startTime - this.pausedDuration) / 1000);
  }

  /**
   * 启动状态更新定时器
   */
  private startStatusTimer(): void {
    this.stopStatusTimer();
    this.statusTimer = setInterval(() => {
      if (this.currentSession) {
        this.currentSession.duration = this.getCurrentDuration();
        this.emit('statusUpdate', {
          sessionId: this.currentSession.id,
          status: this.currentSession.status,
          currentSize: this.currentSession.fileSize,
          currentDuration: this.currentSession.duration,
          highlightCount: this.currentSession.highlightCount
        });
      }
    }, 1000);
  }

  /**
   * 停止状态更新定时器
   */
  private stopStatusTimer(): void {
    if (this.statusTimer) {
      clearInterval(this.statusTimer);
      this.statusTimer = null;
    }
  }

  /**
   * 设置 FFmpeg 事件监听
   */
  private setupFFmpegListeners(): void {
    ffmpegManager.on('stderr', (data: string) => {
      // 解析 FFmpeg 输出获取文件大小等信息
      this.parseFFmpegOutput(data);
    });

    ffmpegManager.on('error', (error: Error) => {
      if (this.currentSession) {
        this.currentSession.status = 'failed';
        this.emit('recordingError', error);
      }
    });

    ffmpegManager.on('close', (code: number) => {
      if (this.currentSession && code !== 0) {
        this.currentSession.status = 'failed';
        this.emit('recordingError', new Error(`FFmpeg 进程退出，代码: ${code}`));
      }
    });
  }

  /**
   * 解析 FFmpeg 输出
   */
  private parseFFmpegOutput(data: string): void {
    // 解析 FFmpeg 输出中的时间信息
    const timeMatch = data.match(/time=(\d{2}):(\d{2}):(\d{2})/);
    if (timeMatch && this.currentSession) {
      const hours = parseInt(timeMatch[1]);
      const minutes = parseInt(timeMatch[2]);
      const seconds = parseInt(timeMatch[3]);
      this.currentSession.duration = hours * 3600 + minutes * 60 + seconds;
    }

    // 解析 FFmpeg 输出中的大小信息
    const sizeMatch = data.match(/size=\s*(\d+)kB/);
    if (sizeMatch && this.currentSession) {
      this.currentSession.fileSize = parseInt(sizeMatch[1]) * 1024;
    }
  }

  /**
   * 销毁引擎，清理资源
   */
  destroy(): void {
    this.stopRecording();
    this.removeAllListeners();
  }
}

// 单例
export const recordingEngine = new RecordingEngine();
