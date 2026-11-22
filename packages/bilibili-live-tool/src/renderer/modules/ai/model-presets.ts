/**
 * Mainstream chat models reachable through an OpenAI-compatible endpoint.
 * The list is only a starting point: the operator can type any model id, and
 * "添加模型" keeps their own names for the next time (PRD 9.13).
 */
export interface ModelPreset {
  readonly value: string;
  readonly label: string;
  readonly vendor: string;
}

export const MODEL_PRESETS: readonly ModelPreset[] = [
  // International
  { value: 'gpt-4o', label: 'GPT-4o', vendor: 'OpenAI' },
  { value: 'gpt-4o-mini', label: 'GPT-4o mini', vendor: 'OpenAI' },
  { value: 'gpt-4.1', label: 'GPT-4.1', vendor: 'OpenAI' },
  { value: 'gpt-4.1-mini', label: 'GPT-4.1 mini', vendor: 'OpenAI' },
  { value: 'o4-mini', label: 'o4-mini', vendor: 'OpenAI' },
  { value: 'claude-3-5-sonnet-latest', label: 'Claude 3.5 Sonnet', vendor: 'Anthropic' },
  { value: 'gemini-2.0-flash', label: 'Gemini 2.0 Flash', vendor: 'Google' },
  { value: 'llama-3.3-70b-instruct', label: 'Llama 3.3 70B', vendor: 'Meta' },
  // Domestic
  { value: 'deepseek-chat', label: 'DeepSeek Chat', vendor: 'DeepSeek' },
  { value: 'deepseek-reasoner', label: 'DeepSeek Reasoner', vendor: 'DeepSeek' },
  { value: 'qwen-plus', label: 'Qwen Plus', vendor: '阿里云' },
  { value: 'qwen-max', label: 'Qwen Max', vendor: '阿里云' },
  { value: 'qwen2.5-72b-instruct', label: 'Qwen2.5 72B', vendor: '阿里云' },
  { value: 'moonshot-v1-8k', label: 'Moonshot v1 8K', vendor: 'Moonshot' },
  { value: 'glm-4-plus', label: 'GLM-4 Plus', vendor: '智谱' },
  { value: 'glm-4-flash', label: 'GLM-4 Flash', vendor: '智谱' },
  { value: 'doubao-pro-32k', label: '豆包 Pro 32K', vendor: '字节跳动' },
  { value: 'ernie-4.0-8k', label: '文心一言 4.0', vendor: '百度' },
  { value: 'hunyuan-lite', label: '混元 Lite', vendor: '腾讯' },
  { value: 'spark-lite', label: '星火 Lite', vendor: '讯飞' }
];

/** Custom ids first (the operator chose them), then the built-in presets. */
export function mergeModelOptions(custom: readonly string[]): ModelPreset[] {
  const seen = new Set<string>();
  const options: ModelPreset[] = [];
  for (const value of custom) {
    const trimmed = value.trim();
    if (!trimmed || seen.has(trimmed)) continue;
    seen.add(trimmed);
    options.push({ value: trimmed, label: trimmed, vendor: '' });
  }
  for (const preset of MODEL_PRESETS) {
    if (seen.has(preset.value)) continue;
    seen.add(preset.value);
    options.push(preset);
  }
  return options;
}

/** Normalises a user-entered list: trimmed, de-duplicated, order preserved. */
export function normalizeCustomModels(values: readonly unknown[]): string[] {
  const seen = new Set<string>();
  const result: string[] = [];
  for (const value of values) {
    if (typeof value !== 'string') continue;
    const trimmed = value.trim();
    if (!trimmed || seen.has(trimmed)) continue;
    seen.add(trimmed);
    result.push(trimmed);
  }
  return result;
}
