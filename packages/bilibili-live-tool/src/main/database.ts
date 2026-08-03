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

interface GiftSoundConfig {
  id: string;
  giftId: string;
  giftName: string;
  soundFile: string;
  soundName: string;
  volume: number;
  duration: number;
  playMode: 'once' | 'loop';
  cooldown: number;
  priority: number;
  enabled: boolean;
  createdAt: Date;
  updatedAt: Date;
}

interface SoundLog {
  id: string;
  giftId: string;
  giftName: string;
  username: string;
  soundFile: string;
  playedAt: Date;
}

interface AppData {
  gifts: Gift[];
  captains: CaptainInfo[];
  logs: LogEntry[];
  recordings: Recording[];
  highlights: Highlight[];
  giftSounds: GiftSoundConfig[];
  soundLogs: SoundLog[];
  config: {
    roomId: number;
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
    recording: RecordingConfig;
  };
}

const DEFAULT_APP_CONFIG: AppData['config'] = {
  roomId: 0,
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
};

function mergeAppDataConfig(
  current: AppData['config'],
  updates: unknown
): AppData['config'] {
  if (!updates || typeof updates !== 'object') {
    return current;
  }

  const source = updates as Record<string, unknown>;
  const readString = (key: keyof AppData['config'], fallback: string) =>
    typeof source[key] === 'string' ? source[key] : fallback;
  const readNumber = (key: keyof AppData['config'], fallback: number) =>
    typeof source[key] === 'number' ? source[key] : fallback;
  const readBoolean = (key: keyof AppData['config'], fallback: boolean) =>
    typeof source[key] === 'boolean' ? source[key] : fallback;

  return {
    roomId: readNumber('roomId', current.roomId),
    enableSensitiveWordFilter: readBoolean(
      'enableSensitiveWordFilter',
      current.enableSensitiveWordFilter
    ),
    sensitiveWords: Array.isArray(source.sensitiveWords)
      ? source.sensitiveWords.filter((word): word is string => typeof word === 'string')
      : current.sensitiveWords,
    replaceChar: readString('replaceChar', current.replaceChar),
    enableLog: readBoolean('enableLog', current.enableLog),
    maxLogEntries: readNumber('maxLogEntries', current.maxLogEntries),
    enableEffects: readBoolean('enableEffects', current.enableEffects),
    effectVolume: readNumber('effectVolume', current.effectVolume),
    enableAI: readBoolean('enableAI', current.enableAI),
    aiModel: readString('aiModel', current.aiModel),
    aiApiKey: readString('aiApiKey', current.aiApiKey),
    aiContextWindow: readNumber('aiContextWindow', current.aiContextWindow),
    recording:
      source.recording && typeof source.recording === 'object'
        ? { ...current.recording, ...(source.recording as Partial<RecordingConfig>) }
        : current.recording
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
        gifts: [],
        captains: [],
        logs: [],
        recordings: [],
        highlights: [],
        giftSounds: [],
        soundLogs: [],
        config: DEFAULT_APP_CONFIG
      }
    });

    this.initialized = true;
  }

  private ensureInitialized(): void {
    if (!this.initialized || !this.store) {
      throw new Error('Database not initialized. Call initialize() first.');
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
    return mergeAppDataConfig(DEFAULT_APP_CONFIG, this.store.get('config'));
  }

  setConfig(config: Partial<AppData['config']>): void {
    this.ensureInitialized();
    this.store.set('config', mergeAppDataConfig(this.getConfig(), config));
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

  // ===== 礼物音效配置 =====
  getGiftSounds(): GiftSoundConfig[] {
    this.ensureInitialized();
    return this.store.get('giftSounds', []) as GiftSoundConfig[];
  }

  getGiftSoundById(id: string): GiftSoundConfig | undefined {
    this.ensureInitialized();
    const giftSounds = this.getGiftSounds();
    return giftSounds.find(gs => gs.id === id);
  }

  getGiftSoundByGiftId(giftId: string): GiftSoundConfig | undefined {
    this.ensureInitialized();
    const giftSounds = this.getGiftSounds();
    return giftSounds.find(gs => gs.giftId === giftId && gs.enabled);
  }

  addGiftSound(giftSound: GiftSoundConfig): void {
    this.ensureInitialized();
    const giftSounds = this.getGiftSounds();
    giftSounds.push(giftSound);
    this.store.set('giftSounds', giftSounds);
  }

  updateGiftSound(id: string, updates: Partial<GiftSoundConfig>): void {
    this.ensureInitialized();
    const giftSounds = this.getGiftSounds();
    const index = giftSounds.findIndex(gs => gs.id === id);
    if (index !== -1) {
      giftSounds[index] = { ...giftSounds[index], ...updates, updatedAt: new Date() };
      this.store.set('giftSounds', giftSounds);
    }
  }

  deleteGiftSound(id: string): void {
    this.ensureInitialized();
    const giftSounds = this.getGiftSounds().filter(gs => gs.id !== id);
    this.store.set('giftSounds', giftSounds);
  }

  // ===== 音效播放日志 =====
  getSoundLogs(): SoundLog[] {
    this.ensureInitialized();
    return this.store.get('soundLogs', []) as SoundLog[];
  }

  addSoundLog(soundLog: SoundLog): void {
    this.ensureInitialized();
    const soundLogs = this.getSoundLogs();
    soundLogs.push(soundLog);
    const maxLogs = this.store.get('config.maxLogEntries', 1000);
    if (soundLogs.length > maxLogs) {
      soundLogs.splice(0, soundLogs.length - maxLogs);
    }
    this.store.set('soundLogs', soundLogs);
  }

  clearSoundLogs(): void {
    this.ensureInitialized();
    this.store.set('soundLogs', []);
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
