#[derive(Clone, Debug, Eq, PartialEq, thiserror::Error)]
#[error("{message}")]
pub struct StoragePortError {
    pub code: &'static str,
    pub message: String,
    pub retryable: bool,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct StoredConfig {
    pub value: String,
    pub updated_at: String,
}

impl StoragePortError {
    pub fn unavailable(message: impl Into<String>) -> Self {
        Self {
            code: "storage_unavailable",
            message: message.into(),
            retryable: true,
        }
    }
}

#[async_trait::async_trait]
pub trait DatabasePort: Send + Sync {
    async fn initialize(&self) -> Result<(), StoragePortError>;
    fn schema_version(&self) -> u32;
    async fn get_config(
        &self,
        key: &str,
        scope: &str,
    ) -> Result<Option<StoredConfig>, StoragePortError>;
    async fn set_config(
        &self,
        key: &str,
        scope: &str,
        value: &str,
    ) -> Result<String, StoragePortError>;
}

#[derive(Clone, Debug, Eq, PartialEq, thiserror::Error)]
#[error("path authorization is unavailable")]
pub struct PathPortError;

pub trait PathGrantPort: Send + Sync {
    fn grant_selection(
        &self,
        selection: Option<PathBuf>,
        purpose: PathPurpose,
    ) -> Result<Option<PathGrantView>, PathPortError>;
    fn revoke(&self, grant_id: &str) -> Result<bool, PathPortError>;
    fn resolve(&self, grant_id: &str, purpose: PathPurpose) -> Result<PathBuf, PathPortError>;
}
use std::path::PathBuf;

use crate::application::paths::{PathGrantView, PathPurpose};
