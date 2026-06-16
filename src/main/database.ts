/**
 * 数据库服务（主进程）
 * 使用 electron-store 进行数据持久化
 */

import type { LogEntry } from '../renderer/modules/danmaku/types.js';
import type {
  Recording,
  Highlight,
  RecordingConfig
} from '../renderer/modules/recording/types.js';

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
  recordings: Recording[];
  highlights: Highlight[];
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
    recording: RecordingConfig;
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
        recordings: [],
        highlights: [],
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
          aiMinInterval: 30,
          recording: {
            enabled: false,
            autoStart: false,
            format: 'mp4',
            quality: 'original',
            segmentSize: 0,
            segmentDuration: 0,
            outputPath: '',
            maxStorageSize: 0,
            autoCleanup: false,
            cleanupDays: 30,
            highlightEnabled: true,
            highlightConfig: {
              danmakuThreshold: 30,
              danmakuWindow: 10,
              giftValueThreshold: 100,
              captainTrigger: true,
              clipDuration: 30,
              beforeSeconds: 15,
              afterSeconds: 15,
              cooldownSeconds: 10
            }
          }
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

  // ===== 录制配置 =====
  getRecordingConfig(): RecordingConfig {
    this.ensureInitialized();
    return this.store.get('config.recording') as RecordingConfig;
  }

  setRecordingConfig(config: Partial<RecordingConfig>): void {
    this.ensureInitialized();
    const currentConfig = this.getRecordingConfig();
    this.store.set('config.recording', { ...currentConfig, ...config });
  }

  // ===== 录制记录 =====
  getRecordings(): Recording[] {
    this.ensureInitialized();
    return this.store.get('recordings', []) as Recording[];
  }

  getRecordingById(id: string): Recording | undefined {
    this.ensureInitialized();
    const recordings = this.getRecordings();
    return recordings.find(r => r.id === id);
  }

  addRecording(recording: Recording): void {
    this.ensureInitialized();
    const recordings = this.getRecordings();
    recordings.unshift(recording);
    this.store.set('recordings', recordings);
  }

  updateRecording(id: string, updates: Partial<Recording>): void {
    this.ensureInitialized();
    const recordings = this.getRecordings();
    const index = recordings.findIndex(r => r.id === id);
    if (index !== -1) {
      recordings[index] = { ...recordings[index], ...updates };
      this.store.set('recordings', recordings);
    }
  }

  deleteRecording(id: string): void {
    this.ensureInitialized();
    const recordings = this.getRecordings().filter(r => r.id !== id);
    this.store.set('recordings', recordings);
  }

  // ===== 高光时刻 =====
  getHighlights(recordingId?: string): Highlight[] {
    this.ensureInitialized();
    const highlights = this.store.get('highlights', []) as Highlight[];
    if (recordingId) {
      return highlights.filter(h => h.recordingId === recordingId);
    }
    return highlights;
  }

  getHighlightById(id: string): Highlight | undefined {
    this.ensureInitialized();
    const highlights = this.getHighlights();
    return highlights.find(h => h.id === id);
  }

  addHighlight(highlight: Highlight): void {
    this.ensureInitialized();
    const highlights = this.getHighlights();
    highlights.unshift(highlight);
    this.store.set('highlights', highlights);
  }

  deleteHighlight(id: string): void {
    this.ensureInitialized();
    const highlights = this.getHighlights().filter(h => h.id !== id);
    this.store.set('highlights', highlights);
  }

  deleteHighlightsByRecordingId(recordingId: string): void {
    this.ensureInitialized();
    const highlights = this.getHighlights().filter(h => h.recordingId !== recordingId);
    this.store.set('highlights', highlights);
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
