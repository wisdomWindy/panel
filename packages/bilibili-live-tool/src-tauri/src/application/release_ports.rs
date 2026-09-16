use async_trait::async_trait;

use super::release::{PluginSnapshot, UpdateSnapshot};

pub trait ReleaseEventSink: Send + Sync {
    fn emit_update_status(&self, snapshot: &UpdateSnapshot);
    fn emit_plugin_status(&self, snapshot: &PluginSnapshot);
    fn emit_plugin_crashed(&self, snapshot: &PluginSnapshot);
}

#[derive(Clone, Debug, Eq, PartialEq, thiserror::Error)]
#[error("{code}")]
pub struct ReleaseServiceError {
    pub code: &'static str,
    pub retryable: bool,
}

impl ReleaseServiceError {
    pub const fn unavailable(code: &'static str) -> Self {
        Self {
            code,
            retryable: false,
        }
    }

    pub const fn invalid(code: &'static str) -> Self {
        Self {
            code,
            retryable: false,
        }
    }
}

#[async_trait]
pub trait UpdateProvider: Send + Sync {
    async fn check(&self) -> Result<UpdateSnapshot, ReleaseServiceError>;
    async fn download(&self) -> Result<UpdateSnapshot, ReleaseServiceError>;
    async fn install(&self) -> Result<UpdateSnapshot, ReleaseServiceError>;
    async fn skip(&self, version: &str) -> Result<UpdateSnapshot, ReleaseServiceError>;
}

#[async_trait]
pub trait PluginProvider: Send + Sync {
    async fn select_install(&self) -> Result<PluginSnapshot, ReleaseServiceError>;
    async fn install(&self, plugin_id: &str) -> Result<PluginSnapshot, ReleaseServiceError>;
    async fn toggle(
        &self,
        plugin_id: &str,
        is_enabled: bool,
    ) -> Result<PluginSnapshot, ReleaseServiceError>;
    async fn uninstall(&self, plugin_id: &str) -> Result<bool, ReleaseServiceError>;
}
