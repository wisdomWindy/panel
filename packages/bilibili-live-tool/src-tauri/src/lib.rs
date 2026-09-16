pub mod application;
pub mod build_channel;
pub mod domain;
pub mod infrastructure;
pub mod ipc;

use std::sync::Arc;

use application::{
    bootstrap::ApplicationState,
    live_data::{LiveDataState, LiveRepository, UnavailableRoomTransport},
    media::{
        default_captain_effect_config, default_gift_notify_config, default_gift_sound_policy,
        default_recording_config, default_tts_config, EffectRuntime, GiftMediaRuntime,
        MediaServices, MediaState, RecordingRuntime, TtsRuntime,
    },
    paths::{PathPurpose, PathState},
    release::ReleaseState,
    shell::{
        ApplicationShellState, ShellRepository, SystemMetricsPort, UnavailableShellRepository,
        UnavailableSystemMetrics,
    },
};
#[cfg(feature = "member")]
use application::{
    live_data::AcceptedLiveEventFanout,
    member::{AiState, MemberRepository, MemberStates, MembershipState, ModerationState},
};
#[cfg(feature = "member")]
use infrastructure::member::{
    SqliteMemberRepository, UnavailableAsrProvider, UnavailableAuthProvider,
    UnavailableEntitlementVerifier, UnavailableLlmProvider, UnavailableMembershipProvider,
    UnavailableModerationProvider,
};
use infrastructure::{
    database::SqliteDatabase,
    live_data::{GrantedLiveFileStore, LiveDataRepository, UnavailableLiveRepository},
    media::{
        GiftSoundFileStore, LocalFfmpegProcessPort, PreviewRegistry, RodioAudioOutput,
        SqliteMediaRepository, SystemSpeechEngine, TauriEffectWindowPort, TauriHotkeyPort,
        TauriNotificationPort, UnavailableSpeechEngine,
    },
    path_grants::PathGrantStore,
    release::{installation_bucket, migration_resilience_snapshot, release_providers},
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
    captain_effect_get, captain_effect_preview, captain_effect_save, gift_notify_get,
    gift_notify_save, gift_notify_test, gift_sound_export, gift_sound_import, gift_sound_list,
    gift_sound_remove, gift_sound_save, gift_sound_set, handle_media_preview_protocol,
    highlight_delete, highlight_export, highlight_list, highlight_mark, highlight_preview,
    recording_delete, recording_get_config, recording_list, recording_preview, recording_rename,
    recording_save_config, recording_set_ffmpeg_source, recording_start, recording_status,
    recording_stop, sound_preview, tts_clear, tts_get, tts_pause, tts_save, tts_set_enabled,
    tts_test, TauriMediaEventSink,
};
#[cfg(feature = "member")]
use ipc::member::{
    ai_config_get, ai_config_save, ai_generate, ai_log_query, ai_regenerate, asr_model_download,
    asr_start, asr_status, asr_stop, auth_bind_uid, auth_logout, auth_qrcode_cancel,
    auth_qrcode_generate, auth_query_uid, member_status, membership_cancel_order,
    membership_orders, membership_purchase, moderate_ignore, moderate_log_query,
    moderate_mark_false_positive, moderate_mute, moderate_set_auto, moderate_status,
    moderate_update_rules, module_cleanup, module_list, module_update, TauriMemberEventSink,
};
use ipc::release::{
    about_info, plugin_config_get, plugin_config_save, plugin_install, plugin_list,
    plugin_select_install, plugin_toggle, plugin_uninstall, resilience_status, update_check,
    update_download, update_install, update_skip, update_status, TauriReleaseEventSink,
};
use ipc::shell::{
    config_export, config_import, config_open_directory, config_overview, config_reset, locale_get,
    locale_save, locale_set_auto, shell_status, ui_get, ui_preview, ui_reset, ui_save,
    TauriShellEventSink,
};
use tauri::Manager;

pub struct AppState {
    application: ApplicationState,
    paths: PathState,
    live_data: Arc<LiveDataState>,
    media: Arc<MediaState>,
    ranking_presenter: Arc<RankingPresenter>,
    shell: Arc<ApplicationShellState>,
    release: Arc<ReleaseState>,
    #[cfg(feature = "member")]
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
        let shell = Arc::new(
            ApplicationShellState::initialize(
                shell_repository,
                Arc::new(NativeConfigArchiveFiles),
                Arc::new(NativeDirectoryOpener),
                system_metrics,
                Arc::new(NativeSystemLocale),
                Arc::new(TauriShellEventSink::new(app.clone())),
                config_directory,
            )
            .await,
        );
        let (update_provider, plugin_provider) = release_providers(app.clone());
        let release = Arc::new(ReleaseState::new(
            update_provider,
            plugin_provider,
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
            Arc::new(UnavailableSpeechEngine),
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
        let media = MediaState::new(effects, gifts, tts, recording);
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
            let ai = Arc::new(
                AiState::new(
                    repository.clone(),
                    Arc::new(UnavailableLlmProvider),
                    Arc::new(UnavailableAsrProvider),
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
            Arc::new(MemberStates::new(membership, ai, moderation))
        });
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
        let live_data =
            LiveDataState::new(Arc::new(UnavailableRoomTransport), repository, event_sink)
                .with_files(Arc::new(GrantedLiveFileStore::new(grants)));
        #[cfg(all(feature = "free", not(feature = "member")))]
        let live_data = live_data.with_accepted_consumer(media.clone());
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
                            media.clone()
                                as Arc<dyn application::live_data::AcceptedLiveEventConsumer>,
                            member.moderation.clone()
                                as Arc<dyn application::live_data::AcceptedLiveEventConsumer>,
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
            ranking_presenter,
            shell,
            release,
            #[cfg(feature = "member")]
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

    pub(crate) fn shell(&self) -> &Arc<ApplicationShellState> {
        &self.shell
    }

    pub(crate) fn release(&self) -> &Arc<ReleaseState> {
        &self.release
    }

    #[cfg(feature = "member")]
    pub(crate) fn member(&self) -> Result<&Arc<MemberStates>, ipc::error::DesktopError> {
        self.member.as_ref().ok_or_else(|| {
            ipc::error::DesktopError::new(
                "member_storage_unavailable",
                "会员本地数据暂不可用",
                true,
            )
        })
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let builder = tauri::Builder::default()
        .plugin(tauri_plugin_updater::Builder::new().build())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_notification::init())
        .plugin(tauri_plugin_global_shortcut::Builder::new().build())
        .register_asynchronous_uri_scheme_protocol("media-preview", handle_media_preview_protocol)
        .on_window_event(|window, event| {
            if window.label() == "main" && matches!(event, tauri::WindowEvent::Destroyed) {
                let media = window.state::<AppState>().media().clone();
                #[cfg(feature = "member")]
                let member = window.state::<AppState>().member.clone();
                tauri::async_runtime::spawn(async move {
                    media.shutdown().await;
                    #[cfg(feature = "member")]
                    if let Some(member) = member {
                        member.shutdown().await;
                    }
                });
            }
        })
        .setup(|app| {
            if cfg!(debug_assertions) {
                app.handle().plugin(
                    tauri_plugin_log::Builder::default()
                        .level(log::LevelFilter::Info)
                        .build(),
                )?;
            }

            let database_path = app.path().app_data_dir()?.join("app.db");
            let state =
                tauri::async_runtime::block_on(AppState::new(app.handle().clone(), database_path));
            app.manage(state);
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
        plugin_list,
        plugin_select_install,
        plugin_install,
        plugin_toggle,
        plugin_uninstall,
        plugin_config_get,
        plugin_config_save,
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
        captain_effect_get,
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
        recording_get_config,
        recording_save_config,
        recording_set_ffmpeg_source,
        recording_status,
        recording_start,
        recording_stop,
        recording_list,
        recording_preview,
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
        plugin_list,
        plugin_select_install,
        plugin_install,
        plugin_toggle,
        plugin_uninstall,
        plugin_config_get,
        plugin_config_save,
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
        ai_generate,
        ai_regenerate,
        ai_log_query,
        asr_status,
        asr_model_download,
        asr_start,
        asr_stop,
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
        recording_get_config,
        recording_save_config,
        recording_set_ffmpeg_source,
        recording_status,
        recording_start,
        recording_stop,
        recording_list,
        recording_preview,
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
