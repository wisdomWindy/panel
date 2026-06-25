/**
 * Bilibili 开放平台 HMAC-SHA256 签名模块
 * 参考: https://open-live.bilibili.com/document/ (鉴权章节)
 */

import { AuthHeaders } from './types';

/**
 * 生成开放平台 API 请求所需的鉴权请求头
 *
 * @param accessKeyId - 开发者 access_key_id
 * @param accessKeySecret - 开发者 access_key_secret
 * @param body - 请求体 JSON 字符串
 * @returns 包含所有必需 header 的鉴权头对象
 */
export function generateAuthHeaders(
  accessKeyId: string,
  accessKeySecret: string,
  body: string
): AuthHeaders {
  // 1. 计算请求体的 MD5
  const contentMd5 = md5(body);

  // 2. 生成 UUID 作为 nonce
  const nonce = generateUUID();

  // 3. 生成 Unix 时间戳（秒）
  const timestamp = Math.floor(Date.now() / 1000).toString();

  // 4. 构建待签名字符串 (x-bili- 前缀 header 按字典排序)
  const signStr =
    `x-bili-accesskeyid:${accessKeyId}\n` +
    `x-bili-content-md5:${contentMd5}\n` +
    'x-bili-signature-method:HMAC-SHA256\n' +
    `x-bili-signature-nonce:${nonce}\n` +
    'x-bili-signature-version:1.0\n' +
    `x-bili-timestamp:${timestamp}`;

  // 5. HMAC-SHA256 签名 (小写 hex)
  const signature = hmacSha256(accessKeySecret, signStr);

  return {
    'Accept': 'application/json',
    'Content-Type': 'application/json',
    'x-bili-content-md5': contentMd5,
    'x-bili-timestamp': timestamp,
    'x-bili-signature-method': 'HMAC-SHA256',
    'x-bili-signature-nonce': nonce,
    'x-bili-accesskeyid': accessKeyId,
    'x-bili-signature-version': '1.0',
    'Authorization': signature
  };
}

/**
 * 计算字符串的 MD5 值 (十六进制，小写)
 * 使用 Web Crypto API
 */
function md5(str: string): string {
  // 使用 SparkMD5 或 crypto-js 等库
  // 这里提供一个基于内置 TextEncoder + 简单实现的备选方案
  // 在生产环境中建议使用 crypto-js 或 spark-md5
  return md5Simple(str);
}

/**
 * 简易 MD5 实现 (RFC 1321)
 * 使用纯 JavaScript 实现，避免依赖问题
 */
function md5Simple(str: string): string {
  function rotateLeft(x: number, n: number): number {
    return (x << n) | (x >>> (32 - n));
  }

  function addUnsigned(x: number, y: number): number {
    const lsw = (x & 0xFFFF) + (y & 0xFFFF);
    const msw = (x >> 16) + (y >> 16) + (lsw >> 16);
    return (msw << 16) | (lsw & 0xFFFF);
  }

  function md5F(x: number, y: number, z: number): number { return (x & y) | (~x & z); }
  function md5G(x: number, y: number, z: number): number { return (x & z) | (y & ~z); }
  function md5H(x: number, y: number, z: number): number { return x ^ y ^ z; }
  function md5I(x: number, y: number, z: number): number { return y ^ (x | ~z); }

  function md5Transform(func: (a: number, b: number, c: number) => number, a: number, b: number, c: number, d: number, x: number, s: number, ac: number): number {
    let result = addUnsigned(a, addUnsigned(addUnsigned(func(b, c, d), x), ac));
    result = rotateLeft(result, s);
    return addUnsigned(b, result);
  }

  const bytes: number[] = [];
  for (let i = 0; i < str.length; i++) {
    const code = str.charCodeAt(i);
    if (code < 0x80) {
      bytes.push(code);
    } else if (code < 0x800) {
      bytes.push(0xC0 | (code >> 6), 0x80 | (code & 0x3F));
    } else if (code < 0xD800 || code >= 0xE000) {
      bytes.push(0xE0 | (code >> 12), 0x80 | ((code >> 6) & 0x3F), 0x80 | (code & 0x3F));
    } else {
      i++;
      const surrogate = 0x10000 + (((code & 0x3FF) << 10) | (str.charCodeAt(i) & 0x3FF));
      bytes.push(
        0xF0 | (surrogate >> 18),
        0x80 | ((surrogate >> 12) & 0x3F),
        0x80 | ((surrogate >> 6) & 0x3F),
        0x80 | (surrogate & 0x3F)
      );
    }
  }

  const msgLen = bytes.length;
  bytes.push(0x80);
  while (bytes.length % 64 !== 56) {
    bytes.push(0);
  }

  const bitLen = msgLen * 8;
  for (let i = 0; i < 4; i++) {
    bytes.push((bitLen >>> (i * 8)) & 0xFF);
  }
  for (let i = 0; i < 4; i++) {
    bytes.push(0);
  }

  let a = 0x67452301, b = 0xEFCDAB89, c = 0x98BADCFE, d = 0x10325476;

  for (let block = 0; block < bytes.length; block += 64) {
    const x: number[] = new Array(16);
    for (let i = 0; i < 16; i++) {
      x[i] = bytes[block + i * 4] | (bytes[block + i * 4 + 1] << 8) | (bytes[block + i * 4 + 2] << 16) | (bytes[block + i * 4 + 3] << 24);
    }

    const aa = a, bb = b, cc = c, dd = d;

    // Round 1
    a = md5Transform(md5F, a, b, c, d, x[0], 7, 0xD76AA478);
    d = md5Transform(md5F, d, a, b, c, x[1], 12, 0xE8C7B756);
    c = md5Transform(md5F, c, d, a, b, x[2], 17, 0x242070DB);
    b = md5Transform(md5F, b, c, d, a, x[3], 22, 0xC1BDCEEE);
    a = md5Transform(md5F, a, b, c, d, x[4], 7, 0xF57C0FAF);
    d = md5Transform(md5F, d, a, b, c, x[5], 12, 0x4787C62A);
    c = md5Transform(md5F, c, d, a, b, x[6], 17, 0xA8304613);
    b = md5Transform(md5F, b, c, d, a, x[7], 22, 0xFD469501);
    a = md5Transform(md5F, a, b, c, d, x[8], 7, 0x698098D8);
    d = md5Transform(md5F, d, a, b, c, x[9], 12, 0x8B44F7AF);
    c = md5Transform(md5F, c, d, a, b, x[10], 17, 0xFFFF5BB1);
    b = md5Transform(md5F, b, c, d, a, x[11], 22, 0x895CD7BE);
    a = md5Transform(md5F, a, b, c, d, x[12], 7, 0x6B901122);
    d = md5Transform(md5F, d, a, b, c, x[13], 12, 0xFD987193);
    c = md5Transform(md5F, c, d, a, b, x[14], 17, 0xA679438E);
    b = md5Transform(md5F, b, c, d, a, x[15], 22, 0x49B40821);

    // Round 2
    a = md5Transform(md5G, a, b, c, d, x[1], 5, 0xF61E2562);
    d = md5Transform(md5G, d, a, b, c, x[6], 9, 0xC040B340);
    c = md5Transform(md5G, c, d, a, b, x[11], 14, 0x265E5A51);
    b = md5Transform(md5G, b, c, d, a, x[0], 20, 0xE9B6C7AA);
    a = md5Transform(md5G, a, b, c, d, x[5], 5, 0xD62F105D);
    d = md5Transform(md5G, d, a, b, c, x[10], 9, 0x02441453);
    c = md5Transform(md5G, c, d, a, b, x[15], 14, 0xD8A1E681);
    b = md5Transform(md5G, b, c, d, a, x[4], 20, 0xE7D3FBC8);
    a = md5Transform(md5G, a, b, c, d, x[9], 5, 0x21E1CDE6);
    d = md5Transform(md5G, d, a, b, c, x[14], 9, 0xC33707D6);
    c = md5Transform(md5G, c, d, a, b, x[3], 14, 0xF4D50D87);
    b = md5Transform(md5G, b, c, d, a, x[8], 20, 0x455A14ED);
    a = md5Transform(md5G, a, b, c, d, x[13], 5, 0xA9E3E905);
    d = md5Transform(md5G, d, a, b, c, x[2], 9, 0xFCEFA3F8);
    c = md5Transform(md5G, c, d, a, b, x[7], 14, 0x676F02D9);
    b = md5Transform(md5G, b, c, d, a, x[12], 20, 0x8D2A4C8A);

    // Round 3
    a = md5Transform(md5H, a, b, c, d, x[5], 4, 0xFFFA3942);
    d = md5Transform(md5H, d, a, b, c, x[8], 11, 0x8771F681);
    c = md5Transform(md5H, c, d, a, b, x[11], 16, 0x6D9D6122);
    b = md5Transform(md5H, b, c, d, a, x[14], 23, 0xFDE5380C);
    a = md5Transform(md5H, a, b, c, d, x[1], 4, 0xA4BEEA44);
    d = md5Transform(md5H, d, a, b, c, x[4], 11, 0x4BDECFA9);
    c = md5Transform(md5H, c, d, a, b, x[7], 16, 0xF6BB4B60);
    b = md5Transform(md5H, b, c, d, a, x[10], 23, 0xBEBFBC70);
    a = md5Transform(md5H, a, b, c, d, x[13], 4, 0x289B7EC6);
    d = md5Transform(md5H, d, a, b, c, x[0], 11, 0xEAA127FA);
    c = md5Transform(md5H, c, d, a, b, x[3], 16, 0xD4EF3085);
    b = md5Transform(md5H, b, c, d, a, x[6], 23, 0x04881D05);
    a = md5Transform(md5H, a, b, c, d, x[9], 4, 0xD9D4D039);
    d = md5Transform(md5H, d, a, b, c, x[12], 11, 0xE6DB99E5);
    c = md5Transform(md5H, c, d, a, b, x[15], 16, 0x1FA27CF8);
    b = md5Transform(md5H, b, c, d, a, x[2], 23, 0xC4AC5665);

    // Round 4
    a = md5Transform(md5I, a, b, c, d, x[0], 6, 0xF4292244);
    d = md5Transform(md5I, d, a, b, c, x[7], 10, 0x432AFF97);
    c = md5Transform(md5I, c, d, a, b, x[14], 15, 0xAB9423A7);
    b = md5Transform(md5I, b, c, d, a, x[5], 21, 0xFC93A039);
    a = md5Transform(md5I, a, b, c, d, x[12], 6, 0x655B59C3);
    d = md5Transform(md5I, d, a, b, c, x[3], 10, 0x8F0CCC92);
    c = md5Transform(md5I, c, d, a, b, x[10], 15, 0xFFEFF47D);
    b = md5Transform(md5I, b, c, d, a, x[1], 21, 0x85845DD1);
    a = md5Transform(md5I, a, b, c, d, x[8], 6, 0x6FA87E4F);
    d = md5Transform(md5I, d, a, b, c, x[15], 10, 0xFE2CE6E0);
    c = md5Transform(md5I, c, d, a, b, x[6], 15, 0xA3014314);
    b = md5Transform(md5I, b, c, d, a, x[13], 21, 0x4E0811A1);
    a = md5Transform(md5I, a, b, c, d, x[4], 6, 0xF7537E82);
    d = md5Transform(md5I, d, a, b, c, x[11], 10, 0xBD3AF235);
    c = md5Transform(md5I, c, d, a, b, x[2], 15, 0x2AD7D2BB);
    b = md5Transform(md5I, b, c, d, a, x[9], 21, 0xEB86D391);

    a = addUnsigned(a, aa);
    b = addUnsigned(b, bb);
    c = addUnsigned(c, cc);
    d = addUnsigned(d, dd);
  }

  const toHex = (n: number): string => {
    let hex = '';
    for (let i = 0; i < 4; i++) {
      hex += ((n >>> (i * 8)) & 0xFF).toString(16).padStart(2, '0');
    }
    return hex;
  };

  return (toHex(a) + toHex(b) + toHex(c) + toHex(d)).toLowerCase();
}

/**
 * 生成 UUID v4
 */
function generateUUID(): string {
  return 'xxxxxxxx-xxxx-4xxx-yxxx-xxxxxxxxxxxx'.replace(/[xy]/g, c => {
    const r = (Math.random() * 16) | 0;
    const v = c === 'x' ? r : (r & 0x3) | 0x8;
    return v.toString(16);
  });
}

/**
 * HMAC-SHA256 签名
 * 使用 Web Crypto API
 */
function hmacSha256(key: string, data: string): string {
  // Web Crypto API 需要异步调用，但我们的目标是提供同步接口
  // 这里使用纯 JS 实现的备选方案
  // 在生产环境中建议使用 crypto-js 库
  return hmacSha256Simple(key, data);
}

/**
 * 简易 HMAC-SHA256 实现
 * 使用纯 JavaScript 实现
 */
function hmacSha256Simple(key: string, data: string): string {
  // SHA-256 常量
  const K: number[] = [
    0x428A2F98, 0x71374491, 0xB5C0FBCF, 0xE9B5DBA5,
    0x3956C25B, 0x59F111F1, 0x923F82A4, 0xAB1C5ED5,
    0xD807AA98, 0x12835B01, 0x243185BE, 0x550C7DC3,
    0x72BE5D74, 0x80DEB1FE, 0x9BDC06A7, 0xC19BF174,
    0xE49B69C1, 0xEFBE4786, 0x0FC19DC6, 0x240CA1CC,
    0x2DE92C6F, 0x4A7484AA, 0x5CB0A9DC, 0x76F988DA,
    0x983E5152, 0xA831C66D, 0xB00327C8, 0xBF597FC7,
    0xC6E00BF3, 0xD5A79147, 0x06CA6351, 0x14292967,
    0x27B70A85, 0x2E1B2138, 0x4D2C6DFC, 0x53380D13,
    0x650A7354, 0x766A0ABB, 0x81C2C92E, 0x92722C85,
    0xA2BFE8A1, 0xA81A664B, 0xC24B8B70, 0xC76C51A3,
    0xD192E819, 0xD6990624, 0xF40E3585, 0x106AA070,
    0x19A4C116, 0x1E376C08, 0x2748774C, 0x34B0BCB5,
    0x391C0CB3, 0x4ED8AA4A, 0x5B9CCA4F, 0x682E6FF3,
    0x748F82EE, 0x78A5636F, 0x84C87814, 0x8CC70208,
    0x90BEFFFA, 0xA4506CEB, 0xBEF9A3F7, 0xC67178F2
  ];

  function sha256(msg: number[]): number[] {
    const H: number[] = [
      0x6A09E667, 0xBB67AE85, 0x3C6EF372, 0xA54FF53A,
      0x510E527F, 0x9B05688C, 0x1F83D9AB, 0x5BE0CD19
    ];

    const l = msg.length * 8;
    msg.push(0x80);
    while ((msg.length * 8 + 64) % 512 !== 0) {
      msg.push(0);
    }
    const lengthBytes = new Array(8);
    for (let i = 0; i < 8; i++) {
      lengthBytes[i] = (l >>> (56 - i * 8)) & 0xFF;
    }
    for (let i = 0; i < 8; i++) {
      msg.push(lengthBytes[i]);
    }

    for (let i = 0; i < msg.length; i += 64) {
      const w: number[] = new Array(64);
      for (let t = 0; t < 16; t++) {
        w[t] = (msg[i + t * 4] << 24) | (msg[i + t * 4 + 1] << 16) | (msg[i + t * 4 + 2] << 8) | msg[i + t * 4 + 3];
      }
      for (let t = 16; t < 64; t++) {
        const s0 = (rightRotate(w[t - 15], 7) ^ rightRotate(w[t - 15], 18) ^ (w[t - 15] >>> 3));
        const s1 = (rightRotate(w[t - 2], 17) ^ rightRotate(w[t - 2], 19) ^ (w[t - 2] >>> 10));
        w[t] = (w[t - 16] + s0 + w[t - 7] + s1) | 0;
      }

      let [a, b, c, d, e, f, g, h] = H;

      for (let t = 0; t < 64; t++) {
        const S1 = (rightRotate(e, 6) ^ rightRotate(e, 11) ^ rightRotate(e, 25));
        const ch = (e & f) ^ (~e & g);
        const temp1 = (h + S1 + ch + K[t] + w[t]) | 0;
        const S0 = (rightRotate(a, 2) ^ rightRotate(a, 13) ^ rightRotate(a, 22));
        const maj = (a & b) ^ (a & c) ^ (b & c);
        const temp2 = (S0 + maj) | 0;

        h = g; g = f; f = e; e = (d + temp1) | 0;
        d = c; c = b; b = a; a = (temp1 + temp2) | 0;
      }

      H[0] = (H[0] + a) | 0; H[1] = (H[1] + b) | 0; H[2] = (H[2] + c) | 0; H[3] = (H[3] + d) | 0;
      H[4] = (H[4] + e) | 0; H[5] = (H[5] + f) | 0; H[6] = (H[6] + g) | 0; H[7] = (H[7] + h) | 0;
    }

    return H;
  }

  function rightRotate(value: number, amount: number): number {
    return (value >>> amount) | (value << (32 - amount));
  }

  function strToBytes(str: string): number[] {
    const bytes: number[] = [];
    for (let i = 0; i < str.length; i++) {
      bytes.push(str.charCodeAt(i) & 0xFF);
    }
    return bytes;
  }

  function bytesToHex(hash: number[]): string {
    return hash.map(h => {
      let hex = '';
      for (let i = 3; i >= 0; i--) {
        hex += ((h >>> (i * 8)) & 0xFF).toString(16).padStart(2, '0');
      }
      return hex;
    }).join('').toLowerCase();
  }

  // HMAC
  const blockSize = 64;
  const keyBytes = strToBytes(key);

  let actualKey: number[];
  if (keyBytes.length > blockSize) {
    actualKey = sha256(keyBytes).map(h => {
      const result: number[] = [];
      for (let i = 3; i >= 0; i--) {
        result.push((h >>> (i * 8)) & 0xFF);
      }
      return result;
    }).flat();
  } else {
    actualKey = [...keyBytes];
  }
  while (actualKey.length < blockSize) {
    actualKey.push(0);
  }

  const iPad = actualKey.map(b => b ^ 0x36);
  const oPad = actualKey.map(b => b ^ 0x5C);

  const innerHash = sha256([...iPad, ...strToBytes(data)]);
  const innerBytes = innerHash.map(h => {
    const result: number[] = [];
    for (let i = 3; i >= 0; i--) {
      result.push((h >>> (i * 8)) & 0xFF);
    }
    return result;
  }).flat();

  return bytesToHex(sha256([...oPad, ...innerBytes]));
}
