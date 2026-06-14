/**
 * Test: SensitiveWordsFilter (TDD)
 * 用法: npx tsx tests/sensitive-words.test.ts
 */
import {
  SensitiveWordsFilter,
  filterSensitiveWords,
  detectSensitiveWordsBatch,
  filterSensitiveWordsBatch
} from '../src/renderer/modules/utils/sensitive-words';

let p = 0, f = 0;
function ok(c: boolean, l: string) { if (c) p++; else f++; console.log('  ' + (c ? '✓' : '✗ FAIL') + ': ' + l); }
function eq(a: any, e: any, l: string) { const m = JSON.stringify(a) === JSON.stringify(e); if (m) p++; else f++; console.log('  ' + (m ? '✓' : '✗ FAIL') + ': ' + l + (m ? '' : '\n    exp: ' + JSON.stringify(e) + '\n    got: ' + JSON.stringify(a))); }

console.log('=== SensitiveWordsFilter ===\n');
const sf = new SensitiveWordsFilter({ caseSensitive: true });

console.log('addWord/hasWord/count');
sf.addWord('敏感');
ok(sf.hasWord('敏感'), 'hasWord true');
eq(sf.getWordCount(), 1, 'count=1');
sf.addWords(['A', 'B']);
eq(sf.getWordCount(), 3, 'count=3');

console.log('\ndetect');
const d1 = sf.detect('含敏感文本');
ok(d1.matched, 'matched');
ok(d1.words.includes('敏感'), 'words含敏感');
ok(!sf.detect('safe').matched, 'safe无匹配');
ok(sf.detect('敏感A B').words.length >= 2, '多word匹配');

console.log('\nfilter');
eq(sf.filter('含敏感'), '含**', 'replace**');
eq(sf.filter('safe'), 'safe', 'safe不变');
eq(filterSensitiveWords('xAy', ['A']), 'x*y', 'filterSensitiveWords');

console.log('\nremove');
sf.removeWord('敏感');
ok(!sf.hasWord('敏感'), 'removed');
eq(sf.getWordCount(), 2, 'count-1');
sf.removeWords(['A', 'B']);
eq(sf.getWordCount(), 0, 'count=0');

console.log('\nclear');
sf.addWords(['X', 'Y']);
sf.clear();
eq(sf.getWordCount(), 0, 'clear=0');

console.log('\ncase');
const ci = new SensitiveWordsFilter({ caseSensitive: false });
ci.addWord('abc');
ok(ci.detect('ABC').matched, 'caseInsensitive');

console.log('\nbatch');
eq(detectSensitiveWordsBatch(['A1', 'B1'], ['A', 'B']).map((r: any) => r.matched), [true, true], 'batchDetect');
eq(filterSensitiveWordsBatch(['A1', 'C1'], ['A']), ['*1', 'C1'], 'batchFilter');

console.log('\n---\n通过: ' + p + ', 失败: ' + f + ', 总计: ' + (p + f));
if (f > 0) process.exit(1);
