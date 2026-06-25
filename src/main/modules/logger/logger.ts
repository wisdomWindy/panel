/**
 * 日志记录器模块
 * 实现日志记录功能，支持不同日志级别和结构化日志格式
 */

import { LogFormatter } from './log-formatter.js';
import { LogStorage } from './log-storage.js';

export type LogLevel = 'debug' | 'info' | 'warn' | 'error';

export interface LogEntry {
  id: string;
  timestamp: Date;
  level: LogLevel;
  module: string;
  message: string;
  error?: {
    type: string;
    message: string;
    stack?: string;
  };
  context?: Record<string, unknown>;
  metadata?: Record<string, unknown>;
}

export interface LoggerConfig {
  level: LogLevel;
  enableConsole: boolean;
  enableStorage: boolean;
  maxEntries: number;
}

const LOG_LEVELS: Record<LogLevel, number> = {
  debug: 0,
  info: 1,
  warn: 2,
  error: 3
};

export class Logger {
  private static instance: Logger;
  private config: LoggerConfig;
  private formatter: LogFormatter;
  private storage: LogStorage | null = null;
  private logQueue: LogEntry[] = [];
  private isProcessing = false;

  private constructor(config?: Partial<LoggerConfig>) {
    this.config = {
      level: 'info',
      enableConsole: true,
      enableStorage: true,
      maxEntries: 10000,
      ...config
    };
    this.formatter = new LogFormatter();
  }

  /**
   * 获取Logger单例
   */
  static getInstance(config?: Partial<LoggerConfig>): Logger {
    if (!Logger.instance) {
      Logger.instance = new Logger(config);
    }
    return Logger.instance;
  }

  /**
   * 初始化存储模块
   */
  async initStorage(logDir: string): Promise<void> {
    if (this.config.enableStorage) {
      this.storage = new LogStorage(logDir);
      await this.storage.init();
    }
  }

  /**
   * 更新配置
   */
  updateConfig(config: Partial<LoggerConfig>): void {
    this.config = { ...this.config, ...config };
  }

  /**
   * 检查日志级别是否应该记录
   */
  private shouldLog(level: LogLevel): boolean {
    return LOG_LEVELS[level] >= LOG_LEVELS[this.config.level];
  }

  /**
   * 生成唯一ID
   */
  private generateId(): string {
    return `${Date.now()}-${Math.random().toString(36).substr(2, 9)}`;
  }

  /**
   * 记录日志
   */
  private async log(
    level: LogLevel,
    module: string,
    message: string,
    error?: Error,
    context?: Record<string, unknown>,
    metadata?: Record<string, unknown>
  ): Promise<void> {
    if (!this.shouldLog(level)) {
      return;
    }

    const entry: LogEntry = {
      id: this.generateId(),
      timestamp: new Date(),
      level,
      module,
      message,
      error: error ? {
        type: error.constructor.name,
        message: error.message,
        stack: error.stack
      } : undefined,
      context,
      metadata
    };

    // 添加到队列
    this.logQueue.push(entry);

    // 控制台输出
    if (this.config.enableConsole) {
      this.consoleLog(entry);
    }

    // 异步处理存储
    if (this.config.enableStorage && this.storage) {
      this.processQueue();
    }
  }

  /**
   * 控制台输出
   */
  private consoleLog(entry: LogEntry): void {
    const formatted = this.formatter.formatConsole(entry);
    
    switch (entry.level) {
      case 'debug':
        console.debug(formatted);
        break;
      case 'info':
        console.info(formatted);
        break;
      case 'warn':
        console.warn(formatted);
        break;
      case 'error':
        console.error(formatted);
        break;
    }
  }

  /**
   * 异步处理日志队列
   */
  private async processQueue(): Promise<void> {
    if (this.isProcessing || this.logQueue.length === 0) {
      return;
    }

    this.isProcessing = true;

    try {
      const entries = [...this.logQueue];
      this.logQueue = [];

      if (this.storage) {
        await this.storage.writeEntries(entries);
      }
    } catch (error) {
      console.error('日志存储失败:', error);
    } finally {
      this.isProcessing = false;

      // 如果队列中还有数据，继续处理
      if (this.logQueue.length > 0) {
        this.processQueue();
      }
    }
  }

  /**
   * 记录DEBUG级别日志
   */
  async debug(
    module: string,
    message: string,
    context?: Record<string, unknown>,
    metadata?: Record<string, unknown>
  ): Promise<void> {
    await this.log('debug', module, message, undefined, context, metadata);
  }

  /**
   * 记录INFO级别日志
   */
  async info(
    module: string,
    message: string,
    context?: Record<string, unknown>,
    metadata?: Record<string, unknown>
  ): Promise<void> {
    await this.log('info', module, message, undefined, context, metadata);
  }

  /**
   * 记录WARN级别日志
   */
  async warn(
    module: string,
    message: string,
    context?: Record<string, unknown>,
    metadata?: Record<string, unknown>
  ): Promise<void> {
    await this.log('warn', module, message, undefined, context, metadata);
  }

  /**
   * 记录ERROR级别日志
   */
  async error(
    module: string,
    message: string,
    error?: Error,
    context?: Record<string, unknown>,
    metadata?: Record<string, unknown>
  ): Promise<void> {
    await this.log('error', module, message, error, context, metadata);
  }

  /**
   * 获取日志队列长度
   */
  getQueueLength(): number {
    return this.logQueue.length;
  }

  /**
   * 清空日志队列
   */
  clearQueue(): void {
    this.logQueue = [];
  }

  /**
   * 获取配置
   */
  getConfig(): LoggerConfig {
    return { ...this.config };
  }
}

// 导出默认实例
export const logger = Logger.getInstance();