/**
 * TTS 主进程模块
 * 处理 edge-tts 的 IPC 调用
 */

import { ipcMain } from 'electron';

// edge-tts 动态导入（可能未安装）
// eslint-disable-next-line @typescript-eslint/no-explicit-any
let EdgeTTS: any = null;

async function loadEdgeTTS(): Promise<boolean> {
  try {
    const module = await import('edge-tts');
    EdgeTTS = module.EdgeTTS || module.default;
    return true;
  } catch (error) {
    console.warn('edge-tts not available:', error);
    return false;
  }
}

// 存储当前的 TTS 实例
// eslint-disable-next-line @typescript-eslint/no-explicit-any
let currentTTS: any = null;
let isSpeaking = false;

/**
 * 初始化 TTS 模块
 */
export function initTTSModule(): void {
  // 预加载 edge-tts
  loadEdgeTTS();

  // 注册 IPC 处理器
  ipcMain.handle('tts:speak', async (_event, params: {
    text: string;
    voice: string;
    rate: number;
    volume: number;
  }) => {
    try {
      if (!EdgeTTS) {
        const loaded = await loadEdgeTTS();
        if (!loaded) {
          return { success: false, error: 'edge-tts not installed' };
        }
      }

      // 停止当前播报
      if (currentTTS) {
        currentTTS = null;
      }

      isSpeaking = true;

      // 创建新的 TTS 实例
      const tts = new EdgeTTS();
      currentTTS = tts;

      // 设置语音
      await tts.setVoice(params.voice || 'zh-CN-XiaoxiaoNeural');

      // 设置语速（edge-tts 使用百分比格式）
      const ratePercent = Math.round((params.rate - 1) * 100);
      await tts.setRate(ratePercent >= 0 ? `+${ratePercent}%` : `${ratePercent}%`);

      // 生成音频
      const audioBuffer = await tts.toBuffer(params.text);

      if (!isSpeaking) {
        return { success: false, error: 'stopped' };
      }

      // 返回音频数据（base64 编码）
      const base64Audio = audioBuffer.toString('base64');

      isSpeaking = false;
      currentTTS = null;

      return { success: true, audio: base64Audio };
    } catch (error) {
      isSpeaking = false;
      currentTTS = null;
      console.error('TTS error:', error);
      return { success: false, error: (error as Error).message };
    }
  });

  ipcMain.handle('tts:stop', async () => {
    isSpeaking = false;
    currentTTS = null;
    return { success: true };
  });

  ipcMain.handle('tts:isAvailable', async () => {
    if (!EdgeTTS) {
      const loaded = await loadEdgeTTS();
      return { available: loaded };
    }
    return { available: true };
  });
}

/**
 * 清理 TTS 模块
 */
export function cleanupTTSModule(): void {
  ipcMain.removeHandler('tts:speak');
  ipcMain.removeHandler('tts:stop');
  ipcMain.removeHandler('tts:isAvailable');
  isSpeaking = false;
  currentTTS = null;
}
