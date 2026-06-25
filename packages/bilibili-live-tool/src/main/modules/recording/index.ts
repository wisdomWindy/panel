/**
 * 录制模块入口
 * 注册所有 IPC 处理器，实现事件推送机制
 */

import { ipcMain, BrowserWindow } from 'electron';
import { ffmpegManager } from './ffmpeg-manager.js';
import { recordingEngine } from './recording-engine.js';
import { HighlightDetector } from './highlight-detector.js';
import { fileManager } from './file-manager.js';
import { mainDatabase } from '../../database.js';
import type {
  RecordingOptions,
  RecordingConfig,
  QueryOptions
} from '../../../renderer/modules/recording/types.js';

let highlightDetector: HighlightDetector | null = null;
let mainWindow: BrowserWindow | null = null;

/**
 * 初始化录制模块
 */
export function initRecordingModule(window: BrowserWindow): void {
  mainWindow = window;

  // 初始化高光检测器
  const recordingConfig = mainDatabase.getRecordingConfig();
  highlightDetector = new HighlightDetector(recordingConfig.highlightConfig);

  // 设置高光检测器事件监听
  setupHighlightDetectorListeners();

  // 设置录制引擎事件监听
  setupRecordingEngineListeners();

  // 注册 IPC 处理器
  registerIpcHandlers();
}

/**
 * 设置高光检测器事件监听
 */
function setupHighlightDetectorListeners(): void {
  if (!highlightDetector) return;

  highlightDetector.on('highlight', (highlight) => {
    // 推送高光事件到渲染进程
    if (mainWindow) {
      mainWindow.webContents.send('recording:highlightMarked', highlight);
    }
  });

  highlightDetector.on('cooldown', (data) => {
    console.log('高光冷却中:', data);
  });
}

/**
 * 设置录制引擎事件监听
 */
function setupRecordingEngineListeners(): void {
  recordingEngine.on('statusUpdate', (status) => {
    // 推送状态更新到渲染进程
    if (mainWindow) {
      mainWindow.webContents.send('recording:statusChanged', status);
    }
  });

  recordingEngine.on('recordingError', (error) => {
    // 推送错误到渲染进程
    if (mainWindow) {
      mainWindow.webContents.send('recording:error', error.message);
    }
  });

  recordingEngine.on('recordingStopped', (session) => {
    // 录制停止后保存到数据库
    mainDatabase.addRecording({
      id: session.id,
      roomId: session.roomId,
      roomName: session.roomName,
      filePath: session.filePath,
      fileName: session.fileName,
      fileSize: session.fileSize,
      duration: session.duration,
      format: session.format,
      quality: session.quality,
      status: session.status,
      isHighlight: false,
      startedAt: session.startedAt,
      endedAt: session.endedAt,
      createdAt: new Date()
    });
  });
}

/**
 * 注册 IPC 处理器
 */
function registerIpcHandlers(): void {
  // ===== 录制控制 =====
  ipcMain.handle('recording:start', async (_event, options: RecordingOptions) => {
    try {
      // 确保输出目录存在
      await fileManager.ensureDirectory(options.outputPath);

      // 开始录制
      const session = await recordingEngine.startRecording(options);

      // 如果启用了高光检测，启动检测器
      if (options.highlightEnabled && highlightDetector) {
        highlightDetector.start();
      }

      return { success: true, session };
    } catch (error) {
      return { success: false, error: (error as Error).message };
    }
  });

  ipcMain.handle('recording:pause', (_event, _sessionId: string) => {
    try {
      recordingEngine.pauseRecording();
      return { success: true };
    } catch (error) {
      return { success: false, error: (error as Error).message };
    }
  });

  ipcMain.handle('recording:resume', (_event, _sessionId: string) => {
    try {
      recordingEngine.resumeRecording();
      return { success: true };
    } catch (error) {
      return { success: false, error: (error as Error).message };
    }
  });

  ipcMain.handle('recording:stop', (_event, _sessionId: string) => {
    try {
      const session = recordingEngine.stopRecording();

      // 停止高光检测器
      if (highlightDetector) {
        highlightDetector.stop();
      }

      return { success: true, session };
    } catch (error) {
      return { success: false, error: (error as Error).message };
    }
  });

  ipcMain.handle('recording:status', (_event, _sessionId: string) => {
    try {
      const status = recordingEngine.getRecordingStatus();
      return { success: true, status };
    } catch (error) {
      return { success: false, error: (error as Error).message };
    }
  });

  // ===== 高光管理 =====
  ipcMain.handle('recording:markHighlight', (_event, _sessionId: string, _reason: string, description?: string) => {
    try {
      if (highlightDetector) {
        highlightDetector.manualMark(description);
      }
      recordingEngine.incrementHighlightCount();
      return { success: true };
    } catch (error) {
      return { success: false, error: (error as Error).message };
    }
  });

  ipcMain.handle('recording:getHighlights', (_event, recordingId?: string) => {
    try {
      const highlights = mainDatabase.getHighlights(recordingId);
      return { success: true, highlights };
    } catch (error) {
      return { success: false, error: (error as Error).message };
    }
  });

  ipcMain.handle('recording:deleteHighlight', (_event, highlightId: string) => {
    try {
      mainDatabase.deleteHighlight(highlightId);
      return { success: true };
    } catch (error) {
      return { success: false, error: (error as Error).message };
    }
  });

  // ===== 录制文件管理 =====
  ipcMain.handle('recording:getRecordings', (_event, options?: QueryOptions) => {
    try {
      let recordings = mainDatabase.getRecordings();

      // 应用过滤条件
      if (options?.roomId) {
        recordings = recordings.filter(r => r.roomId === options.roomId);
      }
      if (options?.status) {
        recordings = recordings.filter(r => r.status === options.status);
      }
      if (options?.startDate) {
        recordings = recordings.filter(r => new Date(r.startedAt) >= options.startDate!);
      }
      if (options?.endDate) {
        recordings = recordings.filter(r => new Date(r.startedAt) <= options.endDate!);
      }

      // 排序
      if (options?.sortBy) {
        recordings.sort((a, b) => {
          const aValue = (a as any)[options.sortBy!];
          const bValue = (b as any)[options.sortBy!];
          const order = options.sortOrder === 'asc' ? 1 : -1;
          return aValue > bValue ? order : -1;
        });
      } else {
        // 默认按开始时间倒序
        recordings.sort((a, b) => new Date(b.startedAt).getTime() - new Date(a.startedAt).getTime());
      }

      // 分页
      if (options?.offset) {
        recordings = recordings.slice(options.offset);
      }
      if (options?.limit) {
        recordings = recordings.slice(0, options.limit);
      }

      return { success: true, recordings };
    } catch (error) {
      return { success: false, error: (error as Error).message };
    }
  });

  ipcMain.handle('recording:deleteRecording', async (_event, recordingId: string) => {
    try {
      const recording = mainDatabase.getRecordingById(recordingId);
      if (recording) {
        // 删除文件
        await fileManager.deleteFile(recording.filePath);
        // 删除关联的高光记录
        mainDatabase.deleteHighlightsByRecordingId(recordingId);
        // 删除录制记录
        mainDatabase.deleteRecording(recordingId);
      }
      return { success: true };
    } catch (error) {
      return { success: false, error: (error as Error).message };
    }
  });

  ipcMain.handle('recording:openFolder', async (_event, recordingId: string) => {
    try {
      const recording = mainDatabase.getRecordingById(recordingId);
      if (recording) {
        await fileManager.openFileFolder(recording.filePath);
      }
      return { success: true };
    } catch (error) {
      return { success: false, error: (error as Error).message };
    }
  });

  ipcMain.handle('recording:getPath', (_event, recordingId: string) => {
    try {
      const recording = mainDatabase.getRecordingById(recordingId);
      return { success: true, path: recording?.filePath || '' };
    } catch (error) {
      return { success: false, error: (error as Error).message };
    }
  });

  // ===== 配置 =====
  ipcMain.handle('recording:getConfig', () => {
    try {
      const config = mainDatabase.getRecordingConfig();
      return { success: true, config };
    } catch (error) {
      return { success: false, error: (error as Error).message };
    }
  });

  ipcMain.handle('recording:setConfig', (_event, config: Partial<RecordingConfig>) => {
    try {
      mainDatabase.setRecordingConfig(config);

      // 如果高光配置更新，同步更新检测器
      if (config.highlightConfig && highlightDetector) {
        highlightDetector.updateConfig(config.highlightConfig);
      }

      return { success: true };
    } catch (error) {
      return { success: false, error: (error as Error).message };
    }
  });

  // ===== FFmpeg 检测 =====
  ipcMain.handle('recording:checkFFmpeg', async () => {
    try {
      const info = await ffmpegManager.checkAvailability();
      return { success: true, info };
    } catch (error) {
      return { success: false, error: (error as Error).message };
    }
  });

  // ===== 事件处理（从渲染进程接收） =====
  ipcMain.on('recording:handleDanmaku', (_event, data) => {
    if (highlightDetector) {
      highlightDetector.handleDanmaku(data);
    }
  });

  ipcMain.on('recording:handleGift', (_event, data) => {
    if (highlightDetector) {
      highlightDetector.handleGift(data);
    }
  });

  ipcMain.on('recording:handleCaptain', (_event, data) => {
    if (highlightDetector) {
      highlightDetector.handleCaptain(data);
    }
  });
}

/**
 * 清理录制模块
 */
export function cleanupRecordingModule(): void {
  // 停止录制
  recordingEngine.destroy();

  // 停止高光检测器
  if (highlightDetector) {
    highlightDetector.destroy();
    highlightDetector = null;
  }

  // 清理 FFmpeg 管理器
  ffmpegManager.destroy();

  // 移除所有 IPC 处理器
  ipcMain.removeHandler('recording:start');
  ipcMain.removeHandler('recording:pause');
  ipcMain.removeHandler('recording:resume');
  ipcMain.removeHandler('recording:stop');
  ipcMain.removeHandler('recording:status');
  ipcMain.removeHandler('recording:markHighlight');
  ipcMain.removeHandler('recording:getHighlights');
  ipcMain.removeHandler('recording:deleteHighlight');
  ipcMain.removeHandler('recording:getRecordings');
  ipcMain.removeHandler('recording:deleteRecording');
  ipcMain.removeHandler('recording:openFolder');
  ipcMain.removeHandler('recording:getPath');
  ipcMain.removeHandler('recording:getConfig');
  ipcMain.removeHandler('recording:setConfig');
  ipcMain.removeHandler('recording:checkFFmpeg');
}
