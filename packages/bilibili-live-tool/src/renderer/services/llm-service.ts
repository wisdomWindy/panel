/**
 * LLM 服务
 * 纯 IPC 调用封装，由 AI store 负责本地文案生成编排
 */

import { invokeIPC } from '../utils/ipc.js';
import type { AIConfig, LLMRequest, LLMResponse } from '../modules/ai/types.js';
import { DEFAULT_AI_CONFIG } from '../modules/ai/types.js';

export class LLMService {
  private config: AIConfig = { ...DEFAULT_AI_CONFIG };

  updateConfig(config: Partial<AIConfig>): void {
    this.config = { ...this.config, ...config };
  }

  getConfig(): AIConfig {
    return { ...this.config };
  }

  async callLLM(request: LLMRequest): Promise<LLMResponse> {
    const data = await invokeIPC<{
      success: boolean;
      response: LLMResponse;
      error?: string;
    }>('ai:chat', {
      provider: this.config.provider,
      apiKey: this.config.apiKey,
      model: request.model,
      endpoint: this.config.endpoint,
      messages: request.messages,
      maxTokens: request.maxTokens,
      temperature: request.temperature
    });
    return (data as { response: LLMResponse }).response;
  }

  async checkAvailability(): Promise<{ available: boolean; error?: string }> {
    try {
      const result = await window.electronAPI.invoke('ai:checkAvailability', {
        provider: this.config.provider,
        apiKey: this.config.apiKey,
        endpoint: this.config.endpoint
      });
      return result as { available: boolean; error?: string };
    } catch (error) {
      return {
        available: false,
        error: (error as Error).message
      };
    }
  }
}

let llmServiceInstance: LLMService | null = null;

export function getLLMService(): LLMService {
  if (!llmServiceInstance) {
    llmServiceInstance = new LLMService();
  }
  return llmServiceInstance;
}
