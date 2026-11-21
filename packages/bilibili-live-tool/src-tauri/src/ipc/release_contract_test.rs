use crate::{
    application::release::{AboutSnapshot, ResilienceSnapshot, UpdateSnapshot},
    build_channel::{release_command_names, BuildChannel},
    domain::release::UpdateStatus,
};

#[test]
fn registers_the_exact_common_release_surface() {
    assert_eq!(
        release_command_names(),
        [
            "update_status",
            "update_check",
            "update_download",
            "update_install",
            "update_skip",
            "about_info",
            "resilience_status",
        ]
    );
}

#[test]
fn safe_release_snapshots_serialize_without_paths_or_secrets() {
    let update = UpdateSnapshot {
        current_version: "0.1.0".into(),
        channel: BuildChannel::current(),
        status: UpdateStatus::Unavailable,
        candidate: None,
        progress_percent: None,
        error_code: Some("UPDATE_PROVIDER_UNAVAILABLE".into()),
    };
    let about = AboutSnapshot {
        application_version: "0.1.0".into(),
        channel: BuildChannel::current(),
        licenses: vec![],
    };
    let resilience = ResilienceSnapshot {
        migration_backup_count: 0,
        last_migration_backup_at: None,
        retention_policy_available: false,
        ffmpeg_artifact_available: false,
    };
    let serialized = format!(
        "{}{}{}",
        serde_json::to_string(&update).unwrap(),
        serde_json::to_string(&about).unwrap(),
        serde_json::to_string(&resilience).unwrap()
    );

    assert!(!serialized.contains("path"));
    assert!(!serialized.contains("token"));
    assert!(!serialized.contains("cookie"));
    assert!(!serialized.contains("apiKey"));
}
