import { readFile, writeFile } from 'node:fs/promises';
import { pathToFileURL } from 'node:url';

const RELEASE_ORIGIN = 'https://github.com';
const REPOSITORY = 'wisdomWindy/panel';
const ASSET_PREFIX = 'biliLiverHelper';

const DOWNLOADS = [
  { label: 'Windows x64', platform: 'Windows_x64-setup', extension: '.exe' },
  {
    label: 'macOS Intel',
    platform: 'macOS_Intel',
    extension: '.dmg',
    requirement: 'macOS 10.15 or later'
  },
  {
    label: 'macOS Apple Silicon',
    platform: 'macOS_AppleSilicon',
    extension: '.dmg',
    requirement: 'macOS 10.15 or later'
  }
];

const SIGNING_NOTICE =
  'The Windows installer is not Authenticode-signed. The macOS disk images are not Apple Developer signed or notarized. Only bypass an operating-system warning after verifying that the file came from this repository.';
const FFMPEG_NOTICE =
  'FFmpeg licensing and source information for the supported media component is documented in [this repository](https://github.com/wisdomWindy/panel/blob/main/README.md#%E7%AC%AC%E4%B8%89%E6%96%B9%E8%BF%90%E8%A1%8C%E6%97%B6%E8%B5%84%E4%BA%A7).';

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

export function buildReleaseNotes(version, tag, changesMarkdown) {
  const normalizedVersion = normalizeVersion(version);
  const releaseTag = tag || `free-v${normalizedVersion}`;
  const changes = String(changesMarkdown ?? '').trim();
  if (!/^Changes:\s*\n\s*\n- /u.test(changes)) {
    throw new Error('Release changes must start with a non-empty Changes list');
  }
  const encodedTag = encodeURIComponent(releaseTag);
  const lines = [changes, '', 'Downloads:', ''];

  for (const download of DOWNLOADS) {
    const assetName = installerAssetName(normalizedVersion, download.platform, download.extension);
    const url = `${RELEASE_ORIGIN}/${REPOSITORY}/releases/download/${encodedTag}/${encodeURIComponent(assetName)}`;
    const requirement = download.requirement ? ` (${download.requirement})` : '';
    lines.push(`- ${download.label}: [\`${assetName}\`](${url})${requirement}`);
  }

  lines.push('', SIGNING_NOTICE, '', FFMPEG_NOTICE);
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
  if (!args.version || !args.tag || !args['changes-file'] || !args.output) {
    throw new Error('--version, --tag, --changes-file and --output are required');
  }
  const changesMarkdown = await readFile(args['changes-file'], 'utf8');
  await writeFile(args.output, buildReleaseNotes(args.version, args.tag, changesMarkdown), 'utf8');
}

if (process.argv[1] && import.meta.url === pathToFileURL(process.argv[1]).href) {
  main().catch(error => {
    console.error(error instanceof Error ? error.message : error);
    process.exitCode = 1;
  });
}
