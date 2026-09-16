use std::sync::Arc;

use serde::Serialize;
use tokio::sync::{Mutex, Notify};

use crate::{
    application::config::{ConfigError, ConfigKey, ConfigScope, ConfigValue},
    application::ports::{DatabasePort, StoragePortError},
    build_channel::BuildChannel,
};

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum BootstrapStatus {
    #[default]
    Uninitialized,
    Initializing,
    Ready,
    Degraded,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum BeginBootstrap {
    Started,
    AlreadyInitializing,
    AlreadyReady,
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct BootstrapState {
    status: BootstrapStatus,
    failure_code: Option<String>,
}

impl BootstrapState {
    pub const fn status(&self) -> BootstrapStatus {
        self.status
    }

    pub fn failure_code(&self) -> Option<&str> {
        self.failure_code.as_deref()
    }

    pub fn begin(&mut self) -> BeginBootstrap {
        match self.status {
            BootstrapStatus::Uninitialized | BootstrapStatus::Degraded => {
                self.status = BootstrapStatus::Initializing;
                self.failure_code = None;
                BeginBootstrap::Started
            }
            BootstrapStatus::Initializing => BeginBootstrap::AlreadyInitializing,
            BootstrapStatus::Ready => BeginBootstrap::AlreadyReady,
        }
    }

    pub fn complete(&mut self) {
        if self.status == BootstrapStatus::Initializing {
            self.status = BootstrapStatus::Ready;
            self.failure_code = None;
        }
    }

    pub fn fail(&mut self, failure_code: impl Into<String>) {
        if self.status == BootstrapStatus::Initializing {
            self.status = BootstrapStatus::Degraded;
            self.failure_code = Some(failure_code.into());
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ApplicationStatus {
    pub build_channel: BuildChannel,
    pub schema_version: u32,
    pub status: BootstrapStatus,
}

pub struct ApplicationState {
    database: Arc<dyn DatabasePort>,
    bootstrap: Mutex<BootstrapState>,
    bootstrap_changed: Notify,
}

impl ApplicationState {
    pub fn new(database: impl DatabasePort + 'static) -> Self {
        Self::from_arc(Arc::new(database))
    }

    pub fn from_arc(database: Arc<dyn DatabasePort>) -> Self {
        Self {
            database,
            bootstrap: Mutex::new(BootstrapState::default()),
            bootstrap_changed: Notify::new(),
        }
    }

    pub async fn bootstrap(&self) -> Result<ApplicationStatus, StoragePortError> {
        loop {
            let changed = self.bootstrap_changed.notified();
            tokio::pin!(changed);
            changed.as_mut().enable();
            let begin = {
                let mut bootstrap = self.bootstrap.lock().await;
                bootstrap.begin()
            };
            match begin {
                BeginBootstrap::Started => break,
                BeginBootstrap::AlreadyReady => return Ok(self.status().await),
                BeginBootstrap::AlreadyInitializing => {
                    changed.as_mut().await;
                    let status = self.status().await;
                    match status.status {
                        BootstrapStatus::Ready => return Ok(status),
                        BootstrapStatus::Degraded => {
                            return Err(StoragePortError::unavailable(
                                "concurrent bootstrap attempt failed",
                            ));
                        }
                        BootstrapStatus::Uninitialized | BootstrapStatus::Initializing => {}
                    }
                }
            }
        }

        match self.database.initialize().await {
            Ok(()) => {
                self.bootstrap.lock().await.complete();
                self.bootstrap_changed.notify_waiters();
                Ok(self.status().await)
            }
            Err(error) => {
                self.bootstrap.lock().await.fail(error.code);
                self.bootstrap_changed.notify_waiters();
                Err(error)
            }
        }
    }

    pub async fn status(&self) -> ApplicationStatus {
        let status = self.bootstrap.lock().await.status();
        ApplicationStatus {
            build_channel: BuildChannel::current(),
            schema_version: self.database.schema_version(),
            status,
        }
    }

    pub async fn get_config(
        &self,
        key: &str,
        scope: &str,
    ) -> Result<Option<ConfigValue>, ConfigError> {
        let key = ConfigKey::parse(key)?;
        let scope = ConfigScope::parse(scope)?;
        self.database
            .get_config(key.as_str(), scope.as_str())
            .await
            .map(|stored| {
                stored.map(|stored| ConfigValue {
                    value: stored.value,
                    updated_at: stored.updated_at,
                })
            })
            .map_err(ConfigError::from)
    }

    pub async fn set_config(
        &self,
        key: &str,
        scope: &str,
        value: &str,
    ) -> Result<String, ConfigError> {
        let key = ConfigKey::parse(key)?;
        let scope = ConfigScope::parse(scope)?;
        self.database
            .set_config(key.as_str(), scope.as_str(), value)
            .await
            .map_err(ConfigError::from)
    }
}
