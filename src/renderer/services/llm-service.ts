/**
 * LLM 服务
 * 支持 OpenAI、Claude、Ollama 等多种 LLM 提供商
 */

import type {
  AIConfig,
  ChatMessage,
  AIResponse,
  AIStatus,
  AIError,
  LLMRequest,
  LLMResponse
} from '../modules/ai/types.js';
import { DEFAULT_AI_CONFIG } from '../modules/ai/types.js';

export class LLMService {
  private config: AIConfig = { ...DEFAULT_AI_CONFIG };
  private status: AIStatus = 'idle';
  private chatHistory: ChatMessage[] = [];
  private lastReplyTime = 0;
  private statusListeners: Array<(status: AIStatus) => void> = [];
  private responseListeners: Array<(response: AIResponse) => void> = [];
  private errorListeners: Array<(error: AIError) => void> = [];

  constructor() {}

  /**
   * 更新配置
   */
  updateConfig(config: Partial<AIConfig>): void {
    this.config = { ...this.config, ...config };
  }

  /**
   * 获取配置
   */
  getConfig(): AIConfig {
    return { ...this.config };
  }

  /**
   * 获取状态
   */
  getStatus(): AIStatus {
    return this.status;
  }

  /**
   * 添加状态监听器
   */
  onStatusChange(listener: (status: AIStatus) => void): void {
    this.statusListeners.push(listener);
  }

  /**
   * 添加回复监听器
   */
  onResponse(listener: (response: AIResponse) => void): void {
    this.responseListeners.push(listener);
  }

  /**
   * 添加错误监听器
   */
  onError(listener: (error: AIError) => void): void {
    this.errorListeners.push(listener);
  }

  /**
   * 更新状态
   */
  private setStatus(newStatus: AIStatus): void {
    this.status = newStatus;
    this.statusListeners.forEach(listener => listener(newStatus));
  }

  /**
   * 处理弹幕消息
   */
  async handleDanmaku(username: string, content: string, _userId: string): Promise<void> {
    if (!this.config.enabled || !this.config.autoReply) return;

    // 检查是否应该回复
    if (!this.shouldReply(content)) return;

    // 检查发送冷却
    const now = Date.now();
    if (now - this.lastReplyTime < this.config.minInterval * 1000) return;

    // 添加到上下文
    this.addToContext({
      role: 'user',
      content: `${username}: ${content}`,
      timestamp: now,
      username
    });

    // 生成回复
    try {
      const response = await this.generateReply();
      if (response.shouldSend) {
        this.lastReplyTime = Date.now();
        this.responseListeners.forEach(listener => listener(response));
      }
    } catch (error) {
      const aiError: AIError = {
        code: 'generate-failed',
        message: '生成回复失败',
        details: (error as Error).message
      };
      this.errorListeners.forEach(listener => listener(aiError));
    }
  }

  /**
   * 检查是否应该回复
   */
  private shouldReply(content: string): boolean {
    // 检查是否是问题
    if (this.config.replyToQuestions) {
      const questionPatterns = ['?', '？', '吗', '呢', '什么', '怎么', '为什么', '哪'];
      if (questionPatterns.some(p => content.includes(p))) {
        return true;
      }
    }

    // 检查是否是 @
    if (this.config.replyToMentions) {
      if (content.includes('@') || content.includes('艾特')) {
        return true;
      }
    }

    // 默认自动回复
    return this.config.autoReply;
  }

  /**
   * 添加到上下文
   */
  private addToContext(message: ChatMessage): void {
    this.chatHistory.push(message);
    // 保持上下文窗口大小
    if (this.chatHistory.length > this.config.contextWindow) {
      this.chatHistory.shift();
    }
  }

  /**
   * 生成回复
   */
  async generateReply(): Promise<AIResponse> {
    const startTime = Date.now();
    this.setStatus('thinking');

    try {
      // 构建请求消息
      const messages: ChatMessage[] = [
        {
          role: 'system',
          content: this.config.personality.systemPrompt,
          timestamp: Date.now()
        },
        ...this.chatHistory.slice(-this.config.contextWindow)
      ];

      // 调用 LLM API
      this.setStatus('generating');
      const response = await this.callLLM({
        messages,
        model: this.config.model,
        maxTokens: this.config.maxTokens,
        temperature: this.config.temperature,
        stream: false
      });

      const latency = Date.now() - startTime;

      // 处理回复
      let replyText = response.content.trim();

      // 截断过长回复
      if (replyText.length > this.config.maxResponseLength) {
        replyText = replyText.substring(0, this.config.maxResponseLength) + '...';
      }

      // 敏感词过滤
      if (this.config.sensitiveWordsFilter) {
        // TODO: 实现敏感词过滤
      }

      const aiResponse: AIResponse = {
        text: replyText,
        shouldSend: true,
        confidence: 0.8,
        tokensUsed: response.usage.totalTokens,
        latency
      };

      // 添加到上下文
      this.addToContext({
        role: 'assistant',
        content: replyText,
        timestamp: Date.now()
      });

      this.setStatus('idle');
      return aiResponse;
    } catch (error) {
      this.setStatus('error');
      throw error;
    }
  }

  /**
   * 调用 LLM API
   */
  private async callLLM(request: LLMRequest): Promise<LLMResponse> {
    // 通过 IPC 调用主进程的 LLM 服务
    const result = await window.electron.ipcRenderer.invoke('ai:chat', {
      provider: this.config.provider,
      apiKey: this.config.apiKey,
      model: request.model,
      endpoint: this.config.endpoint,
      messages: request.messages,
      maxTokens: request.maxTokens,
      temperature: request.temperature
    });

    if (!result.success) {
      throw new Error(result.error);
    }

    return result.response;
  }

  /**
   * 清空上下文
   */
  clearContext(): void {
    this.chatHistory = [];
  }

  /**
   * 获取聊天历史
   */
  getChatHistory(): ChatMessage[] {
    return [...this.chatHistory];
  }

  /**
   * 手动生成回复
   */
  async manualReply(prompt: string): Promise<AIResponse> {
    this.addToContext({
      role: 'user',
      content: prompt,
      timestamp: Date.now()
    });

    return this.generateReply();
  }

  /**
   * 检查 API 是否可用
   */
  async checkAvailability(): Promise<{ available: boolean; error?: string }> {
    try {
      const result = await window.electron.ipcRenderer.invoke('ai:checkAvailability', {
        provider: this.config.provider,
        apiKey: this.config.apiKey,
        endpoint: this.config.endpoint
      });
      return result;
    } catch (error) {
      return {
        available: false,
        error: (error as Error).message
      };
    }
  }

  /**
   * 销毁服务
   */
  destroy(): void {
    this.clearContext();
    this.statusListeners = [];
    this.responseListeners = [];
    this.errorListeners = [];
  }
}

// 单例
let llmServiceInstance: LLMService | null = null;

export function getLLMService(): LLMService {
  if (!llmServiceInstance) {
    llmServiceInstance = new LLMService();
  }
  return llmServiceInstance;
}
