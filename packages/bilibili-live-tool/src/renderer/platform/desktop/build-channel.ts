import type { BuildChannel } from './contracts';

export function resolveBuildChannel(value: unknown): BuildChannel {
  if (value === 'free' || value === 'member') {
    return value;
  }
  throw new Error('VITE_BUILD_CHANNEL must be either free or member');
}

export const BUILD_CHANNEL = resolveBuildChannel(import.meta.env.VITE_BUILD_CHANNEL);
export const IS_MEMBER_BUILD = BUILD_CHANNEL === 'member';
