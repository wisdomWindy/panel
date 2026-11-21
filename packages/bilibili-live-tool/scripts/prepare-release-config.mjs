import { readFile, writeFile } from 'node:fs/promises';
import { pathToFileURL } from 'node:url';

const REPOSITORY = 'wisdomWindy/panel';
const CHANNELS = new Set(['free', 'member']);

export function defaultUpdaterEndpoint(channel) {
  assertChannel(channel);
  return `https://raw.githubusercontent.com/${REPOSITORY}/main/${channel}/latest.json`;
}

export function buildReleaseConfig({ baseConfig, channel, publicKey, endpoint }) {
  assertChannel(channel);
  if (typeof publicKey !== 'string' || publicKey.trim().length === 0) {
    throw new Error('TAURI_SIGNING_PUBLIC_KEY is required');
  }

  let updaterUrl;
  try {
    updaterUrl = new URL(endpoint);
  } catch {
    throw new Error('The updater endpoint must be a valid HTTPS URL');
  }
  if (updaterUrl.protocol !== 'https:') {
    throw new Error('The updater endpoint must use HTTPS');
  }

  return {
    ...baseConfig,
    bundle: {
      ...baseConfig.bundle,
      createUpdaterArtifacts: true
    },
    plugins: {
      ...baseConfig.plugins,
      updater: {
        ...baseConfig.plugins?.updater,
        pubkey: publicKey.trim(),
        endpoints: [updaterUrl.toString()],
        windows: {
          ...baseConfig.plugins?.updater?.windows,
          installMode: 'passive'
        }
      }
    }
  };
}

function assertChannel(channel) {
  if (!CHANNELS.has(channel)) {
    throw new Error(`Unsupported release channel: ${channel}`);
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

async function main() {
  const args = parseArgs(process.argv.slice(2));
  const channel = args.channel;
  assertChannel(channel);
  if (!args.output) {
    throw new Error('--output is required');
  }

  const basePath = new URL(`../src-tauri/tauri.${channel}.conf.json`, import.meta.url);
  const baseConfig = JSON.parse(await readFile(basePath, 'utf8'));
  const config = buildReleaseConfig({
    baseConfig,
    channel,
    publicKey: process.env.TAURI_SIGNING_PUBLIC_KEY,
    endpoint: process.env.TAURI_UPDATER_ENDPOINT ?? defaultUpdaterEndpoint(channel)
  });
  await writeFile(args.output, `${JSON.stringify(config, null, 2)}\n`, 'utf8');
}

if (process.argv[1] && import.meta.url === pathToFileURL(process.argv[1]).href) {
  main().catch(error => {
    console.error(error instanceof Error ? error.message : error);
    process.exitCode = 1;
  });
}
