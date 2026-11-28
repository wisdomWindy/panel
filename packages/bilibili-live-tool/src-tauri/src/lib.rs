pub mod application;
pub mod build_channel;
pub mod domain;
pub mod infrastructure;
pub mod ipc;

use std::sync::Arc;

use application::member::{AiState, MemberStates};
#[cfg(feature = "member")]
use application::member::{MembershipState, ModerationState};
use application::{
    bootstrap::ApplicationState,
    live_data::{
        AcceptedLiveEventConsumer, AcceptedLiveEventFanout, FailedRoomTransport, LiveDataState,
        LiveRepository, RoomTransport,
    },
    media::{
        default_captain_effect_config, default_gift_notify_config, default_gift_sound_policy,
        default_recording_config, default_tts_config, EffectRuntime, GiftMediaRuntime,
        MediaServices, MediaState, RecordingRuntime, TtsRuntime,
    },
    member::MemberRepository,
    paths::{PathPurpose, PathState},
    release::ReleaseState,
    shell::{
        ApplicationShellState, ShellRepository, SystemMetricsPort, UnavailableShellRepository,
        UnavailableSystemMetrics,
    },
};
use infrastructure::member::{
    OpenAiCompatibleLlmProvider, Qwen3TtsAiSpeechOutput, SqliteMemberRepository,
    SystemAiSecretStore, SystemAiSpeechOutput,
};
#[cfg(feature = "member")]
use infrastructure::member::{
    UnavailableAuthProvider, UnavailableEntitlementVerifier, UnavailableMembershipProvider,
    UnavailableModerationProvider,
};
use infrastructure::{
    database::SqliteDatabase,
    database_location,
    live_data::{
        BilibiliOpenLiveTransport, GrantedLiveFileStore, LiveDataRepository, OpenLiveConfig,
        UnavailableLiveRepository,
    },
    media::{
        AppFfmpegLocator, BilibiliPlayUrlResolver, Qwen3Environment, Qwen3ServiceLauncher, GiftSoundFileStore, LocalFfmpegProcessPort,
        PreviewRegistry, Qwen3TtsService, Qwen3TtsSpeechEngine, RodioAudioOutput,
        SqliteMediaRepository, SystemSpeechEngine, TauriEffectWindowPort, TauriHotkeyPort,
        TauriNotificationPort, UnavailableSpeechEngine,
    },
    path_grants::PathGrantStore,
    release::{installation_bucket, migration_resilience_snapshot, update_provider},
    shell::{
        NativeConfigArchiveFiles, NativeDirectoryOpener, NativeSystemLocale, SqliteShellRepository,
        SysinfoMetrics,
    },
};
use ipc::commands::{app_bootstrap, app_status, config_get, config_set, path_revoke, path_select};
use ipc::live_data::{
    audience_detail_query, blacklist_add, blacklist_export, blacklist_import, blacklist_list,
    blacklist_remove, rank_export, rank_query, room_connect, room_disconnect, sensitive_word_add,
    sensitive_word_delete, sensitive_word_export, sensitive_word_import, sensitive_word_list,
    sensitive_word_toggle, sensitive_word_update, stat_export, stat_query, stat_refresh,
    RankingPresenter, TauriLiveEventSink,
};
use ipc::media::{
    captain_effect_current, captain_effect_get, captain_effect_preview, captain_effect_save,
    gift_notify_get, gift_notify_save, gift_notify_test, gift_sound_export, gift_sound_import,
    gift_sound_list, gift_sound_remove, gift_sound_save, gift_sound_set,
    handle_media_preview_protocol, highlight_delete, highlight_export, highlight_list,
    ffmpeg_install, highlight_mark, highlight_preview, recording_delete, recording_get_config,
    recording_list,
    recording_preview, recording_open_folder, recording_rename, recording_save_config, recording_set_ffmpeg_source,
    recording_start, recording_status, recording_stop, recording_stop_task,
    recording_mark_highlight_task, media_cleanup, sound_preview, tts_clear, tts_get,
    tts_pause, tts_qwen3_download_model, tts_qwen3_get, tts_qwen3_models, tts_qwen3_save,
    tts_qwen3_environment_status,
    tts_qwen3_prepare_environment,
    tts_qwen3_start,
    tts_qwen3_test, tts_save, tts_set_enabled, tts_test,
    TauriMediaEventSink,
};
use ipc::member::{
    ai_config_get, ai_config_save, ai_generate, ai_log_query, ai_model_profile_delete,
    ai_model_profile_list, ai_model_profile_save, ai_model_set_default, ai_model_test, ai_models_list,
    ai_regenerate, ai_tts_clear_queue, ai_tts_list_output_devices, ai_tts_pause,
    ai_tts_set_output_device, ai_tts_skip, ai_tts_test, ai_voice_reply_get,
    ai_voice_reply_set_enabled, ai_voice_reply_update_rules, TauriMemberEventSink,
};
#[cfg(feature = "member")]
use ipc::member::{
    auth_bind_uid, auth_logout, auth_qrcode_cancel, auth_qrcode_generate, auth_query_uid,
    member_status, membership_cancel_order, membership_orders, membership_purchase,
    moderate_ignore, moderate_log_query, moderate_mark_false_positive, moderate_mute,
    moderate_set_auto, moderate_status, moderate_update_rules, module_cleanup, module_list,
    module_update,
};
use ipc::release::{
    about_info, resilience_status, update_check, update_download, update_install, update_skip,
    update_status, TauriReleaseEventSink,
};
use ipc::shell::{
    data_clear, data_retention_get, data_retention_save, config_export, config_import, config_open_directory, config_overview, config_reset, locale_get,
    locale_save, locale_set_auto, shell_status, ui_get, ui_preview, ui_reset, ui_save,
    TauriShellEventSink,
};
use tauri::Manager;

fn show_main_window(app: &tauri::AppHandle) {
    if let Some(window) = app.get_webview_window("main") {
        let _ = window.show();
        let _ = window.unminimize();
        let _ = window.set_focus();
    }
}

/// PRD 13 tray entry point. Returns an error instead of aborting startup so a
/// missing icon or menu cannot turn into a crash on launch.
fn install_tray(app: &tauri::App) -> tauri::Result<()> {
    let show_label = match tauri::async_runtime::block_on(
        app.state::<AppState>().shell().locale(),
    ) {
        crate::domain::shell::LocalePreferences {
            locale: crate::domain::shell::LocaleCode::EnUs,
            ..
        } => "Show window",
        _ => "显示主窗口",
    };
    let quit_label = if show_label == "Show window" {
        "Quit"
    } else {
        "退出"
    };
    let show_item =
        tauri::menu::MenuItem::with_id(app, "tray-show", show_label, true, None::<&str>)?;
    let quit_item =
        tauri::menu::MenuItem::with_id(app, "tray-quit", quit_label, true, None::<&str>)?;
    let menu = tauri::menu::Menu::with_items(app, &[&show_item, &quit_item])?;
    let mut tray = tauri::tray::TrayIconBuilder::with_id("main-tray")
        .tooltip("liver")
        .menu(&menu)
        .show_menu_on_left_click(false)
        .on_menu_event(|app, event| match event.id.as_ref() {
            "tray-show" => show_main_window(app),
            "tray-quit" => app.exit(0),
            _ => {}
        })
        .on_tray_icon_event(|tray, event| {
            if let tauri::tray::TrayIconEvent::Click {
                button: tauri::tray::MouseButton::Left,
                button_state: tauri::tray::MouseButtonState::Up,
                ..
            } = event
            {
                show_main_window(tray.app_handle());
            }
        });
    if let Some(icon) = app.default_window_icon() {
        tray = tray.icon(icon.clone());
    } else {
        return Err(tauri::Error::AssetNotFound(
            "default window icon is missing".to_owned(),
        ));
    }
    tray.build(app)?;
    Ok(())
}

pub struct AppState {
    application: ApplicationState,
    paths: PathState,
    live_data: Arc<LiveDataState>,
    media: Arc<MediaState>,
    qwen3_tts: Arc<Qwen3TtsService>,
    ranking_presenter: Arc<RankingPresenter>,
    data_maintenance: Option<Arc<crate::application::data_maintenance::DataMaintenance>>,
    shell: Arc<ApplicationShellState>,
    release: Arc<ReleaseState>,
    ai: Option<Arc<AiState>>,
    member: Option<Arc<MemberStates>>,
}

impl AppState {
    async fn new(app: tauri::AppHandle, database_path: std::path::PathBuf) -> Self {
        let config_directory = database_path.parent().map_or_else(
            || std::path::PathBuf::from("."),
            std::path::Path::to_path_buf,
        );
        let database = Arc::new(SqliteDatabase::new(database_path.clone()));
        let application = ApplicationState::from_arc(database.clone());
        if let Err(error) = application.bootstrap().await {
            log::warn!("application bootstrap is degraded: {}", error.code);
        }

        let pool = database.pool();
        let grants = Arc::new(PathGrantStore::new());
        let shell_repository: Arc<dyn ShellRepository> = pool.clone().map_or_else(
            || Arc::new(UnavailableShellRepository) as Arc<dyn ShellRepository>,
            |pool| Arc::new(SqliteShellRepository::new(pool)) as Arc<dyn ShellRepository>,
        );
        let system_metrics: Arc<dyn SystemMetricsPort> = SysinfoMetrics::current_process()
            .map_or_else(
                |_| Arc::new(UnavailableSystemMetrics) as Arc<dyn SystemMetricsPort>,
                |metrics| Arc::new(metrics) as Arc<dyn SystemMetricsPort>,
            );
        // A degraded database bootstrap must not take the whole app down: the
        // data-maintenance features simply report themselves unavailable.
        let data_maintenance = pool.clone().map(|pool| {
            Arc::new(crate::application::data_maintenance::DataMaintenance::new(
                pool,
                Some(config_directory.join("cache")),
            ))
        });
        let shell = Arc::new(
            ApplicationShellState::initialize(
                shell_repository,
                Arc::new(NativeConfigArchiveFiles),
                Arc::new(NativeDirectoryOpener),
                system_metrics,
                Arc::new(NativeSystemLocale),
                Arc::new(TauriShellEventSink::new(app.clone())),
                config_directory.clone(),
            )
            .await,
        );
        let update_provider = update_provider(app.clone());
        let release = Arc::new(ReleaseState::new(
            update_provider,
            Arc::new(TauriReleaseEventSink::new(app.clone())),
            installation_bucket(&database_path),
            migration_resilience_snapshot(&database_path),
        ));
        let mut captain_config = default_captain_effect_config();
        let mut gift_notify_config = default_gift_notify_config();
        let mut gift_sound_policy = default_gift_sound_policy();
        let mut recording_config = default_recording_config();
        let mut tts_config = default_tts_config();
        let mut gift_sounds = Vec::new();
        if let Some(pool) = pool.clone() {
            let repository = SqliteMediaRepository::new(pool);
            match repository.load_captain_config().await {
                Ok(Some(value)) => captain_config = value,
                Ok(None) => {}
                Err(error) => log::warn!("captain media config is degraded: {error}"),
            }
            match repository.load_gift_notify_config().await {
                Ok(Some(value)) => gift_notify_config = value,
                Ok(None) => {}
                Err(error) => log::warn!("gift notification config is degraded: {error}"),
            }
            match repository.load_gift_sound_policy().await {
                Ok(Some(value)) => gift_sound_policy = value,
                Ok(None) => {}
                Err(error) => log::warn!("gift sound policy is degraded: {error}"),
            }
            match repository.load_recording_config().await {
                Ok(Some(mut value)) => {
                    if let Some(directory) = value.directory.take() {
                        match grants.grant_selection(
                            Some(std::path::PathBuf::from(&directory.display_path)),
                            PathPurpose::RecordingDirectory,
                        ) {
                            Ok(Some(grant)) => {
                                value.directory = Some(domain::media::RecordingDirectoryGrant {
                                    grant_id: grant.id().to_owned(),
                                    display_path: grant.display_path().to_owned(),
                                });
                            }
                            Ok(None) | Err(_) => {
                                log::warn!("recording directory authorization is degraded")
                            }
                        }
                    }
                    recording_config = value;
                }
                Ok(None) => {}
                Err(error) => log::warn!("recording config is degraded: {error}"),
            }
            match repository.load_tts_config().await {
                Ok(Some(value)) => tts_config = value,
                Ok(None) => {}
                Err(error) => log::warn!("tts config is degraded: {error}"),
            }
            match repository.list_gift_sounds().await {
                Ok(value) => gift_sounds = value,
                Err(error) => log::warn!("gift sound bindings are degraded: {error}"),
            }
        }

        let qwen3_tts = Qwen3TtsService::bootstrap(pool.clone()).await;

        let audio = Arc::new(RodioAudioOutput);
        let effects = EffectRuntime::start(
            captain_config,
            audio.clone(),
            Arc::new(TauriEffectWindowPort::new(app.clone())),
        )
        .await;
        let gifts = GiftMediaRuntime::start(
            gift_notify_config,
            gift_sound_policy,
            gift_sounds,
            audio,
            Arc::new(TauriNotificationPort::new(app.clone())),
        )
        .await;
        let system_speech = SystemSpeechEngine::new()
            .map(|engine| Arc::new(engine) as Arc<dyn application::media::SpeechEnginePort>)
            .unwrap_or_else(|error| {
                log::warn!("system speech is degraded: {error}");
                Arc::new(UnavailableSpeechEngine)
            });
        let tts = TtsRuntime::start(
            tts_config,
            system_speech,
            Arc::new(Qwen3TtsSpeechEngine::new(
                qwen3_tts.clone(),
                Arc::new(RodioAudioOutput),
            )),
            Arc::new(UnavailableSpeechEngine),
        )
        .await;
        let recording = RecordingRuntime::new(recording_config, Arc::new(LocalFfmpegProcessPort));
        if let Err(error) = recording
            .register_hotkey(&TauriHotkeyPort::new(app.clone()))
            .await
        {
            log::warn!("recording hotkey is degraded: {error}");
        }
        // PRD 9.11: the speech runtime is only installed and started when the
        // operator opens the 语音播报 page, never at start-up.
        let qwen3_log_directory = app
            .path()
            .app_log_dir()
            .unwrap_or_else(|_| std::env::temp_dir().join("biliLiveHelper/logs"));
        let qwen3_environment = Arc::new(Qwen3Environment::for_current_app(
            environment_root(&app),
            qwen3_log_directory.clone(),
        ));
        let qwen3_venv_python = if cfg!(windows) {
            environment_root(&app)
                .join("venv")
                .join("Scripts")
                .join("python.exe")
        } else {
            environment_root(&app)
                .join("venv")
                .join("bin")
                .join("python")
        };
        let qwen3_launcher = Arc::new(
            Qwen3ServiceLauncher::for_current_app(qwen3_log_directory)
                .with_managed_venv_python(qwen3_venv_python),
        );
        let media = MediaState::new(effects, gifts, tts, recording)
            .with_recording_clients(
                Arc::new(BilibiliPlayUrlResolver::new()),
                Arc::new(
                    AppFfmpegLocator::for_current_app()
                        .with_managed_root(managed_ffmpeg_directory(&app)),
                ),
            )
            // PRD 9.12: recordings land in `<install or data dir>/download` so
            // start-recording never depends on a folder dialog.
            .with_default_recording_directory(default_recording_directory(&app))
            .with_managed_ffmpeg_directory(managed_ffmpeg_directory(&app))
            .with_qwen3_service_launcher(qwen3_launcher)
            .with_qwen3_environment(qwen3_environment);
        let media = if let Some(pool) = pool.clone() {
            media.with_services(MediaServices {
                repository: Arc::new(SqliteMediaRepository::new(pool)),
                files: Arc::new(GiftSoundFileStore::new(grants.clone())),
                preview: Arc::new(PreviewRegistry::new(64)),
                grants: grants.clone(),
                audio: Arc::new(RodioAudioOutput),
            })
        } else {
            media
        };
        let media =
            Arc::new(media.with_event_sink(Arc::new(TauriMediaEventSink::new(app.clone()))));

        let ai = pool.clone().map(|pool| {
            let repository: Arc<dyn MemberRepository> = Arc::new(SqliteMemberRepository::new(pool));
            let secrets = Arc::new(SystemAiSecretStore);
            let llm = Arc::new(OpenAiCompatibleLlmProvider::new(
                repository.clone(),
                secrets.clone(),
            ));
            Arc::new(
                AiState::new_with_services(
                    repository,
                    llm,
                    secrets,
                    Arc::new(Qwen3TtsAiSpeechOutput::new(
                        qwen3_tts.clone(),
                        Arc::new(SystemAiSpeechOutput::default()),
                    )),
                )
                .with_event_sink(Arc::new(TauriMemberEventSink::new(app.clone()))),
            )
        });

        #[cfg(feature = "member")]
        let member = pool.clone().map(|pool| {
            let repository: Arc<dyn MemberRepository> = Arc::new(SqliteMemberRepository::new(pool));
            let event_sink = Arc::new(TauriMemberEventSink::new(app.clone()));
            let membership = Arc::new(
                MembershipState::new(
                    repository.clone(),
                    Arc::new(UnavailableAuthProvider),
                    Arc::new(UnavailableMembershipProvider),
                    Arc::new(UnavailableEntitlementVerifier),
                )
                .with_event_sink(event_sink.clone()),
            );
            let moderation = Arc::new(
                ModerationState::new(
                    repository,
                    Arc::new(UnavailableModerationProvider),
                    membership.clone(),
                )
                .with_event_sink(event_sink),
            );
            Arc::new(MemberStates::new(membership, moderation))
        });
        #[cfg(all(feature = "free", not(feature = "member")))]
        let member: Option<Arc<MemberStates>> = None;
        #[cfg(feature = "member")]
        if let Some(member) = &member {
            member.start_hourly_validation().await;
        }

        let paths = PathState::from_arc(grants.clone());
        let ranking_presenter = Arc::new(RankingPresenter::new());
        let repository: Arc<dyn LiveRepository> = pool.clone().map_or_else(
            || Arc::new(UnavailableLiveRepository) as Arc<dyn LiveRepository>,
            |pool| Arc::new(LiveDataRepository::new(pool)) as Arc<dyn LiveRepository>,
        );
        let event_sink = Arc::new(TauriLiveEventSink::new(app, ranking_presenter.clone()));
        let transport: Arc<dyn RoomTransport> = match OpenLiveConfig::embedded() {
            Ok(config) => Arc::new(BilibiliOpenLiveTransport::new(config)),
            Err(error) => {
                log::warn!("open live transport is unavailable: code={}", error.code);
                Arc::new(FailedRoomTransport::new(error))
            }
        };
        let live_data = LiveDataState::new(transport, repository, event_sink)
            .with_files(Arc::new(GrantedLiveFileStore::new(grants)))
            .with_probe(Arc::new(
                crate::infrastructure::live_data::BilibiliLiveStatusProbe,
            ));
        #[cfg(all(feature = "free", not(feature = "member")))]
        let live_data = {
            let mut consumers = vec![media.clone() as Arc<dyn AcceptedLiveEventConsumer>];
            if let Some(ai) = &ai {
                consumers.push(ai.clone() as Arc<dyn AcceptedLiveEventConsumer>);
            }
            live_data.with_accepted_consumer(Arc::new(AcceptedLiveEventFanout::new(consumers)))
        };
        #[cfg(feature = "member")]
        let live_data =
            live_data.with_accepted_consumer(Arc::new(AcceptedLiveEventFanout::new(
                member.as_ref().map_or_else(
                    || {
                        vec![media.clone()
                            as Arc<dyn application::live_data::AcceptedLiveEventConsumer>]
                    },
                    |member| {
                        vec![
                            media.clone() as Arc<dyn AcceptedLiveEventConsumer>,
                            ai.as_ref()
                                .expect("AI state exists with initialized storage")
                                .clone()
                                as Arc<dyn AcceptedLiveEventConsumer>,
                            member.moderation.clone() as Arc<dyn AcceptedLiveEventConsumer>,
                        ]
                    },
                ),
            )));
        let live_data = Arc::new(live_data);
        if let Err(error) = live_data.load_safety().await {
            log::warn!("live safety bootstrap is degraded: {}", error.code);
        }

        Self {
            application,
            paths,
            live_data,
            media,
            qwen3_tts,
            ranking_presenter,
            data_maintenance,
            shell,
            release,
            ai,
            member,
        }
    }

    pub(crate) fn application(&self) -> &ApplicationState {
        &self.application
    }

    pub(crate) fn paths(&self) -> &PathState {
        &self.paths
    }

    pub(crate) fn live_data(&self) -> &Arc<LiveDataState> {
        &self.live_data
    }

    pub(crate) fn ranking_presenter(&self) -> &RankingPresenter {
        &self.ranking_presenter
    }

    pub(crate) fn media(&self) -> &Arc<MediaState> {
        &self.media
    }

    pub(crate) fn qwen3_tts(&self) -> &Arc<Qwen3TtsService> {
        &self.qwen3_tts
    }

    pub(crate) fn shell(&self) -> &Arc<ApplicationShellState> {
        &self.shell
    }

    pub(crate) fn data_maintenance(
        &self,
    ) -> Option<&Arc<crate::application::data_maintenance::DataMaintenance>> {
        self.data_maintenance.as_ref()
    }

    pub(crate) fn release(&self) -> &Arc<ReleaseState> {
        &self.release
    }

    pub(crate) fn ai(&self) -> Result<&Arc<AiState>, ipc::error::DesktopError> {
        self.ai.as_ref().ok_or_else(|| {
            ipc::error::DesktopError::new("STORAGE_AI_UNAVAILABLE", "AI 鏈湴鏁版嵁鏆備笉鍙敤", true)
        })
    }

    pub(crate) fn member(&self) -> Result<&Arc<MemberStates>, ipc::error::DesktopError> {
        self.member.as_ref().ok_or_else(|| {
            ipc::error::DesktopError::new(
                "STORAGE_MEMBER_UNAVAILABLE",
                "浼氬憳鏈湴鏁版嵁鏆備笉鍙敤",
                true,
            )
        })
    }
}

/// Startup crashes happen before the log plugin exists, so a plain panic hook
/// appends to a fixed file the user can send us. Without it an instant exit
/// leaves no evidence at all.
/// Recordings default to `download` beside the installed executable, or inside
/// the per-user data directory for a development build.
fn default_recording_directory(app: &tauri::AppHandle) -> std::path::PathBuf {
    let directory = if cfg!(debug_assertions) {
        app.path()
            .app_data_dir()
            .map(|data| data.join("download"))
            .unwrap_or_else(|_| std::env::temp_dir().join("biliLiveHelper/download"))
    } else {
        std::env::current_exe()
            .ok()
            .and_then(|executable| executable.parent().map(|parent| parent.join("download")))
            .unwrap_or_else(|| std::env::temp_dir().join("biliLiveHelper/download"))
    };
    if let Err(error) = std::fs::create_dir_all(&directory) {
        log::warn!("default recording directory is unavailable: {error}");
    }
    directory
}

/// FFmpeg installed by the app lives here, outside `PATH` and outside the
/// operator's own tool directories.
fn managed_ffmpeg_directory(app: &tauri::AppHandle) -> std::path::PathBuf {
    app.path()
        .app_data_dir()
        .map(|data| data.join("ffmpeg"))
        .unwrap_or_else(|_| std::env::temp_dir().join("biliLiveHelper/ffmpeg"))
}

fn install_startup_crash_reporter() {    std::panic::set_hook(Box::new(|info| {
        let location = info
            .location()
            .map(|value| format!("{}:{}", value.file(), value.line()))
            .unwrap_or_else(|| "unknown".to_owned());
        let payload = info
            .payload()
            .downcast_ref::<&str>()
            .map(|value| (*value).to_owned())
            .or_else(|| {
                info.payload()
                    .downcast_ref::<String>()
                    .map(|value| value.clone())
            })
            .unwrap_or_else(|| "unknown panic".to_owned());
        let directory = std::env::var_os("LOCALAPPDATA")
            .map(std::path::PathBuf::from)
            .map(|root| root.join("com.bilibili.live-tool"))
            .unwrap_or_else(std::env::temp_dir);
        let _ = std::fs::create_dir_all(&directory);
        let line = format!("panic at {location}: {payload}\n");
        if let Ok(mut file) = std::fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(directory.join("startup-crash.log"))
        {
            use std::io::Write;
            let _ = file.write_all(line.as_bytes());
        }
        eprintln!("{line}");
    }));
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    install_startup_crash_reporter();
    let builder = tauri::Builder::default()
        .plugin(tauri_plugin_updater::Builder::new().build())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_notification::init())
        .plugin(tauri_plugin_global_shortcut::Builder::new().build())
        .register_asynchronous_uri_scheme_protocol("media-preview", handle_media_preview_protocol)
        .on_window_event(|window, event| {
            if window.label() == "main" && matches!(event, tauri::WindowEvent::Destroyed) {
                let media = window.state::<AppState>().media().clone();
                let live_data = window.state::<AppState>().live_data().clone();
                let ai = window.state::<AppState>().ai.clone();
                let member = window.state::<AppState>().member.clone();
                tauri::async_runtime::spawn(async move {
                    let _ = live_data.disconnect_room().await;
                    media.shutdown().await;
                    if let Some(ai) = ai {
                        ai.shutdown().await;
                    }
                    if let Some(member) = member {
                        member.shutdown().await;
                    }
                });
            }
        })
        .setup(|app| {
            // PRD 12.3: runtime logs roll over by size and are pruned by age
            // below, in release builds as well as dev builds. A logging failure
            // must never keep the window from opening.
            let logging = app.handle().plugin(
                tauri_plugin_log::Builder::default()
                    .level(log::LevelFilter::Info)
                    .max_file_size(5 * 1024 * 1024)
                    .rotation_strategy(tauri_plugin_log::RotationStrategy::KeepSome(5))
                    .target(tauri_plugin_log::Target::new(
                        tauri_plugin_log::TargetKind::LogDir { file_name: None },
                    ))
                    .target(tauri_plugin_log::Target::new(
                        tauri_plugin_log::TargetKind::Stdout,
                    ))
                    .build(),
            );
            if let Err(error) = logging {
                eprintln!("file logging is unavailable: {error}");
            }
            let log_directory = app.path().app_log_dir().ok();

            let database_path =
                database_location::resolve_database_path(&app.path().app_data_dir()?)?;
            let state =
                tauri::async_runtime::block_on(AppState::new(app.handle().clone(), database_path));
            app.manage(state);
            // A recording row left in `recording` by an abnormal exit would keep
            // claiming that a recording is running; settle it on start-up.
            {
                let media = Arc::clone(app.state::<AppState>().media());
                tauri::async_runtime::spawn(async move {
                    if let Err(error) = media.recover_interrupted_recordings().await {
                        log::warn!("could not settle interrupted recordings: {}", error.message);
                    }
                });
            }
            // PRD 12.3: purge expired logs and stats locally on a schedule.
            let maintenance = app.state::<AppState>().data_maintenance().map(Arc::clone);
            let log_directory = log_directory.clone();
            if let Some(maintenance) = maintenance {
            tauri::async_runtime::spawn(async move {
                loop {
                    match maintenance.load_policy().await {
                        Ok(policy) => {
                            if let Err(error) = maintenance.purge_expired(policy).await {
                                log::warn!("retention purge failed: {error}");
                            }
                        }
                        Err(error) => log::warn!("retention policy is unavailable: {error}"),
                    }
                    // Runtime logs have their own fixed 30-day window.
                    if let Some(directory) = &log_directory {
                        let removed = maintenance
                            .purge_log_files(directory, crate::application::data_maintenance::RUNTIME_LOG_RETENTION_DAYS)
                            .await;
                        if removed > 0 {
                            log::info!("pruned {removed} expired log files");
                        }
                    }
                    tokio::time::sleep(std::time::Duration::from_secs(6 * 60 * 60)).await;
                }
            });
            }
            // PRD 13: the support matrix claims tray support, so provide it.
            // Anything failing here (missing shell, icon, menu) must degrade to
            // "no tray" instead of aborting startup.
            if let Err(error) = install_tray(app) {
                log::warn!("tray icon is unavailable: {error}");
            }
            Ok(())
        });

    #[cfg(all(feature = "free", not(feature = "member")))]
    let builder = builder.invoke_handler(tauri::generate_handler![
        app_bootstrap,
        app_status,
        config_get,
        config_set,
        path_select,
        path_revoke,
        shell_status,
        config_overview,
        config_open_directory,
        config_export,
        config_import,
        config_reset,
        data_retention_get,
        data_retention_save,
        data_clear,
        locale_get,
        locale_set_auto,
        locale_save,
        ui_get,
        ui_preview,
        ui_save,
        ui_reset,
        update_status,
        update_check,
        update_download,
        update_install,
        update_skip,
        about_info,
        resilience_status,
        room_connect,
        room_disconnect,
        audience_detail_query,
        sensitive_word_list,
        sensitive_word_add,
        sensitive_word_update,
        sensitive_word_delete,
        sensitive_word_toggle,
        sensitive_word_import,
        sensitive_word_export,
        blacklist_list,
        blacklist_add,
        blacklist_remove,
        blacklist_export,
        blacklist_import,
        stat_query,
        stat_refresh,
        stat_export,
        rank_query,
        rank_export,
        ai_config_get,
        ai_config_save,
        ai_model_profile_list,
        ai_model_profile_save,
        ai_model_profile_delete,
        ai_model_set_default,
        ai_models_list,
        ai_model_test,
        ai_voice_reply_get,
        ai_voice_reply_set_enabled,
        ai_voice_reply_update_rules,
        ai_tts_list_output_devices,
        ai_tts_set_output_device,
        ai_tts_test,
        ai_tts_pause,
        ai_tts_skip,
        ai_tts_clear_queue,
        ai_generate,
        ai_regenerate,
        ai_log_query,
        captain_effect_get,
        captain_effect_current,
        captain_effect_save,
        captain_effect_preview,
        gift_notify_get,
        gift_notify_save,
        gift_notify_test,
        gift_sound_list,
        gift_sound_set,
        gift_sound_remove,
        gift_sound_save,
        gift_sound_import,
        gift_sound_export,
        sound_preview,
        tts_get,
        tts_save,
        tts_set_enabled,
        tts_test,
        tts_pause,
        tts_clear,
        tts_qwen3_get,
        tts_qwen3_save,
        tts_qwen3_test,
        tts_qwen3_models,
        tts_qwen3_environment_status,
        tts_qwen3_prepare_environment,
        tts_qwen3_start,
        tts_qwen3_download_model,
        ffmpeg_install,
        recording_get_config,
        recording_save_config,
        recording_set_ffmpeg_source,
        recording_status,
        recording_start,
        recording_stop,
        recording_stop_task,
        recording_mark_highlight_task,
        media_cleanup,
        recording_list,
        recording_preview,
        recording_open_folder,
        recording_rename,
        recording_delete,
        highlight_list,
        highlight_mark,
        highlight_preview,
        highlight_export,
        highlight_delete
    ]);

    #[cfg(feature = "member")]
    let builder = builder.invoke_handler(tauri::generate_handler![
        app_bootstrap,
        app_status,
        config_get,
        config_set,
        path_select,
        path_revoke,
        shell_status,
        config_overview,
        config_open_directory,
        config_export,
        config_import,
        config_reset,
        data_retention_get,
        data_retention_save,
        data_clear,
        locale_get,
        locale_set_auto,
        locale_save,
        ui_get,
        ui_preview,
        ui_save,
        ui_reset,
        update_status,
        update_check,
        update_download,
        update_install,
        update_skip,
        about_info,
        resilience_status,
        member_status,
        auth_qrcode_generate,
        auth_qrcode_cancel,
        auth_query_uid,
        auth_bind_uid,
        auth_logout,
        membership_purchase,
        membership_cancel_order,
        membership_orders,
        module_list,
        module_update,
        module_cleanup,
        ai_config_get,
        ai_config_save,
        ai_model_profile_list,
        ai_model_profile_save,
        ai_model_profile_delete,
        ai_model_set_default,
        ai_models_list,
        ai_model_test,
        ai_voice_reply_get,
        ai_voice_reply_set_enabled,
        ai_voice_reply_update_rules,
        ai_tts_list_output_devices,
        ai_tts_set_output_device,
        ai_tts_test,
        ai_tts_pause,
        ai_tts_skip,
        ai_tts_clear_queue,
        ai_generate,
        ai_regenerate,
        ai_log_query,
        moderate_status,
        moderate_set_auto,
        moderate_update_rules,
        moderate_mute,
        moderate_ignore,
        moderate_log_query,
        moderate_mark_false_positive,
        room_connect,
        room_disconnect,
        audience_detail_query,
        sensitive_word_list,
        sensitive_word_add,
        sensitive_word_update,
        sensitive_word_delete,
        sensitive_word_toggle,
        sensitive_word_import,
        sensitive_word_export,
        blacklist_list,
        blacklist_add,
        blacklist_remove,
        blacklist_export,
        blacklist_import,
        stat_query,
        stat_refresh,
        stat_export,
        rank_query,
        rank_export,
        captain_effect_get,
        captain_effect_current,
        captain_effect_save,
        captain_effect_preview,
        gift_notify_get,
        gift_notify_save,
        gift_notify_test,
        gift_sound_list,
        gift_sound_set,
        gift_sound_remove,
        gift_sound_save,
        gift_sound_import,
        gift_sound_export,
        sound_preview,
        tts_get,
        tts_save,
        tts_set_enabled,
        tts_test,
        tts_pause,
        tts_clear,
        tts_qwen3_get,
        tts_qwen3_save,
        tts_qwen3_test,
        tts_qwen3_models,
        tts_qwen3_environment_status,
        tts_qwen3_prepare_environment,
        tts_qwen3_start,
        tts_qwen3_download_model,
        ffmpeg_install,
        recording_get_config,
        recording_save_config,
        recording_set_ffmpeg_source,
        recording_status,
        recording_start,
        recording_stop,
        recording_stop_task,
        recording_mark_highlight_task,
        media_cleanup,
        recording_list,
        recording_preview,
        recording_open_folder,
        recording_rename,
        recording_delete,
        highlight_list,
        highlight_mark,
        highlight_preview,
        highlight_export,
        highlight_delete
    ]);

    builder
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

/// Virtualenv and pip logs live in the app data directory, never in the
/// install directory, so an upgrade cannot delete them.
fn environment_root(app: &tauri::AppHandle) -> std::path::PathBuf {
    app.path()
        .app_data_dir()
        .map(|data| data.join("qwen3-tts"))
        .unwrap_or_else(|_| std::env::temp_dir().join("biliLiveHelper/qwen3-tts"))
}
