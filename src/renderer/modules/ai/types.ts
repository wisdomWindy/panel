/**
 * AI 模块类型定义
 * LLM 对话生成
 */

// LLM 提供商类型
export type LLMProvider = 'openai' | 'claude' | 'ollama' | 'custom';

// AI 配置
export interface AIConfig {
  enabled: boolean;
  provider: LLMProvider;
  apiKey: string;
  model: string;
  endpoint: string;  // 自定义 API 端点
  personality: AIPersonality;
  contextWindow: number;  // 上下文弹幕数量
  maxTokens: number;  // 最大生成 token 数
  temperature: number;  // 温度参数 (0-2)
  minInterval: number;  // 最小发送间隔（秒）
  maxResponseLength: number;  // 最大回复长度（字符）
  sensitiveWordsFilter: boolean;  // 是否过滤敏感词
  autoReply: boolean;  // 是否自动回复
  replyToQuestions: boolean;  // 是否回复问题
  replyToMentions: boolean;  // 是否回复 @
}

// AI 人设配置
export interface AIPersonality {
  name: string;  // 人设名称
  tone: 'humorous' | 'gentle' | 'professional' | 'anime' | 'custom';
  responseStyle: 'casual' | 'formal' | 'friendly' | 'custom';
  systemPrompt: string;  // 系统提示词
  examples: string[];  // 示例回复
}

// 对话消息
export interface ChatMessage {
  role: 'system' | 'user' | 'assistant';
  content: string;
  timestamp: number;
  username?: string;
}

// AI 回复结果
export interface AIResponse {
  text: string;
  shouldSend: boolean;
  confidence: number;
  tokensUsed: number;
  latency: number;
}

// AI 状态
export type AIStatus = 'idle' | 'thinking' | 'generating' | 'error';

// AI 错误
export interface AIError {
  code: string;
  message: string;
  details?: string;
}

// LLM API 请求
export interface LLMRequest {
  messages: ChatMessage[];
  model: string;
  maxTokens: number;
  temperature: number;
  stream: boolean;
}

// LLM API 响应
export interface LLMResponse {
  content: string;
  finishReason: string;
  usage: {
    promptTokens: number;
    completionTokens: number;
    totalTokens: number;
  };
}

// 默认 AI 配置
export const DEFAULT_AI_CONFIG: AIConfig = {
  enabled: false,
  provider: 'openai',
  apiKey: '',
  model: 'gpt-3.5-turbo',
  endpoint: '',
  personality: {
    name: '直播助手',
    tone: 'friendly',
    responseStyle: 'casual',
    systemPrompt: '你是一个直播间的AI助手，负责和观众互动。请用友好、活泼的语气回复观众的问题和弹幕。回复要简洁，适合直播间场景。',
    examples: []
  },
  contextWindow: 10,
  maxTokens: 150,
  temperature: 0.7,
  minInterval: 10,
  maxResponseLength: 100,
  sensitiveWordsFilter: true,
  autoReply: true,
  replyToQuestions: true,
  replyToMentions: true
};

// 预设人设
export const PRESET_PERSONALITIES: Record<string, AIPersonality> = {
  friendly: {
    name: '友好助手',
    tone: 'friendly',
    responseStyle: 'casual',
    systemPrompt: '你是一个直播间的AI助手，负责和观众互动。请用友好、活泼的语气回复观众的问题和弹幕。回复要简洁，适合直播间场景。',
    examples: ['大家好呀~', '谢谢支持！', '这个问题很好！']
  },
  humorous: {
    name: '幽默达人',
    tone: 'humorous',
    responseStyle: 'casual',
    systemPrompt: '你是一个幽默风趣的直播间AI助手。用轻松幽默的方式回复观众，可以适当使用网络梗和表情。回复要简短有趣。',
    examples: ['哈哈，这个操作666', '笑死我了', '你是懂直播的']
  },
  anime: {
    name: '二次元',
    tone: 'anime',
    responseStyle: 'friendly',
    systemPrompt: '你是一个二次元风格的直播间AI助手。用可爱的语气说话，可以使用颜文字和二次元用语。回复要萌萌哒。',
    examples: ['呜呜呜好可爱', 'awsl', '大佬带带我']
  },
  professional: {
    name: '专业主播',
    tone: 'professional',
    responseStyle: 'formal',
    systemPrompt: '你是一个专业的直播间AI助手。用专业、礼貌的语气回复观众问题，提供有价值的信息。回复要准确、有条理。',
    examples: ['感谢您的提问', '这个问题的答案是...', '欢迎来到直播间']
  }
};
