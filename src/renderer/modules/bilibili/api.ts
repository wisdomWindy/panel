/**
 * Bilibili 开放平台 HTTP API 封装
 * 通过 IPC 调用主进程发送请求
 */

import { StartResponse, BatchHeartbeatResponse, ClientError } from './types';

/**
 * 开放平台 API 客户端
 * 封装项目生命周期管理 API
 */
export class OpenPlatformAPI {
  private accessKeyId: string;
  private accessKeySecret: string;

  constructor(accessKeyId: string, accessKeySecret: string) {
    this.accessKeyId = accessKeyId;
    this.accessKeySecret = accessKeySecret;
  }

  /**
   * 开启项目
   * POST /v2/app/start
   *
   * @param code - 主播身份码
   * @param appId - 应用 ID
   * @returns start 响应 (含 game_id, wss_link, auth_body, anchor_info)
   */
  async start(code: string, appId: number): Promise<StartResponse> {
    const result = await window.electronAPI.invoke('bilibili:start', {
      accessKeyId: this.accessKeyId,
      accessKeySecret: this.accessKeySecret,
      code,
      appId
    }) as StartResponse;

    if (result.code !== 0) {
      throw new ClientError(
        `start 失败: ${result.message} (code=${result.code})`,
        result.code
      );
    }

    return result;
  }

  /**
   * 关闭项目
   * POST /v2/app/end
   *
   * @param appId - 应用 ID
   * @param gameId - 场次 ID
   */
  async end(appId: number, gameId: string): Promise<void> {
    const result = await window.electronAPI.invoke('bilibili:end', {
      accessKeyId: this.accessKeyId,
      accessKeySecret: this.accessKeySecret,
      appId,
      gameId
    }) as { code: number; message: string };

    if (result.code !== 0) {
      throw new ClientError(
        `end 失败: ${result.message} (code=${result.code})`,
        result.code
      );
    }
  }

  /**
   * 发送项目心跳
   * POST /v2/app/heartbeat
   * 推荐每 20 秒调用一次
   *
   * @param gameId - 场次 ID
   */
  async heartbeat(gameId: string): Promise<void> {
    const result = await window.electronAPI.invoke('bilibili:heartbeat', {
      accessKeyId: this.accessKeyId,
      accessKeySecret: this.accessKeySecret,
      gameId
    }) as { code: number; message: string };

    if (result.code !== 0) {
      throw new ClientError(
        `heartbeat 失败: ${result.message} (code=${result.code})`,
        result.code
      );
    }
  }

  /**
   * 批量发送项目心跳
   * POST /v2/app/batchHeartbeat
   *
   * @param gameIds - 场次 ID 列表 (最多 200 条)
   * @returns 失败的心跳 ID 列表
   */
  async batchHeartbeat(gameIds: string[]): Promise<string[]> {
    if (gameIds.length > 200) {
      throw new ClientError('批量心跳单次最大值为 200');
    }

    const result = await window.electronAPI.invoke('bilibili:batchHeartbeat', {
      accessKeyId: this.accessKeyId,
      accessKeySecret: this.accessKeySecret,
      gameIds
    }) as BatchHeartbeatResponse;

    if (result.code !== 0) {
      throw new ClientError(
        `batchHeartbeat 失败: ${result.message} (code=${result.code})`,
        result.code
      );
    }

    return result.data.failed_game_ids;
  }
}
