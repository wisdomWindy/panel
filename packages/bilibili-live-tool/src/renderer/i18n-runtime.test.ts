import { describe, expect, it, vi } from 'vitest';
import { createI18n, type MessageCompilerContext, type MessageContext } from 'vue-i18n';
import enUS from './locales/en-US.json';
import zhCN from './locales/zh-CN.json';
import { cspSafeMessageCompiler } from './i18n-runtime';

const compilerContext: MessageCompilerContext = {
  key: 'shell.productName',
  locale: 'zh-CN'
};

function createMessageContext(values: Record<string, unknown> = {}): MessageContext {
  return {
    list: () => undefined,
    named: key => values[key],
    plural: messages => messages[0],
    linked: key => key,
    type: 'text',
    values
  };
}

function collectLocaleMessages(value: unknown): string[] {
  if (typeof value === 'string') return [value];
  if (value === null || typeof value !== 'object' || Array.isArray(value)) {
    throw new TypeError('Locale resources must contain only objects and string leaves');
  }

  return Object.values(value).flatMap(collectLocaleMessages);
}

function collectLocaleKeys(value: unknown, prefix = ''): string[] {
  if (typeof value === 'string') return [prefix];
  if (value === null || typeof value !== 'object' || Array.isArray(value)) {
    throw new TypeError('Locale resources must contain only objects and string leaves');
  }

  return Object.entries(value).flatMap(([key, child]) =>
    collectLocaleKeys(child, prefix ? `${prefix}.${key}` : key)
  );
}

describe('CSP-safe i18n runtime', () => {
  it('exports a message compiler instead of relying on dynamic code evaluation', async () => {
    const runtime = await import('./i18n-runtime');

    expect(runtime).toHaveProperty('cspSafeMessageCompiler');
    expect(typeof Reflect.get(runtime, 'cspSafeMessageCompiler')).toBe('function');
  });

  it('returns literal locale messages unchanged', () => {
    const message = cspSafeMessageCompiler('biliLiveHelper', compilerContext);

    expect(message(createMessageContext())).toBe('biliLiveHelper');
  });

  it('interpolates every named placeholder without evaluating code', () => {
    const message = cspSafeMessageCompiler('Sent {giftName} x{count}', {
      ...compilerContext,
      key: 'dashboard.giftMessage'
    });

    expect(message(createMessageContext({ giftName: 'Super Chat', count: 2 }))).toBe(
      'Sent Super Chat x2'
    );
  });

  it('reports unsupported precompiled message resources and falls back to the key', () => {
    const onError = vi.fn();
    const message = cspSafeMessageCompiler({} as never, {
      ...compilerContext,
      key: 'unsupported.message',
      onError
    });

    expect(message(createMessageContext())).toBe('unsupported.message');
    expect(onError).toHaveBeenCalledOnce();
  });

  it('translates with vue-i18n when dynamic Function construction is blocked', () => {
    const blockedFunction = vi.fn(() => {
      throw new EvalError('dynamic code evaluation is blocked');
    });
    const warn = vi.spyOn(console, 'warn').mockImplementation(() => undefined);
    vi.stubGlobal('Function', blockedFunction);

    try {
      const i18n = createI18n({
        legacy: false,
        locale: 'en-US',
        messageCompiler: cspSafeMessageCompiler,
        messages: { 'en-US': enUS }
      });

      expect(i18n.global.t('shell.productName')).toBe('liver');
      expect(i18n.global.t('dashboard.giftMessage', { giftName: 'Super Chat', count: 2 })).toBe(
        'Sent Super Chat x2'
      );
      expect(blockedFunction).not.toHaveBeenCalled();
    } finally {
      vi.unstubAllGlobals();
      warn.mockRestore();
    }
  });

  it('keeps locale resources within the supported literal and named-placeholder grammar', () => {
    for (const message of [...collectLocaleMessages(zhCN), ...collectLocaleMessages(enUS)]) {
      const withoutNamedPlaceholders = message.replace(/\{[A-Za-z][A-Za-z0-9_]*\}/g, '');

      expect(withoutNamedPlaceholders).not.toMatch(/[{}|]/);
      expect(withoutNamedPlaceholders).not.toContain('@:');
    }
  });

  it('keeps Chinese and English locale resources structurally complete', () => {
    expect(collectLocaleKeys(enUS).sort()).toEqual(collectLocaleKeys(zhCN).sort());
  });

  it('provides English copy for representative content outside the navigation shell', () => {
    const i18n = createI18n({
      legacy: false,
      locale: 'en-US',
      messageCompiler: cspSafeMessageCompiler,
      messages: { 'en-US': enUS }
    });

    expect(i18n.global.t('tts.title')).toBe('Voice Broadcast');
    expect(i18n.global.t('captainEffect.windowSize')).toBe('Effect window size');
    expect(i18n.global.t('rankings.title')).toBe('Rankings');
    expect(i18n.global.t('sensitiveWords.title')).toBe('Sensitive Words');
    expect(i18n.global.t('recording.settings.title')).toBe('Recording Settings');
  });
});
