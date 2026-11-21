use std::{path::PathBuf, sync::Arc};

use crate::domain::media::{
    CaptainEffectConfig, FfmpegSource, GiftNotifyConfig, GiftSoundPolicy, RecordingConfig,
    RecordingFormat, RecordingQuality, TtsConfig,
};

#[derive(Clone, Debug, Eq, PartialEq, thiserror::Error)]
#[error("{message}")]
pub struct MediaIoError {
    pub code: &'static str,
    pub message: String,
    pub retryable: bool,
}

impl MediaIoError {
    pub fn unavailable(message: impl Into<String>) -> Self {
        Self::new("media_unavailable", message, true)
    }

    pub fn invalid(message: impl Into<String>) -> Self {
        Self::new("media_invalid", message, false)
    }

    pub fn permission(message: impl Into<String>) -> Self {
        Self::new("media_permission_denied", message, false)
    }

    pub fn conflict(message: impl Into<String>) -> Self {
        Self::new("media_conflict", message, false)
    }

    pub fn operation(message: impl Into<String>, retryable: bool) -> Self {
        Self::new("media_operation_failed", message, retryable)
    }

    /// Builds an error with an explicit, adapter-specific code so callers can
    /// distinguish failures without string matching.
    pub fn coded(code: &'static str, message: impl Into<String>, retryable: bool) -> Self {
        Self::new(code, message, retryable)
    }

    fn new(code: &'static str, message: impl Into<String>, retryable: bool) -> Self {
        Self {
            code,
            message: message.into(),
            retryable,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct GiftSoundBindingRecord {
    pub gift_id: String,
    pub gift_name: String,
    pub sound_path: Option<PathBuf>,
    pub volume: f64,
    pub is_enabled: bool,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RecordingItemState {
    Recording,
    Completed,
    Partial,
    Failed,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RecordingRecord {
    pub id: i64,
    pub room_id: String,
    pub file_path: PathBuf,
    pub format: RecordingFormat,
    pub quality: RecordingQuality,
    pub started_at: i64,
    pub ended_at: Option<i64>,
    pub size_bytes: u64,
    pub duration_seconds: u64,
    pub status: RecordingItemState,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum HighlightTriggerType {
    DanmakuSurge,
    Manual,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct HighlightRecord {
    pub id: i64,
    pub recording_id: i64,
    pub room_id: String,
    pub position_seconds: u64,
    pub trigger_type: HighlightTriggerType,
    pub clip_duration_seconds: u64,
    pub file_path: Option<PathBuf>,
    pub created_at: i64,
    pub is_clip_available: bool,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct StagedMediaFile {
    pub original: PathBuf,
    pub staged: PathBuf,
}

#[async_trait::async_trait]
pub trait MediaRepository: Send + Sync {
    async fn load_captain_config(&self) -> Result<Option<CaptainEffectConfig>, MediaIoError>;
    async fn save_captain_config(&self, config: &CaptainEffectConfig) -> Result<(), MediaIoError>;
    async fn load_gift_notify_config(&self) -> Result<Option<GiftNotifyConfig>, MediaIoError>;
    async fn save_gift_notify_config(&self, config: &GiftNotifyConfig) -> Result<(), MediaIoError>;
    async fn load_gift_sound_policy(&self) -> Result<Option<GiftSoundPolicy>, MediaIoError>;
    async fn save_gift_sound_policy(&self, config: &GiftSoundPolicy) -> Result<(), MediaIoError>;
    async fn load_recording_config(&self) -> Result<Option<RecordingConfig>, MediaIoError>;
    async fn save_recording_config(&self, config: &RecordingConfig) -> Result<(), MediaIoError>;
    async fn load_tts_config(&self) -> Result<Option<TtsConfig>, MediaIoError>;
    async fn save_tts_config(&self, config: &TtsConfig) -> Result<(), MediaIoError>;
    async fn list_gift_sounds(&self) -> Result<Vec<GiftSoundBindingRecord>, MediaIoError>;
    async fn upsert_gift_sound(&self, binding: &GiftSoundBindingRecord)
        -> Result<(), MediaIoError>;
    async fn remove_gift_sound(&self, gift_id: &str) -> Result<bool, MediaIoError>;
    async fn replace_gift_sounds(
        &self,
        bindings: &[GiftSoundBindingRecord],
    ) -> Result<(), MediaIoError>;
    async fn insert_recording(
        &self,
        recording: &RecordingRecord,
    ) -> Result<RecordingRecord, MediaIoError>;
    async fn update_recording(&self, recording: &RecordingRecord) -> Result<bool, MediaIoError>;
    async fn list_recordings(&self) -> Result<Vec<RecordingRecord>, MediaIoError>;
    async fn delete_recording(&self, id: i64) -> Result<bool, MediaIoError>;
    async fn insert_highlight(
        &self,
        highlight: &HighlightRecord,
    ) -> Result<HighlightRecord, MediaIoError>;
    async fn list_highlights(
        &self,
        recording_id: i64,
    ) -> Result<Vec<HighlightRecord>, MediaIoError>;
    async fn delete_highlight(&self, id: i64) -> Result<bool, MediaIoError>;
}

#[async_trait::async_trait]
pub trait MediaFilePort: Send + Sync {
    fn resolve_gift_sound(&self, grant_id: &str) -> Result<PathBuf, MediaIoError>;
    async fn import_gift_sounds(
        &self,
        grant_id: &str,
    ) -> Result<Vec<GiftSoundBindingRecord>, MediaIoError>;
    async fn export_gift_sounds(
        &self,
        grant_id: &str,
        bindings: &[GiftSoundBindingRecord],
    ) -> Result<(), MediaIoError>;
    async fn move_file(
        &self,
        source: &std::path::Path,
        target: &std::path::Path,
    ) -> Result<(), MediaIoError>;
    async fn stage_delete(&self, paths: &[PathBuf]) -> Result<Vec<StagedMediaFile>, MediaIoError>;
    async fn restore_staged(&self, files: &[StagedMediaFile]) -> Result<(), MediaIoError>;
    async fn finalize_staged(&self, files: &[StagedMediaFile]) -> Result<(), MediaIoError>;
    async fn export_file(
        &self,
        source: &std::path::Path,
        target: &std::path::Path,
    ) -> Result<(), MediaIoError>;
    async fn file_size(&self, path: &std::path::Path) -> Result<u64, MediaIoError>;
}

#[derive(Clone, Debug, PartialEq)]
pub struct AudioRequest {
    pub source: PathBuf,
    pub volume: f64,
}

pub trait AudioPlayback: Send {
    fn stop(&self);
    fn is_finished(&self) -> bool;
}

#[async_trait::async_trait]
pub trait AudioOutputPort: Send + Sync {
    async fn play(&self, request: AudioRequest) -> Result<Box<dyn AudioPlayback>, MediaIoError>;
    async fn play_alert(&self, volume: f64) -> Result<Box<dyn AudioPlayback>, MediaIoError>;
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SpeechVoice {
    pub id: String,
    pub display_name: String,
}

#[derive(Clone, Debug, PartialEq)]
pub struct SpeechRequest {
    pub text: String,
    pub voice_id: Option<String>,
    pub rate: f64,
    pub volume: f64,
}

#[async_trait::async_trait]
pub trait SpeechEnginePort: Send + Sync {
    async fn voices(&self) -> Result<Vec<SpeechVoice>, MediaIoError>;
    async fn speak(&self, request: SpeechRequest) -> Result<(), MediaIoError>;
    async fn stop(&self) -> Result<(), MediaIoError>;
}

/// A request to synthesise speech into bytes, independent of playback.
#[derive(Clone, Debug, PartialEq)]
pub struct SynthesisRequest {
    pub text: String,
    pub voice_id: Option<String>,
    pub language: String,
    pub instruct: String,
    /// Multiplier around 1.0; adapters that cannot resample fold it into `instruct`.
    pub rate: f64,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SynthesisVoice {
    pub voice_id: String,
    pub display_name: String,
    pub language: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SynthesisHealth {
    pub ready: bool,
    pub engine: String,
    pub model: String,
    pub sample_rate: u32,
    pub error: Option<String>,
}

/// The frozen synthesis contract shared by the live-event 播报 path and the
/// AI voice-reply path. Any implementation (local service, remote API, stub)
/// can be swapped in without the callers knowing which model runs underneath.
#[async_trait::async_trait]
pub trait SpeechSynthesisPort: Send + Sync {
    async fn health(&self) -> Result<SynthesisHealth, MediaIoError>;
    async fn voices(&self) -> Result<Vec<SynthesisVoice>, MediaIoError>;
    /// Returns a complete RIFF/WAVE payload, mono 16-bit PCM.
    async fn synthesize_wav(&self, request: SynthesisRequest) -> Result<Vec<u8>, MediaIoError>;
}

/// A live stream location that FFmpeg can pull from.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ResolvedStream {
    pub url: String,
    /// Qualities the platform actually offers for this room, when reported.
    pub available_qualities: Vec<RecordingQuality>,
}

/// Turns a room id into a playable stream URL.
///
/// Kept as a port so the recorder never depends on a specific platform API,
/// and so tests can drive the whole start path without network access.
#[async_trait::async_trait]
pub trait PlayUrlResolver: Send + Sync {
    async fn resolve(
        &self,
        room_id: &str,
        quality: RecordingQuality,
    ) -> Result<ResolvedStream, MediaIoError>;
}

/// Locates the FFmpeg executable for the configured source.
pub trait FfmpegLocator: Send + Sync {
    fn locate(
        &self,
        source: FfmpegSource,
        granted_executable: Option<&std::path::Path>,
    ) -> Result<PathBuf, MediaIoError>;
}

/// Defaults used when no platform client is wired (tests, degraded start-up).
pub struct UnavailablePlayUrlResolver;

#[async_trait::async_trait]
impl PlayUrlResolver for UnavailablePlayUrlResolver {
    async fn resolve(
        &self,
        _: &str,
        _: RecordingQuality,
    ) -> Result<ResolvedStream, MediaIoError> {
        Err(MediaIoError::coded(
            "recording_stream_unavailable",
            "直播流解析不可用",
            false,
        ))
    }
}

pub struct UnavailableFfmpegLocator;

impl FfmpegLocator for UnavailableFfmpegLocator {
    fn locate(&self, _: FfmpegSource, _: Option<&std::path::Path>) -> Result<PathBuf, MediaIoError> {
        Err(MediaIoError::coded(
            "recording_ffmpeg_missing",
            "尚未配置 FFmpeg",
            false,
        ))
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RecorderLaunch {
    pub executable: PathBuf,
    pub arguments: Vec<String>,
    pub working_directory: Option<PathBuf>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RecorderExit {
    pub success: bool,
    pub code: Option<i32>,
    pub stdout: String,
    pub stderr: String,
}

#[async_trait::async_trait]
pub trait RecorderProcess: Send {
    async fn stop(&mut self) -> Result<(), MediaIoError>;
    async fn wait(&mut self) -> Result<RecorderExit, MediaIoError>;
}

#[async_trait::async_trait]
pub trait RecorderProcessPort: Send + Sync {
    async fn spawn(&self, launch: RecorderLaunch)
        -> Result<Box<dyn RecorderProcess>, MediaIoError>;
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct NotificationRequest {
    pub title: String,
    pub body: String,
    pub image_url: Option<String>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum NotificationPermission {
    Granted,
    Denied,
    Prompt,
}

#[async_trait::async_trait]
pub trait NotificationPort: Send + Sync {
    async fn permission(&self) -> Result<NotificationPermission, MediaIoError>;
    async fn request_permission(&self) -> Result<NotificationPermission, MediaIoError>;
    async fn show(&self, request: NotificationRequest) -> Result<(), MediaIoError>;
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EffectWindowSpec {
    pub label: String,
    pub width: u32,
    pub height: u32,
    pub position: crate::domain::media::MediaPosition,
}

#[async_trait::async_trait]
pub trait EffectWindowPort: Send + Sync {
    async fn ensure(&self, spec: &EffectWindowSpec) -> Result<(), MediaIoError>;
    async fn show(&self, label: &str) -> Result<(), MediaIoError>;
    async fn close(&self, label: &str) -> Result<(), MediaIoError>;
}

pub type HotkeyCallback = Arc<dyn Fn() + Send + Sync>;

pub trait HotkeyRegistration: Send {
    fn unregister(&mut self) -> Result<(), MediaIoError>;
}

pub trait HotkeyPort: Send + Sync {
    fn register(
        &self,
        shortcut: &str,
        callback: HotkeyCallback,
    ) -> Result<Box<dyn HotkeyRegistration>, MediaIoError>;
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PreviewSession {
    pub token: String,
    pub url: String,
    pub expires_at: i64,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ByteRange {
    pub start: u64,
    pub end_inclusive: u64,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PreviewRead {
    pub bytes: Vec<u8>,
    pub total_size: u64,
    pub range: ByteRange,
}

#[async_trait::async_trait]
pub trait PreviewPort: Send + Sync {
    fn issue(
        &self,
        file: PathBuf,
        owner: &str,
        now_ms: i64,
        ttl_ms: i64,
    ) -> Result<PreviewSession, MediaIoError>;
    async fn read(
        &self,
        token: &str,
        owner: &str,
        range: Option<ByteRange>,
        now_ms: i64,
    ) -> Result<PreviewRead, MediaIoError>;
    fn revoke(&self, token: &str, owner: &str) -> Result<bool, MediaIoError>;
}
