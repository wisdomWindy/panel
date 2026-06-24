/**
 * 聊天上下文 composable
 * 管理 LLM 对话历史和上下文窗口
 */

import { ref } from 'vue';
import type { ChatMessage } from '../modules/ai/types.js';

export function useChatContext(maxWindowSize: number = 10) {
  const chatHistory = ref<ChatMessage[]>([]);

  function addToContext(message: ChatMessage): void {
    chatHistory.value.push(message);
    if (chatHistory.value.length > maxWindowSize) {
      chatHistory.value.shift();
    }
  }

  function clearContext(): void {
    chatHistory.value = [];
  }

  function getHistory(): ChatMessage[] {
    return [...chatHistory.value];
  }

  function getContextWindow(windowSize: number): ChatMessage[] {
    return chatHistory.value.slice(-windowSize);
  }

  function updateWindowSize(size: number): void {
    while (chatHistory.value.length > size) {
      chatHistory.value.shift();
    }
  }

  return {
    chatHistory,
    addToContext,
    clearContext,
    getHistory,
    getContextWindow,
    updateWindowSize
  };
}
