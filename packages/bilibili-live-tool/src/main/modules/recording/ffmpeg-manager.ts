/**
 * FFmpeg 管理器
 * 负责检测 FFmpeg 可用性、管理子进程生命周期
 */

import { spawn, ChildProcess } from 'child_process';
import { EventEmitter } from 'events';

export interface FFmpegInfo {
  available: boolean;
  version: string;
  path: string;
}

export class FFmpegManager extends EventEmitter {
  private ffmpegPath: string = 'ffmpeg';
  private currentProcess: ChildProcess | null = null;
  private isAvailable: boolean = false;
  private version: string = '';

  constructor() {
    super();
  }

  /**
   * 检测 FFmpeg 是否可用
   */
  async checkAvailability(): Promise<FFmpegInfo> {
    return new Promise((resolve) => {
      const process = spawn(this.ffmpegPath, ['-version']);
      let output = '';

      process.stdout.on('data', (data: Buffer) => {
        output += data.toString();
      });

      process.on('close', (code) => {
        if (code === 0) {
          this.isAvailable = true;
          this.version = this.parseVersion(output);
          resolve({
            available: true,
            version: this.version,
            path: this.ffmpegPath
          });
        } else {
          this.isAvailable = false;
          resolve({
            available: false,
            version: '',
            path: ''
          });
        }
      });

      process.on('error', () => {
        this.isAvailable = false;
        resolve({
          available: false,
          version: '',
          path: ''
        });
      });
    });
  }

  /**
   * 解析 FFmpeg 版本号
   */
  private parseVersion(output: string): string {
    const versionMatch = output.match(/ffmpeg version (\S+)/);
    return versionMatch ? versionMatch[1] : 'unknown';
  }

  /**
   * 获取 FFmpeg 是否可用
   */
  getIsAvailable(): boolean {
    return this.isAvailable;
  }

  /**
   * 获取 FFmpeg 版本
   */
  getVersion(): string {
    return this.version;
  }

  /**
   * 设置 FFmpeg 路径
   */
  setFFmpegPath(path: string): void {
    this.ffmpegPath = path;
  }

  /**
   * 启动 FFmpeg 进程
   */
  startProcess(args: string[]): ChildProcess {
    if (this.currentProcess) {
      this.stopProcess();
    }

    this.currentProcess = spawn(this.ffmpegPath, args, {
      stdio: ['pipe', 'pipe', 'pipe']
    });

    this.currentProcess.stdout?.on('data', (data: Buffer) => {
      this.emit('stdout', data.toString());
    });

    this.currentProcess.stderr?.on('data', (data: Buffer) => {
      this.emit('stderr', data.toString());
    });

    this.currentProcess.on('close', (code) => {
      this.emit('close', code);
      this.currentProcess = null;
    });

    this.currentProcess.on('error', (error) => {
      this.emit('error', error);
      this.currentProcess = null;
    });

    return this.currentProcess;
  }

  /**
   * 停止当前 FFmpeg 进程
   */
  stopProcess(): void {
    if (this.currentProcess) {
      // 发送 'q' 命令让 FFmpeg 优雅退出
      if (this.currentProcess.stdin) {
        this.currentProcess.stdin.write('q');
      }

      // 等待一段时间后强制杀死
      setTimeout(() => {
        if (this.currentProcess && !this.currentProcess.killed) {
          this.currentProcess.kill('SIGKILL');
        }
      }, 3000);

      this.currentProcess = null;
    }
  }

  /**
   * 暂停当前 FFmpeg 进程（发送 SIGSTOP 信号）
   */
  pauseProcess(): void {
    if (this.currentProcess && !this.currentProcess.killed) {
      this.currentProcess.kill('SIGSTOP');
      this.emit('paused');
    }
  }

  /**
   * 恢复当前 FFmpeg 进程（发送 SIGCONT 信号）
   */
  resumeProcess(): void {
    if (this.currentProcess && !this.currentProcess.killed) {
      this.currentProcess.kill('SIGCONT');
      this.emit('resumed');
    }
  }

  /**
   * 获取当前进程是否正在运行
   */
  isRunning(): boolean {
    return this.currentProcess !== null && !this.currentProcess.killed;
  }

  /**
   * 获取当前进程 PID
   */
  getPid(): number | undefined {
    return this.currentProcess?.pid;
  }

  /**
   * 向当前进程 stdin 写入数据
   */
  writeToStdin(data: string): boolean {
    if (this.currentProcess && this.currentProcess.stdin) {
      return this.currentProcess.stdin.write(data);
    }
    return false;
  }

  /**
   * 销毁管理器，清理资源
   */
  destroy(): void {
    this.stopProcess();
    this.removeAllListeners();
  }
}

// 单例
export const ffmpegManager = new FFmpegManager();
