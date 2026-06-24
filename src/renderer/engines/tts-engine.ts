/**
 * TTS 引擎接口
 * 定义语音播报引擎的统一契约
 */

export interface TTSEngine {
  /** 引擎名称 */
  readonly name: string;

  /** 引擎是否可用 */
  isAvailable(): boolean;

  /** 播报文本 */
  speak(text: string, options: TTSSpeakOptions): Promise<void>;

  /** 停止播报 */
  stop(): Promise<void>;

  /** 暂停播报 */
  pause(): void;

  /** 恢复播报 */
  resume(): void;
}

export interface TTSSpeakOptions {
  voice?: string;
  rate?: number;
  pitch?: number;
  volume?: number;
  lang?: string;
}
