use super::release::UpdateSnapshot;
use async_trait::async_trait;

pub trait ReleaseEventSink: Send + Sync {
    fn emit_update_status(&self, snapshot: &UpdateSnapshot);
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
