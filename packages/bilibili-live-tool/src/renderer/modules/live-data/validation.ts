export function normalizeSensitiveWord(value: string): string {
  return value.replace(/\p{Cc}/gu, '').trim();
}

export function sensitiveWordError(value: string): string {
  const length = Array.from(normalizeSensitiveWord(value)).length;
  return length >= 1 && length <= 100 ? '' : '敏感词长度需为 1-100 个字符';
}

export function userUidError(value: string): string {
  return /^\d{1,20}$/.test(value.trim()) ? '' : 'UID 需为 1-20 位数字';
}

export function usernameError(value: string): string {
  const length = Array.from(value.trim()).length;
  return length >= 1 && length <= 50 ? '' : '用户名长度需为 1-50 个字符';
}
