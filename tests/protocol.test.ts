/**
 * Test: ProtocolParser
 *
 * 用法: npx tsx tests/protocol.test.ts
 */

import { ProtocolParser } from '../src/renderer/modules/bilibili/protocol';
import { OpCode, ProtocolVersion, ProtocolHeader, AuthParams, ProtocolError } from '../src/renderer/modules/bilibili/types';

let p = 0, f = 0;
function ok(c: boolean, l: string) { if (c) p++; else f++; console.log('  ' + (c ? '✓' : '✗ FAIL') + ': ' + l); }
function eq(a: any, e: any, l: string) { const m = JSON.stringify(a) === JSON.stringify(e); if (m) p++; else f++; console.log('  ' + (m ? '✓' : '✗ FAIL') + ': ' + l + (m ? '' : '\n    exp: ' + JSON.stringify(e) + '\n    got: ' + JSON.stringify(a))); }

function makeBody(json: string): ArrayBuffer {
  return new TextEncoder().encode(json).buffer;
}

function makeHeader(overrides: Partial<ProtocolHeader> = {}): ProtocolHeader {
  return {
    length: 16,
    headerLength: 16,
    version: ProtocolVersion.NORMAL,
    opCode: OpCode.MESSAGE,
    sequenceId: 1,
    ...overrides
  };
}

async function run() {
  console.log('=== ProtocolParser 测试 ===\n');

  // --- encodeHeartbeat ---
  console.log('encodeHeartbeat:');
  const hb = ProtocolParser.encodeHeartbeat();
  ok(hb instanceof ArrayBuffer, '返回 ArrayBuffer');
  ok(hb.byteLength === 16, '心跳包总长度应为 16');
  {
    const dv = new DataView(hb);
    eq(dv.getUint32(0, true), 16, 'length = 16');
    eq(dv.getUint16(4, true), 16, 'headerLength = 16');
    eq(dv.getUint16(6, true), ProtocolVersion.HEARTBEAT, 'version = HEARTBEAT (1)');
    eq(dv.getUint32(8, true), OpCode.HEARTBEAT, 'opCode = HEARTBEAT (2)');
    eq(dv.getUint32(12, true), 1, 'sequenceId = 1');
  }

  // --- encodeAuth ---
  console.log('\nencodeAuth:');
  const authParams: AuthParams = {
    uid: 123,
    roomId: 456,
    protover: 3,
    platform: 'web',
    type: 2,
    key: 'test-key'
  };
  const authBuf = ProtocolParser.encodeAuth(authParams);
  ok(authBuf instanceof ArrayBuffer, '返回 ArrayBuffer');
  {
    const dv = new DataView(authBuf);
    const totalLen = dv.getUint32(0, true);
    eq(dv.getUint16(4, true), 16, 'headerLength = 16');
    eq(dv.getUint16(6, true), ProtocolVersion.NORMAL, 'version = NORMAL (0)');
    eq(dv.getUint32(8, true), OpCode.AUTH, 'opCode = AUTH (7)');
    eq(dv.getUint32(12, true), 1, 'sequenceId = 1');
    ok(totalLen > 16, '总长度 > 16（包含 body）');
    // 验证 body 部分
    const expectedBody = JSON.stringify(authParams);
    const bodyLen = totalLen - 16;
    const bodyBytes = new Uint8Array(authBuf, 16, bodyLen);
    const bodyStr = new TextDecoder().decode(bodyBytes);
    eq(bodyStr, expectedBody, 'body 应为 JSON.stringify(params)');
    eq(bodyLen, expectedBody.length, 'body 长度正确');
  }

  // --- parseHeader: valid ---
  console.log('\nparseHeader (valid):');
  {
    const buf = new ArrayBuffer(16);
    const dv = new DataView(buf);
    dv.setUint32(0, 100, true);    // length
    dv.setUint16(4, 16, true);     // headerLength
    dv.setUint16(6, 2, true);      // version = GZIP
    dv.setUint32(8, 5, true);      // opCode = MESSAGE
    dv.setUint32(12, 42, true);    // sequenceId
    const header = ProtocolParser.parseHeader(buf);
    eq(header.length, 100, 'length = 100');
    eq(header.headerLength, 16, 'headerLength = 16');
    eq(header.version, ProtocolVersion.GZIP, 'version = GZIP (2)');
    eq(header.opCode, OpCode.MESSAGE, 'opCode = MESSAGE (5)');
    eq(header.sequenceId, 42, 'sequenceId = 42');
  }

  // --- parseHeader: small buffer ---
  console.log('\nparseHeader (small buffer):');
  {
    const buf = new ArrayBuffer(10);
    try {
      ProtocolParser.parseHeader(buf);
      ok(false, '应抛出 ProtocolError');
    } catch (e) {
      ok(e instanceof ProtocolError, '抛出 ProtocolError');
      ok((e as Error).message.includes('too small'), '错误消息包含 "too small"');
    }
  }

  // --- parseBody: DANMU_MSG ---
  console.log('\nparseBody (DANMU_MSG):');
  {
    const danmakuJson = JSON.stringify({
      cmd: 'DANMU_MSG',
      info: { uid: 1, uname: 'test_user', text: 'hello danmaku', time: 1234567890, type: 0, size: 25, color: 16777215, timestamp: 1234567890000, rnd: 0, uid_hash: 'abc', uname_color: '#ffffff', level: 1, vip: 0, svip: 0, user_rank: 0, barrage_type: 0 }
    });
    const header = makeHeader({ opCode: OpCode.MESSAGE });
    const body = makeBody(danmakuJson);
    const result = await ProtocolParser.parseBody(header, body);
    ok(result !== null, '应返回非 null');
    eq(result!.type, 'danmaku', 'type = danmaku');
    ok(result!.data !== undefined, 'data 存在');
    eq((result!.data as any).cmd, 'DANMU_MSG', 'data.cmd = DANMU_MSG');
    eq((result!.raw as any).cmd, 'DANMU_MSG', 'raw.cmd = DANMU_MSG');
  }

  // --- parseBody: SEND_GIFT ---
  console.log('\nparseBody (SEND_GIFT):');
  {
    const giftJson = JSON.stringify({
      cmd: 'SEND_GIFT',
      data: { uid: 1, uname: 'gifter', giftId: 1, giftName: '花', num: 1, coinType: 'gold', totalCoin: 100, action: 'send', timestamp: 1234567890, face: 'face.jpg', giftType: 1, price: 100, batch_type: '', batch_session_id: '' }
    });
    const header = makeHeader({ opCode: OpCode.MESSAGE });
    const body = makeBody(giftJson);
    const result = await ProtocolParser.parseBody(header, body);
    ok(result !== null, '应返回非 null');
    eq(result!.type, 'gift', 'type = gift');
    eq((result!.data as any).cmd, 'SEND_GIFT', 'data.cmd = SEND_GIFT');
  }

  // --- parseBody: ONLINE_COUNT ---
  console.log('\nparseBody (ONLINE_COUNT):');
  {
    const ocJson = JSON.stringify({ cmd: 'ONLINE_COUNT', count: 1234 });
    const header = makeHeader({ opCode: OpCode.MESSAGE });
    const body = makeBody(ocJson);
    const result = await ProtocolParser.parseBody(header, body);
    ok(result !== null, '应返回非 null');
    eq(result!.type, 'onlineCount', 'type = onlineCount');
    eq((result!.data as any).count, 1234, 'data.count = 1234');
  }

  // --- parseBody: INTERACT_WORD (userEnter) ---
  console.log('\nparseBody (INTERACT_WORD):');
  {
    const iwJson = JSON.stringify({
      cmd: 'INTERACT_WORD',
      data: { uid: 100, uname: 'new_user', timestamp: 1234567890, msg_type: 1 }
    });
    const header = makeHeader({ opCode: OpCode.MESSAGE });
    const body = makeBody(iwJson);
    const result = await ProtocolParser.parseBody(header, body);
    ok(result !== null, '应返回非 null');
    eq(result!.type, 'userEnter', 'type = userEnter');
    eq((result!.data as any).cmd, 'INTERACT_WORD', 'data.cmd = INTERACT_WORD');
  }

  // --- parseBody: HEARTBEAT_REPLY ---
  console.log('\nparseBody (HEARTBEAT_REPLY):');
  {
    const body = makeBody('42');
    const header = makeHeader({ opCode: OpCode.HEARTBEAT_REPLY });
    const result = await ProtocolParser.parseBody(header, body);
    ok(result !== null, '应返回非 null');
    eq(result!.type, 'onlineCount', 'type = onlineCount');
    eq((result!.data as any).cmd, 'ONLINE_COUNT', 'data.cmd = ONLINE_COUNT');
    eq((result!.data as any).count, 42, 'data.count = 42');
  }

  // --- parseBody: unknown cmd (仍为 MESSAGE opCode) ---
  console.log('\nparseBody (unknown cmd with MESSAGE opCode):');
  {
    const unknownJson = JSON.stringify({ cmd: 'SOME_UNKNOWN_CMD', foo: 'bar' });
    const header = makeHeader({ opCode: OpCode.MESSAGE });
    const body = makeBody(unknownJson);
    const result = await ProtocolParser.parseBody(header, body);
    ok(result === null, '未知 cmd 应返回 null');
  }

  // --- parseBody: unknown opCode (非 MESSAGE/HEARTBEAT_REPLY/AUTH_REPLY) ---
  console.log('\nparseBody (unknown opCode, e.g. SEND_DANMAKU):');
  {
    const json = JSON.stringify({ cmd: 'anything', val: 1 });
    const header = makeHeader({ opCode: OpCode.SEND_DANMAKU });
    const body = makeBody(json);
    const result = await ProtocolParser.parseBody(header, body);
    ok(result !== null, '应返回非 null（default 分支）');
    eq(result!.type, 'unknown', 'type = unknown');
    eq((result!.data as any).cmd, 'anything', 'data 保持原样');
  }

  // --- parseBody: invalid JSON ---
  console.log('\nparseBody (invalid JSON):');
  {
    const body = makeBody('not valid json{{{');
    const header = makeHeader({ opCode: OpCode.MESSAGE });
    const result = await ProtocolParser.parseBody(header, body);
    ok(result === null, '无效 JSON 应返回 null');
  }

  // --- parseBody: AUTH_REPLY ---
  console.log('\nparseBody (AUTH_REPLY):');
  {
    const authReplyJson = JSON.stringify({ code: 0, message: 'ok' });
    const header = makeHeader({ opCode: OpCode.AUTH_REPLY });
    const body = makeBody(authReplyJson);
    const result = await ProtocolParser.parseBody(header, body);
    ok(result !== null, '应返回非 null');
    eq(result!.type, 'unknown', 'type = unknown');
    eq((result!.data as any).code, 0, 'data.code = 0');
  }

  // --- Summary ---
  console.log(`\n---`);
  console.log(`通过: ${p}, 失败: ${f}, 总计: ${p + f}`);
  if (f > 0) {
    process.exit(1);
  }
}

run().catch(err => {
  console.error('测试运行异常:', err);
  process.exit(1);
});
