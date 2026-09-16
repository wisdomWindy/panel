use std::{
    path::{Path, PathBuf},
    time::{Duration, SystemTime, UNIX_EPOCH},
};

use sqlx::{
    migrate::MigrateError,
    sqlite::{SqliteConnectOptions, SqliteJournalMode, SqlitePoolOptions},
    SqlitePool,
};
use tokio::sync::OnceCell;

use crate::{
    application::ports::{DatabasePort, StoragePortError, StoredConfig},
    infrastructure::config_repository::ConfigRepository,
};

// Keep this call site touched when adding migrations so incremental builds re-embed the directory.
static MIGRATOR: sqlx::migrate::Migrator = sqlx::migrate!("./migrations");
pub const SCHEMA_VERSION: u32 = 3;

#[derive(Debug, thiserror::Error)]
pub enum DatabaseError {
    #[error("database path has no file name")]
    InvalidPath,
    #[error("failed to prepare the database directory: {0}")]
    Directory(#[from] std::io::Error),
    #[error("database operation failed: {0}")]
    Query(#[from] sqlx::Error),
    #[error("database migration failed: {0}")]
    Migration(#[from] MigrateError),
}

pub async fn open_database(path: &Path) -> Result<SqlitePool, DatabaseError> {
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)?;
    }

    let options = SqliteConnectOptions::new()
        .filename(path)
        .create_if_missing(true)
        .foreign_keys(true)
        .journal_mode(SqliteJournalMode::Wal)
        .busy_timeout(Duration::from_secs(5));
    let existed = path
        .metadata()
        .map(|metadata| metadata.len() > 0)
        .unwrap_or(false);
    let mut pool = connect(options.clone()).await?;

    if existed && !has_migration_history(&pool).await? {
        pool.close().await;
        tokio::fs::copy(path, migration_backup_path(path)?).await?;
        pool = connect(options).await?;
    }

    MIGRATOR.run(&pool).await?;
    prune_migration_backups(path, 3).await?;
    Ok(pool)
}

pub struct SqliteDatabase {
    path: PathBuf,
    pool: OnceCell<SqlitePool>,
}

impl SqliteDatabase {
    pub fn new(path: PathBuf) -> Self {
        Self {
            path,
            pool: OnceCell::new(),
        }
    }

    pub fn pool(&self) -> Option<SqlitePool> {
        self.pool.get().cloned()
    }
}

#[async_trait::async_trait]
impl DatabasePort for SqliteDatabase {
    async fn initialize(&self) -> Result<(), StoragePortError> {
        self.pool
            .get_or_try_init(|| async { open_database(&self.path).await })
            .await
            .map(|_| ())
            .map_err(|error| StoragePortError::unavailable(error.to_string()))
    }

    fn schema_version(&self) -> u32 {
        if self.pool.get().is_some() {
            SCHEMA_VERSION
        } else {
            0
        }
    }

    async fn get_config(
        &self,
        key: &str,
        scope: &str,
    ) -> Result<Option<StoredConfig>, StoragePortError> {
        let pool = self
            .pool
            .get()
            .ok_or_else(|| StoragePortError::unavailable("database is not initialized"))?;
        ConfigRepository::new(pool.clone())
            .get(key, scope)
            .await
            .map(|stored| {
                stored.map(|stored| StoredConfig {
                    value: stored.value,
                    updated_at: stored.updated_at,
                })
            })
            .map_err(|error| StoragePortError::unavailable(error.to_string()))
    }

    async fn set_config(
        &self,
        key: &str,
        scope: &str,
        value: &str,
    ) -> Result<String, StoragePortError> {
        let pool = self
            .pool
            .get()
            .ok_or_else(|| StoragePortError::unavailable("database is not initialized"))?;
        let repository = ConfigRepository::new(pool.clone());
        repository
            .set(key, value, scope)
            .await
            .map_err(|error| StoragePortError::unavailable(error.to_string()))?;
        repository
            .get(key, scope)
            .await
            .map_err(|error| StoragePortError::unavailable(error.to_string()))?
            .map(|stored| stored.updated_at)
            .ok_or_else(|| StoragePortError::unavailable("configuration write was not persisted"))
    }
}

async fn connect(options: SqliteConnectOptions) -> Result<SqlitePool, sqlx::Error> {
    SqlitePoolOptions::new()
        .max_connections(5)
        .connect_with(options)
        .await
}

async fn has_migration_history(pool: &SqlitePool) -> Result<bool, sqlx::Error> {
    let exists: i64 = sqlx::query_scalar(
    "SELECT EXISTS(SELECT 1 FROM sqlite_master WHERE type = 'table' AND name = '_sqlx_migrations')",
  )
  .fetch_one(pool)
  .await?;
    Ok(exists == 1)
}

fn migration_backup_path(path: &Path) -> Result<std::path::PathBuf, DatabaseError> {
    let mut file_name = path
        .file_name()
        .ok_or(DatabaseError::InvalidPath)?
        .to_os_string();
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_err(|_| DatabaseError::InvalidPath)?
        .as_millis();
    file_name.push(format!(".pre-migration.{timestamp:020}.bak"));
    Ok(path.with_file_name(file_name))
}

async fn prune_migration_backups(path: &Path, keep: usize) -> Result<(), DatabaseError> {
    let parent = path.parent().ok_or(DatabaseError::InvalidPath)?;
    let database_name = path
        .file_name()
        .and_then(|value| value.to_str())
        .ok_or(DatabaseError::InvalidPath)?;
    let prefix = format!("{database_name}.pre-migration.");
    let mut entries = tokio::fs::read_dir(parent).await?;
    let mut backups = Vec::new();
    while let Some(entry) = entries.next_entry().await? {
        let file_type = entry.file_type().await?;
        let name = entry.file_name();
        let Some(name) = name.to_str() else {
            continue;
        };
        if file_type.is_file() && name.starts_with(&prefix) && name.ends_with(".bak") {
            backups.push(entry.path());
        }
    }
    backups.sort();
    let remove_count = backups.len().saturating_sub(keep);
    for backup in backups.into_iter().take(remove_count) {
        tokio::fs::remove_file(backup).await?;
    }
    Ok(())
}

#[cfg(test)]
async fn run_transactional_batch(
    pool: &SqlitePool,
    statements: &[&str],
) -> Result<(), DatabaseError> {
    let mut transaction = pool.begin().await?;

    for statement in statements {
        if let Err(error) = sqlx::query(statement).execute(&mut *transaction).await {
            transaction.rollback().await?;
            return Err(DatabaseError::Query(error));
        }
    }

    transaction.commit().await?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use sqlx::{
        sqlite::SqliteConnectOptions, ConnectOptions, Connection, Executor, Row, SqliteConnection,
    };

    use super::{open_database, run_transactional_batch};

    #[tokio::test]
    async fn creates_the_prd_schema_and_enables_wal() {
        let directory = tempfile::tempdir().unwrap();
        let pool = open_database(&directory.path().join("app.db"))
            .await
            .unwrap();

        let journal_mode: String = sqlx::query_scalar("PRAGMA journal_mode")
            .fetch_one(&pool)
            .await
            .unwrap();
        let tables: Vec<String> = sqlx::query_scalar(
            "SELECT name FROM sqlite_master WHERE type = 'table' AND name LIKE 't_%' ORDER BY name",
        )
        .fetch_all(&pool)
        .await
        .unwrap();
        let indexes: Vec<String> = sqlx::query_scalar(
      "SELECT name FROM sqlite_master WHERE type = 'index' AND name LIKE 'idx_%' ORDER BY name",
    )
    .fetch_all(&pool)
    .await
    .unwrap();

        assert_eq!(journal_mode, "wal");
        assert_eq!(
            tables,
            [
                "t_account",
                "t_ai_config",
                "t_ai_log",
                "t_app_setting",
                "t_asr_config",
                "t_black_user",
                "t_event_log",
                "t_gift_sound",
                "t_highlight",
                "t_live_stat",
                "t_membership",
                "t_moderate_config",
                "t_moderation_alert",
                "t_moderation_log",
                "t_module",
                "t_order",
                "t_plugin",
                "t_ranking_archive",
                "t_recording",
                "t_room_config",
                "t_schema_state",
                "t_sensitive_word",
                "t_tts_config"
            ]
        );
        assert_eq!(
            indexes,
            [
                "idx_ai_log_room_time",
                "idx_black_user_uid",
                "idx_event_room_time",
                "idx_gift_sound_gift_id",
                "idx_highlight_recording_id",
                "idx_live_stat_room_date",
                "idx_moderation_alert_status_time",
                "idx_moderation_log_user_time",
                "idx_order_user_time",
                "idx_ranking_archive_room_date",
                "idx_recording_start_time",
                "idx_sensitive_word_type"
            ]
        );

        let account_columns: Vec<String> =
            sqlx::query_scalar("SELECT name FROM pragma_table_info('t_account') ORDER BY cid")
                .fetch_all(&pool)
                .await
                .unwrap();
        let ai_columns: Vec<String> =
            sqlx::query_scalar("SELECT name FROM pragma_table_info('t_ai_config') ORDER BY cid")
                .fetch_all(&pool)
                .await
                .unwrap();
        let order_columns: Vec<String> =
            sqlx::query_scalar("SELECT name FROM pragma_table_info('t_order') ORDER BY cid")
                .fetch_all(&pool)
                .await
                .unwrap();

        assert!(!account_columns.iter().any(|name| name.contains("token")));
        assert!(!ai_columns.iter().any(|name| name == "api_key"));
        assert!(ai_columns.iter().any(|name| name == "api_key_ciphertext"));
        assert!(order_columns.iter().any(|name| name == "amount_minor"));
        assert!(!order_columns.iter().any(|name| name == "amount"));
    }

    #[tokio::test]
    async fn reopening_the_database_is_idempotent() {
        let directory = tempfile::tempdir().unwrap();
        let path = directory.path().join("app.db");

        open_database(&path).await.unwrap().close().await;
        let pool = open_database(&path).await.unwrap();
        let state_count: i64 = sqlx::query_scalar(
            "SELECT COUNT(*) FROM t_schema_state WHERE state_key = 'schema_version_3'",
        )
        .fetch_one(&pool)
        .await
        .unwrap();

        assert_eq!(state_count, 1);
    }

    #[tokio::test]
    async fn backs_up_an_existing_unmigrated_database_before_schema_changes() {
        let directory = tempfile::tempdir().unwrap();
        let path = directory.path().join("app.db");
        let mut legacy = SqliteConnection::connect_with(
            &SqliteConnectOptions::new()
                .filename(&path)
                .create_if_missing(true),
        )
        .await
        .unwrap();
        legacy
            .execute("CREATE TABLE legacy_marker (value TEXT NOT NULL)")
            .await
            .unwrap();
        legacy
            .execute("INSERT INTO legacy_marker (value) VALUES ('preserved')")
            .await
            .unwrap();
        legacy.close().await.unwrap();

        open_database(&path).await.unwrap().close().await;

        let backup_path = std::fs::read_dir(directory.path())
            .unwrap()
            .map(|entry| entry.unwrap().path())
            .find(|path| {
                path.file_name()
                    .and_then(|value| value.to_str())
                    .is_some_and(|name| {
                        name.starts_with("app.db.pre-migration.") && name.ends_with(".bak")
                    })
            })
            .unwrap();
        let mut backup = SqliteConnectOptions::new()
            .filename(backup_path)
            .connect()
            .await
            .unwrap();
        let backup_value: String = sqlx::query_scalar("SELECT value FROM legacy_marker")
            .fetch_one(&mut backup)
            .await
            .unwrap();
        assert_eq!(backup_value, "preserved");
    }

    #[tokio::test]
    async fn keeps_only_the_three_newest_migration_backups() {
        let directory = tempfile::tempdir().unwrap();
        let database_path = directory.path().join("app.db");
        for index in 0..5 {
            tokio::fs::write(
                directory
                    .path()
                    .join(format!("app.db.pre-migration.{index:020}.bak")),
                index.to_string(),
            )
            .await
            .unwrap();
        }
        tokio::fs::write(directory.path().join("unrelated.bak"), "keep")
            .await
            .unwrap();

        super::prune_migration_backups(&database_path, 3)
            .await
            .unwrap();
        let mut names = std::fs::read_dir(directory.path())
            .unwrap()
            .map(|entry| entry.unwrap().file_name().to_string_lossy().into_owned())
            .filter(|name| name.starts_with("app.db.pre-migration."))
            .collect::<Vec<_>>();
        names.sort();

        assert_eq!(names.len(), 3);
        assert!(names[0].contains("00000000000000000002"));
        assert!(directory.path().join("unrelated.bak").exists());
    }

    #[tokio::test]
    async fn a_failed_sql_batch_rolls_back_prior_writes() {
        let directory = tempfile::tempdir().unwrap();
        let pool = open_database(&directory.path().join("app.db"))
            .await
            .unwrap();

        let result = run_transactional_batch(
            &pool,
            &[
                "INSERT INTO t_app_setting (key, value) VALUES ('before-failure', 'value')",
                "INSERT INTO table_that_does_not_exist (value) VALUES ('fail')",
            ],
        )
        .await;
        let row = sqlx::query("SELECT COUNT(*) AS count FROM t_app_setting")
            .fetch_one(&pool)
            .await
            .unwrap();

        assert!(result.is_err());
        assert_eq!(row.get::<i64, _>("count"), 0);
    }
}
