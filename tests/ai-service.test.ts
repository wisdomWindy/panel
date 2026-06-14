/**
 * Test: AI Service — 无 API Key 时应抛错而非返回模拟数据
 *
 * 用法: npx tsx tests/ai-service.test.ts
 */

import { AIServiceImpl } from '../src/renderer/modules/ai/ai-service';
import { AIProvider, AIPersonality, AIResponseStyle } from '../src/renderer/modules/ai/types';

let passed = 0;
let failed = 0;

function assert(condition: boolean, label: string) {
  if (condition) {
    passed++;
    console.log(`  ✓ ${label}`);
  } else {
    failed++;
    console.log(`  ✗ FAIL: ${label}`);
  }
}

async function run() {
  console.log('=== AI Service 测试 ===\n');

  // Test 1: generateResponse 无 apiKey 时必须抛错
  console.log('Test 1: generateResponse 在 apiKey="" 时应抛错');
  const service = new AIServiceImpl();
  service.setConfig({
    enabled: true,
    provider: AIProvider.OPENAI,
    apiKey: '',
    model: 'gpt-3.5-turbo',
    endpoint: 'https://api.openai.com/v1/chat/completions',
    personality: {
      name: '助手',
      tone: AIPersonality.GENTLE,
      responseStyle: AIResponseStyle.FRIENDLY
    },
    contextWindow: 10,
    minInterval: 10,
    sensitiveWords: [],
    maxTokens: 100,
    temperature: 0.7
  });

  await service.initialize();

  try {
    await service.generateResponse(
      { messages: [], recentDanmaku: [], recentGifts: [], recentCaptains: [], lastResponseTime: 0, responseCount: 0 },
      [{ role: 'user', content: '测试', timestamp: Date.now() }]
    );
    assert(false, '应抛出异常但未抛出');
  } catch (err) {
    const msg = err instanceof Error ? err.message : '';
    assert(
      msg.includes('API Key'),
      `错误消息应包含 "API Key"，实际: "${msg}"`
    );
  }

  // Test 2: generateResponse 有 apiKey 但未 enabled 时必须抛错
  console.log('\nTest 2: generateResponse 在 enabled=false 时应抛错');
  const service2 = new AIServiceImpl();
  service2.setConfig({
    enabled: false,
    provider: AIProvider.OPENAI,
    apiKey: 'dummy-key',
    model: 'gpt-3.5-turbo',
    endpoint: 'https://api.openai.com/v1/chat/completions',
    personality: {
      name: '助手',
      tone: AIPersonality.GENTLE,
      responseStyle: AIResponseStyle.FRIENDLY
    },
    contextWindow: 10,
    minInterval: 10,
    sensitiveWords: [],
    maxTokens: 100,
    temperature: 0.7
  });

  await service2.initialize();

  try {
    await service2.generateResponse(
      { messages: [], recentDanmaku: [], recentGifts: [], recentCaptains: [], lastResponseTime: 0, responseCount: 0 },
      [{ role: 'user', content: '测试', timestamp: Date.now() }]
    );
    assert(false, '应抛出异常但未抛出');
  } catch (err) {
    const msg = err instanceof Error ? err.message : '';
    assert(
      msg.includes('禁用'),
      `错误消息应包含 "禁用"，实际: "${msg}"`
    );
  }

  // Test 3: 未初始化时应抛错
  console.log('\nTest 3: 未初始化时应抛错');
  const service3 = new AIServiceImpl();
  try {
    await service3.generateResponse(
      { messages: [], recentDanmaku: [], recentGifts: [], recentCaptains: [], lastResponseTime: 0, responseCount: 0 },
      [{ role: 'user', content: '测试', timestamp: Date.now() }]
    );
    assert(false, '应抛出异常但未抛出');
  } catch (err) {
    const msg = err instanceof Error ? err.message : '';
    assert(
      msg.includes('未初始化'),
      `错误消息应包含 "未初始化"，实际: "${msg}"`
    );
  }

  // Summary
  console.log(`\n---`);
  console.log(`通过: ${passed}, 失败: ${failed}, 总计: ${passed + failed}`);
  if (failed > 0) {
    process.exit(1);
  }
}

run().catch(err => {
  console.error('测试运行异常:', err);
  process.exit(1);
});

