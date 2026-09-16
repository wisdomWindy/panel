import { readFile, writeFile } from 'node:fs/promises';
import { pathToFileURL } from 'node:url';

const RELEASE_ORIGIN = 'https://github.com';
const RELEASE_PATH_PREFIX = '/wisdomWindy/panel/releases/download/';

export function finalizeUpdateManifest(manifest, expectedVersion) {
  const version = String(manifest.version ?? '').replace(/^v/, '');
  if (version !== expectedVersion.replace(/^v/, '')) {
    throw new Error(
      `Updater manifest version ${version || '<missing>'} does not match ${expectedVersion}`
    );
  }
  if (!manifest.platforms || Object.keys(manifest.platforms).length === 0) {
    throw new Error('Updater manifest has no platform artifacts');
  }
  for (const [target, artifact] of Object.entries(manifest.platforms)) {
    const url = new URL(artifact.url);
    if (url.protocol !== 'https:') {
      throw new Error(`Updater artifact ${target} must use HTTPS`);
    }
    if (url.origin !== RELEASE_ORIGIN || !url.pathname.startsWith(RELEASE_PATH_PREFIX)) {
      throw new Error(`Updater artifact ${target} is not hosted by the project GitHub release`);
    }
    if (typeof artifact.signature !== 'string' || artifact.signature.trim() === '') {
      throw new Error(`Updater artifact ${target} is missing its Tauri signature`);
    }
  }
  return {
    ...manifest,
    rolloutPercent: 100,
    isPaused: false,
    isForced: false,
    blacklistedVersions: []
  };
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
  if (!args.input || !args.version) {
    throw new Error('--input and --version are required');
  }
  const manifest = JSON.parse(await readFile(args.input, 'utf8'));
  const finalized = finalizeUpdateManifest(manifest, args.version);
  await writeFile(args.input, `${JSON.stringify(finalized, null, 2)}\n`, 'utf8');
}

if (process.argv[1] && import.meta.url === pathToFileURL(process.argv[1]).href) {
  main().catch(error => {
    console.error(error instanceof Error ? error.message : error);
    process.exitCode = 1;
  });
}
