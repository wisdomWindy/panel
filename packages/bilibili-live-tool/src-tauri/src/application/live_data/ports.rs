use std::sync::Arc;

use tokio::sync::mpsc;

use crate::domain::live_data::{
    BlacklistEntry, LiveEvent, RankingSnapshot, RoomId, SensitiveRule, StatMetrics,
};

use super::RoomSnapshot;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SensitiveFileFormat {
    Csv,
    Json,
    Txt,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BlacklistImportEntry {
    pub uid: String,
    pub username: String,
    pub reason: Option<String>,
    pub created_at: Option<i64>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ExportReceipt {
    pub display_path: String,
    pub exported_count: u32,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ImportSummary {
    pub imported_count: u32,
    pub duplicate_count: u32,
}

#[derive(Clone, Debug, Eq, PartialEq, thiserror::Error)]
#[error("{message}")]
pub struct LiveRuntimeError {
    pub code: &'static str,
    pub message: String,
    pub retryable: bool,
}

impl LiveRuntimeError {
    pub fn new(code: &'static str, message: impl Into<String>, retryable: bool) -> Self {
        Self {
            code,
            message: message.into(),
            retryable,
        }
    }

    pub fn unavailable(message: impl Into<String>) -> Self {
        Self::new("external_service_unavailable", message, false)
    }

    pub fn storage(message: impl Into<String>) -> Self {
        Self::new("storage_unavailable", message, true)
    }
}

pub struct RoomConnection {
    pub room_id: RoomId,
    pub room_name: Option<String>,
    pub online_count: Option<u64>,
    pub events: mpsc::Receiver<LiveEvent>,
}

#[async_trait::async_trait]
pub trait RoomTransport: Send + Sync {
    async fn connect(&self, identity_code: &str) -> Result<RoomConnection, LiveRuntimeError>;
}

pub struct UnavailableRoomTransport;

#[async_trait::async_trait]
impl RoomTransport for UnavailableRoomTransport {
    async fn connect(&self, _: &str) -> Result<RoomConnection, LiveRuntimeError> {
        Err(LiveRuntimeError::unavailable(
            "Bilibili live transport is not configured",
        ))
    }
}

pub struct FailedRoomTransport {
    error: LiveRuntimeError,
}

impl FailedRoomTransport {
    pub fn new(error: LiveRuntimeError) -> Self {
        Self { error }
    }
}

#[async_trait::async_trait]
impl RoomTransport for FailedRoomTransport {
    async fn connect(&self, _: &str) -> Result<RoomConnection, LiveRuntimeError> {
        Err(self.error.clone())
    }
}

#[async_trait::async_trait]
pub trait LiveRepository: Send + Sync {
    async fn load_sensitive(&self) -> Result<Vec<SensitiveRule>, LiveRuntimeError>;
    async fn record_event(
        &self,
        event: &LiveEvent,
        filtered_reason: Option<&'static str>,
    ) -> Result<(), LiveRuntimeError>;

    async fn add_sensitive(&self, _: &SensitiveRule) -> Result<SensitiveRule, LiveRuntimeError> {
        Err(LiveRuntimeError::storage("operation is unavailable"))
    }

    async fn update_sensitive(&self, _: &SensitiveRule) -> Result<SensitiveRule, LiveRuntimeError> {
        Err(LiveRuntimeError::storage("operation is unavailable"))
    }

    async fn delete_sensitive(&self, _: i64) -> Result<bool, LiveRuntimeError> {
        Err(LiveRuntimeError::storage("operation is unavailable"))
    }

    async fn replace_sensitive(
        &self,
        _: &[SensitiveRule],
    ) -> Result<Vec<SensitiveRule>, LiveRuntimeError> {
        Err(LiveRuntimeError::storage("operation is unavailable"))
    }

    async fn list_blacklist_entries(&self) -> Result<Vec<BlacklistEntry>, LiveRuntimeError> {
        Err(LiveRuntimeError::storage("operation is unavailable"))
    }

    async fn today_sensitive_hits(&self) -> Result<u64, LiveRuntimeError> {
        Err(LiveRuntimeError::storage("operation is unavailable"))
    }

    async fn today_blacklist_additions(&self) -> Result<u64, LiveRuntimeError> {
        Err(LiveRuntimeError::storage("operation is unavailable"))
    }

    async fn upsert_blacklist(
        &self,
        _: &str,
        _: &str,
        _: Option<&str>,
    ) -> Result<BlacklistEntry, LiveRuntimeError> {
        Err(LiveRuntimeError::storage("operation is unavailable"))
    }

    async fn remove_blacklist(&self, _: &str) -> Result<bool, LiveRuntimeError> {
        Err(LiveRuntimeError::storage("operation is unavailable"))
    }

    async fn replace_blacklist(
        &self,
        _: &[BlacklistImportEntry],
    ) -> Result<Vec<BlacklistEntry>, LiveRuntimeError> {
        Err(LiveRuntimeError::storage("operation is unavailable"))
    }
}

pub trait LiveEventSink: Send + Sync {
    fn publish_room_status(&self, snapshot: &RoomSnapshot) -> Result<(), LiveRuntimeError>;
    fn publish_live_event(&self, event: &LiveEvent) -> Result<(), LiveRuntimeError>;

    fn publish_stat(&self, _: &StatMetrics) -> Result<(), LiveRuntimeError> {
        Ok(())
    }

    fn publish_ranking(&self, _: &RankingSnapshot) -> Result<(), LiveRuntimeError> {
        Ok(())
    }

    fn publish_sensitive_compile_progress(&self, _: u8, _: &str) -> Result<(), LiveRuntimeError> {
        Ok(())
    }

    fn publish_export(&self, _: &str, _: &str) -> Result<(), LiveRuntimeError> {
        Ok(())
    }
}

#[async_trait::async_trait]
pub trait AcceptedLiveEventConsumer: Send + Sync {
    async fn consume_accepted(&self, event: &LiveEvent) -> Result<(), LiveRuntimeError>;
}

pub struct NoopAcceptedLiveEventConsumer;

#[async_trait::async_trait]
impl AcceptedLiveEventConsumer for NoopAcceptedLiveEventConsumer {
    async fn consume_accepted(&self, _: &LiveEvent) -> Result<(), LiveRuntimeError> {
        Ok(())
    }
}

pub struct AcceptedLiveEventFanout {
    consumers: Vec<SharedAcceptedLiveEventConsumer>,
}

impl AcceptedLiveEventFanout {
    pub fn new(consumers: Vec<SharedAcceptedLiveEventConsumer>) -> Self {
        Self { consumers }
    }
}

#[async_trait::async_trait]
impl AcceptedLiveEventConsumer for AcceptedLiveEventFanout {
    async fn consume_accepted(&self, event: &LiveEvent) -> Result<(), LiveRuntimeError> {
        for consumer in &self.consumers {
            if let Err(error) = consumer.consume_accepted(event).await {
                log::warn!("accepted event consumer degraded: {}", error.code);
            }
        }
        Ok(())
    }
}

#[async_trait::async_trait]
pub trait LiveFilePort: Send + Sync {
    async fn read_sensitive(
        &self,
        grant_id: &str,
        format: SensitiveFileFormat,
    ) -> Result<Vec<SensitiveRule>, LiveRuntimeError>;
    async fn write_sensitive(
        &self,
        grant_id: &str,
        format: SensitiveFileFormat,
        rules: &[SensitiveRule],
    ) -> Result<ExportReceipt, LiveRuntimeError>;
    async fn read_blacklist(
        &self,
        grant_id: &str,
    ) -> Result<Vec<BlacklistImportEntry>, LiveRuntimeError>;
    async fn write_blacklist(
        &self,
        grant_id: &str,
        rows: &[BlacklistEntry],
    ) -> Result<ExportReceipt, LiveRuntimeError>;
    async fn write_statistics(
        &self,
        grant_id: &str,
        metrics: &StatMetrics,
        generated_at: i64,
    ) -> Result<ExportReceipt, LiveRuntimeError>;
    async fn write_rankings(
        &self,
        grant_id: &str,
        snapshot: &RankingSnapshot,
        generated_at: i64,
    ) -> Result<ExportReceipt, LiveRuntimeError>;
}

pub struct UnavailableLiveFilePort;

#[async_trait::async_trait]
impl LiveFilePort for UnavailableLiveFilePort {
    async fn read_sensitive(
        &self,
        _: &str,
        _: SensitiveFileFormat,
    ) -> Result<Vec<SensitiveRule>, LiveRuntimeError> {
        unavailable_file()
    }
    async fn write_sensitive(
        &self,
        _: &str,
        _: SensitiveFileFormat,
        _: &[SensitiveRule],
    ) -> Result<ExportReceipt, LiveRuntimeError> {
        unavailable_file()
    }
    async fn read_blacklist(&self, _: &str) -> Result<Vec<BlacklistImportEntry>, LiveRuntimeError> {
        unavailable_file()
    }
    async fn write_blacklist(
        &self,
        _: &str,
        _: &[BlacklistEntry],
    ) -> Result<ExportReceipt, LiveRuntimeError> {
        unavailable_file()
    }
    async fn write_statistics(
        &self,
        _: &str,
        _: &StatMetrics,
        _: i64,
    ) -> Result<ExportReceipt, LiveRuntimeError> {
        unavailable_file()
    }
    async fn write_rankings(
        &self,
        _: &str,
        _: &RankingSnapshot,
        _: i64,
    ) -> Result<ExportReceipt, LiveRuntimeError> {
        unavailable_file()
    }
}

fn unavailable_file<T>() -> Result<T, LiveRuntimeError> {
    Err(LiveRuntimeError::new(
        "path_authorization_failed",
        "file access is unavailable",
        false,
    ))
}

pub type SharedRoomTransport = Arc<dyn RoomTransport>;
pub type SharedLiveRepository = Arc<dyn LiveRepository>;
pub type SharedLiveEventSink = Arc<dyn LiveEventSink>;
pub type SharedLiveFilePort = Arc<dyn LiveFilePort>;
pub type SharedAcceptedLiveEventConsumer = Arc<dyn AcceptedLiveEventConsumer>;
