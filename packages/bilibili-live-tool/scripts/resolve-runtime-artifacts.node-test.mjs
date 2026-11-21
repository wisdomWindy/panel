import assert from 'node:assert/strict';
import test from 'node:test';

import { resolveRuntimeArtifacts } from './resolve-runtime-artifacts.mjs';

const digest = `sha256:${'a'.repeat(64)}`;
const generatedAt = '2026-09-16T00:00:00.000Z';
const FFMPEG_REPOSITORY = 'FFmpeg-Builds/FFmpeg-Builds';

function asset(name, repository) {
  return {
    name,
    browser_download_url: `https://github.com/${repository}/releases/download/v1/${name}`,
    digest,
    size: 42
  };
}

test('resolveRuntimeArtifacts emits only exact official platform packages', () => {
  const ffmpegRelease = {
    tag_name: 'latest',
    html_url: `https://github.com/${FFMPEG_REPOSITORY}/releases/tag/latest`,
    assets: [
      asset('ffmpeg-master-latest-win64-lgpl-shared.zip', FFMPEG_REPOSITORY),
      asset('ffmpeg-master-latest-linux64-lgpl-shared.tar.xz', FFMPEG_REPOSITORY)
    ]
  };
  const catalog = resolveRuntimeArtifacts({ ffmpegRelease, generatedAt });

  assert.equal(catalog.schemaVersion, 1);
  assert.equal(catalog.generatedAt, generatedAt);
  assert.deepEqual(
    catalog.artifacts.map(({ artifactId, os, arch }) => ({ artifactId, os, arch })),
    [
      { artifactId: 'ffmpeg', os: 'windows', arch: 'x86_64' },
      { artifactId: 'ffmpeg', os: 'linux', arch: 'x86_64' }
    ]
  );
  assert.ok(catalog.artifacts.every(entry => entry.sha256 === 'a'.repeat(64)));
  assert.ok(catalog.artifacts.every(entry => entry.url.startsWith('https://github.com/')));
  assert.equal(
    catalog.artifacts.some(entry => entry.os === 'darwin'),
    false
  );
});

test('resolveRuntimeArtifacts rejects assets without GitHub SHA-256 digests', () => {
  const ffmpegRelease = {
    tag_name: 'latest',
    assets: [
      {
        ...asset('ffmpeg-master-latest-win64-lgpl-shared.zip', FFMPEG_REPOSITORY),
        digest: null
      }
    ]
  };

  assert.throws(
    () => resolveRuntimeArtifacts({ ffmpegRelease, generatedAt }),
    /digest/i
  );
});

test('resolveRuntimeArtifacts rejects assets from a renamed or unknown host', () => {
  const ffmpegRelease = {
    tag_name: 'latest',
    assets: [
      asset('ffmpeg-master-latest-win64-lgpl-shared.zip', 'BtbN/FFmpeg-Builds')
    ]
  };

  assert.throws(
    () => resolveRuntimeArtifacts({ ffmpegRelease, generatedAt }),
    /not hosted by the allowlisted repository/i
  );
});

test('resolveRuntimeArtifacts rejects duplicate assets and non-positive sizes', () => {
  const named = asset('ffmpeg-master-latest-win64-lgpl-shared.zip', FFMPEG_REPOSITORY);

  assert.throws(
    () =>
      resolveRuntimeArtifacts({
        ffmpegRelease: { tag_name: 'latest', assets: [named, named] },
        generatedAt
      }),
    /duplicate release asset/i
  );
  assert.throws(
    () =>
      resolveRuntimeArtifacts({
        ffmpegRelease: { tag_name: 'latest', assets: [{ ...named, size: 0 }] },
        generatedAt
      }),
    /asset size/i
  );
});
