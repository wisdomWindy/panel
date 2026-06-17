/**
 * AI 状态管理
 * 管理 LLM 对话生成相关的状态和操作
 */

import { defineStore } from 'pinia';
import { ref, computed } from 'vue';
import type { AIConfig, AIResponse, AIStatus, AIError, ChatMessage } from '../modules/ai/types.js';
import { DEFAULT_AI_CONFIG, PRESET_PERSONALITIES } from '../modules/ai/types.js';
import { getLLMService } from '../services/llm-service.js';

export const useAIStore = defineStore('ai', () => {
  // State
  const config = ref<AIConfig>({ ...DEFAULT_AI_CONFIG });
  const status = ref<AIStatus>('idle');
  const isAvailable = ref<boolean>(false);
  const recentResponses = ref<AIResponse[]>([]);
  const chatHistory = ref<ChatMessage[]>([]);
  const lastError = ref<AIError | null>(null);
  const totalTokensUsed = ref<number>(0);

  // Getters
  const isEnabled = computed(() => config.value.enabled);
  const isThinking = computed(() => status.value === 'thinking');
  const isGenerating = computed(() => status.value === 'generating');

  // Actions
  function updateConfig(updates: Partial<AIConfig>): void {
    config.value = { ...config.value, ...updates };
    const llmService = getLLMService();
    llmService.updateConfig(updates);
    saveConfig();
  }

  function toggleEnabled(): void {
    updateConfig({ enabled: !config.value.enabled });
  }

  function setPersonality(personalityKey: string): void {
    const personality = PRESET_PERSONALITIES[personalityKey];
    if (personality) {
      updateConfig({ personality });
    }
  }

  async function startListening(): Promise<void> {
    try {
      const llmService = getLLMService();
      llmService.updateConfig(config.value);

      // 设置监听器
      llmService.onResponse((response) => {
        addResponse(response);
      });

      llmService.onStatusChange((newStatus) => {
        status.value = newStatus;
      });

      llmService.onError((error) => {
        lastError.value = error;
      });
    } catch (error) {
      lastError.value = {
        code: 'start-failed',
        message: (error as Error).message
      };
      throw error;
    }
  }

  function stopListening(): void {
    const llmService = getLLMService();
    llmService.destroy();
    status.value = 'idle';
  }

  function addResponse(response: AIResponse): void {
    recentResponses.value.unshift(response);
    totalTokensUsed.value += response.tokensUsed;
    // 只保留最近 50 条回复
    if (recentResponses.value.length > 50) {
      recentResponses.value = recentResponses.value.slice(0, 50);
    }
  }

  function clearResponses(): void {
    recentResponses.value = [];
  }

  function clearError(): void {
    lastError.value = null;
  }

  function clearContext(): void {
    const llmService = getLLMService();
    llmService.clearContext();
    chatHistory.value = [];
  }

  async function manualReply(prompt: string): Promise<AIResponse> {
    const llmService = getLLMService();
    return llmService.manualReply(prompt);
  }

  async function checkAvailability(): Promise<void> {
    const llmService = getLLMService();
    const result = await llmService.checkAvailability();
    isAvailable.value = result.available;
    if (!result.available && result.error) {
      lastError.value = {
        code: 'unavailable',
        message: result.error
      };
    }
  }

  function saveConfig(): void {
    try {
      localStorage.setItem('ai-config', JSON.stringify(config.value));
    } catch (error) {
      console.error('Failed to save AI config:', error);
    }
  }

  function loadConfig(): void {
    try {
      const saved = localStorage.getItem('ai-config');
      if (saved) {
        const parsed = JSON.parse(saved);
        config.value = { ...DEFAULT_AI_CONFIG, ...parsed };
      }
    } catch (error) {
      console.error('Failed to load AI config:', error);
    }
  }

  // 初始化
  loadConfig();

  return {
    // State
    config,
    status,
    isAvailable,
    recentResponses,
    chatHistory,
    lastError,
    totalTokensUsed,

    // Getters
    isEnabled,
    isThinking,
    isGenerating,

    // Actions
    updateConfig,
    toggleEnabled,
    setPersonality,
    startListening,
    stopListening,
    addResponse,
    clearResponses,
    clearError,
    clearContext,
    manualReply,
    checkAvailability,
    saveConfig,
    loadConfig
  };
});
