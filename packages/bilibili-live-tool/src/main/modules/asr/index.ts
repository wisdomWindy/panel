/**
 * ASR 主进程模块
 * 处理 whisper.cpp 的 IPC 调用
 */

import { ipcMain } from 'electron';
import { spawn, ChildProcess } from 'child_process';
import * as path from 'path';
import * as fs from 'fs';
import * as os from 'os';

// whisper.cpp 动态导入（可能未安装）
let whisperAvailable = false;
let whisperPath = '';

// 当前录音进程
let currentProcess: ChildProcess | null = null;
let isListening = false;

/**
 * 检查 whisper.cpp 是否可用
 */
async function checkWhisper(): Promise<boolean> {
  try {
    // 尝试查找 whisper.cpp
    const possiblePaths = [
      '/usr/local/bin/whisper',
      '/opt/homebrew/bin/whisper',
      path.join(os.homedir(), '.local/bin/whisper'),
      'whisper'  // 系统 PATH
    ];

    for (const p of possiblePaths) {
      try {
        const proc = spawn(p, ['--version'], { stdio: 'pipe' });
        const exitCode = await new Promise<number>((resolve) => {
          proc.on('close', resolve);
        });
        if (exitCode === 0) {
          whisperPath = p;
          whisperAvailable = true;
          return true;
        }
      } catch (e) {
        continue;
      }
    }

    // 尝试检查 whisper-node 包
    try {
      const whisperNode = await import('whisper-node');
      if (whisperNode) {
        whisperAvailable = true;
        return true;
      }
    } catch (e) {
      // whisper-node 未安装
    }

    return false;
  } catch (error) {
    console.error('Whisper check error:', error);
    return false;
  }
}

/**
 * 初始化 ASR 模块
 */
export function initASRModule(): void {
  // 预检查 whisper.cpp
  checkWhisper();

  // 注册 IPC 处理器
  ipcMain.handle('asr:start', async (_event, _params: {
    engine: string;
    model: string;
    language: string;
  }) => {
    try {
      if (isListening) {
        return { success: false, error: 'Already listening' };
      }

      isListening = true;
      return { success: true };
    } catch (error) {
      return { success: false, error: (error as Error).message };
    }
  });

  ipcMain.handle('asr:process', async (_event, params: {
    audio: string;  // base64 编码的音频
    language: string;
  }) => {
    try {
      if (!whisperAvailable) {
        // 降级：返回模拟结果
        return {
          success: true,
          text: '[语音识别不可用，请安装 whisper.cpp]',
          confidence: 0
        };
      }

      // 将 base64 音频保存为临时文件
      const tempDir = os.tmpdir();
      const tempFile = path.join(tempDir, `asr_${Date.now()}.wav`);
      const audioBuffer = Buffer.from(params.audio, 'base64');
      fs.writeFileSync(tempFile, audioBuffer);

      // 调用 whisper.cpp 进行识别
      const result = await new Promise<{ text: string; confidence: number }>((resolve, reject) => {
        const args = [
          '-m', params.language === 'zh' ? 'ggml-base.bin' : 'ggml-base.bin',
          '-f', tempFile,
          '--language', params.language === 'auto' ? 'auto' : params.language,
          '--output-txt'
        ];

        const proc = spawn(whisperPath, args, { stdio: 'pipe' });
        let output = '';
        let errorOutput = '';

        proc.stdout.on('data', (data) => {
          output += data.toString();
        });

        proc.stderr.on('data', (data) => {
          errorOutput += data.toString();
        });

        proc.on('close', (code) => {
          // 清理临时文件
          try {
            fs.unlinkSync(tempFile);
          } catch (e) {
            // 忽略错误
          }

          if (code === 0) {
            // 解析输出
            const textMatch = output.match(/\[.*\]\s*(.*)/);
            const text = textMatch ? textMatch[1].trim() : output.trim();
            resolve({ text, confidence: 0.9 });
          } else {
            reject(new Error(errorOutput || 'Whisper process failed'));
          }
        });

        proc.on('error', (error) => {
          reject(error);
        });
      });

      return { success: true, ...result };
    } catch (error) {
      return { success: false, error: (error as Error).message };
    }
  });

  ipcMain.handle('asr:stop', async () => {
    isListening = false;
    if (currentProcess) {
      currentProcess.kill();
      currentProcess = null;
    }
    return { success: true };
  });

  ipcMain.handle('asr:checkAvailability', async () => {
    const available = await checkWhisper();
    return {
      available,
      version: whisperPath ? 'whisper.cpp' : undefined
    };
  });
}

/**
 * 清理 ASR 模块
 */
export function cleanupASRModule(): void {
  ipcMain.removeHandler('asr:start');
  ipcMain.removeHandler('asr:process');
  ipcMain.removeHandler('asr:stop');
  ipcMain.removeHandler('asr:checkAvailability');
  isListening = false;
  if (currentProcess) {
    currentProcess.kill();
    currentProcess = null;
  }
}
