import type { TtsTemplates } from '@/platform/desktop';

export interface ValidationResult {
  valid: boolean;
  message: string | null;
}

const TEMPLATE_TOKENS = {
  danmu: new Set(['用户', '内容']),
  gift: new Set(['用户', '礼物', '数量']),
  captain: new Set(['用户', '等级'])
} as const;

export function validateVolume(value: number): ValidationResult {
  return Number.isFinite(value) && value >= 0 && value <= 1
    ? { valid: true, message: null }
    : { valid: false, message: '音量必须在 0 到 1 之间' };
}

export function validateHexColor(value: string): ValidationResult {
  return /^#[0-9a-fA-F]{6}$/.test(value)
    ? { valid: true, message: null }
    : { valid: false, message: '颜色必须使用 #RRGGBB 格式' };
}

export function validateTtsTemplate(
  value: string,
  kind: keyof TtsTemplates
): ValidationResult {
  if (value.trim().length === 0) {
    return { valid: false, message: '播报模板不能为空' };
  }
  if ([...value].some(character => {
    const code = character.charCodeAt(0);
    return code <= 31 || (code >= 127 && code <= 159);
  })) {
    return { valid: false, message: '播报模板不能包含换行或控制字符' };
  }
  const tokens = [...value.matchAll(/\{([^{}]+)\}/g)].map(match => match[1]);
  if (tokens.some(token => !TEMPLATE_TOKENS[kind].has(token as never))) {
    return { valid: false, message: '播报模板包含不支持的变量' };
  }
  return { valid: true, message: null };
}

export function validateTtsTemplates(templates: TtsTemplates): ValidationResult {
  for (const kind of ['danmu', 'gift', 'captain'] as const) {
    const result = validateTtsTemplate(templates[kind], kind);
    if (!result.valid) return result;
  }
  return { valid: true, message: null };
}

export function validateFileNameStem(value: string): ValidationResult {
  const trimmed = value.trim();
  if (trimmed.length === 0) return { valid: false, message: '文件名不能为空' };
  if (/[\\/:*?"<>|]/.test(trimmed)) {
    return { valid: false, message: '文件名不能包含路径或保留字符' };
  }
  return { valid: true, message: null };
}
