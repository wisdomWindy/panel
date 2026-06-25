/**
 * 日志工具
 * 渲染进程兼容实现（不依赖 Node.js 原生模块）
 */

/**
 * 日志级别
 */
export type LogLevel = 'error' | 'warn' | 'info' | 'debug';

/**
 * 日志条目
 */
export interface LogEntry {
  id: string;
  timestamp: number;
  level: LogLevel;
  message: string;
  module?: string;
  data?: unknown;
}

const LOG_LEVELS: Record<LogLevel, number> = {
  error: 0,
  warn: 1,
  info: 2,
  debug: 3
};

/**
 * 日志器类
 */
export class Logger {
  private moduleName: string;
  private level: LogLevel = 'info';

  constructor(moduleName: string) {
    this.moduleName = moduleName;
  }

  private shouldLog(level: LogLevel): boolean {
    return LOG_LEVELS[level] <= LOG_LEVELS[this.level];
  }

  private formatTimestamp(): string {
    return new Date().toISOString();
  }

  private log(level: LogLevel, message: string, data?: unknown): void {
    if (!this.shouldLog(level)) return;

    const timestamp = this.formatTimestamp();
    const prefix = `[${timestamp}] [${this.moduleName}] [${level.toUpperCase()}]`;

    const consoleMethod = level === 'error' ? console.error
      : level === 'warn' ? console.warn
        : level === 'debug' ? console.debug
          : console.log;

    if (data !== undefined) {
      consoleMethod(`${prefix} ${message}`, data);
    } else {
      consoleMethod(`${prefix} ${message}`);
    }
  }

  /**
   * 记录错误日志
   */
  error(message: string, data?: unknown): void {
    this.log('error', message, data);
  }

  /**
   * 记录警告日志
   */
  warn(message: string, data?: unknown): void {
    this.log('warn', message, data);
  }

  /**
   * 记录信息日志
   */
  info(message: string, data?: unknown): void {
    this.log('info', message, data);
  }

  /**
   * 记录调试日志
   */
  debug(message: string, data?: unknown): void {
    this.log('debug', message, data);
  }

  /**
   * 设置日志级别
   */
  setLevel(level: LogLevel): void {
    this.level = level;
  }

  /**
   * 获取日志级别
   */
  getLevel(): string {
    return this.level;
  }

  /**
   * 创建新的日志器实例
   */
  static create(moduleName: string): Logger {
    return new Logger(moduleName);
  }
}
