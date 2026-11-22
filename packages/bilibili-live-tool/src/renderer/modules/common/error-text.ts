/**
 * Backend error codes carry an English fallback message; the UI must show a
 * localized sentence instead. Codes without a translation keep the backend
 * text, so nothing becomes less informative.
 */
const ERROR_KEYS: Readonly<Record<string, string>> = {
  RECORD_FFMPEG_MISSING: 'errors.RECORD_FFMPEG_MISSING',
  TTS_SERVICE_SCRIPT_MISSING: 'errors.TTS_SERVICE_SCRIPT_MISSING',
  TTS_SERVICE_PYTHON_MISSING: 'errors.TTS_SERVICE_PYTHON_MISSING',
  TTS_SERVICE_START_FAILED: 'errors.TTS_SERVICE_START_FAILED',
  RECORD_FFMPEG_DOWNLOAD_FAILED: 'errors.RECORD_FFMPEG_DOWNLOAD_FAILED',
  RECORD_FFMPEG_INSTALL_FAILED: 'errors.RECORD_FFMPEG_INSTALL_FAILED',
  RECORD_FFMPEG_UNSUPPORTED: 'errors.RECORD_FFMPEG_UNSUPPORTED',
  RECORD_DIRECTORY_REQUIRED: 'errors.RECORD_DIRECTORY_REQUIRED',
  RECORD_DIRECTORY_UNAVAILABLE: 'errors.RECORD_DIRECTORY_UNAVAILABLE',
  RECORD_ROOM_INVALID: 'errors.RECORD_ROOM_INVALID',
  RECORD_OFFLINE: 'errors.RECORD_OFFLINE',
  RECORD_CONFLICT: 'errors.RECORD_CONFLICT',
  RECORD_STREAM_UNAVAILABLE: 'errors.RECORD_STREAM_UNAVAILABLE',
  STORAGE_UNAVAILABLE: 'errors.STORAGE_UNAVAILABLE',
  STORAGE_PATH_DENIED: 'errors.STORAGE_PATH_DENIED',
  SYS_DIALOG_CANCELLED: 'errors.SYS_DIALOG_CANCELLED'
};

/** `null` means "no translation"; the caller keeps the backend message. */
export function errorKey(code: string | null | undefined): string | null {
  if (!code) return null;
  return ERROR_KEYS[code] ?? null;
}

/** The user dismissed a native dialog: that is never an error to report. */
export function isUserCancellation(code: string | null | undefined): boolean {
  return code === 'SYS_DIALOG_CANCELLED';
}
