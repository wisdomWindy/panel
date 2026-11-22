import type { LicenseDescriptor } from '@/platform/desktop';

/**
 * Third-party components and models this build relies on.
 *
 * The list is compiled into the app so the About page always shows credits,
 * even when the release metadata cannot be fetched. Model weights are listed
 * separately in spirit: they are downloaded by the operator, never bundled.
 */
export const THIRD_PARTY_NOTICES: readonly LicenseDescriptor[] = [
  {
    name: 'Qwen3-TTS（语音合成模型，权重由用户自行下载）',
    license: 'Apache-2.0',
    sourceUrl: 'https://github.com/QwenLM/Qwen3-TTS'
  },
  {
    name: 'FFmpeg（直播录制与片段截取，LGPL 预编译构建）',
    license: 'LGPL-2.1-or-later',
    sourceUrl: 'https://ffmpeg.org/legal.html'
  },
  { name: 'Tauri', license: 'MIT / Apache-2.0', sourceUrl: 'https://tauri.app' },
  { name: 'Vue 3', license: 'MIT', sourceUrl: 'https://vuejs.org' },
  { name: 'Vue Router', license: 'MIT', sourceUrl: 'https://router.vuejs.org' },
  { name: 'Pinia', license: 'MIT', sourceUrl: 'https://pinia.vuejs.org' },
  { name: 'Vue I18n', license: 'MIT', sourceUrl: 'https://vue-i18n.intlify.dev' },
  { name: 'Element Plus', license: 'MIT', sourceUrl: 'https://element-plus.org' },
  { name: 'Apache ECharts', license: 'Apache-2.0', sourceUrl: 'https://echarts.apache.org' },
  { name: 'sqlx', license: 'MIT / Apache-2.0', sourceUrl: 'https://github.com/launchbadge/sqlx' },
  { name: 'Tokio', license: 'MIT', sourceUrl: 'https://tokio.rs' },
  { name: 'reqwest', license: 'MIT / Apache-2.0', sourceUrl: 'https://github.com/seanmonstar/reqwest' },
  { name: 'serde', license: 'MIT / Apache-2.0', sourceUrl: 'https://serde.rs' },
  {
    name: 'encoding_rs',
    license: 'Apache-2.0 / MIT',
    sourceUrl: 'https://github.com/hsivonen/encoding_rs'
  },
  { name: 'notify-rust', license: 'MIT', sourceUrl: 'https://github.com/hoodie/notify-rust' },
  { name: 'rodio', license: 'MIT', sourceUrl: 'https://github.com/RustAudio/rodio' },
  {
    name: 'Microsoft Edge WebView2 Runtime',
    license: 'Microsoft 软件许可条款',
    sourceUrl: 'https://developer.microsoft.com/microsoft-edge/webview2/'
  }
];

/** Release-provided entries first, then the built-in credits, de-duplicated. */
export function mergeLicenses(provided: readonly LicenseDescriptor[]): LicenseDescriptor[] {
  const seen = new Set<string>();
  const merged: LicenseDescriptor[] = [];
  for (const entry of [...provided, ...THIRD_PARTY_NOTICES]) {
    const key = entry.name.trim().toLowerCase();
    if (!key || seen.has(key)) continue;
    seen.add(key);
    merged.push(entry);
  }
  return merged;
}
