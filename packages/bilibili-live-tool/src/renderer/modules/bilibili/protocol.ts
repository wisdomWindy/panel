/**
 * Bilibili 开放平台 WebSocket 协议解析器
 * 参考: https://open-live.bilibili.com/document/ (长链数据协议)
 *
 * 二进制包格式 (大端序):
 * - Packet Length (4 bytes): 整个 Packet 的长度
 * - Header Length (2 bytes): Header 长度，固定为 16
 * - Version (2 bytes): 协议版本 (0=JSON, 2=Gzip 压缩)
 * - Operation (4 bytes): 消息类型
 * - Sequence ID (4 bytes): 保留字段
 * - Body: 消息体
 */

import * as pako from 'pako';
import {
  OpCode,
  ProtocolVersion,
  ProtocolHeader,
  ParsedMessage,
  RawWsMessage,
  ProtocolError
} from './types';

/**
 * 消息分发映射
 * LIVE_OPEN_PLATFORM_* 系列 CMD → 消息类型
 */
const CMD_TYPE_MAP: Record<string, ParsedMessage['type']> = {
  'LIVE_OPEN_PLATFORM_DM': 'danmaku',
  'LIVE_OPEN_PLATFORM_DM_MIRROR': 'mirrorDanmaku',
  'LIVE_OPEN_PLATFORM_SEND_GIFT': 'gift',
  'LIVE_OPEN_PLATFORM_SUPER_CHAT': 'superChat',
  'LIVE_OPEN_PLATFORM_SUPER_CHAT_DEL': 'superChatDel',
  'LIVE_OPEN_PLATFORM_GUARD': 'guard',
  'LIVE_OPEN_PLATFORM_LIKE': 'like',
  'LIVE_OPEN_PLATFORM_LIVE_ROOM_ENTER': 'userEnter',
  'LIVE_OPEN_PLATFORM_LIVE_START': 'liveStart',
  'LIVE_OPEN_PLATFORM_LIVE_END': 'liveEnd',
  'LIVE_OPEN_PLATFORM_INTERACTION_END': 'interactionEnd'
};

/**
 * 协议解析器
 */
export class ProtocolParser {
  private static HEADER_LENGTH = 16;

  /**
   * 解析消息头
   * @param buffer - 数据缓冲区
   * @returns 协议头
   */
  static parseHeader(buffer: ArrayBuffer): ProtocolHeader {
    if (buffer.byteLength < this.HEADER_LENGTH) {
      throw new ProtocolError('Buffer too small for header');
    }

    const view = new DataView(buffer);

    // 大端序读取
    const length = view.getUint32(0, false);
    const headerLength = view.getUint16(4, false);
    const version = view.getUint16(6, false) as ProtocolVersion;
    const opCode = view.getUint32(8, false) as OpCode;
    const sequenceId = view.getUint32(12, false);

    return {
      length,
      headerLength,
      version,
      opCode,
      sequenceId
    };
  }

  /**
   * 解析消息体
   * @param header - 协议头
   * @param body - 消息体缓冲区
   * @returns 解析后的消息列表 (压缩包可能包含多条消息)
   */
  static async parseBody(
    header: ProtocolHeader,
    body: ArrayBuffer
  ): Promise<ParsedMessage[]> {
    try {
      let data = body;

      // 处理压缩
      if (header.version === ProtocolVersion.GZIP) {
        data = this.decompressGzip(body);
      } else if (header.version === ProtocolVersion.BROTLI) {
        throw new ProtocolError('Brotli compression not supported');
      }

      // 解压后的数据可能包含多个完整的 proto 包 (用 \0 分隔)
      const text = new TextDecoder().decode(data);
      const messages = text.split('\0').filter(m => m.trim());

      const results: ParsedMessage[] = [];
      for (const msg of messages) {
        const parsed = this.parseMessage(msg, header.opCode);
        if (parsed) {
          results.push(parsed);
        }
      }

      return results;
    } catch (error) {
      if (error instanceof ProtocolError) throw error;
      throw new ProtocolError(`Failed to parse body: ${error}`);
    }
  }

  /**
   * 解析单条消息
   * @param text - 消息 JSON 文本
   * @param opCode - 操作码
   * @returns 解析后的消息
   */
  private static parseMessage(text: string, opCode: OpCode): ParsedMessage | null {
    try {
      const json = JSON.parse(text) as RawWsMessage;

      switch (opCode) {
        case OpCode.SEND_SMS_REPLY:
          // 消息推送
          return this.dispatchByCmd(json);

        case OpCode.HEARTBEAT_REPLY:
          // 心跳回复
          return {
            type: 'unknown',
            data: json,
            raw: json
          };

        case OpCode.AUTH_REPLY:
          // 鉴权回复
          return {
            type: 'unknown',
            data: json,
            raw: json
          };

        default:
          return {
            type: 'unknown',
            data: json,
            raw: json
          };
      }
    } catch {
      return null;
    }
  }

  /**
   * 根据 CMD 字段分发消息
   * @param json - 原始消息 JSON
   * @returns 解析后的消息
   */
  private static dispatchByCmd(json: RawWsMessage): ParsedMessage | null {
    const cmd = json.cmd;

    if (!cmd) {
      return { type: 'unknown', data: json, raw: json };
    }

    const type = CMD_TYPE_MAP[cmd];
    if (type) {
      return { type, data: json, raw: json };
    }

    return { type: 'unknown', data: json, raw: json };
  }

  /**
   * Gzip 解压
   * @param buffer - 压缩数据
   * @returns 解压后的数据
   */
  private static decompressGzip(buffer: ArrayBuffer): ArrayBuffer {
    try {
      const compressed = new Uint8Array(buffer);
      const decompressed = pako.inflate(compressed);
      return decompressed.buffer;
    } catch (error) {
      throw new ProtocolError(`Gzip decompression failed: ${error}`);
    }
  }

  // ============ 编码方法 ============

  /**
   * 编码心跳包
   * @returns 编码后的 ArrayBuffer
   */
  static encodeHeartbeat(): ArrayBuffer {
    const body = new ArrayBuffer(0);
    return this.encodePacket(OpCode.HEARTBEAT, ProtocolVersion.HEARTBEAT, body);
  }

  /**
   * 编码认证包
   * @param authBody - 认证 JSON 字符串 (来自 /v2/app/start 的 auth_body)
   * @returns 编码后的 ArrayBuffer
   */
  static encodeAuth(authBody: string): ArrayBuffer {
    const bodyBuffer = new TextEncoder().encode(authBody);
    return this.encodePacket(OpCode.AUTH, ProtocolVersion.NORMAL, bodyBuffer.buffer);
  }

  /**
   * 编码数据包
   * @param opCode - 操作码
   * @param version - 协议版本
   * @param body - 消息体
   * @returns 编码后的 ArrayBuffer
   */
  private static encodePacket(
    opCode: OpCode,
    version: ProtocolVersion,
    body: ArrayBuffer
  ): ArrayBuffer {
    const headerLength = this.HEADER_LENGTH;
    const bodyLength = body.byteLength;
    const totalLength = headerLength + bodyLength;

    const buffer = new ArrayBuffer(totalLength);
    const view = new DataView(buffer);

    // 大端序写入消息头
    view.setUint32(0, totalLength, false);   // 消息总长度
    view.setUint16(4, headerLength, false);  // 消息头长度
    view.setUint16(6, version, false);       // 协议版本
    view.setUint32(8, opCode, false);        // 操作码
    view.setUint32(12, 1, false);            // 序列号

    // 写入消息体
    if (bodyLength > 0) {
      const bodyView = new Uint8Array(body);
      const packetView = new Uint8Array(buffer);
      packetView.set(bodyView, headerLength);
    }

    return buffer;
  }
}
