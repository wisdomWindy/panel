use sqlx::{Row, SqlitePool};

use crate::{
    application::member::{
        AiConfigRecord, AiModelConnectionState, AiModelProfileRecord, AiReasoningEffort,
        AiVoiceConfigRecord, MemberRepository, MemberServiceError, ModerationConfigRecord,
        OrderRecord, OrderStatus, StoredEntitlement,
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
    async fn query_ai_logs(
        &self,
        user_uid: Option<&str>,
        started_at: Option<i64>,
        ended_at: Option<i64>,
    ) -> Result<Vec<crate::application::member::AiLogRecord>, MemberServiceError> {
        let rows = sqlx::query(
            "SELECT *, CAST(strftime('%s', create_time) AS INTEGER) * 1000 AS created_at
             FROM t_ai_log
             WHERE (? IS NULL OR user_uid = ?)
               AND (? IS NULL OR CAST(strftime('%s', create_time) AS INTEGER) * 1000 >= ?)
               AND (? IS NULL OR CAST(strftime('%s', create_time) AS INTEGER) * 1000 <= ?)
             ORDER BY create_time DESC, id DESC LIMIT 500",
        )
        .bind(user_uid)
        .bind(user_uid)
        .bind(started_at)
        .bind(started_at)
        .bind(ended_at)
        .bind(ended_at)
        .fetch_all(&self.pool)
        .await
        .map_err(repository_error)?;
        rows.into_iter()
            .map(|row| {
                Ok(crate::application::member::AiLogRecord {
                    id: row.try_get("id").map_err(repository_error)?,
                    operation_type: row.try_get("operation_type").map_err(repository_error)?,
                    username: row.try_get("username").map_err(repository_error)?,
                    user_uid: row.try_get("user_uid").map_err(repository_error)?,
                    original_content: row.try_get("original_content").map_err(repository_error)?,
                    ai_content: row.try_get("ai_content").map_err(repository_error)?,
                    generation_status: row
                        .try_get("generation_status")
                        .map_err(repository_error)?,
                    reject_reason: row.try_get("reject_reason").map_err(repository_error)?,
                    room_id: row.try_get("room_id").map_err(repository_error)?,
                    created_at: row.try_get("created_at").map_err(repository_error)?,
                })
            })
            .collect()
    }
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
        let row = sqlx::query("SELECT identity_text, speaking_style, system_prompt, provider, endpoint, model_name, api_key_ciphertext, temperature, max_response_len, max_sentences, sensitive_filter_enabled, banned_words_json, danmaku_context_count, manual_context_count, updated_at FROM t_ai_config WHERE id = 1")
            .fetch_optional(&self.pool)
            .await
            .map_err(repository_error)?;
        row.map(map_ai_config).transpose()
    }

    async fn save_ai_config(&self, config: &AiConfigRecord) -> Result<(), MemberServiceError> {
        validate_ai_config(config)?;
        let banned_words =
            serde_json::to_string(&config.banned_words).map_err(|_| invalid_data())?;
        sqlx::query("INSERT INTO t_ai_config (id, identity_text, speaking_style, system_prompt, provider, endpoint, model_name, api_key_ciphertext, temperature, max_response_len, max_sentences, sensitive_filter_enabled, banned_words_json, danmaku_context_count, manual_context_count, updated_at) VALUES (1, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?) ON CONFLICT(id) DO UPDATE SET identity_text = excluded.identity_text, speaking_style = excluded.speaking_style, system_prompt = excluded.system_prompt, provider = excluded.provider, endpoint = excluded.endpoint, model_name = excluded.model_name, api_key_ciphertext = excluded.api_key_ciphertext, temperature = excluded.temperature, max_response_len = excluded.max_response_len, max_sentences = excluded.max_sentences, sensitive_filter_enabled = excluded.sensitive_filter_enabled, banned_words_json = excluded.banned_words_json, danmaku_context_count = excluded.danmaku_context_count, manual_context_count = excluded.manual_context_count, updated_at = excluded.updated_at")
            .bind(&config.identity)
            .bind(&config.speaking_style)
            .bind(&config.system_prompt)
            .bind(&config.provider)
            .bind(&config.endpoint)
            .bind(&config.model)
            .bind(&config.api_key_ciphertext)
            .bind(config.temperature)
            .bind(i64::from(config.max_length))
            .bind(i64::from(config.max_sentences))
            .bind(config.is_sensitive_filter_enabled)
            .bind(banned_words)
            .bind(i64::from(config.danmaku_context_count))
            .bind(i64::from(config.manual_context_count))
            .bind(config.updated_at)
            .execute(&self.pool)
            .await
            .map_err(repository_error)?;
        Ok(())
    }

    async fn list_ai_model_profiles(
        &self,
    ) -> Result<Vec<AiModelProfileRecord>, MemberServiceError> {
        sqlx::query("SELECT profile_id, name, provider_kind, base_url, model_name, reasoning_effort, max_tokens, timeout_seconds, proxy_url, is_enabled, is_default, connection_state, has_api_key, updated_at FROM t_ai_model_profile ORDER BY is_default DESC, updated_at DESC")
            .fetch_all(&self.pool)
            .await
            .map_err(repository_error)?
            .into_iter()
            .map(map_ai_model_profile)
            .collect()
    }

    async fn load_ai_model_profile(
        &self,
        profile_id: &str,
    ) -> Result<Option<AiModelProfileRecord>, MemberServiceError> {
        let row = sqlx::query("SELECT profile_id, name, provider_kind, base_url, model_name, reasoning_effort, max_tokens, timeout_seconds, proxy_url, is_enabled, is_default, connection_state, has_api_key, updated_at FROM t_ai_model_profile WHERE profile_id = ?")
            .bind(profile_id)
            .fetch_optional(&self.pool)
            .await
            .map_err(repository_error)?;
        row.map(map_ai_model_profile).transpose()
    }

    async fn load_default_ai_model_profile(
        &self,
    ) -> Result<Option<AiModelProfileRecord>, MemberServiceError> {
        let row = sqlx::query("SELECT profile_id, name, provider_kind, base_url, model_name, reasoning_effort, max_tokens, timeout_seconds, proxy_url, is_enabled, is_default, connection_state, has_api_key, updated_at FROM t_ai_model_profile WHERE is_default = TRUE LIMIT 1")
            .fetch_optional(&self.pool)
            .await
            .map_err(repository_error)?;
        row.map(map_ai_model_profile).transpose()
    }

    async fn save_ai_model_profile(
        &self,
        profile: &AiModelProfileRecord,
    ) -> Result<(), MemberServiceError> {
        validate_ai_model_profile(profile)?;
        sqlx::query("INSERT INTO t_ai_model_profile (profile_id, name, provider_kind, base_url, model_name, temperature, reasoning_effort, max_tokens, timeout_seconds, proxy_url, is_enabled, is_default, connection_state, has_api_key, updated_at) VALUES (?, ?, ?, ?, ?, 0.7, ?, ?, ?, ?, ?, ?, ?, ?, ?) ON CONFLICT(profile_id) DO UPDATE SET name = excluded.name, provider_kind = excluded.provider_kind, base_url = excluded.base_url, model_name = excluded.model_name, reasoning_effort = excluded.reasoning_effort, max_tokens = excluded.max_tokens, timeout_seconds = excluded.timeout_seconds, proxy_url = excluded.proxy_url, is_enabled = excluded.is_enabled, connection_state = excluded.connection_state, has_api_key = excluded.has_api_key, updated_at = excluded.updated_at")
            .bind(&profile.profile_id)
            .bind(&profile.name)
            .bind(&profile.provider_kind)
            .bind(&profile.base_url)
            .bind(&profile.model)
            .bind(profile.reasoning_effort.as_str())
            .bind(i64::from(profile.max_tokens))
            .bind(i64::from(profile.timeout_seconds))
            .bind(&profile.proxy_url)
            .bind(profile.is_enabled)
            .bind(profile.is_default)
            .bind(ai_connection_state_name(profile.connection_state))
            .bind(profile.has_api_key)
            .bind(profile.updated_at)
            .execute(&self.pool)
            .await
            .map_err(repository_error)?;
        Ok(())
    }

    async fn delete_ai_model_profile(&self, profile_id: &str) -> Result<bool, MemberServiceError> {
        let result = sqlx::query(
            "DELETE FROM t_ai_model_profile WHERE profile_id = ? AND is_default = FALSE",
        )
        .bind(profile_id)
        .execute(&self.pool)
        .await
        .map_err(repository_error)?;
        Ok(result.rows_affected() > 0)
    }

    async fn set_default_ai_model_profile(
        &self,
        profile_id: &str,
        updated_at: i64,
    ) -> Result<(), MemberServiceError> {
        let mut transaction = self.pool.begin().await.map_err(repository_error)?;
        let exists: bool = sqlx::query_scalar("SELECT EXISTS(SELECT 1 FROM t_ai_model_profile WHERE profile_id = ? AND is_enabled = TRUE)")
            .bind(profile_id)
            .fetch_one(&mut *transaction)
            .await
            .map_err(repository_error)?;
        if !exists {
            transaction.rollback().await.map_err(repository_error)?;
            return Err(MemberServiceError::invalid("ai_model_profile_unavailable"));
        }
        sqlx::query("UPDATE t_ai_model_profile SET is_default = FALSE WHERE is_default = TRUE")
            .execute(&mut *transaction)
            .await
            .map_err(repository_error)?;
        sqlx::query(
            "UPDATE t_ai_model_profile SET is_default = TRUE, updated_at = ? WHERE profile_id = ?",
        )
        .bind(updated_at)
        .bind(profile_id)
        .execute(&mut *transaction)
        .await
        .map_err(repository_error)?;
        transaction.commit().await.map_err(repository_error)?;
        Ok(())
    }

    async fn load_ai_voice_config(&self) -> Result<AiVoiceConfigRecord, MemberServiceError> {
        let row = sqlx::query("SELECT is_enabled, trigger_scope, keywords_json, global_cooldown_seconds, user_cooldown_seconds, queue_capacity, duplicate_filter_enabled, output_device_id, voice_id, volume, audio_route_confirmed, updated_at FROM t_ai_voice_config WHERE id = 1")
            .fetch_one(&self.pool)
            .await
            .map_err(repository_error)?;
        map_ai_voice_config(row)
    }

    async fn save_ai_voice_config(
        &self,
        config: &AiVoiceConfigRecord,
    ) -> Result<(), MemberServiceError> {
        validate_ai_voice_config(config)?;
        let keywords = serde_json::to_string(&config.keywords).map_err(|_| invalid_data())?;
        sqlx::query("UPDATE t_ai_voice_config SET is_enabled = ?, trigger_scope = ?, keywords_json = ?, global_cooldown_seconds = ?, user_cooldown_seconds = ?, queue_capacity = ?, duplicate_filter_enabled = ?, output_device_id = ?, voice_id = ?, volume = ?, audio_route_confirmed = ?, updated_at = ? WHERE id = 1")
            .bind(config.is_enabled)
            .bind(&config.trigger_scope)
            .bind(keywords)
            .bind(i64::from(config.global_cooldown_seconds))
            .bind(i64::from(config.user_cooldown_seconds))
            .bind(i64::from(config.queue_capacity))
            .bind(config.is_duplicate_filter_enabled)
            .bind(&config.output_device_id)
            .bind(&config.voice_id)
            .bind(config.volume)
            .bind(config.is_audio_route_confirmed)
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
        max_sentences: to_u32(row.try_get("max_sentences").map_err(repository_error)?)?,
        is_sensitive_filter_enabled: row
            .try_get("sensitive_filter_enabled")
            .map_err(repository_error)?,
        banned_words: serde_json::from_str(&banned_words_json).map_err(|_| invalid_data())?,
        danmaku_context_count: to_u32(
            row.try_get("danmaku_context_count")
                .map_err(repository_error)?,
        )?,
        manual_context_count: to_u32(
            row.try_get("manual_context_count")
                .map_err(repository_error)?,
        )?,
        updated_at: positive_i64(row.try_get("updated_at").map_err(repository_error)?)?,
    };
    validate_ai_config(&config)?;
    Ok(config)
}

fn map_ai_model_profile(
    row: sqlx::sqlite::SqliteRow,
) -> Result<AiModelProfileRecord, MemberServiceError> {
    let profile = AiModelProfileRecord {
        profile_id: required_text(row.try_get("profile_id").map_err(repository_error)?)?,
        name: required_text(row.try_get("name").map_err(repository_error)?)?,
        provider_kind: required_text(row.try_get("provider_kind").map_err(repository_error)?)?,
        base_url: required_text(row.try_get("base_url").map_err(repository_error)?)?,
        model: required_text(row.try_get("model_name").map_err(repository_error)?)?,
        reasoning_effort: parse_ai_reasoning_effort(
            &row.try_get::<String, _>("reasoning_effort")
                .map_err(repository_error)?,
        )?,
        max_tokens: to_u32(row.try_get("max_tokens").map_err(repository_error)?)?,
        timeout_seconds: to_u32(row.try_get("timeout_seconds").map_err(repository_error)?)?,
        proxy_url: row.try_get("proxy_url").map_err(repository_error)?,
        is_enabled: row.try_get("is_enabled").map_err(repository_error)?,
        is_default: row.try_get("is_default").map_err(repository_error)?,
        connection_state: parse_ai_connection_state(
            &row.try_get::<String, _>("connection_state")
                .map_err(repository_error)?,
        )?,
        has_api_key: row.try_get("has_api_key").map_err(repository_error)?,
        updated_at: positive_i64(row.try_get("updated_at").map_err(repository_error)?)?,
    };
    validate_ai_model_profile(&profile)?;
    Ok(profile)
}

fn map_ai_voice_config(
    row: sqlx::sqlite::SqliteRow,
) -> Result<AiVoiceConfigRecord, MemberServiceError> {
    let keywords_json: String = row.try_get("keywords_json").map_err(repository_error)?;
    let config = AiVoiceConfigRecord {
        is_enabled: row.try_get("is_enabled").map_err(repository_error)?,
        trigger_scope: row.try_get("trigger_scope").map_err(repository_error)?,
        keywords: serde_json::from_str(&keywords_json).map_err(|_| invalid_data())?,
        global_cooldown_seconds: to_u32(
            row.try_get("global_cooldown_seconds")
                .map_err(repository_error)?,
        )?,
        user_cooldown_seconds: to_u32(
            row.try_get("user_cooldown_seconds")
                .map_err(repository_error)?,
        )?,
        queue_capacity: to_u32(row.try_get("queue_capacity").map_err(repository_error)?)?,
        is_duplicate_filter_enabled: row
            .try_get("duplicate_filter_enabled")
            .map_err(repository_error)?,
        output_device_id: row.try_get("output_device_id").map_err(repository_error)?,
        voice_id: row.try_get("voice_id").map_err(repository_error)?,
        volume: row.try_get("volume").map_err(repository_error)?,
        is_audio_route_confirmed: row
            .try_get("audio_route_confirmed")
            .map_err(repository_error)?,
        updated_at: positive_i64(row.try_get("updated_at").map_err(repository_error)?)?,
    };
    validate_ai_voice_config(&config)?;
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
        || !(1..=3).contains(&config.max_sentences)
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

fn validate_ai_model_profile(profile: &AiModelProfileRecord) -> Result<(), MemberServiceError> {
    let base_url = reqwest::Url::parse(profile.base_url.trim()).map_err(|_| invalid_data())?;
    let proxy_valid = profile.proxy_url.as_deref().is_none_or(|proxy| {
        reqwest::Url::parse(proxy).is_ok_and(|url| matches!(url.scheme(), "http" | "https"))
    });
    if profile.profile_id.trim().is_empty()
        || profile.name.trim().is_empty()
        || profile.model.trim().is_empty()
        || !matches!(
            profile.provider_kind.as_str(),
            "cloud" | "aggregator" | "local" | "custom_openai"
        )
        || !matches!(base_url.scheme(), "http" | "https")
        || !(1..=8192).contains(&profile.max_tokens)
        || !(1..=120).contains(&profile.timeout_seconds)
        || !proxy_valid
        || profile.updated_at <= 0
    {
        return Err(MemberServiceError::invalid("ai_model_profile_invalid"));
    }
    Ok(())
}

fn validate_ai_voice_config(config: &AiVoiceConfigRecord) -> Result<(), MemberServiceError> {
    if !matches!(
        config.trigger_scope.as_str(),
        "all" | "questions" | "captains" | "keywords" | "high_value"
    ) || !(1..=60).contains(&config.global_cooldown_seconds)
        || !(5..=300).contains(&config.user_cooldown_seconds)
        || !(5..=100).contains(&config.queue_capacity)
        || !config.volume.is_finite()
        || !(0.0..=1.0).contains(&config.volume)
        || config.voice_id.trim().is_empty()
        || config
            .keywords
            .iter()
            .any(|keyword| keyword.trim().is_empty())
        || config.updated_at <= 0
    {
        return Err(MemberServiceError::invalid("ai_voice_rules_invalid"));
    }
    Ok(())
}

const fn ai_connection_state_name(value: AiModelConnectionState) -> &'static str {
    match value {
        AiModelConnectionState::Untested => "untested",
        AiModelConnectionState::Available => "available",
        AiModelConnectionState::Unavailable => "unavailable",
    }
}

fn parse_ai_connection_state(value: &str) -> Result<AiModelConnectionState, MemberServiceError> {
    match value {
        "untested" => Ok(AiModelConnectionState::Untested),
        "available" => Ok(AiModelConnectionState::Available),
        "unavailable" => Ok(AiModelConnectionState::Unavailable),
        _ => Err(invalid_data()),
    }
}

fn parse_ai_reasoning_effort(value: &str) -> Result<AiReasoningEffort, MemberServiceError> {
    match value {
        "none" => Ok(AiReasoningEffort::None),
        "minimal" => Ok(AiReasoningEffort::Minimal),
        "low" => Ok(AiReasoningEffort::Low),
        "medium" => Ok(AiReasoningEffort::Medium),
        "high" => Ok(AiReasoningEffort::High),
        "xhigh" => Ok(AiReasoningEffort::XHigh),
        "max" => Ok(AiReasoningEffort::Max),
        _ => Err(invalid_data()),
    }
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
