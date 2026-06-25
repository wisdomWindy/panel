/**
 * 日志查看器模块
 * 实现日志查看功能，支持日志查询和导出
 */

import type { LogEntry, LogLevel } from '../../main/modules/logger/logger.js';

export interface QueryOptions {
  startTime?: Date;
  endTime?: Date;
  level?: LogLevel;
  module?: string;
  keyword?: string;
  limit?: number;
  offset?: number;
}

export interface ExportOptions {
  startTime?: Date;
  endTime?: Date;
  format: 'json' | 'csv';
}

export interface LogStats {
  totalFiles: number;
  totalSize: number;
  oldestLog: Date | null;
  newestLog: Date | null;
}

interface ElectronAPI {
  invoke: (channel: string, ...args: unknown[]) => Promise<unknown>;
  on: (channel: string, callback: (...args: unknown[]) => void) => void;
  off: (channel: string, callback: (...args: unknown[]) => void) => void;
}

function getApi(): ElectronAPI {
  return (window as { electronAPI: ElectronAPI }).electronAPI;
}

export class LogViewer {
  private api = getApi();

  /**
   * 查询日志
   */
  async queryLogs(options: QueryOptions): Promise<LogEntry[]> {
    try {
      const result = await this.api.invoke('logger:query', options) as {
        success: boolean;
        logs: LogEntry[];
        error?: string;
      };

      if (!result.success) {
        throw new Error(result.error || '查询日志失败');
      }

      return result.logs;
    } catch (error) {
      console.error('查询日志失败:', error);
      throw error;
    }
  }

  /**
   * 导出日志
   */
  async exportLogs(options: ExportOptions): Promise<string> {
    try {
      const result = await this.api.invoke('logger:export', options) as {
        success: boolean;
        data: string;
        error?: string;
      };

      if (!result.success) {
        throw new Error(result.error || '导出日志失败');
      }

      return result.data;
    } catch (error) {
      console.error('导出日志失败:', error);
      throw error;
    }
  }

  /**
   * 获取日志统计信息
   */
  async getStats(): Promise<LogStats> {
    try {
      const result = await this.api.invoke('logger:stats') as {
        success: boolean;
        stats: LogStats;
        error?: string;
      };

      if (!result.success) {
        throw new Error(result.error || '获取日志统计信息失败');
      }

      return result.stats;
    } catch (error) {
      console.error('获取日志统计信息失败:', error);
      throw error;
    }
  }

  /**
   * 下载导出的日志文件
   */
  async downloadExportedLogs(options: ExportOptions): Promise<void> {
    try {
      const data = await this.exportLogs(options);
      
      // 创建下载链接
      const blob = new Blob([data], { 
        type: options.format === 'json' ? 'application/json' : 'text/csv' 
      });
      const url = URL.createObjectURL(blob);
      
      const a = document.createElement('a');
      a.href = url;
      a.download = `logs-${new Date().toISOString().split('T')[0]}.${options.format}`;
      document.body.appendChild(a);
      a.click();
      document.body.removeChild(a);
      URL.revokeObjectURL(url);
    } catch (error) {
      console.error('下载日志文件失败:', error);
      throw error;
    }
  }

  /**
   * 格式化日志级别
   */
  formatLevel(level: LogLevel): string {
    const levelMap: Record<LogLevel, string> = {
      debug: 'DEBUG',
      info: 'INFO',
      warn: 'WARN',
      error: 'ERROR'
    };
    return levelMap[level] || level.toUpperCase();
  }

  /**
   * 获取日志级别颜色
   */
  getLevelColor(level: LogLevel): string {
    const colorMap: Record<LogLevel, string> = {
      debug: '#6c757d',
      info: '#28a745',
      warn: '#ffc107',
      error: '#dc3545'
    };
    return colorMap[level] || '#6c757d';
  }

  /**
   * 格式化时间戳
   */
  formatTimestamp(timestamp: Date): string {
    return new Date(timestamp).toLocaleString('zh-CN', {
      year: 'numeric',
      month: '2-digit',
      day: '2-digit',
      hour: '2-digit',
      minute: '2-digit',
      second: '2-digit'
    });
  }

  /**
   * 格式化文件大小
   */
  formatFileSize(bytes: number): string {
    if (bytes === 0) return '0 B';
    
    const k = 1024;
    const sizes = ['B', 'KB', 'MB', 'GB'];
    const i = Math.floor(Math.log(bytes) / Math.log(k));
    
    return parseFloat((bytes / Math.pow(k, i)).toFixed(2)) + ' ' + sizes[i];
  }

  /**
   * 截断消息
   */
  truncateMessage(message: string, maxLength: number = 100): string {
    if (message.length <= maxLength) {
      return message;
    }
    return message.substring(0, maxLength) + '...';
  }

  /**
   * 搜索日志
   */
  searchLogs(logs: LogEntry[], keyword: string): LogEntry[] {
    const lowerKeyword = keyword.toLowerCase();
    return logs.filter(log => 
      log.message.toLowerCase().includes(lowerKeyword) ||
      log.module.toLowerCase().includes(lowerKeyword) ||
      (log.error && log.error.message.toLowerCase().includes(lowerKeyword))
    );
  }

  /**
   * 按级别过滤日志
   */
  filterByLevel(logs: LogEntry[], level: LogLevel): LogEntry[] {
    return logs.filter(log => log.level === level);
  }

  /**
   * 按模块过滤日志
   */
  filterByModule(logs: LogEntry[], module: string): LogEntry[] {
    return logs.filter(log => log.module === module);
  }

  /**
   * 按时间范围过滤日志
   */
  filterByTimeRange(logs: LogEntry[], startTime: Date, endTime: Date): LogEntry[] {
    return logs.filter(log => {
      const timestamp = new Date(log.timestamp);
      return timestamp >= startTime && timestamp <= endTime;
    });
  }

  /**
   * 获取所有模块列表
   */
  getModules(logs: LogEntry[]): string[] {
    const modules = new Set(logs.map(log => log.module));
    return Array.from(modules).sort();
  }

  /**
   * 统计各级别日志数量
   */
  countByLevel(logs: LogEntry[]): Record<LogLevel, number> {
    const counts: Record<LogLevel, number> = {
      debug: 0,
      info: 0,
      warn: 0,
      error: 0
    };

    logs.forEach(log => {
      counts[log.level]++;
    });

    return counts;
  }
}

// 导出单例实例
export const logViewer = new LogViewer();