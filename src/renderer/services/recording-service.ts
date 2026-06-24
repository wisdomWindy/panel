/**
 * 录制服务
 * 封装 IPC 调用，提供统一的服务接口
 */

import type {
  Recording,
  Highlight,
  RecordingConfig,
  RecordingSession,
  RecordingStatus,
  RecordingOptions,
  QueryOptions
} from '../modules/recording/types.js';

export interface RecordingService {
  // 录制控制
  startRecording(options: RecordingOptions): Promise<RecordingSession>;
  pauseRecording(sessionId: string): Promise<void>;
  resumeRecording(sessionId: string): Promise<void>;
  stopRecording(sessionId: string): Promise<RecordingSession>;

  // 高光管理
  markHighlight(sessionId: string, reason: string, description?: string): Promise<void>;
  getHighlights(recordingId?: string): Promise<Highlight[]>;
  deleteHighlight(highlightId: string): Promise<void>;

  // 录制查询
  getRecordingStatus(sessionId: string): Promise<RecordingStatus>;
  getRecordings(options?: QueryOptions): Promise<Recording[]>;

  // 文件操作
  deleteRecording(recordingId: string): Promise<void>;
  openRecordingFolder(recordingId: string): Promise<void>;
  getRecordingPath(recordingId: string): Promise<string>;

  // 配置
  getConfig(): Promise<RecordingConfig>;
  updateConfig(config: Partial<RecordingConfig>): Promise<void>;

  // FFmpeg
  checkFFmpeg(): Promise<{ available: boolean; version: string }>;
}

class RecordingServiceImpl implements RecordingService {
  /**
   * 开始录制
   */
  async startRecording(options: RecordingOptions): Promise<RecordingSession> {
    const result = await window.electronAPI.invoke('recording:start', options) as any;
    if (!result.success) {
      throw new Error(result.error);
    }
    return result.session;
  }

  /**
   * 暂停录制
   */
  async pauseRecording(sessionId: string): Promise<void> {
    const result = await window.electronAPI.invoke('recording:pause', sessionId) as any;
    if (!result.success) {
      throw new Error(result.error);
    }
  }

  /**
   * 恢复录制
   */
  async resumeRecording(sessionId: string): Promise<void> {
    const result = await window.electronAPI.invoke('recording:resume', sessionId) as any;
    if (!result.success) {
      throw new Error(result.error);
    }
  }

  /**
   * 停止录制
   */
  async stopRecording(sessionId: string): Promise<RecordingSession> {
    const result = await window.electronAPI.invoke('recording:stop', sessionId) as any;
    if (!result.success) {
      throw new Error(result.error);
    }
    return result.session;
  }

  /**
   * 标记高光
   */
  async markHighlight(sessionId: string, reason: string, description?: string): Promise<void> {
    const result = await window.electronAPI.invoke('recording:markHighlight', sessionId, reason, description) as any;
    if (!result.success) {
      throw new Error(result.error);
    }
  }

  /**
   * 获取高光列表
   */
  async getHighlights(recordingId?: string): Promise<Highlight[]> {
    const result = await window.electronAPI.invoke('recording:getHighlights', recordingId) as any;
    if (!result.success) {
      throw new Error(result.error);
    }
    return result.highlights;
  }

  /**
   * 删除高光
   */
  async deleteHighlight(highlightId: string): Promise<void> {
    const result = await window.electronAPI.invoke('recording:deleteHighlight', highlightId) as any;
    if (!result.success) {
      throw new Error(result.error);
    }
  }

  /**
   * 获取录制状态
   */
  async getRecordingStatus(sessionId: string): Promise<RecordingStatus> {
    const result = await window.electronAPI.invoke('recording:status', sessionId) as any;
    if (!result.success) {
      throw new Error(result.error);
    }
    return result.status;
  }

  /**
   * 获取录制列表
   */
  async getRecordings(options?: QueryOptions): Promise<Recording[]> {
    const result = await window.electronAPI.invoke('recording:getRecordings', options) as any;
    if (!result.success) {
      throw new Error(result.error);
    }
    return result.recordings;
  }

  /**
   * 删除录制
   */
  async deleteRecording(recordingId: string): Promise<void> {
    const result = await window.electronAPI.invoke('recording:deleteRecording', recordingId) as any;
    if (!result.success) {
      throw new Error(result.error);
    }
  }

  /**
   * 打开录制文件所在目录
   */
  async openRecordingFolder(recordingId: string): Promise<void> {
    const result = await window.electronAPI.invoke('recording:openFolder', recordingId) as any;
    if (!result.success) {
      throw new Error(result.error);
    }
  }

  /**
   * 获取录制文件路径
   */
  async getRecordingPath(recordingId: string): Promise<string> {
    const result = await window.electronAPI.invoke('recording:getPath', recordingId) as any;
    if (!result.success) {
      throw new Error(result.error);
    }
    return result.path;
  }

  /**
   * 获取录制配置
   */
  async getConfig(): Promise<RecordingConfig> {
    const result = await window.electronAPI.invoke('recording:getConfig') as any;
    if (!result.success) {
      throw new Error(result.error);
    }
    return result.config;
  }

  /**
   * 更新录制配置
   */
  async updateConfig(config: Partial<RecordingConfig>): Promise<void> {
    const result = await window.electronAPI.invoke('recording:setConfig', config) as any;
    if (!result.success) {
      throw new Error(result.error);
    }
  }

  /**
   * 检测 FFmpeg 可用性
   */
  async checkFFmpeg(): Promise<{ available: boolean; version: string }> {
    const result = await window.electronAPI.invoke('recording:checkFFmpeg') as any;
    if (!result.success) {
      throw new Error(result.error);
    }
    return result.info;
  }
}

// 单例
let recordingServiceInstance: RecordingService | null = null;

export function getRecordingService(): RecordingService {
  if (!recordingServiceInstance) {
    recordingServiceInstance = new RecordingServiceImpl();
  }
  return recordingServiceInstance;
}
