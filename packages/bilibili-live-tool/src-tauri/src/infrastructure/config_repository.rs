use sqlx::{Row, SqlitePool};

use super::database::DatabaseError;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct StoredConfig {
    pub value: String,
    pub scope: String,
    pub updated_at: String,
}

#[derive(Clone)]
pub struct ConfigRepository {
    pool: SqlitePool,
}

impl ConfigRepository {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }

    pub async fn get(&self, key: &str, scope: &str) -> Result<Option<StoredConfig>, DatabaseError> {
        let row = sqlx::query(
            "SELECT value, scope, update_time FROM t_app_setting WHERE key = ? AND scope = ?",
        )
        .bind(key)
        .bind(scope)
        .fetch_optional(&self.pool)
        .await?;

        Ok(row.map(|row| StoredConfig {
            value: row.get("value"),
            scope: row.get("scope"),
            updated_at: row.get("update_time"),
        }))
    }

    pub async fn set(&self, key: &str, value: &str, scope: &str) -> Result<(), DatabaseError> {
        sqlx::query(
            "INSERT INTO t_app_setting (key, value, scope) VALUES (?, ?, ?) \
       ON CONFLICT(key) DO UPDATE SET value = excluded.value, \
       scope = excluded.scope, update_time = CURRENT_TIMESTAMP",
        )
        .bind(key)
        .bind(value)
        .bind(scope)
        .execute(&self.pool)
        .await?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::ConfigRepository;
    use crate::infrastructure::database::open_database;

    #[tokio::test]
    async fn stores_updates_and_reads_non_secret_configuration() {
        let directory = tempfile::tempdir().unwrap();
        let pool = open_database(&directory.path().join("app.db"))
            .await
            .unwrap();
        let repository = ConfigRepository::new(pool);

        assert!(repository
            .get("ui.theme", "global")
            .await
            .unwrap()
            .is_none());

        repository.set("ui.theme", "light", "global").await.unwrap();
        repository.set("ui.theme", "dark", "global").await.unwrap();
        let stored = repository.get("ui.theme", "global").await.unwrap().unwrap();

        assert_eq!(stored.value, "dark");
        assert_eq!(stored.scope, "global");
        assert!(!stored.updated_at.is_empty());
    }
}
