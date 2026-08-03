import { strict as assert } from 'node:assert';
import test from 'node:test';

import {
  resolvePluginInstallTarget,
  toPluginDirectoryPath,
  createPluginRuntimeState
} from '../packages/bilibili-live-tool/src/renderer/plugins/runtime/helpers.ts';

test('resolvePluginInstallTarget uses manifest id as installed directory id', () => {
  const result = resolvePluginInstallTarget('/tmp/external-plugin', {
    id: 'hello-world'
  });

  assert.equal(result.sourcePath, '/tmp/external-plugin');
  assert.equal(result.pluginId, 'hello-world');
  assert.equal(result.targetDirectoryName, 'hello-world');
});

test('toPluginDirectoryPath joins plugin root and manifest id', () => {
  assert.equal(
    toPluginDirectoryPath('/plugins-root', 'hello-world'),
    '/plugins-root/hello-world'
  );
});

test('createPluginRuntimeState starts uninitialized with empty plugins', () => {
  const state = createPluginRuntimeState();

  assert.equal(state.initialized, false);
  assert.deepEqual(state.plugins, []);
});
