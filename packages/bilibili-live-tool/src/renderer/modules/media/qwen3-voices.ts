/**
 * Speakers shipped with `Qwen3-TTS-12Hz-*-CustomVoice`, per the upstream README
 * ("Custom Voice Generate" speaker table).
 *
 * The service reports `/v1/voices` when it runs, but the operator may configure
 * speech before the service is reachable, so the same table is compiled in as a
 * fallback: an empty voice list reads as "this feature is broken".
 */
export interface Qwen3VoicePreset {
  readonly voiceId: string;
  readonly label: string;
  readonly language: string;
}

export const QWEN3_VOICE_PRESETS: readonly Qwen3VoicePreset[] = [
  { voiceId: 'Vivian', label: 'Vivian · 女声（中文）', language: 'Chinese' },
  { voiceId: 'Serena', label: 'Serena · 女声（中文）', language: 'Chinese' },
  { voiceId: 'Uncle_Fu', label: 'Uncle Fu · 男声（中文）', language: 'Chinese' },
  { voiceId: 'Dylan', label: 'Dylan · 男声（中文）', language: 'Chinese' },
  { voiceId: 'Eric', label: 'Eric · 男声（中文）', language: 'Chinese' },
  { voiceId: 'Ryan', label: 'Ryan · 男声（英文）', language: 'English' },
  { voiceId: 'Aiden', label: 'Aiden · 男声（英文）', language: 'English' },
  { voiceId: 'Ono_Anna', label: 'Ono Anna · 女声（日文）', language: 'Japanese' },
  { voiceId: 'Sohee', label: 'Sohee · 女声（韩文）', language: 'Korean' }
];

export interface Qwen3VoiceOption {
  readonly value: string;
  readonly label: string;
}

/**
 * Live service ids win (they can include re-downloaded checkpoints), and the
 * presets fill the gaps so the select is never empty.
 */
export function mergeVoiceOptions(
  serviceVoiceIds: readonly string[]
): Qwen3VoiceOption[] {
  const seen = new Set<string>();
  const options: Qwen3VoiceOption[] = [];
  for (const voiceId of serviceVoiceIds) {
    const trimmed = voiceId.trim();
    if (!trimmed || seen.has(trimmed)) continue;
    seen.add(trimmed);
    const preset = QWEN3_VOICE_PRESETS.find(entry => entry.voiceId === trimmed);
    options.push({ value: trimmed, label: preset?.label ?? trimmed });
  }
  for (const preset of QWEN3_VOICE_PRESETS) {
    if (seen.has(preset.voiceId)) continue;
    seen.add(preset.voiceId);
    options.push({ value: preset.voiceId, label: preset.label });
  }
  return options;
}
