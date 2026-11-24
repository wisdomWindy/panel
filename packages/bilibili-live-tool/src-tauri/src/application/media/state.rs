/// Reserved grant id for the app-owned `download` folder: recording must work
/// without the operator picking a directory first.
pub const APP_DOWNLOAD_GRANT_ID: &str = "app-download";

use std::{
    collections::{HashSet, VecDeque},
    path::{Path, PathBuf},
    sync::{
        atomic::{AtomicU64, Ordering},
        Arc,
    },
    time::{SystemTime, UNIX_EPOCH},
};

use tokio::sync::{Mutex, RwLock};

use crate::{
    application::{
        live_data::{AcceptedLiveEventConsumer, LiveRuntimeError},
        media::{
            AudioOutputPort, AudioPlayback, AudioRequest, ByteRange, EffectRuntime, FfmpegLocator,
            GiftMediaRuntime, GiftSoundBindingRecord, MediaFilePort, MediaIoError, MediaRepository,
            PlayUrlResolver, PreviewPort, PreviewRead, PreviewSession, RecorderLaunch,
            RecordingRecord, RecordingRuntime, TtsRuntime, UnavailableFfmpegLocator,
            UnavailablePlayUrlResolver,
        },
        paths::PathPurpose,
        ports::PathGrantPort,
    },
    domain::{
        live_data::LiveEvent,
        media::{
            validate_captain_effect_config, validate_gift_notify_config,
            validate_gift_sound_policy, validate_recording_config,
            validate_recording_filename_stem, validate_tts_config, CaptainEffectConfig,
            FfmpegSource, GiftNotifyConfig, GiftNotifyMethods, GiftSoundPolicy, MediaPosition,
            RecordingConfig, RecordingFormat, RecordingQuality, TtsConfig, TtsEventKind,
            TtsIdentityKind, TtsSelectedEngine, TtsTemplates, DEFAULT_RECORDING_QUOTA_BYTES,
            DEFAULT_RECORDING_WATERLINE_BYTES,
        },
    },
};

const MEDIA_EVENT_HISTORY_CAPACITY: usize = 4_096;

/// PRD 9.12: reconnect a dropped recording stream at most three times.
const RECORDING_MAX_RECONNECTS: u8 = 3;
const RECORDING_ALIVE_CHECK_MILLIS: u64 = 2_000;

/// Watches the recorder process and restarts it when the stream drops.
pub(crate) struct RecordingReconnectWatchdog {
    pub(crate) recording: Arc<RecordingRuntime>,
    pub(crate) play_url: Arc<dyn PlayUrlResolver>,
    pub(crate) ffmpeg: Arc<dyn FfmpegLocator>,
    pub(crate) granted_executable: Option<PathBuf>,
    pub(crate) ffmpeg_source: FfmpegSource,
    pub(crate) room_id: String,
    pub(crate) directory: PathBuf,
    pub(crate) quality: RecordingQuality,
    pub(crate) format: RecordingFormat,
}

impl RecordingReconnectWatchdog {
    pub(crate) async fn run(self) {
        let mut attempts: u8 = 0;
        loop {
            tokio::time::sleep(std::time::Duration::from_millis(
                RECORDING_ALIVE_CHECK_MILLIS,
            ))
            .await;
            if self.recording.snapshot().await.state != crate::domain::media::RecordingState::Recording
            {
                return;
            }
            if !self.recording.has_exited().await {
                continue;
            }
            attempts += 1;
            if attempts > RECORDING_MAX_RECONNECTS {
                // Before calling it a failure, ask whether the host simply
                // stopped broadcasting: 下播 is a normal end, not an error.
                if self.stream_offline().await {
                    log::info!("recording ended because the room went offline");
                    self.recording.end_naturally().await;
                } else {
                    log::warn!("recording stream lost after {attempts} attempts");
                    self.recording.fail("RECORD_STREAM_LOST").await;
                }
                return;
            }
            match self.restart(attempts).await {
                Ok(()) => log::info!("recording stream reconnected (attempt {attempts})"),
                Err(error) => {
                    log::warn!("recording reconnect {attempts} failed: {}", error.code);
                    if error.code == "ROOM_OFFLINE" || self.stream_offline().await {
                        log::info!("recording ended because the room went offline");
                        self.recording.end_naturally().await;
                        return;
                    }
                    if attempts >= RECORDING_MAX_RECONNECTS {
                        self.recording.fail("RECORD_STREAM_LOST").await;
                        return;
                    }
                }
            }
        }
    }

    /// True when the platform says the room is not live any more.
    async fn stream_offline(&self) -> bool {
        matches!(
            self.play_url.resolve(&self.room_id, self.quality).await,
            Err(error) if error.code == "ROOM_OFFLINE"
        )
    }

    /// Restart the recorder into a continuation file. Segments stay playable
    /// on their own, which is what PRD 9.12 requires after a drop.
    async fn restart(&self, attempt: u8) -> Result<(), MediaIoError> {
        let executable = self
            .ffmpeg
            .locate(self.ffmpeg_source, self.granted_executable.as_deref())?;
        let stream = self.play_url.resolve(&self.room_id, self.quality).await?;
        let extension = crate::domain::media::file_extension(self.format);
        let path = self.directory.join(format!(
            "{}-{}-part{}.{extension}",
            self.room_id,
            Self::stamp_millis(),
            attempt + 1
        ));
        let arguments = crate::domain::media::recording_arguments(
            &crate::domain::media::RecordingPlanInput {
                stream_url: &stream.url,
                output: &path,
                format: self.format,
                quality: self.quality,
            },
        )
        .map_err(|error| MediaIoError::coded("RECORD_PIPELINE_INVALID", error.to_string(), false))?;
        self.recording
            .replace_process(RecorderLaunch {
                executable,
                arguments,
                working_directory: Some(self.directory.clone()),
            })
            .await
    }

    fn stamp_millis() -> u128 {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_or(0, |duration| duration.as_millis())
    }
}


#[derive(Default)]
struct EventHistory {
    ids: HashSet<String>,
    order: VecDeque<String>,
}

#[derive(Clone)]
pub struct MediaServices {
    pub repository: Arc<dyn MediaRepository>,
    pub files: Arc<dyn MediaFilePort>,
    pub preview: Arc<dyn PreviewPort>,
    pub grants: Arc<dyn PathGrantPort>,
    pub audio: Arc<dyn AudioOutputPort>,
}

/// One concurrent recording the operator started, besides the primary runtime.
struct ExtraRecordingTask {
    id: u64,
    runtime: Arc<RecordingRuntime>,
    record_id: i64,
    room_id: String,
}

/// Bookkeeping for the recording owned by the primary runtime.
#[derive(Clone)]
struct PrimaryRecordingTask {
    room_id: String,
}

/// A single recording task as the UI sees it.
#[derive(Clone, Debug)]
pub struct RecordingTaskSnapshot {
    pub id: u64,
    pub room_id: Option<String>,
    pub state: crate::domain::media::RecordingState,
    pub elapsed_seconds: u64,
    pub file_size_bytes: u64,
    pub failure_code: Option<&'static str>,
}

pub trait MediaEventSink: Send + Sync {
    fn publish_captain_effect(
        &self,
        event: &LiveEvent,
        config: &CaptainEffectConfig,
    ) -> Result<(), MediaIoError>;
    fn publish_gift(
        &self,
        event: &LiveEvent,
        config: &GiftNotifyConfig,
        snapshot: &crate::application::media::GiftRuntimeSnapshot,
    ) -> Result<(), MediaIoError>;
    fn publish_tts(
        &self,
        snapshot: &crate::application::media::TtsRuntimeSnapshot,
    ) -> Result<(), MediaIoError>;
    fn publish_recording(
        &self,
        snapshot: &crate::application::media::RecordingRuntimeSnapshot,
    ) -> Result<(), MediaIoError>;
    fn publish_recording_completed(
        &self,
        recording: &crate::application::media::RecordingRecord,
    ) -> Result<(), MediaIoError>;
    fn publish_highlight(
        &self,
        highlight: &crate::application::media::HighlightRecord,
    ) -> Result<(), MediaIoError>;
}

struct NoopMediaEventSink;

impl MediaEventSink for NoopMediaEventSink {
    fn publish_captain_effect(
        &self,
        _: &LiveEvent,
        _: &CaptainEffectConfig,
    ) -> Result<(), MediaIoError> {
        Ok(())
    }

    fn publish_gift(
        &self,
        _: &LiveEvent,
        _: &GiftNotifyConfig,
        _: &crate::application::media::GiftRuntimeSnapshot,
    ) -> Result<(), MediaIoError> {
        Ok(())
    }

    fn publish_tts(
        &self,
        _: &crate::application::media::TtsRuntimeSnapshot,
    ) -> Result<(), MediaIoError> {
        Ok(())
    }

    fn publish_recording(
        &self,
        _: &crate::application::media::RecordingRuntimeSnapshot,
    ) -> Result<(), MediaIoError> {
        Ok(())
    }

    fn publish_recording_completed(
        &self,
        _: &crate::application::media::RecordingRecord,
    ) -> Result<(), MediaIoError> {
        Ok(())
    }

    fn publish_highlight(
        &self,
        _: &crate::application::media::HighlightRecord,
    ) -> Result<(), MediaIoError> {
        Ok(())
    }
}

pub struct MediaState {
    effects: Arc<EffectRuntime>,
    gifts: Arc<GiftMediaRuntime>,
    tts: Arc<TtsRuntime>,
    pub(crate) recording: Arc<RecordingRuntime>,
    accepted_events: Mutex<EventHistory>,
    services: Option<MediaServices>,
    preview_playbacks: Mutex<Vec<Box<dyn AudioPlayback>>>,
    local_ffmpeg_executable: RwLock<Option<PathBuf>>,
    event_sink: Arc<dyn MediaEventSink>,
    pub(crate) play_url: Arc<dyn PlayUrlResolver>,
    pub(crate) ffmpeg: Arc<dyn FfmpegLocator>,
    default_recording_directory: Option<PathBuf>,
    /// Directory holding the FFmpeg the app installed on the operator's behalf.
    managed_ffmpeg_directory: Option<PathBuf>,
    qwen3_service_launcher: Option<Arc<crate::infrastructure::media::Qwen3ServiceLauncher>>,
    qwen3_environment: Option<Arc<crate::infrastructure::media::Qwen3Environment>>,
    /// Parallel recordings; the primary runtime above owns task id 1.
    extra_recordings: Mutex<Vec<ExtraRecordingTask>>,
    primary_recording: Mutex<Option<PrimaryRecordingTask>>,
    next_recording_task_id: AtomicU64,
}

impl MediaState {
    pub fn new(
        effects: Arc<EffectRuntime>,
        gifts: Arc<GiftMediaRuntime>,
        tts: Arc<TtsRuntime>,
        recording: Arc<RecordingRuntime>,
    ) -> Self {
        Self {
            effects,
            gifts,
            tts,
            recording,
            accepted_events: Mutex::new(EventHistory::default()),
            services: None,
            preview_playbacks: Mutex::new(Vec::new()),
            local_ffmpeg_executable: RwLock::new(None),
            event_sink: Arc::new(NoopMediaEventSink),
            play_url: Arc::new(UnavailablePlayUrlResolver),
            ffmpeg: Arc::new(UnavailableFfmpegLocator),
            default_recording_directory: None,
            managed_ffmpeg_directory: None,
            qwen3_service_launcher: None,
            qwen3_environment: None,
            extra_recordings: Mutex::new(Vec::new()),
            primary_recording: Mutex::new(None),
            next_recording_task_id: AtomicU64::new(2),
        }
    }

    /// Wires the platform client and the FFmpeg locator used by recording.
    pub fn with_recording_clients(
        mut self,
        play_url: Arc<dyn PlayUrlResolver>,
        ffmpeg: Arc<dyn FfmpegLocator>,
    ) -> Self {
        self.play_url = play_url;
        self.ffmpeg = ffmpeg;
        self
    }

    /// PRD 9.12 recording must work out of the box, so the app owns a default
    /// `download` folder; the operator can still point elsewhere.
    pub fn with_default_recording_directory(mut self, directory: PathBuf) -> Self {
        self.default_recording_directory = Some(directory);
        self
    }

    pub fn with_managed_ffmpeg_directory(mut self, directory: PathBuf) -> Self {
        self.managed_ffmpeg_directory = Some(directory);
        self
    }

    pub fn with_qwen3_service_launcher(
        mut self,
        launcher: Arc<crate::infrastructure::media::Qwen3ServiceLauncher>,
    ) -> Self {
        self.qwen3_service_launcher = Some(launcher);
        self
    }

    pub fn with_qwen3_environment(
        mut self,
        environment: Arc<crate::infrastructure::media::Qwen3Environment>,
    ) -> Self {
        self.qwen3_environment = Some(environment);
        self
    }

    pub fn qwen3_environment(&self) -> Option<&Arc<crate::infrastructure::media::Qwen3Environment>> {
        self.qwen3_environment.as_ref()
    }

    pub fn qwen3_service_launcher(
        &self,
    ) -> Option<&Arc<crate::infrastructure::media::Qwen3ServiceLauncher>> {
        self.qwen3_service_launcher.as_ref()
    }

    pub fn managed_ffmpeg_directory(&self) -> Option<&Path> {
        self.managed_ffmpeg_directory.as_deref()
    }

    pub fn default_recording_directory(&self) -> Option<&Path> {
        self.default_recording_directory.as_deref()
    }

    pub fn with_services(mut self, services: MediaServices) -> Self {
        self.services = Some(services);
        self
    }

    pub fn with_event_sink(mut self, event_sink: Arc<dyn MediaEventSink>) -> Self {
        self.event_sink = event_sink;
        self
    }

    pub const fn effects(&self) -> &Arc<EffectRuntime> {
        &self.effects
    }

    pub const fn gifts(&self) -> &Arc<GiftMediaRuntime> {
        &self.gifts
    }

    pub const fn tts(&self) -> &Arc<TtsRuntime> {
        &self.tts
    }

    pub const fn recording(&self) -> &Arc<RecordingRuntime> {
        &self.recording
    }

    pub async fn consume(&self, event: &LiveEvent) {
        if !self.remember_event(&event.base().event_id).await {
            return;
        }
        self.gifts.consume(event).await;
        self.tts.enqueue_event(event).await;
        self.effects.enqueue(event).await;
        if let Some(marker) = self.recording.consume_event(event).await {
            let persisted = async {
                let recording = self.active_recording().await?;
                self.persist_highlight(
                    recording,
                    marker,
                    crate::application::media::HighlightTriggerType::DanmakuSurge,
                )
                .await
            }
            .await;
            if let Err(error) = persisted {
                log::warn!("automatic highlight is degraded: {}", error.code);
            }
        }
        // Parallel recordings keep their own danmaku heat window.
        let parallel: Vec<(Arc<RecordingRuntime>, i64)> = self
            .extra_recordings
            .lock()
            .await
            .iter()
            .map(|task| (task.runtime.clone(), task.record_id))
            .collect();
        for (runtime, record_id) in parallel {
            let Some(marker) = runtime.consume_event(event).await else {
                continue;
            };
            let persisted = async {
                let services = self.services()?;
                let recording = services
                    .repository
                    .list_recordings()
                    .await?
                    .into_iter()
                    .find(|recording| recording.id == record_id)
                    .ok_or_else(|| {
                        MediaIoError::unavailable("parallel recording record is unavailable")
                    })?;
                self.persist_highlight(
                    recording,
                    marker,
                    crate::application::media::HighlightTriggerType::DanmakuSurge,
                )
                .await
            }
            .await;
            if let Err(error) = persisted {
                log::warn!("automatic highlight is degraded: {}", error.code);
            }
        }
        match event {
            LiveEvent::Captain { .. } => {
                if let Err(error) = self
                    .event_sink
                    .publish_captain_effect(event, &self.effects.config().await)
                {
                    log::warn!("captain effect event is degraded: {}", error.code);
                }
            }
            LiveEvent::Gift { .. } => {
                if let Err(error) = self.event_sink.publish_gift(
                    event,
                    &self.gifts.notify_config().await,
                    &self.gifts.snapshot().await,
                ) {
                    log::warn!("gift media event is degraded: {}", error.code);
                }
            }
            LiveEvent::Danmu { .. } | LiveEvent::Enter { .. } => {}
            LiveEvent::SessionStarted { base } => {
                self.apply_session_boundary(base.room_id.as_str(), true).await;
            }
            LiveEvent::SessionEnded { base } => {
                self.apply_session_boundary(base.room_id.as_str(), false).await;
            }
            LiveEvent::SuperChat { .. } | LiveEvent::SuperChatDeleted { .. } => {}
            // PRD 9.1.1: likes only feed the interaction counter.
            LiveEvent::Like { .. } => {}
        }
        if let Err(error) = self.event_sink.publish_tts(&self.tts.snapshot().await) {
            log::warn!("tts status event is degraded: {}", error.code);
        }
        if let Err(error) = self
            .event_sink
            .publish_recording(&self.recording.snapshot().await)
        {
            log::warn!("recording status event is degraded: {}", error.code);
        }
    }

    pub async fn shutdown(&self) {
        self.effects.shutdown().await;
        self.gifts.shutdown().await;
        self.tts.shutdown().await;
        let extra_runtimes: Vec<Arc<RecordingRuntime>> = self
            .extra_recordings
            .lock()
            .await
            .iter()
            .map(|task| task.runtime.clone())
            .collect();
        for runtime in extra_runtimes {
            runtime.shutdown().await;
        }
        self.extra_recordings.lock().await.clear();
        self.recording.shutdown().await;
        let mut previews = self.preview_playbacks.lock().await;
        for playback in previews.drain(..) {
            playback.stop();
        }
    }

    pub async fn save_captain_config(
        &self,
        config: CaptainEffectConfig,
    ) -> Result<CaptainEffectConfig, MediaIoError> {
        validate_captain_effect_config(&config)
            .map_err(|error| MediaIoError::invalid(error.to_string()))?;
        self.services()?
            .repository
            .save_captain_config(&config)
            .await?;
        self.effects.set_config(config.clone()).await;
        Ok(config)
    }

    pub async fn save_gift_notify_config(
        &self,
        config: GiftNotifyConfig,
    ) -> Result<GiftNotifyConfig, MediaIoError> {
        validate_gift_notify_config(&config)
            .map_err(|error| MediaIoError::invalid(error.to_string()))?;
        self.services()?
            .repository
            .save_gift_notify_config(&config)
            .await?;
        self.gifts.set_notify_config(config.clone()).await;
        Ok(config)
    }

    pub async fn save_gift_sounds(
        &self,
        policy: GiftSoundPolicy,
        bindings: Vec<GiftSoundBindingRecord>,
    ) -> Result<(), MediaIoError> {
        validate_gift_sound_policy(&policy)
            .map_err(|error| MediaIoError::invalid(error.to_string()))?;
        let services = self.services()?;
        services.repository.replace_gift_sounds(&bindings).await?;
        services.repository.save_gift_sound_policy(&policy).await?;
        self.gifts.set_sound_policy(policy).await;
        self.gifts.replace_bindings(bindings).await;
        Ok(())
    }

    pub async fn set_gift_sound(
        &self,
        gift_id: String,
        gift_name: String,
        grant_id: &str,
    ) -> Result<GiftSoundBindingRecord, MediaIoError> {
        let services = self.services()?;
        let source = services.files.resolve_gift_sound(grant_id)?;
        let existing = self
            .gifts
            .bindings()
            .await
            .into_iter()
            .find(|binding| binding.gift_id == gift_id);
        let binding = GiftSoundBindingRecord {
            gift_id,
            gift_name,
            sound_path: Some(source),
            volume: existing.as_ref().map_or(0.8, |value| value.volume),
            is_enabled: existing.as_ref().is_none_or(|value| value.is_enabled),
        };
        services.repository.upsert_gift_sound(&binding).await?;
        let bindings = services.repository.list_gift_sounds().await?;
        self.gifts.replace_bindings(bindings).await;
        Ok(binding)
    }

    pub async fn remove_gift_sound(&self, gift_id: &str) -> Result<bool, MediaIoError> {
        let services = self.services()?;
        let deleted = services.repository.remove_gift_sound(gift_id).await?;
        self.gifts
            .replace_bindings(services.repository.list_gift_sounds().await?)
            .await;
        Ok(deleted)
    }

    pub async fn import_gift_sounds(&self, grant_id: &str) -> Result<usize, MediaIoError> {
        let services = self.services()?;
        let bindings = services.files.import_gift_sounds(grant_id).await?;
        services.repository.replace_gift_sounds(&bindings).await?;
        self.gifts.replace_bindings(bindings.clone()).await;
        Ok(bindings.len())
    }

    pub async fn export_gift_sounds(&self, grant_id: &str) -> Result<usize, MediaIoError> {
        let services = self.services()?;
        let bindings = self.gifts.bindings().await;
        services
            .files
            .export_gift_sounds(grant_id, &bindings)
            .await?;
        Ok(bindings.len())
    }

    pub async fn preview_sound(&self, source: PathBuf, volume: f64) -> Result<(), MediaIoError> {
        let playback = self
            .services()?
            .audio
            .play(AudioRequest { source, volume })
            .await?;
        let mut previews = self.preview_playbacks.lock().await;
        previews.retain(|current| !current.is_finished());
        previews.push(playback);
        Ok(())
    }

    pub async fn sound_source(&self, binding_id: &str) -> Result<PathBuf, MediaIoError> {
        let effect = self.effects.config().await;
        if let Some(source) = [effect.first_sound, effect.renewal_sound, effect.combo_sound]
            .into_iter()
            .flatten()
            .find(|binding| binding.binding_id == binding_id)
            .map(|binding| binding.source)
        {
            return Ok(source);
        }
        self.gifts
            .bindings()
            .await
            .into_iter()
            .find(|binding| binding.gift_id == binding_id)
            .and_then(|binding| binding.sound_path)
            .ok_or_else(|| MediaIoError::invalid("sound binding does not exist"))
    }

    pub async fn save_tts_config(&self, config: TtsConfig) -> Result<TtsConfig, MediaIoError> {
        validate_tts_config(&config).map_err(|error| MediaIoError::invalid(error.to_string()))?;
        self.services()?.repository.save_tts_config(&config).await?;
        self.tts.set_config(config.clone()).await;
        Ok(config)
    }

    pub async fn save_recording_config(
        &self,
        config: RecordingConfig,
    ) -> Result<RecordingConfig, MediaIoError> {
        validate_recording_config(&config)
            .map_err(|error| MediaIoError::invalid(error.to_string()))?;
        self.services()?
            .repository
            .save_recording_config(&config)
            .await?;
        self.recording.set_config(config.clone()).await;
        Ok(config)
    }

    pub async fn set_ffmpeg_source(
        &self,
        source: FfmpegSource,
        grant_id: Option<&str>,
    ) -> Result<RecordingConfig, MediaIoError> {
        let executable = match source {
            // Nothing to store: both resolve to whatever is available at start.
            FfmpegSource::Bundled | FfmpegSource::System | FfmpegSource::Installed => None,
            FfmpegSource::Local => Some(
                self.services()?
                    .grants
                    .resolve(
                        grant_id
                            .ok_or_else(|| MediaIoError::invalid("FFmpeg grant is required"))?,
                        PathPurpose::LocalFfmpegExecutable,
                    )
                    .map_err(|_| MediaIoError::permission("FFmpeg authorization is invalid"))?,
            ),
        };
        *self.local_ffmpeg_executable.write().await = executable;
        let mut config = self.recording.config().await;
        config.ffmpeg_source = source;
        self.save_recording_config(config).await
    }

    pub async fn list_recordings(&self) -> Result<Vec<RecordingRecord>, MediaIoError> {
        self.services()?.repository.list_recordings().await
    }

    pub async fn rename_recording(
        &self,
        id: i64,
        file_name_stem: &str,
    ) -> Result<RecordingRecord, MediaIoError> {
        let services = self.services()?;
        let mut recording = services
            .repository
            .list_recordings()
            .await?
            .into_iter()
            .find(|recording| recording.id == id)
            .ok_or_else(|| MediaIoError::invalid("recording does not exist"))?;
        if recording.status == crate::application::media::RecordingItemState::Recording {
            return Err(MediaIoError::conflict("active recording cannot be renamed"));
        }
        let stem = validate_recording_filename_stem(file_name_stem)
            .map_err(|error| MediaIoError::invalid(error.to_string()))?;
        let extension = match recording.format {
            RecordingFormat::Mp4 => "mp4",
            RecordingFormat::Flv => "flv",
        };
        let target = recording
            .file_path
            .parent()
            .ok_or_else(|| MediaIoError::invalid("recording path is invalid"))?
            .join(format!("{stem}.{extension}"));
        let original = recording.file_path.clone();
        services.files.move_file(&original, &target).await?;
        recording.file_path = target.clone();
        match services.repository.update_recording(&recording).await {
            Ok(true) => Ok(recording),
            Ok(false) => {
                services.files.move_file(&target, &original).await?;
                Err(MediaIoError::invalid("recording does not exist"))
            }
            Err(error) => {
                services.files.move_file(&target, &original).await?;
                Err(error)
            }
        }
    }

    pub async fn delete_recording(&self, id: i64) -> Result<bool, MediaIoError> {
        let services = self.services()?;
        let recording = services
            .repository
            .list_recordings()
            .await?
            .into_iter()
            .find(|recording| recording.id == id)
            .ok_or_else(|| MediaIoError::invalid("recording does not exist"))?;
        if recording.status == crate::application::media::RecordingItemState::Recording {
            return Err(MediaIoError::conflict("active recording cannot be deleted"));
        }
        let mut files = vec![recording.file_path];
        files.extend(
            services
                .repository
                .list_highlights(id)
                .await?
                .into_iter()
                .filter_map(|highlight| highlight.file_path),
        );
        files.sort();
        files.dedup();
        let staged = services.files.stage_delete(&files).await?;
        match services.repository.delete_recording(id).await {
            Ok(true) => {
                services.files.finalize_staged(&staged).await?;
                Ok(true)
            }
            Ok(false) => {
                services.files.restore_staged(&staged).await?;
                Err(MediaIoError::invalid("recording does not exist"))
            }
            Err(error) => {
                services.files.restore_staged(&staged).await?;
                Err(error)
            }
        }
    }

    pub async fn list_highlights(
        &self,
    ) -> Result<Vec<crate::application::media::HighlightRecord>, MediaIoError> {
        let services = self.services()?;
        let mut highlights = Vec::new();
        for recording in services.repository.list_recordings().await? {
            highlights.extend(services.repository.list_highlights(recording.id).await?);
        }
        Ok(highlights)
    }

    pub async fn delete_highlight(&self, id: i64) -> Result<bool, MediaIoError> {
        let services = self.services()?;
        let Some(highlight) = self
            .list_highlights()
            .await?
            .into_iter()
            .find(|highlight| highlight.id == id)
        else {
            // Already deleted (double click, or removed elsewhere) — treat the
            // operation as done instead of raising a configuration error.
            return Ok(false);
        };
        let staged = match highlight.file_path {
            Some(path) => services.files.stage_delete(&[path]).await?,
            None => Vec::new(),
        };
        match services.repository.delete_highlight(id).await {
            Ok(true) => {
                services.files.finalize_staged(&staged).await?;
                Ok(true)
            }
            Ok(false) => {
                services.files.restore_staged(&staged).await?;
                Err(MediaIoError::invalid("highlight does not exist"))
            }
            Err(error) => {
                services.files.restore_staged(&staged).await?;
                Err(error)
            }
        }
    }

    pub async fn export_highlight(
        &self,
        id: i64,
        target: PathBuf,
    ) -> Result<PathBuf, MediaIoError> {
        let source = self
            .list_highlights()
            .await?
            .into_iter()
            .find(|highlight| highlight.id == id)
            .and_then(|highlight| highlight.file_path)
            .ok_or_else(|| MediaIoError::unavailable("highlight clip is unavailable"))?;
        self.services()?.files.export_file(&source, &target).await?;
        Ok(target)
    }

    pub fn issue_preview(
        &self,
        file: PathBuf,
        owner: &str,
        now_ms: i64,
    ) -> Result<PreviewSession, MediaIoError> {
        self.services()?.preview.issue(file, owner, now_ms, 60_000)
    }

    pub async fn read_preview(
        &self,
        token: &str,
        owner: &str,
        range: Option<ByteRange>,
        now_ms: i64,
    ) -> Result<PreviewRead, MediaIoError> {
        self.services()?
            .preview
            .read(token, owner, range, now_ms)
            .await
    }

    pub async fn mark_highlight(
        &self,
    ) -> Result<crate::application::media::HighlightRecord, MediaIoError> {
        let recording = self.active_recording().await?;
        let marker = self.recording.mark_highlight().await?;
        self.persist_highlight(
            recording,
            marker,
            crate::application::media::HighlightTriggerType::Manual,
        )
        .await
    }

    /// Resolves everything a recording needs (directory, FFmpeg, stream URL),
    /// persists the record, then hands the launch line to the runtime.
    pub async fn start_recording(&self, room_id: &str) -> Result<(), MediaIoError> {
        let room_id = crate::domain::media::validate_recording_room_id(room_id).map_err(|_| {
            MediaIoError::coded("ROOM_INVALID_ROOM", "请输入正确的直播间号", false)
        })?;
        let services = self.services()?;
        let config = self.recording.config().await;

        let grant = config.directory.as_ref();
        // A configured grant wins; otherwise the app-owned `download` folder is
        // used so recording works without asking the operator to pick a folder.
        let directory = match grant {
            Some(grant) if grant.grant_id != APP_DOWNLOAD_GRANT_ID => services
                .grants
                .resolve(&grant.grant_id, PathPurpose::RecordingDirectory)
                .map_err(|_| {
                    MediaIoError::coded(
                        "RECORD_DIRECTORY_REQUIRED",
                        "录制目录授权已失效，请重新选择",
                        false,
                    )
                })?,
            _ => self
                .default_recording_directory
                .clone()
                .ok_or_else(|| {
                    MediaIoError::coded(
                        "RECORD_DIRECTORY_REQUIRED",
                        "请先在录制设置中选择保存目录",
                        false,
                    )
                })?,
        };
        tokio::fs::create_dir_all(&directory)
            .await
            .map_err(|_| {
                MediaIoError::coded(
                    "RECORD_DIRECTORY_UNAVAILABLE",
                    "录制目录无法创建或不可写",
                    false,
                )
            })?;

        let granted_executable = self.local_ffmpeg_executable.read().await.clone();
        let executable = self
            .ffmpeg
            .locate(config.ffmpeg_source, granted_executable.as_deref())?;
        let stream = self.play_url.resolve(room_id, config.quality).await?;

        let format = config.format;
        let path = Self::unique_recording_path(&directory, room_id, format);
        let arguments = crate::domain::media::recording_arguments(
            &crate::domain::media::RecordingPlanInput {
                stream_url: &stream.url,
                output: &path,
                format,
                quality: config.quality,
            },
        )
        .map_err(|error| MediaIoError::coded("RECORD_PIPELINE_INVALID", error.to_string(), false))?;

        let mut record = services
            .repository
            .insert_recording(&RecordingRecord {
                id: 0,
                room_id: room_id.to_owned(),
                file_path: path.clone(),
                format,
                quality: config.quality,
                started_at: Self::now_millis(),
                ended_at: None,
                size_bytes: 0,
                duration_seconds: 0,
                status: crate::application::media::RecordingItemState::Recording,
            })
            .await?;

        let launch = RecorderLaunch {
            executable,
            arguments,
            working_directory: Some(directory.clone()),
        };

        // Several rooms can record at once: the first start owns the primary
        // runtime (auto-record and the global hotkey use it), every additional
        // start forks an independent runtime with its own recorder process.
        let reuse_primary = {
            let state = self.recording.snapshot().await.state;
            matches!(
                state,
                crate::domain::media::RecordingState::Idle
                    | crate::domain::media::RecordingState::Error
            ) && self.primary_recording.lock().await.is_none()
                && self.extra_recordings.lock().await.is_empty()
        };
        let (task_id, runtime) = if reuse_primary {
            *self.primary_recording.lock().await = Some(PrimaryRecordingTask {
                room_id: room_id.to_owned(),
            });
            (1, self.recording.clone())
        } else {
            let runtime = self.recording.fork().await;
            let task_id = self.next_recording_task_id.fetch_add(1, Ordering::Relaxed);
            self.extra_recordings
                .lock()
                .await
                .push(ExtraRecordingTask {
                    id: task_id,
                    runtime: runtime.clone(),
                    record_id: record.id,
                    room_id: room_id.to_owned(),
                });
            (task_id, runtime)
        };
        let config_for_task = config.clone();
        let record_id = record.id;
        match runtime.start(room_id.to_owned(), launch).await {
            Ok(()) => {
                let _ = self
                    .event_sink
                    .publish_recording(&self.recording.snapshot().await);
                self.spawn_recording_watchdog(
                    runtime.clone(),
                    room_id.to_owned(),
                    directory,
                    granted_executable,
                    config_for_task,
                );
                self.spawn_recording_supervisor(runtime, record_id);
                let _ = task_id;
                Ok(())
            }
            Err(error) => {
                self.drop_recording_task(task_id).await;
                record.status = crate::application::media::RecordingItemState::Failed;
                record.ended_at = Some(Self::now_millis());
                let _ = services.repository.update_recording(&record).await;
                let _ = self
                    .event_sink
                    .publish_recording(&self.recording.snapshot().await);
                Err(error)
            }
        }
    }

    /// Forgets the bookkeeping of a task that never started.
    async fn drop_recording_task(&self, task_id: u64) {
        if task_id == 1 {
            *self.primary_recording.lock().await = None;
        } else {
            self.extra_recordings
                .lock()
                .await
                .retain(|task| task.id != task_id);
        }
    }

    fn spawn_recording_watchdog(
        &self,
        runtime: Arc<RecordingRuntime>,
        room_id: String,
        directory: PathBuf,
        granted_executable: Option<PathBuf>,
        config: RecordingConfig,
    ) {
        let watchdog = RecordingReconnectWatchdog {
            recording: runtime,
            play_url: self.play_url.clone(),
            ffmpeg: self.ffmpeg.clone(),
            granted_executable,
            ffmpeg_source: config.ffmpeg_source,
            room_id,
            directory,
            quality: config.quality,
            format: config.format,
        };
        tokio::spawn(watchdog.run());
    }

    /// Watches one task and stores its outcome when the recorder reaches a
    /// terminal state on its own (host stopped broadcasting, unrecoverable
    /// failure), so the list never keeps a finished task as still recording.
    fn spawn_recording_supervisor(&self, runtime: Arc<RecordingRuntime>, record_id: i64) {
        let Some(services) = self.services.clone() else {
            return;
        };
        let sink = self.event_sink.clone();
        tokio::spawn(async move {
            loop {
                tokio::time::sleep(std::time::Duration::from_millis(1500)).await;
                let snapshot = runtime.snapshot().await;
                let outcome = match snapshot.state {
                    crate::domain::media::RecordingState::Idle => {
                        Some(crate::application::media::RecordingItemState::Completed)
                    }
                    crate::domain::media::RecordingState::Error => {
                        Some(crate::application::media::RecordingItemState::Partial)
                    }
                    _ => None,
                };
                let Some(status) = outcome else {
                    continue;
                };
                if let Ok(records) = services.repository.list_recordings().await {
                    if let Some(mut record) =
                        records.into_iter().find(|record| record.id == record_id)
                    {
                        if record.status == crate::application::media::RecordingItemState::Recording
                        {
                            record.ended_at = Some(MediaState::now_millis());
                            let ended = record.ended_at.unwrap_or(record.started_at);
                            record.duration_seconds =
                                (ended.saturating_sub(record.started_at) / 1000) as u64;
                            record.size_bytes =
                                services.files.file_size(&record.file_path).await.unwrap_or(0);
                            record.status = status;
                            if services.repository.update_recording(&record).await.is_ok() {
                                let _ = sink.publish_recording_completed(&record);
                            }
                        }
                    }
                }
                runtime.reset().await;
                break;
            }
        });
    }

    /// Every recording the operator started, newest last.
    pub async fn recording_tasks(&self) -> Vec<RecordingTaskSnapshot> {
        let mut tasks = Vec::new();
        let primary = self.primary_recording.lock().await.clone();
        if let Some(primary) = primary {
            let snapshot = self.recording.snapshot().await;
            if matches!(snapshot.state, crate::domain::media::RecordingState::Idle) {
                *self.primary_recording.lock().await = None;
            } else {
                tasks.push(RecordingTaskSnapshot {
                    id: 1,
                    room_id: Some(primary.room_id),
                    state: snapshot.state,
                    elapsed_seconds: snapshot.elapsed_seconds,
                    file_size_bytes: self.recording.output_size_bytes().await,
                    failure_code: snapshot.failure_code,
                });
            }
        }
        let extras: Vec<(u64, Arc<RecordingRuntime>, String)> = self
            .extra_recordings
            .lock()
            .await
            .iter()
            .map(|task| (task.id, task.runtime.clone(), task.room_id.clone()))
            .collect();
        let mut finished = Vec::new();
        for (id, runtime, room_id) in extras {
            let snapshot = runtime.snapshot().await;
            if matches!(snapshot.state, crate::domain::media::RecordingState::Idle) {
                finished.push(id);
                continue;
            }
            tasks.push(RecordingTaskSnapshot {
                id,
                room_id: Some(room_id),
                state: snapshot.state,
                elapsed_seconds: snapshot.elapsed_seconds,
                file_size_bytes: runtime.output_size_bytes().await,
                failure_code: snapshot.failure_code,
            });
        }
        if !finished.is_empty() {
            self.extra_recordings
                .lock()
                .await
                .retain(|task| !finished.contains(&task.id));
        }
        tasks
    }

    fn unique_recording_path(directory: &Path, room_id: &str, format: RecordingFormat) -> PathBuf {        let extension = crate::domain::media::file_extension(format);
        let stamp = Self::now_millis();
        let mut candidate = directory.join(format!("{room_id}-{stamp}.{extension}"));
        let mut index = 1;
        while candidate.exists() {
            candidate = directory.join(format!("{room_id}-{stamp}-{index}.{extension}"));
            index += 1;
        }
        candidate
    }

    /// A recording row is left in `Recording` when the previous run ended
    /// without a clean stop (crash, kill). Mark those rows as partial so the
    /// list stops claiming a recording is still running.
    pub async fn recover_interrupted_recordings(&self) -> Result<(), MediaIoError> {
        let services = self.services()?;
        let interrupted: Vec<_> = services
            .repository
            .list_recordings()
            .await?
            .into_iter()
            .filter(|recording| {
                recording.status == crate::application::media::RecordingItemState::Recording
            })
            .collect();
        for mut recording in interrupted {
            recording.ended_at = Some(Self::now_millis());
            recording.size_bytes = services
                .files
                .file_size(&recording.file_path)
                .await
                .unwrap_or(0);
            recording.status = crate::application::media::RecordingItemState::Partial;
            let _ = services.repository.update_recording(&recording).await?;
        }
        Ok(())
    }

    /// Stops every running recording (the primary task and any parallel ones).
    pub async fn stop_recording(&self) -> Result<(), MediaIoError> {
        let primary = self.stop_primary_recording().await;
        let extra_ids: Vec<u64> = self
            .extra_recordings
            .lock()
            .await
            .iter()
            .map(|task| task.id)
            .collect();
        let mut first_error = primary.err();
        for task_id in extra_ids {
            if let Err(error) = self.stop_recording_task(task_id).await {
                if first_error.is_none() {
                    first_error = Some(error);
                }
            }
        }
        match first_error {
            Some(error) => Err(error),
            None => Ok(()),
        }
    }

    /// Stops one task by id (1 = the primary runtime).
    pub async fn stop_recording_task(&self, task_id: u64) -> Result<(), MediaIoError> {
        if task_id == 1 {
            return self.stop_primary_recording().await;
        }
        let target = self
            .extra_recordings
            .lock()
            .await
            .iter()
            .find(|task| task.id == task_id)
            .map(|task| (task.runtime.clone(), task.record_id));
        let Some((runtime, record_id)) = target else {
            return Err(MediaIoError::invalid("recording task does not exist"));
        };
        let result = runtime.stop().await;
        let status = if result.is_ok() {
            crate::application::media::RecordingItemState::Completed
        } else {
            crate::application::media::RecordingItemState::Partial
        };
        self.finalize_recording_record(record_id, status).await;
        runtime.reset().await;
        self.extra_recordings
            .lock()
            .await
            .retain(|task| task.id != task_id);
        let _ = self
            .event_sink
            .publish_recording(&self.recording.snapshot().await);
        result
    }

    /// Marks a highlight on one task (1 = the primary runtime).
    pub async fn mark_highlight_for_task(
        &self,
        task_id: u64,
    ) -> Result<crate::application::media::HighlightRecord, MediaIoError> {
        if task_id == 1 {
            return self.mark_highlight().await;
        }
        let target = self
            .extra_recordings
            .lock()
            .await
            .iter()
            .find(|task| task.id == task_id)
            .map(|task| (task.runtime.clone(), task.record_id));
        let Some((runtime, record_id)) = target else {
            return Err(MediaIoError::invalid("recording task does not exist"));
        };
        let marker = runtime.mark_highlight().await?;
        let record = self
            .services()?
            .repository
            .list_recordings()
            .await?
            .into_iter()
            .find(|recording| recording.id == record_id)
            .ok_or_else(|| MediaIoError::unavailable("recording record is unavailable"))?;
        self.persist_highlight(
            record,
            marker,
            crate::application::media::HighlightTriggerType::Manual,
        )
        .await
    }

    /// Stores the outcome of a finished task, leaving already-finalised rows
    /// alone so a manual stop followed by the supervisor cannot double-write.
    async fn finalize_recording_record(
        &self,
        record_id: i64,
        status: crate::application::media::RecordingItemState,
    ) {
        let Ok(services) = self.services() else {
            return;
        };
        let Ok(records) = services.repository.list_recordings().await else {
            return;
        };
        let Some(mut record) = records.into_iter().find(|record| record.id == record_id) else {
            return;
        };
        if record.status != crate::application::media::RecordingItemState::Recording {
            return;
        }
        record.ended_at = Some(Self::now_millis());
        let ended = record.ended_at.unwrap_or(record.started_at);
        record.duration_seconds = (ended.saturating_sub(record.started_at) / 1000) as u64;
        record.size_bytes = services
            .files
            .file_size(&record.file_path)
            .await
            .unwrap_or(0);
        record.status = status;
        if services
            .repository
            .update_recording(&record)
            .await
            .unwrap_or(false)
        {
            let _ = self.event_sink.publish_recording_completed(&record);
        }
    }

    async fn stop_primary_recording(&self) -> Result<(), MediaIoError> {
        let before = self.recording.snapshot().await;
        let active = match before.room_id.as_deref() {
            Some(room_id) => self
                .services()?
                .repository
                .list_recordings()
                .await?
                .into_iter()
                .rev()
                .find(|recording| {
                    recording.room_id == room_id
                        && recording.status
                            == crate::application::media::RecordingItemState::Recording
                }),
            None => None,
        };
        self.recording.stop().await?;
        if let Some(mut recording) = active {
            recording.ended_at = Some(Self::now_millis());
            recording.duration_seconds = before.elapsed_seconds;
            recording.size_bytes = self
                .services()?
                .files
                .file_size(&recording.file_path)
                .await?;
            recording.status = crate::application::media::RecordingItemState::Completed;
            if !self
                .services()?
                .repository
                .update_recording(&recording)
                .await?
            {
                return Err(MediaIoError::operation(
                    "recording finalization record is unavailable",
                    false,
                ));
            }
            self.event_sink.publish_recording_completed(&recording)?;
        }
        *self.primary_recording.lock().await = None;
        self.event_sink
            .publish_recording(&self.recording.snapshot().await)
    }

    /// PRD 9.1.1: `LIVE_START` is the auto-record trigger and `LIVE_END` stops
    /// it. Manual recordings are never interrupted by a session boundary the
    /// user did not ask for.
    async fn apply_session_boundary(&self, room_id: &str, started: bool) {
        let config = self.recording.config().await;
        if !config.auto_record {
            return;
        }
        let state = self.recording.snapshot().await.state;
        if started {
            if state != crate::domain::media::RecordingState::Idle {
                return;
            }
            if let Err(error) = self.start_recording(room_id).await {
                log::warn!("automatic recording start is degraded: {}", error.code);
            }
            return;
        }
        if matches!(
            state,
            crate::domain::media::RecordingState::Recording
                | crate::domain::media::RecordingState::Stopping
                | crate::domain::media::RecordingState::Finalizing
        ) {
            if let Err(error) = self.stop_recording().await {
                log::warn!("automatic recording stop is degraded: {}", error.code);
            }
        }
    }

    async fn persist_highlight(        &self,
        recording: RecordingRecord,
        marker: crate::domain::media::HighlightWindow,
        trigger_type: crate::application::media::HighlightTriggerType,
    ) -> Result<crate::application::media::HighlightRecord, MediaIoError> {
        let services = self.services()?;
        let config = self.recording.config().await;
        let mut highlight = services
            .repository
            .insert_highlight(&crate::application::media::HighlightRecord {
                id: 0,
                recording_id: recording.id,
                room_id: recording.room_id.clone(),
                position_seconds: marker
                    .end_seconds
                    .saturating_sub(u64::from(config.highlight_after_seconds)),
                trigger_type,
                clip_duration_seconds: u64::from(config.highlight_before_seconds)
                    + u64::from(config.highlight_after_seconds),
                file_path: None,
                created_at: Self::now_millis(),
                is_clip_available: false,
            })
            .await?;
        // PRD 9.12: cut the clip right away so the highlight is exportable.
        match self.extract_highlight_clip(&recording, &highlight).await {
            Ok(clip) => {
                if services
                    .repository
                    .set_highlight_clip(highlight.id, &clip)
                    .await
                    .is_ok()
                {
                    highlight.file_path = Some(clip);
                    highlight.is_clip_available = true;
                }
            }
            Err(error) => {
                log::warn!("highlight clip extraction is degraded: {}", error.code);
            }
        }
        self.event_sink.publish_highlight(&highlight)?;
        Ok(highlight)
    }

    /// Remuxes [marker - before, marker + after] out of the active recording.
    async fn extract_highlight_clip(
        &self,
        recording: &RecordingRecord,
        highlight: &crate::application::media::HighlightRecord,
    ) -> Result<PathBuf, MediaIoError> {
        let config = self.recording.config().await;
        let source = PathBuf::from(&recording.file_path);
        if !source.is_file() {
            return Err(MediaIoError::unavailable("recording file is unavailable"));
        }
        let granted_executable = self.local_ffmpeg_executable.read().await.clone();
        let executable = self
            .ffmpeg
            .locate(config.ffmpeg_source, granted_executable.as_deref())?;
        let directory = source
            .parent()
            .ok_or_else(|| MediaIoError::invalid("recording directory is invalid"))?;
        let extension = source
            .extension()
            .and_then(std::ffi::OsStr::to_str)
            .unwrap_or("flv")
            .to_owned();
        let target = directory.join(format!(
            "recording-{}-highlight-{}.{extension}",
            recording.id, highlight.id
        ));
        let arguments = crate::domain::media::highlight_clip_arguments(
            &crate::domain::media::HighlightClipInput {
                source: &source,
                output: &target,
                position_seconds: highlight.position_seconds,
                before_seconds: u32::from(config.highlight_before_seconds),
                after_seconds: u32::from(config.highlight_after_seconds),
            },
        )
        .map_err(|_| MediaIoError::invalid("highlight clip arguments are invalid"))?;
        self.recording.run_tool(&executable, &arguments).await?;
        if !target.is_file() {
            return Err(MediaIoError::operation(
                "highlight clip was not written",
                false,
            ));
        }
        Ok(target)
    }

    async fn active_recording(&self) -> Result<RecordingRecord, MediaIoError> {
        let room_id = self
            .recording
            .snapshot()
            .await
            .room_id
            .ok_or_else(|| MediaIoError::conflict("recording is not active"))?;
        self.services()?
            .repository
            .list_recordings()
            .await?
            .into_iter()
            .rev()
            .find(|recording| {
                recording.room_id == room_id
                    && recording.status == crate::application::media::RecordingItemState::Recording
            })
            .ok_or_else(|| MediaIoError::unavailable("active recording record is unavailable"))
    }

    pub fn resolve_grant(
        &self,
        grant_id: &str,
        purpose: PathPurpose,
    ) -> Result<PathBuf, MediaIoError> {
        self.services()?
            .grants
            .resolve(grant_id, purpose)
            .map_err(|_| MediaIoError::permission("path authorization is invalid"))
    }

    pub fn now_millis() -> i64 {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_millis()
            .try_into()
            .unwrap_or(i64::MAX)
    }

    async fn remember_event(&self, event_id: &str) -> bool {
        let mut history = self.accepted_events.lock().await;
        if !history.ids.insert(event_id.to_owned()) {
            return false;
        }
        history.order.push_back(event_id.to_owned());
        while history.order.len() > MEDIA_EVENT_HISTORY_CAPACITY {
            if let Some(expired) = history.order.pop_front() {
                history.ids.remove(&expired);
            }
        }
        true
    }

    fn services(&self) -> Result<&MediaServices, MediaIoError> {
        self.services
            .as_ref()
            .ok_or_else(|| MediaIoError::unavailable("media services are unavailable"))
    }
}

#[async_trait::async_trait]
impl AcceptedLiveEventConsumer for MediaState {
    async fn consume_accepted(&self, event: &LiveEvent) -> Result<(), LiveRuntimeError> {
        self.consume(event).await;
        Ok(())
    }
}

pub fn default_captain_effect_config() -> CaptainEffectConfig {
    CaptainEffectConfig {
        preset_id: "captain-default".into(),
        duration_seconds: 5,
        position: MediaPosition::Center,
        window_width: 560,
        window_height: 280,
        primary_color: "#00AEEC".into(),
        secondary_color: "#FB7299".into(),
        first_sound: None,
        renewal_sound: None,
        combo_sound: None,
        volume: 0.8,
        combo_enabled: true,
        combo_window_seconds: 10,
    }
}

pub const fn default_gift_notify_config() -> GiftNotifyConfig {
    GiftNotifyConfig {
        is_enabled: true,
        high_value_threshold_minor: 10_000,
        methods: GiftNotifyMethods {
            system_notification: true,
            local_popup: true,
            sound: true,
        },
        combo_enabled: true,
        combo_window_seconds: 10,
        popup_position: MediaPosition::TopRight,
        popup_duration_seconds: 5,
    }
}

pub const fn default_gift_sound_policy() -> GiftSoundPolicy {
    GiftSoundPolicy {
        global_volume: 0.8,
        queue_capacity_per_second: 20,
    }
}

pub fn default_tts_config() -> TtsConfig {
    TtsConfig {
        is_enabled: false,
        selected_engine: TtsSelectedEngine::System,
        selected_voice_id: None,
        rate: 1.0,
        volume: 0.8,
        event_kinds: vec![
            TtsEventKind::Danmu,
            TtsEventKind::Gift,
            TtsEventKind::Captain,
            TtsEventKind::Entry,
        ],
        identity_kinds: vec![
            TtsIdentityKind::Admin,
            TtsIdentityKind::Captain,
            TtsIdentityKind::Vip,
            TtsIdentityKind::Ordinary,
        ],
        templates: TtsTemplates {
            danmu: "{用户}说：{内容}".into(),
            gift: "感谢{用户}赠送的{礼物}{数量}个".into(),
            captain: "感谢{用户}开通{等级}".into(),
        },
    }
}

pub fn default_recording_config() -> RecordingConfig {
    RecordingConfig {
        format: RecordingFormat::Mp4,
        quality: RecordingQuality::Original,
        directory: None,
        segmentation_enabled: false,
        segment_duration_seconds: 3_600,
        auto_record: false,
        danmaku_threshold: 20,
        highlight_before_seconds: 30,
        highlight_after_seconds: 30,
        quota_bytes: DEFAULT_RECORDING_QUOTA_BYTES,
        waterline_bytes: DEFAULT_RECORDING_WATERLINE_BYTES,
        ffmpeg_source: FfmpegSource::Bundled,
        shortcut: "F9".into(),
    }
}
