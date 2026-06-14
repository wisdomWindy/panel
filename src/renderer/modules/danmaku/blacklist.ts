/**
 * 黑名单管理器
 * 管理禁言用户列表，检测刷屏行为
 */

import { EventEmitter } from 'events';
import { BlacklistItem } from './types';

export class BlacklistManager extends EventEmitter {
  private blacklist: Map<string, BlacklistItem> = new Map();
  private recentMessages: Map<string, { content: string; timestamp: number }[]> = new Map();

  addToBlacklist(userId: string, username: string, reason: string, duration?: number): void {
    const bannedAt = Date.now();
    const bannedUntil = duration && duration > 0 ? bannedAt + duration * 60000 : 0;

    const item: BlacklistItem = { userId, username, reason, bannedAt, bannedUntil };
    this.blacklist.set(userId, item);
    this.emit('banned', item);
  }

  removeFromBlacklist(userId: string): void {
    const item = this.blacklist.get(userId);
    if (item) {
      this.blacklist.delete(userId);
      this.emit('unbanned', item);
    }
  }

  isBlacklisted(userId: string): boolean {
    const item = this.blacklist.get(userId);
    if (!item) return false;
    if (item.bannedUntil !== 0 && Date.now() > item.bannedUntil) {
      this.blacklist.delete(userId);
      return false;
    }
    return true;
  }

  getBlacklist(): BlacklistItem[] {
    const items: BlacklistItem[] = [];
    const now = Date.now();
    for (const [userId, item] of this.blacklist) {
      if (item.bannedUntil === 0 || now <= item.bannedUntil) {
        items.push(item);
      } else {
        this.blacklist.delete(userId);
      }
    }
    return items;
  }

  checkSpam(userId: string, content: string, interval: number = 3000): boolean {
    const now = Date.now();
    const messages = this.recentMessages.get(userId) || [];

    const recent = messages.filter(m => now - m.timestamp < interval);
    const isSpam = recent.some(m => m.content === content);

    recent.push({ content, timestamp: now });
    if (recent.length > 50) recent.splice(0, recent.length - 50);
    this.recentMessages.set(userId, recent);

    if (isSpam) {
      this.emit('spamDetected', { userId, content, timestamp: now });
    }
    return isSpam;
  }

  clear(): void {
    this.blacklist.clear();
    this.recentMessages.clear();
  }
}

export const blacklistManager = new BlacklistManager();
