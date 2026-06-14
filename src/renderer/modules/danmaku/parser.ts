/**
 * 弹幕解析器
 * 从开放平台消息结构中解析弹幕数据、提取用户信息、识别用户身份
 */

import { Danmaku } from './types';
import { OpenPlatformDanmakuMessage } from '../bilibili/types';

/**
 * 解析后的弹幕数据
 */
export interface ParsedDanmaku extends Danmaku {
  roomRank?: number;
  userMedal?: {
    level: number;
    name: string;
    color: string;
  };
  userGuard?: number;
  isVip: boolean;
  isSvip: boolean;
  isCaptain: boolean;
  isAdmin: boolean;
  isOwner: boolean;
}

/**
 * 弹幕解析器
 */
export class DanmakuParser {
  /**
   * 解析弹幕消息
   * @param message - 开放平台弹幕消息
   * @param roomId - 房间号
   * @param ownerId - 房主 ID (open_id)
   * @returns 解析后的弹幕
   */
  static parse(
    message: OpenPlatformDanmakuMessage,
    _roomId: number,
    ownerOpenId: string
  ): ParsedDanmaku {
    const data = message.data;

    // 基础信息
    const danmaku: ParsedDanmaku = {
      id: data.msg_id || `${data.timestamp}_${data.open_id}`,
      userId: data.open_id,
      username: data.uname,
      content: data.msg,
      timestamp: data.timestamp,
      userType: this.determineUserType(data, ownerOpenId),
      isVip: false,
      isSvip: false,
      isCaptain: data.guard_level > 0,
      isAdmin: data.is_admin === 1,
      isOwner: ownerOpenId ? data.open_id === ownerOpenId : false
    };

    // 粉丝牌信息
    if (data.fans_medal_level > 0) {
      danmaku.medal = {
        level: data.fans_medal_level,
        name: data.fans_medal_name,
        color: '#FB7299' // B站粉色
      };
    }

    // 舰长等级
    if (data.guard_level > 0) {
      danmaku.userGuard = data.guard_level;
    }

    // 荣耀等级
    if (data.glory_level) {
      danmaku.badge = {
        level: data.glory_level,
        name: `荣耀等级 ${data.glory_level}`
      };
    }

    return danmaku;
  }

  /**
   * 确定用户类型
   */
  private static determineUserType(
    data: OpenPlatformDanmakuMessage['data'],
    ownerOpenId: string
  ): 'normal' | 'captain' | 'admin' | 'owner' {
    // 房主
    if (ownerOpenId && data.open_id === ownerOpenId) {
      return 'owner';
    }

    // 房管
    if (data.is_admin === 1) {
      return 'admin';
    }

    // 舰长 (guard_level > 0 表示: 1=总督 2=提督 3=舰长)
    if (data.guard_level > 0) {
      return 'captain';
    }

    return 'normal';
  }

  /**
   * 批量解析弹幕
   * @param messages - 弹幕消息列表
   * @param roomId - 房间号
   * @param ownerId - 房主 ID
   * @returns 解析后的弹幕列表
   */
  static parseBatch(
    messages: OpenPlatformDanmakuMessage[],
    roomId: number,
    ownerOpenId: string
  ): ParsedDanmaku[] {
    return messages.map(msg => this.parse(msg, roomId, ownerOpenId));
  }

  /**
   * 过滤弹幕
   * @param danmakus 弹幕列表
   * @param filters 过滤条件
   * @returns 过滤后的弹幕列表
   */
  static filter(
    danmakus: ParsedDanmaku[],
    filters: {
      userType?: ('normal' | 'captain' | 'admin' | 'owner')[];
      minLevel?: number;
      keywords?: string[];
      excludeKeywords?: string[];
    }
  ): ParsedDanmaku[] {
    return danmakus.filter(danmaku => {
      // 用户类型过滤
      if (filters.userType && !filters.userType.includes(danmaku.userType)) {
        return false;
      }

      // 最低等级过滤
      if (filters.minLevel && danmaku.badge && danmaku.badge.level < filters.minLevel) {
        return false;
      }

      // 关键词过滤
      if (filters.keywords && filters.keywords.length > 0) {
        const hasKeyword = filters.keywords.some(keyword => danmaku.content.includes(keyword));
        if (!hasKeyword) {
          return false;
        }
      }

      // 排除关键词过滤
      if (filters.excludeKeywords && filters.excludeKeywords.length > 0) {
        const hasExcludeKeyword = filters.excludeKeywords.some(keyword =>
          danmaku.content.includes(keyword)
        );
        if (hasExcludeKeyword) {
          return false;
        }
      }

      return true;
    });
  }

  /**
   * 统计弹幕
   * @param danmakus 弹幕列表
   * @returns 统计信息
   */
  static statistics(danmakus: ParsedDanmaku[]): {
    total: number;
    byUserType: Record<string, number>;
    byHour: Record<string, number>;
    topUsers: Array<{ username: string; count: number }>;
  } {
    const stats = {
      total: danmakus.length,
      byUserType: {} as Record<string, number>,
      byHour: {} as Record<string, number>,
      topUsers: {} as Record<string, number>
    };

    danmakus.forEach(danmaku => {
      stats.byUserType[danmaku.userType] = (stats.byUserType[danmaku.userType] || 0) + 1;

      const hour = new Date(danmaku.timestamp * 1000).getHours().toString();
      stats.byHour[hour] = (stats.byHour[hour] || 0) + 1;

      stats.topUsers[danmaku.username] = (stats.topUsers[danmaku.username] || 0) + 1;
    });

    const sortedUsers = Object.entries(stats.topUsers)
      .sort(([, a], [, b]) => b - a)
      .slice(0, 10)
      .map(([username, count]) => ({ username, count }));

    return {
      ...stats,
      topUsers: sortedUsers
    };
  }
}
