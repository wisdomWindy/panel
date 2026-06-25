/**
 * 日志查询模块
 * 实现日志查询功能，支持日志过滤和导出
 */

import type { LogEntry, LogLevel } from '../../main/modules/logger/logger.js';

export interface QueryFilter {
  startTime?: Date;
  endTime?: Date;
  level?: LogLevel;
  module?: string;
  keyword?: string;
}

export interface QueryResult {
  logs: LogEntry[];
  total: number;
  page: number;
  pageSize: number;
}

export interface ExportResult {
  data: string;
  filename: string;
  format: 'json' | 'csv';
}

interface ElectronAPI {
  invoke: (channel: string, ...args: unknown[]) => Promise<unknown>;
  on: (channel: string, callback: (...args: unknown[]) => void) => void;
  off: (channel: string, callback: (...args: unknown[]) => void) => void;
}

function getApi(): ElectronAPI {
  return (window as { electronAPI: ElectronAPI }).electronAPI;
}

export class LogQuery {
  private api = getApi();
  private cache: Map<string, { data: LogEntry[]; timestamp: number }> = new Map();
  private readonly CACHE_DURATION = 5 * 60 * 1000; // 5分钟缓存

  /**
   * 查询日志
   */
  async queryLogs(
    filter: QueryFilter,
    page: number = 1,
    pageSize: number = 50
  ): Promise<QueryResult> {
    try {
      const cacheKey = this.generateCacheKey(filter, page, pageSize);
      const cached = this.getFromCache(cacheKey);
      
      if (cached) {
        return cached;
      }

      const result = await this.api.invoke('logger:query', {
        ...filter,
        limit: pageSize,
        offset: (page - 1) * pageSize
      }) as {
        success: boolean;
        logs: LogEntry[];
        total: number;
        error?: string;
      };

      if (!result.success) {
        throw new Error(result.error || '查询日志失败');
      }

      const queryResult: QueryResult = {
        logs: result.logs,
        total: result.total,
        page,
        pageSize
      };

      this.setCache(cacheKey, queryResult);

      return queryResult;
    } catch (error) {
      console.error('查询日志失败:', error);
      throw error;
    }
  }

  /**
   * 获取所有日志
   */
  async getAllLogs(): Promise<LogEntry[]> {
    try {
      const result = await this.api.invoke('logger:query', {
        limit: 10000,
        offset: 0
      }) as {
        success: boolean;
        logs: LogEntry[];
        error?: string;
      };

      if (!result.success) {
        throw new Error(result.error || '获取日志失败');
      }

      return result.logs;
    } catch (error) {
      console.error('获取日志失败:', error);
      throw error;
    }
  }

  /**
   * 按级别查询日志
   */
  async queryByLevel(level: LogLevel): Promise<LogEntry[]> {
    try {
      const result = await this.api.invoke('logger:query', {
        level,
        limit: 10000,
        offset: 0
      }) as {
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
   * 按模块查询日志
   */
  async queryByModule(module: string): Promise<LogEntry[]> {
    try {
      const result = await this.api.invoke('logger:query', {
        module,
        limit: 10000,
        offset: 0
      }) as {
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
   * 按时间范围查询日志
   */
  async queryByTimeRange(startTime: Date, endTime: Date): Promise<LogEntry[]> {
    try {
      const result = await this.api.invoke('logger:query', {
        startTime,
        endTime,
        limit: 10000,
        offset: 0
      }) as {
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
   * 搜索日志
   */
  async searchLogs(keyword: string): Promise<LogEntry[]> {
    try {
      const result = await this.api.invoke('logger:query', {
        keyword,
        limit: 10000,
        offset: 0
      }) as {
        success: boolean;
        logs: LogEntry[];
        error?: string;
      };

      if (!result.success) {
        throw new Error(result.error || '搜索日志失败');
      }

      return result.logs;
    } catch (error) {
      console.error('搜索日志失败:', error);
      throw error;
    }
  }

  /**
   * 导出日志
   */
  async exportLogs(
    filter: QueryFilter,
    format: 'json' | 'csv'
  ): Promise<ExportResult> {
    try {
      const result = await this.api.invoke('logger:export', {
        ...filter,
        format
      }) as {
        success: boolean;
        data: string;
        error?: string;
      };

      if (!result.success) {
        throw new Error(result.error || '导出日志失败');
      }

      const filename = `logs-${new Date().toISOString().split('T')[0]}.${format}`;

      return {
        data: result.data,
        filename,
        format
      };
    } catch (error) {
      console.error('导出日志失败:', error);
      throw error;
    }
  }

  /**
   * 下载导出的日志
   */
  async downloadLogs(filter: QueryFilter, format: 'json' | 'csv'): Promise<void> {
    try {
      const exportResult = await this.exportLogs(filter, format);
      
      // 创建下载链接
      const blob = new Blob([exportResult.data], { 
        type: format === 'json' ? 'application/json' : 'text/csv' 
      });
      const url = URL.createObjectURL(blob);
      
      const a = document.createElement('a');
      a.href = url;
      a.download = exportResult.filename;
      document.body.appendChild(a);
      a.click();
      document.body.removeChild(a);
      URL.revokeObjectURL(url);
    } catch (error) {
      console.error('下载日志失败:', error);
      throw error;
    }
  }

  /**
   * 生成缓存键
   */
  private generateCacheKey(filter: QueryFilter, page: number, pageSize: number): string {
    return JSON.stringify({ filter, page, pageSize });
  }

  /**
   * 从缓存获取
   */
  private getFromCache(key: string): QueryResult | null {
    const cached = this.cache.get(key);
    if (cached && Date.now() - cached.timestamp < this.CACHE_DURATION) {
      return cached.data as unknown as QueryResult;
    }
    return null;
  }

  /**
   * 设置缓存
   */
  private setCache(key: string, data: QueryResult): void {
    this.cache.set(key, {
      data: data.logs,
      timestamp: Date.now()
    });
  }

  /**
   * 清除缓存
   */
  clearCache(): void {
    this.cache.clear();
  }

  /**
   * 获取缓存大小
   */
  getCacheSize(): number {
    return this.cache.size;
  }

  /**
   * 过滤日志
   */
  filterLogs(logs: LogEntry[], filter: QueryFilter): LogEntry[] {
    return logs.filter(log => {
      if (filter.startTime && new Date(log.timestamp) < filter.startTime) {
        return false;
      }
      if (filter.endTime && new Date(log.timestamp) > filter.endTime) {
        return false;
      }
      if (filter.level && log.level !== filter.level) {
        return false;
      }
      if (filter.module && log.module !== filter.module) {
        return false;
      }
      if (filter.keyword) {
        const keyword = filter.keyword.toLowerCase();
        const messageMatch = log.message.toLowerCase().includes(keyword);
        const moduleMatch = log.module.toLowerCase().includes(keyword);
        const errorMatch = log.error && log.error.message.toLowerCase().includes(keyword);
        
        if (!messageMatch && !moduleMatch && !errorMatch) {
          return false;
        }
      }
      return true;
    });
  }

  /**
   * 排序日志
   */
  sortLogs(logs: LogEntry[], field: 'timestamp' | 'level' | 'module', order: 'asc' | 'desc' = 'desc'): LogEntry[] {
    return [...logs].sort((a, b) => {
      let comparison = 0;
      
      switch (field) {
        case 'timestamp':
          comparison = new Date(a.timestamp).getTime() - new Date(b.timestamp).getTime();
          break;
        case 'level':
          const levelOrder = { debug: 0, info: 1, warn: 2, error: 3 };
          comparison = levelOrder[a.level] - levelOrder[b.level];
          break;
        case 'module':
          comparison = a.module.localeCompare(b.module);
          break;
      }
      
      return order === 'asc' ? comparison : -comparison;
    });
  }

  /**
   * 分页日志
   */
  paginateLogs(logs: LogEntry[], page: number, pageSize: number): QueryResult {
    const start = (page - 1) * pageSize;
    const end = start + pageSize;
    
    return {
      logs: logs.slice(start, end),
      total: logs.length,
      page,
      pageSize
    };
  }
}

// 导出单例实例
export const logQuery = new LogQuery();