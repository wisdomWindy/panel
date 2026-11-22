use std::{
    collections::{HashSet, VecDeque},
    path::PathBuf,
};

use serde::{Deserialize, Serialize};

pub const LATEST_GIFT_LIMIT: usize = 20;
pub const DANMAKU_HEAT_WINDOW_MS: i64 = 10_000;
pub const HIGHLIGHT_BEFORE_SECONDS: u64 = 30;
pub const HIGHLIGHT_AFTER_SECONDS: u64 = 30;
pub const DEFAULT_RECORDING_QUOTA_BYTES: u64 = 10 * 1024 * 1024 * 1024;
pub const DEFAULT_RECORDING_WATERLINE_BYTES: u64 = 5 * 1024 * 1024 * 1024;
pub const DEFAULT_CAPTAIN_EFFECT_WINDOW_WIDTH: u32 = 560;
pub const DEFAULT_CAPTAIN_EFFECT_WINDOW_HEIGHT: u32 = 280;

const fn default_captain_effect_window_width() -> u32 {
    DEFAULT_CAPTAIN_EFFECT_WINDOW_WIDTH
}

const fn default_captain_effect_window_height() -> u32 {
    DEFAULT_CAPTAIN_EFFECT_WINDOW_HEIGHT
}

#[derive(Clone, Debug, PartialEq, thiserror::Error)]
pub enum MediaDomainError {
    #[error("invalid media field: {0}")]
    InvalidField(&'static str),
    #[error("invalid media state transition")]
    InvalidTransition,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum MediaPosition {
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
    Center,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MediaSoundBinding {
    pub binding_id: String,
    pub file_name: String,
    pub source: PathBuf,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CaptainEffectConfig {
    pub preset_id: String,
    pub duration_seconds: u8,
    pub position: MediaPosition,
    #[serde(default = "default_captain_effect_window_width")]
    pub window_width: u32,
    #[serde(default = "default_captain_effect_window_height")]
    pub window_height: u32,
    pub primary_color: String,
    pub secondary_color: String,
    pub first_sound: Option<MediaSoundBinding>,
    pub renewal_sound: Option<MediaSoundBinding>,
    pub combo_sound: Option<MediaSoundBinding>,
    pub volume: f64,
    pub combo_enabled: bool,
    pub combo_window_seconds: u8,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GiftNotifyMethods {
    pub system_notification: bool,
    pub local_popup: bool,
    pub sound: bool,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GiftNotifyConfig {
    pub is_enabled: bool,
    pub high_value_threshold_minor: i64,
    pub methods: GiftNotifyMethods,
    pub combo_enabled: bool,
    pub combo_window_seconds: u8,
    pub popup_position: MediaPosition,
    pub popup_duration_seconds: u32,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct GiftRecord {
    pub event_id: String,
    pub room_id: String,
    pub user_uid: String,
    pub username: String,
    pub gift_id: String,
    pub gift_name: String,
    pub gift_icon_url: Option<String>,
    pub quantity: u32,
    pub value_minor: i64,
    pub occurred_at: i64,
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct GiftStats {
    pub total_value_minor: i64,
    pub donor_uids: HashSet<String>,
    pub gift_count: u64,
    pub latest: VecDeque<GiftRecord>,
    pub seen_event_ids: HashSet<String>,
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum TtsPriority {
    Ordinary,
    Admin,
    Gift,
    Captain,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TtsQueueItem {
    pub id: String,
    pub text: String,
    pub priority: TtsPriority,
    pub sequence: u64,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TtsTemplateKind {
    Danmu,
    Gift,
    Captain,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct GiftSoundQueueItem {
    pub event_id: String,
    pub is_high_value: bool,
    pub sequence: u64,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RecordingState {
    Idle,
    Starting,
    Recording,
    Stopping,
    Finalizing,
    Error,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct HighlightWindow {
    pub start_seconds: u64,
    pub end_seconds: u64,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GiftSoundPolicy {
    pub global_volume: f64,
    pub queue_capacity_per_second: u32,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum TtsSelectedEngine {
    System,
    Cloud,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum TtsEventKind {
    Danmu,
    Gift,
    Captain,
    Entry,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum TtsIdentityKind {
    Admin,
    Captain,
    Vip,
    Ordinary,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TtsTemplates {
    pub danmu: String,
    pub gift: String,
    pub captain: String,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TtsConfig {
    pub is_enabled: bool,
    pub selected_engine: TtsSelectedEngine,
    pub selected_voice_id: Option<String>,
    pub rate: f64,
    pub volume: f64,
    pub event_kinds: Vec<TtsEventKind>,
    pub identity_kinds: Vec<TtsIdentityKind>,
    pub templates: TtsTemplates,
}

pub const DEFAULT_QWEN3_TTS_BASE_URL: &str = "http://127.0.0.1:8020";
pub const DEFAULT_QWEN3_TTS_VOICE_ID: &str = "Serena";
pub const DEFAULT_QWEN3_TTS_LANGUAGE: &str = "Chinese";
pub const DEFAULT_QWEN3_TTS_TIMEOUT_MS: u64 = 8_000;
pub const MAX_QWEN3_TTS_INSTRUCT_CHARS: usize = 400;

/// Languages advertised by every released Qwen3-TTS-12Hz checkpoint.
pub const QWEN3_TTS_LANGUAGES: [&str; 11] = [
    "Auto", "Chinese", "English", "Japanese", "Korean", "German", "French", "Russian", "Portuguese",
    "Spanish", "Italian",
];

const fn default_qwen3_enabled() -> bool {
    false
}

fn default_qwen3_base_url() -> String {
    DEFAULT_QWEN3_TTS_BASE_URL.to_owned()
}

fn default_qwen3_language() -> String {
    DEFAULT_QWEN3_TTS_LANGUAGE.to_owned()
}

const fn default_qwen3_timeout_ms() -> u64 {
    DEFAULT_QWEN3_TTS_TIMEOUT_MS
}

const fn default_true() -> bool {
    true
}

/// Older records could store an explicit null, which serde(default) does not
/// cover; treat it as the empty instruction so a stale row cannot brick TTS.
fn string_or_empty_on_null<'de, D>(deserializer: D) -> Result<String, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let value = Option::<String>::deserialize(deserializer)?;
    Ok(value.unwrap_or_default())
}

/// Connection and voice settings for the Qwen3-TTS speech service.
///
/// The record is shared by the two speech paths (live-event 播报 and AI voice
/// reply) so a single service endpoint drives both. Every field carries a
/// serde default so an older or partial JSON record still deserializes.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Qwen3TtsConfig {
    #[serde(default = "default_qwen3_enabled")]
    pub is_enabled: bool,
    #[serde(default = "default_qwen3_base_url")]
    pub base_url: String,
    #[serde(default)]
    pub voice_id: Option<String>,
    #[serde(default = "default_qwen3_language")]
    pub language: String,
    #[serde(default, deserialize_with = "string_or_empty_on_null")]
    pub instruct: String,
    #[serde(default = "default_qwen3_timeout_ms")]
    pub timeout_ms: u64,
    /// When the service is unreachable or errors, fall back to the system engine.
    #[serde(default = "default_true")]
    pub fallback_to_system: bool,
    /// Speaker for the AI voice-reply path; `None` reuses `voice_id`.
    #[serde(default)]
    pub ai_voice_id: Option<String>,
    /// Instruction override for the AI voice-reply path; `None` reuses `instruct`.
    #[serde(default)]
    pub ai_instruct: Option<String>,
}

impl Default for Qwen3TtsConfig {
    fn default() -> Self {
        Self {
            is_enabled: default_qwen3_enabled(),
            base_url: default_qwen3_base_url(),
            voice_id: None,
            language: default_qwen3_language(),
            instruct: String::new(),
            timeout_ms: default_qwen3_timeout_ms(),
            fallback_to_system: true,
            ai_voice_id: None,
            ai_instruct: None,
        }
    }
}

impl Qwen3TtsConfig {
    /// Speaker used for live-event 播报.
    pub fn event_voice(&self) -> &str {
        self.voice_id
            .as_deref()
            .filter(|value| !value.trim().is_empty())
            .unwrap_or(DEFAULT_QWEN3_TTS_VOICE_ID)
    }

    /// Speaker used for AI voice replies.
    pub fn reply_voice(&self) -> &str {
        self.ai_voice_id
            .as_deref()
            .filter(|value| !value.trim().is_empty())
            .unwrap_or_else(|| self.event_voice())
    }

    /// Instruction used for AI voice replies.
    pub fn reply_instruct(&self) -> &str {
        self.ai_instruct.as_deref().unwrap_or(&self.instruct)
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum RecordingFormat {
    Mp4,
    Flv,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum RecordingQuality {
    Original,
    High,
    Standard,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum FfmpegSource {
    /// Shipped sidecar; falls back to a system install when absent.
    Bundled,
    /// Whatever FFmpeg the machine already has (PATH and the usual installers).
    System,
    /// A file the operator picked and authorized.
    Local,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RecordingDirectoryGrant {
    pub grant_id: String,
    pub display_path: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RecordingConfig {
    pub format: RecordingFormat,
    pub quality: RecordingQuality,
    pub directory: Option<RecordingDirectoryGrant>,
    pub segmentation_enabled: bool,
    pub segment_duration_seconds: u32,
    pub auto_record: bool,
    pub danmaku_threshold: u32,
    pub highlight_before_seconds: u8,
    pub highlight_after_seconds: u8,
    pub quota_bytes: u64,
    pub waterline_bytes: u64,
    pub ffmpeg_source: FfmpegSource,
    pub shortcut: String,
}
