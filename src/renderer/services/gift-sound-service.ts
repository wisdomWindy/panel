/**
 * 礼物音效服务
 * 处理礼物音效配置的业务逻辑
 */

import type {
  GiftSoundConfig,
  PlaySoundRequest,
  PlaySoundResult,
  SoundLog,
  GiftSoundExportData,
  ConfigValidationResult,
  BatchOperationRequest,
  BatchOperationResult
} from '../modules/gift-sound/types.js';

interface ElectronAPI {
  invoke: (channel: string, ...args: unknown[]) => Promise<unknown>;
  on: (channel: string, callback: (...args: unknown[]) => void) => void;
  off: (channel: string, callback: (...args: unknown[]) => void) => void;
}

function getApi(): ElectronAPI {
  return (window as { electronAPI: ElectronAPI }).electronAPI;
}

export class GiftSoundService {
  private api = getApi();

  /**
   * 获取所有礼物音效配置
   */
  async getAllGiftSounds(): Promise<GiftSoundConfig[]> {
    try {
      const result = await this.api.invoke('gift-sound:getAll') as {
        success: boolean;
        giftSounds: GiftSoundConfig[];
        error?: string;
      };

      if (!result.success) {
        throw new Error(result.error || '获取礼物音效配置失败');
      }

      return result.giftSounds;
    } catch (error) {
      console.error('获取礼物音效配置失败:', error);
      throw error;
    }
  }

  /**
   * 根据ID获取礼物音效配置
   */
  async getGiftSoundById(id: string): Promise<GiftSoundConfig | null> {
    try {
      const result = await this.api.invoke('gift-sound:getById', id) as {
        success: boolean;
        giftSound: GiftSoundConfig | null;
        error?: string;
      };

      if (!result.success) {
        throw new Error(result.error || '获取礼物音效配置失败');
      }

      return result.giftSound;
    } catch (error) {
      console.error('获取礼物音效配置失败:', error);
      throw error;
    }
  }

  /**
   * 根据礼物ID获取音效配置
   */
  async getGiftSoundByGiftId(giftId: string): Promise<GiftSoundConfig | null> {
    try {
      const result = await this.api.invoke('gift-sound:getByGiftId', giftId) as {
        success: boolean;
        giftSound: GiftSoundConfig | null;
        error?: string;
      };

      if (!result.success) {
        throw new Error(result.error || '获取礼物音效配置失败');
      }

      return result.giftSound;
    } catch (error) {
      console.error('获取礼物音效配置失败:', error);
      throw error;
    }
  }

  /**
   * 添加礼物音效配置
   */
  async addGiftSound(giftSound: GiftSoundConfig): Promise<void> {
    try {
      // 验证配置
      const validation = this.validateGiftSoundConfig(giftSound);
      if (!validation.valid) {
        throw new Error(`配置验证失败: ${validation.errors.join(', ')}`);
      }

      const result = await this.api.invoke('gift-sound:add', giftSound) as {
        success: boolean;
        error?: string;
      };

      if (!result.success) {
        throw new Error(result.error || '添加礼物音效配置失败');
      }
    } catch (error) {
      console.error('添加礼物音效配置失败:', error);
      throw error;
    }
  }

  /**
   * 更新礼物音效配置
   */
  async updateGiftSound(id: string, updates: Partial<GiftSoundConfig>): Promise<void> {
    try {
      const result = await this.api.invoke('gift-sound:update', id, updates) as {
        success: boolean;
        error?: string;
      };

      if (!result.success) {
        throw new Error(result.error || '更新礼物音效配置失败');
      }
    } catch (error) {
      console.error('更新礼物音效配置失败:', error);
      throw error;
    }
  }

  /**
   * 删除礼物音效配置
   */
  async deleteGiftSound(id: string): Promise<void> {
    try {
      const result = await this.api.invoke('gift-sound:delete', id) as {
        success: boolean;
        error?: string;
      };

      if (!result.success) {
        throw new Error(result.error || '删除礼物音效配置失败');
      }
    } catch (error) {
      console.error('删除礼物音效配置失败:', error);
      throw error;
    }
  }

  /**
   * 播放礼物音效
   */
  async playGiftSound(request: PlaySoundRequest): Promise<PlaySoundResult> {
    try {
      const result = await this.api.invoke('gift-sound:play', request) as PlaySoundResult;
      return result;
    } catch (error) {
      console.error('播放礼物音效失败:', error);
      return {
        success: false,
        error: (error as Error).message
      };
    }
  }

  /**
   * 停止所有音效
   */
  async stopAllSounds(): Promise<void> {
    try {
      const result = await this.api.invoke('gift-sound:stopAll') as {
        success: boolean;
        error?: string;
      };

      if (!result.success) {
        throw new Error(result.error || '停止音效失败');
      }
    } catch (error) {
      console.error('停止音效失败:', error);
      throw error;
    }
  }

  /**
   * 预览音效
   */
  async previewSound(soundFile: string, volume: number): Promise<void> {
    try {
      const result = await this.api.invoke('gift-sound:preview', soundFile, volume) as {
        success: boolean;
        error?: string;
      };

      if (!result.success) {
        throw new Error(result.error || '预览音效失败');
      }
    } catch (error) {
      console.error('预览音效失败:', error);
      throw error;
    }
  }

  /**
   * 导入音效配置
   */
  async importGiftSounds(configs: GiftSoundConfig[]): Promise<void> {
    try {
      // 验证所有配置
      for (const config of configs) {
        const validation = this.validateGiftSoundConfig(config);
        if (!validation.valid) {
          throw new Error(`配置验证失败: ${validation.errors.join(', ')}`);
        }
      }

      const result = await this.api.invoke('gift-sound:import', configs) as {
        success: boolean;
        error?: string;
      };

      if (!result.success) {
        throw new Error(result.error || '导入音效配置失败');
      }
    } catch (error) {
      console.error('导入音效配置失败:', error);
      throw error;
    }
  }

  /**
   * 导出音效配置
   */
  async exportGiftSounds(): Promise<GiftSoundExportData> {
    try {
      const result = await this.api.invoke('gift-sound:export') as {
        success: boolean;
        giftSounds: GiftSoundConfig[];
        error?: string;
      };

      if (!result.success) {
        throw new Error(result.error || '导出音效配置失败');
      }

      return {
        version: '1.0',
        exportedAt: new Date(),
        giftSounds: result.giftSounds
      };
    } catch (error) {
      console.error('导出音效配置失败:', error);
      throw error;
    }
  }

  /**
   * 获取播放日志
   */
  async getSoundLogs(): Promise<SoundLog[]> {
    try {
      const result = await this.api.invoke('gift-sound:getLogs') as {
        success: boolean;
        logs: SoundLog[];
        error?: string;
      };

      if (!result.success) {
        throw new Error(result.error || '获取播放日志失败');
      }

      return result.logs;
    } catch (error) {
      console.error('获取播放日志失败:', error);
      throw error;
    }
  }

  /**
   * 清空播放日志
   */
  async clearSoundLogs(): Promise<void> {
    try {
      const result = await this.api.invoke('gift-sound:clearLogs') as {
        success: boolean;
        error?: string;
      };

      if (!result.success) {
        throw new Error(result.error || '清空播放日志失败');
      }
    } catch (error) {
      console.error('清空播放日志失败:', error);
      throw error;
    }
  }

  /**
   * 批量操作
   */
  async batchOperation(request: BatchOperationRequest): Promise<BatchOperationResult> {
    try {
      let processed = 0;
      let failed = 0;
      const errors: string[] = [];

      for (const id of request.ids) {
        try {
          switch (request.type) {
            case 'enable':
              await this.updateGiftSound(id, { enabled: true });
              break;
            case 'disable':
              await this.updateGiftSound(id, { enabled: false });
              break;
            case 'delete':
              await this.deleteGiftSound(id);
              break;
            case 'export':
              // 导出操作单独处理
              break;
          }
          processed++;
        } catch (error) {
          failed++;
          errors.push(`操作失败 ${id}: ${(error as Error).message}`);
        }
      }

      return {
        success: failed === 0,
        processed,
        failed,
        errors
      };
    } catch (error) {
      console.error('批量操作失败:', error);
      return {
        success: false,
        processed: 0,
        failed: request.ids.length,
        errors: [(error as Error).message]
      };
    }
  }

  /**
   * 验证礼物音效配置
   */
  validateGiftSoundConfig(config: GiftSoundConfig): ConfigValidationResult {
    const errors: string[] = [];
    const warnings: string[] = [];

    // 必填字段验证
    if (!config.giftId) {
      errors.push('礼物ID不能为空');
    }
    if (!config.giftName) {
      errors.push('礼物名称不能为空');
    }
    if (!config.soundFile) {
      errors.push('音效文件不能为空');
    }

    // 音量验证
    if (config.volume < 0 || config.volume > 1) {
      errors.push('音量必须在0-1之间');
    }

    // 时长验证
    if (config.duration < 0) {
      errors.push('播放时长不能为负数');
    }

    // 冷却时间验证
    if (config.cooldown < 0) {
      errors.push('冷却时间不能为负数');
    }

    // 优先级验证
    if (config.priority < 0) {
      warnings.push('优先级为负数可能导致音效很少播放');
    }

    return {
      valid: errors.length === 0,
      errors,
      warnings
    };
  }

  /**
   * 创建默认配置
   */
  createDefaultConfig(giftId: string, giftName: string): GiftSoundConfig {
    return {
      id: Date.now().toString(),
      giftId,
      giftName,
      soundFile: '',
      soundName: '',
      volume: 0.8,
      duration: 0,
      playMode: 'once',
      cooldown: 1000,
      priority: 0,
      enabled: true,
      createdAt: new Date(),
      updatedAt: new Date()
    };
  }
}

// 创建单例实例
export const giftSoundService = new GiftSoundService();
