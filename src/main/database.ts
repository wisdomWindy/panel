/**
 * 数据库服务（主进程）
 * 使用 electron-store 进行数据持久化
 */

import type { LogEntry } from '../renderer/modules/danmaku/types.js';

interface DanmakuPoolItem {
  id: number;
  content: string;
  type: 'timed' | 'random' | 'trigger';
  triggerWord?: string;
  enabled: boolean;
  priority: number;
  interval?: number;
  createdAt: Date;
  updatedAt: Date;
}

interface Gift {
  id: string;
  giftId: string;
  giftName: string;
  userId: string;
  username: string;
  count: number;
  price: number;
  timestamp: number;
  isCombo: boolean;
  comboId?: string;
}

interface CaptainInfo {
  userId: string;
  username: string;
  level: number;
  title: string;
}

interface AppData {
  danmakuPool: DanmakuPoolItem[];
  gifts: Gift[];
  captains: CaptainInfo[];
  logs: LogEntry[];
  config: {
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
    enableAI: boolean;
    aiModel: string;
    aiApiKey: string;
    aiContextWindow: number;
    aiMinInterval: number;
  };
}

export class MainDatabaseService {
  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  private store: any = null;
  private initialized = false;

  async initialize(): Promise<void> {
    if (this.initialized) return;

    // 动态导入 electron-store (ESM 模块)
    const StoreModule = await import('electron-store');
    const Store = StoreModule.default;

    this.store = new Store({
      defaults: {
        danmakuPool: [],
        gifts: [],
        captains: [],
        logs: [],
        config: {
          roomId: 0,
          csrfToken: '',
          minSendInterval: 3000,
          enableAutoSend: false,
          autoSendInterval: 5000,
          enableSensitiveWordFilter: true,
          sensitiveWords: ['脏话', '广告'],
          replaceChar: '*',
          enableLog: true,
          maxLogEntries: 1000,
          enableEffects: true,
          effectVolume: 0.7,
          enableAI: false,
          aiModel: 'gpt-3.5-turbo',
          aiApiKey: '',
          aiContextWindow: 10,
          aiMinInterval: 30
        }
      }
    });

    this.initialized = true;
  }

  private ensureInitialized(): void {
    if (!this.initialized || !this.store) {
      throw new Error('Database not initialized. Call initialize() first.');
    }
  }

  // ===== 弹幕池操作 =====
  getDanmakuPool(): DanmakuPoolItem[] {
    this.ensureInitialized();
    return this.store.get('danmakuPool', []) as DanmakuPoolItem[];
  }

  setDanmakuPool(pool: DanmakuPoolItem[]): void {
    this.ensureInitialized();
    this.store.set('danmakuPool', pool);
  }

  addDanmakuToPool(item: DanmakuPoolItem): void {
    const pool = this.getDanmakuPool();
    pool.push(item);
    this.setDanmakuPool(pool);
  }

  removeDanmakuFromPool(id: number): void {
    const pool = this.getDanmakuPool().filter(item => item.id !== id);
    this.setDanmakuPool(pool);
  }

  updateDanmakuPoolItem(id: number, updates: Partial<DanmakuPoolItem>): void {
    const pool = this.getDanmakuPool();
    const index = pool.findIndex(item => item.id === id);
    if (index !== -1) {
      pool[index] = { ...pool[index], ...updates, updatedAt: new Date() };
      this.setDanmakuPool(pool);
    }
  }

  // ===== 礼物记录 =====
  getGifts(): Gift[] {
    this.ensureInitialized();
    return this.store.get('gifts', []) as Gift[];
  }

  addGift(gift: Gift): void {
    this.ensureInitialized();
    const gifts = this.getGifts();
    gifts.push(gift);
    this.store.set('gifts', gifts);
  }

  // ===== 舰长记录 =====
  getCaptains(): CaptainInfo[] {
    this.ensureInitialized();
    return this.store.get('captains', []) as CaptainInfo[];
  }

  addCaptain(captain: CaptainInfo): void {
    this.ensureInitialized();
    const captains = this.getCaptains();
    captains.push(captain);
    this.store.set('captains', captains);
  }

  // ===== 日志 =====
  getLogs(): LogEntry[] {
    this.ensureInitialized();
    return this.store.get('logs', []) as LogEntry[];
  }

  addLog(log: LogEntry): void {
    this.ensureInitialized();
    const logs = this.getLogs();
    logs.push(log);
    const maxLogs = this.store.get('config.maxLogEntries', 1000);
    if (logs.length > maxLogs) {
      logs.splice(0, logs.length - maxLogs);
    }
    this.store.set('logs', logs);
  }

  clearLogs(): void {
    this.ensureInitialized();
    this.store.set('logs', []);
  }

  // ===== 配置 =====
  getConfig(): AppData['config'] {
    this.ensureInitialized();
    return this.store.get('config') as AppData['config'];
  }

  setConfig(config: Partial<AppData['config']>): void {
    this.ensureInitialized();
    const currentConfig = this.getConfig();
    this.store.set('config', { ...currentConfig, ...config });
  }

  // ===== 工具 =====
  reset(): void {
    this.ensureInitialized();
    this.store.clear();
  }

  backup(): string {
    this.ensureInitialized();
    return JSON.stringify(this.store.store, null, 2);
  }

  restore(data: string): void {
    this.ensureInitialized();
    const parsed = JSON.parse(data);
    this.store.store = parsed;
  }
}

// 单例
export const mainDatabase = new MainDatabaseService();
