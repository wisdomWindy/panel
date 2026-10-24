import { writeFile } from 'node:fs/promises';
import { pathToFileURL } from 'node:url';

const RELEASE_ORIGIN = 'https://github.com';
const REPOSITORY = 'wisdomWindy/panel';
const ASSET_PREFIX = 'biliLiverHelper';

const DOWNLOADS = [
  { label: 'Windows x64', platform: 'windows-x64', extension: '.exe' },
  { label: 'macOS Apple Silicon', platform: 'macos-arm64', extension: '.dmg' },
  { label: 'macOS Intel', platform: 'macos-x64', extension: '.dmg' }
];

function normalizeVersion(version) {
  const normalized = String(version ?? '').replace(/^v/, '');
  if (!/^\d+\.\d+\.\d+(?:[-+][0-9A-Za-z.-]+)?$/.test(normalized)) {
    throw new Error(`Invalid release version: ${normalized || '<missing>'}`);
  }
  return normalized;
}

export function installerAssetName(version, platform, extension) {
  const normalizedVersion = normalizeVersion(version);
  if (
    !DOWNLOADS.some(download => download.platform === platform && download.extension === extension)
  ) {
    throw new Error(`Unsupported installer target: ${platform}${extension}`);
  }
  return `${ASSET_PREFIX}_${normalizedVersion}_${platform}${extension}`;
}

export function buildReleaseNotes(version, tag = `free-v${normalizeVersion(version)}`) {
  const normalizedVersion = normalizeVersion(version);
  const encodedTag = encodeURIComponent(tag);
  const lines = [`${ASSET_PREFIX} v${normalizedVersion}`, '', '## Downloads', ''];

  for (const download of DOWNLOADS) {
    const assetName = installerAssetName(normalizedVersion, download.platform, download.extension);
    const url = `${RELEASE_ORIGIN}/${REPOSITORY}/releases/download/${encodedTag}/${encodeURIComponent(assetName)}`;
    lines.push(`- [${download.label}](${url})`);
  }

  return `${lines.join('\n')}\n`;
}

function parseArgs(argv) {
  const result = {};
  for (let index = 0; index < argv.length; index += 2) {
    if (!argv[index]?.startsWith('--') || !argv[index + 1]) {
      throw new Error(`Invalid argument: ${argv[index] ?? '<missing>'}`);
    }
    result[argv[index].slice(2)] = argv[index + 1];
  }
  return result;
}

async function main() {
  const args = parseArgs(process.argv.slice(2));
  if (!args.version || !args.tag || !args.output) {
    throw new Error('--version, --tag and --output are required');
  }
  await writeFile(args.output, buildReleaseNotes(args.version, args.tag), 'utf8');
}

if (process.argv[1] && import.meta.url === pathToFileURL(process.argv[1]).href) {
  main().catch(error => {
    console.error(error instanceof Error ? error.message : error);
    process.exitCode = 1;
  });
}
