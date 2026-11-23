/**
 * Error codes that mean "the local speech runtime is not usable yet".
 *
 * A retry for these must re-run the one-click install (and therefore show the
 * preparation panel again), not merely refresh the page data.
 */
const SPEECH_INSTALL_CODES: ReadonlySet<string> = new Set([
  'TTS_ENVIRONMENT_INCOMPLETE',
  'TTS_SERVICE_SCRIPT_MISSING',
  'TTS_SERVICE_PYTHON_MISSING',
  'TTS_SERVICE_START_FAILED',
  'TTS_DOWNLOAD_UNAVAILABLE',
  'TTS_SYNTHESIS_FAILED_ENGINE',
  'TTS_INVALID_CONFIG',
  'TTS_TIMEOUT',
  'TTS_MODEL_LOADING',
  'qwen3_tts_unreachable'
]);

/** True when retrying should reinstall/restart the speech runtime. */
export function retriesSpeechInstall(code: string | null | undefined): boolean {
  return typeof code === 'string' && SPEECH_INSTALL_CODES.has(code);
}
