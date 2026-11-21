use sqlx::{Row, SqlitePool};

use crate::application::live_data::BlacklistImportEntry;
use crate::domain::{
    live_data::{
        normalize_optional_reason, BlacklistEntry, LiveEvent, SensitiveMatchType, SensitiveRule,
        SensitiveWordCategory, UserUid,
    },
    validation::Username,
};

#[derive(Debug, thiserror::Error)]
pub enum RepositoryError {
    #[error("live data is invalid")]
    InvalidData,
    #[error("live data repository failed: {0}")]
    Query(#[from] sqlx::Error),
}

pub type StoredBlacklistEntry = BlacklistEntry;

#[derive(Clone)]
pub struct LiveDataRepository {
    pool: SqlitePool,
}

impl LiveDataRepository {
    pub const fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }

    pub async fn list_sensitive(&self) -> Result<Vec<SensitiveRule>, RepositoryError> {
        sqlx::query(
            "SELECT id, word, word_type, match_type, enabled FROM t_sensitive_word ORDER BY id",
        )
        .fetch_all(&self.pool)
        .await?
        .into_iter()
        .map(map_sensitive_row)
        .collect()
    }

    pub async fn insert_sensitive(
        &self,
        rule: &SensitiveRule,
    ) -> Result<SensitiveRule, RepositoryError> {
        let result = sqlx::query(
            "INSERT INTO t_sensitive_word (word, word_type, match_type, enabled) VALUES (?, ?, ?, ?)",
        )
        .bind(&rule.word)
        .bind(category_name(rule.category))
        .bind(match_type_name(rule.match_type))
        .bind(rule.is_enabled)
        .execute(&self.pool)
        .await?;
        Ok(SensitiveRule {
            id: result.last_insert_rowid(),
            ..rule.clone()
        })
    }

    pub async fn update_sensitive(
        &self,
        rule: &SensitiveRule,
    ) -> Result<SensitiveRule, RepositoryError> {
        let result = sqlx::query(
            "UPDATE t_sensitive_word SET word = ?, word_type = ?, match_type = ?, enabled = ? WHERE id = ?",
        )
        .bind(&rule.word)
        .bind(category_name(rule.category))
        .bind(match_type_name(rule.match_type))
        .bind(rule.is_enabled)
        .bind(rule.id)
        .execute(&self.pool)
        .await?;
        if result.rows_affected() != 1 {
            return Err(RepositoryError::InvalidData);
        }
        Ok(rule.clone())
    }

    pub async fn delete_sensitive(&self, id: i64) -> Result<bool, RepositoryError> {
        Ok(sqlx::query("DELETE FROM t_sensitive_word WHERE id = ?")
            .bind(id)
            .execute(&self.pool)
            .await?
            .rows_affected()
            == 1)
    }

    pub async fn replace_sensitive(
        &self,
        rules: &[SensitiveRule],
    ) -> Result<Vec<SensitiveRule>, RepositoryError> {
        let mut transaction = self.pool.begin().await?;
        sqlx::query("DELETE FROM t_sensitive_word")
            .execute(&mut *transaction)
            .await?;
        let mut stored = Vec::with_capacity(rules.len());
        for rule in rules {
            let result = sqlx::query(
                "INSERT INTO t_sensitive_word (word, word_type, match_type, enabled) VALUES (?, ?, ?, ?)",
            )
            .bind(&rule.word)
            .bind(category_name(rule.category))
            .bind(match_type_name(rule.match_type))
            .bind(rule.is_enabled)
            .execute(&mut *transaction)
            .await;
            let result = match result {
                Ok(result) => result,
                Err(error) => {
                    transaction.rollback().await?;
                    return Err(RepositoryError::Query(error));
                }
            };
            stored.push(SensitiveRule {
                id: result.last_insert_rowid(),
                ..rule.clone()
            });
        }
        transaction.commit().await?;
        Ok(stored)
    }

    pub async fn list_blacklist(&self) -> Result<Vec<StoredBlacklistEntry>, RepositoryError> {
        sqlx::query(
            "SELECT user_uid, username, reason, CAST(strftime('%s', create_time) AS INTEGER) * 1000 AS created_at FROM t_black_user ORDER BY id",
        )
        .fetch_all(&self.pool)
        .await?
        .into_iter()
        .map(map_blacklist_row)
        .collect()
    }

    pub async fn upsert_blacklist(
        &self,
        uid: &str,
        username: &str,
        reason: Option<&str>,
    ) -> Result<StoredBlacklistEntry, RepositoryError> {
        let uid = UserUid::parse(uid).map_err(|_| RepositoryError::InvalidData)?;
        let username = Username::parse(username).map_err(|_| RepositoryError::InvalidData)?;
        let reason = normalize_optional_reason(reason);
        sqlx::query(
            "INSERT INTO t_black_user (user_uid, username, reason) VALUES (?, ?, ?) ON CONFLICT(user_uid) DO UPDATE SET username = excluded.username, reason = excluded.reason",
        )
        .bind(uid.as_str())
        .bind(username.as_str())
        .bind(reason)
        .execute(&self.pool)
        .await?;
        let row = sqlx::query(
            "SELECT user_uid, username, reason, CAST(strftime('%s', create_time) AS INTEGER) * 1000 AS created_at FROM t_black_user WHERE user_uid = ?",
        )
        .bind(uid.as_str())
        .fetch_one(&self.pool)
        .await?;
        map_blacklist_row(row)
    }

    pub async fn remove_blacklist(&self, uid: &str) -> Result<bool, RepositoryError> {
        let uid = UserUid::parse(uid).map_err(|_| RepositoryError::InvalidData)?;
        Ok(sqlx::query("DELETE FROM t_black_user WHERE user_uid = ?")
            .bind(uid.as_str())
            .execute(&self.pool)
            .await?
            .rows_affected()
            == 1)
    }

    pub async fn replace_blacklist(
        &self,
        rows: &[BlacklistImportEntry],
    ) -> Result<Vec<BlacklistEntry>, RepositoryError> {
        let mut normalized = std::collections::BTreeMap::new();
        for row in rows {
            let uid = UserUid::parse(&row.uid).map_err(|_| RepositoryError::InvalidData)?;
            let username = Username::parse(&row.username)
                .map_err(|_| RepositoryError::InvalidData)?
                .as_str()
                .to_owned();
            normalized.insert(
                uid.as_str().to_owned(),
                (
                    uid,
                    username,
                    normalize_optional_reason(row.reason.as_deref()),
                    row.created_at,
                ),
            );
        }
        let mut transaction = self.pool.begin().await?;
        sqlx::query("DELETE FROM t_black_user")
            .execute(&mut *transaction)
            .await?;
        for (uid, username, reason, created_at) in normalized.values() {
            if let Some(created_at) = created_at {
                sqlx::query(
                    "INSERT INTO t_black_user (user_uid, username, reason, create_time) VALUES (?, ?, ?, datetime(?, 'unixepoch'))",
                )
                .bind(uid.as_str())
                .bind(username)
                .bind(reason)
                .bind(created_at / 1_000)
                .execute(&mut *transaction)
                .await?;
            } else {
                sqlx::query(
                    "INSERT INTO t_black_user (user_uid, username, reason) VALUES (?, ?, ?)",
                )
                .bind(uid.as_str())
                .bind(username)
                .bind(reason)
                .execute(&mut *transaction)
                .await?;
            }
        }
        transaction.commit().await?;
        self.list_blacklist().await
    }

    pub async fn record_event(
        &self,
        event: &LiveEvent,
        filtered_reason: Option<&'static str>,
    ) -> Result<(), RepositoryError> {
        let event_type = match event {
            LiveEvent::Danmu { .. } => "danmu",
            LiveEvent::Gift { .. } => "gift",
            LiveEvent::Captain { .. } => "captain",
            LiveEvent::Enter { .. } => "enter",
            LiveEvent::SessionStarted { .. } => "session_started",
            LiveEvent::SessionEnded { .. } => "session_ended",
            LiveEvent::SuperChat { .. } => "super_chat",
            LiveEvent::SuperChatDeleted { .. } => "super_chat_deleted",
        };
        let payload = serde_json::json!({
            "eventId": event.base().event_id,
            "occurredAt": event.base().occurred_at,
            "filterReason": filtered_reason,
        });
        sqlx::query(
            "INSERT INTO t_event_log (room_id, event_type, user_uid, payload_json, filtered, create_time) VALUES (?, ?, ?, ?, ?, datetime(?, 'unixepoch'))",
        )
        .bind(event.base().room_id.as_str())
        .bind(event_type)
        .bind(event.base().audience.user_uid.as_str())
        .bind(payload.to_string())
        .bind(filtered_reason.is_some())
        .bind(event.base().occurred_at / 1_000)
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    pub async fn today_sensitive_hits(&self) -> Result<u64, RepositoryError> {
        let count: i64 = sqlx::query_scalar(
            "SELECT COUNT(*) FROM t_event_log WHERE filtered = 1 AND json_extract(payload_json, '$.filterReason') = 'sensitive' AND date(create_time, 'localtime') = date('now', 'localtime')",
        )
        .fetch_one(&self.pool)
        .await?;
        u64::try_from(count).map_err(|_| RepositoryError::InvalidData)
    }

    pub async fn today_blacklist_additions(&self) -> Result<u64, RepositoryError> {
        let count: i64 = sqlx::query_scalar(
            "SELECT COUNT(*) FROM t_black_user WHERE date(create_time, 'localtime') = date('now', 'localtime')",
        )
        .fetch_one(&self.pool)
        .await?;
        u64::try_from(count).map_err(|_| RepositoryError::InvalidData)
    }
}

fn map_sensitive_row(row: sqlx::sqlite::SqliteRow) -> Result<SensitiveRule, RepositoryError> {
    SensitiveRule::new(
        row.try_get("id")?,
        row.try_get("word")?,
        parse_category(row.try_get("word_type")?)?,
        parse_match_type(row.try_get("match_type")?)?,
        row.try_get("enabled")?,
    )
    .map_err(|_| RepositoryError::InvalidData)
}

fn map_blacklist_row(row: sqlx::sqlite::SqliteRow) -> Result<BlacklistEntry, RepositoryError> {
    let uid: String = row.try_get("user_uid")?;
    Ok(BlacklistEntry {
        uid: UserUid::parse(&uid).map_err(|_| RepositoryError::InvalidData)?,
        username: row.try_get("username")?,
        reason: row.try_get("reason")?,
        created_at: row.try_get("created_at")?,
    })
}

impl From<RepositoryError> for crate::application::live_data::LiveRuntimeError {
    fn from(error: RepositoryError) -> Self {
        crate::application::live_data::LiveRuntimeError::storage(error.to_string())
    }
}

#[async_trait::async_trait]
impl crate::application::live_data::LiveRepository for LiveDataRepository {
    async fn load_sensitive(
        &self,
    ) -> Result<Vec<SensitiveRule>, crate::application::live_data::LiveRuntimeError> {
        self.list_sensitive().await.map_err(Into::into)
    }

    async fn record_event(
        &self,
        event: &LiveEvent,
        filtered_reason: Option<&'static str>,
    ) -> Result<(), crate::application::live_data::LiveRuntimeError> {
        LiveDataRepository::record_event(self, event, filtered_reason)
            .await
            .map_err(Into::into)
    }

    async fn add_sensitive(
        &self,
        rule: &SensitiveRule,
    ) -> Result<SensitiveRule, crate::application::live_data::LiveRuntimeError> {
        self.insert_sensitive(rule).await.map_err(Into::into)
    }

    async fn update_sensitive(
        &self,
        rule: &SensitiveRule,
    ) -> Result<SensitiveRule, crate::application::live_data::LiveRuntimeError> {
        LiveDataRepository::update_sensitive(self, rule)
            .await
            .map_err(Into::into)
    }

    async fn delete_sensitive(
        &self,
        id: i64,
    ) -> Result<bool, crate::application::live_data::LiveRuntimeError> {
        LiveDataRepository::delete_sensitive(self, id)
            .await
            .map_err(Into::into)
    }

    async fn replace_sensitive(
        &self,
        rules: &[SensitiveRule],
    ) -> Result<Vec<SensitiveRule>, crate::application::live_data::LiveRuntimeError> {
        LiveDataRepository::replace_sensitive(self, rules)
            .await
            .map_err(Into::into)
    }

    async fn list_blacklist_entries(
        &self,
    ) -> Result<Vec<BlacklistEntry>, crate::application::live_data::LiveRuntimeError> {
        self.list_blacklist().await.map_err(Into::into)
    }

    async fn today_sensitive_hits(
        &self,
    ) -> Result<u64, crate::application::live_data::LiveRuntimeError> {
        LiveDataRepository::today_sensitive_hits(self)
            .await
            .map_err(Into::into)
    }

    async fn today_blacklist_additions(
        &self,
    ) -> Result<u64, crate::application::live_data::LiveRuntimeError> {
        LiveDataRepository::today_blacklist_additions(self)
            .await
            .map_err(Into::into)
    }

    async fn upsert_blacklist(
        &self,
        uid: &str,
        username: &str,
        reason: Option<&str>,
    ) -> Result<BlacklistEntry, crate::application::live_data::LiveRuntimeError> {
        LiveDataRepository::upsert_blacklist(self, uid, username, reason)
            .await
            .map_err(Into::into)
    }

    async fn remove_blacklist(
        &self,
        uid: &str,
    ) -> Result<bool, crate::application::live_data::LiveRuntimeError> {
        LiveDataRepository::remove_blacklist(self, uid)
            .await
            .map_err(Into::into)
    }

    async fn replace_blacklist(
        &self,
        rows: &[BlacklistImportEntry],
    ) -> Result<Vec<BlacklistEntry>, crate::application::live_data::LiveRuntimeError> {
        LiveDataRepository::replace_blacklist(self, rows)
            .await
            .map_err(Into::into)
    }
}

pub struct UnavailableLiveRepository;

#[async_trait::async_trait]
impl crate::application::live_data::LiveRepository for UnavailableLiveRepository {
    async fn load_sensitive(
        &self,
    ) -> Result<Vec<SensitiveRule>, crate::application::live_data::LiveRuntimeError> {
        Err(crate::application::live_data::LiveRuntimeError::storage(
            "database is unavailable",
        ))
    }

    async fn record_event(
        &self,
        _: &LiveEvent,
        _: Option<&'static str>,
    ) -> Result<(), crate::application::live_data::LiveRuntimeError> {
        Err(crate::application::live_data::LiveRuntimeError::storage(
            "database is unavailable",
        ))
    }
}

const fn category_name(category: SensitiveWordCategory) -> &'static str {
    match category {
        SensitiveWordCategory::Illegal => "illegal",
        SensitiveWordCategory::Advertising => "advertising",
        SensitiveWordCategory::PersonalAttack => "personal_attack",
        SensitiveWordCategory::Custom => "custom",
    }
}

fn parse_category(value: &str) -> Result<SensitiveWordCategory, RepositoryError> {
    match value {
        "illegal" => Ok(SensitiveWordCategory::Illegal),
        "advertising" => Ok(SensitiveWordCategory::Advertising),
        "personal_attack" => Ok(SensitiveWordCategory::PersonalAttack),
        "custom" => Ok(SensitiveWordCategory::Custom),
        _ => Err(RepositoryError::InvalidData),
    }
}

const fn match_type_name(match_type: SensitiveMatchType) -> &'static str {
    match match_type {
        SensitiveMatchType::Exact => "exact",
        SensitiveMatchType::Fuzzy => "fuzzy",
    }
}

fn parse_match_type(value: &str) -> Result<SensitiveMatchType, RepositoryError> {
    match value {
        "exact" => Ok(SensitiveMatchType::Exact),
        "fuzzy" => Ok(SensitiveMatchType::Fuzzy),
        _ => Err(RepositoryError::InvalidData),
    }
}
