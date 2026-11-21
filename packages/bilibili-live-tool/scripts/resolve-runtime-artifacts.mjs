import { createHash } from 'node:crypto';
import { mkdir, writeFile } from 'node:fs/promises';
import path from 'node:path';
import { pathToFileURL } from 'node:url';

const FFMPEG_REPOSITORY = 'BtbN/FFmpeg-Builds';

const FFMPEG_PACKAGES = [
  ['windows', 'x86_64', 'ffmpeg-master-latest-win64-lgpl-shared.zip', 'zip'],
  ['windows', 'aarch64', 'ffmpeg-master-latest-winarm64-lgpl-shared.zip', 'zip'],
  ['linux', 'x86_64', 'ffmpeg-master-latest-linux64-lgpl-shared.tar.xz', 'tar.xz'],
  ['linux', 'aarch64', 'ffmpeg-master-latest-linuxarm64-lgpl-shared.tar.xz', 'tar.xz']
];

export function resolveRuntimeArtifacts({ ffmpegRelease, generatedAt }) {
  if (
    typeof generatedAt !== 'string' ||
    Number.isNaN(Date.parse(generatedAt)) ||
    new Date(generatedAt).toISOString() !== generatedAt
  ) {
    throw new Error('generatedAt must be an ISO timestamp');
  }
  const artifacts = resolveReleaseAssets({
    artifactId: 'ffmpeg',
    release: ffmpegRelease,
    repository: FFMPEG_REPOSITORY,
    packages: FFMPEG_PACKAGES,
    license: 'LGPL-2.1-or-later',
    sourceUrl: 'https://ffmpeg.org/download.html'
  });
  assertUniqueTargets(artifacts);

  return {
    schemaVersion: 1,
    generatedAt,
    artifacts
  };
}

function resolveReleaseAssets({ artifactId, release, repository, packages, license, sourceUrl }) {
  const assets = new Map();
  for (const asset of release?.assets ?? []) {
    if (assets.has(asset.name)) {
      throw new Error(`Duplicate release asset: ${asset.name}`);
    }
    assets.set(asset.name, asset);
  }
  return packages.flatMap(([os, arch, name, archiveKind]) => {
    const asset = assets.get(name);
    if (!asset) return [];

    const expectedPrefix = `https://github.com/${repository}/releases/download/`;
    if (!asset.browser_download_url?.startsWith(expectedPrefix)) {
      throw new Error(`${name} is not hosted by the allowlisted repository ${repository}`);
    }
    if (!/^sha256:[a-fA-F0-9]{64}$/.test(asset.digest ?? '')) {
      throw new Error(`${name} is missing a valid GitHub SHA-256 digest`);
    }
    if (!Number.isSafeInteger(asset.size) || asset.size <= 0) {
      throw new Error(`${name} has an invalid asset size`);
    }
    if (typeof release.tag_name !== 'string' || release.tag_name.trim() === '') {
      throw new Error(`${name} is missing its upstream release version`);
    }

    return [
      {
        artifactId,
        version: release.tag_name,
        os,
        arch,
        fileName: name,
        selectedEntry: name,
        archiveKind,
        url: asset.browser_download_url,
        sha256: asset.digest.slice('sha256:'.length).toLowerCase(),
        sizeBytes: asset.size,
        license,
        sourceUrl
      }
    ];
  });
}

function assertUniqueTargets(artifacts) {
  const targets = new Set();
  for (const artifact of artifacts) {
    const key = `${artifact.artifactId}:${artifact.os}:${artifact.arch}`;
    if (targets.has(key)) {
      throw new Error(`Duplicate runtime artifact target: ${key}`);
    }
    targets.add(key);
  }
}

function parseArgs(argv) {
  const result = {};
  for (let index = 0; index < argv.length; index += 1) {
    const key = argv[index];
    if (!key.startsWith('--') || !argv[index + 1]) {
      throw new Error(`Invalid argument: ${key}`);
    }
    result[key.slice(2)] = argv[index + 1];
    index += 1;
  }
  return result;
}

async function githubJson(apiPath) {
  const headers = {
    Accept: 'application/vnd.github+json',
    'User-Agent': 'panel-release-pipeline',
    'X-GitHub-Api-Version': '2022-11-28'
  };
  const githubToken = process.env.GITHUB_TOKEN ?? process.env.GH_TOKEN;
  if (githubToken) {
    headers.Authorization = `Bearer ${githubToken}`;
  }
  const response = await fetch(`https://api.github.com${apiPath}`, { headers });
  if (!response.ok) {
    throw new Error(`GitHub API request failed (${response.status}): ${apiPath}`);
  }
  return response.json();
}

async function downloadAndVerify(entry, directory) {
  const response = await fetch(entry.url, {
    headers: { 'User-Agent': 'panel-release-pipeline' }
  });
  if (!response.ok) {
    throw new Error(`Download failed (${response.status}): ${entry.url}`);
  }
  const bytes = Buffer.from(await response.arrayBuffer());
  const actual = createHash('sha256').update(bytes).digest('hex');
  if (actual !== entry.sha256) {
    throw new Error(`SHA-256 mismatch for ${entry.fileName}`);
  }
  await mkdir(directory, { recursive: true });
  await writeFile(path.join(directory, entry.fileName), bytes);
}

async function main() {
  const args = parseArgs(process.argv.slice(2));
  if (!args.output) throw new Error('--output is required');

  const ffmpegRelease = await githubJson(`/repos/${FFMPEG_REPOSITORY}/releases/latest`);
  const catalog = resolveRuntimeArtifacts({
    ffmpegRelease,
    generatedAt: new Date().toISOString()
  });
  await mkdir(path.dirname(args.output), { recursive: true });
  await writeFile(args.output, `${JSON.stringify(catalog, null, 2)}\n`, 'utf8');

  if (args['download-dir']) {
    for (const entry of catalog.artifacts) {
      await downloadAndVerify(entry, args['download-dir']);
    }
  }
}

if (process.argv[1] && import.meta.url === pathToFileURL(process.argv[1]).href) {
  main().catch(error => {
    console.error(error instanceof Error ? error.message : error);
    process.exitCode = 1;
  });
}
