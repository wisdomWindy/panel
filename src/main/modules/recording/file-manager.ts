/**
 * 录制文件管理器
 * 负责文件路径管理、元数据读取、存储空间检查
 */

import * as fs from 'fs';
import * as path from 'path';
import { app } from 'electron';

export interface FileInfo {
  path: string;
  name: string;
  size: number;
  createdAt: Date;
  modifiedAt: Date;
}

export interface StorageInfo {
  totalSpace: number;  // 总空间（字节）
  freeSpace: number;   // 可用空间（字节）
  usedSpace: number;   // 已用空间（字节）
}

export class FileManager {
  private defaultOutputPath: string;

  constructor() {
    // 默认存储路径：用户目录/BilibiliLiveTool/recordings
    const userHome = app.getPath('home');
    this.defaultOutputPath = path.join(userHome, 'BilibiliLiveTool', 'recordings');
  }

  /**
   * 获取默认输出路径
   */
  getDefaultOutputPath(): string {
    return this.defaultOutputPath;
  }

  /**
   * 设置默认输出路径
   */
  setDefaultOutputPath(outputPath: string): void {
    this.defaultOutputPath = outputPath;
  }

  /**
   * 确保目录存在
   */
  async ensureDirectory(dirPath: string): Promise<void> {
    try {
      await fs.promises.access(dirPath);
    } catch {
      await fs.promises.mkdir(dirPath, { recursive: true });
    }
  }

  /**
   * 生成录制文件路径
   */
  generateRecordingPath(roomId: string, format: string, outputPath?: string): string {
    const baseDir = outputPath || this.defaultOutputPath;
    const now = new Date();
    const dateStr = now.toISOString().replace(/[:.]/g, '-').slice(0, 19);
    const fileName = `${roomId}_${dateStr}.${format}`;
    return path.join(baseDir, fileName);
  }

  /**
   * 生成高光片段路径
   */
  generateClipPath(recordingPath: string, highlightId: string): string {
    const dir = path.dirname(recordingPath);
    const ext = path.extname(recordingPath);
    const baseName = path.basename(recordingPath, ext);
    return path.join(dir, `${baseName}_highlight_${highlightId}${ext}`);
  }

  /**
   * 检查文件是否存在
   */
  async fileExists(filePath: string): Promise<boolean> {
    try {
      await fs.promises.access(filePath);
      return true;
    } catch {
      return false;
    }
  }

  /**
   * 获取文件信息
   */
  async getFileInfo(filePath: string): Promise<FileInfo | null> {
    try {
      const stats = await fs.promises.stat(filePath);
      return {
        path: filePath,
        name: path.basename(filePath),
        size: stats.size,
        createdAt: stats.birthtime,
        modifiedAt: stats.mtime
      };
    } catch {
      return null;
    }
  }

  /**
   * 获取文件大小（字节）
   */
  async getFileSize(filePath: string): Promise<number> {
    const info = await this.getFileInfo(filePath);
    return info ? info.size : 0;
  }

  /**
   * 删除文件
   */
  async deleteFile(filePath: string): Promise<void> {
    try {
      await fs.promises.unlink(filePath);
    } catch (error) {
      // 文件不存在时忽略错误
      if ((error as NodeJS.ErrnoException).code !== 'ENOENT') {
        throw error;
      }
    }
  }

  /**
   * 重命名文件
   */
  async renameFile(oldPath: string, newPath: string): Promise<void> {
    await fs.promises.rename(oldPath, newPath);
  }

  /**
   * 打开文件所在目录
   */
  async openFileFolder(filePath: string): Promise<void> {
    const dir = path.dirname(filePath);
    const { shell } = await import('electron');
    shell.openPath(dir);
  }

  /**
   * 获取存储空间信息
   */
  async getStorageInfo(dirPath?: string): Promise<StorageInfo> {
    const targetPath = dirPath || this.defaultOutputPath;

    try {
      // 使用 statvfs 获取磁盘空间信息（类 Unix 系统）
      const stats = await fs.promises.statfs(targetPath);
      const totalSpace = stats.blocks * stats.bsize;
      const freeSpace = stats.bavail * stats.bsize;
      const usedSpace = totalSpace - freeSpace;

      return {
        totalSpace,
        freeSpace,
        usedSpace
      };
    } catch {
      // 如果 statfs 不可用，返回默认值
      return {
        totalSpace: 0,
        freeSpace: 0,
        usedSpace: 0
      };
    }
  }

  /**
   * 检查是否有足够的存储空间
   */
  async hasEnoughSpace(requiredSpace: number, dirPath?: string): Promise<boolean> {
    const storageInfo = await this.getStorageInfo(dirPath);
    return storageInfo.freeSpace >= requiredSpace;
  }

  /**
   * 获取录制文件列表
   */
  async listRecordingFiles(dirPath?: string): Promise<FileInfo[]> {
    const targetPath = dirPath || this.defaultOutputPath;

    try {
      await this.ensureDirectory(targetPath);
      const files = await fs.promises.readdir(targetPath);
      const recordingFiles: FileInfo[] = [];

      for (const file of files) {
        const filePath = path.join(targetPath, file);
        const info = await this.getFileInfo(filePath);
        if (info && this.isRecordingFile(file)) {
          recordingFiles.push(info);
        }
      }

      // 按修改时间倒序排列
      recordingFiles.sort((a, b) => b.modifiedAt.getTime() - a.modifiedAt.getTime());

      return recordingFiles;
    } catch {
      return [];
    }
  }

  /**
   * 检查是否为录制文件
   */
  private isRecordingFile(fileName: string): boolean {
    const recordingExtensions = ['.mp4', '.flv', '.mkv', '.avi'];
    const ext = path.extname(fileName).toLowerCase();
    return recordingExtensions.includes(ext);
  }

  /**
   * 清理旧文件
   */
  async cleanupOldFiles(maxAgeDays: number, dirPath?: string): Promise<number> {
    const targetPath = dirPath || this.defaultOutputPath;
    const files = await this.listRecordingFiles(targetPath);
    const now = Date.now();
    const maxAgeMs = maxAgeDays * 24 * 60 * 60 * 1000;
    let deletedCount = 0;

    for (const file of files) {
      const fileAge = now - file.modifiedAt.getTime();
      if (fileAge > maxAgeMs) {
        await this.deleteFile(file.path);
        deletedCount++;
      }
    }

    return deletedCount;
  }

  /**
   * 清理超大文件（按总大小限制）
   */
  async cleanupBySize(maxSizeGB: number, dirPath?: string): Promise<number> {
    const targetPath = dirPath || this.defaultOutputPath;
    const files = await this.listRecordingFiles(targetPath);
    const maxSizeBytes = maxSizeGB * 1024 * 1024 * 1024;
    let totalSize = 0;
    let deletedCount = 0;

    // 计算总大小
    for (const file of files) {
      totalSize += file.size;
    }

    // 如果超过限制，从最旧的文件开始删除
    for (const file of files) {
      if (totalSize <= maxSizeBytes) {
        break;
      }
      await this.deleteFile(file.path);
      totalSize -= file.size;
      deletedCount++;
    }

    return deletedCount;
  }

  /**
   * 格式化文件大小
   */
  formatFileSize(bytes: number): string {
    if (bytes === 0) return '0 B';

    const units = ['B', 'KB', 'MB', 'GB', 'TB'];
    const k = 1024;
    const i = Math.floor(Math.log(bytes) / Math.log(k));

    return parseFloat((bytes / Math.pow(k, i)).toFixed(2)) + ' ' + units[i];
  }

  /**
   * 格式化时长
   */
  formatDuration(seconds: number): string {
    const hours = Math.floor(seconds / 3600);
    const minutes = Math.floor((seconds % 3600) / 60);
    const secs = seconds % 60;

    return `${hours.toString().padStart(2, '0')}:${minutes.toString().padStart(2, '0')}:${secs.toString().padStart(2, '0')}`;
  }

  /**
   * 销毁管理器
   */
  destroy(): void {
    // 清理资源
  }
}

// 单例
export const fileManager = new FileManager();
