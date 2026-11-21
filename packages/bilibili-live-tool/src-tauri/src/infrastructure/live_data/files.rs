use std::sync::Arc;

use crate::{
    application::{
        live_data::{
            BlacklistImportEntry, ExportReceipt, LiveFilePort, LiveRuntimeError,
            SensitiveFileFormat,
        },
        paths::PathPurpose,
    },
    domain::live_data::{BlacklistEntry, RankingSnapshot, SensitiveRule, StatMetrics},
    infrastructure::path_grants::PathGrantStore,
};

use super::file_codec::{
    decode_blacklist_csv, decode_sensitive_words, encode_blacklist_csv, encode_sensitive_words,
    BlacklistFileRow,
};

pub struct GrantedLiveFileStore {
    grants: Arc<PathGrantStore>,
}

impl GrantedLiveFileStore {
    pub const fn new(grants: Arc<PathGrantStore>) -> Self {
        Self { grants }
    }

    async fn read(
        &self,
        grant_id: &str,
        purpose: PathPurpose,
    ) -> Result<Vec<u8>, LiveRuntimeError> {
        let path = self
            .grants
            .resolve_root(grant_id, purpose)
            .map_err(map_path_error)?;
        tokio::fs::read(path)
            .await
            .map_err(|error| map_io_error("STORAGE_READ_FAILED", error))
    }

    async fn write(
        &self,
        grant_id: &str,
        purpose: PathPurpose,
        content: &[u8],
        exported_count: u32,
    ) -> Result<ExportReceipt, LiveRuntimeError> {
        let path = self
            .grants
            .resolve_root(grant_id, purpose)
            .map_err(map_path_error)?;
        tokio::fs::write(&path, content)
            .await
            .map_err(|error| map_io_error("STORAGE_WRITE_FAILED", error))?;
        Ok(ExportReceipt {
            display_path: path.to_string_lossy().into_owned(),
            exported_count,
        })
    }
}

#[async_trait::async_trait]
impl LiveFilePort for GrantedLiveFileStore {
    async fn read_sensitive(
        &self,
        grant_id: &str,
        format: SensitiveFileFormat,
    ) -> Result<Vec<SensitiveRule>, LiveRuntimeError> {
        let content = self
            .read(grant_id, PathPurpose::SensitiveWordImport)
            .await?;
        decode_sensitive_words(&content, format).map_err(map_codec_error)
    }

    async fn write_sensitive(
        &self,
        grant_id: &str,
        format: SensitiveFileFormat,
        rules: &[SensitiveRule],
    ) -> Result<ExportReceipt, LiveRuntimeError> {
        let content = encode_sensitive_words(rules, format).map_err(map_codec_error)?;
        self.write(
            grant_id,
            PathPurpose::SensitiveWordExport,
            &content,
            rules.len() as u32,
        )
        .await
    }

    async fn read_blacklist(
        &self,
        grant_id: &str,
    ) -> Result<Vec<BlacklistImportEntry>, LiveRuntimeError> {
        let content = self.read(grant_id, PathPurpose::BlacklistImport).await?;
        decode_blacklist_csv(&content)
            .map(|rows| {
                rows.into_iter()
                    .map(|row| BlacklistImportEntry {
                        uid: row.uid,
                        username: row.username,
                        reason: row.reason,
                        created_at: row.created_at,
                    })
                    .collect()
            })
            .map_err(map_codec_error)
    }

    async fn write_blacklist(
        &self,
        grant_id: &str,
        rows: &[BlacklistEntry],
    ) -> Result<ExportReceipt, LiveRuntimeError> {
        let rows: Vec<_> = rows
            .iter()
            .map(|row| BlacklistFileRow {
                uid: row.uid.as_str().to_owned(),
                username: row.username.clone(),
                reason: row.reason.clone(),
                created_at: Some(row.created_at),
            })
            .collect();
        let content = encode_blacklist_csv(&rows).map_err(map_codec_error)?;
        self.write(
            grant_id,
            PathPurpose::BlacklistExport,
            &content,
            rows.len() as u32,
        )
        .await
    }

    async fn write_statistics(
        &self,
        grant_id: &str,
        metrics: &StatMetrics,
        generated_at: i64,
    ) -> Result<ExportReceipt, LiveRuntimeError> {
        let mut writer = csv::Writer::from_writer(Vec::new());
        writer
            .write_record([
                "occurred_at",
                "online_count",
                "danmu_count",
                "danmu_rate",
                "gift_value_minor",
                "captain_count",
                "filtered_count",
            ])
            .map_err(map_codec_error)?;
        writer
            .serialize((
                generated_at,
                metrics.online_count,
                metrics.danmu_count,
                metrics.danmu_rate,
                metrics.gift_value_minor,
                metrics.captain_count,
                metrics.filtered_count,
            ))
            .map_err(map_codec_error)?;
        let content = super::super::text::with_utf8_bom(
            writer
                .into_inner()
                .map_err(|error| map_io_error("STORAGE_WRITE_FAILED", error.into_error()))?,
        );
        self.write(grant_id, PathPurpose::StatExport, &content, 1)
            .await
    }

    async fn write_rankings(
        &self,
        grant_id: &str,
        snapshot: &RankingSnapshot,
        generated_at: i64,
    ) -> Result<ExportReceipt, LiveRuntimeError> {
        let mut writer = csv::Writer::from_writer(Vec::new());
        writer
            .write_record([
                "ranking_type",
                "rank",
                "uid",
                "username",
                "metric",
                "generated_at",
            ])
            .map_err(map_codec_error)?;
        for row in &snapshot.gift {
            writer
                .serialize((
                    "gift",
                    row.rank,
                    row.user_uid.as_str(),
                    &row.username,
                    row.amount_minor,
                    generated_at,
                ))
                .map_err(map_codec_error)?;
        }
        for row in &snapshot.danmu {
            writer
                .serialize((
                    "danmu",
                    row.rank,
                    row.user_uid.as_str(),
                    &row.username,
                    row.danmu_count,
                    generated_at,
                ))
                .map_err(map_codec_error)?;
        }
        for row in &snapshot.captain {
            writer
                .serialize((
                    "captain",
                    row.rank,
                    row.user_uid.as_str(),
                    &row.username,
                    row.amount_minor,
                    generated_at,
                ))
                .map_err(map_codec_error)?;
        }
        let count = snapshot.gift.len() + snapshot.danmu.len() + snapshot.captain.len();
        let content = super::super::text::with_utf8_bom(
            writer
                .into_inner()
                .map_err(|error| map_io_error("STORAGE_WRITE_FAILED", error.into_error()))?,
        );
        self.write(grant_id, PathPurpose::RankExport, &content, count as u32)
            .await
    }
}

fn map_path_error(error: impl std::fmt::Display) -> LiveRuntimeError {
    LiveRuntimeError::new("STORAGE_PATH_DENIED", error.to_string(), false)
}

fn map_codec_error(error: impl std::fmt::Display) -> LiveRuntimeError {
    LiveRuntimeError::new("RULE_INVALID_IMPORT", error.to_string(), false)
}

fn map_io_error(code: &'static str, error: std::io::Error) -> LiveRuntimeError {
    let code = match error.kind() {
        std::io::ErrorKind::PermissionDenied => "STORAGE_PERMISSION_DENIED",
        std::io::ErrorKind::WriteZero => "disk_full",
        _ => code,
    };
    LiveRuntimeError::new(code, error.to_string(), true)
}
