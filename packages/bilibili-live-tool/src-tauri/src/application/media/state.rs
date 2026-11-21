use std::{
    collections::{HashSet, VecDeque},
    path::{Path, PathBuf},
    sync::Arc,
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
struct RecordingReconnectWatchdog {
    recording: Arc<RecordingRuntime>,
    play_url: Arc<dyn PlayUrlResolver>,
    ffmpeg: Arc<dyn FfmpegLocator>,
    granted_executable: Option<PathBuf>,
    ffmpeg_source: FfmpegSource,
    room_id: String,
    directory: PathBuf,
    quality: RecordingQuality,
    format: RecordingFormat,
}

impl RecordingReconnectWatchdog {
    async fn run(self) {
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
                log::warn!("recording stream lost after {attempts} attempts");
                self.recording.fail("recording_stream_lost").await;
                return;
            }
            match self.restart(attempts).await {
                Ok(()) => log::info!("recording stream reconnected (attempt {attempts})"),
                Err(error) => {
                    log::warn!("recording reconnect {attempts} failed: {}", error.code);
                    if attempts >= RECORDING_MAX_RECONNECTS {
                        self.recording.fail("recording_stream_lost").await;
                        return;
                    }
                }
            }
        }
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
        .map_err(|error| MediaIoError::coded("recording_pipeline_invalid", error.to_string(), false))?;
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

pub struct MediaServices {
    pub repository: Arc<dyn MediaRepository>,
    pub files: Arc<dyn MediaFilePort>,
    pub preview: Arc<dyn PreviewPort>,
    pub grants: Arc<dyn PathGrantPort>,
    pub audio: Arc<dyn AudioOutputPort>,
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
    recording: Arc<RecordingRuntime>,
    accepted_events: Mutex<EventHistory>,
    services: Option<MediaServices>,
    preview_playbacks: Mutex<Vec<Box<dyn AudioPlayback>>>,
    local_ffmpeg_executable: RwLock<Option<PathBuf>>,
    event_sink: Arc<dyn MediaEventSink>,
    play_url: Arc<dyn PlayUrlResolver>,
    ffmpeg: Arc<dyn FfmpegLocator>,
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
            FfmpegSource::Bundled => None,
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
        let highlight = self
            .list_highlights()
            .await?
            .into_iter()
            .find(|highlight| highlight.id == id)
            .ok_or_else(|| MediaIoError::invalid("highlight does not exist"))?;
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
            MediaIoError::coded("recording_room_invalid", "请输入正确的直播间号", false)
        })?;
        let services = self.services()?;
        let config = self.recording.config().await;

        let grant = config.directory.as_ref().ok_or_else(|| {
            MediaIoError::coded(
                "recording_directory_required",
                "请先在录制设置中选择保存目录",
                false,
            )
        })?;
        let directory = services
            .grants
            .resolve(&grant.grant_id, PathPurpose::RecordingDirectory)
            .map_err(|_| {
                MediaIoError::coded(
                    "recording_directory_required",
                    "录制目录授权已失效，请重新选择",
                    false,
                )
            })?;
        tokio::fs::create_dir_all(&directory)
            .await
            .map_err(|_| {
                MediaIoError::coded(
                    "recording_directory_unavailable",
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
        .map_err(|error| MediaIoError::coded("recording_pipeline_invalid", error.to_string(), false))?;

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
        match self.recording.start(room_id.to_owned(), launch).await {
            Ok(()) => {
                let _ = self
                    .event_sink
                    .publish_recording(&self.recording.snapshot().await);
                let watchdog = RecordingReconnectWatchdog {
                    recording: self.recording.clone(),
                    play_url: self.play_url.clone(),
                    ffmpeg: self.ffmpeg.clone(),
                    granted_executable: granted_executable.clone(),
                    ffmpeg_source: config.ffmpeg_source,
                    room_id: room_id.to_owned(),
                    directory,
                    quality: config.quality,
                    format,
                };
                tokio::spawn(watchdog.run());
                Ok(())
            }
            Err(error) => {
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

    pub async fn stop_recording(&self) -> Result<(), MediaIoError> {
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
        self.event_sink
            .publish_recording(&self.recording.snapshot().await)
    }

    async fn persist_highlight(
        &self,
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
