/**
 * 日志存储模块
 * 实现日志持久化存储功能，支持日志轮转
 */

import * as fs from 'fs/promises';
import * as path from 'path';
import type { LogEntry } from './logger.js';
import { LogFormatter } from './log-formatter.js';

export interface StorageConfig {
  maxFileSize: number;      // 单个文件最大大小（字节）
  maxFiles: number;         // 最大文件数量
  maxAge: number;           // 日志保留天数
  rotationInterval: number; // 轮转间隔（毫秒）
}

const DEFAULT_STORAGE_CONFIG: StorageConfig = {
  maxFileSize: 10 * 1024 * 1024, // 10MB
  maxFiles: 10,
  maxAge: 30,
  rotationInterval: 24 * 60 * 60 * 1000 // 24小时
};

export class LogStorage {
  private logDir: string;
  private config: StorageConfig;
  private formatter: LogFormatter;
  private currentFile: string | null = null;
  private currentSize = 0;
  private rotationTimer: NodeJS.Timeout | null = null;

  constructor(logDir: string, config?: Partial<StorageConfig>) {
    this.logDir = logDir;
    this.config = { ...DEFAULT_STORAGE_CONFIG, ...config };
    this.formatter = new LogFormatter({ colorize: false });
  }

  /**
   * 初始化存储
   */
  async init(): Promise<void> {
    try {
      // 确保日志目录存在
      await fs.mkdir(this.logDir, { recursive: true });

      // 获取当前日志文件
      await this.getCurrentLogFile();

      // 启动定时轮转
      this.startRotationTimer();

      // 清理旧日志
      await this.cleanOldLogs();
    } catch (error) {
      console.error('日志存储初始化失败:', error);
      throw error;
    }
  }

  /**
   * 获取当前日志文件
   */
  private async getCurrentLogFile(): Promise<string> {
    if (this.currentFile) {
      return this.currentFile;
    }

    const date = new Date();
    const dateStr = date.toISOString().split('T')[0];
    const logFile = path.join(this.logDir, `app-${dateStr}.log`);

    try {
      const stats = await fs.stat(logFile);
      this.currentSize = stats.size;
      this.currentFile = logFile;
    } catch {
      // 文件不存在，创建新文件
      this.currentFile = logFile;
      this.currentSize = 0;
      await fs.writeFile(logFile, '', 'utf-8');
    }

    return this.currentFile;
  }

  /**
   * 检查是否需要轮转
   */
  private async checkRotation(): Promise<void> {
    if (this.currentSize >= this.config.maxFileSize) {
      await this.rotate();
    }
  }

  /**
   * 执行日志轮转
   */
  private async rotate(): Promise<void> {
    if (!this.currentFile) {
      return;
    }

    try {
      const timestamp = Date.now();
      const rotatedFile = `${this.currentFile}.${timestamp}`;
      await fs.rename(this.currentFile, rotatedFile);

      // 重置当前文件
      this.currentFile = null;
      this.currentSize = 0;

      // 清理旧文件
      await this.cleanOldFiles();
    } catch (error) {
      console.error('日志轮转失败:', error);
    }
  }

  /**
   * 清理旧日志文件
   */
  private async cleanOldFiles(): Promise<void> {
    try {
      const files = await fs.readdir(this.logDir);
      const logFiles = files.filter(f => f.startsWith('app-') && f.endsWith('.log'));

      // 按修改时间排序
      const fileStats = await Promise.all(
        logFiles.map(async file => {
          const filePath = path.join(this.logDir, file);
          const stats = await fs.stat(filePath);
          return { file, path: filePath, mtime: stats.mtime };
        })
      );

      fileStats.sort((a, b) => b.mtime.getTime() - a.mtime.getTime());

      // 删除超过最大数量的文件
      if (fileStats.length > this.config.maxFiles) {
        const filesToDelete = fileStats.slice(this.config.maxFiles);
        for (const file of filesToDelete) {
          await fs.unlink(file.path);
        }
      }
    } catch (error) {
      console.error('清理旧日志文件失败:', error);
    }
  }

  /**
   * 清理过期日志
   */
  private async cleanOldLogs(): Promise<void> {
    try {
      const files = await fs.readdir(this.logDir);
      const logFiles = files.filter(f => f.startsWith('app-') && f.endsWith('.log'));

      const now = Date.now();
      const maxAge = this.config.maxAge * 24 * 60 * 60 * 1000;

      for (const file of logFiles) {
        const filePath = path.join(this.logDir, file);
        const stats = await fs.stat(filePath);

        if (now - stats.mtime.getTime() > maxAge) {
          await fs.unlink(filePath);
        }
      }
    } catch (error) {
      console.error('清理过期日志失败:', error);
    }
  }

  /**
   * 启动定时轮转
   */
  private startRotationTimer(): void {
    if (this.rotationTimer) {
      clearInterval(this.rotationTimer);
    }

    this.rotationTimer = setInterval(async () => {
      await this.rotate();
    }, this.config.rotationInterval);
  }

  /**
   * 写入日志条目
   */
  async writeEntry(entry: LogEntry): Promise<void> {
    try {
      await this.checkRotation();

      const logFile = await this.getCurrentLogFile();
      const formatted = this.formatter.formatJSON(entry) + '\n';

      await fs.appendFile(logFile, formatted, 'utf-8');
      this.currentSize += Buffer.byteLength(formatted, 'utf-8');
    } catch (error) {
      console.error('写入日志失败:', error);
      throw error;
    }
  }

  /**
   * 批量写入日志条目
   */
  async writeEntries(entries: LogEntry[]): Promise<void> {
    try {
      await this.checkRotation();

      const logFile = await this.getCurrentLogFile();
      const formatted = entries.map(e => this.formatter.formatJSON(e)).join('\n') + '\n';

      await fs.appendFile(logFile, formatted, 'utf-8');
      this.currentSize += Buffer.byteLength(formatted, 'utf-8');
    } catch (error) {
      console.error('批量写入日志失败:', error);
      throw error;
    }
  }

  /**
   * 查询日志
   */
  async queryLogs(options: {
    startTime?: Date;
    endTime?: Date;
    level?: string;
    module?: string;
    keyword?: string;
    limit?: number;
    offset?: number;
  }): Promise<LogEntry[]> {
    try {
      const files = await this.getLogFiles();
      const results: LogEntry[] = [];

      for (const file of files) {
        const content = await fs.readFile(file, 'utf-8');
        const lines = content.split('\n').filter(line => line.trim());

        for (const line of lines) {
          try {
            const entry = JSON.parse(line) as LogEntry;
            
            // 应用过滤条件
            if (options.startTime && new Date(entry.timestamp) < options.startTime) {
              continue;
            }
            if (options.endTime && new Date(entry.timestamp) > options.endTime) {
              continue;
            }
            if (options.level && entry.level !== options.level) {
              continue;
            }
            if (options.module && entry.module !== options.module) {
              continue;
            }
            if (options.keyword && !entry.message.includes(options.keyword)) {
              continue;
            }

            results.push(entry);
          } catch {
            // 忽略解析错误的行
          }
        }
      }

      // 按时间降序排序
      results.sort((a, b) => new Date(b.timestamp).getTime() - new Date(a.timestamp).getTime());

      // 应用分页
      const offset = options.offset || 0;
      const limit = options.limit || 100;
      return results.slice(offset, offset + limit);
    } catch (error) {
      console.error('查询日志失败:', error);
      throw error;
    }
  }

  /**
   * 获取所有日志文件
   */
  private async getLogFiles(): Promise<string[]> {
    try {
      const files = await fs.readdir(this.logDir);
      const logFiles = files
        .filter(f => f.startsWith('app-') && f.endsWith('.log'))
        .sort()
        .reverse();

      return logFiles.map(f => path.join(this.logDir, f));
    } catch (error) {
      console.error('获取日志文件列表失败:', error);
      return [];
    }
  }

  /**
   * 导出日志
   */
  async exportLogs(options: {
    startTime?: Date;
    endTime?: Date;
    format: 'json' | 'csv';
  }): Promise<string> {
    try {
      const entries = await this.queryLogs({
        startTime: options.startTime,
        endTime: options.endTime,
        limit: 10000
      });

      if (options.format === 'json') {
        return this.formatter.formatJSONBatch(entries);
      } else {
        return this.formatter.formatCSVBatch(entries);
      }
    } catch (error) {
      console.error('导出日志失败:', error);
      throw error;
    }
  }

  /**
   * 获取存储统计信息
   */
  async getStats(): Promise<{
    totalFiles: number;
    totalSize: number;
    oldestLog: Date | null;
    newestLog: Date | null;
  }> {
    try {
      const files = await this.getLogFiles();
      let totalSize = 0;
      let oldestLog: Date | null = null;
      let newestLog: Date | null = null;

      for (const file of files) {
        const stats = await fs.stat(file);
        totalSize += stats.size;

        if (!oldestLog || stats.mtime < oldestLog) {
          oldestLog = stats.mtime;
        }
        if (!newestLog || stats.mtime > newestLog) {
          newestLog = stats.mtime;
        }
      }

      return {
        totalFiles: files.length,
        totalSize,
        oldestLog,
        newestLog
      };
    } catch (error) {
      console.error('获取存储统计信息失败:', error);
      return {
        totalFiles: 0,
        totalSize: 0,
        oldestLog: null,
        newestLog: null
      };
    }
  }

  /**
   * 关闭存储
   */
  async close(): Promise<void> {
    if (this.rotationTimer) {
      clearInterval(this.rotationTimer);
      this.rotationTimer = null;
    }
  }
}