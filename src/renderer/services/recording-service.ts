/**
 * 录制服务
 * 封装 IPC 调用，提供统一的服务接口
 */

import { invokeIPC, invokeIPCVoid } from '../utils/ipc.js';
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
  startRecording(options: RecordingOptions): Promise<RecordingSession>;
  pauseRecording(sessionId: string): Promise<void>;
  resumeRecording(sessionId: string): Promise<void>;
  stopRecording(sessionId: string): Promise<RecordingSession>;
  markHighlight(sessionId: string, reason: string, description?: string): Promise<void>;
  getHighlights(recordingId?: string): Promise<Highlight[]>;
  deleteHighlight(highlightId: string): Promise<void>;
  getRecordingStatus(sessionId: string): Promise<RecordingStatus>;
  getRecordings(options?: QueryOptions): Promise<Recording[]>;
  deleteRecording(recordingId: string): Promise<void>;
  openRecordingFolder(recordingId: string): Promise<void>;
  getRecordingPath(recordingId: string): Promise<string>;
  getConfig(): Promise<RecordingConfig>;
  updateConfig(config: Partial<RecordingConfig>): Promise<void>;
  checkFFmpeg(): Promise<{ available: boolean; version: string }>;
}

class RecordingServiceImpl implements RecordingService {
  async startRecording(options: RecordingOptions): Promise<RecordingSession> {
    const data = await invokeIPC('recording:start', options) as { session: RecordingSession };
    return data.session;
  }

  async pauseRecording(sessionId: string): Promise<void> {
    await invokeIPCVoid('recording:pause', sessionId);
  }

  async resumeRecording(sessionId: string): Promise<void> {
    await invokeIPCVoid('recording:resume', sessionId);
  }

  async stopRecording(sessionId: string): Promise<RecordingSession> {
    const data = await invokeIPC('recording:stop', sessionId) as { session: RecordingSession };
    return data.session;
  }

  async markHighlight(sessionId: string, reason: string, description?: string): Promise<void> {
    await invokeIPCVoid('recording:markHighlight', sessionId, reason, description);
  }

  async getHighlights(recordingId?: string): Promise<Highlight[]> {
    const data = await invokeIPC('recording:getHighlights', recordingId) as { highlights: Highlight[] };
    return data.highlights;
  }

  async deleteHighlight(highlightId: string): Promise<void> {
    await invokeIPCVoid('recording:deleteHighlight', highlightId);
  }

  async getRecordingStatus(sessionId: string): Promise<RecordingStatus> {
    const data = await invokeIPC('recording:status', sessionId) as { status: RecordingStatus };
    return data.status;
  }

  async getRecordings(options?: QueryOptions): Promise<Recording[]> {
    const data = await invokeIPC('recording:getRecordings', options) as { recordings: Recording[] };
    return data.recordings;
  }

  async deleteRecording(recordingId: string): Promise<void> {
    await invokeIPCVoid('recording:deleteRecording', recordingId);
  }

  async openRecordingFolder(recordingId: string): Promise<void> {
    await invokeIPCVoid('recording:openFolder', recordingId);
  }

  async getRecordingPath(recordingId: string): Promise<string> {
    const data = await invokeIPC('recording:getPath', recordingId) as { path: string };
    return data.path;
  }

  async getConfig(): Promise<RecordingConfig> {
    const data = await invokeIPC('recording:getConfig') as { config: RecordingConfig };
    return data.config;
  }

  async updateConfig(config: Partial<RecordingConfig>): Promise<void> {
    await invokeIPCVoid('recording:setConfig', config);
  }

  async checkFFmpeg(): Promise<{ available: boolean; version: string }> {
    const data = await invokeIPC('recording:checkFFmpeg') as { info: { available: boolean; version: string } };
    return data.info;
  }
}

let recordingServiceInstance: RecordingService | null = null;

export function getRecordingService(): RecordingService {
  if (!recordingServiceInstance) {
    recordingServiceInstance = new RecordingServiceImpl();
  }
  return recordingServiceInstance;
}
