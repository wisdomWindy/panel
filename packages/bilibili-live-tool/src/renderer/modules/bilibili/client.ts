/**
 * Bilibili 开放平台 WebSocket 客户端
 * 实现完整的 start → heartbeat → end 生命周期
 *
 * 连接流程:
 * 1. POST /v2/app/start → 获取 game_id, wss_link[], auth_body, anchor_info
 * 2. 连接 WebSocket (wss_link[0], 失败切换到备用地址)
 * 3. 发送 OP_AUTH 包 (body = auth_body JSON)
 * 4. 等待 OP_AUTH_REPLY
 * 5. 开始双心跳:
 *    - WebSocket 心跳: 每 20s 发送空 body OP_HEARTBEAT
 *    - HTTP 心跳: 每 20s POST /v2/app/heartbeat
 * 6. 接收消息 (OP_SEND_SMS_REPLY)
 * 7. 断开时: POST /v2/app/end → 关闭 WebSocket
 */

import { EventEmitter } from 'events';
import {
  ClientConfig,
  ClientEvent,
  RoomInfo,
  ParsedMessage,
  ClientError
} from './types';
import { ProtocolParser } from './protocol';
import { OpenPlatformAPI } from './api';

/**
 * Bilibili 开放平台客户端
 */
export class BilibiliClient extends EventEmitter {
  private ws: WebSocket | null = null;
  private config: ClientConfig;
  private api: OpenPlatformAPI;

  // 会话状态
  private gameId: string = '';
  private isConnected: boolean = false;
  private isAuthenticated: boolean = false;
  private wssLinks: string[] = [];
  private currentWssIndex: number = 0;

  // 心跳相关
  private wsHeartbeatTimer: NodeJS.Timeout | null = null;
  private httpHeartbeatTimer: NodeJS.Timeout | null = null;
  private heartbeatInterval: number = 20; // 默认 20 秒

  // 重连相关
  private reconnectTimer: NodeJS.Timeout | null = null;
  private reconnectAttempts: number = 0;
  private maxReconnectAttempts: number = 10;
  private reconnectInterval: number = 3000;

  // 消息缓冲
  private messageBuffer: ArrayBuffer = new ArrayBuffer(0);

  constructor(config: ClientConfig) {
    super();
    this.config = config;
    this.api = new OpenPlatformAPI(config.accessKeyId, config.accessKeySecret);
    this.heartbeatInterval = config.heartbeatInterval || 20;
    this.maxReconnectAttempts = config.maxReconnectAttempts || 10;
    this.reconnectInterval = config.reconnectInterval || 3000;
  }

  /**
   * 连接直播间
   */
  async connect(): Promise<RoomInfo> {
    if (this.isConnected) {
      throw new ClientError('Already connected');
    }

    this.reconnectAttempts = 0;

    try {
      // 1. 调用 /v2/app/start
      const startRes = await this.api.start(this.config.code, this.config.appId);

      this.gameId = startRes.data.game_info.game_id;
      this.wssLinks = startRes.data.websocket_info.wss_link;
      this.currentWssIndex = 0;

      const authBody = startRes.data.websocket_info.auth_body;
      const anchorInfo = startRes.data.anchor_info;

      // 构建直播间信息
      const roomInfo: RoomInfo = {
        roomId: anchorInfo.room_id,
        roomName: anchorInfo.uname || '',
        ownerId: anchorInfo.uid,
        ownerOpenId: anchorInfo.open_id,
        ownerName: anchorInfo.uname || '',
        onlineCount: 0,
        liveStatus: 1,
        openId: anchorInfo.open_id,
        unionId: anchorInfo.union_id
      };

      // 2. 连接 WebSocket
      await this.connectWebSocket();

      // 3. 发送 AUTH 包
      await this.authenticate(authBody);

      // 4. 开始双心跳
      this.startHeartbeat();

      this.isConnected = true;
      this.emit(ClientEvent.CONNECTED, roomInfo);

      return roomInfo;
    } catch (error) {
      this.isConnected = false;
      this.cleanup();
      this.emit(ClientEvent.ERROR, error);
      throw error;
    }
  }

  /**
   * 断开连接
   */
  async disconnect(): Promise<void> {
    if (!this.isConnected) {
      return;
    }

    this.isConnected = false;
    this.isAuthenticated = false;

    // 调用 /v2/app/end
    if (this.gameId) {
      try {
        await this.api.end(this.config.appId, this.gameId);
      } catch (error) {
        console.warn('end 调用失败:', error);
      }
    }

    this.cleanup();
    this.emit(ClientEvent.DISCONNECTED);
  }

  /**
   * 连接 WebSocket (支持多集群切换)
   */
  private connectWebSocket(): Promise<void> {
    return new Promise<void>((resolve, reject) => {
      let resolved = false;

      const tryConnect = (index: number) => {
        if (resolved) return;
        if (index >= this.wssLinks.length) {
          resolved = true;
          reject(new ClientError('所有 WebSocket 地址连接失败'));
          return;
        }

        const url = this.wssLinks[index];
        const ws = new WebSocket(url);
        this.ws = ws;

        const timeout = setTimeout(() => {
          ws.close();
          if (!resolved) {
            tryConnect(index + 1);
          }
        }, 10000);

        ws.addEventListener('open', () => {
          if (resolved) return;
          resolved = true;
          clearTimeout(timeout);
          this.currentWssIndex = index;
          this.setupWebSocketHandlers();
          resolve();
        });

        ws.addEventListener('error', () => {
          clearTimeout(timeout);
          if (!resolved) {
            tryConnect(index + 1);
          }
        });
      };

      tryConnect(this.currentWssIndex);
    });
  }

  /**
   * 发送认证
   * @param authBody - 来自 /v2/app/start 的 auth_body JSON
   */
  private async authenticate(authBody: string): Promise<void> {
    const authPacket = ProtocolParser.encodeAuth(authBody);
    this.send(authPacket);

    // 等待 AUTH_REPLY (简化处理)
    this.isAuthenticated = true;
  }

  /**
   * 设置 WebSocket 事件处理器
   */
  private setupWebSocketHandlers(): void {
    if (!this.ws) return;

    this.ws.addEventListener('message', async event => {
      await this.handleMessage(event.data);
    });

    this.ws.addEventListener('close', () => {
      this.handleDisconnect();
    });

    this.ws.addEventListener('error', error => {
      this.emit(ClientEvent.ERROR, error);
    });
  }

  /**
   * 处理收到的消息
   * @param data - ArrayBuffer 消息数据
   */
  private async handleMessage(data: ArrayBuffer): Promise<void> {
    try {
      // 添加到消息缓冲
      const newBuffer = new ArrayBuffer(this.messageBuffer.byteLength + data.byteLength);
      const newView = new Uint8Array(newBuffer);
      newView.set(new Uint8Array(this.messageBuffer), 0);
      newView.set(new Uint8Array(data), this.messageBuffer.byteLength);
      this.messageBuffer = newBuffer;

      // 消息缓冲大小保护：超过 1MB 重置
      if (this.messageBuffer.byteLength > 1024 * 1024) {
        console.warn('Message buffer exceeded 1MB, resetting');
        this.messageBuffer = new ArrayBuffer(0);
      }

      // 解析消息 (循环处理缓冲中的多个包)
      while (this.messageBuffer.byteLength >= 16) {
        const header = ProtocolParser.parseHeader(this.messageBuffer);

        if (this.messageBuffer.byteLength < header.length) {
          // 消息不完整，等待更多数据
          break;
        }

        const bodyStart = header.headerLength;
        const bodyEnd = header.length;
        const body = this.messageBuffer.slice(bodyStart, bodyEnd);

        // 处理 AUTH_REPLY
        if (header.opCode === 8) { // OP_AUTH_REPLY
          this.isAuthenticated = true;
        }

        // 解析消息体 (可能返回多条消息)
        const messages = await ProtocolParser.parseBody(header, body);

        for (const message of messages) {
          if (message) {
            this.dispatchMessage(message);
          }
        }

        // 移除已处理的消息
        const remainingLength = this.messageBuffer.byteLength - header.length;
        if (remainingLength > 0) {
          this.messageBuffer = this.messageBuffer.slice(header.length);
        } else {
          this.messageBuffer = new ArrayBuffer(0);
        }
      }
    } catch (error) {
      console.error('Failed to handle message:', error);
    }
  }

  /**
   * 分发消息
   * @param message - 解析后的消息
   */
  private dispatchMessage(message: ParsedMessage): void {
    this.emit(ClientEvent.MESSAGE, message);

    switch (message.type) {
      case 'danmaku':
        this.emit(ClientEvent.DANMAKU, message.data);
        break;
      case 'gift':
        this.emit(ClientEvent.GIFT, message.data);
        break;
      case 'guard':
        this.emit(ClientEvent.GUARD, message.data);
        break;
      case 'like':
        this.emit(ClientEvent.LIKE, message.data);
        break;
      case 'userEnter':
        this.emit(ClientEvent.USER_ENTER, message.data);
        break;
      case 'liveStart':
        this.emit(ClientEvent.LIVE_START, message.data);
        break;
      case 'liveEnd':
        this.emit(ClientEvent.LIVE_END, message.data);
        break;
      case 'superChat':
        this.emit(ClientEvent.SUPER_CHAT, message.data);
        break;
      case 'interactionEnd':
        // 消息推送结束 — 停止心跳，准备重连
        this.stopHeartbeat();
        this.emit(ClientEvent.INTERACTION_END, message.data);
        break;
      // mirrorDanmaku, superChatDel 暂不触发独立事件
    }
  }

  // ============ 心跳 ============

  /**
   * 开始双心跳 (WebSocket + HTTP)
   */
  private startHeartbeat(): void {
    this.stopHeartbeat();

    // WebSocket 心跳
    this.wsHeartbeatTimer = setInterval(() => {
      this.sendWsHeartbeat();
    }, this.heartbeatInterval * 1000);

    // HTTP 项目心跳
    this.httpHeartbeatTimer = setInterval(async () => {
      await this.sendHttpHeartbeat();
    }, this.heartbeatInterval * 1000);
  }

  /**
   * 停止双心跳
   */
  private stopHeartbeat(): void {
    if (this.wsHeartbeatTimer) {
      clearInterval(this.wsHeartbeatTimer);
      this.wsHeartbeatTimer = null;
    }
    if (this.httpHeartbeatTimer) {
      clearInterval(this.httpHeartbeatTimer);
      this.httpHeartbeatTimer = null;
    }
  }

  /**
   * 发送 WebSocket 心跳
   */
  private sendWsHeartbeat(): void {
    if (!this.isConnected || !this.isAuthenticated) return;

    try {
      const heartbeatPacket = ProtocolParser.encodeHeartbeat();
      this.send(heartbeatPacket);
      this.emit(ClientEvent.HEARTBEAT);
    } catch (error) {
      console.error('WS heartbeat failed:', error);
    }
  }

  /**
   * 发送 HTTP 项目心跳
   */
  private async sendHttpHeartbeat(): Promise<void> {
    if (!this.gameId) return;

    try {
      await this.api.heartbeat(this.gameId);
    } catch (error) {
      console.error('HTTP heartbeat failed:', error);
    }
  }

  // ============ 重连 ============

  /**
   * 处理断开连接
   */
  private handleDisconnect(): void {
    this.isConnected = false;
    this.isAuthenticated = false;
    this.stopHeartbeat();

    // 尝试重连
    if (this.reconnectAttempts < this.maxReconnectAttempts) {
      this.reconnectAttempts++;
      const delay = this.reconnectInterval * Math.pow(2, this.reconnectAttempts - 1);

      this.reconnectTimer = setTimeout(() => {
        console.warn(`Reconnecting... (attempt ${this.reconnectAttempts})`);
        this.connect().catch(error => {
          console.error('Reconnect failed:', error);
        });
      }, delay);
    } else {
      this.emit(ClientEvent.ERROR, new ClientError('Max reconnect attempts reached'));
    }
  }

  // ============ 基础方法 ============

  /**
   * 发送数据
   * @param data - ArrayBuffer 数据
   */
  private send(data: ArrayBuffer): void {
    if (this.ws && this.ws.readyState === WebSocket.OPEN) {
      this.ws.send(new Uint8Array(data));
    }
  }

  /**
   * 清理资源
   */
  private cleanup(): void {
    this.stopHeartbeat();

    if (this.reconnectTimer) {
      clearTimeout(this.reconnectTimer);
      this.reconnectTimer = null;
    }

    if (this.ws) {
      this.ws.close();
      this.ws = null;
    }

    this.messageBuffer = new ArrayBuffer(0);
  }

  /**
   * 获取连接状态
   */
  get connected(): boolean {
    return this.isConnected;
  }

  /**
   * 获取认证状态
   */
  get authenticated(): boolean {
    return this.isAuthenticated;
  }

  /**
   * 获取当前场次 ID
   */
  get currentGameId(): string {
    return this.gameId;
  }
}
