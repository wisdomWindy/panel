// One-shot migration: rename every backend error code to the PRD 14 taxonomy.
// Run with `node scripts/migrate-error-codes.mjs` from the package root.
import { readFileSync, writeFileSync, readdirSync, statSync } from 'node:fs';
import { join } from 'node:path';

/** old -> new. Only genuine error codes; generic words are excluded. */
const MAP = {
  // ROOM
  invalid_room_id: 'ROOM_INVALID_ID',
  audience_not_found: 'ROOM_AUDIENCE_NOT_FOUND',
  recording_room_invalid: 'ROOM_INVALID_ROOM',
  recording_room_offline: 'ROOM_OFFLINE',
  room_required: 'ROOM_REQUIRED',
  // NET
  external_service_unavailable: 'NET_SERVICE_UNAVAILABLE',
  llm_unavailable: 'NET_LLM_UNAVAILABLE',
  // RULE
  invalid_sensitive_word: 'RULE_INVALID_WORD',
  invalid_import_file: 'RULE_INVALID_IMPORT',
  invalid_user_uid: 'RULE_INVALID_USER',
  mute_duration_invalid: 'RULE_INVALID_MUTE_DURATION',
  // RECORD
  media_conflict: 'RECORD_CONFLICT',
  media_invalid: 'RECORD_INVALID_PARAM',
  media_operation_failed: 'RECORD_OPERATION_FAILED',
  media_unavailable: 'RECORD_UNAVAILABLE',
  recording_pipeline_invalid: 'RECORD_PIPELINE_INVALID',
  recording_start_failed: 'RECORD_START_FAILED',
  recording_start_interrupted: 'RECORD_START_INTERRUPTED',
  recording_finalize_failed: 'RECORD_FINALIZE_FAILED',
  recording_stream_lost: 'RECORD_STREAM_LOST',
  recording_stream_unavailable: 'RECORD_STREAM_UNAVAILABLE',
  recording_ffmpeg_missing: 'RECORD_FFMPEG_MISSING',
  open_live_websocket_unavailable: 'SYS_OPEN_LIVE_WS_UNAVAILABLE',
  // TTS
  qwen3_tts_invalid_config: 'TTS_INVALID_CONFIG',
  qwen3_tts_loading: 'TTS_MODEL_LOADING',
  qwen3_tts_timeout: 'TTS_TIMEOUT',
  ai_tts_disabled: 'TTS_DISABLED',
  ai_tts_failed: 'TTS_SYNTHESIS_FAILED',
  ai_tts_request_invalid: 'TTS_REQUEST_INVALID',
  ai_tts_unavailable: 'TTS_UNAVAILABLE',
  ai_tts_voice_invalid: 'TTS_VOICE_INVALID',
  // AUDIO
  ai_audio_device_invalid: 'AUDIO_DEVICE_INVALID',
  ai_audio_device_missing: 'AUDIO_DEVICE_MISSING',
  ai_audio_device_unavailable: 'AUDIO_DEVICE_UNAVAILABLE',
  ai_audio_route_unconfirmed: 'AUDIO_ROUTE_UNCONFIRMED',
  // AI_MODEL
  ai_api_key_invalid: 'AI_MODEL_API_KEY_INVALID',
  ai_api_key_required: 'AI_MODEL_API_KEY_REQUIRED',
  ai_config_invalid: 'AI_MODEL_CONFIG_INVALID',
  ai_model_not_configured: 'AI_MODEL_NOT_CONFIGURED',
  ai_model_profile_invalid: 'AI_MODEL_PROFILE_INVALID',
  ai_model_profile_not_found: 'AI_MODEL_PROFILE_NOT_FOUND',
  ai_model_profile_unavailable: 'AI_MODEL_PROFILE_UNAVAILABLE',
  ai_provider_invalid_response: 'AI_MODEL_PROVIDER_RESPONSE_INVALID',
  ai_provider_unavailable: 'AI_MODEL_PROVIDER_UNAVAILABLE',
  ai_secret_store_unavailable: 'AI_MODEL_SECRET_STORE_UNAVAILABLE',
  // AI_GEN
  ai_data_disclosure_required: 'AI_GEN_DATA_DISCLOSURE_REQUIRED',
  ai_generation_not_found: 'AI_GEN_NOT_FOUND',
  ai_generation_pending: 'AI_GEN_PENDING',
  ai_input_required: 'AI_GEN_INPUT_REQUIRED',
  ai_rate_limited: 'AI_GEN_RATE_LIMITED',
  ai_sensitive_output: 'AI_GEN_SENSITIVE_OUTPUT',
  ai_timeout: 'AI_GEN_TIMEOUT',
  ai_unmapped_error: 'AI_GEN_UNMAPPED_ERROR',
  ai_voice_rules_invalid: 'AI_GEN_VOICE_RULES_INVALID',
  // CONFIG
  invalid_config_key: 'CONFIG_INVALID_KEY',
  invalid_config_file: 'CONFIG_INVALID_FILE',
  config_export_failed: 'CONFIG_EXPORT_FAILED',
  config_transaction_failed: 'CONFIG_TRANSACTION_FAILED',
  config_directory_unavailable: 'CONFIG_DIRECTORY_UNAVAILABLE',
  // STORAGE
  storage_unavailable: 'STORAGE_UNAVAILABLE',
  ai_storage_unavailable: 'STORAGE_AI_UNAVAILABLE',
  member_storage_unavailable: 'STORAGE_MEMBER_UNAVAILABLE',
  data_maintenance_failed: 'STORAGE_MAINTENANCE_FAILED',
  no_export_data: 'STORAGE_NO_EXPORT_DATA',
  path_authorization_failed: 'STORAGE_PATH_DENIED',
  // UPDATE
  update_candidate_mismatch: 'UPDATE_CANDIDATE_MISMATCH',
  update_candidate_unavailable: 'UPDATE_CANDIDATE_UNAVAILABLE',
  update_check_failed: 'UPDATE_CHECK_FAILED',
  update_download_failed: 'UPDATE_DOWNLOAD_FAILED',
  update_install_failed: 'UPDATE_INSTALL_FAILED',
  update_not_verified: 'UPDATE_NOT_VERIFIED',
  update_provider_unavailable: 'UPDATE_PROVIDER_UNAVAILABLE',
  invalid_update_manifest: 'UPDATE_INVALID_MANIFEST',
  invalid_update_version: 'UPDATE_INVALID_VERSION',
  stale_update_check: 'UPDATE_STALE_CHECK',
  stale_update_download: 'UPDATE_STALE_DOWNLOAD',
  // MEMBER
  auth_account_invalid: 'MEMBER_AUTH_ACCOUNT_INVALID',
  auth_account_unavailable: 'MEMBER_AUTH_ACCOUNT_UNAVAILABLE',
  auth_qr_unavailable: 'MEMBER_AUTH_QR_UNAVAILABLE',
  auth_session_invalid: 'MEMBER_AUTH_SESSION_INVALID',
  auth_uid_invalid: 'MEMBER_AUTH_UID_INVALID',
  entitlement_key_invalid: 'MEMBER_ENTITLEMENT_KEY_INVALID',
  entitlement_signature_invalid: 'MEMBER_ENTITLEMENT_SIGNATURE_INVALID',
  member_data_invalid: 'MEMBER_DATA_INVALID',
  member_log_range_invalid: 'MEMBER_LOG_RANGE_INVALID',
  membership_order_invalid: 'MEMBER_ORDER_INVALID',
  membership_plan_invalid: 'MEMBER_PLAN_INVALID',
  module_entry_invalid: 'MEMBER_MODULE_ENTRY_INVALID',
  module_hash_invalid: 'MEMBER_MODULE_HASH_INVALID',
  module_hash_mismatch: 'MEMBER_MODULE_HASH_MISMATCH',
  module_id_invalid: 'MEMBER_MODULE_ID_INVALID',
  module_manifest_unavailable: 'MEMBER_MODULE_MANIFEST_UNAVAILABLE',
  module_path_invalid: 'MEMBER_MODULE_PATH_INVALID',
  module_progress_invalid: 'MEMBER_MODULE_PROGRESS_INVALID',
  moderation_alert_not_found: 'MEMBER_MODERATION_ALERT_NOT_FOUND',
  moderation_log_invalid: 'MEMBER_MODERATION_LOG_INVALID',
  moderation_log_unavailable: 'MEMBER_MODERATION_LOG_UNAVAILABLE',
  moderation_risk_required: 'MEMBER_MODERATION_RISK_REQUIRED',
  // SYS
  media_permission_denied: 'SYS_PERMISSION_DENIED',
  event_publish_failed: 'SYS_EVENT_PUBLISH_FAILED',
  shell_event_failed: 'SYS_SHELL_EVENT_FAILED',
  system_metric_unavailable: 'SYS_METRIC_UNAVAILABLE',
  dialog_cancelled: 'SYS_DIALOG_CANCELLED',
  stale_failure: 'SYS_STALE_FAILURE',
  missing_open_live_credentials: 'SYS_OPEN_LIVE_CREDENTIALS_MISSING',
  open_live_auth_failed: 'SYS_OPEN_LIVE_AUTH_FAILED',
  open_live_network_error: 'SYS_OPEN_LIVE_NETWORK_ERROR',
  reconnect_failed: 'SYS_RECONNECT_FAILED',
  unexpected: 'SYS_UNEXPECTED',
  transport_unavailable: 'NET_TRANSPORT_UNAVAILABLE',
  media_failed: 'RECORD_FAILED',
  recording_directory_required: 'RECORD_DIRECTORY_REQUIRED',
  recording_directory_unavailable: 'RECORD_DIRECTORY_UNAVAILABLE',
  tts_unavailable: 'TTS_UNAVAILABLE_ENGINE',
  ai_model_storage_unavailable: 'STORAGE_AI_MODEL_UNAVAILABLE',
  ai_voice_storage_unavailable: 'STORAGE_AI_VOICE_UNAVAILABLE',
  ai_voice_config_unavailable: 'AI_MODEL_VOICE_CONFIG_UNAVAILABLE',
  entitlement_online_verification_required: 'MEMBER_ENTITLEMENT_ONLINE_REQUIRED',
  entitlement_clock_invalid: 'MEMBER_ENTITLEMENT_CLOCK_INVALID',
  entitlement_verifier_unavailable: 'MEMBER_ENTITLEMENT_VERIFIER_UNAVAILABLE',
  module_cleanup_unavailable: 'MEMBER_MODULE_CLEANUP_UNAVAILABLE',
  moderation_capability_unavailable: 'MEMBER_MODERATION_CAPABILITY_UNAVAILABLE',
  moderation_api_unavailable: 'MEMBER_MODERATION_API_UNAVAILABLE',
  auth_uid_lookup_unavailable: 'MEMBER_AUTH_UID_LOOKUP_UNAVAILABLE',
  membership_service_unavailable: 'MEMBER_SERVICE_UNAVAILABLE',
  open_live_request_encoding_failed: 'SYS_OPEN_LIVE_ENCODING_FAILED',
  open_live_http_error: 'SYS_OPEN_LIVE_HTTP_ERROR',
  open_live_api_error: 'SYS_OPEN_LIVE_API_ERROR',
  file_read_failed: 'STORAGE_READ_FAILED',
  file_write_failed: 'STORAGE_WRITE_FAILED',
  file_permission_denied: 'STORAGE_PERMISSION_DENIED',
  qwen3_tts_synthesis_failed: 'TTS_SYNTHESIS_FAILED_ENGINE',
  qwen3_tts_download_unavailable: 'TTS_DOWNLOAD_UNAVAILABLE',
  tts_download_unavailable: 'TTS_DOWNLOAD_UNAVAILABLE_ENGINE',
  config_import_failed: 'CONFIG_IMPORT_FAILED',
  historical_data_unavailable: 'STORAGE_HISTORICAL_UNAVAILABLE',
  ranking_state_unavailable: 'STORAGE_RANKING_STATE_UNAVAILABLE'
};

const roots = ['src-tauri/src', 'src/renderer'];
const extensions = ['.rs', '.ts', '.vue', '.json'];

function walk(dir, out = []) {
  for (const name of readdirSync(dir)) {
    const path = join(dir, name);
    if (statSync(path).isDirectory()) {
      if (name === 'node_modules' || name === 'target') continue;
      walk(path, out);
    } else if (extensions.some(extension => name.endsWith(extension))) {
      out.push(path);
    }
  }
  return out;
}

// Longest first so a shorter name never eats a longer one.
const entries = Object.entries(MAP).sort((a, b) => b[0].length - a[0].length);
let changedFiles = 0;
let replacements = 0;

for (const file of roots.flatMap(root => walk(root))) {
  let content = readFileSync(file, 'utf8');
  const before = content;
  for (const [from, to] of entries) {
    const quoted = new RegExp(`"${from}"`, 'g');
    const single = new RegExp(`'${from}'`, 'g');
    content = content
      .replace(quoted, `"${to}"`)
      .replace(single, `'${to}'`);
  }
  if (content !== before) {
    writeFileSync(file, content);
    changedFiles += 1;
    replacements += entries.reduce(
      (total, [from]) => total + (before.match(new RegExp(`["']${from}["']`, 'g'))?.length ?? 0),
      0
    );
  }
}

console.log(`files=${changedFiles} replacements=${replacements} codes=${entries.length}`);
