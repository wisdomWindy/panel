import assert from 'node:assert/strict';
import test from 'node:test';

import { resolveRuntimeArtifacts, selectWhisperRelease } from './resolve-runtime-artifacts.mjs';

const digest = `sha256:${'a'.repeat(64)}`;
const generatedAt = '2026-09-16T00:00:00.000Z';

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
    html_url: 'https://github.com/BtbN/FFmpeg-Builds/releases/tag/latest',
    assets: [
      asset('ffmpeg-master-latest-win64-lgpl-shared.zip', 'BtbN/FFmpeg-Builds'),
      asset('ffmpeg-master-latest-linux64-lgpl-shared.tar.xz', 'BtbN/FFmpeg-Builds')
    ]
  };
  const whisperRelease = {
    tag_name: 'b5130',
    html_url: 'https://github.com/ggml-org/whisper.cpp/releases/tag/b5130',
    assets: [
      asset('whisper-bin-x64.zip', 'ggml-org/whisper.cpp'),
      asset('whisper-bin-ubuntu-x64.tar.gz', 'ggml-org/whisper.cpp')
    ]
  };

  const catalog = resolveRuntimeArtifacts({ ffmpegRelease, whisperRelease, generatedAt });

  assert.equal(catalog.schemaVersion, 1);
  assert.equal(catalog.generatedAt, generatedAt);
  assert.deepEqual(
    catalog.artifacts.map(({ artifactId, os, arch }) => ({ artifactId, os, arch })),
    [
      { artifactId: 'ffmpeg', os: 'windows', arch: 'x86_64' },
      { artifactId: 'ffmpeg', os: 'linux', arch: 'x86_64' },
      { artifactId: 'whisper', os: 'windows', arch: 'x86_64' },
      { artifactId: 'whisper', os: 'linux', arch: 'x86_64' }
    ]
  );
  assert.ok(catalog.artifacts.every(entry => entry.sha256 === 'a'.repeat(64)));
  assert.ok(catalog.artifacts.every(entry => entry.url.startsWith('https://github.com/')));
  assert.equal(
    catalog.artifacts.some(entry => entry.os === 'darwin'),
    false
  );
});

test('selectWhisperRelease skips releases without usable official binaries', () => {
  const selected = selectWhisperRelease([
    { tag_name: 'v1.9.4', assets: [] },
    {
      tag_name: 'b5130',
      assets: [asset('whisper-bin-x64.zip', 'ggml-org/whisper.cpp')]
    }
  ]);

  assert.equal(selected.tag_name, 'b5130');
});

test('resolveRuntimeArtifacts rejects assets without GitHub SHA-256 digests', () => {
  const ffmpegRelease = {
    tag_name: 'latest',
    assets: [
      {
        ...asset('ffmpeg-master-latest-win64-lgpl-shared.zip', 'BtbN/FFmpeg-Builds'),
        digest: null
      }
    ]
  };

  assert.throws(
    () => resolveRuntimeArtifacts({ ffmpegRelease, whisperRelease: { assets: [] }, generatedAt }),
    /digest/i
  );
});

test('resolveRuntimeArtifacts rejects duplicate assets and non-positive sizes', () => {
  const named = asset('ffmpeg-master-latest-win64-lgpl-shared.zip', 'BtbN/FFmpeg-Builds');

  assert.throws(
    () =>
      resolveRuntimeArtifacts({
        ffmpegRelease: { tag_name: 'latest', assets: [named, named] },
        whisperRelease: { assets: [] },
        generatedAt
      }),
    /duplicate release asset/i
  );
  assert.throws(
    () =>
      resolveRuntimeArtifacts({
        ffmpegRelease: { tag_name: 'latest', assets: [{ ...named, size: 0 }] },
        whisperRelease: { assets: [] },
        generatedAt
      }),
    /asset size/i
  );
});
