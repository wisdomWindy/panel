/**
 * 数据库服务（渲染进程 IPC 客户端）
 * 通过 IPC 调用主进程的 electron-store 数据库
 */

import { Logger } from '../modules/utils/logger';
import { DanmakuPoolItem, Gift, CaptainInfo, LogEntry } from '../modules/danmaku/types';
import { EffectConfig } from '../modules/effects/types';

interface AppConfig {
  roomId: number;
  csrfToken: string;
  minSendInterval: number;
  enableAutoSend: boolean;
  autoSendInterval: number;
  enableSensitiveWordFilter: boolean;
  sensitiveWords: string[];
  replaceChar: string;
  enableLog: boolean;
  maxLogEntries: number;
  enableEffects: boolean;
  effectVolume: number;
}

interface ElectronAPI {
  invoke: (channel: string, ...args: unknown[]) => Promise<unknown>;
  on: (channel: string, callback: (...args: unknown[]) => void) => void;
  off: (channel: string, callback: (...args: unknown[]) => void) => void;
}

// 获取 preload 暴露的 API
function getApi(): ElectronAPI {
  return (window as { electronAPI: ElectronAPI }).electronAPI;
}

/**
 * 数据库服务（渲染进程侧）
 * 所有操作通过 IPC 代理到主进程
 */
export class DatabaseService {
  private logger: Logger;
  private api = getApi();

  constructor() {
    this.logger = new Logger('DatabaseService');
  }

  private async invoke<T>(channel: string, ...args: unknown[]): Promise<T> {
    return this.api.invoke(channel, ...args) as Promise<T>;
  }

  // ===== 弹幕池 =====
  async getDanmakuPool(): Promise<DanmakuPoolItem[]> {
    return this.invoke('db:getDanmakuPool');
  }

  async setDanmakuPool(pool: DanmakuPoolItem[]): Promise<void> {
    await this.invoke('db:setDanmakuPool', pool);
    this.logger.info('弹幕池已更新');
  }

  async addDanmakuToPool(item: DanmakuPoolItem): Promise<void> {
    await this.invoke('db:addDanmakuToPool', item);
  }

  async removeDanmakuFromPool(id: number): Promise<void> {
    await this.invoke('db:removeDanmakuFromPool', id);
  }

  async updateDanmakuPoolItem(id: number, updates: Partial<DanmakuPoolItem>): Promise<void> {
    await this.invoke('db:updateDanmakuPoolItem', id, updates);
  }

  // ===== 礼物 =====
  async getGifts(): Promise<Gift[]> {
    return this.invoke('db:getGifts');
  }

  async addGift(gift: Gift): Promise<void> {
    await this.invoke('db:addGift', gift);
    this.logger.info(`添加礼物记录: ${gift.username} 送了 ${gift.giftName}`);
  }

  // ===== 舰长 =====
  async getCaptains(): Promise<CaptainInfo[]> {
    return this.invoke('db:getCaptains');
  }

  async addCaptain(captain: CaptainInfo): Promise<void> {
    await this.invoke('db:addCaptain', captain);
    this.logger.info(`添加舰长记录: ${captain.username} 开通了 ${captain.level}`);
  }

  // ===== 日志 =====
  async getLogs(): Promise<LogEntry[]> {
    return this.invoke('db:getLogs');
  }

  async addLog(log: LogEntry): Promise<void> {
    await this.invoke('db:addLog', log);
  }

  async clearLogs(): Promise<void> {
    await this.invoke('db:clearLogs');
    this.logger.info('日志已清除');
  }

  // ===== 特效配置 =====
  async getEffectConfigs(): Promise<EffectConfig[]> {
    try {
      const raw = localStorage.getItem('effectConfigs');
      return raw ? JSON.parse(raw) : [];
    } catch {
      return [];
    }
  }

  async addEffectConfig(config: EffectConfig): Promise<void> {
    const configs = await this.getEffectConfigs();
    configs.push(config);
    localStorage.setItem('effectConfigs', JSON.stringify(configs));
    this.logger.info(`添加特效配置: ${config.id}`);
  }

  async updateEffectConfig(id: string, updates: Partial<EffectConfig>): Promise<void> {
    const configs = await this.getEffectConfigs();
    const index = configs.findIndex(c => c.id === id);
    if (index !== -1) {
      configs[index] = { ...configs[index], ...updates, updatedAt: new Date() };
      localStorage.setItem('effectConfigs', JSON.stringify(configs));
    }
  }

  async deleteEffectConfig(id: string): Promise<void> {
    const configs = await this.getEffectConfigs();
    localStorage.setItem('effectConfigs', JSON.stringify(configs.filter(c => c.id !== id)));
  }

  // ===== 配置 =====
  async getConfig(): Promise<AppConfig> {
    return this.invoke('db:getConfig');
  }

  async setConfig(config: Partial<AppConfig>): Promise<void> {
    await this.invoke('db:setConfig', config);
    this.logger.info('应用配置已更新');
  }

  // ===== 工具 =====
  async reset(): Promise<void> {
    await this.invoke('db:reset');
    this.logger.info('所有数据已重置');
  }

  async backup(): Promise<string> {
    return this.invoke('db:backup');
  }

  async restore(data: string): Promise<void> {
    await this.invoke('db:restore', data);
    this.logger.info('数据已恢复');
  }
}

// 创建单例实例
export const databaseService = new DatabaseService();
