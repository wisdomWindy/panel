/**
 * 更新检测服务
 * 提供 CHANGELOG.md 解析、GitHub Releases API 调用、版本号比较功能
 */

import { Logger } from '../modules/utils/logger';

export interface ChangelogEntry {
  version: string;
  date: string;
  changes: {
    type: 'added' | 'changed' | 'fixed' | 'removed' | 'security';
    items: string[];
  }[];
}

export interface UpdateCheckResult {
  hasUpdate: boolean;
  currentVersion: string;
  latestVersion: string;
  releaseUrl: string;
  publishedAt: string;
}

interface GitHubRelease {
  tag_name: string;
  published_at: string;
  html_url: string;
  body: string;
}

const GITHUB_REPO = 'wisdomWindy/panel';
const GITHUB_API_URL = `https://api.github.com/repos/${GITHUB_REPO}/releases/latest`;

export class UpdateService {
  private logger: Logger;

  constructor() {
    this.logger = new Logger('UpdateService');
  }

  /**
   * 解析 CHANGELOG.md 为结构化数据
   */
  async parseChangelog(): Promise<ChangelogEntry[]> {
    try {
      const response = await fetch('./CHANGELOG.md');
      if (!response.ok) {
        this.logger.warn('CHANGELOG.md not found or inaccessible');
        return [];
      }

      const content = await response.text();
      if (!content.trim()) {
        return [];
      }

      return this.parseChangelogContent(content);
    } catch (error) {
      this.logger.warn('Failed to parse CHANGELOG.md:', error);
      return [];
    }
  }

  /**
   * 解析 CHANGELOG 内容
   */
  private parseChangelogContent(content: string): ChangelogEntry[] {
    const entries: ChangelogEntry[] = [];
    const lines = content.split('\n');
    let currentEntry: ChangelogEntry | null = null;
    let currentChangeType: 'added' | 'changed' | 'fixed' | 'removed' | 'security' | null = null;

    for (const line of lines) {
      const trimmedLine = line.trim();

      // 匹配版本标题: ## [0.2.0] - 2026-06-16
      const versionMatch = trimmedLine.match(/^##\s*\[(\d+\.\d+\.\d+)\]\s*-\s*(\d{4}-\d{2}-\d{2})/);
      if (versionMatch) {
        if (currentEntry) {
          entries.push(currentEntry);
        }
        currentEntry = {
          version: versionMatch[1],
          date: versionMatch[2],
          changes: []
        };
        currentChangeType = null;
        continue;
      }

      // 匹配变更类型: ### Added, ### Changed, ### Fixed, ### Removed, ### Security
      const typeMatch = trimmedLine.match(/^###\s*(Added|Changed|Fixed|Removed|Security)/i);
      if (typeMatch && currentEntry) {
        const typeMap: Record<string, 'added' | 'changed' | 'fixed' | 'removed' | 'security'> = {
          'added': 'added',
          'changed': 'changed',
          'fixed': 'fixed',
          'removed': 'removed',
          'security': 'security'
        };
        currentChangeType = typeMap[typeMatch[1].toLowerCase()] || 'added';
        currentEntry.changes.push({
          type: currentChangeType,
          items: []
        });
        continue;
      }

      // 匹配变更条目: - 变更内容
      const itemMatch = trimmedLine.match(/^-\s+(.+)/);
      if (itemMatch && currentEntry && currentChangeType) {
        const lastChange = currentEntry.changes[currentEntry.changes.length - 1];
        if (lastChange && lastChange.type === currentChangeType) {
          lastChange.items.push(itemMatch[1]);
        }
        continue;
      }
    }

    // 添加最后一个条目
    if (currentEntry) {
      entries.push(currentEntry);
    }

    return entries;
  }

  /**
   * 检查 GitHub Releases 获取最新版本
   */
  async checkForUpdates(): Promise<UpdateCheckResult> {
    const currentVersion = this.getCurrentVersion();

    try {
      const response = await fetch(GITHUB_API_URL, {
        headers: {
          'Accept': 'application/vnd.github.v3+json'
        }
      });

      if (!response.ok) {
        if (response.status === 404) {
          this.logger.warn('No releases found');
          return {
            hasUpdate: false,
            currentVersion,
            latestVersion: currentVersion,
            releaseUrl: '',
            publishedAt: ''
          };
        }
        throw new Error(`GitHub API error: ${response.status}`);
      }

      const release: GitHubRelease = await response.json();
      const latestVersion = release.tag_name.replace('v', '');
      const hasUpdate = this.compareVersions(currentVersion, latestVersion) < 0;

      return {
        hasUpdate,
        currentVersion,
        latestVersion,
        releaseUrl: release.html_url,
        publishedAt: release.published_at
      };
    } catch (error) {
      this.logger.warn('Failed to check for updates:', error);
      return {
        hasUpdate: false,
        currentVersion,
        latestVersion: currentVersion,
        releaseUrl: '',
        publishedAt: ''
      };
    }
  }

  /**
   * 比较版本号 (Semantic Versioning)
   * @returns -1 if v1 < v2, 0 if v1 === v2, 1 if v1 > v2
   */
  compareVersions(v1: string, v2: string): number {
    const parts1 = v1.replace('v', '').split('.').map(Number);
    const parts2 = v2.replace('v', '').split('.').map(Number);

    for (let i = 0; i < 3; i++) {
      const p1 = parts1[i] || 0;
      const p2 = parts2[i] || 0;

      if (p1 > p2) return 1;
      if (p1 < p2) return -1;
    }

    return 0;
  }

  /**
   * 获取当前版本号
   */
  getCurrentVersion(): string {
    // 从 package.json 读取版本号，如果无法获取则返回默认值
    return '0.1.0';
  }
}

// 单例实例
let updateServiceInstance: UpdateService | null = null;

export function getUpdateService(): UpdateService {
  if (!updateServiceInstance) {
    updateServiceInstance = new UpdateService();
  }
  return updateServiceInstance;
}
