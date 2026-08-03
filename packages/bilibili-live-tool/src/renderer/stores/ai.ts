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

function mergeAIConfig(current: AIConfig, updates: unknown): AIConfig {
  if (!updates || typeof updates !== 'object') {
    return current;
  }

  const source = updates as Record<string, unknown>;
  const personalitySource =
    source.personality && typeof source.personality === 'object'
      ? (source.personality as Record<string, unknown>)
      : {};
  const provider = source.provider;
  const tone = personalitySource.tone;
  const responseStyle = personalitySource.responseStyle;
  const readString = (key: keyof AIConfig, fallback: string) =>
    typeof source[key] === 'string' ? source[key] : fallback;
  const readNumber = (key: keyof AIConfig, fallback: number) =>
    typeof source[key] === 'number' ? source[key] : fallback;

  return {
    enabled: typeof source.enabled === 'boolean' ? source.enabled : current.enabled,
    provider:
      provider === 'openai' ||
      provider === 'claude' ||
      provider === 'ollama' ||
      provider === 'custom'
        ? provider
        : current.provider,
    apiKey: readString('apiKey', current.apiKey),
    model: readString('model', current.model),
    endpoint: readString('endpoint', current.endpoint),
    personality: {
      name:
        typeof personalitySource.name === 'string'
          ? personalitySource.name
          : current.personality.name,
      tone:
        tone === 'humorous' ||
        tone === 'gentle' ||
        tone === 'professional' ||
        tone === 'anime' ||
        tone === 'custom' ||
        tone === 'friendly'
          ? tone
          : current.personality.tone,
      responseStyle:
        responseStyle === 'casual' ||
        responseStyle === 'formal' ||
        responseStyle === 'friendly' ||
        responseStyle === 'custom'
          ? responseStyle
          : current.personality.responseStyle,
      systemPrompt:
        typeof personalitySource.systemPrompt === 'string'
          ? personalitySource.systemPrompt
          : current.personality.systemPrompt,
      examples: Array.isArray(personalitySource.examples)
        ? personalitySource.examples.filter(
            (example): example is string => typeof example === 'string'
          )
        : current.personality.examples
    },
    contextWindow: readNumber('contextWindow', current.contextWindow),
    maxTokens: readNumber('maxTokens', current.maxTokens),
    temperature: readNumber('temperature', current.temperature),
    maxResponseLength: readNumber('maxResponseLength', current.maxResponseLength),
    sensitiveWordsFilter:
      typeof source.sensitiveWordsFilter === 'boolean'
        ? source.sensitiveWordsFilter
        : current.sensitiveWordsFilter
  };
}

export const useAIStore = defineStore('ai', () => {
  const llmService = getLLMService();
  const chatContext = useChatContext();

  // State
  const config = ref<AIConfig>({ ...DEFAULT_AI_CONFIG });
  const status = ref<AIStatus>('idle');
  const isAvailable = ref<boolean>(false);
  const recentResponses = ref<AIResponse[]>([]);
  const lastError = ref<AIError | null>(null);
  const totalTokensUsed = ref<number>(0);

  // Getters
  const isEnabled = computed(() => config.value.enabled);
  const isThinking = computed(() => status.value === 'thinking');
  const isGenerating = computed(() => status.value === 'generating');

  // Actions
  function updateConfig(updates: Partial<AIConfig> | Record<string, unknown>): void {
    config.value = mergeAIConfig(config.value, updates);
    llmService.updateConfig(config.value);
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
      const replyText = response.content.trim().slice(0, config.value.maxResponseLength);

      const aiResponse: AIResponse = {
        text: replyText,
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
        config.value = mergeAIConfig(DEFAULT_AI_CONFIG, JSON.parse(saved));
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
