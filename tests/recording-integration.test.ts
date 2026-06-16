import { HighlightDetector } from '../src/main/modules/recording/highlight-detector';
import type { HighlightConfig } from '../src/renderer/modules/recording/types';

let p = 0, f = 0;
function ok(c: boolean, l: string) { if (c) p++; else f++; console.log('  ' + (c ? '✓' : '✗ FAIL') + ': ' + l); }
function eq(a: any, e: any, l: string) { const m = JSON.stringify(a) === JSON.stringify(e); if (m) p++; else f++; console.log('  ' + (m ? '✓' : '✗ FAIL') + ': ' + l + (m ? '' : '\n    exp: ' + JSON.stringify(e) + '\n    got: ' + JSON.stringify(a))); }

// ------- 集成测试：高光检测流程 -------

console.log('=== 集成测试：完整的高光检测流程 ===');
{
  const config: HighlightConfig = {
    danmakuThreshold: 10,
    danmakuWindow: 5,
    giftValueThreshold: 50,
    captainTrigger: true,
    clipDuration: 30,
    beforeSeconds: 15,
    afterSeconds: 15,
    cooldownSeconds: 2
  };
  
  const detector = new HighlightDetector(config);
  const highlights: Array<{ reason: string; description: string }> = [];
  
  detector.on('highlight', (data) => {
    highlights.push({ reason: data.reason, description: data.description });
  });
  
  // 启动检测器
  detector.start();
  ok(detector.getStatus().isRunning, '检测器已启动');
  
  // 场景1：弹幕激增
  console.log('\n--- 场景1：弹幕激增 ---');
  const now = Date.now();
  for (let i = 0; i < 10; i++) {
    detector.handleDanmaku({
      userId: `user${i}`,
      username: `user${i}`,
      content: '666',
      timestamp: now + i * 100
    });
  }
  
  eq(highlights.length, 1, '弹幕激增触发1次高光');
  eq(highlights[0].reason, 'danmaku_surge', '原因为弹幕激增');
  
  // 等待冷却时间
  console.log('\n--- 等待冷却时间 ---');
  const cooldownStart = Date.now();
  while (Date.now() - cooldownStart < 2100) {
    // 等待2.1秒
  }
  
  // 场景2：高价值礼物
  console.log('\n--- 场景2：高价值礼物 ---');
  detector.handleGift({
    userId: 'giftUser',
    username: '大佬',
    giftId: 'gift1',
    giftName: '火箭',
    price: 100,
    count: 1,
    timestamp: Date.now()
  });
  
  eq(highlights.length, 2, '高价值礼物触发高光');
  eq(highlights[1].reason, 'high_value_gift', '原因为高价值礼物');
  
  // 场景3：舰长赠送
  console.log('\n--- 场景3：舰长赠送 ---');
  // 等待冷却时间
  const cooldownStart2 = Date.now();
  while (Date.now() - cooldownStart2 < 2100) {
    // 等待2.1秒
  }
  
  detector.handleCaptain({
    userId: 'captainUser',
    username: '舰长大人',
    level: 1,
    timestamp: Date.now()
  });
  
  eq(highlights.length, 3, '舰长赠送触发高光');
  eq(highlights[2].reason, 'captain', '原因为舰长赠送');
  
  // 场景4：手动标记
  console.log('\n--- 场景4：手动标记 ---');
  const cooldownStart3 = Date.now();
  while (Date.now() - cooldownStart3 < 2100) {
    // 等待2.1秒
  }
  
  detector.manualMark('精彩操作');
  
  eq(highlights.length, 4, '手动标记触发高光');
  eq(highlights[3].reason, 'manual', '原因为手动标记');
  eq(highlights[3].description, '精彩操作', '描述正确');
  
  // 停止检测器
  detector.stop();
  ok(!detector.getStatus().isRunning, '检测器已停止');
}

console.log('\n=== 集成测试：弹幕频率监控 ===');
{
  const config: HighlightConfig = {
    danmakuThreshold: 5,
    danmakuWindow: 2,
    giftValueThreshold: 100,
    captainTrigger: true,
    clipDuration: 30,
    beforeSeconds: 15,
    afterSeconds: 15,
    cooldownSeconds: 0
  };
  
  const detector = new HighlightDetector(config);
  detector.start();
  
  // 初始频率
  eq(detector.getDanmakuFrequency(), 0, '初始频率为0');
  
  // 发送3个弹幕
  const now = Date.now();
  for (let i = 0; i < 3; i++) {
    detector.handleDanmaku({
      userId: `user${i}`,
      username: `user${i}`,
      content: 'test',
      timestamp: now
    });
  }
  
  eq(detector.getDanmakuFrequency(), 3, '频率更新为3');
  
  detector.stop();
}

console.log('\n=== 集成测试：配置动态更新 ===');
{
  const config: HighlightConfig = {
    danmakuThreshold: 100,
    danmakuWindow: 10,
    giftValueThreshold: 100,
    captainTrigger: true,
    clipDuration: 30,
    beforeSeconds: 15,
    afterSeconds: 15,
    cooldownSeconds: 0
  };
  
  const detector = new HighlightDetector(config);
  let highlightCount = 0;
  
  detector.on('highlight', () => { highlightCount++; });
  detector.start();
  
  // 使用高阈值，发送50个弹幕
  const now = Date.now();
  for (let i = 0; i < 50; i++) {
    detector.handleDanmaku({
      userId: `user${i}`,
      username: `user${i}`,
      content: 'test',
      timestamp: now + i * 100
    });
  }
  
  eq(highlightCount, 0, '高阈值时不触发');
  
  // 更新配置降低阈值
  detector.updateConfig({ danmakuThreshold: 10 });
  
  // 由于之前的弹幕还在窗口内，再发送10个弹幕会累积触发
  // 这里我们验证配置更新生效：发送足够多的弹幕触发
  for (let i = 50; i < 70; i++) {
    detector.handleDanmaku({
      userId: `user${i}`,
      username: `user${i}`,
      content: 'test',
      timestamp: now + i * 100
    });
  }
  
  ok(highlightCount >= 1, '降低阈值后触发高光');
  
  detector.stop();
}

console.log('\n=== 集成测试：多种礼物类型 ===');
{
  const config: HighlightConfig = {
    danmakuThreshold: 100,
    danmakuWindow: 10,
    giftValueThreshold: 100,
    captainTrigger: true,
    clipDuration: 30,
    beforeSeconds: 15,
    afterSeconds: 15,
    cooldownSeconds: 0
  };
  
  const detector = new HighlightDetector(config);
  const reasons: string[] = [];
  
  detector.on('highlight', (data) => { reasons.push(data.reason); });
  detector.start();
  
  // 高价值礼物
  detector.handleGift({
    userId: 'user1',
    username: 'user1',
    giftId: 'g1',
    giftName: '大礼物',
    price: 200,
    count: 1,
    timestamp: Date.now()
  });
  
  // 低价值礼物（不触发）
  detector.handleGift({
    userId: 'user2',
    username: 'user2',
    giftId: 'g2',
    giftName: '小礼物',
    price: 10,
    count: 1,
    timestamp: Date.now()
  });
  
  // 高数量低价值礼物（触发）
  detector.handleGift({
    userId: 'user3',
    username: 'user3',
    giftId: 'g3',
    giftName: '连击礼物',
    price: 20,
    count: 10,
    timestamp: Date.now()
  });
  
  eq(reasons.length, 2, '触发2次高光');
  eq(reasons[0], 'high_value_gift', '第一次高价值礼物');
  eq(reasons[1], 'high_value_gift', '第二次高数量礼物');
  
  detector.stop();
}

console.log('\n=== 集成测试：舰长等级检测 ===');
{
  const config: HighlightConfig = {
    danmakuThreshold: 100,
    danmakuWindow: 10,
    giftValueThreshold: 100,
    captainTrigger: true,
    clipDuration: 30,
    beforeSeconds: 15,
    afterSeconds: 15,
    cooldownSeconds: 0
  };
  
  const detector = new HighlightDetector(config);
  const descriptions: string[] = [];
  
  detector.on('highlight', (data) => { descriptions.push(data.description); });
  detector.start();
  
  // 舰长
  detector.handleCaptain({
    userId: 'user1',
    username: '舰长用户',
    level: 1,
    timestamp: Date.now()
  });
  
  // 提督
  detector.handleCaptain({
    userId: 'user2',
    username: '提督用户',
    level: 2,
    timestamp: Date.now()
  });
  
  // 总督
  detector.handleCaptain({
    userId: 'user3',
    username: '总督用户',
    level: 3,
    timestamp: Date.now()
  });
  
  eq(descriptions.length, 3, '触发3次高光');
  ok(descriptions[0].includes('舰长'), '舰长描述正确');
  ok(descriptions[1].includes('提督'), '提督描述正确');
  ok(descriptions[2].includes('总督'), '总督描述正确');
  
  detector.stop();
}

console.log('');
console.log(`--- ${p} passed, ${f} failed, ${p + f} total ---`);
