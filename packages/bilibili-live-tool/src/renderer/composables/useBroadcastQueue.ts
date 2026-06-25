/**
 * 播报队列 composable
 * 管理 TTS 播报队列和处理逻辑
 */

import { ref, onUnmounted } from 'vue';
import type { BroadcastMessage, TTSConfig } from '../modules/tts/types.js';
import { MAX_QUEUE_SIZE } from '../modules/tts/types.js';

export function useBroadcastQueue() {
  const queue = ref<BroadcastMessage[]>([]);
  const isProcessing = ref(false);
  const lastBroadcastTime = ref(0);

  function addToQueue(message: BroadcastMessage): void {
    if (queue.value.length >= MAX_QUEUE_SIZE) {
      queue.value.shift();
    }
    queue.value.push(message);
  }

  function shouldFilter(content: string, filterKeywords: string[]): boolean {
    const lowerContent = content.toLowerCase();
    return filterKeywords.some(keyword => lowerContent.includes(keyword.toLowerCase()));
  }

  function formatMessage(message: BroadcastMessage, config: TTSConfig): string {
    let template = config.formatTemplate;

    if (message.type === 'gift') {
      template = '感谢{username}送出的{giftName}';
    } else if (message.type === 'captain') {
      template = '欢迎{username}开通{captainLevel}';
    }

    return template
      .replace('{username}', message.username)
      .replace('{content}', message.content)
      .replace('{giftName}', message.giftName || '')
      .replace('{giftCount}', String(message.giftCount || ''))
      .replace('{captainLevel}', message.content);
  }

  async function processQueue(
    speakFn: (text: string) => Promise<void>,
    config: TTSConfig
  ): Promise<void> {
    if (isProcessing.value || queue.value.length === 0) return;

    isProcessing.value = true;

    while (queue.value.length > 0) {
      const now = Date.now();
      const timeSinceLastBroadcast = now - lastBroadcastTime.value;
      if (timeSinceLastBroadcast < config.minInterval) {
        await new Promise(resolve =>
          setTimeout(resolve, config.minInterval - timeSinceLastBroadcast)
        );
      }

      const message = queue.value.shift();
      if (message) {
        const text = formatMessage(message, config);
        await speakFn(text);
        lastBroadcastTime.value = Date.now();
      }
    }

    isProcessing.value = false;
  }

  function clearQueue(): void {
    queue.value = [];
    isProcessing.value = false;
  }

  function getQueueLength(): number {
    return queue.value.length;
  }

  onUnmounted(clearQueue);

  return {
    queue,
    isProcessing,
    addToQueue,
    shouldFilter,
    formatMessage,
    processQueue,
    clearQueue,
    getQueueLength
  };
}
