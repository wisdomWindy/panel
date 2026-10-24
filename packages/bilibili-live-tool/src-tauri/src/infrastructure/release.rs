use std::{
    path::Path,
    sync::{
        atomic::{AtomicU64, Ordering},
        Arc,
    },
};

use async_trait::async_trait;
use sha2::{Digest, Sha256};
use tauri::AppHandle;
use tauri_plugin_updater::{Update, UpdaterExt};
use tokio::sync::Mutex;

use crate::application::{
    release::{ResilienceSnapshot, UpdateSnapshot},
    release_ports::{ReleaseServiceError, UpdateProvider},
};
use crate::{
    build_channel::BuildChannel,
    domain::release::{UpdateCandidate, UpdateStatus},
};

#[derive(Default)]
struct PendingUpdate {
    generation: u64,
    update: Option<Update>,
    bytes: Option<Vec<u8>>,
    candidate: Option<UpdateCandidate>,
}

pub struct TauriUpdateProvider {
    app: AppHandle,
    next_generation: AtomicU64,
    pending: Mutex<PendingUpdate>,
}

impl TauriUpdateProvider {
    pub fn new(app: AppHandle) -> Self {
        Self {
            app,
            next_generation: AtomicU64::new(0),
            pending: Mutex::new(PendingUpdate::default()),
        }
    }

    async fn ensure_current_generation(&self, generation: u64) -> Result<(), ReleaseServiceError> {
        if self.pending.lock().await.generation != generation {
            return Err(ReleaseServiceError::invalid("stale_update_check"));
        }
        Ok(())
    }
}

#[async_trait]
impl UpdateProvider for TauriUpdateProvider {
    async fn check(&self) -> Result<UpdateSnapshot, ReleaseServiceError> {
        let generation = self.next_generation.fetch_add(1, Ordering::SeqCst) + 1;
        *self.pending.lock().await = PendingUpdate {
            generation,
            ..PendingUpdate::default()
        };
        let updater = self
            .app
            .updater()
            .map_err(|_| ReleaseServiceError::unavailable("update_provider_unavailable"))?;
        let update = updater
            .check()
            .await
            .map_err(|error| map_update_check_error(&error))?;

        let Some(update) = update else {
            self.ensure_current_generation(generation).await?;
            return Ok(current_update_snapshot());
        };
        let candidate =
            candidate_from_metadata(&update.version, update.body.as_deref(), &update.raw_json)?;
        let mut pending = self.pending.lock().await;
        if pending.generation != generation {
            return Err(ReleaseServiceError::invalid("stale_update_check"));
        }
        *pending = PendingUpdate {
            generation,
            update: Some(update),
            bytes: None,
            candidate: Some(candidate.clone()),
        };
        Ok(candidate_snapshot(UpdateStatus::Available, candidate, None))
    }

    async fn download(&self) -> Result<UpdateSnapshot, ReleaseServiceError> {
        let (generation, update, candidate) =
            {
                let pending = self.pending.lock().await;
                (
                    pending.generation,
                    pending.update.clone().ok_or_else(|| {
                        ReleaseServiceError::invalid("update_candidate_unavailable")
                    })?,
                    pending.candidate.clone().ok_or_else(|| {
                        ReleaseServiceError::invalid("update_candidate_unavailable")
                    })?,
                )
            };
        let bytes = update
            .download(|_, _| {}, || {})
            .await
            .map_err(|_| ReleaseServiceError::unavailable("update_download_failed"))?;

        let mut pending = self.pending.lock().await;
        if pending.generation != generation {
            return Err(ReleaseServiceError::invalid("stale_update_download"));
        }
        pending.bytes = Some(bytes);
        Ok(candidate_snapshot(
            UpdateStatus::Downloaded,
            candidate,
            Some(100),
        ))
    }

    async fn install(&self) -> Result<UpdateSnapshot, ReleaseServiceError> {
        let mut pending = self.pending.lock().await;
        let update = pending
            .update
            .clone()
            .ok_or_else(|| ReleaseServiceError::invalid("update_not_verified"))?;
        let bytes = pending
            .bytes
            .clone()
            .ok_or_else(|| ReleaseServiceError::invalid("update_not_verified"))?;
        update
            .install(&bytes)
            .map_err(|_| ReleaseServiceError::unavailable("update_install_failed"))?;
        *pending = PendingUpdate::default();
        Ok(current_update_snapshot())
    }

    async fn skip(&self, version: &str) -> Result<UpdateSnapshot, ReleaseServiceError> {
        let mut pending = self.pending.lock().await;
        if pending
            .candidate
            .as_ref()
            .is_none_or(|candidate| candidate.version != version)
        {
            return Err(ReleaseServiceError::invalid("update_candidate_mismatch"));
        }
        *pending = PendingUpdate::default();
        Ok(current_update_snapshot())
    }
}

fn map_update_check_error(error: &tauri_plugin_updater::Error) -> ReleaseServiceError {
    if matches!(
        error,
        tauri_plugin_updater::Error::Reqwest(_)
            | tauri_plugin_updater::Error::Network(_)
            | tauri_plugin_updater::Error::ReleaseNotFound
    ) {
        return ReleaseServiceError {
            code: "update_check_failed",
            retryable: true,
        };
    }
    ReleaseServiceError::invalid("invalid_update_manifest")
}

fn candidate_from_metadata(
    version: &str,
    body: Option<&str>,
    raw_json: &serde_json::Value,
) -> Result<UpdateCandidate, ReleaseServiceError> {
    let rollout_percent = optional_u8(raw_json, "rolloutPercent")?.unwrap_or(100);
    let is_paused = optional_bool(raw_json, "isPaused")?.unwrap_or(false);
    let is_forced = optional_bool(raw_json, "isForced")?.unwrap_or(false);
    let blacklisted_versions = match raw_json.get("blacklistedVersions") {
        None => Vec::new(),
        Some(value) => value
            .as_array()
            .ok_or_else(|| ReleaseServiceError::invalid("invalid_update_manifest"))?
            .iter()
            .map(|entry| {
                entry
                    .as_str()
                    .map(str::to_owned)
                    .ok_or_else(|| ReleaseServiceError::invalid("invalid_update_manifest"))
            })
            .collect::<Result<Vec<_>, _>>()?,
    };

    Ok(UpdateCandidate {
        version: version.to_owned(),
        release_notes: body
            .into_iter()
            .flat_map(str::lines)
            .map(str::trim)
            .filter(|line| !line.is_empty())
            .map(str::to_owned)
            .collect(),
        rollout_percent,
        is_paused,
        is_forced,
        blacklisted_versions,
    })
}

fn optional_u8(value: &serde_json::Value, key: &str) -> Result<Option<u8>, ReleaseServiceError> {
    value
        .get(key)
        .map(|value| {
            value
                .as_u64()
                .filter(|value| *value <= 100)
                .and_then(|value| u8::try_from(value).ok())
                .ok_or_else(|| ReleaseServiceError::invalid("invalid_update_manifest"))
        })
        .transpose()
}

fn optional_bool(
    value: &serde_json::Value,
    key: &str,
) -> Result<Option<bool>, ReleaseServiceError> {
    value
        .get(key)
        .map(|value| {
            value
                .as_bool()
                .ok_or_else(|| ReleaseServiceError::invalid("invalid_update_manifest"))
        })
        .transpose()
}

fn current_update_snapshot() -> UpdateSnapshot {
    UpdateSnapshot {
        current_version: env!("CARGO_PKG_VERSION").to_owned(),
        channel: BuildChannel::current(),
        status: UpdateStatus::Current,
        candidate: None,
        progress_percent: None,
        error_code: None,
    }
}

fn candidate_snapshot(
    status: UpdateStatus,
    candidate: UpdateCandidate,
    progress_percent: Option<u8>,
) -> UpdateSnapshot {
    UpdateSnapshot {
        current_version: env!("CARGO_PKG_VERSION").to_owned(),
        channel: BuildChannel::current(),
        status,
        candidate: Some(candidate),
        progress_percent,
        error_code: None,
    }
}

pub fn update_provider(app: AppHandle) -> Arc<dyn UpdateProvider> {
    Arc::new(TauriUpdateProvider::new(app))
}

pub fn installation_bucket(database_path: &Path) -> u8 {
    let digest = Sha256::digest(database_path.as_os_str().to_string_lossy().as_bytes());
    let prefix = u64::from_be_bytes(
        digest[..8]
            .try_into()
            .expect("SHA-256 prefix is eight bytes"),
    );
    (prefix % 100) as u8
}

pub fn migration_resilience_snapshot(database_path: &Path) -> ResilienceSnapshot {
    let Some(parent) = database_path.parent() else {
        return ResilienceSnapshot::unavailable();
    };
    let Some(database_name) = database_path.file_name().and_then(|name| name.to_str()) else {
        return ResilienceSnapshot::unavailable();
    };
    let prefix = format!("{database_name}.pre-migration.");
    let timestamps = std::fs::read_dir(parent)
        .into_iter()
        .flatten()
        .filter_map(Result::ok)
        .filter(|entry| entry.file_type().is_ok_and(|kind| kind.is_file()))
        .filter_map(|entry| entry.file_name().to_str().map(str::to_owned))
        .filter_map(|name| {
            let timestamp = name.strip_prefix(&prefix)?.strip_suffix(".bak")?;
            (timestamp.len() == 20 && timestamp.bytes().all(|byte| byte.is_ascii_digit()))
                .then(|| timestamp.parse::<i64>().ok())
                .flatten()
        })
        .collect::<Vec<_>>();

    ResilienceSnapshot {
        migration_backup_count: u8::try_from(timestamps.len()).unwrap_or(u8::MAX),
        last_migration_backup_at: timestamps.into_iter().max(),
        retention_policy_available: true,
        ffmpeg_artifact_available: false,
        whisper_artifact_available: false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reports_only_exact_migration_backups() {
        let directory = tempfile::tempdir().unwrap();
        let database = directory.path().join("app.db");
        for name in [
            "app.db.pre-migration.00000000000000000001.bak",
            "app.db.pre-migration.00000000000000000002.bak",
            "app.db.pre-migration.invalid.bak",
            "other.db.pre-migration.00000000000000000003.bak",
        ] {
            std::fs::write(directory.path().join(name), []).unwrap();
        }
        std::fs::create_dir(
            directory
                .path()
                .join("app.db.pre-migration.00000000000000000004.bak"),
        )
        .unwrap();

        assert_eq!(
            migration_resilience_snapshot(&database),
            ResilienceSnapshot {
                migration_backup_count: 2,
                last_migration_backup_at: Some(2),
                retention_policy_available: true,
                ffmpeg_artifact_available: false,
                whisper_artifact_available: false,
            }
        );
    }

    #[test]
    fn installation_bucket_is_stable_and_bounded() {
        let path = Path::new("stable/app.db");
        let first = installation_bucket(path);
        assert_eq!(first, installation_bucket(path));
        assert!(first < 100);
    }

    #[test]
    fn updater_metadata_defaults_are_safe_and_custom_rollout_is_strict() {
        let defaults = candidate_from_metadata(
            "1.2.0",
            Some("First line\n\nSecond line"),
            &serde_json::json!({}),
        )
        .unwrap();
        assert_eq!(defaults.rollout_percent, 100);
        assert_eq!(defaults.release_notes, ["First line", "Second line"]);
        assert!(!defaults.is_paused);

        let custom = candidate_from_metadata(
            "1.3.0",
            None,
            &serde_json::json!({
                "rolloutPercent": 15,
                "isPaused": true,
                "isForced": true,
                "blacklistedVersions": ["1.0.0"]
            }),
        )
        .unwrap();
        assert_eq!(custom.rollout_percent, 15);
        assert!(custom.is_paused);
        assert!(custom.is_forced);
        assert_eq!(custom.blacklisted_versions, ["1.0.0"]);

        assert_eq!(
            candidate_from_metadata("1.3.0", None, &serde_json::json!({ "rolloutPercent": 101 }))
                .unwrap_err()
                .code,
            "invalid_update_manifest"
        );
    }

    #[test]
    fn updater_check_errors_preserve_transient_and_configuration_semantics() {
        assert_eq!(
            map_update_check_error(&tauri_plugin_updater::Error::ReleaseNotFound),
            ReleaseServiceError {
                code: "update_check_failed",
                retryable: true,
            }
        );
        assert_eq!(
            map_update_check_error(&tauri_plugin_updater::Error::EmptyEndpoints),
            ReleaseServiceError::invalid("invalid_update_manifest")
        );
    }
}
