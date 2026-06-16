import { HighlightDetector } from '../src/main/modules/recording/highlight-detector';
import type { HighlightConfig } from '../src/renderer/modules/recording/types';

console.log('=== 性能测试：高光检测器 ===\n');

// 测试1：大量弹幕处理性能
console.log('--- 测试1：大量弹幕处理性能 ---');
{
  const config: HighlightConfig = {
    danmakuThreshold: 1000,
    danmakuWindow: 10,
    giftValueThreshold: 100,
    captainTrigger: true,
    clipDuration: 30,
    beforeSeconds: 15,
    afterSeconds: 15,
    cooldownSeconds: 0
  };
  
  const detector = new HighlightDetector(config);
  detector.start();
  
  const iterations = 10000;
  const startTime = performance.now();
  const startMemory = process.memoryUsage().heapUsed;
  
  for (let i = 0; i < iterations; i++) {
    detector.handleDanmaku({
      userId: `user${i}`,
      username: `user${i}`,
      content: 'test message',
      timestamp: Date.now()
    });
  }
  
  const endTime = performance.now();
  const endMemory = process.memoryUsage().heapUsed;
  const duration = endTime - startTime;
  const memoryUsed = (endMemory - startMemory) / 1024 / 1024; // MB
  
  console.log(`  处理 ${iterations} 条弹幕`);
  console.log(`  耗时: ${duration.toFixed(2)} ms`);
  console.log(`  平均每条: ${(duration / iterations).toFixed(4)} ms`);
  console.log(`  内存增长: ${memoryUsed.toFixed(2)} MB`);
  console.log(`  性能: ${duration < 1000 ? '✓ 通过' : '✗ 未通过'} (< 1秒)`);
  
  detector.stop();
}

// 测试2：高频礼物处理性能
console.log('\n--- 测试2：高频礼物处理性能 ---');
{
  const config: HighlightConfig = {
    danmakuThreshold: 100,
    danmakuWindow: 10,
    giftValueThreshold: 10,
    captainTrigger: true,
    clipDuration: 30,
    beforeSeconds: 15,
    afterSeconds: 15,
    cooldownSeconds: 0
  };
  
  const detector = new HighlightDetector(config);
  detector.start();
  
  const iterations = 1000;
  const startTime = performance.now();
  
  for (let i = 0; i < iterations; i++) {
    detector.handleGift({
      userId: `user${i}`,
      username: `user${i}`,
      giftId: `gift${i}`,
      giftName: '礼物',
      price: 100,
      count: 1,
      timestamp: Date.now()
    });
  }
  
  const endTime = performance.now();
  const duration = endTime - startTime;
  
  console.log(`  处理 ${iterations} 个礼物`);
  console.log(`  耗时: ${duration.toFixed(2)} ms`);
  console.log(`  平均每个: ${(duration / iterations).toFixed(4)} ms`);
  console.log(`  性能: ${duration < 500 ? '✓ 通过' : '✗ 未通过'} (< 500ms)`);
  
  detector.stop();
}

// 测试3：内存泄漏检测
console.log('\n--- 测试3：内存泄漏检测 ---');
{
  const config: HighlightConfig = {
    danmakuThreshold: 10,
    danmakuWindow: 10,
    giftValueThreshold: 100,
    captainTrigger: true,
    clipDuration: 30,
    beforeSeconds: 15,
    afterSeconds: 15,
    cooldownSeconds: 0
  };
  
  const initialMemory = process.memoryUsage().heapUsed;
  
  // 创建和销毁多个检测器实例
  for (let i = 0; i < 100; i++) {
    const detector = new HighlightDetector(config);
    detector.start();
    
    // 处理一些事件
    for (let j = 0; j < 100; j++) {
      detector.handleDanmaku({
        userId: `user${j}`,
        username: `user${j}`,
        content: 'test',
        timestamp: Date.now()
      });
    }
    
    detector.stop();
    detector.destroy();
  }
  
  // 强制垃圾回收（如果可用）
  if (global.gc) {
    global.gc();
  }
  
  const finalMemory = process.memoryUsage().heapUsed;
  const memoryGrowth = (finalMemory - initialMemory) / 1024 / 1024; // MB
  
  console.log(`  初始内存: ${(initialMemory / 1024 / 1024).toFixed(2)} MB`);
  console.log(`  最终内存: ${(finalMemory / 1024 / 1024).toFixed(2)} MB`);
  console.log(`  内存增长: ${memoryGrowth.toFixed(2)} MB`);
  console.log(`  泄漏检测: ${memoryGrowth < 10 ? '✓ 通过' : '✗ 未通过'} (< 10MB)`);
}

// 测试4：并发事件处理性能
console.log('\n--- 测试4：并发事件处理性能 ---');
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
  let eventCount = 0;
  
  detector.on('highlight', () => { eventCount++; });
  detector.start();
  
  const iterations = 1000;
  const startTime = performance.now();
  
  // 模拟并发事件
  for (let i = 0; i < iterations; i++) {
    // 随机选择事件类型
    const eventType = i % 3;
    
    switch (eventType) {
      case 0:
        detector.handleDanmaku({
          userId: `user${i}`,
          username: `user${i}`,
          content: 'test',
          timestamp: Date.now()
        });
        break;
      case 1:
        detector.handleGift({
          userId: `user${i}`,
          username: `user${i}`,
          giftId: `gift${i}`,
          giftName: '礼物',
          price: 200,
          count: 1,
          timestamp: Date.now()
        });
        break;
      case 2:
        detector.handleCaptain({
          userId: `user${i}`,
          username: `user${i}`,
          level: 1,
          timestamp: Date.now()
        });
        break;
    }
  }
  
  const endTime = performance.now();
  const duration = endTime - startTime;
  
  console.log(`  处理 ${iterations} 个混合事件`);
  console.log(`  耗时: ${duration.toFixed(2)} ms`);
  console.log(`  平均每个: ${(duration / iterations).toFixed(4)} ms`);
  console.log(`  触发高光: ${eventCount} 次`);
  console.log(`  性能: ${duration < 1000 ? '✓ 通过' : '✗ 未通过'} (< 1秒)`);
  
  detector.stop();
}

console.log('\n=== 性能测试完成 ===');
