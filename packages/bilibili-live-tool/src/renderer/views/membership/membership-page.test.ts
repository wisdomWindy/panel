import { readFileSync } from 'node:fs';
import { resolve } from 'node:path';
import { describe, expect, it } from 'vitest';

const source = (path: string) => readFileSync(resolve(process.cwd(), path), 'utf8');
const templateOf = (page: string) => page.slice(page.indexOf('<template>'));

const zh = JSON.parse(source('src/renderer/locales/zh-CN.json')) as {
  membership: {
    expiredNotice: string;
    serviceUnavailable: string;
    qr: Record<string, string>;
  };
};

const PAGES = {
  index: 'src/renderer/views/membership/index.vue',
  login: 'src/renderer/views/membership/components/MemberLoginDialog.vue',
  modules: 'src/renderer/views/membership/components/MembershipModuleDialog.vue',
  orders: 'src/renderer/views/membership/components/MembershipOrderDialog.vue',
  plans: 'src/renderer/views/membership/components/MembershipPlanGrid.vue',
  paused: 'src/renderer/components/member/MembershipPausedNotice.vue'
};

describe('membership page contract', () => {
  it('keeps native effects in the store and exposes the approved member states', () => {
    const page = source(PAGES.index);
    expect(page).not.toMatch(/invokeDesktop|listenDesktop|localStorage|electronAPI/);
    expect(page).toContain("store.status.shellState === 'loggedOut'");
    expect(page).toContain("store.status.shellState === 'expired'");
    expect(page).toContain("store.status.shellState === 'unavailable'");
    // The paused copy is translated; the shared resource keeps the approved wording.
    expect(zh.membership.expiredNotice).toContain('会员功能已暂停');
    expect(zh.membership.expiredNotice).toContain('已下载模块保留');
    expect(page).toContain("t('membership.expiredNotice')");
  });

  it('uses explicit login, module, order, and plan interaction boundaries', () => {
    const page = source(PAGES.index);
    for (const component of [
      'MemberLoginDialog',
      'MembershipModuleDialog',
      'MembershipOrderDialog',
      'MembershipPlanGrid'
    ]) {
      expect(page).toContain(component);
    }
    const login = source(PAGES.login);
    expect(login).toContain('/^[0-9]+$/');
    expect(zh.membership.qr.bind).toBe('确认绑定');
    expect(zh.membership.qr.unavailable).toBe('扫码登录暂不可用');
    expect(login).toContain("t('membership.qr.bind')");
    expect(login).toContain("t('membership.qr.unavailable')");
  });

  it('does not fabricate plan amounts, payment success, or module installation', () => {
    const files = [
      source(PAGES.index),
      source(PAGES.plans),
      source(PAGES.orders)
    ].join('\n');
    expect(files).not.toMatch(/¥\s*\d|支付成功|充值成功|安装成功/);
    expect(zh.membership.serviceUnavailable).toBe('会员服务暂不可用，请稍后重试');
    expect(files).toContain("t('membership.serviceUnavailable')");
  });

  it('renders every membership surface with tokens and without dark: literals', () => {
    const templates = Object.values(PAGES).map(path => templateOf(source(path)));

    for (const template of templates) {
      expect(template).toContain('app-card');
      expect(template).not.toMatch(
        /dark:|bg-white|text-gray-|border-gray-|divide-gray-|bg-zinc-|text-zinc-|border-zinc-/
      );
      // Copy is translated, never inlined in the template.
      expect(template).not.toMatch(/[\u4e00-\u9fff]/);
    }

    const page = templates[0];
    expect(page).toContain('app-text');
    expect(page).toContain('app-muted');
    expect(page).toContain('app-faint');
    expect(page).toContain('app-badge');
    expect(page).toContain('app-btn-primary');
    expect(page).toContain('app-chip');
  });
});
