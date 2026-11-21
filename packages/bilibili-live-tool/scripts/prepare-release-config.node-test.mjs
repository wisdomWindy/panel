import assert from 'node:assert/strict';
import test from 'node:test';

import { buildReleaseConfig, defaultUpdaterEndpoint } from './prepare-release-config.mjs';

test('buildReleaseConfig adds signed updater settings without leaking signing secrets', () => {
  const config = buildReleaseConfig({
    baseConfig: {
      productName: 'biliLiveHelper',
      build: { beforeBuildCommand: 'pnpm build:web:free' }
    },
    channel: 'free',
    publicKey: 'RWQ-public-key',
    endpoint: defaultUpdaterEndpoint('free')
  });

  assert.equal(config.productName, 'biliLiveHelper');
  assert.equal(config.bundle.createUpdaterArtifacts, true);
  assert.deepEqual(config.plugins.updater, {
    pubkey: 'RWQ-public-key',
    endpoints: ['https://raw.githubusercontent.com/wisdomWindy/panel/main/free/latest.json'],
    windows: { installMode: 'passive' }
  });
  assert.equal(JSON.stringify(config).includes('PRIVATE'), false);
});

test('buildReleaseConfig rejects unsupported channels and insecure endpoints', () => {
  assert.throws(
    () =>
      buildReleaseConfig({
        baseConfig: {},
        channel: 'preview',
        publicKey: 'public-key',
        endpoint: 'https://example.com/latest.json'
      }),
    /channel/i
  );

  assert.throws(
    () =>
      buildReleaseConfig({
        baseConfig: {},
        channel: 'member',
        publicKey: 'public-key',
        endpoint: 'http://example.com/latest.json'
      }),
    /https/i
  );
});

test('defaultUpdaterEndpoint keeps free and member manifests isolated', () => {
  assert.equal(
    defaultUpdaterEndpoint('free'),
    'https://raw.githubusercontent.com/wisdomWindy/panel/main/free/latest.json'
  );
  assert.equal(
    defaultUpdaterEndpoint('member'),
    'https://raw.githubusercontent.com/wisdomWindy/panel/main/member/latest.json'
  );
  assert.notEqual(defaultUpdaterEndpoint('free'), defaultUpdaterEndpoint('member'));
});
