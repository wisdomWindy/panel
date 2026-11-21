/**
 * PRD 13 platform matrix: Windows is the only fully verified target, so the
 * app must not claim full support elsewhere — the About page names the current
 * platform and flags the unverified one.
 */
export type HostPlatform = 'windows' | 'macos' | 'linux' | 'unknown';

/** Pure so the mapping is assertable without a browser. */
export function detectHostPlatform(userAgent: string): HostPlatform {
  const value = userAgent.toLowerCase();
  if (value.includes('windows')) return 'windows';
  if (value.includes('mac os') || value.includes('macintosh')) return 'macos';
  if (value.includes('linux') || value.includes('x11')) return 'linux';
  return 'unknown';
}

/** Windows is the first-class target; everything else carries a warning. */
export function isFullyVerifiedPlatform(platform: HostPlatform): boolean {
  return platform === 'windows';
}
