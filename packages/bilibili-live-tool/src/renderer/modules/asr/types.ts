/**
 * ASR 模块类型定义
 * 语音识别 (Automatic Speech Recognition)
 */

// ASR 引擎类型
export type ASREngine = 'whisper' | 'whisper-cpp' | 'vosk' | 'web-speech';

// ASR 配置
export interface ASRConfig {
  enabled: boolean;
  engine: ASREngine;
  language: string;  // 语言代码，如 'zh', 'en'
  model: string;  // 模型名称或路径
  continuous: boolean;  // 是否连续识别
  interimResults: boolean;  // 是否返回中间结果
  maxAlternatives: number;  // 最大候选结果数
  noiseReduction: boolean;  // 是否启用降噪
  autoStart: boolean;  // 是否自动开始识别
  hotkey: string;  // 快捷键
}

// ASR 状态
export type ASRStatus = 'idle' | 'listening' | 'processing' | 'error';

// ASR 结果
export interface ASRResult {
  text: string;
  confidence: number;  // 0-1
  isFinal: boolean;
  timestamp: number;
  alternatives?: ASRAlternative[];
}

// ASR 候选结果
export interface ASRAlternative {
  text: string;
  confidence: number;
}

// ASR 错误
export interface ASRError {
  code: string;
  message: string;
  details?: string;
}

// Whisper 模型大小
export type WhisperModelSize = 'tiny' | 'base' | 'small' | 'medium' | 'large';

// Whisper 模型信息
export interface WhisperModelInfo {
  size: WhisperModelSize;
  name: string;
  description: string;
  multilingual: boolean;
  parameters: string;
  relativeSpeed: string;
  requiredMemory: string;
}

// 默认 ASR 配置
export const DEFAULT_ASR_CONFIG: ASRConfig = {
  enabled: false,
  engine: 'whisper-cpp',
  language: 'zh',
  model: 'base',
  continuous: true,
  interimResults: true,
  maxAlternatives: 3,
  noiseReduction: true,
  autoStart: false,
  hotkey: 'CommandOrControl+Shift+V'
};

// Whisper 模型列表
export const WHISPER_MODELS: WhisperModelInfo[] = [
  {
    size: 'tiny',
    name: 'tiny',
    description: '最快，准确率较低',
    multilingual: true,
    parameters: '39M',
    relativeSpeed: '~32x',
    requiredMemory: '~1GB'
  },
  {
    size: 'base',
    name: 'base',
    description: '平衡速度和准确率',
    multilingual: true,
    parameters: '74M',
    relativeSpeed: '~16x',
    requiredMemory: '~1GB'
  },
  {
    size: 'small',
    name: 'small',
    description: '较好的准确率',
    multilingual: true,
    parameters: '244M',
    relativeSpeed: '~6x',
    requiredMemory: '~2GB'
  },
  {
    size: 'medium',
    name: 'medium',
    description: '高准确率',
    multilingual: true,
    parameters: '769M',
    relativeSpeed: '~2x',
    requiredMemory: '~5GB'
  },
  {
    size: 'large',
    name: 'large',
    description: '最高准确率，速度较慢',
    multilingual: true,
    parameters: '1550M',
    relativeSpeed: '1x',
    requiredMemory: '~10GB'
  }
];

// 支持的语言列表
export const SUPPORTED_LANGUAGES = [
  { code: 'zh', name: '中文' },
  { code: 'en', name: 'English' },
  { code: 'ja', name: '日本語' },
  { code: 'ko', name: '한국어' },
  { code: 'auto', name: '自动检测' }
];
