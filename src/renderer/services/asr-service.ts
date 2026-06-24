/**
 * ASR 服务
 * 纯配置管理，识别逻辑已委托给引擎
 */

import type { ASRConfig } from '../modules/asr/types.js';
import { DEFAULT_ASR_CONFIG } from '../modules/asr/types.js';

export class ASRService {
  private config: ASRConfig = { ...DEFAULT_ASR_CONFIG };

  updateConfig(config: Partial<ASRConfig>): void {
    this.config = { ...this.config, ...config };
  }

  getConfig(): ASRConfig {
    return { ...this.config };
  }
}

let asrServiceInstance: ASRService | null = null;

export function getASRService(): ASRService {
  if (!asrServiceInstance) {
    asrServiceInstance = new ASRService();
  }
  return asrServiceInstance;
}
