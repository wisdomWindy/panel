use tauri::{
    http::{header, Request, Response, StatusCode},
    AppHandle, Emitter, EventTarget, Manager, Runtime, State, UriSchemeContext, UriSchemeResponder,
};

use crate::{
    application::{
        media::{
            ActiveSpeechEngine, ByteRange, GiftRuntimeSnapshot, GiftSoundBindingRecord,
            HighlightRecord, HighlightTriggerType, MediaEventSink, MediaIoError,
            RecordingItemState, RecordingRecord, SpeechSynthesisPort, TtsRuntimeSnapshot,
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
    infrastructure::media::Qwen3TtsService,
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
            build_captain_effect_payload(
                base.event_id.clone(),
                audience_identity(&base.audience),
                *action,
                *level,
                config.clone(),
                false,
            ),
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
pub async fn captain_effect_current(
    state: State<'_, AppState>,
) -> Result<Option<CaptainEffectPayloadDto>, DesktopError> {
    Ok(state
        .media()
        .effects()
        .active_effect()
        .await
        .map(captain_effect_payload))
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
) -> Result<GiftAlertPayloadDto, DesktopError> {
    let config = gift_notify_from_dto(request.config);
    crate::domain::media::validate_gift_notify_config(&config)
        .map_err(|error| DesktopError::from(MediaIoError::invalid(error.to_string())))?;
    let record = state
        .media()
        .gifts()
        .preview(config.clone(), format!("preview-{}", now_millis()))
        .await;
    Ok(GiftAlertPayloadDto {
        event: gift_record(record),
        popup_position: position(config.popup_position),
        popup_duration_seconds: config.popup_duration_seconds,
        is_preview: true,
    })
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
pub async fn tts_qwen3_get(
    state: State<'_, AppState>,
) -> Result<Qwen3TtsSnapshotDto, DesktopError> {
    Ok(qwen3_snapshot(state.qwen3_tts()).await)
}

#[tauri::command]
pub async fn tts_qwen3_save(
    state: State<'_, AppState>,
    request: Qwen3TtsConfigRequestDto,
) -> Result<Qwen3TtsSnapshotDto, DesktopError> {
    state
        .qwen3_tts()
        .save_config(qwen3_config_from_dto(request.config))
        .await?;
    Ok(qwen3_snapshot(state.qwen3_tts()).await)
}

#[tauri::command]
pub async fn tts_qwen3_test(
    state: State<'_, AppState>,
) -> Result<Qwen3TtsSnapshotDto, DesktopError> {
    let service = state.qwen3_tts();
    if !service.is_enabled() {
        return Err(MediaIoError::coded(
            "qwen3_tts_disabled",
            "请先启用 Qwen3-TTS 语音服务",
            false,
        )
        .into());
    }
    let mut config = state.media().tts().config().await;
    config.selected_engine = TtsSelectedEngine::Cloud;
    config.selected_voice_id = Some(service.config().event_voice().to_owned());
    state
        .media()
        .tts()
        .test(&config, "用户A说：今天直播真不错".into())
        .await?;
    Ok(qwen3_snapshot(service).await)
}

#[tauri::command]
pub async fn tts_qwen3_models(
    state: State<'_, AppState>,
) -> Result<Qwen3ModelCatalogDto, DesktopError> {
    let catalog = tokio::time::timeout(
        std::time::Duration::from_secs(3),
        state.qwen3_tts().model_catalog(),
    )
    .await
    .map_err(|_| MediaIoError::coded("TTS_TIMEOUT", "模型列表请求超时", true))??;
    Ok(qwen3_catalog(catalog))
}

/// Downloads the released checkpoint into the speech service cache.
/// With no explicit model this fetches the 1.7B CustomVoice default.
#[tauri::command]
pub async fn tts_qwen3_download_model(
    state: State<'_, AppState>,
    request: Qwen3DownloadRequestDto,
) -> Result<Qwen3DownloadStateDto, DesktopError> {
    let state = tokio::time::timeout(
        std::time::Duration::from_secs(10),
        state
            .qwen3_tts()
            .start_model_download(request.model.as_deref()),
    )
    .await
    .map_err(|_| MediaIoError::coded("TTS_TIMEOUT", "模型下载请求超时", true))??;
    Ok(qwen3_download(state))
}

fn qwen3_catalog(catalog: crate::infrastructure::media::Qwen3ModelCatalog) -> Qwen3ModelCatalogDto {
    Qwen3ModelCatalogDto {
        models: catalog
            .models
            .into_iter()
            .map(|model| Qwen3ModelEntryDto {
                model_id: model.model_id,
                display_name: model.display_name,
                task: model.task,
                is_default: model.is_default,
                is_installed: model.is_installed,
            })
            .collect(),
        download: qwen3_download(catalog.download),
    }
}

fn qwen3_download(
    state: crate::infrastructure::media::Qwen3DownloadState,
) -> Qwen3DownloadStateDto {
    Qwen3DownloadStateDto {
        model_id: state.model_id,
        status: match state.status {
            crate::infrastructure::media::Qwen3DownloadStatus::Idle => Qwen3DownloadStatusDto::Idle,
            crate::infrastructure::media::Qwen3DownloadStatus::Downloading => {
                Qwen3DownloadStatusDto::Downloading
            }
            crate::infrastructure::media::Qwen3DownloadStatus::Ready => {
                Qwen3DownloadStatusDto::Ready
            }
            crate::infrastructure::media::Qwen3DownloadStatus::Error => Qwen3DownloadStatusDto::Error,
        },
        progress_percent: state.progress_percent,
        bytes_downloaded: state.bytes_downloaded,
        bytes_total: state.bytes_total,
        path: state.path,
        error: state.error,
    }
}

/// A bounded probe: never let an unreachable service stall the settings page.
async fn qwen3_snapshot(service: &Qwen3TtsService) -> Qwen3TtsSnapshotDto {
    let config = service.config();
    let probe = tokio::time::timeout(std::time::Duration::from_secs(2), async {
        let health = service.health().await.ok();
        let voices = service.voices().await.ok().unwrap_or_default();
        (health, voices)
    })
    .await;
    let (health, voices) = probe.unwrap_or((None, Vec::new()));
    Qwen3TtsSnapshotDto {
        config: qwen3_config(config),
        health: health.map(|health| Qwen3TtsHealthDto {
            ready: health.ready,
            status: health.status,
            device: health.device,
            engine: health.engine,
            model: health.model,
            sample_rate: health.sample_rate,
            error: health.error,
        }),
        voice_ids: voices.into_iter().map(|voice| voice.voice_id).collect(),
    }
}

fn qwen3_config(config: crate::domain::media::Qwen3TtsConfig) -> Qwen3TtsConfigDto {
    Qwen3TtsConfigDto {
        is_enabled: config.is_enabled,
        base_url: config.base_url,
        voice_id: config.voice_id,
        language: config.language,
        instruct: config.instruct,
        timeout_ms: config.timeout_ms,
        fallback_to_system: config.fallback_to_system,
        ai_voice_id: config.ai_voice_id,
        ai_instruct: config.ai_instruct,
        pip_mirror: match config.pip_mirror {
            crate::domain::media::PipMirror::Tsinghua => PipMirrorDto::Tsinghua,
            crate::domain::media::PipMirror::Aliyun => PipMirrorDto::Aliyun,
            crate::domain::media::PipMirror::Official => PipMirrorDto::Official,
        },
    }
}

fn qwen3_config_from_dto(config: Qwen3TtsConfigDto) -> crate::domain::media::Qwen3TtsConfig {
    crate::domain::media::Qwen3TtsConfig {
        is_enabled: config.is_enabled,
        base_url: config.base_url,
        voice_id: config.voice_id,
        language: config.language,
        instruct: config.instruct,
        timeout_ms: config.timeout_ms,
        fallback_to_system: config.fallback_to_system,
        ai_voice_id: config.ai_voice_id,
        ai_instruct: config.ai_instruct,
        pip_mirror: match config.pip_mirror {
            PipMirrorDto::Tsinghua => crate::domain::media::PipMirror::Tsinghua,
            PipMirrorDto::Aliyun => crate::domain::media::PipMirror::Aliyun,
            PipMirrorDto::Official => crate::domain::media::PipMirror::Official,
        },
    }
}

#[tauri::command]
pub async fn recording_get_config(
    state: State<'_, AppState>,
) -> Result<RecordingConfigDto, DesktopError> {
    let mut config = recording_config(state.media().recording().config().await);
    // PRD 9.12: show the app-owned default so the operator sees where files go
    // without having to pick a folder first.
    if config.directory.is_none() {
        if let Some(directory) = state.media().default_recording_directory() {
            config.directory = Some(RecordingDirectoryDto {
                grant_id: crate::application::media::APP_DOWNLOAD_GRANT_ID.to_owned(),
                display_path: directory.display().to_string(),
            });
        }
    }
    Ok(config)
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
    state: State<'_, AppState>,
    request: RecordingStartRequestDto,
) -> Result<RecordingStatusDto, DesktopError> {
    state.media().start_recording(&request.room_id).await?;
    recording_status(state).await
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
        window_width: dto.window_width,
        window_height: dto.window_height,
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
        presets: captain_effect_presets(),
        updated_at,
    }
}

pub(super) fn captain_effect_payload(
    active: crate::application::media::ActiveCaptainEffect,
) -> CaptainEffectPayloadDto {
    let audience = active
        .item
        .audience
        .as_ref()
        .map(audience_identity)
        .unwrap_or_else(|| AudienceIdentityDto {
            user_uid: "preview-user".into(),
            username: active.item.username.clone(),
            user_level: None,
            medal: None,
            tags: vec![IdentityTagDto::Captain],
        });
    build_captain_effect_payload(
        active.item.event_id,
        audience,
        active.item.action,
        active.item.level,
        active.config,
        active.is_preview,
    )
}

fn build_captain_effect_payload(
    event_id: String,
    audience: AudienceIdentityDto,
    action: crate::domain::live_data::CaptainAction,
    level: crate::domain::live_data::CaptainLevel,
    config: CaptainEffectConfig,
    is_preview: bool,
) -> CaptainEffectPayloadDto {
    CaptainEffectPayloadDto {
        event_id,
        audience,
        action: captain_action_dto(action),
        level: captain_level_dto(level),
        config: captain_config(config),
        is_preview,
    }
}

pub(super) fn captain_effect_presets() -> Vec<CaptainEffectPresetDto> {
    [
        ("captain-default", "经典应援", "#00AEEC", "#FB7299"),
        ("golden-premium", "金色尊享", "#FFD700", "#FFA500"),
        ("starlight-blue", "星海蓝耀", "#38BDF8", "#6366F1"),
        ("cherry-blossom", "樱花绽放", "#FB7185", "#F9A8D4"),
        ("violet-galaxy", "紫曜星河", "#A78BFA", "#7C3AED"),
        ("emerald-stream", "翡翠流光", "#34D399", "#0F766E"),
        ("crimson-celebration", "赤焰庆典", "#F43F5E", "#F97316"),
        ("silver-frost", "冰晶银辉", "#E2E8F0", "#94A3B8"),
        ("neon-party", "霓虹派对", "#22D3EE", "#F472B6"),
        ("aurora-dream", "极光幻彩", "#2DD4BF", "#C084FC"),
        ("sunset-flare", "落日鎏光", "#FB923C", "#EAB308"),
        ("midnight-crown", "午夜王冠", "#818CF8", "#312E81"),
    ]
    .into_iter()
    .map(
        |(preset_id, display_name, primary_color, secondary_color)| CaptainEffectPresetDto {
            preset_id: preset_id.to_owned(),
            display_name: display_name.to_owned(),
            primary_color: primary_color.to_owned(),
            secondary_color: secondary_color.to_owned(),
        },
    )
    .collect()
}

fn captain_config(config: CaptainEffectConfig) -> CaptainEffectConfigDto {
    CaptainEffectConfigDto {
        preset_id: config.preset_id,
        duration_seconds: config.duration_seconds,
        position: position(config.position),
        window_width: config.window_width,
        window_height: config.window_height,
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
    } else if status.fallback_reason_code == Some("TTS_UNAVAILABLE_ENGINE") {
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
            FfmpegSource::System => FfmpegSourceDto::System,
            FfmpegSource::Installed => FfmpegSourceDto::Installed,
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
        gift_icon_url: record.gift_icon_url,
        quantity: record.quantity,
        value_minor: record.value_minor,
    }
}

fn gift_event_dto(event: &crate::domain::live_data::LiveEvent) -> Option<GiftEventDto> {
    let crate::domain::live_data::LiveEvent::Gift {
        base,
        gift_id,
        gift_name,
        gift_icon_url,
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
        gift_icon_url: gift_icon_url.clone(),
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
        FfmpegSourceDto::System => FfmpegSource::System,
        FfmpegSourceDto::Installed => FfmpegSource::Installed,
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
#[tauri::command]
pub async fn ffmpeg_install(state: State<'_, AppState>) -> Result<String, DesktopError> {
    let directory = state
        .media()
        .managed_ffmpeg_directory()
        .map(std::path::Path::to_path_buf)
        .ok_or_else(|| {
            DesktopError::from(crate::application::media::MediaIoError::coded(
                "RECORD_FFMPEG_INSTALL_FAILED",
                "无法确定 FFmpeg 安装目录",
                true,
            ))
        })?;
    let path = crate::infrastructure::media::FfmpegInstaller::new(directory)
        .install()
        .await
        .map_err(DesktopError::from)?;
    Ok(path.display().to_string())
}

#[tauri::command]
pub async fn tts_qwen3_start(
    state: State<'_, AppState>,
) -> Result<Qwen3TtsSnapshotDto, DesktopError> {
    let service = state.qwen3_tts();
    let config = service.config();
    let base_url = config.base_url.clone();
    if let Some(launcher) = state.media().qwen3_service_launcher() {
        let status = launcher
            .ensure_running(
                &base_url,
                crate::infrastructure::media::port_from_base_url(&base_url),
            )
            .await?;
        // The OS may reserve the configured port; keep the config pointing at
        // whatever actually serves so later calls hit the right endpoint.
        if status.base_url != base_url {
            let mut updated = config.clone();
            updated.base_url = status.base_url;
            service.save_config(updated).await?;
        }
    }
    Ok(qwen3_snapshot(service).await)
}

#[tauri::command]
pub async fn tts_qwen3_environment_status(
    state: State<'_, AppState>,
) -> Result<crate::infrastructure::media::EnvironmentStatus, DesktopError> {
    let environment = state
        .media()
        .qwen3_environment()
        .ok_or_else(|| DesktopError::from(unavailable_environment()))?
        .clone();
    Ok(environment.status())
}

#[tauri::command]
pub async fn tts_qwen3_prepare_environment(
    app: tauri::AppHandle,
    state: State<'_, AppState>,
) -> Result<crate::infrastructure::media::EnvironmentStatus, DesktopError> {
    let environment = state
        .media()
        .qwen3_environment()
        .ok_or_else(|| DesktopError::from(unavailable_environment()))?
        .clone();
    let handle = app.clone();
    let mirror = state.qwen3_tts().config().pip_mirror.index_url();
    crate::infrastructure::media::emit_environment_progress(
        &app,
        "start",
        "正在检查语音运行环境…",
    );
    let emitter = handle.clone();
    let prepared = environment
        .prepare(mirror, move |stage, message| {
            crate::infrastructure::media::emit_environment_progress(
                &emitter, stage, message,
            );
        })
        .await;
    match prepared {
        Ok(status) => Ok(status),
        Err(error) => {
            crate::infrastructure::media::emit_environment_progress(
                &handle,
                "failed",
                &error.message,
            );
            Err(error.into())
        }
    }
}
fn unavailable_environment() -> crate::application::media::MediaIoError {
    crate::application::media::MediaIoError::coded(
        "TTS_ENVIRONMENT_INCOMPLETE",
        "语音运行环境不可用，请重启应用后重试",
        true,
    )
}
