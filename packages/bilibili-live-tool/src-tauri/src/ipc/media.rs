use tauri::{
    http::{header, Request, Response, StatusCode},
    AppHandle, Emitter, EventTarget, Manager, Runtime, State, UriSchemeContext, UriSchemeResponder,
};

use crate::{
    application::{
        media::{
            ActiveSpeechEngine, ByteRange, GiftRuntimeSnapshot, GiftSoundBindingRecord,
            HighlightRecord, HighlightTriggerType, MediaEventSink, MediaIoError,
            RecordingItemState, RecordingRecord, TtsRuntimeSnapshot,
        },
        paths::PathPurpose,
    },
    domain::media::{
        CaptainEffectConfig, FfmpegSource, GiftNotifyConfig, GiftNotifyMethods, GiftSoundPolicy,
        MediaPosition, MediaSoundBinding, RecordingConfig, RecordingDirectoryGrant,
        RecordingFormat, RecordingQuality, RecordingState, TtsConfig, TtsEventKind,
        TtsIdentityKind, TtsSelectedEngine, TtsTemplates,
    },
    ipc::{dto::*, error::DesktopError},
    AppState,
};

const PREVIEW_CHUNK_BYTES: u64 = 8 * 1024 * 1024;

pub fn handle_media_preview_protocol<R: Runtime>(
    context: UriSchemeContext<'_, R>,
    request: Request<Vec<u8>>,
    responder: UriSchemeResponder,
) {
    let token = request.uri().path().trim_start_matches('/').to_owned();
    let owner = context.webview_label().to_owned();
    let app = context.app_handle().clone();
    let range = parse_range(request.headers().get(header::RANGE));
    tauri::async_runtime::spawn(async move {
        let response = match range {
            Ok(range) if !token.is_empty() && owner == "main" => app
                .state::<AppState>()
                .media()
                .read_preview(&token, &owner, Some(range), now_millis())
                .await
                .map(preview_response)
                .unwrap_or_else(|_| empty_response(StatusCode::NOT_FOUND)),
            Ok(_) => empty_response(StatusCode::FORBIDDEN),
            Err(()) => empty_response(StatusCode::RANGE_NOT_SATISFIABLE),
        };
        responder.respond(response);
    });
}

fn parse_range(value: Option<&tauri::http::HeaderValue>) -> Result<ByteRange, ()> {
    let Some(value) = value else {
        return Ok(ByteRange {
            start: 0,
            end_inclusive: PREVIEW_CHUNK_BYTES - 1,
        });
    };
    let value = value.to_str().map_err(|_| ())?;
    let range = value.strip_prefix("bytes=").ok_or(())?;
    if range.contains(',') {
        return Err(());
    }
    let (start, end) = range.split_once('-').ok_or(())?;
    let start = start.parse::<u64>().map_err(|_| ())?;
    let end_inclusive = if end.is_empty() {
        start.checked_add(PREVIEW_CHUNK_BYTES - 1).ok_or(())?
    } else {
        end.parse::<u64>().map_err(|_| ())?
    };
    if end_inclusive < start || end_inclusive.saturating_sub(start) >= PREVIEW_CHUNK_BYTES {
        return Err(());
    }
    Ok(ByteRange {
        start,
        end_inclusive,
    })
}

fn preview_response(read: crate::application::media::PreviewRead) -> Response<Vec<u8>> {
    Response::builder()
        .status(StatusCode::PARTIAL_CONTENT)
        .header(header::ACCEPT_RANGES, "bytes")
        .header(header::CONTENT_TYPE, "application/octet-stream")
        .header(header::CONTENT_LENGTH, read.bytes.len().to_string())
        .header(
            header::CONTENT_RANGE,
            format!(
                "bytes {}-{}/{}",
                read.range.start, read.range.end_inclusive, read.total_size
            ),
        )
        .body(read.bytes)
        .unwrap_or_else(|_| empty_response(StatusCode::INTERNAL_SERVER_ERROR))
}

fn empty_response(status: StatusCode) -> Response<Vec<u8>> {
    let mut response = Response::new(Vec::new());
    *response.status_mut() = status;
    response
}

pub struct TauriMediaEventSink {
    app: AppHandle,
}

impl TauriMediaEventSink {
    pub fn new(app: AppHandle) -> Self {
        Self { app }
    }

    fn emit_main<T: serde::Serialize + Clone>(
        &self,
        event: &str,
        payload: T,
    ) -> Result<(), MediaIoError> {
        self.app
            .emit_to(EventTarget::webview_window("main"), event, payload)
            .map_err(|_| MediaIoError::operation("media event delivery failed", true))
    }

    fn emit_effect<T: serde::Serialize + Clone>(
        &self,
        event: &str,
        payload: T,
    ) -> Result<(), MediaIoError> {
        self.app
            .emit_to(
                EventTarget::webview_window("effect-captain"),
                event,
                payload,
            )
            .map_err(|_| MediaIoError::operation("effect event delivery failed", true))
    }
}

impl MediaEventSink for TauriMediaEventSink {
    fn publish_captain_effect(
        &self,
        event: &crate::domain::live_data::LiveEvent,
        config: &CaptainEffectConfig,
    ) -> Result<(), MediaIoError> {
        let crate::domain::live_data::LiveEvent::Captain {
            base,
            action,
            level,
            ..
        } = event
        else {
            return Ok(());
        };
        self.emit_effect(
            "captain_effect_trigger",
            CaptainEffectPayloadDto {
                event_id: base.event_id.clone(),
                audience: audience_identity(&base.audience),
                action: captain_action_dto(*action),
                level: captain_level_dto(*level),
                config: captain_config(config.clone()),
                is_preview: false,
            },
        )
    }

    fn publish_gift(
        &self,
        event: &crate::domain::live_data::LiveEvent,
        config: &GiftNotifyConfig,
        snapshot: &GiftRuntimeSnapshot,
    ) -> Result<(), MediaIoError> {
        let Some(event) = gift_event_dto(event) else {
            return Ok(());
        };
        if config.is_enabled
            && config.methods.local_popup
            && event.value_minor >= config.high_value_threshold_minor
        {
            self.emit_main(
                "gift_alert_trigger",
                GiftAlertPayloadDto {
                    event: event.clone(),
                    popup_position: position(config.popup_position),
                    popup_duration_seconds: config.popup_duration_seconds,
                    is_preview: false,
                },
            )?;
        }
        self.emit_main(
            "gift_notify_updated",
            gift_snapshot_from(config.clone(), snapshot.clone()),
        )
    }

    fn publish_tts(&self, snapshot: &TtsRuntimeSnapshot) -> Result<(), MediaIoError> {
        self.emit_main("tts_status_changed", tts_status(snapshot.clone()))
    }

    fn publish_recording(
        &self,
        snapshot: &crate::application::media::RecordingRuntimeSnapshot,
    ) -> Result<(), MediaIoError> {
        self.emit_main(
            "recording_status_changed",
            recording_status_dto(snapshot.clone()),
        )
    }

    fn publish_recording_completed(&self, recording: &RecordingRecord) -> Result<(), MediaIoError> {
        self.emit_main("recording_completed", recording_summary(recording.clone()))
    }

    fn publish_highlight(&self, highlight: &HighlightRecord) -> Result<(), MediaIoError> {
        self.emit_main("highlight_created", highlight_summary(highlight.clone()))
    }
}

#[tauri::command]
pub async fn captain_effect_get(
    state: State<'_, AppState>,
) -> Result<CaptainEffectSnapshotDto, DesktopError> {
    Ok(captain_snapshot(
        state.media().effects().config().await,
        now_millis(),
    ))
}

#[tauri::command]
pub async fn captain_effect_save(
    state: State<'_, AppState>,
    request: CaptainEffectConfigRequestDto,
) -> Result<CaptainEffectSnapshotDto, DesktopError> {
    let config = captain_from_dto(state.media(), request.config).await?;
    state
        .media()
        .save_captain_config(config)
        .await
        .map(|config| captain_snapshot(config, now_millis()))
        .map_err(Into::into)
}

#[tauri::command]
pub async fn captain_effect_preview(
    state: State<'_, AppState>,
    request: CaptainEffectConfigRequestDto,
) -> Result<QueuedResultDto, DesktopError> {
    let config = captain_from_dto(state.media(), request.config).await?;
    crate::domain::media::validate_captain_effect_config(&config)
        .map_err(|error| DesktopError::from(MediaIoError::invalid(error.to_string())))?;
    state
        .media()
        .effects()
        .preview(config, format!("preview-{}", now_millis()))
        .await;
    Ok(QueuedResultDto { queued: true })
}

#[tauri::command]
pub async fn gift_notify_get(
    state: State<'_, AppState>,
) -> Result<GiftNotifySnapshotDto, DesktopError> {
    Ok(gift_snapshot(state.media()).await)
}

#[tauri::command]
pub async fn gift_notify_save(
    state: State<'_, AppState>,
    request: GiftNotifyConfigRequestDto,
) -> Result<GiftNotifySnapshotDto, DesktopError> {
    state
        .media()
        .save_gift_notify_config(gift_notify_from_dto(request.config))
        .await?;
    Ok(gift_snapshot(state.media()).await)
}

#[tauri::command]
pub async fn gift_notify_test(
    state: State<'_, AppState>,
    request: GiftNotifyConfigRequestDto,
) -> Result<TriggeredResultDto, DesktopError> {
    let config = gift_notify_from_dto(request.config);
    crate::domain::media::validate_gift_notify_config(&config)
        .map_err(|error| DesktopError::from(MediaIoError::invalid(error.to_string())))?;
    state
        .media()
        .gifts()
        .preview(config, format!("preview-{}", now_millis()))
        .await;
    Ok(TriggeredResultDto { triggered: true })
}

#[tauri::command]
pub async fn gift_sound_list(
    state: State<'_, AppState>,
) -> Result<GiftSoundSnapshotDto, DesktopError> {
    Ok(gift_sound_snapshot(state.media()).await)
}

#[tauri::command]
pub async fn gift_sound_set(
    state: State<'_, AppState>,
    request: GiftSoundSetRequestDto,
) -> Result<GiftSoundBindingDto, DesktopError> {
    state
        .media()
        .set_gift_sound(request.gift_id, request.gift_name, &request.grant_id)
        .await
        .map(gift_sound_binding)
        .map_err(Into::into)
}

#[tauri::command]
pub async fn gift_sound_remove(
    state: State<'_, AppState>,
    request: GiftIdRequestDto,
) -> Result<DeleteResultDto, DesktopError> {
    state
        .media()
        .remove_gift_sound(&request.gift_id)
        .await
        .map(|deleted| DeleteResultDto { deleted })
        .map_err(Into::into)
}

#[tauri::command]
pub async fn gift_sound_save(
    state: State<'_, AppState>,
    request: GiftSoundSaveRequestDto,
) -> Result<GiftSoundSnapshotDto, DesktopError> {
    let current = state.media().gifts().bindings().await;
    let bindings = request
        .bindings
        .into_iter()
        .map(|dto| gift_sound_from_dto(dto, &current))
        .collect::<Result<Vec<_>, _>>()?;
    let policy = GiftSoundPolicy {
        global_volume: request.global_volume,
        queue_capacity_per_second: state
            .media()
            .gifts()
            .sound_policy()
            .await
            .queue_capacity_per_second,
    };
    state.media().save_gift_sounds(policy, bindings).await?;
    Ok(gift_sound_snapshot(state.media()).await)
}

#[tauri::command]
pub async fn gift_sound_import(
    state: State<'_, AppState>,
    request: GrantRequestDto,
) -> Result<ImportedCountDto, DesktopError> {
    let count = state.media().import_gift_sounds(&request.grant_id).await?;
    Ok(ImportedCountDto {
        imported_count: count.try_into().unwrap_or(u32::MAX),
    })
}

#[tauri::command]
pub async fn gift_sound_export(
    state: State<'_, AppState>,
    request: GrantRequestDto,
) -> Result<ExportedCountDto, DesktopError> {
    let display = state
        .media()
        .resolve_grant(&request.grant_id, PathPurpose::GiftSoundExport)?;
    let count = state.media().export_gift_sounds(&request.grant_id).await?;
    Ok(ExportedCountDto {
        exported_count: count.try_into().unwrap_or(u32::MAX),
        display_path: display.to_string_lossy().into_owned(),
    })
}

#[tauri::command]
pub async fn sound_preview(
    state: State<'_, AppState>,
    request: SoundPreviewRequestDto,
) -> Result<StartedResultDto, DesktopError> {
    let source = match request.target {
        SoundPreviewTargetDto::Binding { binding_id } => {
            state.media().sound_source(&binding_id).await?
        }
        SoundPreviewTargetDto::Grant { grant_id } => state
            .media()
            .resolve_grant(&grant_id, PathPurpose::GiftSoundFile)?,
    };
    state.media().preview_sound(source, request.volume).await?;
    Ok(StartedResultDto { started: true })
}

#[tauri::command]
pub async fn tts_get(state: State<'_, AppState>) -> Result<TtsSnapshotDto, DesktopError> {
    Ok(tts_snapshot(state.media()).await)
}

#[tauri::command]
pub async fn tts_save(
    state: State<'_, AppState>,
    request: TtsConfigRequestDto,
) -> Result<TtsSnapshotDto, DesktopError> {
    state
        .media()
        .save_tts_config(tts_from_dto(request.config))
        .await?;
    Ok(tts_snapshot(state.media()).await)
}

#[tauri::command]
pub async fn tts_set_enabled(
    state: State<'_, AppState>,
    request: TtsEnabledRequestDto,
) -> Result<TtsStatusDto, DesktopError> {
    let mut config = state.media().tts().config().await;
    config.is_enabled = request.is_enabled;
    state.media().save_tts_config(config).await?;
    Ok(tts_status(state.media().tts().snapshot().await))
}

#[tauri::command]
pub async fn tts_test(
    state: State<'_, AppState>,
    request: TtsConfigRequestDto,
) -> Result<TtsStatusDto, DesktopError> {
    let config = tts_from_dto(request.config);
    crate::domain::media::validate_tts_config(&config)
        .map_err(|error| DesktopError::from(MediaIoError::invalid(error.to_string())))?;
    state
        .media()
        .tts()
        .test(&config, "用户A说：今天直播真不错".into())
        .await?;
    Ok(tts_status(state.media().tts().snapshot().await))
}

#[tauri::command]
pub async fn tts_pause(
    state: State<'_, AppState>,
    request: TtsPauseRequestDto,
) -> Result<TtsStatusDto, DesktopError> {
    state.media().tts().set_paused(request.is_paused).await;
    Ok(tts_status(state.media().tts().snapshot().await))
}

#[tauri::command]
pub async fn tts_clear(state: State<'_, AppState>) -> Result<TtsStatusDto, DesktopError> {
    state.media().tts().clear_pending().await;
    Ok(tts_status(state.media().tts().snapshot().await))
}

#[tauri::command]
pub async fn recording_get_config(
    state: State<'_, AppState>,
) -> Result<RecordingConfigDto, DesktopError> {
    Ok(recording_config(state.media().recording().config().await))
}

#[tauri::command]
pub async fn recording_save_config(
    state: State<'_, AppState>,
    request: RecordingConfigRequestDto,
) -> Result<RecordingConfigDto, DesktopError> {
    state
        .media()
        .save_recording_config(recording_from_dto(request.config))
        .await
        .map(recording_config)
        .map_err(Into::into)
}

#[tauri::command]
pub async fn recording_set_ffmpeg_source(
    state: State<'_, AppState>,
    request: RecordingFfmpegSourceRequestDto,
) -> Result<RecordingConfigDto, DesktopError> {
    state
        .media()
        .set_ffmpeg_source(
            ffmpeg_source_from_dto(request.source),
            request.grant_id.as_deref(),
        )
        .await
        .map(recording_config)
        .map_err(Into::into)
}

#[tauri::command]
pub async fn recording_status(
    state: State<'_, AppState>,
) -> Result<RecordingStatusDto, DesktopError> {
    Ok(recording_status_dto(
        state.media().recording().snapshot().await,
    ))
}

#[tauri::command]
pub async fn recording_start(
    _: State<'_, AppState>,
    request: RecordingStartRequestDto,
) -> Result<RecordingStatusDto, DesktopError> {
    crate::domain::validation::RoomId::parse(&request.room_id)
        .map_err(|_| DesktopError::from(MediaIoError::invalid("recording room is invalid")))?;
    Err(MediaIoError::unavailable("live stream resolver is unavailable").into())
}

#[tauri::command]
pub async fn recording_stop(
    state: State<'_, AppState>,
) -> Result<RecordingStatusDto, DesktopError> {
    state.media().stop_recording().await?;
    recording_status(state).await
}

#[tauri::command]
pub async fn recording_list(
    state: State<'_, AppState>,
) -> Result<Vec<RecordingSummaryDto>, DesktopError> {
    state
        .media()
        .list_recordings()
        .await
        .map(|items| items.into_iter().map(recording_summary).collect())
        .map_err(Into::into)
}

#[tauri::command]
pub async fn recording_preview(
    state: State<'_, AppState>,
    request: RecordingIdRequestDto,
) -> Result<PreviewSessionDto, DesktopError> {
    let id = positive_id(&request.recording_id, "recording")?;
    let recording = state
        .media()
        .list_recordings()
        .await?
        .into_iter()
        .find(|recording| recording.id == id)
        .ok_or_else(|| DesktopError::from(MediaIoError::invalid("recording does not exist")))?;
    Ok(preview_session(state.media().issue_preview(
        recording.file_path,
        "main",
        now_millis(),
    )?))
}

#[tauri::command]
pub async fn recording_rename(
    state: State<'_, AppState>,
    request: RecordingRenameRequestDto,
) -> Result<RecordingSummaryDto, DesktopError> {
    let id = positive_id(&request.recording_id, "recording")?;
    state
        .media()
        .rename_recording(id, &request.file_name_stem)
        .await
        .map(recording_summary)
        .map_err(Into::into)
}

#[tauri::command]
pub async fn recording_delete(
    state: State<'_, AppState>,
    request: RecordingIdRequestDto,
) -> Result<DeleteResultDto, DesktopError> {
    let id = positive_id(&request.recording_id, "recording")?;
    state
        .media()
        .delete_recording(id)
        .await
        .map(|deleted| DeleteResultDto { deleted })
        .map_err(Into::into)
}

#[tauri::command]
pub async fn highlight_list(
    state: State<'_, AppState>,
) -> Result<Vec<HighlightSummaryDto>, DesktopError> {
    state
        .media()
        .list_highlights()
        .await
        .map(|items| items.into_iter().map(highlight_summary).collect())
        .map_err(Into::into)
}

#[tauri::command]
pub async fn highlight_mark(
    state: State<'_, AppState>,
) -> Result<HighlightSummaryDto, DesktopError> {
    state
        .media()
        .mark_highlight()
        .await
        .map(highlight_summary)
        .map_err(Into::into)
}

#[tauri::command]
pub async fn highlight_preview(
    state: State<'_, AppState>,
    request: HighlightIdRequestDto,
) -> Result<PreviewSessionDto, DesktopError> {
    let id = positive_id(&request.highlight_id, "highlight")?;
    let highlight = state
        .media()
        .list_highlights()
        .await?
        .into_iter()
        .find(|highlight| highlight.id == id)
        .and_then(|highlight| highlight.file_path)
        .ok_or_else(|| {
            DesktopError::from(MediaIoError::unavailable("highlight clip is unavailable"))
        })?;
    Ok(preview_session(state.media().issue_preview(
        highlight,
        "main",
        now_millis(),
    )?))
}

#[tauri::command]
pub async fn highlight_export(
    state: State<'_, AppState>,
    request: HighlightExportRequestDto,
) -> Result<ExportedResultDto, DesktopError> {
    let id = positive_id(&request.highlight_id, "highlight")?;
    let target = state
        .media()
        .resolve_grant(&request.grant_id, PathPurpose::HighlightExport)?;
    let exported = state.media().export_highlight(id, target).await?;
    Ok(ExportedResultDto {
        exported: true,
        display_path: exported.to_string_lossy().into_owned(),
    })
}

#[tauri::command]
pub async fn highlight_delete(
    state: State<'_, AppState>,
    request: HighlightIdRequestDto,
) -> Result<DeleteResultDto, DesktopError> {
    let id = positive_id(&request.highlight_id, "highlight")?;
    state
        .media()
        .delete_highlight(id)
        .await
        .map(|deleted| DeleteResultDto { deleted })
        .map_err(Into::into)
}

async fn captain_from_dto(
    media: &crate::application::media::MediaState,
    dto: CaptainEffectConfigDto,
) -> Result<CaptainEffectConfig, DesktopError> {
    let current = media.effects().config().await;
    Ok(CaptainEffectConfig {
        preset_id: dto.preset_id,
        duration_seconds: dto.duration_seconds,
        position: position_from_dto(dto.position),
        primary_color: dto.primary_color,
        secondary_color: dto.secondary_color,
        first_sound: sound_from_dto(media, dto.first_sound, current.first_sound.as_ref())?,
        renewal_sound: sound_from_dto(media, dto.renewal_sound, current.renewal_sound.as_ref())?,
        combo_sound: sound_from_dto(media, dto.combo_sound, current.combo_sound.as_ref())?,
        volume: dto.volume,
        combo_enabled: dto.combo_enabled,
        combo_window_seconds: dto.combo_window_seconds,
    })
}

fn sound_from_dto(
    media: &crate::application::media::MediaState,
    dto: Option<SoundBindingSummaryDto>,
    current: Option<&MediaSoundBinding>,
) -> Result<Option<MediaSoundBinding>, DesktopError> {
    let Some(dto) = dto else {
        return Ok(None);
    };
    if let Some(current) = current.filter(|binding| binding.binding_id == dto.binding_id) {
        return Ok(Some(current.clone()));
    }
    let source = media.resolve_grant(&dto.binding_id, PathPurpose::GiftSoundFile)?;
    Ok(Some(MediaSoundBinding {
        binding_id: dto.binding_id,
        file_name: dto.file_name,
        source,
    }))
}

fn captain_snapshot(config: CaptainEffectConfig, updated_at: i64) -> CaptainEffectSnapshotDto {
    CaptainEffectSnapshotDto {
        config: captain_config(config),
        presets: Vec::new(),
        updated_at,
    }
}

fn captain_config(config: CaptainEffectConfig) -> CaptainEffectConfigDto {
    CaptainEffectConfigDto {
        preset_id: config.preset_id,
        duration_seconds: config.duration_seconds,
        position: position(config.position),
        primary_color: config.primary_color,
        secondary_color: config.secondary_color,
        first_sound: sound_summary(config.first_sound),
        renewal_sound: sound_summary(config.renewal_sound),
        combo_sound: sound_summary(config.combo_sound),
        volume: config.volume,
        combo_enabled: config.combo_enabled,
        combo_window_seconds: config.combo_window_seconds,
    }
}

fn sound_summary(binding: Option<MediaSoundBinding>) -> Option<SoundBindingSummaryDto> {
    binding.map(|binding| SoundBindingSummaryDto {
        binding_id: binding.binding_id,
        file_name: binding.file_name,
        is_missing: !binding.source.is_file(),
    })
}

async fn gift_snapshot(media: &crate::application::media::MediaState) -> GiftNotifySnapshotDto {
    let config = media.gifts().notify_config().await;
    let runtime = media.gifts().snapshot().await;
    gift_snapshot_from(config, runtime)
}

fn gift_snapshot_from(
    config: GiftNotifyConfig,
    runtime: GiftRuntimeSnapshot,
) -> GiftNotifySnapshotDto {
    GiftNotifySnapshotDto {
        config: gift_notify_config(config),
        total_value_minor: runtime.stats.total_value_minor,
        donor_count: runtime
            .stats
            .donor_uids
            .len()
            .try_into()
            .unwrap_or(u32::MAX),
        gift_count: runtime.stats.gift_count,
        latest_gifts: runtime.stats.latest.into_iter().map(gift_record).collect(),
        updated_at: now_millis(),
    }
}

fn gift_notify_config(config: GiftNotifyConfig) -> GiftNotifyConfigDto {
    GiftNotifyConfigDto {
        is_enabled: config.is_enabled,
        high_value_threshold_minor: config.high_value_threshold_minor,
        methods: GiftNotifyMethodsDto {
            system_notification: config.methods.system_notification,
            local_popup: config.methods.local_popup,
            sound: config.methods.sound,
        },
        combo_enabled: config.combo_enabled,
        combo_window_seconds: config.combo_window_seconds,
        popup_position: position(config.popup_position),
        popup_duration_seconds: config.popup_duration_seconds,
    }
}

fn gift_notify_from_dto(config: GiftNotifyConfigDto) -> GiftNotifyConfig {
    GiftNotifyConfig {
        is_enabled: config.is_enabled,
        high_value_threshold_minor: config.high_value_threshold_minor,
        methods: GiftNotifyMethods {
            system_notification: config.methods.system_notification,
            local_popup: config.methods.local_popup,
            sound: config.methods.sound,
        },
        combo_enabled: config.combo_enabled,
        combo_window_seconds: config.combo_window_seconds,
        popup_position: position_from_dto(config.popup_position),
        popup_duration_seconds: config.popup_duration_seconds,
    }
}

async fn gift_sound_snapshot(
    media: &crate::application::media::MediaState,
) -> GiftSoundSnapshotDto {
    let policy = media.gifts().sound_policy().await;
    GiftSoundSnapshotDto {
        bindings: media
            .gifts()
            .bindings()
            .await
            .into_iter()
            .map(gift_sound_binding)
            .collect(),
        global_volume: policy.global_volume,
        queue_capacity_per_second: policy.queue_capacity_per_second,
        updated_at: now_millis(),
    }
}

fn gift_sound_binding(binding: GiftSoundBindingRecord) -> GiftSoundBindingDto {
    let is_missing = binding
        .sound_path
        .as_ref()
        .is_some_and(|source| !source.is_file());
    GiftSoundBindingDto {
        gift_id: binding.gift_id,
        gift_name: binding.gift_name,
        sound_file_name: binding
            .sound_path
            .as_ref()
            .and_then(|source| source.file_name())
            .map(|name| name.to_string_lossy().into_owned()),
        volume: binding.volume,
        is_enabled: binding.is_enabled,
        is_missing,
    }
}

fn gift_sound_from_dto(
    dto: GiftSoundBindingDto,
    current: &[GiftSoundBindingRecord],
) -> Result<GiftSoundBindingRecord, DesktopError> {
    let sound_path = current
        .iter()
        .find(|binding| binding.gift_id == dto.gift_id)
        .and_then(|binding| binding.sound_path.clone());
    if dto.sound_file_name.is_some() && sound_path.is_none() {
        return Err(MediaIoError::invalid("gift sound binding is not authorized").into());
    }
    Ok(GiftSoundBindingRecord {
        gift_id: dto.gift_id,
        gift_name: dto.gift_name,
        sound_path,
        volume: dto.volume,
        is_enabled: dto.is_enabled,
    })
}

async fn tts_snapshot(media: &crate::application::media::MediaState) -> TtsSnapshotDto {
    let config = media.tts().config().await;
    let status = media.tts().snapshot().await;
    let mut voices = Vec::new();
    for (engine, result) in media.tts().voices().await {
        let dto_engine = match engine {
            ActiveSpeechEngine::System => Some(TtsSelectedEngineDto::System),
            ActiveSpeechEngine::Cloud => Some(TtsSelectedEngineDto::Cloud),
            ActiveSpeechEngine::LocalFallback => None,
        };
        if let (Some(engine), Ok(items)) = (dto_engine, result) {
            voices.extend(items.into_iter().map(|voice| TtsVoiceDto {
                voice_id: voice.id,
                display_name: voice.display_name,
                engine,
                is_available: true,
                unavailable_reason_code: None,
            }));
        }
    }
    TtsSnapshotDto {
        config: tts_config(config),
        voices,
        status: tts_status(status),
        updated_at: now_millis(),
    }
}

fn tts_config(config: TtsConfig) -> TtsConfigDto {
    TtsConfigDto {
        is_enabled: config.is_enabled,
        selected_engine: match config.selected_engine {
            TtsSelectedEngine::System => TtsSelectedEngineDto::System,
            TtsSelectedEngine::Cloud => TtsSelectedEngineDto::Cloud,
        },
        selected_voice_id: config.selected_voice_id,
        rate: config.rate,
        volume: config.volume,
        event_kinds: config.event_kinds.into_iter().map(tts_event_kind).collect(),
        identity_kinds: config
            .identity_kinds
            .into_iter()
            .map(tts_identity_kind)
            .collect(),
        templates: TtsTemplatesDto {
            danmu: config.templates.danmu,
            gift: config.templates.gift,
            captain: config.templates.captain,
        },
    }
}

fn tts_from_dto(config: TtsConfigDto) -> TtsConfig {
    TtsConfig {
        is_enabled: config.is_enabled,
        selected_engine: match config.selected_engine {
            TtsSelectedEngineDto::System => TtsSelectedEngine::System,
            TtsSelectedEngineDto::Cloud => TtsSelectedEngine::Cloud,
        },
        selected_voice_id: config.selected_voice_id,
        rate: config.rate,
        volume: config.volume,
        event_kinds: config
            .event_kinds
            .into_iter()
            .map(tts_event_kind_from_dto)
            .collect(),
        identity_kinds: config
            .identity_kinds
            .into_iter()
            .map(tts_identity_kind_from_dto)
            .collect(),
        templates: TtsTemplates {
            danmu: config.templates.danmu,
            gift: config.templates.gift,
            captain: config.templates.captain,
        },
    }
}

fn tts_status(status: TtsRuntimeSnapshot) -> TtsStatusDto {
    let state = if status.is_paused {
        TtsStateDto::Paused
    } else if status.current_text.is_some() {
        TtsStateDto::Speaking
    } else if status.fallback_reason_code == Some("tts_unavailable") {
        TtsStateDto::Unavailable
    } else if status.fallback_reason_code.is_some() {
        TtsStateDto::Degraded
    } else {
        TtsStateDto::Idle
    };
    TtsStatusDto {
        state,
        current_text: status.current_text,
        pending_count: status.pending_count.try_into().unwrap_or(u32::MAX),
        active_engine: match status.active_engine {
            ActiveSpeechEngine::System => ActiveTtsEngineDto::System,
            ActiveSpeechEngine::Cloud => ActiveTtsEngineDto::Cloud,
            ActiveSpeechEngine::LocalFallback => ActiveTtsEngineDto::LocalFallback,
        },
        fallback_reason_code: status.fallback_reason_code.map(str::to_owned),
    }
}

fn recording_config(config: RecordingConfig) -> RecordingConfigDto {
    RecordingConfigDto {
        format: recording_format(config.format),
        quality: recording_quality(config.quality),
        directory: config.directory.map(|directory| RecordingDirectoryDto {
            grant_id: directory.grant_id,
            display_path: directory.display_path,
        }),
        segmentation_enabled: config.segmentation_enabled,
        segment_duration_seconds: config.segment_duration_seconds,
        auto_record: config.auto_record,
        danmaku_threshold: config.danmaku_threshold,
        highlight_before_seconds: config.highlight_before_seconds,
        highlight_after_seconds: config.highlight_after_seconds,
        quota_bytes: config.quota_bytes,
        waterline_bytes: config.waterline_bytes,
        ffmpeg_source: match config.ffmpeg_source {
            FfmpegSource::Bundled => FfmpegSourceDto::Bundled,
            FfmpegSource::Local => FfmpegSourceDto::Local,
        },
        shortcut: config.shortcut,
    }
}

fn recording_from_dto(config: RecordingConfigDto) -> RecordingConfig {
    RecordingConfig {
        format: recording_format_from_dto(config.format),
        quality: recording_quality_from_dto(config.quality),
        directory: config.directory.map(|directory| RecordingDirectoryGrant {
            grant_id: directory.grant_id,
            display_path: directory.display_path,
        }),
        segmentation_enabled: config.segmentation_enabled,
        segment_duration_seconds: config.segment_duration_seconds,
        auto_record: config.auto_record,
        danmaku_threshold: config.danmaku_threshold,
        highlight_before_seconds: config.highlight_before_seconds,
        highlight_after_seconds: config.highlight_after_seconds,
        quota_bytes: config.quota_bytes,
        waterline_bytes: config.waterline_bytes,
        ffmpeg_source: ffmpeg_source_from_dto(config.ffmpeg_source),
        shortcut: config.shortcut,
    }
}

fn recording_status_dto(
    status: crate::application::media::RecordingRuntimeSnapshot,
) -> RecordingStatusDto {
    RecordingStatusDto {
        state: recording_state(status.state),
        session_id: None,
        room_id: status.room_id,
        elapsed_seconds: status.elapsed_seconds,
        file_size_bytes: 0,
        disk_remaining_bytes: status.disk_remaining_bytes.unwrap_or(0),
        highlight_count: status.highlights.len().try_into().unwrap_or(u32::MAX),
        last_error_code: status.failure_code.map(str::to_owned),
    }
}

fn recording_summary(recording: RecordingRecord) -> RecordingSummaryDto {
    RecordingSummaryDto {
        id: recording.id.to_string(),
        file_name: recording
            .file_path
            .file_name()
            .map_or_else(String::new, |name| name.to_string_lossy().into_owned()),
        format: recording_format(recording.format),
        size_bytes: recording.size_bytes,
        duration_seconds: recording.duration_seconds,
        started_at: recording.started_at,
        ended_at: recording.ended_at,
        status: match recording.status {
            RecordingItemState::Recording => RecordingItemStateDto::Recording,
            RecordingItemState::Completed => RecordingItemStateDto::Completed,
            RecordingItemState::Partial => RecordingItemStateDto::Partial,
            RecordingItemState::Failed => RecordingItemStateDto::Failed,
        },
    }
}

fn highlight_summary(highlight: HighlightRecord) -> HighlightSummaryDto {
    HighlightSummaryDto {
        id: highlight.id.to_string(),
        recording_id: highlight.recording_id.to_string(),
        position_seconds: highlight.position_seconds,
        trigger_type: match highlight.trigger_type {
            HighlightTriggerType::DanmakuSurge => HighlightTriggerTypeDto::DanmakuSurge,
            HighlightTriggerType::Manual => HighlightTriggerTypeDto::Manual,
        },
        clip_duration_seconds: highlight.clip_duration_seconds,
        created_at: highlight.created_at,
        is_clip_available: highlight.is_clip_available,
    }
}

fn preview_session(session: crate::application::media::PreviewSession) -> PreviewSessionDto {
    PreviewSessionDto {
        session_id: session.token,
        url: session.url,
        expires_at: session.expires_at,
    }
}

fn gift_record(record: crate::domain::media::GiftRecord) -> GiftEventDto {
    GiftEventDto {
        kind: "gift".into(),
        event_id: record.event_id,
        room_id: record.room_id,
        occurred_at: record.occurred_at,
        audience: AudienceIdentityDto {
            user_uid: record.user_uid,
            username: record.username,
            user_level: None,
            medal: None,
            tags: Vec::new(),
        },
        gift_id: record.gift_id,
        gift_name: record.gift_name,
        quantity: record.quantity,
        value_minor: record.value_minor,
    }
}

fn gift_event_dto(event: &crate::domain::live_data::LiveEvent) -> Option<GiftEventDto> {
    let crate::domain::live_data::LiveEvent::Gift {
        base,
        gift_id,
        gift_name,
        quantity,
        value_minor,
    } = event
    else {
        return None;
    };
    Some(GiftEventDto {
        kind: "gift".into(),
        event_id: base.event_id.clone(),
        room_id: base.room_id.as_str().to_owned(),
        occurred_at: base.occurred_at,
        audience: audience_identity(&base.audience),
        gift_id: gift_id.clone(),
        gift_name: gift_name.clone(),
        quantity: *quantity,
        value_minor: *value_minor,
    })
}

fn audience_identity(audience: &crate::domain::live_data::AudienceIdentity) -> AudienceIdentityDto {
    AudienceIdentityDto {
        user_uid: audience.user_uid.as_str().to_owned(),
        username: audience.username.clone(),
        user_level: audience.user_level,
        medal: audience.medal.as_ref().map(|medal| FanMedalDto {
            name: medal.name.clone(),
            level: medal.level,
        }),
        tags: audience
            .tags
            .iter()
            .copied()
            .map(|tag| match tag {
                crate::domain::live_data::IdentityTag::Admin => IdentityTagDto::Admin,
                crate::domain::live_data::IdentityTag::Governor => IdentityTagDto::Governor,
                crate::domain::live_data::IdentityTag::Admiral => IdentityTagDto::Admiral,
                crate::domain::live_data::IdentityTag::Captain => IdentityTagDto::Captain,
                crate::domain::live_data::IdentityTag::Vip => IdentityTagDto::Vip,
            })
            .collect(),
    }
}

const fn captain_action_dto(action: crate::domain::live_data::CaptainAction) -> CaptainActionDto {
    match action {
        crate::domain::live_data::CaptainAction::Purchase => CaptainActionDto::Purchase,
        crate::domain::live_data::CaptainAction::Renewal => CaptainActionDto::Renewal,
    }
}

const fn captain_level_dto(level: crate::domain::live_data::CaptainLevel) -> CaptainLevelDto {
    match level {
        crate::domain::live_data::CaptainLevel::Governor => CaptainLevelDto::Governor,
        crate::domain::live_data::CaptainLevel::Admiral => CaptainLevelDto::Admiral,
        crate::domain::live_data::CaptainLevel::Captain => CaptainLevelDto::Captain,
    }
}

fn positive_id(value: &str, name: &str) -> Result<i64, DesktopError> {
    value
        .parse::<i64>()
        .ok()
        .filter(|id| *id > 0)
        .ok_or_else(|| MediaIoError::invalid(format!("{name} id is invalid")).into())
}

const fn position(position: MediaPosition) -> MediaPositionDto {
    match position {
        MediaPosition::TopLeft => MediaPositionDto::TopLeft,
        MediaPosition::TopRight => MediaPositionDto::TopRight,
        MediaPosition::BottomLeft => MediaPositionDto::BottomLeft,
        MediaPosition::BottomRight => MediaPositionDto::BottomRight,
        MediaPosition::Center => MediaPositionDto::Center,
    }
}

const fn position_from_dto(position: MediaPositionDto) -> MediaPosition {
    match position {
        MediaPositionDto::TopLeft => MediaPosition::TopLeft,
        MediaPositionDto::TopRight => MediaPosition::TopRight,
        MediaPositionDto::BottomLeft => MediaPosition::BottomLeft,
        MediaPositionDto::BottomRight => MediaPosition::BottomRight,
        MediaPositionDto::Center => MediaPosition::Center,
    }
}

const fn tts_event_kind(kind: TtsEventKind) -> TtsEventKindDto {
    match kind {
        TtsEventKind::Danmu => TtsEventKindDto::Danmu,
        TtsEventKind::Gift => TtsEventKindDto::Gift,
        TtsEventKind::Captain => TtsEventKindDto::Captain,
        TtsEventKind::Entry => TtsEventKindDto::Entry,
    }
}

const fn tts_event_kind_from_dto(kind: TtsEventKindDto) -> TtsEventKind {
    match kind {
        TtsEventKindDto::Danmu => TtsEventKind::Danmu,
        TtsEventKindDto::Gift => TtsEventKind::Gift,
        TtsEventKindDto::Captain => TtsEventKind::Captain,
        TtsEventKindDto::Entry => TtsEventKind::Entry,
    }
}

const fn tts_identity_kind(kind: TtsIdentityKind) -> TtsIdentityKindDto {
    match kind {
        TtsIdentityKind::Admin => TtsIdentityKindDto::Admin,
        TtsIdentityKind::Captain => TtsIdentityKindDto::Captain,
        TtsIdentityKind::Vip => TtsIdentityKindDto::Vip,
        TtsIdentityKind::Ordinary => TtsIdentityKindDto::Ordinary,
    }
}

const fn tts_identity_kind_from_dto(kind: TtsIdentityKindDto) -> TtsIdentityKind {
    match kind {
        TtsIdentityKindDto::Admin => TtsIdentityKind::Admin,
        TtsIdentityKindDto::Captain => TtsIdentityKind::Captain,
        TtsIdentityKindDto::Vip => TtsIdentityKind::Vip,
        TtsIdentityKindDto::Ordinary => TtsIdentityKind::Ordinary,
    }
}

const fn recording_format(format: RecordingFormat) -> RecordingFormatDto {
    match format {
        RecordingFormat::Mp4 => RecordingFormatDto::Mp4,
        RecordingFormat::Flv => RecordingFormatDto::Flv,
    }
}

const fn recording_format_from_dto(format: RecordingFormatDto) -> RecordingFormat {
    match format {
        RecordingFormatDto::Mp4 => RecordingFormat::Mp4,
        RecordingFormatDto::Flv => RecordingFormat::Flv,
    }
}

const fn recording_quality(quality: RecordingQuality) -> RecordingQualityDto {
    match quality {
        RecordingQuality::Original => RecordingQualityDto::Original,
        RecordingQuality::High => RecordingQualityDto::High,
        RecordingQuality::Standard => RecordingQualityDto::Standard,
    }
}

const fn recording_quality_from_dto(quality: RecordingQualityDto) -> RecordingQuality {
    match quality {
        RecordingQualityDto::Original => RecordingQuality::Original,
        RecordingQualityDto::High => RecordingQuality::High,
        RecordingQualityDto::Standard => RecordingQuality::Standard,
    }
}

const fn ffmpeg_source_from_dto(source: FfmpegSourceDto) -> FfmpegSource {
    match source {
        FfmpegSourceDto::Bundled => FfmpegSource::Bundled,
        FfmpegSourceDto::Local => FfmpegSource::Local,
    }
}

const fn recording_state(state: RecordingState) -> RecordingStateDto {
    match state {
        RecordingState::Idle => RecordingStateDto::Idle,
        RecordingState::Starting => RecordingStateDto::Starting,
        RecordingState::Recording => RecordingStateDto::Recording,
        RecordingState::Stopping => RecordingStateDto::Stopping,
        RecordingState::Finalizing => RecordingStateDto::Finalizing,
        RecordingState::Error => RecordingStateDto::Error,
    }
}

fn now_millis() -> i64 {
    crate::application::media::MediaState::now_millis()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn preview_range_parser_accepts_one_bounded_range_only() {
        assert_eq!(
            parse_range(None).unwrap(),
            ByteRange {
                start: 0,
                end_inclusive: PREVIEW_CHUNK_BYTES - 1,
            }
        );
        let explicit = tauri::http::HeaderValue::from_static("bytes=10-19");
        assert_eq!(
            parse_range(Some(&explicit)).unwrap(),
            ByteRange {
                start: 10,
                end_inclusive: 19,
            }
        );
        let open = tauri::http::HeaderValue::from_static("bytes=20-");
        assert_eq!(
            parse_range(Some(&open)).unwrap().end_inclusive,
            20 + PREVIEW_CHUNK_BYTES - 1
        );
        for invalid in ["bytes=-20", "bytes=20-10", "bytes=0-1,4-5"] {
            let value = tauri::http::HeaderValue::from_str(invalid).unwrap();
            assert!(parse_range(Some(&value)).is_err());
        }
    }

    #[test]
    fn preview_response_exposes_range_without_a_local_path() {
        let response = preview_response(crate::application::media::PreviewRead {
            bytes: vec![1, 2, 3],
            total_size: 10,
            range: ByteRange {
                start: 2,
                end_inclusive: 4,
            },
        });
        assert_eq!(response.status(), StatusCode::PARTIAL_CONTENT);
        assert_eq!(response.headers()[header::CONTENT_RANGE], "bytes 2-4/10");
        assert_eq!(response.body(), &[1, 2, 3]);
    }
}
