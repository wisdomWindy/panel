use std::sync::{
    atomic::{AtomicUsize, Ordering},
    Arc,
};

use bilibili_live_tool_lib::{
    application::{
        bootstrap::{ApplicationState, BeginBootstrap, BootstrapState, BootstrapStatus},
        config::ConfigError,
        paths::PathState,
        ports::{DatabasePort, StoragePortError, StoredConfig},
    },
    build_channel::{foundation_command_names, member_command_names, BuildChannel},
    domain::validation::{RoomId, SensitiveWord, Threshold, UserId, Username, Volume},
    infrastructure::{
        database::SqliteDatabase,
        path_grants::{PathGrantStore, PathPurpose},
    },
    ipc::dto::{AppStatusDto, ConfigGetRequestDto, ConfigSetRequestDto},
    ipc::error::DesktopError,
};

struct CountingDatabase {
    initialize_calls: Arc<AtomicUsize>,
}

#[async_trait::async_trait]
impl DatabasePort for CountingDatabase {
    async fn initialize(&self) -> Result<(), StoragePortError> {
        self.initialize_calls.fetch_add(1, Ordering::SeqCst);
        tokio::time::sleep(std::time::Duration::from_millis(20)).await;
        Ok(())
    }

    fn schema_version(&self) -> u32 {
        1
    }

    async fn get_config(
        &self,
        _key: &str,
        _scope: &str,
    ) -> Result<Option<StoredConfig>, StoragePortError> {
        unreachable!("configuration is not used by the bootstrap test")
    }

    async fn set_config(
        &self,
        _key: &str,
        _scope: &str,
        _value: &str,
    ) -> Result<String, StoragePortError> {
        unreachable!("configuration is not used by the bootstrap test")
    }
}

#[test]
fn validates_numeric_identifiers_at_the_prd_boundaries() {
    assert_eq!(RoomId::parse(" 123 ").unwrap().as_str(), "123");
    assert_eq!(UserId::parse("9").unwrap().as_str(), "9");
    assert!(RoomId::parse("").is_err());
    assert!(RoomId::parse("123456789012345678901").is_err());
    assert!(UserId::parse("１２３").is_err());
    assert!(UserId::parse("12a").is_err());
}

#[test]
fn normalizes_text_before_enforcing_length() {
    assert_eq!(
        Username::parse("\0  Alice  Bob \n").unwrap().as_str(),
        "Alice  Bob"
    );
    assert_eq!(
        SensitiveWord::parse("\u{0007} blocked ").unwrap().as_str(),
        "blocked"
    );
    assert!(Username::parse("\n\t").is_err());
    assert!(Username::parse(&"x".repeat(51)).is_err());
    assert!(SensitiveWord::parse(&"x".repeat(101)).is_err());
}

#[test]
fn validates_volume_and_threshold_without_clamping() {
    assert_eq!(Volume::parse(0.0).unwrap().get(), 0.0);
    assert_eq!(Volume::parse(1.0).unwrap().get(), 1.0);
    assert!(Volume::parse(-0.01).is_err());
    assert!(Volume::parse(1.01).is_err());
    assert!(Volume::parse(f64::NAN).is_err());
    assert_eq!(Threshold::parse(0).unwrap().get(), 0);
    assert!(Threshold::parse(-1).is_err());
}

#[test]
fn serializes_stable_errors_and_drops_sensitive_detail_keys() {
    let error = DesktopError::new("invalid_input", "Input is invalid", false)
        .with_detail("field", "roomId")
        .with_detail("token", "must-not-leak");
    let value = serde_json::to_value(error).unwrap();

    assert_eq!(value["code"], "invalid_input");
    assert_eq!(value["message"], "Input is invalid");
    assert_eq!(value["retryable"], false);
    assert_eq!(value["details"]["field"], "roomId");
    assert!(value["details"].get("token").is_none());
}

#[test]
fn bootstrap_state_prevents_overlapping_initializers_and_allows_retry() {
    let mut state = BootstrapState::default();
    assert_eq!(state.status(), BootstrapStatus::Uninitialized);
    assert_eq!(state.begin(), BeginBootstrap::Started);
    assert_eq!(state.begin(), BeginBootstrap::AlreadyInitializing);

    state.fail("storage_unavailable");
    assert_eq!(state.status(), BootstrapStatus::Degraded);
    assert_eq!(state.failure_code(), Some("storage_unavailable"));
    assert_eq!(state.begin(), BeginBootstrap::Started);

    state.complete();
    assert_eq!(state.status(), BootstrapStatus::Ready);
    assert_eq!(state.begin(), BeginBootstrap::AlreadyReady);
}

#[test]
fn build_channel_controls_member_command_availability() {
    assert_eq!(
        foundation_command_names(),
        [
            "app_bootstrap",
            "app_status",
            "config_get",
            "config_set",
            "path_select",
            "path_revoke",
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
            "rank_export"
        ]
    );

    #[cfg(feature = "free")]
    {
        assert_eq!(BuildChannel::current(), BuildChannel::Free);
        assert!(member_command_names().is_empty());
    }

    #[cfg(feature = "member")]
    {
        assert_eq!(BuildChannel::current(), BuildChannel::Member);
        assert_eq!(member_command_names().len(), 28);
        assert_eq!(member_command_names().first(), Some(&"member_status"));
        assert_eq!(
            member_command_names().last(),
            Some(&"moderate_mark_false_positive")
        );
        assert!(!member_command_names()
            .iter()
            .any(|command| command.contains("send")));
    }
}

#[test]
fn app_status_dto_matches_the_typescript_contract() {
    let value = serde_json::to_value(AppStatusDto::new(
        BuildChannel::current(),
        7,
        BootstrapStatus::Ready,
    ))
    .unwrap();

    #[cfg(feature = "free")]
    assert_eq!(value["buildChannel"], "free");

    #[cfg(feature = "member")]
    assert_eq!(value["buildChannel"], "member");

    assert_eq!(value["schemaVersion"], 7);
    assert_eq!(value["status"], "ready");
}

#[tokio::test]
async fn concurrent_application_bootstrap_calls_share_database_initialization() {
    let initialize_calls = Arc::new(AtomicUsize::new(0));
    let state = ApplicationState::new(CountingDatabase {
        initialize_calls: initialize_calls.clone(),
    });

    let (first, second) = tokio::join!(state.bootstrap(), state.bootstrap());
    let first = first.unwrap();
    let second = second.unwrap();

    assert_eq!(first.status, BootstrapStatus::Ready);
    assert_eq!(first.schema_version, 1);
    assert_eq!(second, first);
    assert_eq!(state.status().await, first);
    assert_eq!(initialize_calls.load(Ordering::SeqCst), 1);
}

#[tokio::test]
async fn database_failure_leaves_bootstrap_degraded_and_retryable() {
    let directory = tempfile::tempdir().unwrap();
    let invalid_parent = directory.path().join("not-a-directory");
    std::fs::write(&invalid_parent, "occupied").unwrap();
    let state = ApplicationState::new(SqliteDatabase::new(invalid_parent.join("app.db")));

    assert!(state.bootstrap().await.is_err());
    let status = state.status().await;
    assert_eq!(status.status, BootstrapStatus::Degraded);
    assert_eq!(status.schema_version, 0);
    assert!(state.bootstrap().await.is_err());
}

#[tokio::test]
async fn application_config_use_case_persists_only_valid_non_secret_keys() {
    let directory = tempfile::tempdir().unwrap();
    let state = ApplicationState::new(SqliteDatabase::new(directory.path().join("app.db")));
    state.bootstrap().await.unwrap();

    assert!(state
        .get_config("ui.theme", "global")
        .await
        .unwrap()
        .is_none());
    let revision = state
        .set_config("ui.theme", "global", "dark")
        .await
        .unwrap();
    let stored = state
        .get_config("ui.theme", "global")
        .await
        .unwrap()
        .unwrap();

    assert_eq!(stored.value, "dark");
    assert_eq!(stored.updated_at, revision);
    assert!(state
        .set_config("auth.token", "global", "secret")
        .await
        .is_err());
    assert!(state.get_config("", "global").await.is_err());
    assert!(state.get_config("ui.theme", "workspace").await.is_err());
}

#[test]
fn ipc_config_dtos_use_the_typescript_camel_case_contract() {
    let get: ConfigGetRequestDto = serde_json::from_value(serde_json::json!({
        "key": "ui.theme",
        "scope": "global"
    }))
    .unwrap();
    let set: ConfigSetRequestDto = serde_json::from_value(serde_json::json!({
        "key": "room.overlay",
        "scope": "room",
        "value": "compact"
    }))
    .unwrap();

    assert_eq!(get.key, "ui.theme");
    assert_eq!(get.scope, "global");
    assert_eq!(set.key, "room.overlay");
    assert_eq!(set.value, "compact");
}

#[test]
fn ipc_error_mapping_keeps_stable_codes_without_sensitive_details() {
    let invalid = serde_json::to_value(DesktopError::from(ConfigError::InvalidKey)).unwrap();
    let unavailable = serde_json::to_value(DesktopError::from(StoragePortError::unavailable(
        "database path cannot be opened",
    )))
    .unwrap();

    assert_eq!(invalid["code"], "invalid_config_key");
    assert_eq!(invalid["retryable"], false);
    assert_eq!(unavailable["code"], "storage_unavailable");
    assert_eq!(unavailable["retryable"], true);
    assert!(unavailable.get("details").is_none());
}

#[test]
fn path_grants_are_opaque_scoped_revocable_and_stale_safe() {
    let directory = tempfile::tempdir().unwrap();
    let selected = directory.path().join("selected");
    let outside = directory.path().join("outside");
    std::fs::create_dir_all(&selected).unwrap();
    std::fs::create_dir_all(&outside).unwrap();
    std::fs::write(selected.join("inside.txt"), "inside").unwrap();
    std::fs::write(outside.join("outside.txt"), "outside").unwrap();
    let store = PathGrantStore::new();

    assert!(store
        .grant_selection(None, PathPurpose::RecordingDirectory)
        .unwrap()
        .is_none());
    let grant = store
        .grant_selection(Some(selected.clone()), PathPurpose::RecordingDirectory)
        .unwrap()
        .unwrap();

    assert!(!grant.id().is_empty());
    assert_eq!(grant.display_path(), selected.to_string_lossy());
    assert_eq!(
        store
            .resolve_child(
                grant.id(),
                PathPurpose::RecordingDirectory,
                std::path::Path::new("inside.txt"),
            )
            .unwrap(),
        selected.join("inside.txt").canonicalize().unwrap()
    );
    assert!(store
        .resolve_root(grant.id(), PathPurpose::ExportDirectory)
        .is_err());
    assert!(store
        .resolve_child(
            grant.id(),
            PathPurpose::RecordingDirectory,
            std::path::Path::new("../outside/outside.txt"),
        )
        .is_err());
    assert!(store.revoke(grant.id()).unwrap());
    assert!(store
        .resolve_root(grant.id(), PathPurpose::RecordingDirectory)
        .is_err());
}

#[test]
fn path_grants_enforce_the_selected_path_kind() {
    let directory = tempfile::tempdir().unwrap();
    let selected_file = directory.path().join("sound.mp3");
    std::fs::write(&selected_file, "sound").unwrap();
    let store = PathGrantStore::new();

    assert!(store
        .grant_selection(
            Some(directory.path().to_path_buf()),
            PathPurpose::GiftSoundFile,
        )
        .is_err());
    assert!(store
        .grant_selection(Some(selected_file.clone()), PathPurpose::RecordingDirectory,)
        .is_err());
    assert!(store
        .grant_selection(Some(selected_file), PathPurpose::GiftSoundFile)
        .unwrap()
        .is_some());
}

#[test]
fn path_grants_reject_disappeared_roots_and_symlink_escapes() {
    let directory = tempfile::tempdir().unwrap();
    let selected = directory.path().join("selected");
    let outside = directory.path().join("outside");
    std::fs::create_dir_all(&selected).unwrap();
    std::fs::create_dir_all(&outside).unwrap();
    let store = PathGrantStore::new();
    let grant = store
        .grant_selection(Some(selected.clone()), PathPurpose::ExportDirectory)
        .unwrap()
        .unwrap();

    let link = selected.join("outside-link");
    #[cfg(windows)]
    let symlink_created = std::os::windows::fs::symlink_dir(&outside, &link).is_ok();
    #[cfg(unix)]
    let symlink_created = std::os::unix::fs::symlink(&outside, &link).is_ok();
    if symlink_created {
        assert!(store
            .resolve_child(
                grant.id(),
                PathPurpose::ExportDirectory,
                std::path::Path::new("outside-link"),
            )
            .is_err());
    }

    std::fs::remove_dir_all(&selected).unwrap();
    assert!(store
        .resolve_root(grant.id(), PathPurpose::ExportDirectory)
        .is_err());
}

#[test]
fn path_purpose_deserialization_is_a_closed_enum() {
    assert_eq!(
        serde_json::from_str::<PathPurpose>("\"giftSoundFile\"").unwrap(),
        PathPurpose::GiftSoundFile
    );
    assert!(serde_json::from_str::<PathPurpose>("\"modulesDirectory\"").is_err());
}

#[test]
fn application_path_use_case_returns_only_display_data_and_revoke_status() {
    let directory = tempfile::tempdir().unwrap();
    let selected = directory.path().join("exports");
    std::fs::create_dir_all(&selected).unwrap();
    let state = PathState::new(PathGrantStore::new());

    assert!(state
        .select(None, PathPurpose::ExportDirectory)
        .unwrap()
        .is_none());
    let grant = state
        .select(Some(selected.clone()), PathPurpose::ExportDirectory)
        .unwrap()
        .unwrap();
    assert_eq!(grant.display_path, selected.to_string_lossy());
    assert!(state.revoke(&grant.grant_id).unwrap());
    assert!(!state.revoke(&grant.grant_id).unwrap());
}

#[test]
fn capabilities_limit_foundation_commands_to_main_and_leave_effect_empty() {
    let manifest_dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
    let main: serde_json::Value = serde_json::from_str(
        &std::fs::read_to_string(manifest_dir.join("capabilities/main.json")).unwrap(),
    )
    .unwrap();
    let effect: serde_json::Value = serde_json::from_str(
        &std::fs::read_to_string(manifest_dir.join("capabilities/effect.json")).unwrap(),
    )
    .unwrap();
    let tauri_config: serde_json::Value = serde_json::from_str(
        &std::fs::read_to_string(manifest_dir.join("tauri.conf.json")).unwrap(),
    )
    .unwrap();
    let permission =
        std::fs::read_to_string(manifest_dir.join("permissions/foundation.toml")).unwrap();

    assert_eq!(main["windows"], serde_json::json!(["main"]));
    assert_eq!(
        main["permissions"],
        serde_json::json!(["core:default", "foundation"])
    );
    assert_eq!(effect["windows"], serde_json::json!(["effect-*"]));
    assert_eq!(effect["permissions"], serde_json::json!([]));
    assert_eq!(
        tauri_config["app"]["security"]["capabilities"],
        serde_json::json!(["main", "effect"])
    );
    for command in foundation_command_names() {
        assert!(permission.contains(command));
    }
    assert!(!permission.contains("send_danmaku"));
}
