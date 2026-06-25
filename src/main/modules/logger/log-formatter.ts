/**
 * 日志格式化器模块
 * 实现日志格式化功能，支持结构化日志格式
 */

import type { LogEntry, LogLevel } from './logger.js';

export interface FormatOptions {
  includeTimestamp: boolean;
  includeLevel: boolean;
  includeModule: boolean;
  includeContext: boolean;
  includeMetadata: boolean;
  colorize: boolean;
}

const DEFAULT_FORMAT_OPTIONS: FormatOptions = {
  includeTimestamp: true,
  includeLevel: true,
  includeModule: true,
  includeContext: true,
  includeMetadata: true,
  colorize: true
};

const LEVEL_COLORS: Record<LogLevel, string> = {
  debug: '\x1b[36m', // Cyan
  info: '\x1b[32m',  // Green
  warn: '\x1b[33m',  // Yellow
  error: '\x1b[31m'  // Red
};

const RESET_COLOR = '\x1b[0m';

export class LogFormatter {
  private options: FormatOptions;

  constructor(options?: Partial<FormatOptions>) {
    this.options = { ...DEFAULT_FORMAT_OPTIONS, ...options };
  }

  /**
   * 更新格式化选项
   */
  updateOptions(options: Partial<FormatOptions>): void {
    this.options = { ...this.options, ...options };
  }

  /**
   * 格式化时间戳
   */
  private formatTimestamp(timestamp: Date): string {
    return timestamp.toISOString();
  }

  /**
   * 格式化日志级别
   */
  private formatLevel(level: LogLevel): string {
    return level.toUpperCase().padEnd(5);
  }

  /**
   * 应用颜色
   */
  private applyColor(text: string, level: LogLevel): string {
    if (!this.options.colorize) {
      return text;
    }
    return `${LEVEL_COLORS[level]}${text}${RESET_COLOR}`;
  }

  /**
   * 格式化控制台输出
   */
  formatConsole(entry: LogEntry): string {
    const parts: string[] = [];

    // 时间戳
    if (this.options.includeTimestamp) {
      parts.push(`[${this.formatTimestamp(entry.timestamp)}]`);
    }

    // 日志级别
    if (this.options.includeLevel) {
      const level = this.formatLevel(entry.level);
      parts.push(this.applyColor(`[${level}]`, entry.level));
    }

    // 模块名
    if (this.options.includeModule) {
      parts.push(`[${entry.module}]`);
    }

    // 消息
    parts.push(entry.message);

    // 错误信息
    if (entry.error) {
      parts.push(`\nError: ${entry.error.type}: ${entry.error.message}`);
      if (entry.error.stack) {
        parts.push(`\nStack: ${entry.error.stack}`);
      }
    }

    // 上下文
    if (this.options.includeContext && entry.context) {
      parts.push(`\nContext: ${JSON.stringify(entry.context, null, 2)}`);
    }

    // 元数据
    if (this.options.includeMetadata && entry.metadata) {
      parts.push(`\nMetadata: ${JSON.stringify(entry.metadata, null, 2)}`);
    }

    return parts.join(' ');
  }

  /**
   * 格式化为JSON
   */
  formatJSON(entry: LogEntry): string {
    const obj: Record<string, unknown> = {
      id: entry.id,
      timestamp: entry.timestamp.toISOString(),
      level: entry.level,
      module: entry.module,
      message: entry.message
    };

    if (entry.error) {
      obj.error = entry.error;
    }

    if (entry.context) {
      obj.context = entry.context;
    }

    if (entry.metadata) {
      obj.metadata = entry.metadata;
    }

    return JSON.stringify(obj);
  }

  /**
   * 格式化为CSV行
   */
  formatCSV(entry: LogEntry): string {
    const escapeCSV = (value: string): string => {
      if (value.includes(',') || value.includes('"') || value.includes('\n')) {
        return `"${value.replace(/"/g, '""')}"`;
      }
      return value;
    };

    const fields = [
      entry.id,
      entry.timestamp.toISOString(),
      entry.level,
      entry.module,
      escapeCSV(entry.message),
      entry.error ? escapeCSV(`${entry.error.type}: ${entry.error.message}`) : '',
      entry.context ? escapeCSV(JSON.stringify(entry.context)) : '',
      entry.metadata ? escapeCSV(JSON.stringify(entry.metadata)) : ''
    ];

    return fields.join(',');
  }

  /**
   * 获取CSV头
   */
  getCSVHeader(): string {
    return 'id,timestamp,level,module,message,error,context,metadata';
  }

  /**
   * 格式化为纯文本
   */
  formatText(entry: LogEntry): string {
    const parts: string[] = [];

    parts.push(`[${this.formatTimestamp(entry.timestamp)}]`);
    parts.push(`[${this.formatLevel(entry.level)}]`);
    parts.push(`[${entry.module}]`);
    parts.push(entry.message);

    if (entry.error) {
      parts.push(`\nError: ${entry.error.type}: ${entry.error.message}`);
      if (entry.error.stack) {
        parts.push(`\nStack: ${entry.error.stack}`);
      }
    }

    if (entry.context) {
      parts.push(`\nContext: ${JSON.stringify(entry.context)}`);
    }

    if (entry.metadata) {
      parts.push(`\nMetadata: ${JSON.stringify(entry.metadata)}`);
    }

    return parts.join(' ');
  }

  /**
   * 批量格式化为JSON
   */
  formatJSONBatch(entries: LogEntry[]): string {
    return JSON.stringify(entries.map(entry => ({
      id: entry.id,
      timestamp: entry.timestamp.toISOString(),
      level: entry.level,
      module: entry.module,
      message: entry.message,
      error: entry.error,
      context: entry.context,
      metadata: entry.metadata
    })));
  }

  /**
   * 批量格式化为CSV
   */
  formatCSVBatch(entries: LogEntry[]): string {
    const lines = [this.getCSVHeader()];
    entries.forEach(entry => {
      lines.push(this.formatCSV(entry));
    });
    return lines.join('\n');
  }
}