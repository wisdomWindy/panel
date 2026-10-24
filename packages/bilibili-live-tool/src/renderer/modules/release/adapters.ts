import type {
  AboutSnapshot,
  LicenseDescriptor,
  ResilienceSnapshot,
  UpdateCandidate,
  UpdateSnapshot
} from '@/platform/desktop';

function record(value: unknown, name: string): Record<string, unknown> {
  if (typeof value !== 'object' || value === null || Array.isArray(value)) {
    throw new TypeError(`${name} must be an object`);
  }
  return value as Record<string, unknown>;
}

function text(value: unknown, name: string): string {
  if (typeof value !== 'string' || value.length === 0) throw new TypeError(`${name} must be text`);
  return value;
}

function nullableText(value: unknown, name: string): string | null {
  return value === null ? null : text(value, name);
}

function candidate(value: unknown): UpdateCandidate {
  const source = record(value, 'updateCandidate');
  if (!Array.isArray(source.releaseNotes) || !Array.isArray(source.blacklistedVersions)) {
    throw new TypeError('update candidate lists are invalid');
  }
  if (
    typeof source.rolloutPercent !== 'number' ||
    !Number.isInteger(source.rolloutPercent) ||
    source.rolloutPercent < 0 ||
    source.rolloutPercent > 100 ||
    typeof source.isPaused !== 'boolean' ||
    typeof source.isForced !== 'boolean'
  ) {
    throw new TypeError('update candidate policy is invalid');
  }
  return Object.freeze({
    version: text(source.version, 'version'),
    releaseNotes: Object.freeze(source.releaseNotes.map(value => text(value, 'releaseNote'))),
    rolloutPercent: source.rolloutPercent,
    isPaused: source.isPaused,
    isForced: source.isForced,
    blacklistedVersions: Object.freeze(source.blacklistedVersions.map(value => text(value, 'blacklistedVersion')))
  });
}

export function adaptUpdateSnapshot(value: unknown): UpdateSnapshot {
  const source = record(value, 'updateSnapshot');
  if (!['free', 'member'].includes(source.channel as string)) throw new TypeError('channel is invalid');
  if (!['unavailable', 'current', 'available', 'downloading', 'downloaded', 'failed'].includes(source.status as string)) {
    throw new TypeError('update status is invalid');
  }
  const progress = source.progressPercent;
  if (progress !== null && (typeof progress !== 'number' || !Number.isInteger(progress) || progress < 0 || progress > 100)) {
    throw new TypeError('update progress is invalid');
  }
  return Object.freeze({
    currentVersion: text(source.currentVersion, 'currentVersion'),
    channel: source.channel as UpdateSnapshot['channel'],
    status: source.status as UpdateSnapshot['status'],
    candidate: source.candidate === null ? null : candidate(source.candidate),
    progressPercent: progress as number | null,
    errorCode: nullableText(source.errorCode, 'errorCode')
  });
}

function license(value: unknown): LicenseDescriptor {
  const source = record(value, 'license');
  return Object.freeze({
    name: text(source.name, 'licenseName'),
    license: text(source.license, 'licenseType'),
    sourceUrl: text(source.sourceUrl, 'licenseSourceUrl')
  });
}

export function adaptAboutSnapshot(value: unknown): AboutSnapshot {
  const source = record(value, 'aboutSnapshot');
  if (!['free', 'member'].includes(source.channel as string) || !Array.isArray(source.licenses)) {
    throw new TypeError('about snapshot is invalid');
  }
  return Object.freeze({
    applicationVersion: text(source.applicationVersion, 'applicationVersion'),
    channel: source.channel as AboutSnapshot['channel'],
    licenses: Object.freeze(source.licenses.map(license))
  });
}

export function adaptResilienceSnapshot(value: unknown): ResilienceSnapshot {
  const source = record(value, 'resilienceSnapshot');
  const count = source.migrationBackupCount;
  const backupAt = source.lastMigrationBackupAt;
  if (typeof count !== 'number' || !Number.isInteger(count) || count < 0) throw new TypeError('backup count is invalid');
  if (backupAt !== null && (typeof backupAt !== 'number' || backupAt < 0)) throw new TypeError('backup time is invalid');
  for (const key of ['retentionPolicyAvailable', 'ffmpegArtifactAvailable', 'whisperArtifactAvailable']) {
    if (typeof source[key] !== 'boolean') throw new TypeError(`${key} must be boolean`);
  }
  return Object.freeze({
    migrationBackupCount: count,
    lastMigrationBackupAt: backupAt as number | null,
    retentionPolicyAvailable: source.retentionPolicyAvailable as boolean,
    ffmpegArtifactAvailable: source.ffmpegArtifactAvailable as boolean,
    whisperArtifactAvailable: source.whisperArtifactAvailable as boolean
  });
}
