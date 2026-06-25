/**
 * 回复判断逻辑 composable
 * 判断是否应该回复弹幕消息
 */

import type { AIConfig } from '../modules/ai/types.js';

export function useReplyLogic() {
  function shouldReply(content: string, config: AIConfig): boolean {
    // 检查是否是问题
    if (config.replyToQuestions) {
      const questionPatterns = ['?', '？', '吗', '呢', '什么', '怎么', '为什么', '哪'];
      if (questionPatterns.some(p => content.includes(p))) {
        return true;
      }
    }

    // 检查是否是 @
    if (config.replyToMentions) {
      if (content.includes('@') || content.includes('艾特')) {
        return true;
      }
    }

    // 默认自动回复
    return config.autoReply;
  }

  function checkCooldown(lastReplyTime: number, minInterval: number): boolean {
    const now = Date.now();
    return now - lastReplyTime >= minInterval * 1000;
  }

  function truncateReply(text: string, maxLength: number): string {
    if (text.length > maxLength) {
      return text.substring(0, maxLength) + '...';
    }
    return text;
  }

  return {
    shouldReply,
    checkCooldown,
    truncateReply
  };
}
