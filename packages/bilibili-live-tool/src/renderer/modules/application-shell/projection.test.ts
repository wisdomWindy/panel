import { afterEach, describe, expect, it, vi } from 'vitest';
import type { UiPreferences } from '@/platform/desktop';
import { applyUiProjection } from './projection';

const ui = (theme: UiPreferences['theme'], accentColor = '#FB7299'): UiPreferences => ({
  theme,
  accentColor,
  opacity: 0.95,
  fontSize: 'medium',
  layoutMode: 'default',
  customLayout: null,
  showDanmakuLevel: true,
  showDanmakuMedal: true,
  showDanmakuTime: true,
  maxDanmakuEntries: 1000
});

function installDom(isSystemDark: boolean) {
  const properties = new Map<string, string>();
  const toggle = vi.fn();
  const root = {
    classList: { toggle },
    dataset: {} as Record<string, string>,
    style: {
      colorScheme: '',
      setProperty: (name: string, value: string) => properties.set(name, value)
    }
  };
  vi.stubGlobal('document', { documentElement: root });
  vi.stubGlobal('window', { matchMedia: () => ({ matches: isSystemDark }) });
  return { properties, root, toggle };
}

describe('application shell UI projection', () => {
  afterEach(() => vi.unstubAllGlobals());

  it.each([
    ['light', true, false],
    ['dark', false, true],
    ['system', true, true],
    ['system', false, false]
  ] as const)('resolves %s mode with system dark=%s', (theme, isSystemDark, expectedDark) => {
    const { root, toggle } = installDom(isSystemDark);

    applyUiProjection(ui(theme));

    expect(toggle).toHaveBeenCalledWith('dark', expectedDark);
    expect(root.style.colorScheme).toBe(expectedDark ? 'dark' : 'light');
    expect(root.dataset.theme).toBe(theme);
  });

  it('projects the bilibili accent through every Element Plus primary state', () => {
    const { properties } = installDom(false);

    applyUiProjection(ui('light'));

    expect(Object.fromEntries(properties)).toMatchObject({
      '--shell-accent': '#FB7299',
      '--el-color-primary': '#FB7299',
      '--el-color-primary-rgb': '251, 114, 153',
      '--el-color-primary-light-3': 'color-mix(in srgb, #FB7299 70%, white)',
      '--el-color-primary-light-5': 'color-mix(in srgb, #FB7299 50%, white)',
      '--el-color-primary-light-7': 'color-mix(in srgb, #FB7299 30%, white)',
      '--el-color-primary-light-8': 'color-mix(in srgb, #FB7299 20%, white)',
      '--el-color-primary-light-9': 'color-mix(in srgb, #FB7299 10%, white)',
      '--el-color-primary-dark-2': 'color-mix(in srgb, #FB7299 80%, black)'
    });
  });
});
