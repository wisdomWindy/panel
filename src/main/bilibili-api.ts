/**
 * Bilibili 开放平台 API - 主进程版本
 * 在主进程中发送请求，避免渲染进程的网络限制
 */

import crypto from 'crypto';

const BASE_URL = 'https://live-open.biliapi.com';

interface AuthHeaders {
  'Accept': string;
  'Content-Type': string;
  'x-bili-content-md5': string;
  'x-bili-timestamp': string;
  'x-bili-signature-method': string;
  'x-bili-signature-nonce': string;
  'x-bili-accesskeyid': string;
  'x-bili-signature-version': string;
  'Authorization': string;
}

function generateUUID(): string {
  return 'xxxxxxxx-xxxx-4xxx-yxxx-xxxxxxxxxxxx'.replace(/[xy]/g, c => {
    const r = (Math.random() * 16) | 0;
    const v = c === 'x' ? r : (r & 0x3) | 0x8;
    return v.toString(16);
  });
}

function generateAuthHeaders(
  accessKeyId: string,
  accessKeySecret: string,
  body: string
): AuthHeaders {
  const contentMd5 = crypto.createHash('md5').update(body).digest('hex');
  const nonce = generateUUID();
  const timestamp = Math.floor(Date.now() / 1000).toString();

  const signStr =
    `x-bili-accesskeyid:${accessKeyId}\n` +
    `x-bili-content-md5:${contentMd5}\n` +
    'x-bili-signature-method:HMAC-SHA256\n' +
    `x-bili-signature-nonce:${nonce}\n` +
    'x-bili-signature-version:1.0\n' +
    `x-bili-timestamp:${timestamp}`;

  const signature = crypto
    .createHmac('sha256', accessKeySecret)
    .update(signStr)
    .digest('hex');

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

export async function startProject(
  accessKeyId: string,
  accessKeySecret: string,
  code: string,
  appId: number
) {
  const body = JSON.stringify({ code, app_id: appId });
  const headers = generateAuthHeaders(accessKeyId, accessKeySecret, body);

  const response = await fetch(`${BASE_URL}/v2/app/start`, {
    method: 'POST',
    headers: headers as unknown as Record<string, string>,
    body
  });

  return await response.json();
}

export async function endProject(
  accessKeyId: string,
  accessKeySecret: string,
  appId: number,
  gameId: string
) {
  const body = JSON.stringify({ app_id: appId, game_id: gameId });
  const headers = generateAuthHeaders(accessKeyId, accessKeySecret, body);

  const response = await fetch(`${BASE_URL}/v2/app/end`, {
    method: 'POST',
    headers: headers as unknown as Record<string, string>,
    body
  });

  return await response.json();
}

export async function heartbeat(
  accessKeyId: string,
  accessKeySecret: string,
  gameId: string
) {
  const body = JSON.stringify({ game_id: gameId });
  const headers = generateAuthHeaders(accessKeyId, accessKeySecret, body);

  const response = await fetch(`${BASE_URL}/v2/app/heartbeat`, {
    method: 'POST',
    headers: headers as unknown as Record<string, string>,
    body
  });

  return await response.json();
}

export async function batchHeartbeat(
  accessKeyId: string,
  accessKeySecret: string,
  gameIds: string[]
) {
  const body = JSON.stringify({ game_ids: gameIds });
  const headers = generateAuthHeaders(accessKeyId, accessKeySecret, body);

  const response = await fetch(`${BASE_URL}/v2/app/batchHeartbeat`, {
    method: 'POST',
    headers: headers as unknown as Record<string, string>,
    body
  });

  return await response.json();
}
