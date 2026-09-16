use std::{
    collections::BTreeMap,
    sync::{
        atomic::{AtomicU64, Ordering},
        Arc, Mutex,
    },
};

use serde::Serialize;
use tokio::sync::RwLock;

use crate::{
    build_channel::BuildChannel,
    domain::release::{
        is_update_eligible, is_update_forced, is_valid_plugin_id, is_valid_update_candidate,
        PluginPermission, UpdateCandidate, UpdateStatus,
    },
};

use super::release_ports::{PluginProvider, ReleaseEventSink, ReleaseServiceError, UpdateProvider};

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateSnapshot {
    pub current_version: String,
    pub channel: BuildChannel,
    pub status: UpdateStatus,
    pub candidate: Option<UpdateCandidate>,
    pub progress_percent: Option<u8>,
    pub error_code: Option<String>,
}

impl UpdateSnapshot {
    pub fn unavailable() -> Self {
        Self {
            current_version: env!("CARGO_PKG_VERSION").to_owned(),
            channel: BuildChannel::current(),
            status: UpdateStatus::Unavailable,
            candidate: None,
            progress_percent: None,
            error_code: Some("update_provider_unavailable".into()),
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum PluginStatus {
    Disabled,
    Enabled,
    UpdateAvailable,
    Failed,
    Unavailable,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PluginSnapshot {
    pub id: String,
    pub name: String,
    pub version: String,
    pub status: PluginStatus,
    pub permissions: Vec<PluginPermission>,
    pub error_code: Option<String>,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AboutSnapshot {
    pub application_version: String,
    pub channel: BuildChannel,
    pub licenses: Vec<LicenseDescriptor>,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LicenseDescriptor {
    pub name: String,
    pub license: String,
    pub source_url: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ResilienceSnapshot {
    pub migration_backup_count: u8,
    pub last_migration_backup_at: Option<i64>,
    pub retention_policy_available: bool,
    pub ffmpeg_artifact_available: bool,
    pub whisper_artifact_available: bool,
}

impl ResilienceSnapshot {
    pub const fn unavailable() -> Self {
        Self {
            migration_backup_count: 0,
            last_migration_backup_at: None,
            retention_policy_available: false,
            ffmpeg_artifact_available: false,
            whisper_artifact_available: false,
        }
    }
}

pub struct ReleaseState {
    update: RwLock<UpdateSnapshot>,
    plugins: RwLock<BTreeMap<String, PluginSnapshot>>,
    update_provider: Arc<dyn UpdateProvider>,
    plugin_provider: Arc<dyn PluginProvider>,
    event_sink: Arc<dyn ReleaseEventSink>,
    installation_bucket: u8,
    update_generation: AtomicU64,
    plugin_generations: Mutex<BTreeMap<String, u64>>,
    resilience: ResilienceSnapshot,
}

impl ReleaseState {
    pub fn new(
        update_provider: Arc<dyn UpdateProvider>,
        plugin_provider: Arc<dyn PluginProvider>,
        event_sink: Arc<dyn ReleaseEventSink>,
        installation_bucket: u8,
        resilience: ResilienceSnapshot,
    ) -> Self {
        Self {
            update: RwLock::new(UpdateSnapshot::unavailable()),
            plugins: RwLock::new(BTreeMap::new()),
            update_provider,
            plugin_provider,
            event_sink,
            installation_bucket,
            update_generation: AtomicU64::new(0),
            plugin_generations: Mutex::new(BTreeMap::new()),
            resilience,
        }
    }

    pub async fn update_status(&self) -> UpdateSnapshot {
        self.update.read().await.clone()
    }

    pub async fn plugins(&self) -> Vec<PluginSnapshot> {
        self.plugins.read().await.values().cloned().collect()
    }

    pub async fn check_update(&self) -> Result<UpdateSnapshot, ReleaseServiceError> {
        let generation = self.begin_update_operation();
        let result = self.update_provider.check().await;
        if !self.is_current_update_operation(generation) {
            return Ok(self.update_status().await);
        }
        let snapshot = normalize_update_snapshot(result?);
        validate_update_snapshot(&snapshot)?;
        let snapshot = apply_update_exposure_policy(snapshot, self.installation_bucket);
        let mut current = self.update.write().await;
        if !self.is_current_update_operation(generation) {
            return Ok(current.clone());
        }
        *current = snapshot.clone();
        drop(current);
        self.event_sink.emit_update_status(&snapshot);
        Ok(snapshot)
    }

    pub async fn download_update(&self) -> Result<UpdateSnapshot, ReleaseServiceError> {
        let generation = self.begin_update_operation();
        let result = self.update_provider.download().await;
        if !self.is_current_update_operation(generation) {
            return Ok(self.update_status().await);
        }
        let snapshot = normalize_update_snapshot(result?);
        validate_update_snapshot(&snapshot)?;
        let mut current = self.update.write().await;
        if !self.is_current_update_operation(generation) {
            return Ok(current.clone());
        }
        *current = snapshot.clone();
        drop(current);
        self.event_sink.emit_update_status(&snapshot);
        Ok(snapshot)
    }

    pub async fn install_update(&self) -> Result<UpdateSnapshot, ReleaseServiceError> {
        if self.update.read().await.status != UpdateStatus::Downloaded {
            return Err(ReleaseServiceError::invalid("update_not_verified"));
        }
        let generation = self.begin_update_operation();
        let result = self.update_provider.install().await;
        if !self.is_current_update_operation(generation) {
            return Ok(self.update_status().await);
        }
        let snapshot = normalize_update_snapshot(result?);
        validate_update_snapshot(&snapshot)?;
        let mut current = self.update.write().await;
        if !self.is_current_update_operation(generation) {
            return Ok(current.clone());
        }
        *current = snapshot.clone();
        drop(current);
        self.event_sink.emit_update_status(&snapshot);
        Ok(snapshot)
    }

    pub async fn skip_update(&self, version: &str) -> Result<UpdateSnapshot, ReleaseServiceError> {
        if semver::Version::parse(version).is_err() {
            return Err(ReleaseServiceError::invalid("invalid_update_version"));
        }
        let generation = self.begin_update_operation();
        let result = self.update_provider.skip(version).await;
        if !self.is_current_update_operation(generation) {
            return Ok(self.update_status().await);
        }
        let snapshot = normalize_update_snapshot(result?);
        validate_update_snapshot(&snapshot)?;
        let mut current = self.update.write().await;
        if !self.is_current_update_operation(generation) {
            return Ok(current.clone());
        }
        *current = snapshot.clone();
        drop(current);
        self.event_sink.emit_update_status(&snapshot);
        Ok(snapshot)
    }

    pub async fn select_plugin_install(&self) -> Result<PluginSnapshot, ReleaseServiceError> {
        let plugin = self.plugin_provider.select_install().await?;
        validate_plugin_snapshot(&plugin)?;
        Ok(plugin)
    }

    pub async fn install_plugin(
        &self,
        plugin_id: &str,
    ) -> Result<PluginSnapshot, ReleaseServiceError> {
        validate_plugin_id(plugin_id)?;
        let generation = self.begin_plugin_operation(plugin_id);
        let result = self.plugin_provider.install(plugin_id).await;
        if !self.is_current_plugin_operation(plugin_id, generation) {
            return self.current_plugin(plugin_id).await;
        }
        let plugin = result?;
        validate_plugin_response(plugin_id, &plugin)?;
        let mut plugins = self.plugins.write().await;
        if !self.is_current_plugin_operation(plugin_id, generation) {
            return plugins
                .get(plugin_id)
                .cloned()
                .ok_or_else(|| ReleaseServiceError::invalid("stale_plugin_operation"));
        }
        plugins.insert(plugin.id.clone(), plugin.clone());
        drop(plugins);
        self.event_sink.emit_plugin_status(&plugin);
        Ok(plugin)
    }

    pub async fn toggle_plugin(
        &self,
        plugin_id: &str,
        is_enabled: bool,
    ) -> Result<PluginSnapshot, ReleaseServiceError> {
        validate_plugin_id(plugin_id)?;
        let generation = self.begin_plugin_operation(plugin_id);
        let result = self.plugin_provider.toggle(plugin_id, is_enabled).await;
        if !self.is_current_plugin_operation(plugin_id, generation) {
            return self.current_plugin(plugin_id).await;
        }
        let plugin = result?;
        validate_plugin_response(plugin_id, &plugin)?;
        let mut plugins = self.plugins.write().await;
        if !self.is_current_plugin_operation(plugin_id, generation) {
            return plugins
                .get(plugin_id)
                .cloned()
                .ok_or_else(|| ReleaseServiceError::invalid("stale_plugin_operation"));
        }
        plugins.insert(plugin.id.clone(), plugin.clone());
        drop(plugins);
        self.event_sink.emit_plugin_status(&plugin);
        Ok(plugin)
    }

    pub async fn uninstall_plugin(&self, plugin_id: &str) -> Result<bool, ReleaseServiceError> {
        validate_plugin_id(plugin_id)?;
        let generation = self.begin_plugin_operation(plugin_id);
        let result = self.plugin_provider.uninstall(plugin_id).await;
        if !self.is_current_plugin_operation(plugin_id, generation) {
            return Ok(false);
        }
        let removed = result?;
        let mut plugins = self.plugins.write().await;
        if !self.is_current_plugin_operation(plugin_id, generation) {
            return Ok(false);
        }
        if removed {
            plugins.remove(plugin_id);
        }
        Ok(removed)
    }

    pub fn about(&self) -> AboutSnapshot {
        AboutSnapshot {
            application_version: env!("CARGO_PKG_VERSION").to_owned(),
            channel: BuildChannel::current(),
            licenses: Vec::new(),
        }
    }

    pub fn resilience(&self) -> ResilienceSnapshot {
        self.resilience.clone()
    }

    fn begin_update_operation(&self) -> u64 {
        self.update_generation.fetch_add(1, Ordering::SeqCst) + 1
    }

    fn is_current_update_operation(&self, generation: u64) -> bool {
        self.update_generation.load(Ordering::SeqCst) == generation
    }

    fn begin_plugin_operation(&self, plugin_id: &str) -> u64 {
        let mut generations = self
            .plugin_generations
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner);
        let generation = generations.entry(plugin_id.to_owned()).or_default();
        *generation += 1;
        *generation
    }

    fn is_current_plugin_operation(&self, plugin_id: &str, generation: u64) -> bool {
        self.plugin_generations
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner)
            .get(plugin_id)
            .is_some_and(|current| *current == generation)
    }

    async fn current_plugin(&self, plugin_id: &str) -> Result<PluginSnapshot, ReleaseServiceError> {
        self.plugins
            .read()
            .await
            .get(plugin_id)
            .cloned()
            .ok_or_else(|| ReleaseServiceError::invalid("stale_plugin_operation"))
    }

    #[cfg(test)]
    async fn mark_crashed(&self, id: &str) {
        self.begin_plugin_operation(id);
        let crashed = if let Some(plugin) = self.plugins.write().await.get_mut(id) {
            plugin.status = PluginStatus::Failed;
            plugin.error_code = Some("plugin_runtime_crashed".into());
            Some(plugin.clone())
        } else {
            None
        };
        if let Some(plugin) = crashed {
            self.event_sink.emit_plugin_crashed(&plugin);
        }
    }
}

fn validate_update_snapshot(snapshot: &UpdateSnapshot) -> Result<(), ReleaseServiceError> {
    if semver::Version::parse(&snapshot.current_version).is_err()
        || snapshot
            .candidate
            .as_ref()
            .is_some_and(|candidate| !is_valid_update_candidate(candidate))
        || snapshot.progress_percent.is_some_and(|value| value > 100)
        || (snapshot.status == UpdateStatus::Downloaded && snapshot.candidate.is_none())
    {
        return Err(ReleaseServiceError::invalid("invalid_update_manifest"));
    }
    Ok(())
}

fn normalize_update_snapshot(mut snapshot: UpdateSnapshot) -> UpdateSnapshot {
    if let Some(candidate) = &mut snapshot.candidate {
        candidate.is_forced = is_update_forced(candidate, &snapshot.current_version);
    }
    snapshot
}

fn apply_update_exposure_policy(
    mut snapshot: UpdateSnapshot,
    installation_bucket: u8,
) -> UpdateSnapshot {
    if snapshot.status == UpdateStatus::Available
        && snapshot.candidate.as_ref().is_some_and(|candidate| {
            !is_update_eligible(candidate, &snapshot.current_version, installation_bucket)
        })
    {
        snapshot.status = UpdateStatus::Current;
        snapshot.candidate = None;
        snapshot.progress_percent = None;
        snapshot.error_code = None;
    }
    snapshot
}

fn validate_plugin_id(plugin_id: &str) -> Result<(), ReleaseServiceError> {
    if !is_valid_plugin_id(plugin_id) {
        return Err(ReleaseServiceError::invalid("invalid_plugin_id"));
    }
    Ok(())
}

fn validate_plugin_snapshot(plugin: &PluginSnapshot) -> Result<(), ReleaseServiceError> {
    validate_plugin_id(&plugin.id)?;
    if plugin.name.trim().is_empty() || semver::Version::parse(&plugin.version).is_err() {
        return Err(ReleaseServiceError::invalid("invalid_plugin_manifest"));
    }
    Ok(())
}

fn validate_plugin_response(
    requested_id: &str,
    plugin: &PluginSnapshot,
) -> Result<(), ReleaseServiceError> {
    validate_plugin_snapshot(plugin)?;
    if plugin.id != requested_id {
        return Err(ReleaseServiceError::invalid("plugin_identity_mismatch"));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use std::sync::{atomic::AtomicUsize, Mutex};

    use tokio::sync::Notify;

    use super::*;

    #[derive(Default)]
    struct RecordingEventSink {
        update_statuses: Mutex<Vec<UpdateSnapshot>>,
        plugin_statuses: Mutex<Vec<PluginSnapshot>>,
        plugin_crashes: Mutex<Vec<PluginSnapshot>>,
    }

    impl ReleaseEventSink for RecordingEventSink {
        fn emit_update_status(&self, snapshot: &UpdateSnapshot) {
            self.update_statuses.lock().unwrap().push(snapshot.clone());
        }

        fn emit_plugin_status(&self, snapshot: &PluginSnapshot) {
            self.plugin_statuses.lock().unwrap().push(snapshot.clone());
        }

        fn emit_plugin_crashed(&self, snapshot: &PluginSnapshot) {
            self.plugin_crashes.lock().unwrap().push(snapshot.clone());
        }
    }

    struct UnusedUpdateProvider;

    #[async_trait::async_trait]
    impl UpdateProvider for UnusedUpdateProvider {
        async fn check(&self) -> Result<UpdateSnapshot, ReleaseServiceError> {
            Ok(UpdateSnapshot::unavailable())
        }
        async fn download(&self) -> Result<UpdateSnapshot, ReleaseServiceError> {
            Err(ReleaseServiceError::unavailable("unused"))
        }
        async fn install(&self) -> Result<UpdateSnapshot, ReleaseServiceError> {
            Err(ReleaseServiceError::unavailable("unused"))
        }
        async fn skip(&self, _version: &str) -> Result<UpdateSnapshot, ReleaseServiceError> {
            Err(ReleaseServiceError::unavailable("unused"))
        }
    }

    struct RacingUpdateProvider {
        calls: AtomicUsize,
        first_started: Notify,
        release_first: Notify,
    }

    impl RacingUpdateProvider {
        fn new() -> Self {
            Self {
                calls: AtomicUsize::new(0),
                first_started: Notify::new(),
                release_first: Notify::new(),
            }
        }
    }

    #[async_trait::async_trait]
    impl UpdateProvider for RacingUpdateProvider {
        async fn check(&self) -> Result<UpdateSnapshot, ReleaseServiceError> {
            let call = self.calls.fetch_add(1, Ordering::SeqCst);
            if call == 0 {
                self.first_started.notify_one();
                self.release_first.notified().await;
                Err(ReleaseServiceError::unavailable("stale_failure"))
            } else {
                Ok(available_update("1.2.0", 100, false, false))
            }
        }

        async fn download(&self) -> Result<UpdateSnapshot, ReleaseServiceError> {
            Err(ReleaseServiceError::unavailable("unused"))
        }

        async fn install(&self) -> Result<UpdateSnapshot, ReleaseServiceError> {
            Err(ReleaseServiceError::unavailable("unused"))
        }

        async fn skip(&self, _version: &str) -> Result<UpdateSnapshot, ReleaseServiceError> {
            Err(ReleaseServiceError::unavailable("unused"))
        }
    }

    struct UnusedPluginProvider;

    #[async_trait::async_trait]
    impl PluginProvider for UnusedPluginProvider {
        async fn select_install(&self) -> Result<PluginSnapshot, ReleaseServiceError> {
            Err(ReleaseServiceError::unavailable("unused"))
        }
        async fn install(&self, _plugin_id: &str) -> Result<PluginSnapshot, ReleaseServiceError> {
            Err(ReleaseServiceError::unavailable("unused"))
        }
        async fn toggle(
            &self,
            _plugin_id: &str,
            _is_enabled: bool,
        ) -> Result<PluginSnapshot, ReleaseServiceError> {
            Err(ReleaseServiceError::unavailable("unused"))
        }
        async fn uninstall(&self, _plugin_id: &str) -> Result<bool, ReleaseServiceError> {
            Err(ReleaseServiceError::unavailable("unused"))
        }
    }

    fn state() -> ReleaseState {
        ReleaseState::new(
            Arc::new(UnusedUpdateProvider),
            Arc::new(UnusedPluginProvider),
            Arc::new(RecordingEventSink::default()),
            0,
            ResilienceSnapshot::unavailable(),
        )
    }

    fn available_update(
        version: &str,
        rollout_percent: u8,
        is_paused: bool,
        is_forced: bool,
    ) -> UpdateSnapshot {
        UpdateSnapshot {
            current_version: "1.0.0".into(),
            channel: BuildChannel::current(),
            status: UpdateStatus::Available,
            candidate: Some(UpdateCandidate {
                version: version.into(),
                release_notes: vec![],
                rollout_percent,
                is_paused,
                is_forced,
                blacklisted_versions: vec![],
            }),
            progress_percent: None,
            error_code: None,
        }
    }

    #[test]
    fn exposure_policy_hides_ineligible_candidates_and_preserves_forced_updates() {
        let hidden = apply_update_exposure_policy(available_update("1.1.0", 5, false, false), 5);
        assert_eq!(hidden.status, UpdateStatus::Current);
        assert!(hidden.candidate.is_none());

        let forced = apply_update_exposure_policy(available_update("1.1.0", 0, false, true), 99);
        assert_eq!(forced.status, UpdateStatus::Available);
        assert!(forced
            .candidate
            .is_some_and(|candidate| candidate.is_forced));

        let paused = apply_update_exposure_policy(available_update("1.1.0", 100, true, false), 0);
        assert_eq!(paused.status, UpdateStatus::Current);
    }

    #[tokio::test]
    async fn late_update_failure_cannot_replace_or_error_a_newer_generation() {
        let provider = Arc::new(RacingUpdateProvider::new());
        let event_sink = Arc::new(RecordingEventSink::default());
        let state = Arc::new(ReleaseState::new(
            provider.clone(),
            Arc::new(UnusedPluginProvider),
            event_sink.clone(),
            0,
            ResilienceSnapshot::unavailable(),
        ));

        let first_state = state.clone();
        let first = tokio::spawn(async move { first_state.check_update().await.unwrap() });
        provider.first_started.notified().await;
        let newest = state.check_update().await.unwrap();
        provider.release_first.notify_one();
        let stale_result = first.await.unwrap();

        assert_eq!(newest.candidate.as_ref().unwrap().version, "1.2.0");
        assert_eq!(stale_result, newest);
        assert_eq!(state.update_status().await, newest);
        let events = event_sink.update_statuses.lock().unwrap();
        assert_eq!(events.as_slice(), &[newest]);
    }

    #[tokio::test]
    async fn crashing_one_plugin_does_not_change_another_runtime() {
        let event_sink = Arc::new(RecordingEventSink::default());
        let state = ReleaseState::new(
            Arc::new(UnusedUpdateProvider),
            Arc::new(UnusedPluginProvider),
            event_sink.clone(),
            0,
            ResilienceSnapshot::unavailable(),
        );
        state.plugins.write().await.extend([
            (
                "one".into(),
                PluginSnapshot {
                    id: "one".into(),
                    name: "one".into(),
                    version: "1.0.0".into(),
                    status: PluginStatus::Enabled,
                    permissions: vec![],
                    error_code: None,
                },
            ),
            (
                "two".into(),
                PluginSnapshot {
                    id: "two".into(),
                    name: "two".into(),
                    version: "1.0.0".into(),
                    status: PluginStatus::Enabled,
                    permissions: vec![],
                    error_code: None,
                },
            ),
        ]);
        state.mark_crashed("one").await;
        let plugins = state.plugins().await;
        assert_eq!(plugins[0].status, PluginStatus::Failed);
        assert_eq!(plugins[1].status, PluginStatus::Enabled);
        let crashes = event_sink.plugin_crashes.lock().unwrap();
        assert_eq!(crashes.len(), 1);
        assert_eq!(crashes[0].id, "one");
    }

    #[tokio::test]
    async fn invalid_identifiers_fail_before_provider_calls() {
        let state = state();
        assert_eq!(
            state.install_plugin("../plugin").await.unwrap_err().code,
            "invalid_plugin_id"
        );
        assert_eq!(
            state.skip_update("latest").await.unwrap_err().code,
            "invalid_update_version"
        );
    }
}
