import assert from 'node:assert/strict';
import test from 'node:test';

import { finalizeUpdateManifest } from './finalize-update-manifest.mjs';

test('finalizeUpdateManifest validates signatures and adds rollout policy', () => {
  const manifest = finalizeUpdateManifest(
    {
      version: 'v1.2.3',
      platforms: {
        'windows-x86_64': {
          url: 'https://github.com/wisdomWindy/panel/releases/download/v1/app.zip',
          signature: 'signed'
        }
      }
    },
    '1.2.3'
  );

  assert.equal(manifest.rolloutPercent, 100);
  assert.equal(manifest.isPaused, false);
  assert.deepEqual(manifest.blacklistedVersions, []);
});

test('finalizeUpdateManifest rejects unsigned, insecure, or mismatched releases', () => {
  assert.throws(
    () => finalizeUpdateManifest({ version: '2.0.0', platforms: {} }, '1.0.0'),
    /does not match/
  );
  assert.throws(
    () => finalizeUpdateManifest({ version: '1.0.0', platforms: {} }, '1.0.0'),
    /no platform/
  );
  assert.throws(
    () =>
      finalizeUpdateManifest(
        {
          version: '1.0.0',
          platforms: { linux: { url: 'http://example.com/app', signature: '' } }
        },
        '1.0.0'
      ),
    /HTTPS/
  );
  assert.throws(
    () =>
      finalizeUpdateManifest(
        {
          version: '1.0.0',
          platforms: {
            linux: {
              url: 'https://example.com/app.tar.gz',
              signature: 'signed'
            }
          }
        },
        '1.0.0'
      ),
    /project GitHub release/
  );
});
