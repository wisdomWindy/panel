use sqlx::{Row, SqlitePool};

use crate::{
    application::member::{
        AiConfigRecord, MemberRepository, MemberServiceError, ModerationConfigRecord, OrderRecord,
        OrderStatus, StoredEntitlement,
    },
    domain::{
        live_data::UserUid,
        member::{
            Entitlement, EntitlementStatus, MemberAccount, MemberModule, MemberModuleStatus,
            ModuleId, OrderId,
        },
    },
};

#[derive(Clone)]
pub struct SqliteMemberRepository {
    pool: SqlitePool,
}

impl SqliteMemberRepository {
    pub const fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl MemberRepository for SqliteMemberRepository {
    async fn load_account(&self) -> Result<Option<MemberAccount>, MemberServiceError> {
        let row = sqlx::query(
            "SELECT user_uid, username, avatar_url, logged_in_at FROM t_account ORDER BY updated_at DESC LIMIT 1",
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(repository_error)?;
        row.map(map_account).transpose()
    }

    async fn save_account(&self, account: &MemberAccount) -> Result<(), MemberServiceError> {
        if account.username.trim().is_empty() || account.logged_in_at <= 0 {
            return Err(invalid_data());
        }
        let mut transaction = self.pool.begin().await.map_err(repository_error)?;
        sqlx::query("DELETE FROM t_account")
            .execute(&mut *transaction)
            .await
            .map_err(repository_error)?;
        sqlx::query(
            "INSERT INTO t_account (user_uid, username, avatar_url, logged_in_at, updated_at) VALUES (?, ?, ?, ?, ?)",
        )
        .bind(account.user_uid.as_str())
        .bind(&account.username)
        .bind(&account.avatar_url)
        .bind(account.logged_in_at)
        .bind(account.logged_in_at)
        .execute(&mut *transaction)
        .await
        .map_err(repository_error)?;
        transaction.commit().await.map_err(repository_error)?;
        Ok(())
    }

    async fn remove_account(&self) -> Result<bool, MemberServiceError> {
        let mut transaction = self.pool.begin().await.map_err(repository_error)?;
        let removed = sqlx::query("DELETE FROM t_account")
            .execute(&mut *transaction)
            .await
            .map_err(repository_error)?
            .rows_affected()
            > 0;
        sqlx::query("DELETE FROM t_membership")
            .execute(&mut *transaction)
            .await
            .map_err(repository_error)?;
        transaction.commit().await.map_err(repository_error)?;
        Ok(removed)
    }

    async fn load_entitlement(&self) -> Result<Option<StoredEntitlement>, MemberServiceError> {
        let row = sqlx::query(
            "SELECT status, plan_id, token_ciphertext, token_signature, issued_at, expires_at, last_verified_at FROM t_membership ORDER BY updated_at DESC LIMIT 1",
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(repository_error)?;
        row.map(map_entitlement).transpose()
    }

    async fn save_entitlement(
        &self,
        user_uid: &UserUid,
        stored: &StoredEntitlement,
        updated_at: i64,
    ) -> Result<(), MemberServiceError> {
        let entitlement = &stored.entitlement;
        if stored.token_ciphertext.is_empty()
            || stored.token_signature.is_empty()
            || entitlement.plan_id.trim().is_empty()
            || entitlement.issued_at <= 0
            || entitlement.last_verified_at < entitlement.issued_at
            || entitlement.expires_at <= entitlement.issued_at
            || updated_at < entitlement.issued_at
        {
            return Err(invalid_data());
        }
        let mut transaction = self.pool.begin().await.map_err(repository_error)?;
        sqlx::query("DELETE FROM t_membership")
            .execute(&mut *transaction)
            .await
            .map_err(repository_error)?;
        sqlx::query(
            "INSERT INTO t_membership (user_uid, status, plan_id, token_ciphertext, token_signature, issued_at, expires_at, last_verified_at, updated_at) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)",
        )
        .bind(user_uid.as_str())
        .bind(entitlement_status_name(entitlement.status))
        .bind(&entitlement.plan_id)
        .bind(&stored.token_ciphertext)
        .bind(&stored.token_signature)
        .bind(entitlement.issued_at)
        .bind(entitlement.expires_at)
        .bind(entitlement.last_verified_at)
        .bind(updated_at)
        .execute(&mut *transaction)
        .await
        .map_err(repository_error)?;
        transaction.commit().await.map_err(repository_error)?;
        Ok(())
    }

    async fn list_modules(&self) -> Result<Vec<MemberModule>, MemberServiceError> {
        sqlx::query(
            "SELECT module_id, name, version, status, downloaded_bytes, total_bytes, error_code FROM t_module ORDER BY name, version",
        )
        .fetch_all(&self.pool)
        .await
        .map_err(repository_error)?
        .into_iter()
        .map(map_module)
        .collect()
    }

    async fn upsert_module(
        &self,
        module: &MemberModule,
        relative_install_path: &str,
        sha256_hex: &str,
        updated_at: i64,
    ) -> Result<(), MemberServiceError> {
        if module.name.trim().is_empty()
            || module.version.trim().is_empty()
            || relative_install_path.trim().is_empty()
            || sha256_hex.len() != 64
            || !sha256_hex.bytes().all(|byte| byte.is_ascii_hexdigit())
            || updated_at <= 0
        {
            return Err(invalid_data());
        }
        let downloaded = to_i64(module.downloaded_bytes)?;
        let total = module.total_bytes.map(to_i64).transpose()?;
        sqlx::query(
            "INSERT INTO t_module (module_id, name, version, relative_install_path, sha256_hex, status, downloaded_bytes, total_bytes, error_code, installed_at, updated_at) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?) ON CONFLICT(module_id) DO UPDATE SET name = excluded.name, version = excluded.version, relative_install_path = excluded.relative_install_path, sha256_hex = excluded.sha256_hex, status = excluded.status, downloaded_bytes = excluded.downloaded_bytes, total_bytes = excluded.total_bytes, error_code = excluded.error_code, installed_at = excluded.installed_at, updated_at = excluded.updated_at",
        )
        .bind(module.module_id.as_str())
        .bind(&module.name)
        .bind(&module.version)
        .bind(relative_install_path)
        .bind(sha256_hex.to_ascii_lowercase())
        .bind(module_status_name(module.status))
        .bind(downloaded)
        .bind(total)
        .bind(&module.error_code)
        .bind((module.status == MemberModuleStatus::Installed).then_some(updated_at))
        .bind(updated_at)
        .execute(&self.pool)
        .await
        .map_err(repository_error)?;
        Ok(())
    }

    async fn remove_module(&self, module_id: &ModuleId) -> Result<bool, MemberServiceError> {
        Ok(sqlx::query("DELETE FROM t_module WHERE module_id = ?")
            .bind(module_id.as_str())
            .execute(&self.pool)
            .await
            .map_err(repository_error)?
            .rows_affected()
            == 1)
    }

    async fn list_orders(&self) -> Result<Vec<OrderRecord>, MemberServiceError> {
        sqlx::query(
            "SELECT order_id, user_uid, plan_id, plan_label, amount_minor, currency, status, created_at, paid_at FROM t_order ORDER BY created_at DESC",
        )
        .fetch_all(&self.pool)
        .await
        .map_err(repository_error)?
        .into_iter()
        .map(map_order)
        .collect()
    }

    async fn upsert_order(&self, order: &OrderRecord) -> Result<(), MemberServiceError> {
        if order.plan_id.trim().is_empty()
            || order.plan_label.trim().is_empty()
            || order.currency.len() != 3
            || order.created_at <= 0
        {
            return Err(invalid_data());
        }
        sqlx::query(
            "INSERT INTO t_order (order_id, user_uid, plan_id, plan_label, amount_minor, currency, status, created_at, paid_at) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?) ON CONFLICT(order_id) DO UPDATE SET status = excluded.status, paid_at = excluded.paid_at",
        )
        .bind(order.order_id.as_str())
        .bind(order.user_uid.as_str())
        .bind(&order.plan_id)
        .bind(&order.plan_label)
        .bind(to_i64(order.amount_minor)?)
        .bind(order.currency.to_ascii_uppercase())
        .bind(order_status_name(order.status))
        .bind(order.created_at)
        .bind(order.paid_at)
        .execute(&self.pool)
        .await
        .map_err(repository_error)?;
        Ok(())
    }

    async fn load_ai_config(&self) -> Result<Option<AiConfigRecord>, MemberServiceError> {
        let row = sqlx::query("SELECT identity_text, speaking_style, system_prompt, provider, endpoint, model_name, api_key_ciphertext, temperature, max_response_len, sensitive_filter_enabled, banned_words_json, danmaku_context_count, asr_context_count, manual_context_count, updated_at FROM t_ai_config WHERE id = 1")
            .fetch_optional(&self.pool)
            .await
            .map_err(repository_error)?;
        row.map(map_ai_config).transpose()
    }

    async fn save_ai_config(&self, config: &AiConfigRecord) -> Result<(), MemberServiceError> {
        validate_ai_config(config)?;
        let banned_words =
            serde_json::to_string(&config.banned_words).map_err(|_| invalid_data())?;
        sqlx::query("INSERT INTO t_ai_config (id, identity_text, speaking_style, system_prompt, provider, endpoint, model_name, api_key_ciphertext, temperature, max_response_len, sensitive_filter_enabled, banned_words_json, danmaku_context_count, asr_context_count, manual_context_count, updated_at) VALUES (1, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?) ON CONFLICT(id) DO UPDATE SET identity_text = excluded.identity_text, speaking_style = excluded.speaking_style, system_prompt = excluded.system_prompt, provider = excluded.provider, endpoint = excluded.endpoint, model_name = excluded.model_name, api_key_ciphertext = excluded.api_key_ciphertext, temperature = excluded.temperature, max_response_len = excluded.max_response_len, sensitive_filter_enabled = excluded.sensitive_filter_enabled, banned_words_json = excluded.banned_words_json, danmaku_context_count = excluded.danmaku_context_count, asr_context_count = excluded.asr_context_count, manual_context_count = excluded.manual_context_count, updated_at = excluded.updated_at")
            .bind(&config.identity)
            .bind(&config.speaking_style)
            .bind(&config.system_prompt)
            .bind(&config.provider)
            .bind(&config.endpoint)
            .bind(&config.model)
            .bind(&config.api_key_ciphertext)
            .bind(config.temperature)
            .bind(i64::from(config.max_length))
            .bind(config.is_sensitive_filter_enabled)
            .bind(banned_words)
            .bind(i64::from(config.danmaku_context_count))
            .bind(i64::from(config.asr_context_count))
            .bind(i64::from(config.manual_context_count))
            .bind(config.updated_at)
            .execute(&self.pool)
            .await
            .map_err(repository_error)?;
        Ok(())
    }

    async fn load_moderation_config(
        &self,
    ) -> Result<Option<ModerationConfigRecord>, MemberServiceError> {
        let row = sqlx::query("SELECT auto_enable, mute_duration, frequency_window_seconds, frequency_threshold, repeat_window_seconds, repeat_threshold, similarity_window_seconds, similarity_threshold, similarity_ratio, risk_confirmed, updated_at FROM t_moderate_config WHERE id = 1")
            .fetch_optional(&self.pool)
            .await
            .map_err(repository_error)?;
        row.map(map_moderation_config).transpose()
    }

    async fn save_moderation_config(
        &self,
        config: &ModerationConfigRecord,
    ) -> Result<(), MemberServiceError> {
        validate_moderation_config(config)?;
        sqlx::query("INSERT INTO t_moderate_config (id, auto_enable, mute_duration, frequency_window_seconds, frequency_threshold, repeat_window_seconds, repeat_threshold, similarity_window_seconds, similarity_threshold, similarity_ratio, risk_confirmed, updated_at) VALUES (1, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?) ON CONFLICT(id) DO UPDATE SET auto_enable = excluded.auto_enable, mute_duration = excluded.mute_duration, frequency_window_seconds = excluded.frequency_window_seconds, frequency_threshold = excluded.frequency_threshold, repeat_window_seconds = excluded.repeat_window_seconds, repeat_threshold = excluded.repeat_threshold, similarity_window_seconds = excluded.similarity_window_seconds, similarity_threshold = excluded.similarity_threshold, similarity_ratio = excluded.similarity_ratio, risk_confirmed = excluded.risk_confirmed, updated_at = excluded.updated_at")
            .bind(config.is_auto_enabled)
            .bind(&config.default_mute_duration)
            .bind(i64::from(config.frequency_window_seconds))
            .bind(i64::from(config.frequency_threshold))
            .bind(i64::from(config.repeat_window_seconds))
            .bind(i64::from(config.repeat_threshold))
            .bind(i64::from(config.similarity_window_seconds))
            .bind(i64::from(config.similarity_threshold))
            .bind(config.similarity_ratio)
            .bind(config.is_risk_confirmed)
            .bind(config.updated_at)
            .execute(&self.pool)
            .await
            .map_err(repository_error)?;
        Ok(())
    }
}

fn map_account(row: sqlx::sqlite::SqliteRow) -> Result<MemberAccount, MemberServiceError> {
    Ok(MemberAccount {
        user_uid: UserUid::parse(
            row.try_get::<String, _>("user_uid")
                .map_err(repository_error)?
                .as_str(),
        )
        .map_err(|_| invalid_data())?,
        username: required_text(row.try_get("username").map_err(repository_error)?)?,
        avatar_url: row.try_get("avatar_url").map_err(repository_error)?,
        logged_in_at: positive_i64(row.try_get("logged_in_at").map_err(repository_error)?)?,
    })
}

fn map_entitlement(row: sqlx::sqlite::SqliteRow) -> Result<StoredEntitlement, MemberServiceError> {
    let issued_at = positive_i64(row.try_get("issued_at").map_err(repository_error)?)?;
    let expires_at = positive_i64(row.try_get("expires_at").map_err(repository_error)?)?;
    let last_verified_at =
        positive_i64(row.try_get("last_verified_at").map_err(repository_error)?)?;
    if expires_at <= issued_at || last_verified_at < issued_at {
        return Err(invalid_data());
    }
    let token_ciphertext: Vec<u8> = row.try_get("token_ciphertext").map_err(repository_error)?;
    let token_signature: Vec<u8> = row.try_get("token_signature").map_err(repository_error)?;
    if token_ciphertext.is_empty() || token_signature.is_empty() {
        return Err(invalid_data());
    }
    Ok(StoredEntitlement {
        entitlement: Entitlement {
            status: parse_entitlement_status(
                &row.try_get::<String, _>("status")
                    .map_err(repository_error)?,
            )?,
            plan_id: required_text(row.try_get("plan_id").map_err(repository_error)?)?,
            expires_at,
            issued_at,
            last_verified_at,
        },
        token_ciphertext,
        token_signature,
    })
}

fn map_module(row: sqlx::sqlite::SqliteRow) -> Result<MemberModule, MemberServiceError> {
    let downloaded_bytes = to_u64(row.try_get("downloaded_bytes").map_err(repository_error)?)?;
    let total_bytes = row
        .try_get::<Option<i64>, _>("total_bytes")
        .map_err(repository_error)?
        .map(to_u64)
        .transpose()?;
    if total_bytes.is_some_and(|total| downloaded_bytes > total) {
        return Err(invalid_data());
    }
    Ok(MemberModule {
        module_id: ModuleId::parse(
            &row.try_get::<String, _>("module_id")
                .map_err(repository_error)?,
        )
        .map_err(|_| invalid_data())?,
        name: required_text(row.try_get("name").map_err(repository_error)?)?,
        version: required_text(row.try_get("version").map_err(repository_error)?)?,
        status: parse_module_status(
            &row.try_get::<String, _>("status")
                .map_err(repository_error)?,
        )?,
        downloaded_bytes,
        total_bytes,
        error_code: row.try_get("error_code").map_err(repository_error)?,
    })
}

fn map_order(row: sqlx::sqlite::SqliteRow) -> Result<OrderRecord, MemberServiceError> {
    Ok(OrderRecord {
        order_id: OrderId::parse(
            &row.try_get::<String, _>("order_id")
                .map_err(repository_error)?,
        )
        .map_err(|_| invalid_data())?,
        user_uid: UserUid::parse(
            &row.try_get::<String, _>("user_uid")
                .map_err(repository_error)?,
        )
        .map_err(|_| invalid_data())?,
        plan_id: required_text(row.try_get("plan_id").map_err(repository_error)?)?,
        plan_label: required_text(row.try_get("plan_label").map_err(repository_error)?)?,
        amount_minor: to_u64(row.try_get("amount_minor").map_err(repository_error)?)?,
        currency: required_text(row.try_get("currency").map_err(repository_error)?)?,
        status: parse_order_status(
            &row.try_get::<String, _>("status")
                .map_err(repository_error)?,
        )?,
        created_at: positive_i64(row.try_get("created_at").map_err(repository_error)?)?,
        paid_at: row.try_get("paid_at").map_err(repository_error)?,
    })
}

fn map_ai_config(row: sqlx::sqlite::SqliteRow) -> Result<AiConfigRecord, MemberServiceError> {
    let banned_words_json: String = row.try_get("banned_words_json").map_err(repository_error)?;
    let config = AiConfigRecord {
        identity: row.try_get("identity_text").map_err(repository_error)?,
        speaking_style: row.try_get("speaking_style").map_err(repository_error)?,
        system_prompt: row.try_get("system_prompt").map_err(repository_error)?,
        provider: row.try_get("provider").map_err(repository_error)?,
        endpoint: row.try_get("endpoint").map_err(repository_error)?,
        model: row.try_get("model_name").map_err(repository_error)?,
        api_key_ciphertext: row
            .try_get("api_key_ciphertext")
            .map_err(repository_error)?,
        temperature: row.try_get("temperature").map_err(repository_error)?,
        max_length: to_u32(row.try_get("max_response_len").map_err(repository_error)?)?,
        is_sensitive_filter_enabled: row
            .try_get("sensitive_filter_enabled")
            .map_err(repository_error)?,
        banned_words: serde_json::from_str(&banned_words_json).map_err(|_| invalid_data())?,
        danmaku_context_count: to_u32(
            row.try_get("danmaku_context_count")
                .map_err(repository_error)?,
        )?,
        asr_context_count: to_u32(row.try_get("asr_context_count").map_err(repository_error)?)?,
        manual_context_count: to_u32(
            row.try_get("manual_context_count")
                .map_err(repository_error)?,
        )?,
        updated_at: positive_i64(row.try_get("updated_at").map_err(repository_error)?)?,
    };
    validate_ai_config(&config)?;
    Ok(config)
}

fn map_moderation_config(
    row: sqlx::sqlite::SqliteRow,
) -> Result<ModerationConfigRecord, MemberServiceError> {
    let config = ModerationConfigRecord {
        is_auto_enabled: row.try_get("auto_enable").map_err(repository_error)?,
        default_mute_duration: row.try_get("mute_duration").map_err(repository_error)?,
        frequency_window_seconds: to_u32(
            row.try_get("frequency_window_seconds")
                .map_err(repository_error)?,
        )?,
        frequency_threshold: to_u32(
            row.try_get("frequency_threshold")
                .map_err(repository_error)?,
        )?,
        repeat_window_seconds: to_u32(
            row.try_get("repeat_window_seconds")
                .map_err(repository_error)?,
        )?,
        repeat_threshold: to_u32(row.try_get("repeat_threshold").map_err(repository_error)?)?,
        similarity_window_seconds: to_u32(
            row.try_get("similarity_window_seconds")
                .map_err(repository_error)?,
        )?,
        similarity_threshold: to_u32(
            row.try_get("similarity_threshold")
                .map_err(repository_error)?,
        )?,
        similarity_ratio: row.try_get("similarity_ratio").map_err(repository_error)?,
        is_risk_confirmed: row.try_get("risk_confirmed").map_err(repository_error)?,
        updated_at: positive_i64(row.try_get("updated_at").map_err(repository_error)?)?,
    };
    validate_moderation_config(&config)?;
    Ok(config)
}

fn validate_ai_config(config: &AiConfigRecord) -> Result<(), MemberServiceError> {
    if !matches!(
        config.speaking_style.as_str(),
        "short" | "humorous" | "formal" | "casual"
    ) || !config.temperature.is_finite()
        || !(0.0..=2.0).contains(&config.temperature)
        || config.max_length == 0
        || config.updated_at <= 0
        || config
            .banned_words
            .iter()
            .any(|word| word.trim().is_empty())
    {
        return Err(invalid_data());
    }
    Ok(())
}

fn validate_moderation_config(config: &ModerationConfigRecord) -> Result<(), MemberServiceError> {
    if !matches!(
        config.default_mute_duration.as_str(),
        "10m" | "1h" | "permanent"
    ) || config.frequency_window_seconds == 0
        || config.frequency_threshold == 0
        || config.repeat_window_seconds == 0
        || config.repeat_threshold == 0
        || config.similarity_window_seconds == 0
        || config.similarity_threshold == 0
        || !config.similarity_ratio.is_finite()
        || !(0.0..=1.0).contains(&config.similarity_ratio)
        || config.updated_at <= 0
        || (config.is_auto_enabled && !config.is_risk_confirmed)
    {
        return Err(invalid_data());
    }
    Ok(())
}

fn parse_entitlement_status(value: &str) -> Result<EntitlementStatus, MemberServiceError> {
    match value {
        "active" => Ok(EntitlementStatus::Active),
        "expired" => Ok(EntitlementStatus::Expired),
        _ => Err(invalid_data()),
    }
}

const fn entitlement_status_name(value: EntitlementStatus) -> &'static str {
    match value {
        EntitlementStatus::Active => "active",
        EntitlementStatus::Expired => "expired",
    }
}

fn parse_module_status(value: &str) -> Result<MemberModuleStatus, MemberServiceError> {
    match value {
        "missing" => Ok(MemberModuleStatus::Missing),
        "downloading" => Ok(MemberModuleStatus::Downloading),
        "verifying" => Ok(MemberModuleStatus::Verifying),
        "installed" => Ok(MemberModuleStatus::Installed),
        "failed" => Ok(MemberModuleStatus::Failed),
        _ => Err(invalid_data()),
    }
}

const fn module_status_name(value: MemberModuleStatus) -> &'static str {
    match value {
        MemberModuleStatus::Missing => "missing",
        MemberModuleStatus::Downloading => "downloading",
        MemberModuleStatus::Verifying => "verifying",
        MemberModuleStatus::Installed => "installed",
        MemberModuleStatus::Failed => "failed",
    }
}

fn parse_order_status(value: &str) -> Result<OrderStatus, MemberServiceError> {
    match value {
        "pending" => Ok(OrderStatus::Pending),
        "paid" => Ok(OrderStatus::Paid),
        "cancelled" => Ok(OrderStatus::Cancelled),
        "failed" => Ok(OrderStatus::Failed),
        "timed_out" => Ok(OrderStatus::TimedOut),
        _ => Err(invalid_data()),
    }
}

const fn order_status_name(value: OrderStatus) -> &'static str {
    match value {
        OrderStatus::Pending => "pending",
        OrderStatus::Paid => "paid",
        OrderStatus::Cancelled => "cancelled",
        OrderStatus::Failed => "failed",
        OrderStatus::TimedOut => "timed_out",
    }
}

fn required_text(value: String) -> Result<String, MemberServiceError> {
    if value.trim().is_empty() {
        Err(invalid_data())
    } else {
        Ok(value)
    }
}

fn positive_i64(value: i64) -> Result<i64, MemberServiceError> {
    (value > 0).then_some(value).ok_or_else(invalid_data)
}

fn to_i64(value: u64) -> Result<i64, MemberServiceError> {
    i64::try_from(value).map_err(|_| invalid_data())
}

fn to_u64(value: i64) -> Result<u64, MemberServiceError> {
    u64::try_from(value).map_err(|_| invalid_data())
}

fn to_u32(value: i64) -> Result<u32, MemberServiceError> {
    u32::try_from(value).map_err(|_| invalid_data())
}

const fn invalid_data() -> MemberServiceError {
    MemberServiceError::invalid("member_data_invalid")
}

fn repository_error(_: sqlx::Error) -> MemberServiceError {
    MemberServiceError::unavailable("member_storage_unavailable")
}
