/**
 * AI 状态管理
 * 管理 LLM 对话生成相关的状态和操作
 */

import { defineStore } from 'pinia';
import { ref, computed } from 'vue';
import type { AIConfig, AIResponse, AIStatus, AIError, ChatMessage } from '../modules/ai/types.js';
import { DEFAULT_AI_CONFIG, PRESET_PERSONALITIES } from '../modules/ai/types.js';
import { getLLMService } from '../services/llm-service.js';
import { useChatContext } from '../composables/useChatContext.js';
import { useReplyLogic } from '../composables/useReplyLogic.js';

export const useAIStore = defineStore('ai', () => {
  const llmService = getLLMService();
  const chatContext = useChatContext();
  const replyLogic = useReplyLogic();

  // State
  const config = ref<AIConfig>({ ...DEFAULT_AI_CONFIG });
  const status = ref<AIStatus>('idle');
  const isAvailable = ref<boolean>(false);
  const recentResponses = ref<AIResponse[]>([]);
  const lastError = ref<AIError | null>(null);
  const totalTokensUsed = ref<number>(0);
  const lastReplyTime = ref<number>(0);

  // Getters
  const isEnabled = computed(() => config.value.enabled);
  const isThinking = computed(() => status.value === 'thinking');
  const isGenerating = computed(() => status.value === 'generating');

  // Actions
  function updateConfig(updates: Partial<AIConfig>): void {
    config.value = { ...config.value, ...updates };
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

  async function handleDanmaku(username: string, content: string): Promise<void> {
    if (!config.value.enabled || !config.value.autoReply) return;
    if (!replyLogic.shouldReply(content, config.value)) return;
    if (!replyLogic.checkCooldown(lastReplyTime.value, config.value.minInterval)) return;

    chatContext.addToContext({
      role: 'user',
      content: `${username}: ${content}`,
      timestamp: Date.now(),
      username
    });

    try {
      const response = await generateReply();
      if (response.shouldSend) {
        lastReplyTime.value = Date.now();
        addResponse(response);
      }
    } catch (error) {
      lastError.value = {
        code: 'generate-failed',
        message: (error as Error).message
      };
    }
  }

  async function generateReply(): Promise<AIResponse> {
    const startTime = Date.now();
    status.value = 'thinking';

    try {
      const messages: ChatMessage[] = [
        {
          role: 'system',
          content: config.value.personality.systemPrompt,
          timestamp: Date.now()
        },
        ...chatContext.getContextWindow(config.value.contextWindow)
      ];

      status.value = 'generating';
      const response = await llmService.callLLM({
        messages,
        model: config.value.model,
        maxTokens: config.value.maxTokens,
        temperature: config.value.temperature,
        stream: false
      });

      const latency = Date.now() - startTime;
      const replyText = replyLogic.truncateReply(response.content.trim(), config.value.maxResponseLength);

      const aiResponse: AIResponse = {
        text: replyText,
        shouldSend: true,
        confidence: 0.8,
        tokensUsed: response.usage.totalTokens,
        latency
      };

      chatContext.addToContext({
        role: 'assistant',
        content: replyText,
        timestamp: Date.now()
      });

      status.value = 'idle';
      return aiResponse;
    } catch (error) {
      status.value = 'error';
      throw error;
    }
  }

  function addResponse(response: AIResponse): void {
    recentResponses.value.unshift(response);
    totalTokensUsed.value += response.tokensUsed;
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
    chatContext.clearContext();
  }

  async function manualReply(prompt: string): Promise<AIResponse> {
    chatContext.addToContext({
      role: 'user',
      content: prompt,
      timestamp: Date.now()
    });
    return generateReply();
  }

  async function checkAvailability(): Promise<void> {
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

  loadConfig();

  return {
    config,
    status,
    isAvailable,
    recentResponses,
    lastError,
    totalTokensUsed,
    isEnabled,
    isThinking,
    isGenerating,
    updateConfig,
    toggleEnabled,
    setPersonality,
    handleDanmaku,
    generateReply,
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
