//! Data retention and clearing (PRD 12.3 / 12.4).
//!
//! Retention defaults come from the PRD table: event logs 90 days, AI logs 30
//! days, statistics 180 days. `None` means "keep forever". Clearing is explicit
//! and scoped so the UI can offer the four entries the PRD lists.

use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};
use sqlx::{Row, SqlitePool};

const RETENTION_KEY: &str = "data.retention";
const RETENTION_SCOPE: &str = "global";

/// PRD 12.3 fixes the runtime-log window at 30 days; it is not user-tunable.
pub const RUNTIME_LOG_RETENTION_DAYS: u64 = 30;

#[derive(Debug, thiserror::Error)]
pub enum DataMaintenanceError {
    #[error("database operation failed: {0}")]
    Database(#[from] sqlx::Error),
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum DataClearScope {
    EventLogs,
    AiLogs,
    Cache,
    All,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RetentionPolicy {
    pub event_log_days: Option<u32>,
    pub ai_log_days: Option<u32>,
    pub stat_days: Option<u32>,
}impl Default for RetentionPolicy {
    fn default() -> Self {
        Self {
            event_log_days: Some(90),
            ai_log_days: Some(30),
            stat_days: Some(180),
        }
    }
}

impl RetentionPolicy {
    /// The PRD allows a fixed set of choices per data type.
    pub fn is_valid(&self) -> bool {
        const EVENT: [u32; 5] = [7, 30, 90, 180, 0];
        const AI: [u32; 4] = [7, 30, 90, 0];
        const STAT: [u32; 4] = [30, 90, 180, 0];
        fn allowed(value: Option<u32>, choices: &[u32]) -> bool {
            value.map_or(true, |days| choices.contains(&days))
        }
        // 0 is the wire representation of "permanent"; normalise it to None.
        allowed(self.event_log_days, &EVENT)
            && allowed(self.ai_log_days, &AI)
            && allowed(self.stat_days, &STAT)
    }

    fn normalised(self) -> Self {
        fn clean(value: Option<u32>) -> Option<u32> {
            value.filter(|days| *days > 0)
        }
        Self {
            event_log_days: clean(self.event_log_days),
            ai_log_days: clean(self.ai_log_days),
            stat_days: clean(self.stat_days),
        }
    }
}

#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ClearReport {
    pub cleared: Vec<String>,
}

#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PurgeReport {
    pub event_logs: u64,
    pub ai_logs: u64,
    pub stats: u64,
}

#[derive(Clone)]
pub struct DataMaintenance {
    pool: SqlitePool,
    cache_directory: Option<PathBuf>,
}

impl DataMaintenance {
    pub fn new(pool: SqlitePool, cache_directory: Option<PathBuf>) -> Self {
        Self {
            pool,
            cache_directory,
        }
    }

    pub async fn load_policy(&self) -> Result<RetentionPolicy, DataMaintenanceError> {
        let row = sqlx::query("SELECT value FROM t_app_setting WHERE key = ? AND scope = ?")
            .bind(RETENTION_KEY)
            .bind(RETENTION_SCOPE)
            .fetch_optional(&self.pool)
            .await?;
        let Some(row) = row else {
            return Ok(RetentionPolicy::default());
        };
        let raw: String = row.get("value");
        Ok(serde_json::from_str::<RetentionPolicy>(&raw)
            .unwrap_or_default()
            .normalised())
    }

    pub async fn save_policy(&self, policy: RetentionPolicy) -> Result<RetentionPolicy, DataMaintenanceError> {
        let policy = policy.normalised();
        let value = serde_json::to_string(&policy).unwrap_or_else(|_| "{}".into());
        sqlx::query(
            "INSERT INTO t_app_setting (key, value, scope) VALUES (?, ?, ?) \
             ON CONFLICT(key) DO UPDATE SET value = excluded.value, \
             scope = excluded.scope, update_time = CURRENT_TIMESTAMP",
        )
        .bind(RETENTION_KEY)
        .bind(value)
        .bind(RETENTION_SCOPE)
        .execute(&self.pool)
        .await?;
        Ok(policy)
    }

    /// Deletes rows older than the configured retention. A `None` policy keeps
    /// that data type forever.
    pub async fn purge_expired(&self, policy: RetentionPolicy) -> Result<PurgeReport, DataMaintenanceError> {
        let policy = policy.normalised();
        let event_logs = purge_table(&self.pool, "t_event_log", policy.event_log_days).await?;
        let ai_logs = purge_table(&self.pool, "t_ai_log", policy.ai_log_days).await?;
        let stats = purge_table(&self.pool, "t_live_stat", policy.stat_days).await?;
        Ok(PurgeReport {
            event_logs,
            ai_logs,
            stats,
        })
    }

    /// Deletes runtime log files whose modification time is older than
    /// `days`. Returns how many files were removed.
    pub async fn purge_log_files(&self, directory: &Path, days: u64) -> usize {
        let Ok(mut entries) = tokio::fs::read_dir(directory).await else {
            return 0;
        };
        let cutoff = std::time::SystemTime::now()
            .checked_sub(std::time::Duration::from_secs(days.saturating_mul(86_400)));
        let Some(cutoff) = cutoff else {
            return 0;
        };
        let mut removed = 0;
        while let Ok(Some(entry)) = entries.next_entry().await {
            let path = entry.path();
            if !path.is_file() {
                continue;
            }
            let Ok(metadata) = entry.metadata().await else {
                continue;
            };
            let Ok(modified) = metadata.modified() else {
                continue;
            };
            if modified < cutoff && tokio::fs::remove_file(&path).await.is_ok() {
                removed += 1;
            }
        }
        removed
    }

    pub async fn clear(&self, scope: DataClearScope) -> Result<ClearReport, DataMaintenanceError> {        let mut cleared = Vec::new();
        match scope {
            DataClearScope::EventLogs => {
                clear_table(&self.pool, "t_event_log", &mut cleared).await?;
            }
            DataClearScope::AiLogs => {
                clear_table(&self.pool, "t_ai_log", &mut cleared).await?;
                clear_table(&self.pool, "t_moderation_log", &mut cleared).await?;
            }
            DataClearScope::Cache => {
                if let Some(directory) = &self.cache_directory {
                    if clear_directory(directory).await {
                        cleared.push("cache".to_owned());
                    }
                }
            }
            DataClearScope::All => {
                // Everything the user created, but not the schema bookkeeping.
                for table in [
                    "t_event_log",
                    "t_ai_log",
                    "t_moderation_log",
                    "t_moderation_alert",
                    "t_live_stat",
                    "t_ranking_archive",
                    "t_highlight",
                    "t_recording",
                    "t_sensitive_word",
                    "t_black_user",
                    "t_gift_sound",
                    "t_room_config",
                    "t_tts_config",
                    "t_asr_config",
                    "t_moderate_config",
                    "t_ai_config",
                    "t_ai_model_profile",
                    "t_ai_voice_config",
                    "t_account",
                    "t_membership",
                    "t_order",
                    "t_module",
                ] {
                    clear_table(&self.pool, table, &mut cleared).await?;
                }
                sqlx::query("DELETE FROM t_app_setting")
                    .execute(&self.pool)
                    .await?;
                cleared.push("t_app_setting".to_owned());
                if let Some(directory) = &self.cache_directory {
                    if clear_directory(directory).await {
                        cleared.push("cache".to_owned());
                    }
                }
            }
        }
        Ok(ClearReport { cleared })
    }
}

async fn purge_table(pool: &SqlitePool, table: &str, days: Option<u32>) -> Result<u64, sqlx::Error> {
    let Some(days) = days else {
        return Ok(0);
    };
    // Table names are compile-time constants, never user input.
    let statement = format!("DELETE FROM {table} WHERE create_time < datetime('now', ?1)");
    let modifier = format!("-{days} days");
    let result = sqlx::query(&statement).bind(modifier).execute(pool).await?;
    Ok(result.rows_affected())
}

async fn clear_table(
    pool: &SqlitePool,
    table: &str,
    cleared: &mut Vec<String>,
) -> Result<(), sqlx::Error> {
    let statement = format!("DELETE FROM {table}");
    sqlx::query(&statement).execute(pool).await?;
    cleared.push(table.to_owned());
    Ok(())
}

/// Removes the files inside `directory` without deleting the directory itself.
async fn clear_directory(directory: &Path) -> bool {
    let Ok(mut entries) = tokio::fs::read_dir(directory).await else {
        return false;
    };
    let mut removed = false;
    while let Ok(Some(entry)) = entries.next_entry().await {
        let path = entry.path();
        let outcome = if path.is_dir() {
            tokio::fs::remove_dir_all(&path).await
        } else {
            tokio::fs::remove_file(&path).await
        };
        removed |= outcome.is_ok();
    }
    removed
}

#[cfg(test)]
mod tests {
    use sqlx::Row;

    use super::{DataClearScope, DataMaintenance, RetentionPolicy};
    use crate::infrastructure::database::open_database;

    async fn service() -> (DataMaintenance, tempfile::TempDir) {
        let directory = tempfile::tempdir().unwrap();
        let pool = open_database(&directory.path().join("app.db"))
            .await
            .unwrap();
        (
            DataMaintenance::new(pool, Some(directory.path().join("cache"))),
            directory,
        )
    }

    #[tokio::test]
    async fn retention_defaults_match_the_prd_table() {
        let policy = RetentionPolicy::default();
        assert_eq!(policy.event_log_days, Some(90));
        assert_eq!(policy.ai_log_days, Some(30));
        assert_eq!(policy.stat_days, Some(180));
        assert!(policy.is_valid());
    }

    #[tokio::test]
    async fn rejects_retention_values_outside_the_prd_choices() {
        assert!(!RetentionPolicy {
            event_log_days: Some(45),
            ..RetentionPolicy::default()
        }
        .is_valid());
        assert!(RetentionPolicy {
            event_log_days: Some(180),
            ai_log_days: None,
            stat_days: Some(30),
        }
        .is_valid());
    }

    #[tokio::test]
    async fn saves_and_reloads_the_policy() {
        let (service, _directory) = service().await;
        let saved = service
            .save_policy(RetentionPolicy {
                event_log_days: Some(7),
                ai_log_days: None,
                stat_days: Some(90),
            })
            .await
            .unwrap();
        assert_eq!(saved.event_log_days, Some(7));
        assert_eq!(saved.ai_log_days, None);

        let loaded = service.load_policy().await.unwrap();
        assert_eq!(loaded, saved);
    }

    #[tokio::test]
    async fn purges_only_rows_older_than_the_policy() {
        let (service, _directory) = service().await;
        let pool = service.pool.clone();
        sqlx::query(
            "INSERT INTO t_event_log (room_id, event_type, create_time) \
             VALUES ('1', 'danmu', datetime('now', '-120 days'))",
        )
        .execute(&pool)
        .await
        .unwrap();
        sqlx::query(
            "INSERT INTO t_event_log (room_id, event_type, create_time) \
             VALUES ('1', 'danmu', datetime('now', '-10 days'))",
        )
        .execute(&pool)
        .await
        .unwrap();

        let report = service
            .purge_expired(RetentionPolicy::default())
            .await
            .unwrap();
        assert_eq!(report.event_logs, 1);

        let remaining: i64 = sqlx::query("SELECT COUNT(*) AS total FROM t_event_log")
            .fetch_one(&pool)
            .await
            .unwrap()
            .get("total");
        assert_eq!(remaining, 1);
    }

    #[tokio::test]
    async fn keeping_forever_purges_nothing() {
        let (service, _directory) = service().await;
        let pool = service.pool.clone();
        sqlx::query(
            "INSERT INTO t_event_log (room_id, event_type, create_time) \
             VALUES ('1', 'danmu', datetime('now', '-999 days'))",
        )
        .execute(&pool)
        .await
        .unwrap();

        let report = service
            .purge_expired(RetentionPolicy {
                event_log_days: None,
                ai_log_days: Some(30),
                stat_days: Some(180),
            })
            .await
            .unwrap();
        assert_eq!(report.event_logs, 0);
    }

    #[tokio::test]
    async fn clearing_event_logs_leaves_ai_logs_untouched() {
        let (service, _directory) = service().await;
        let pool = service.pool.clone();
        sqlx::query("INSERT INTO t_event_log (room_id, event_type) VALUES ('1', 'danmu')")
            .execute(&pool)
            .await
            .unwrap();
        sqlx::query(
            "INSERT INTO t_ai_log (operation_type, generation_status, room_id) \
             VALUES ('reply', 'success', '1')",
        )
        .execute(&pool)
        .await
        .unwrap();

        let report = service.clear(DataClearScope::EventLogs).await.unwrap();
        assert_eq!(report.cleared, vec!["t_event_log".to_owned()]);

        let ai: i64 = sqlx::query("SELECT COUNT(*) AS total FROM t_ai_log")
            .fetch_one(&pool)
            .await
            .unwrap()
            .get("total");
        assert_eq!(ai, 1);
    }

    #[tokio::test]
    async fn clearing_everything_keeps_the_schema_state() {
        let (service, _directory) = service().await;
        let pool = service.pool.clone();
        sqlx::query("INSERT INTO t_event_log (room_id, event_type) VALUES ('1', 'danmu')")
            .execute(&pool)
            .await
            .unwrap();

        let report = service.clear(DataClearScope::All).await.unwrap();
        assert!(report.cleared.contains(&"t_event_log".to_owned()));
        assert!(report.cleared.contains(&"t_app_setting".to_owned()));
        assert!(!report.cleared.contains(&"t_schema_state".to_owned()));

        let schema: i64 = sqlx::query("SELECT COUNT(*) AS total FROM t_schema_state")
            .fetch_one(&pool)
            .await
            .unwrap()
            .get("total");
        assert!(schema > 0);
    }

    #[tokio::test]
    async fn clears_cache_before_the_log_window_keeps_fresh_files() {
        let (service, directory) = service().await;
        let logs = directory.path().join("logs");
        tokio::fs::create_dir_all(&logs).await.unwrap();
        let fresh = logs.join("app.log");
        tokio::fs::write(&fresh, b"x").await.unwrap();
        let stale = logs.join("app.old.log");
        tokio::fs::write(&stale, b"x").await.unwrap();
        // Backdate the second file beyond the 30-day window.
        let old = std::time::SystemTime::now() - std::time::Duration::from_secs(40 * 86_400);
        std::fs::File::options()
            .write(true)
            .open(&stale)
            .unwrap()
            .set_modified(old)
            .unwrap();

        let removed = service
            .purge_log_files(&logs, super::RUNTIME_LOG_RETENTION_DAYS)
            .await;

        assert_eq!(removed, 1);
        assert!(fresh.exists());
        assert!(!stale.exists());
    }

    #[tokio::test]
    async fn clears_the_cache_directory_contents() {
        let (service, directory) = service().await;
        let cache = directory.path().join("cache");
        tokio::fs::create_dir_all(&cache).await.unwrap();
        tokio::fs::write(cache.join("image.png"), b"x").await.unwrap();

        let report = service.clear(DataClearScope::Cache).await.unwrap();
        assert_eq!(report.cleared, vec!["cache".to_owned()]);
        assert!(cache.exists());
        assert_eq!(std::fs::read_dir(&cache).unwrap().count(), 0);
    }
}
