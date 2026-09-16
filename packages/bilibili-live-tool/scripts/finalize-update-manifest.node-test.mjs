import assert from 'node:assert/strict';
import test from 'node:test';

import * as manifestModule from './finalize-update-manifest.mjs';

const { finalizeUpdateManifest } = manifestModule;

test('normalizeDraftAssetUrls converts draft API URLs to stable release URLs', () => {
  const manifest = manifestModule.normalizeDraftAssetUrls(
    {
      version: '1.2.3',
      platforms: {
        'windows-x86_64': {
          url: 'https://api.github.com/repos/wisdomWindy/panel/releases/assets/42',
          signature: 'signed'
        }
      }
    },
    {
      tagName: 'free-v1.2.3',
      assets: [
        {
          apiUrl: 'https://api.github.com/repos/wisdomWindy/panel/releases/assets/42',
          name: 'Bilibili Live Tool_1.2.3_x64-setup.exe'
        }
      ]
    }
  );

  assert.equal(
    manifest.platforms['windows-x86_64'].url,
    'https://github.com/wisdomWindy/panel/releases/download/free-v1.2.3/Bilibili%20Live%20Tool_1.2.3_x64-setup.exe'
  );
});

test('normalizeDraftAssetUrls rejects API assets missing from the release', () => {
  assert.throws(
    () =>
      manifestModule.normalizeDraftAssetUrls(
        {
          version: '1.2.3',
          platforms: {
            linux: {
              url: 'https://api.github.com/repos/wisdomWindy/panel/releases/assets/99',
              signature: 'signed'
            }
          }
        },
        { tagName: 'free-v1.2.3', assets: [] }
      ),
    /not present/
  );
});

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
