import type { TtsEngine, TtsVoice } from '@/platform/desktop';

export function resolvePreferredVoiceId(
  engine: TtsEngine,
  selectedVoiceId: string | null,
  voices: readonly TtsVoice[]
): string | null {
  if (selectedVoiceId || engine !== 'system') return selectedVoiceId;

  return (
    voices.find(
      voice =>
        voice.engine === 'system' &&
        voice.isAvailable &&
        voice.displayName.toLocaleLowerCase().includes('microsoft huihui')
    )?.voiceId ?? null
  );
}
