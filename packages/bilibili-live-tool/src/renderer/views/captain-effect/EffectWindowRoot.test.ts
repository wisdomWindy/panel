import { createRenderer } from 'vue';
import { beforeEach, describe, expect, it, vi } from 'vitest';
import type { CaptainEffectPayload } from '@/platform/desktop';

interface TestNode {
  type: string;
  children: TestNode[];
  parent: TestNode | null;
  props: Record<string, unknown>;
  text?: string;
}

vi.mock('@/platform/desktop', async () => {
  const actual = await vi.importActual<typeof import('@/platform/desktop')>('@/platform/desktop');
  return { ...actual, invokeDesktop: vi.fn(), listenDesktop: vi.fn() };
});

import { invokeDesktop, listenDesktop } from '@/platform/desktop';
import EffectWindowRoot from './EffectWindowRoot.vue';

const previewPayload: CaptainEffectPayload = {
  eventId: 'preview-visible' as never,
  audience: {
    userUid: 'preview-user' as never,
    username: 'Preview User',
    userLevel: null,
    medal: null,
    tags: ['captain']
  },
  action: 'purchase',
  level: 'captain',
  config: {
    presetId: 'golden-premium',
    durationSeconds: 5,
    position: 'center',
    windowWidth: 560,
    windowHeight: 280,
    primaryColor: '#FFD700',
    secondaryColor: '#FFA500',
    firstSound: null,
    renewalSound: null,
    comboSound: null,
    volume: 0.7,
    comboEnabled: true,
    comboWindowSeconds: 10
  },
  isPreview: true
};

function createTestNode(type: string, text?: string): TestNode {
  return { type, children: [], parent: null, props: {}, text };
}

const renderer = createRenderer<TestNode, TestNode>({
  patchProp(node, key, _previous, next) {
    node.props[key] = next;
  },
  insert(child, parent, anchor) {
    child.parent = parent;
    const index = anchor ? parent.children.indexOf(anchor) : -1;
    if (index < 0) parent.children.push(child);
    else parent.children.splice(index, 0, child);
  },
  remove(child) {
    const parent = child.parent;
    if (!parent) return;
    parent.children = parent.children.filter(node => node !== child);
    child.parent = null;
  },
  createElement(type) {
    return createTestNode(type);
  },
  createText(text) {
    return createTestNode('text', text);
  },
  createComment(text) {
    return createTestNode('comment', text);
  },
  setText(node, text) {
    node.text = text;
  },
  setElementText(node, text) {
    node.text = text;
  },
  parentNode(node) {
    return node.parent;
  },
  nextSibling(node) {
    const parent = node.parent;
    if (!parent) return null;
    const index = parent.children.indexOf(node);
    return parent.children[index + 1] ?? null;
  }
});

describe('effect window startup', () => {
  beforeEach(() => {
    vi.mocked(invokeDesktop).mockReset();
    vi.mocked(listenDesktop).mockReset();
  });

  it('loads the active preview when event-listener registration is rejected', async () => {
    const calls: string[] = [];
    vi.mocked(listenDesktop).mockImplementation(async () => {
      calls.push('listen');
      throw new Error('event listen denied');
    });
    vi.mocked(invokeDesktop).mockImplementation(async name => {
      calls.push('current');
      if (name !== 'captain_effect_current') throw new Error(`unexpected command: ${name}`);
      return previewPayload;
    });

    const app = renderer.createApp(EffectWindowRoot);
    app.provide(Symbol.for('v-scx'), { modules: new Set<string>() });
    app.config.warnHandler = () => undefined;
    app.mount(createTestNode('root'));
    await vi.waitFor(() => expect(calls).toEqual(['listen', 'current']));
    app.unmount();
  });
});
