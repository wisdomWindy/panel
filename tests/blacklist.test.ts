import { BlacklistManager } from '../src/renderer/modules/danmaku/blacklist';
import { BlacklistItem } from '../src/renderer/modules/danmaku/types';

let p = 0, f = 0;
function ok(c: boolean, l: string) { if (c) p++; else f++; console.log('  ' + (c ? '✓' : '✗ FAIL') + ': ' + l); }
function eq(a: any, e: any, l: string) { const m = JSON.stringify(a) === JSON.stringify(e); if (m) p++; else f++; console.log('  ' + (m ? '✓' : '✗ FAIL') + ': ' + l + (m ? '' : '\n    exp: ' + JSON.stringify(e) + '\n    got: ' + JSON.stringify(a))); }

(async () => {
  console.log('\nBlacklistManager tests\n');

  // --- addToBlacklist (permanent) ---
  {
    const bm = new BlacklistManager();
    bm.addToBlacklist('u1', 'user1', 'spam');
    ok(bm.isBlacklisted('u1'), 'permanent ban — isBlacklisted returns true');
    const items = bm.getBlacklist();
    eq(items.length, 1, 'permanent ban — getBlacklist has 1 item');
    eq(items[0].userId, 'u1', 'permanent ban — item userId');
    eq(items[0].username, 'user1', 'permanent ban — item username');
    eq(items[0].reason, 'spam', 'permanent ban — item reason');
    eq(items[0].bannedUntil, 0, 'permanent ban — bannedUntil is 0 (permanent)');
  }

  // --- addToBlacklist (with duration) ---
  {
    const bm = new BlacklistManager();
    const before = Date.now();
    bm.addToBlacklist('u2', 'user2', 'toxic', 60); // 60 minutes
    const items = bm.getBlacklist();
    eq(items.length, 1, 'timed ban — getBlacklist has 1 item');
    ok(items[0].bannedUntil > before + 60 * 60000 - 5000, 'timed ban — bannedUntil is ~60 min in future');
    ok(items[0].bannedUntil !== 0, 'timed ban — bannedUntil is not 0');
  }

  // --- isBlacklisted (non-existent user) ---
  {
    const bm = new BlacklistManager();
    ok(!bm.isBlacklisted('noone'), 'isBlacklisted returns false for unknown user');
  }

  // --- removeFromBlacklist ---
  {
    const bm = new BlacklistManager();
    bm.addToBlacklist('u3', 'user3', 'abuse');
    ok(bm.isBlacklisted('u3'), 'before remove — isBlacklisted returns true');
    bm.removeFromBlacklist('u3');
    ok(!bm.isBlacklisted('u3'), 'after remove — isBlacklisted returns false');
    eq(bm.getBlacklist().length, 0, 'after remove — getBlacklist is empty');
  }

  // --- removeFromBlacklist (non-existent) ---
  {
    const bm = new BlacklistManager();
    bm.removeFromBlacklist('ghost'); // should not throw
    ok(true, 'removeFromBlacklist on non-existent user does not throw');
  }

  // --- getBlacklist (multiple entries) ---
  {
    const bm = new BlacklistManager();
    bm.addToBlacklist('a', 'A', 'r1');
    bm.addToBlacklist('b', 'B', 'r2', 30);
    bm.addToBlacklist('c', 'C', 'r3');
    eq(bm.getBlacklist().length, 3, 'getBlacklist returns all 3 entries');
  }

  // --- clear ---
  {
    const bm = new BlacklistManager();
    bm.addToBlacklist('x', 'X', 'r');
    bm.addToBlacklist('y', 'Y', 'r', 5);
    bm.clear();
    eq(bm.getBlacklist().length, 0, 'clear empties blacklist');
    ok(!bm.isBlacklisted('x'), 'clear — isBlacklisted returns false');
    ok(!bm.isBlacklisted('y'), 'clear — isBlacklisted returns false');
  }

  // --- expiry (duration parameter in minutes) ---
  {
    const bm = new BlacklistManager();
    bm.addToBlacklist('exp', 'expUser', 'test expiry', 0.01); // 0.01 min = 600ms
    ok(bm.isBlacklisted('exp'), 'before expiry — isBlacklisted returns true');

    // isBlacklisted should remove after expiry
    await new Promise(r => setTimeout(r, 700));
    ok(!bm.isBlacklisted('exp'), 'after expiry — isBlacklisted returns false (auto-removed)');
    eq(bm.getBlacklist().length, 0, 'after expiry — getBlacklist is empty');
  }

  // --- getBlacklist cleans expired items ---
  {
    const bm = new BlacklistManager();
    bm.addToBlacklist('e1', 'E1', 'r', 0.01); // will expire
    bm.addToBlacklist('e2', 'E2', 'r');         // permanent
    await new Promise(r => setTimeout(r, 700));
    const items = bm.getBlacklist();
    eq(items.length, 1, 'getBlacklist cleans expired, keeps permanent');
    eq(items[0].userId, 'e2', 'getBlacklist keeps the permanent ban');
  }

  // --- EventEmitter: banned event ---
  {
    const bm = new BlacklistManager();
    let emitted: BlacklistItem | null = null;
    bm.on('banned', (item: BlacklistItem) => { emitted = item; });
    bm.addToBlacklist('ev1', 'eventUser', 'event reason', 10);
    ok(emitted !== null, 'banned event fired');
    eq(emitted!.userId, 'ev1', 'banned event — userId');
    eq(emitted!.reason, 'event reason', 'banned event — reason');
  }

  // --- EventEmitter: unbanned event ---
  {
    const bm = new BlacklistManager();
    let emitted: BlacklistItem | null = null;
    bm.on('unbanned', (item: BlacklistItem) => { emitted = item; });
    bm.addToBlacklist('ev2', 'eventUser2', 'reason2');
    bm.removeFromBlacklist('ev2');
    ok(emitted !== null, 'unbanned event fired');
    eq(emitted!.userId, 'ev2', 'unbanned event — userId');
  }

  // --- checkSpam ---
  {
    const bm = new BlacklistManager();
    // first message — not spam
    ok(!bm.checkSpam('spammer', 'hello'), 'checkSpam — first message not spam');
    // same message within interval — spam
    ok(bm.checkSpam('spammer', 'hello'), 'checkSpam — duplicate message is spam');
    // different message — not spam
    ok(!bm.checkSpam('spammer', 'world'), 'checkSpam — different message not spam');
    // same message after interval (wait > 3000ms)
    await new Promise(r => setTimeout(r, 3100));
    ok(!bm.checkSpam('spammer', 'hello'), 'checkSpam — same message after interval not spam');
  }

  // --- Summary ---
  console.log('\n' + p + ' passed, ' + f + ' failed, ' + (p + f) + ' total');
  if (f > 0) process.exit(1);
})();
