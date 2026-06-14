/**
 * 弹幕发送器
 * 支持频率限制、重试机制、错误处理
 */

import axios from 'axios';
import { ClientError } from '../bilibili/types';

/**
 * 发送结果
 */
interface SendResult {
  success: boolean;
  message: string;
  retryable: boolean;
}

/**
 * 频率限制器
 */
class RateLimiter {
  private lastSendTime: number = 0;
  private minInterval: number; // 最小发送间隔（毫秒）

  constructor(minInterval: number = 3000) {
    this.minInterval = minInterval;
  }

  /**
   * 检查是否可以发送
   */
  canSend(): boolean {
    const now = Date.now();
    return now - this.lastSendTime >= this.minInterval;
  }

  /**
   * 记录发送时间
   */
  recordSend(): void {
    this.lastSendTime = Date.now();
  }

  /**
   * 获取下次可发送时间
   */
  getNextSendTime(): number {
    return this.lastSendTime + this.minInterval;
  }
}

/**
 * 弹幕发送器
 */
export class DanmakuSender {
  private rateLimiter: RateLimiter;
  private maxRetries: number = 3;
  private retryDelay: number = 1000;
  private csrfToken?: string;
  private rateLimitBackoff: boolean = false;
  private originalMinInterval: number;

  constructor(minInterval: number = 3000, csrfToken?: string) {
    this.originalMinInterval = minInterval;
    this.rateLimiter = new RateLimiter(minInterval);
    this.csrfToken = csrfToken;
  }

  /**
   * 发送弹幕
   * @param content 弹幕内容
   * @param roomId 房间号
   * @param cookie Cookie
   * @returns 发送结果
   */
  async send(content: string, roomId: number, cookie?: string): Promise<SendResult> {
    // 检查频率限制
    if (!this.rateLimiter.canSend()) {
      const waitTime = this.rateLimiter.getNextSendTime() - Date.now();
      return {
        success: false,
        message: `发送过于频繁，请等待 ${Math.ceil(waitTime / 1000)} 秒`,
        retryable: true
      };
    }

    // 检查内容
    if (!content || content.trim().length === 0) {
      return {
        success: false,
        message: '弹幕内容不能为空',
        retryable: false
      };
    }

    if (content.length > 30) {
      return {
        success: false,
        message: '弹幕内容不能超过 30 个字符',
        retryable: false
      };
    }

    // 发送弹幕
    const result = await this.sendWithRetry(content, roomId, cookie);

    if (result.success) {
      this.rateLimiter.recordSend();
    }

    return result;
  }

  /**
   * 带重试的发送
   */
  private async sendWithRetry(
    content: string,
    roomId: number,
    cookie?: string,
    retryCount: number = 0
  ): Promise<SendResult> {
    try {
      const response = await this.sendRequest(content, roomId, cookie);

      if (response.code === 0) {
        return {
          success: true,
          message: '发送成功',
          retryable: false
        };
      } else {
        // 检查是否需要重试
        const retryable = this.isRetryable(response.code, response.message);

        if (retryable && retryCount < this.maxRetries) {
          await this.delay(this.retryDelay * (retryCount + 1));
          return this.sendWithRetry(content, roomId, cookie, retryCount + 1);
        }

        return {
          success: false,
          message: response.message || '发送失败',
          retryable
        };
      }
    } catch (error) {
      if (retryCount < this.maxRetries) {
        await this.delay(this.retryDelay * (retryCount + 1));
        return this.sendWithRetry(content, roomId, cookie, retryCount + 1);
      }

      return {
        success: false,
        message: `发送失败: ${error}`,
        retryable: true
      };
    }
  }

  /**
   * 发送 HTTP 请求
   */
  private async sendRequest(
    content: string,
    roomId: number,
    cookie?: string
  ): Promise<{ code: number; message: string }> {
    const url = 'https://api.live.bilibili.com/msg/send';

    // 提取 CSRF token
    const csrf = this.csrfToken || this.extractCsrfToken(cookie || '');

    if (!csrf) {
      throw new ClientError('CSRF token not found');
    }

    const response = await axios.post(
      url,
      {
        bubble: 0,
        msg: content,
        roomid: roomId,
        color: 16777215, // 白色
        mode: 1, // 普通弹幕
        fontsize: 25,
        rnd: Date.now()
      },
      {
        headers: {
          'Content-Type': 'application/x-www-form-urlencoded',
          Cookie: cookie || '',
          Referer: `https://live.bilibili.com/${roomId}`
        },
        params: {
          csrf: csrf
        }
      }
    );

    return response.data;
  }

  /**
   * 从 Cookie 中提取 CSRF token
   */
  private extractCsrfToken(cookie: string): string | null {
    const match = cookie.match(/bili_jct=([^;]+)/);
    return match ? match[1] : null;
  }

  /**
   * 判断是否可重试
   */
  private isRetryable(code: number, message?: string): boolean {
    // 风控相关错误不可重试
    const nonRetryableCodes = [1003, 1004, 1005, 1006];

    if (nonRetryableCodes.includes(code)) {
      return false;
    }

    // 检查消息中是否包含风控关键词
    if (message) {
      const keywords = ['风控', '封禁', '违规', '频繁'];
      if (keywords.some(keyword => message.includes(keyword))) {
        return false;
      }
    }

    return true;
  }

  /**
   * 延迟
   */
  private delay(ms: number): Promise<void> {
    return new Promise(resolve => setTimeout(resolve, ms));
  }

  /**
   * 设置 CSRF token
   */
  setCsrfToken(token: string): void {
    this.csrfToken = token;
  }

  /**
   * 设置最小发送间隔
   */
  setMinInterval(interval: number): void {
    this.originalMinInterval = interval;
    this.rateLimiter = new RateLimiter(interval);
  }

  /**
   * 风控降频 — 被平台限流后将间隔翻倍（最大 60s）
   */
  applyRateLimitBackoff(): void {
    if (this.rateLimitBackoff) return;
    this.rateLimitBackoff = true;
    const newInterval = Math.min(
      (this.rateLimiter as unknown as { minInterval: number }).minInterval * 2 || this.originalMinInterval * 2,
      60000
    );
    this.rateLimiter = new RateLimiter(newInterval);
    console.warn(`风控降频: 发送间隔调整为 ${newInterval / 1000}s`);
  }

  /**
   * 恢复原始发送间隔
   */
  resetRateLimitBackoff(): void {
    if (!this.rateLimitBackoff) return;
    this.rateLimitBackoff = false;
    this.rateLimiter = new RateLimiter(this.originalMinInterval);
    console.warn('风控降频已恢复');
  }

  /**
   * 获取下次可发送时间
   */
  getNextSendTime(): number {
    return this.rateLimiter.getNextSendTime();
  }

  /**
   * 检查是否可以发送
   */
  canSend(): boolean {
    return this.rateLimiter.canSend();
  }
}
