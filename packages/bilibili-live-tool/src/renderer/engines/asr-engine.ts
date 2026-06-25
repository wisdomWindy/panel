/**
 * ASR 引擎接口
 * 定义语音识别引擎的统一契约
 */

import type { ASRResult } from '../modules/asr/types.js';

export interface ASREngine {
  /** 引擎名称 */
  readonly name: string;

  /** 引擎是否可用 */
  isAvailable(): Promise<boolean>;

  /** 开始识别 */
  start(config: ASREngineConfig): Promise<void>;

  /** 停止识别 */
  stop(): Promise<void>;

  /** 暂停识别 */
  pause(): void;

  /** 恢复识别 */
  resume(): void;

  /** 设置结果回调 */
  onResult(callback: (result: ASRResult) => void): void;

  /** 设置错误回调 */
  onError(callback: (error: { code: string; message: string }) => void): void;

  /** 设置结束回调 */
  onEnd(callback: () => void): void;
}

export interface ASREngineConfig {
  language: string;
  continuous: boolean;
  interimResults: boolean;
  maxAlternatives: number;
  model?: string;
}
