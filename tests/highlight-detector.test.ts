import { HighlightDetector } from '../src/main/modules/recording/highlight-detector';
import type { HighlightConfig } from '../src/renderer/modules/recording/types';

let p = 0, f = 0;
function ok(c: boolean, l: string) { if (c) p++; else f++; console.log('  ' + (c ? '✓' : '✗ FAIL') + ': ' + l); }
function eq(a: any, e: any, l: string) { const m = JSON.stringify(a) === JSON.stringify(e); if (m) p++; else f++; console.log('  ' + (m ? '✓' : '✗ FAIL') + ': ' + l + (m ? '' : '\n    exp: ' + JSON.stringify(e) + '\n    got: ' + JSON.stringify(a))); }

const defaultConfig: HighlightConfig = {
  danmakuThreshold: 30,
  danmakuWindow: 10,
  giftValueThreshold: 100,
  captainTrigger: true,
  clipDuration: 30,
  beforeSeconds: 15,
  afterSeconds: 15,
  cooldownSeconds: 10
};

// ------- HighlightDetector tests -------

console.log('=== HighlightDetector: 初始化 ===');
{
  const detector = new HighlightDetector(defaultConfig);
  const status = detector.getStatus();
  ok(!status.isRunning, '初始状态未运行');
  eq(status.danmakuFrequency, 0, '初始弹幕频率为0');
  eq(status.lastHighlightTime, 0, '初始高光时间为0');
  eq(status.cooldownRemaining, 0, '初始冷却时间为0');
}

console.log('=== HighlightDetector: 启动和停止 ===');
{
  const detector = new HighlightDetector(defaultConfig);
  let startedCalled = false;
  let stoppedCalled = false;
  
  detector.on('started', () => { startedCalled = true; });
  detector.on('stopped', () => { stoppedCalled = true; });
  
  detector.start();
  ok(detector.getStatus().isRunning, '启动后状态为运行');
  ok(startedCalled, '触发started事件');
  
  detector.stop();
  ok(!detector.getStatus().isRunning, '停止后状态为未运行');
  ok(stoppedCalled, '触发stopped事件');
}

console.log('=== HighlightDetector: 弹幕激增检测 ===');
{
  const config = { ...defaultConfig, danmakuThreshold: 5, danmakuWindow: 10, cooldownSeconds: 0 };
  const detector = new HighlightDetector(config);
  let highlightReason = '';
  
  detector.on('highlight', (data) => { highlightReason = data.reason; });
  detector.start();
  
  // 发送5个弹幕触发高光
  for (let i = 0; i < 5; i++) {
    detector.handleDanmaku({
      userId: `user${i}`,
      username: `user${i}`,
      content: 'test',
      timestamp: Date.now()
    });
  }
  
  eq(highlightReason, 'danmaku_surge', '弹幕激增触发高光');
  detector.stop();
}

console.log('=== HighlightDetector: 高价值礼物检测 ===');
{
  const config = { ...defaultConfig, giftValueThreshold: 100, cooldownSeconds: 0 };
  const detector = new HighlightDetector(config);
  let highlightReason = '';
  let highlightDesc = '';
  
  detector.on('highlight', (data) => { 
    highlightReason = data.reason;
    highlightDesc = data.description;
  });
  detector.start();
  
  // 发送高价值礼物
  detector.handleGift({
    userId: 'user1',
    username: 'giftUser',
    giftId: 'gift1',
    giftName: '大礼物',
    price: 150,
    count: 1,
    timestamp: Date.now()
  });
  
  eq(highlightReason, 'high_value_gift', '高价值礼物触发高光');
  ok(highlightDesc.includes('giftUser'), '描述包含用户名');
  ok(highlightDesc.includes('150'), '描述包含礼物价值');
  detector.stop();
}

console.log('=== HighlightDetector: 低价值礼物不触发 ===');
{
  const config = { ...defaultConfig, giftValueThreshold: 100, cooldownSeconds: 0 };
  const detector = new HighlightDetector(config);
  let highlightCalled = false;
  
  detector.on('highlight', () => { highlightCalled = true; });
  detector.start();
  
  // 发送低价值礼物
  detector.handleGift({
    userId: 'user1',
    username: 'giftUser',
    giftId: 'gift1',
    giftName: '小礼物',
    price: 50,
    count: 1,
    timestamp: Date.now()
  });
  
  ok(!highlightCalled, '低价值礼物不触发高光');
  detector.stop();
}

console.log('=== HighlightDetector: 舰长赠送检测 ===');
{
  const config = { ...defaultConfig, captainTrigger: true, cooldownSeconds: 0 };
  const detector = new HighlightDetector(config);
  let highlightReason = '';
  let highlightDesc = '';
  
  detector.on('highlight', (data) => { 
    highlightReason = data.reason;
    highlightDesc = data.description;
  });
  detector.start();
  
  // 舰长赠送
  detector.handleCaptain({
    userId: 'user1',
    username: 'captainUser',
    level: 1,
    timestamp: Date.now()
  });
  
  eq(highlightReason, 'captain', '舰长赠送触发高光');
  ok(highlightDesc.includes('captainUser'), '描述包含用户名');
  ok(highlightDesc.includes('舰长'), '描述包含舰长');
  detector.stop();
}

console.log('=== HighlightDetector: 舰长触发关闭时不触发 ===');
{
  const config = { ...defaultConfig, captainTrigger: false, cooldownSeconds: 0 };
  const detector = new HighlightDetector(config);
  let highlightCalled = false;
  
  detector.on('highlight', () => { highlightCalled = true; });
  detector.start();
  
  detector.handleCaptain({
    userId: 'user1',
    username: 'captainUser',
    level: 1,
    timestamp: Date.now()
  });
  
  ok(!highlightCalled, '舰长触发关闭时不触发高光');
  detector.stop();
}

console.log('=== HighlightDetector: 冷却时间 ===');
{
  const config = { ...defaultConfig, cooldownSeconds: 10 };
  const detector = new HighlightDetector(config);
  let highlightCount = 0;
  
  detector.on('highlight', () => { highlightCount++; });
  detector.start();
  
  // 手动标记两次
  detector.manualMark('first');
  detector.manualMark('second');
  
  eq(highlightCount, 1, '冷却时间内只触发一次');
  detector.stop();
}

console.log('=== HighlightDetector: 手动标记 ===');
{
  const config = { ...defaultConfig, cooldownSeconds: 0 };
  const detector = new HighlightDetector(config);
  let highlightReason = '';
  let highlightDesc = '';
  
  detector.on('highlight', (data) => { 
    highlightReason = data.reason;
    highlightDesc = data.description;
  });
  detector.start();
  
  detector.manualMark('测试手动标记');
  
  eq(highlightReason, 'manual', '手动标记原因正确');
  eq(highlightDesc, '测试手动标记', '手动标记描述正确');
  detector.stop();
}

console.log('=== HighlightDetector: 弹幕频率统计 ===');
{
  const detector = new HighlightDetector(defaultConfig);
  detector.start();
  
  // 发送3个弹幕
  for (let i = 0; i < 3; i++) {
    detector.handleDanmaku({
      userId: `user${i}`,
      username: `user${i}`,
      content: 'test',
      timestamp: Date.now()
    });
  }
  
  eq(detector.getDanmakuFrequency(), 3, '弹幕频率统计正确');
  detector.stop();
}

console.log('=== HighlightDetector: 未运行时不处理事件 ===');
{
  const detector = new HighlightDetector({ ...defaultConfig, cooldownSeconds: 0 });
  let highlightCalled = false;
  
  detector.on('highlight', () => { highlightCalled = true; });
  
  // 未启动时处理事件
  detector.handleDanmaku({
    userId: 'user1',
    username: 'user1',
    content: 'test',
    timestamp: Date.now()
  });
  
  ok(!highlightCalled, '未运行时不处理弹幕事件');
}

console.log('=== HighlightDetector: 更新配置 ===');
{
  const detector = new HighlightDetector(defaultConfig);
  
  detector.updateConfig({ danmakuThreshold: 50 });
  
  // 验证配置已更新（通过行为验证）
  detector.start();
  let highlightCalled = false;
  detector.on('highlight', () => { highlightCalled = true; });
  
  // 发送30个弹幕（原阈值）
  for (let i = 0; i < 30; i++) {
    detector.handleDanmaku({
      userId: `user${i}`,
      username: `user${i}`,
      content: 'test',
      timestamp: Date.now()
    });
  }
  
  ok(!highlightCalled, '更新阈值后30个弹幕不触发');
  detector.stop();
}

console.log('');
console.log(`--- ${p} passed, ${f} failed, ${p + f} total ---`);
