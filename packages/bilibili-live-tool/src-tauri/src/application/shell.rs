use std::{
    path::Path,
    sync::Arc,
    time::{SystemTime, UNIX_EPOCH},
};

use async_trait::async_trait;
use tokio::sync::RwLock;

use crate::domain::shell::{
    available_config_scopes, executable_config_scopes, validate_config_archive,
    validate_config_scopes, validate_ui_preferences, ConfigArchiveV1, LocaleCode,
    LocalePreferences, ShellConfigScope, ShellValidationError, UiPreferences,
};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SystemMetrics {
    pub memory_bytes: Option<u64>,
    pub cpu_percent: Option<f64>,
    pub observed_at: i64,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ConfigOverview {
    pub display_path: String,
    pub last_backup_at: Option<i64>,
    pub scopes: Vec<ShellConfigScope>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ConfigMutation {
    pub revision: String,
    pub scopes: Vec<ShellConfigScope>,
    pub display_path: Option<String>,
}

#[derive(Clone, Debug, Eq, PartialEq, thiserror::Error)]
#[error("{message}")]
pub struct ShellError {
    pub code: &'static str,
    pub message: String,
    pub retryable: bool,
}

impl ShellError {
    pub fn new(code: &'static str, message: impl Into<String>, retryable: bool) -> Self {
        Self {
            code,
            message: message.into(),
            retryable,
        }
    }
}

impl From<ShellValidationError> for ShellError {
    fn from(error: ShellValidationError) -> Self {
        let code = match error {
            ShellValidationError::InvalidColor => "invalid_ui_color",
            ShellValidationError::InvalidOpacity => "invalid_ui_opacity",
            ShellValidationError::InvalidLayout => "invalid_ui_layout",
            ShellValidationError::InvalidMaxEntries => "invalid_ui_max_entries",
            ShellValidationError::InvalidConfigScope => "invalid_config_scope",
        };
        Self::new(code, error.to_string(), false)
    }
}

#[async_trait]
pub trait ShellRepository: Send + Sync {
    async fn load_locale(&self) -> Result<Option<LocalePreferences>, ShellError>;
    async fn save_locale(&self, value: &LocalePreferences) -> Result<String, ShellError>;
    async fn load_ui(&self) -> Result<Option<UiPreferences>, ShellError>;
    async fn save_ui(&self, value: &UiPreferences) -> Result<String, ShellError>;
    async fn last_backup_at(&self) -> Result<Option<i64>, ShellError>;
    async fn save_last_backup_at(&self, value: i64) -> Result<(), ShellError>;
    async fn export_archive(
        &self,
        scopes: &[ShellConfigScope],
    ) -> Result<ConfigArchiveV1, ShellError>;
    async fn import_archive(&self, archive: &ConfigArchiveV1) -> Result<String, ShellError>;
    async fn reset(&self) -> Result<String, ShellError>;
}

#[async_trait]
pub trait ConfigArchiveFilePort: Send + Sync {
    async fn read(&self, path: &Path) -> Result<Vec<u8>, ShellError>;
    async fn write(&self, path: &Path, bytes: &[u8]) -> Result<(), ShellError>;
}

pub trait DirectoryOpenerPort: Send + Sync {
    fn open(&self, path: &Path) -> Result<(), ShellError>;
}

#[async_trait]
pub trait SystemMetricsPort: Send + Sync {
    async fn sample(&self) -> Result<SystemMetrics, ShellError>;
}

pub trait SystemLocalePort: Send + Sync {
    fn current(&self) -> Option<String>;
}

pub trait ShellEventSink: Send + Sync {
    fn publish_locale(&self, value: LocalePreferences) -> Result<(), ShellError>;
    fn publish_ui(&self, value: &UiPreferences) -> Result<(), ShellError>;
}

pub struct ApplicationShellState {
    repository: Arc<dyn ShellRepository>,
    files: Arc<dyn ConfigArchiveFilePort>,
    opener: Arc<dyn DirectoryOpenerPort>,
    metrics: Arc<dyn SystemMetricsPort>,
    system_locale: Arc<dyn SystemLocalePort>,
    events: Arc<dyn ShellEventSink>,
    config_directory: std::path::PathBuf,
    locale: RwLock<LocalePreferences>,
    ui: RwLock<UiPreferences>,
}

impl ApplicationShellState {
    pub async fn initialize(
        repository: Arc<dyn ShellRepository>,
        files: Arc<dyn ConfigArchiveFilePort>,
        opener: Arc<dyn DirectoryOpenerPort>,
        metrics: Arc<dyn SystemMetricsPort>,
        system_locale: Arc<dyn SystemLocalePort>,
        events: Arc<dyn ShellEventSink>,
        config_directory: std::path::PathBuf,
    ) -> Self {
        let locale = repository
            .load_locale()
            .await
            .ok()
            .flatten()
            .unwrap_or_default();
        let ui = repository
            .load_ui()
            .await
            .ok()
            .flatten()
            .and_then(|value| validate_ui_preferences(value).ok())
            .unwrap_or_default();
        Self {
            repository,
            files,
            opener,
            metrics,
            system_locale,
            events,
            config_directory,
            locale: RwLock::new(locale),
            ui: RwLock::new(ui),
        }
    }

    pub async fn status(&self) -> Result<SystemMetrics, ShellError> {
        self.metrics.sample().await
    }

    pub async fn config_overview(&self) -> Result<ConfigOverview, ShellError> {
        Ok(ConfigOverview {
            display_path: self.config_directory.to_string_lossy().into_owned(),
            last_backup_at: self.repository.last_backup_at().await?,
            scopes: available_config_scopes(),
        })
    }

    pub fn open_config_directory(&self) -> Result<(), ShellError> {
        self.opener.open(&self.config_directory)
    }

    pub async fn export_config(
        &self,
        path: &Path,
        scopes: &[ShellConfigScope],
    ) -> Result<ConfigMutation, ShellError> {
        let scopes = validate_config_scopes(scopes)?;
        let archive = self.repository.export_archive(&scopes).await?;
        let bytes = serde_json::to_vec_pretty(&archive)
            .map_err(|error| ShellError::new("CONFIG_EXPORT_FAILED", error.to_string(), false))?;
        self.files.write(path, &bytes).await?;
        let observed_at = now_millis();
        self.repository.save_last_backup_at(observed_at).await?;
        Ok(ConfigMutation {
            revision: observed_at.to_string(),
            scopes,
            display_path: Some(path.to_string_lossy().into_owned()),
        })
    }

    pub async fn import_config(&self, path: &Path) -> Result<ConfigMutation, ShellError> {
        let bytes = self.files.read(path).await?;
        let archive: ConfigArchiveV1 = serde_json::from_slice(&bytes).map_err(|_| {
            ShellError::new(
                "CONFIG_INVALID_FILE",
                "configuration file is damaged",
                false,
            )
        })?;
        let archive = validate_config_archive(archive)?;
        let scopes = archive
            .sections
            .iter()
            .map(|section| section.scope)
            .collect();
        let revision = self.repository.import_archive(&archive).await?;
        self.reload_saved().await;
        Ok(ConfigMutation {
            revision,
            scopes,
            display_path: None,
        })
    }

    pub async fn reset_config(&self) -> Result<ConfigMutation, ShellError> {
        let revision = self.repository.reset().await?;
        self.reload_saved().await;
        Ok(ConfigMutation {
            revision,
            scopes: executable_config_scopes(),
            display_path: None,
        })
    }

    pub async fn locale(&self) -> LocalePreferences {
        *self.locale.read().await
    }

    pub async fn set_locale_auto(&self, is_enabled: bool) -> Result<LocalePreferences, ShellError> {
        let mut value = *self.locale.read().await;
        value.follow_system = is_enabled;
        if is_enabled {
            value.locale = crate::domain::shell::resolve_system_locale(
                self.system_locale.current().as_deref(),
            );
        }
        self.save_locale(value).await
    }

    pub async fn save_locale(
        &self,
        value: LocalePreferences,
    ) -> Result<LocalePreferences, ShellError> {
        self.repository.save_locale(&value).await?;
        *self.locale.write().await = value;
        self.events.publish_locale(value)?;
        Ok(value)
    }

    pub async fn ui(&self) -> UiPreferences {
        self.ui.read().await.clone()
    }

    pub async fn preview_ui(&self, value: UiPreferences) -> Result<UiPreferences, ShellError> {
        validate_ui_preferences(value).map_err(Into::into)
    }

    pub async fn save_ui(&self, value: UiPreferences) -> Result<UiPreferences, ShellError> {
        let value = validate_ui_preferences(value)?;
        self.repository.save_ui(&value).await?;
        *self.ui.write().await = value.clone();
        self.events.publish_ui(&value)?;
        Ok(value)
    }

    pub async fn reset_ui(&self) -> Result<UiPreferences, ShellError> {
        self.save_ui(UiPreferences::default()).await
    }

    async fn reload_saved(&self) {
        let locale = self
            .repository
            .load_locale()
            .await
            .ok()
            .flatten()
            .unwrap_or_default();
        let ui = self
            .repository
            .load_ui()
            .await
            .ok()
            .flatten()
            .and_then(|value| validate_ui_preferences(value).ok())
            .unwrap_or_default();
        *self.locale.write().await = locale;
        *self.ui.write().await = ui;
    }
}

pub struct NoopShellEventSink;

impl ShellEventSink for NoopShellEventSink {
    fn publish_locale(&self, _: LocalePreferences) -> Result<(), ShellError> {
        Ok(())
    }
    fn publish_ui(&self, _: &UiPreferences) -> Result<(), ShellError> {
        Ok(())
    }
}

pub struct UnavailableShellRepository;

#[async_trait]
impl ShellRepository for UnavailableShellRepository {
    async fn load_locale(&self) -> Result<Option<LocalePreferences>, ShellError> {
        Ok(None)
    }
    async fn save_locale(&self, _: &LocalePreferences) -> Result<String, ShellError> {
        Err(storage_unavailable())
    }
    async fn load_ui(&self) -> Result<Option<UiPreferences>, ShellError> {
        Ok(None)
    }
    async fn save_ui(&self, _: &UiPreferences) -> Result<String, ShellError> {
        Err(storage_unavailable())
    }
    async fn last_backup_at(&self) -> Result<Option<i64>, ShellError> {
        Ok(None)
    }
    async fn save_last_backup_at(&self, _: i64) -> Result<(), ShellError> {
        Err(storage_unavailable())
    }
    async fn export_archive(&self, _: &[ShellConfigScope]) -> Result<ConfigArchiveV1, ShellError> {
        Err(storage_unavailable())
    }
    async fn import_archive(&self, _: &ConfigArchiveV1) -> Result<String, ShellError> {
        Err(storage_unavailable())
    }
    async fn reset(&self) -> Result<String, ShellError> {
        Err(storage_unavailable())
    }
}

pub struct UnavailableSystemMetrics;

#[async_trait]
impl SystemMetricsPort for UnavailableSystemMetrics {
    async fn sample(&self) -> Result<SystemMetrics, ShellError> {
        Err(ShellError::new(
            "SYS_METRIC_UNAVAILABLE",
            "system metrics are unavailable",
            true,
        ))
    }
}

fn storage_unavailable() -> ShellError {
    ShellError::new(
        "CONFIG_TRANSACTION_FAILED",
        "configuration storage is unavailable",
        true,
    )
}

pub fn locale_for_system(
    preferences: LocalePreferences,
    system_locale: Option<&str>,
) -> LocaleCode {
    if preferences.follow_system {
        crate::domain::shell::resolve_system_locale(system_locale)
    } else {
        preferences.locale
    }
}

fn now_millis() -> i64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis()
        .try_into()
        .unwrap_or(i64::MAX)
}
