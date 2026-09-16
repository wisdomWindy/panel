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
    Bundled,
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
