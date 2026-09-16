import { readFileSync } from 'node:fs';
import { resolve } from 'node:path';
import { describe, expect, it } from 'vitest';

const source = (path: string) => readFileSync(resolve(process.cwd(), path), 'utf8');

describe('membership page contract', () => {
  it('keeps native effects in the store and exposes the approved member states', () => {
    const page = source('src/renderer/views/membership/index.vue');
    expect(page).not.toMatch(/invokeDesktop|listenDesktop|localStorage|electronAPI/);
    expect(page).toContain('store.status.shellState === \'loggedOut\'');
    expect(page).toContain('store.status.shellState === \'expired\'');
    expect(page).toContain('store.status.shellState === \'unavailable\'');
    expect(page).toContain('会员功能已暂停');
    expect(page).toContain('已下载模块保留');
  });

  it('uses explicit login, module, order, and plan interaction boundaries', () => {
    const page = source('src/renderer/views/membership/index.vue');
    for (const component of [
      'MemberLoginDialog',
      'MembershipModuleDialog',
      'MembershipOrderDialog',
      'MembershipPlanGrid'
    ]) {
      expect(page).toContain(component);
    }
    const login = source('src/renderer/views/membership/components/MemberLoginDialog.vue');
    expect(login).toContain('/^[0-9]+$/');
    expect(login).toContain('确认绑定');
    expect(login).toContain('扫码登录暂不可用');
  });

  it('does not fabricate plan amounts, payment success, or module installation', () => {
    const files = [
      source('src/renderer/views/membership/index.vue'),
      source('src/renderer/views/membership/components/MembershipPlanGrid.vue'),
      source('src/renderer/views/membership/components/MembershipOrderDialog.vue')
    ].join('\n');
    expect(files).not.toMatch(/¥\s*\d|支付成功|充值成功|安装成功/);
    expect(files).toContain('会员服务暂不可用，请稍后重试');
  });
});
