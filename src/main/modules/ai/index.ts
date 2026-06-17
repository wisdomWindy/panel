/**
 * AI 主进程模块
 * 处理 LLM API 调用
 */

import { ipcMain } from 'electron';
import type { ChatMessage, LLMResponse } from '../../../renderer/modules/ai/types.js';

// LLM 提供商配置
const LLM_ENDPOINTS: Record<string, string> = {
  openai: 'https://api.openai.com/v1/chat/completions',
  claude: 'https://api.anthropic.com/v1/messages',
  ollama: 'http://localhost:11434/api/chat',
  custom: ''
};

/**
 * 调用 OpenAI API
 */
async function callOpenAI(apiKey: string, model: string, messages: ChatMessage[], maxTokens: number, temperature: number): Promise<LLMResponse> {
  const response = await fetch(LLM_ENDPOINTS.openai, {
    method: 'POST',
    headers: {
      'Content-Type': 'application/json',
      'Authorization': `Bearer ${apiKey}`
    },
    body: JSON.stringify({
      model,
      messages: messages.map(m => ({ role: m.role, content: m.content })),
      max_tokens: maxTokens,
      temperature
    })
  });

  if (!response.ok) {
    const error = await response.text();
    throw new Error(`OpenAI API error: ${response.status} - ${error}`);
  }

  const data = await response.json();
  return {
    content: data.choices[0].message.content,
    finishReason: data.choices[0].finish_reason,
    usage: {
      promptTokens: data.usage.prompt_tokens,
      completionTokens: data.usage.completion_tokens,
      totalTokens: data.usage.total_tokens
    }
  };
}

/**
 * 调用 Claude API
 */
async function callClaude(apiKey: string, model: string, messages: ChatMessage[], maxTokens: number, temperature: number): Promise<LLMResponse> {
  const systemMessage = messages.find(m => m.role === 'system');
  const userMessages = messages.filter(m => m.role !== 'system');

  const response = await fetch(LLM_ENDPOINTS.claude, {
    method: 'POST',
    headers: {
      'Content-Type': 'application/json',
      'x-api-key': apiKey,
      'anthropic-version': '2023-06-01'
    },
    body: JSON.stringify({
      model,
      max_tokens: maxTokens,
      temperature,
      system: systemMessage?.content || '',
      messages: userMessages.map(m => ({ role: m.role, content: m.content }))
    })
  });

  if (!response.ok) {
    const error = await response.text();
    throw new Error(`Claude API error: ${response.status} - ${error}`);
  }

  const data = await response.json();
  return {
    content: data.content[0].text,
    finishReason: data.stop_reason,
    usage: {
      promptTokens: data.usage.input_tokens,
      completionTokens: data.usage.output_tokens,
      totalTokens: data.usage.input_tokens + data.usage.output_tokens
    }
  };
}

/**
 * 调用 Ollama API
 */
async function callOllama(model: string, messages: ChatMessage[], maxTokens: number, temperature: number): Promise<LLMResponse> {
  const response = await fetch(LLM_ENDPOINTS.ollama, {
    method: 'POST',
    headers: {
      'Content-Type': 'application/json'
    },
    body: JSON.stringify({
      model,
      messages: messages.map(m => ({ role: m.role, content: m.content })),
      stream: false,
      options: {
        num_predict: maxTokens,
        temperature
      }
    })
  });

  if (!response.ok) {
    const error = await response.text();
    throw new Error(`Ollama API error: ${response.status} - ${error}`);
  }

  const data = await response.json();
  return {
    content: data.message.content,
    finishReason: 'stop',
    usage: {
      promptTokens: data.prompt_eval_count || 0,
      completionTokens: data.eval_count || 0,
      totalTokens: (data.prompt_eval_count || 0) + (data.eval_count || 0)
    }
  };
}

/**
 * 调用自定义 API
 */
async function callCustom(endpoint: string, apiKey: string, model: string, messages: ChatMessage[], maxTokens: number, temperature: number): Promise<LLMResponse> {
  const response = await fetch(endpoint, {
    method: 'POST',
    headers: {
      'Content-Type': 'application/json',
      'Authorization': `Bearer ${apiKey}`
    },
    body: JSON.stringify({
      model,
      messages: messages.map(m => ({ role: m.role, content: m.content })),
      max_tokens: maxTokens,
      temperature
    })
  });

  if (!response.ok) {
    const error = await response.text();
    throw new Error(`Custom API error: ${response.status} - ${error}`);
  }

  const data = await response.json();
  return {
    content: data.choices?.[0]?.message?.content || data.content?.[0]?.text || '',
    finishReason: data.choices?.[0]?.finish_reason || 'stop',
    usage: {
      promptTokens: data.usage?.prompt_tokens || 0,
      completionTokens: data.usage?.completion_tokens || 0,
      totalTokens: data.usage?.total_tokens || 0
    }
  };
}

/**
 * 初始化 AI 模块
 */
export function initAIModule(): void {
  // 注册 IPC 处理器
  ipcMain.handle('ai:chat', async (_event, params: {
    provider: string;
    apiKey: string;
    model: string;
    endpoint: string;
    messages: ChatMessage[];
    maxTokens: number;
    temperature: number;
  }) => {
    try {
      let response: LLMResponse;

      switch (params.provider) {
        case 'openai':
          response = await callOpenAI(params.apiKey, params.model, params.messages, params.maxTokens, params.temperature);
          break;
        case 'claude':
          response = await callClaude(params.apiKey, params.model, params.messages, params.maxTokens, params.temperature);
          break;
        case 'ollama':
          response = await callOllama(params.model, params.messages, params.maxTokens, params.temperature);
          break;
        case 'custom':
          response = await callCustom(params.endpoint, params.apiKey, params.model, params.messages, params.maxTokens, params.temperature);
          break;
        default:
          throw new Error(`Unknown provider: ${params.provider}`);
      }

      return { success: true, response };
    } catch (error) {
      return { success: false, error: (error as Error).message };
    }
  });

  ipcMain.handle('ai:checkAvailability', async (_event, params: {
    provider: string;
    apiKey: string;
    endpoint: string;
  }) => {
    try {
      // 尝试发送一个简单的请求来检查 API 可用性
      const testMessages: ChatMessage[] = [
        { role: 'user', content: 'Hello', timestamp: Date.now() }
      ];

      switch (params.provider) {
        case 'openai':
          await callOpenAI(params.apiKey, 'gpt-3.5-turbo', testMessages, 10, 0.7);
          break;
        case 'claude':
          await callClaude(params.apiKey, 'claude-3-haiku-20240307', testMessages, 10, 0.7);
          break;
        case 'ollama':
          await callOllama('llama2', testMessages, 10, 0.7);
          break;
        case 'custom':
          if (!params.endpoint) {
            return { available: false, error: 'Custom endpoint not configured' };
          }
          await callCustom(params.endpoint, params.apiKey, 'default', testMessages, 10, 0.7);
          break;
        default:
          return { available: false, error: `Unknown provider: ${params.provider}` };
      }

      return { available: true };
    } catch (error) {
      return { available: false, error: (error as Error).message };
    }
  });
}

/**
 * 清理 AI 模块
 */
export function cleanupAIModule(): void {
  ipcMain.removeHandler('ai:chat');
  ipcMain.removeHandler('ai:checkAvailability');
}
