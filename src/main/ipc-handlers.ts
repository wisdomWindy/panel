/**
 * IPC 处理器
 * 注册主进程 IPC 通道，供渲染进程调用
 */

import { ipcMain } from 'electron';
import { mainDatabase } from './database.js';
import { startProject, endProject, heartbeat, batchHeartbeat } from './bilibili-api.js';

export async function registerIpcHandlers(): Promise<void> {
  // 初始化数据库
  await mainDatabase.initialize();

  // ===== Bilibili API =====
  ipcMain.handle('bilibili:start', async (_event, params: {
    accessKeyId: string;
    accessKeySecret: string;
    code: string;
    appId: number;
  }) => {
    return await startProject(
      params.accessKeyId,
      params.accessKeySecret,
      params.code,
      params.appId
    );
  });

  ipcMain.handle('bilibili:end', async (_event, params: {
    accessKeyId: string;
    accessKeySecret: string;
    appId: number;
    gameId: string;
  }) => {
    return await endProject(
      params.accessKeyId,
      params.accessKeySecret,
      params.appId,
      params.gameId
    );
  });

  ipcMain.handle('bilibili:heartbeat', async (_event, params: {
    accessKeyId: string;
    accessKeySecret: string;
    gameId: string;
  }) => {
    return await heartbeat(
      params.accessKeyId,
      params.accessKeySecret,
      params.gameId
    );
  });

  ipcMain.handle('bilibili:batchHeartbeat', async (_event, params: {
    accessKeyId: string;
    accessKeySecret: string;
    gameIds: string[];
  }) => {
    return await batchHeartbeat(
      params.accessKeyId,
      params.accessKeySecret,
      params.gameIds
    );
  });

  // ===== 弹幕池 =====
  ipcMain.handle('db:getDanmakuPool', () => {
    return mainDatabase.getDanmakuPool();
  });

  ipcMain.handle('db:setDanmakuPool', (_event, pool) => {
    mainDatabase.setDanmakuPool(pool);
  });

  ipcMain.handle('db:addDanmakuToPool', (_event, item) => {
    mainDatabase.addDanmakuToPool(item);
  });

  ipcMain.handle('db:removeDanmakuFromPool', (_event, id: number) => {
    mainDatabase.removeDanmakuFromPool(id);
  });

  ipcMain.handle('db:updateDanmakuPoolItem', (_event, id: number, updates) => {
    mainDatabase.updateDanmakuPoolItem(id, updates);
  });

  // ===== 礼物 =====
  ipcMain.handle('db:getGifts', () => {
    return mainDatabase.getGifts();
  });

  ipcMain.handle('db:addGift', (_event, gift) => {
    mainDatabase.addGift(gift);
  });

  // ===== 舰长 =====
  ipcMain.handle('db:getCaptains', () => {
    return mainDatabase.getCaptains();
  });

  ipcMain.handle('db:addCaptain', (_event, captain) => {
    mainDatabase.addCaptain(captain);
  });

  // ===== 日志 =====
  ipcMain.handle('db:getLogs', () => {
    return mainDatabase.getLogs();
  });

  ipcMain.handle('db:addLog', (_event, log) => {
    mainDatabase.addLog(log);
  });

  ipcMain.handle('db:clearLogs', () => {
    mainDatabase.clearLogs();
  });

  // ===== 配置 =====
  ipcMain.handle('db:getConfig', () => {
    return mainDatabase.getConfig();
  });

  ipcMain.handle('db:setConfig', (_event, config) => {
    mainDatabase.setConfig(config);
  });

  // ===== 工具 =====
  ipcMain.handle('db:reset', () => {
    mainDatabase.reset();
  });

  ipcMain.handle('db:backup', () => {
    return mainDatabase.backup();
  });

  ipcMain.handle('db:restore', (_event, data: string) => {
    mainDatabase.restore(data);
  });
}
