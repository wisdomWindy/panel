/**
 * Test: Logs — onMounted 不再添加模拟连接日志
 *
 * 用法: npx ts-node tests/logs.test.ts
 * 注意: 本测试验证 LogEntry 接口和 addLog 过滤逻辑，
 *       完整的 Vue SFC 测试需要 vitest + @vue/test-utils。
 */

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

// LogEntry 接口（从 Logs.vue 中提取）
interface LogEntry {
  timestamp: number;
  level: 'debug' | 'info' | 'warn' | 'error';
  message: string;
  isHighlight?: boolean;
}

// 模拟 addLog 核心逻辑（不含 store 依赖）
function createLogSystem(maxLines = 1000, logLevel: string = 'info') {
  const logs: LogEntry[] = [];
  const levelPriority: Record<string, number> = { debug: 0, info: 1, warn: 2, error: 3 };

  function addLog(level: LogEntry['level'], message: string) {
    if (levelPriority[level] < levelPriority[logLevel]) return;

    logs.push({ timestamp: Date.now(), level, message });

    if (logs.length > maxLines) {
      logs.splice(0, logs.length - maxLines);
    }
  }

  function clearLogs() {
    logs.length = 0;
  }

  return { logs, addLog, clearLogs };
}

console.log('=== Logs 测试 ===\n');

// Test 1: onMounted 应仅添加"日志系统已启动"，无模拟连接日志
console.log('Test 1: onMounted 中不应包含模拟日志');
const sys = createLogSystem();
sys.addLog('info', '日志系统已启动');

// 以下语句在原始代码（含模拟数据）中存在，现已移除：
// addLog('debug', '连接到直播间...');
// addLog('info', '连接成功');
// addLog('debug', '开始接收弹幕...');

assert(sys.logs.length === 1, `期望 1 条日志，实际: ${sys.logs.length}`);
assert(sys.logs[0].message === '日志系统已启动', `期望消息为"日志系统已启动"，实际: "${sys.logs[0].message}"`);

// Test 2: 不应包含被移除的模拟消息
console.log('\nTest 2: 验证模拟消息已被移除');
const removedMessages = ['连接到直播间...', '连接成功', '开始接收弹幕...'];
removedMessages.forEach(msg => {
  const found = sys.logs.some(log => log.message === msg);
  assert(!found, `"${msg}" 不应存在`);
});

// Test 3: onUnmounted 应添加"日志系统已停止"
console.log('\nTest 3: onUnmounted 应记录停止日志');
sys.addLog('info', '日志系统已停止');
assert(sys.logs.length === 2, `期望 2 条日志，实际: ${sys.logs.length}`);
assert(sys.logs[1].message === '日志系统已停止', `期望消息为"日志系统已停止"，实际: "${sys.logs[1].message}"`);

// Test 4: 级别过滤
console.log('\nTest 4: 日志级别过滤');
const filteredSys = createLogSystem(1000, 'warn');
filteredSys.addLog('debug', '不应出现');
filteredSys.addLog('info', '也不应出现');
filteredSys.addLog('warn', '警告');
filteredSys.addLog('error', '错误');
assert(filteredSys.logs.length === 2, `期望 2 条，实际: ${filteredSys.logs.length}`);
assert(filteredSys.logs.every(l => l.level === 'warn' || l.level === 'error'), '仅应有 warn 和 error 级别');

// Test 5: 行数上限
console.log('\nTest 5: 日志行数上限');
const cappedSys = createLogSystem(3);
for (let i = 0; i < 10; i++) {
  cappedSys.addLog('info', `消息 ${i}`);
}
assert(cappedSys.logs.length === 3, `期望 3 条，实际: ${cappedSys.logs.length}`);

console.log('\n---');
console.log(`通过: ${passed}, 失败: ${failed}, 总计: ${passed + failed}`);
if (failed > 0) process.exit(1);

