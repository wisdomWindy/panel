/**
 * 礼物音效模块入口
 * 注册所有 IPC 处理器，实现音效播放功能
 */

import { ipcMain, BrowserWindow } from 'electron';
import { mainDatabase } from '../../database.js';
import type {
  GiftSoundConfig,
  PlaySoundRequest,
  PlaySoundResult
} from '../../../renderer/modules/gift-sound/types.js';

let mainWindow: BrowserWindow | null = null;

/**
 * 初始化礼物音效模块
 */
export function initGiftSoundModule(window: BrowserWindow): void {
  mainWindow = window;

  // 注册 IPC 处理器
  registerIpcHandlers();
}

/**
 * 注册 IPC 处理器
 */
function registerIpcHandlers(): void {
  // ===== 礼物音效配置 =====
  ipcMain.handle('gift-sound:getAll', () => {
    try {
      const giftSounds = mainDatabase.getGiftSounds();
      return { success: true, giftSounds };
    } catch (error) {
      return { success: false, error: (error as Error).message };
    }
  });

  ipcMain.handle('gift-sound:getById', (_event, id: string) => {
    try {
      const giftSound = mainDatabase.getGiftSoundById(id);
      return { success: true, giftSound };
    } catch (error) {
      return { success: false, error: (error as Error).message };
    }
  });

  ipcMain.handle('gift-sound:getByGiftId', (_event, giftId: string) => {
    try {
      const giftSound = mainDatabase.getGiftSoundByGiftId(giftId);
      return { success: true, giftSound };
    } catch (error) {
      return { success: false, error: (error as Error).message };
    }
  });

  ipcMain.handle('gift-sound:add', (_event, giftSound: GiftSoundConfig) => {
    try {
      mainDatabase.addGiftSound(giftSound);
      return { success: true };
    } catch (error) {
      return { success: false, error: (error as Error).message };
    }
  });

  ipcMain.handle('gift-sound:update', (_event, id: string, updates: Partial<GiftSoundConfig>) => {
    try {
      mainDatabase.updateGiftSound(id, updates);
      return { success: true };
    } catch (error) {
      return { success: false, error: (error as Error).message };
    }
  });

  ipcMain.handle('gift-sound:delete', (_event, id: string) => {
    try {
      mainDatabase.deleteGiftSound(id);
      return { success: true };
    } catch (error) {
      return { success: false, error: (error as Error).message };
    }
  });

  // ===== 音效播放 =====
  ipcMain.handle('gift-sound:play', async (_event, request: PlaySoundRequest) => {
    try {
      // 查找礼物音效配置
      const giftSound = mainDatabase.getGiftSoundByGiftId(request.giftId);
      if (!giftSound) {
        return { success: false, error: '未找到礼物音效配置' };
      }

      // 发送播放命令到渲染进程
      if (mainWindow) {
        mainWindow.webContents.send('gift-sound:playCommand', {
          giftSound,
          request
        });
      }

      // 记录播放日志
      mainDatabase.addSoundLog({
        id: Date.now().toString(),
        giftId: request.giftId,
        giftName: request.giftName,
        username: request.username,
        soundFile: giftSound.soundFile,
        playedAt: new Date()
      });

      return { success: true, soundFile: giftSound.soundFile };
    } catch (error) {
      return { success: false, error: (error as Error).message };
    }
  });

  ipcMain.handle('gift-sound:stopAll', () => {
    try {
      // 发送停止命令到渲染进程
      if (mainWindow) {
        mainWindow.webContents.send('gift-sound:stopAllCommand');
      }
      return { success: true };
    } catch (error) {
      return { success: false, error: (error as Error).message };
    }
  });

  ipcMain.handle('gift-sound:preview', (_event, soundFile: string, volume: number) => {
    try {
      // 发送预览命令到渲染进程
      if (mainWindow) {
        mainWindow.webContents.send('gift-sound:previewCommand', {
          soundFile,
          volume
        });
      }
      return { success: true };
    } catch (error) {
      return { success: false, error: (error as Error).message };
    }
  });

  // ===== 导入导出 =====
  ipcMain.handle('gift-sound:import', (_event, configs: GiftSoundConfig[]) => {
    try {
      configs.forEach(config => {
        mainDatabase.addGiftSound(config);
      });
      return { success: true };
    } catch (error) {
      return { success: false, error: (error as Error).message };
    }
  });

  ipcMain.handle('gift-sound:export', () => {
    try {
      const giftSounds = mainDatabase.getGiftSounds();
      return { success: true, giftSounds };
    } catch (error) {
      return { success: false, error: (error as Error).message };
    }
  });

  // ===== 播放日志 =====
  ipcMain.handle('gift-sound:getLogs', () => {
    try {
      const logs = mainDatabase.getSoundLogs();
      return { success: true, logs };
    } catch (error) {
      return { success: false, error: (error as Error).message };
    }
  });

  ipcMain.handle('gift-sound:clearLogs', () => {
    try {
      mainDatabase.clearSoundLogs();
      return { success: true };
    } catch (error) {
      return { success: false, error: (error as Error).message };
    }
  });

  // ===== 事件处理（从渲染进程接收） =====
  ipcMain.on('gift-sound:playComplete', (_event, data) => {
    // 播放完成事件处理
    console.log('音效播放完成:', data);
  });

  ipcMain.on('gift-sound:playError', (_event, error) => {
    // 播放错误事件处理
    console.error('音效播放错误:', error);
  });
}

/**
 * 清理礼物音效模块
 */
export function cleanupGiftSoundModule(): void {
  // 移除所有 IPC 处理器
  ipcMain.removeHandler('gift-sound:getAll');
  ipcMain.removeHandler('gift-sound:getById');
  ipcMain.removeHandler('gift-sound:getByGiftId');
  ipcMain.removeHandler('gift-sound:add');
  ipcMain.removeHandler('gift-sound:update');
  ipcMain.removeHandler('gift-sound:delete');
  ipcMain.removeHandler('gift-sound:play');
  ipcMain.removeHandler('gift-sound:stopAll');
  ipcMain.removeHandler('gift-sound:preview');
  ipcMain.removeHandler('gift-sound:import');
  ipcMain.removeHandler('gift-sound:export');
  ipcMain.removeHandler('gift-sound:getLogs');
  ipcMain.removeHandler('gift-sound:clearLogs');
}
