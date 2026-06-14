/**
 * Test: VariableReplacer — 变量替换工具 (TDD)
 *
 * 用法: npx tsx tests/variable-replacer.test.ts
 */

import {
  VariableReplacer,
  replaceVariables,
  replaceVariablesBatch,
  extractVariables
} from '../src/renderer/modules/utils/variable-replacer';

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

function assertEq<T>(actual: T, expected: T, label: string) {
  const ok = JSON.stringify(actual) === JSON.stringify(expected);
  if (ok) {
    passed++;
    console.log(`  ✓ ${label}`);
  } else {
    failed++;
    console.log(`  ✗ FAIL: ${label}\n    期望: ${JSON.stringify(expected)}\n    实际: ${JSON.stringify(actual)}`);
  }
}

console.log('=== VariableReplacer 测试 ===\n');

// --- replace() ---
console.log('replace() — 基础替换');
const replacer = new VariableReplacer();
assertEq(replacer.replace('Hello {name}', { name: 'World' }), 'Hello World', '简单变量替换');
assertEq(replacer.replace('{greeting}, {name}!', { greeting: 'Hi', name: 'Alice' }), 'Hi, Alice!', '多变量替换');
assertEq(replacer.replace('No variables here', {}), 'No variables here', '无变量原样返回');
assertEq(replacer.replace('Value: {num}', { num: 42 }), 'Value: 42', '数字类型转字符串');

// --- 缺失变量处理（非严格模式） ---
console.log('\nreplace() — 缺失变量处理');
assertEq(replacer.replace('{missing}', {}), '{missing}', '非严格模式占位符保留');
assertEq(replacer.replace('{a} {b}', { a: 'ok' }), 'ok {b}', '部分缺失');

// --- 严格模式 ---
console.log('\nreplace() — 严格模式');
const strictReplacer = new VariableReplacer({ strict: true });
try {
  strictReplacer.replace('{missing}', {});
  assert(false, '严格模式下缺失变量应抛错');
} catch (err) {
  assert(err instanceof Error && err.message.includes('missing'), '严格模式抛错包含变量名');
}

// --- 嵌套属性访问 ---
console.log('\nreplace() — 嵌套属性');
const userCtx = { user: { name: 'Bob', profile: { age: 25 } } };
assertEq(replacer.replace('{user.name}', userCtx), 'Bob', '嵌套一层');
assertEq(replacer.replace('{user.profile.age}', userCtx), '25', '嵌套两层');
assertEq(replacer.replace('{user.missing}', userCtx), '{user.missing}', '嵌套路径缺失保留占位符');

// --- null/undefined ---
console.log('\nreplace() — null/undefined 值');
assertEq(replacer.replace('{val}', { val: null }), '', 'null → 空字符串');
assertEq(replacer.replace('{val}', { val: undefined }), '', 'undefined → 空字符串');

// --- 对象值 ---
console.log('\nreplace() — 对象值');
assertEq(replacer.replace('{obj}', { obj: { x: 1 } }), '{"x":1}', '对象序列化为 JSON');

// --- 函数值 ---
console.log('\nreplace() — 函数值');
const fnCtx = { random: () => 'fixed' };
assertEq(replacer.replace('{random}', fnCtx), 'fixed', '函数值调用');

// --- 自定义前缀／后缀 ---
console.log('\nreplace() — 自定义分隔符');
const customReplacer = new VariableReplacer({ prefix: '{{', suffix: '}}' });
assertEq(customReplacer.replace('Hello {{name}}', { name: 'World' }), 'Hello World', '双花括号');

// --- replaceBatch ---
console.log('\nreplaceBatch — 批量替换');
assertEq(
  replacer.replaceBatch(['Hi {name}', 'Bye {name}'], { name: 'X' }),
  ['Hi X', 'Bye X'],
  '批量替换'
);

// --- extractVariables ---
console.log('\nextractVariables — 提取变量');
assertEq(replacer.extractVariables('{a} and {b} again {a}'), ['a', 'b'], '提取去重');
assertEq(replacer.extractVariables('no vars'), [], '无变量返回空数组');

// --- validate ---
console.log('\nvalidate — 变量验证');
const v1 = replacer.validate('{name}', { name: 'ok' });
assert(v1.valid === true && v1.missing.length === 0, '所有变量存在 → valid=true');
const v2 = replacer.validate('{a} {b}', { a: 1 });
assert(v2.valid === false && JSON.stringify(v2.missing) === JSON.stringify(['b']), 'b缺失 → valid=false');

// --- 递归替换 ---
console.log('\nreplace() — 递归替换');
const recReplacer = new VariableReplacer({ recursive: true });
const recCtx = { name: 'Alice', greeting: 'Hello {name}' };
assertEq(recReplacer.replace('{greeting}', recCtx), 'Hello Alice', '递归展开');

// --- setContext / getContext / clearContext ---
console.log('\nsetContext / getContext / clearContext');
const ctxReplacer = new VariableReplacer();
ctxReplacer.setContext({ color: 'red' });
assertEq(ctxReplacer.getContext(), { color: 'red' }, 'setContext 设置');
ctxReplacer.clearContext();
assertEq(ctxReplacer.getContext(), {}, 'clearContext 清空');

// --- 便捷函数 ---
console.log('\n便捷函数');
assertEq(replaceVariables('{x}', { x: 100 }), '100', 'replaceVariables');
assertEq(replaceVariablesBatch(['{a}', '{b}'], { a: 1, b: 2 }), ['1', '2'], 'replaceVariablesBatch');
assertEq(extractVariables('{x} {y}'), ['x', 'y'], 'extractVariables');

// ======== Summary ========
console.log('\n---');
console.log(`通过: ${passed}, 失败: ${failed}, 总计: ${passed + failed}`);
if (failed > 0) process.exit(1);

