use serde::{Deserialize, Serialize};

use crate::{
    application::{
        bootstrap::{ApplicationStatus, BootstrapStatus},
        config::ConfigValue,
        paths::{PathGrantView, PathPurpose},
    },
    build_channel::BuildChannel,
};

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AppStatusDto {
    build_channel: BuildChannel,
    schema_version: u32,
    status: BootstrapStatus,
}

impl AppStatusDto {
    pub const fn new(
        build_channel: BuildChannel,
        schema_version: u32,
        status: BootstrapStatus,
    ) -> Self {
        Self {
            build_channel,
            schema_version,
            status,
        }
    }
}

impl From<ApplicationStatus> for AppStatusDto {
    fn from(status: ApplicationStatus) -> Self {
        Self::new(status.build_channel, status.schema_version, status.status)
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ConfigGetRequestDto {
    pub key: String,
    pub scope: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ConfigSetRequestDto {
    pub key: String,
    pub scope: String,
    pub value: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ConfigValueDto {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
}

impl From<Option<ConfigValue>> for ConfigValueDto {
    fn from(value: Option<ConfigValue>) -> Self {
        match value {
            Some(value) => Self {
                value: Some(value.value),
                updated_at: Some(value.updated_at),
            },
            None => Self {
                value: None,
                updated_at: None,
            },
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct ConfigRevisionDto {
    pub revision: String,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct PathSelectRequestDto {
    pub purpose: PathPurpose,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct PathRevokeRequestDto {
    pub grant_id: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PathGrantDto {
    pub grant_id: String,
    pub display_path: String,
}

impl From<PathGrantView> for PathGrantDto {
    fn from(grant: PathGrantView) -> Self {
        Self {
            grant_id: grant.grant_id,
            display_path: grant.display_path,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize)]
pub struct PathRevokeResultDto {
    pub revoked: bool,
}

pub const LIVE_DATA_COMMAND_NAMES: &[&str] = &[
    "room_connect",
    "room_disconnect",
    "audience_detail_query",
    "sensitive_word_list",
    "sensitive_word_add",
    "sensitive_word_update",
    "sensitive_word_delete",
    "sensitive_word_toggle",
    "sensitive_word_import",
    "sensitive_word_export",
    "blacklist_list",
    "blacklist_add",
    "blacklist_remove",
    "blacklist_export",
    "blacklist_import",
    "stat_query",
    "stat_refresh",
    "stat_export",
    "rank_query",
    "rank_export",
];

pub const LIVE_DATA_EVENT_NAMES: &[&str] = &[
    "room_status_changed",
    "danmu_received",
    "gift_received",
    "captain_received",
    "enter_received",
    "session_started",
    "session_ended",
    "super_chat_received",
    "super_chat_deleted",
    "sensitive_word_compile_progress",
    "stat_updated",
    "rank_updated",
    "export_done",
];

pub const MEDIA_COMMAND_NAMES: &[&str] = &[
    "captain_effect_get",
    "captain_effect_current",
    "captain_effect_save",
    "captain_effect_preview",
    "gift_notify_get",
    "gift_notify_save",
    "gift_notify_test",
    "gift_sound_list",
    "gift_sound_set",
    "gift_sound_remove",
    "gift_sound_save",
    "gift_sound_import",
    "gift_sound_export",
    "sound_preview",
    "tts_get",
    "tts_save",
    "tts_set_enabled",
    "tts_test",
    "tts_pause",
    "tts_clear",
    "recording_get_config",
    "recording_save_config",
    "recording_set_ffmpeg_source",
    "recording_status",
    "recording_start",
    "recording_stop",
    "recording_list",
    "recording_preview",
    "recording_rename",
    "recording_delete",
    "highlight_list",
    "highlight_mark",
    "highlight_preview",
    "highlight_export",
    "highlight_delete",
    "tts_qwen3_get",
    "tts_qwen3_save",
    "tts_qwen3_test",
    "tts_qwen3_models",
    "tts_qwen3_download_model",
];

pub const MEDIA_EVENT_NAMES: &[&str] = &[
    "captain_effect_trigger",
    "gift_alert_trigger",
    "gift_notify_updated",
    "tts_status_changed",
    "recording_status_changed",
    "recording_completed",
    "highlight_created",
];

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum ConnectionStatusDto {
    Disconnected,
    Connecting,
    Connected,
    Reconnecting,
    Error,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RoomSnapshotDto {
    pub status: ConnectionStatusDto,
    pub room_id: Option<String>,
    pub room_name: Option<String>,
    pub online_count: Option<u64>,
    pub reconnect_attempt: u8,
    pub last_error_code: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(tag = "kind", rename_all = "camelCase")]
pub enum DataRangeDto {
    LiveSession,
    Today,
    Last7Days,
    Custom {
        #[serde(rename = "startAt")]
        start_at: i64,
        #[serde(rename = "endAt")]
        end_at: i64,
    },
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum IdentityTagDto {
    Admin,
    Governor,
    Admiral,
    Captain,
    Vip,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FanMedalDto {
    pub name: String,
    pub level: Option<u32>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AudienceIdentityDto {
    pub user_uid: String,
    pub username: String,
    pub user_level: Option<u32>,
    pub medal: Option<FanMedalDto>,
    pub tags: Vec<IdentityTagDto>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DanmuEventDto {
    pub kind: String,
    pub event_id: String,
    pub room_id: String,
    pub occurred_at: i64,
    pub audience: AudienceIdentityDto,
    pub content: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GiftEventDto {
    pub kind: String,
    pub event_id: String,
    pub room_id: String,
    pub occurred_at: i64,
    pub audience: AudienceIdentityDto,
    pub gift_id: String,
    pub gift_name: String,
    pub gift_icon_url: Option<String>,
    pub quantity: u32,
    pub value_minor: i64,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum CaptainActionDto {
    Purchase,
    Renewal,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum CaptainLevelDto {
    Governor,
    Admiral,
    Captain,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CaptainEventDto {
    pub kind: String,
    pub event_id: String,
    pub room_id: String,
    pub occurred_at: i64,
    pub audience: AudienceIdentityDto,
    pub action: CaptainActionDto,
    pub level: CaptainLevelDto,
    pub value_minor: i64,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct EnterEventDto {
    pub kind: String,
    pub event_id: String,
    pub room_id: String,
    pub occurred_at: i64,
    pub audience: AudienceIdentityDto,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SuperChatEventDto {
    pub kind: String,
    pub event_id: String,
    pub room_id: String,
    pub occurred_at: i64,
    pub audience: AudienceIdentityDto,
    pub message_id: i64,
    pub content: String,
    pub value_minor: i64,
    pub duration_seconds: u64,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SuperChatDeletedEventDto {
    pub kind: String,
    pub event_id: String,
    pub room_id: String,
    pub occurred_at: i64,
    pub audience: AudienceIdentityDto,
    pub message_ids: Vec<i64>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(tag = "kind", rename_all = "camelCase")]
pub enum LiveEventDto {
    Danmu(DanmuEventBodyDto),
    Gift(GiftEventBodyDto),
    Captain(CaptainEventBodyDto),
    Enter(EnterEventBodyDto),
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LiveEventBaseDto {
    pub event_id: String,
    pub room_id: String,
    pub occurred_at: i64,
    pub audience: AudienceIdentityDto,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DanmuEventBodyDto {
    #[serde(flatten)]
    pub base: LiveEventBaseDto,
    pub content: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GiftEventBodyDto {
    #[serde(flatten)]
    pub base: LiveEventBaseDto,
    pub gift_id: String,
    pub gift_name: String,
    pub quantity: u32,
    pub value_minor: i64,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CaptainEventBodyDto {
    #[serde(flatten)]
    pub base: LiveEventBaseDto,
    pub action: CaptainActionDto,
    pub level: CaptainLevelDto,
    pub value_minor: i64,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct EnterEventBodyDto {
    #[serde(flatten)]
    pub base: LiveEventBaseDto,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum MediaPositionDto {
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
    Center,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SoundBindingSummaryDto {
    pub binding_id: String,
    pub file_name: String,
    pub is_missing: bool,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CaptainEffectConfigDto {
    pub preset_id: String,
    pub duration_seconds: u8,
    pub position: MediaPositionDto,
    pub window_width: u32,
    pub window_height: u32,
    pub primary_color: String,
    pub secondary_color: String,
    pub first_sound: Option<SoundBindingSummaryDto>,
    pub renewal_sound: Option<SoundBindingSummaryDto>,
    pub combo_sound: Option<SoundBindingSummaryDto>,
    pub volume: f64,
    pub combo_enabled: bool,
    pub combo_window_seconds: u8,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum TtsSelectedEngineDto {
    System,
    Cloud,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum ActiveTtsEngineDto {
    System,
    Cloud,
    LocalFallback,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum TtsEventKindDto {
    Danmu,
    Gift,
    Captain,
    Entry,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum TtsIdentityKindDto {
    Admin,
    Captain,
    Vip,
    Ordinary,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum TtsStateDto {
    Idle,
    Speaking,
    Paused,
    Degraded,
    Unavailable,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TtsTemplatesDto {
    pub danmu: String,
    pub gift: String,
    pub captain: String,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TtsConfigDto {
    pub is_enabled: bool,
    pub selected_engine: TtsSelectedEngineDto,
    pub selected_voice_id: Option<String>,
    pub rate: f64,
    pub volume: f64,
    pub event_kinds: Vec<TtsEventKindDto>,
    pub identity_kinds: Vec<TtsIdentityKindDto>,
    pub templates: TtsTemplatesDto,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TtsStatusDto {
    pub state: TtsStateDto,
    pub current_text: Option<String>,
    pub pending_count: u32,
    pub active_engine: ActiveTtsEngineDto,
    pub fallback_reason_code: Option<String>,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum RecordingFormatDto {
    Mp4,
    Flv,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum RecordingQualityDto {
    Original,
    High,
    Standard,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum FfmpegSourceDto {
    Bundled,
    Local,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum RecordingStateDto {
    Idle,
    Starting,
    Recording,
    Stopping,
    Finalizing,
    Error,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RecordingStatusDto {
    pub state: RecordingStateDto,
    pub session_id: Option<String>,
    pub room_id: Option<String>,
    pub elapsed_seconds: u64,
    pub file_size_bytes: u64,
    pub disk_remaining_bytes: u64,
    pub highlight_count: u32,
    pub last_error_code: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CaptainEffectPresetDto {
    pub preset_id: String,
    pub display_name: String,
    pub primary_color: String,
    pub secondary_color: String,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CaptainEffectSnapshotDto {
    pub config: CaptainEffectConfigDto,
    pub presets: Vec<CaptainEffectPresetDto>,
    pub updated_at: i64,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GiftNotifyMethodsDto {
    pub system_notification: bool,
    pub local_popup: bool,
    pub sound: bool,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GiftNotifyConfigDto {
    pub is_enabled: bool,
    pub high_value_threshold_minor: i64,
    pub methods: GiftNotifyMethodsDto,
    pub combo_enabled: bool,
    pub combo_window_seconds: u8,
    pub popup_position: MediaPositionDto,
    pub popup_duration_seconds: u32,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GiftNotifySnapshotDto {
    pub config: GiftNotifyConfigDto,
    pub total_value_minor: i64,
    pub donor_count: u32,
    pub gift_count: u64,
    pub latest_gifts: Vec<GiftEventDto>,
    pub updated_at: i64,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GiftSoundBindingDto {
    pub gift_id: String,
    pub gift_name: String,
    pub sound_file_name: Option<String>,
    pub volume: f64,
    pub is_enabled: bool,
    pub is_missing: bool,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GiftSoundSnapshotDto {
    pub bindings: Vec<GiftSoundBindingDto>,
    pub global_volume: f64,
    pub queue_capacity_per_second: u32,
    pub updated_at: i64,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TtsVoiceDto {
    pub voice_id: String,
    pub display_name: String,
    pub engine: TtsSelectedEngineDto,
    pub is_available: bool,
    pub unavailable_reason_code: Option<String>,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TtsSnapshotDto {
    pub config: TtsConfigDto,
    pub voices: Vec<TtsVoiceDto>,
    pub status: TtsStatusDto,
    pub updated_at: i64,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RecordingDirectoryDto {
    pub grant_id: String,
    pub display_path: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RecordingConfigDto {
    pub format: RecordingFormatDto,
    pub quality: RecordingQualityDto,
    pub directory: Option<RecordingDirectoryDto>,
    pub segmentation_enabled: bool,
    pub segment_duration_seconds: u32,
    pub auto_record: bool,
    pub danmaku_threshold: u32,
    pub highlight_before_seconds: u8,
    pub highlight_after_seconds: u8,
    pub quota_bytes: u64,
    pub waterline_bytes: u64,
    pub ffmpeg_source: FfmpegSourceDto,
    pub shortcut: String,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum RecordingItemStateDto {
    Recording,
    Completed,
    Partial,
    Failed,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RecordingSummaryDto {
    pub id: String,
    pub file_name: String,
    pub format: RecordingFormatDto,
    pub size_bytes: u64,
    pub duration_seconds: u64,
    pub started_at: i64,
    pub ended_at: Option<i64>,
    pub status: RecordingItemStateDto,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum HighlightTriggerTypeDto {
    DanmakuSurge,
    Manual,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct HighlightSummaryDto {
    pub id: String,
    pub recording_id: String,
    pub position_seconds: u64,
    pub trigger_type: HighlightTriggerTypeDto,
    pub clip_duration_seconds: u64,
    pub created_at: i64,
    pub is_clip_available: bool,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PreviewSessionDto {
    pub session_id: String,
    pub url: String,
    pub expires_at: i64,
}

#[derive(Clone, Debug, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct CaptainEffectConfigRequestDto {
    pub config: CaptainEffectConfigDto,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct GiftNotifyConfigRequestDto {
    pub config: GiftNotifyConfigDto,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct GiftSoundSetRequestDto {
    pub gift_id: String,
    pub gift_name: String,
    pub grant_id: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct GiftIdRequestDto {
    pub gift_id: String,
}

#[derive(Clone, Debug, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct GiftSoundSaveRequestDto {
    pub global_volume: f64,
    pub bindings: Vec<GiftSoundBindingDto>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
#[serde(tag = "kind", rename_all = "camelCase")]
pub enum SoundPreviewTargetDto {
    Binding { binding_id: String },
    Grant { grant_id: String },
}

#[derive(Clone, Debug, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct SoundPreviewRequestDto {
    pub target: SoundPreviewTargetDto,
    pub volume: f64,
}

#[derive(Clone, Debug, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct TtsConfigRequestDto {
    pub config: TtsConfigDto,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct TtsEnabledRequestDto {
    pub is_enabled: bool,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct TtsPauseRequestDto {
    pub is_paused: bool,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Qwen3TtsConfigDto {
    pub is_enabled: bool,
    pub base_url: String,
    pub voice_id: Option<String>,
    pub language: String,
    pub instruct: String,
    pub timeout_ms: u64,
    pub fallback_to_system: bool,
    pub ai_voice_id: Option<String>,
    pub ai_instruct: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Qwen3TtsHealthDto {
    pub ready: bool,
    pub engine: String,
    pub model: String,
    pub sample_rate: u32,
    pub error: Option<String>,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Qwen3TtsSnapshotDto {
    pub config: Qwen3TtsConfigDto,
    /// `None` when the service could not be reached at all.
    pub health: Option<Qwen3TtsHealthDto>,
    pub voice_ids: Vec<String>,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Qwen3ModelEntryDto {
    pub model_id: String,
    pub display_name: String,
    pub task: String,
    pub is_default: bool,
    pub is_installed: bool,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum Qwen3DownloadStatusDto {
    Idle,
    Downloading,
    Ready,
    Error,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Qwen3DownloadStateDto {
    pub model_id: Option<String>,
    pub status: Qwen3DownloadStatusDto,
    pub progress_percent: Option<u8>,
    pub bytes_downloaded: u64,
    pub bytes_total: Option<u64>,
    pub path: Option<String>,
    pub error: Option<String>,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Qwen3ModelCatalogDto {
    pub models: Vec<Qwen3ModelEntryDto>,
    pub download: Qwen3DownloadStateDto,
}

#[derive(Clone, Debug, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Qwen3DownloadRequestDto {
    pub model: Option<String>,
}

#[derive(Clone, Debug, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Qwen3TtsConfigRequestDto {
    pub config: Qwen3TtsConfigDto,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct RecordingConfigRequestDto {
    pub config: RecordingConfigDto,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct RecordingFfmpegSourceRequestDto {
    pub source: FfmpegSourceDto,
    pub grant_id: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct RecordingStartRequestDto {
    pub room_id: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct RecordingIdRequestDto {
    pub recording_id: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct RecordingRenameRequestDto {
    pub recording_id: String,
    pub file_name_stem: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct HighlightIdRequestDto {
    pub highlight_id: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct HighlightExportRequestDto {
    pub highlight_id: String,
    pub grant_id: String,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize)]
pub struct QueuedResultDto {
    pub queued: bool,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize)]
pub struct TriggeredResultDto {
    pub triggered: bool,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ImportedCountDto {
    pub imported_count: u32,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ExportedCountDto {
    pub exported_count: u32,
    pub display_path: String,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize)]
pub struct StartedResultDto {
    pub started: bool,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct ExportedResultDto {
    pub exported: bool,
    pub display_path: String,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CaptainEffectPayloadDto {
    pub event_id: String,
    pub audience: AudienceIdentityDto,
    pub action: CaptainActionDto,
    pub level: CaptainLevelDto,
    pub config: CaptainEffectConfigDto,
    pub is_preview: bool,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GiftAlertPayloadDto {
    pub event: GiftEventDto,
    pub popup_position: MediaPositionDto,
    pub popup_duration_seconds: u32,
    pub is_preview: bool,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum SensitiveWordCategoryDto {
    Illegal,
    Advertising,
    PersonalAttack,
    Custom,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum SensitiveMatchTypeDto {
    Exact,
    Fuzzy,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SensitiveWordDto {
    pub id: i64,
    pub word: String,
    pub category: SensitiveWordCategoryDto,
    pub match_type: SensitiveMatchTypeDto,
    pub is_enabled: bool,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SensitiveWordSnapshotDto {
    pub items: Vec<SensitiveWordDto>,
    pub today_hit_count: u64,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum SensitiveFileFormatDto {
    Csv,
    Json,
    Txt,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ImportErrorDto {
    pub line: u32,
    pub code: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ImportResultDto {
    pub imported_count: u32,
    pub duplicate_count: u32,
    pub error_count: u32,
    pub errors: Vec<ImportErrorDto>,
    pub is_rolled_back: bool,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BlacklistEntryDto {
    pub uid: String,
    pub username: String,
    pub reason: Option<String>,
    pub created_at: i64,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BlacklistSnapshotDto {
    pub items: Vec<BlacklistEntryDto>,
    pub today_added_count: u64,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AudienceDetailDto {
    pub identity: AudienceIdentityDto,
    pub session_danmu_count: u64,
    pub gift_value_minor: i64,
    pub is_blacklisted: bool,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct StatMetricsDto {
    pub online_count: u64,
    pub danmu_count: u64,
    pub danmu_rate: f64,
    pub gift_value_minor: i64,
    pub captain_count: u64,
    pub filtered_count: u64,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct StatTrendPointDto {
    pub occurred_at: i64,
    #[serde(flatten)]
    pub metrics: StatMetricsDto,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct StatSnapshotDto {
    pub range: DataRangeDto,
    pub metrics: StatMetricsDto,
    pub trend_points: Vec<StatTrendPointDto>,
    pub generated_at: i64,
    pub is_historical_data_available: bool,
    pub is_stale: bool,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum RankChangeDto {
    New,
    Up,
    Down,
    Same,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MoneyRankingEntryDto {
    pub rank: u32,
    pub user_uid: String,
    pub username: String,
    pub amount_minor: i64,
    pub previous_rank: Option<u32>,
    pub change: RankChangeDto,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DanmuRankingEntryDto {
    pub rank: u32,
    pub user_uid: String,
    pub username: String,
    pub danmu_count: u64,
    pub previous_rank: Option<u32>,
    pub change: RankChangeDto,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RankingSnapshotDto {
    pub range: DataRangeDto,
    pub gift: Vec<MoneyRankingEntryDto>,
    pub danmu: Vec<DanmuRankingEntryDto>,
    pub captain: Vec<MoneyRankingEntryDto>,
    pub gift_total_users: u32,
    pub danmu_total_users: u32,
    pub captain_total_users: u32,
    pub generated_at: i64,
    pub is_historical_data_available: bool,
    pub is_stale: bool,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum ExportTypeDto {
    SensitiveWords,
    Blacklist,
    Statistics,
    Rankings,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ExportResultDto {
    pub export_type: ExportTypeDto,
    pub display_path: String,
    pub exported_count: u32,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct RoomConnectRequestDto {
    pub identity_code: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct AudienceDetailQueryRequestDto {
    pub room_id: String,
    pub user_uid: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct SensitiveWordListRequestDto {
    pub category: Option<SensitiveWordCategoryDto>,
    pub search: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct SensitiveWordFormDto {
    pub word: String,
    pub category: SensitiveWordCategoryDto,
    pub match_type: SensitiveMatchTypeDto,
    pub is_enabled: bool,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct SensitiveWordUpdateRequestDto {
    pub id: i64,
    #[serde(flatten)]
    pub form: SensitiveWordFormDto,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct SensitiveWordIdRequestDto {
    pub id: i64,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct SensitiveWordToggleRequestDto {
    pub id: i64,
    pub is_enabled: bool,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct SensitiveWordFileRequestDto {
    pub grant_id: String,
    pub format: SensitiveFileFormatDto,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct BlacklistAddRequestDto {
    pub uid: String,
    pub username: String,
    pub reason: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct BlacklistUidRequestDto {
    pub uid: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct GrantRequestDto {
    pub grant_id: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct RangeRequestDto {
    pub range: DataRangeDto,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct RangeExportRequestDto {
    pub range: DataRangeDto,
    pub grant_id: String,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DeleteResultDto {
    pub deleted: bool,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RoomStatusChangedDto {
    pub status: ConnectionStatusDto,
    pub room_id: Option<String>,
    pub attempt: u8,
    pub error_code: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SensitiveWordCompileProgressDto {
    pub percentage: u8,
    pub stage: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ExportDoneDto {
    pub export_type: ExportTypeDto,
    pub display_path: String,
}
