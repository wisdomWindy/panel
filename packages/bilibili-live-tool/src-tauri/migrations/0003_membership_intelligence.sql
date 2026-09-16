DROP INDEX IF EXISTS idx_order_user_time;

ALTER TABLE t_account RENAME TO t_account_legacy;
CREATE TABLE t_account (
    user_uid TEXT PRIMARY KEY NOT NULL,
    username TEXT NOT NULL,
    avatar_url TEXT,
    logged_in_at INTEGER NOT NULL,
    updated_at INTEGER NOT NULL
);
INSERT INTO t_account (user_uid, username, avatar_url, logged_in_at, updated_at)
SELECT user_uid, COALESCE(username, ''), avatar,
       CAST(strftime('%s', COALESCE(create_time, CURRENT_TIMESTAMP)) AS INTEGER) * 1000,
       CAST(strftime('%s', COALESCE(update_time, CURRENT_TIMESTAMP)) AS INTEGER) * 1000
FROM t_account_legacy;
DROP TABLE t_account_legacy;

ALTER TABLE t_membership RENAME TO t_membership_legacy;
CREATE TABLE t_membership (
    user_uid TEXT PRIMARY KEY NOT NULL,
    status TEXT NOT NULL CHECK (status IN ('active', 'expired')),
    plan_id TEXT NOT NULL,
    token_ciphertext BLOB NOT NULL,
    token_signature BLOB NOT NULL,
    issued_at INTEGER NOT NULL,
    expires_at INTEGER NOT NULL,
    last_verified_at INTEGER NOT NULL,
    updated_at INTEGER NOT NULL
);
DROP TABLE t_membership_legacy;

ALTER TABLE t_order RENAME TO t_order_legacy;
CREATE TABLE t_order (
    order_id TEXT PRIMARY KEY NOT NULL,
    user_uid TEXT NOT NULL,
    plan_id TEXT NOT NULL,
    plan_label TEXT NOT NULL,
    amount_minor INTEGER NOT NULL CHECK (amount_minor >= 0),
    currency TEXT NOT NULL,
    status TEXT NOT NULL CHECK (status IN ('pending', 'paid', 'cancelled', 'failed', 'timed_out')),
    created_at INTEGER NOT NULL,
    paid_at INTEGER
);
INSERT INTO t_order (
    order_id, user_uid, plan_id, plan_label, amount_minor, currency, status, created_at, paid_at
)
SELECT order_id, user_uid, plan_id, plan_id,
       MAX(0, CAST(ROUND(amount * 100) AS INTEGER)), 'CNY',
       CASE status
           WHEN 'paid' THEN 'paid'
           WHEN 'cancelled' THEN 'cancelled'
           WHEN 'failed' THEN 'failed'
           WHEN 'timed_out' THEN 'timed_out'
           ELSE 'pending'
       END,
       CAST(strftime('%s', COALESCE(create_time, CURRENT_TIMESTAMP)) AS INTEGER) * 1000,
       CASE WHEN pay_time IS NULL THEN NULL ELSE CAST(strftime('%s', pay_time) AS INTEGER) * 1000 END
FROM t_order_legacy;
DROP TABLE t_order_legacy;
CREATE INDEX idx_order_user_time ON t_order(user_uid, created_at DESC);

ALTER TABLE t_module RENAME TO t_module_legacy;
CREATE TABLE t_module (
    module_id TEXT PRIMARY KEY NOT NULL,
    name TEXT NOT NULL,
    version TEXT NOT NULL,
    relative_install_path TEXT NOT NULL,
    sha256_hex TEXT NOT NULL,
    status TEXT NOT NULL CHECK (status IN ('missing', 'downloading', 'verifying', 'installed', 'failed')),
    downloaded_bytes INTEGER NOT NULL DEFAULT 0 CHECK (downloaded_bytes >= 0),
    total_bytes INTEGER CHECK (total_bytes IS NULL OR total_bytes >= 0),
    error_code TEXT,
    installed_at INTEGER,
    updated_at INTEGER NOT NULL
);
INSERT INTO t_module (
    module_id, name, version, relative_install_path, sha256_hex, status,
    downloaded_bytes, total_bytes, error_code, installed_at, updated_at
)
SELECT module_name, module_name, version, install_path, COALESCE(hash, ''),
       CASE status
           WHEN 'downloading' THEN 'downloading'
           WHEN 'verifying' THEN 'verifying'
           WHEN 'installed' THEN 'installed'
           WHEN 'failed' THEN 'failed'
           ELSE 'missing'
       END,
       0, NULL, NULL,
       CASE WHEN status = 'installed' THEN CAST(strftime('%s', COALESCE(install_time, CURRENT_TIMESTAMP)) AS INTEGER) * 1000 ELSE NULL END,
       CAST(strftime('%s', COALESCE(update_time, CURRENT_TIMESTAMP)) AS INTEGER) * 1000
FROM t_module_legacy;
DROP TABLE t_module_legacy;

ALTER TABLE t_ai_config RENAME TO t_ai_config_legacy;
CREATE TABLE t_ai_config (
    id INTEGER PRIMARY KEY CHECK (id = 1),
    identity_text TEXT NOT NULL DEFAULT '',
    speaking_style TEXT NOT NULL DEFAULT 'short' CHECK (speaking_style IN ('short', 'humorous', 'formal', 'casual')),
    system_prompt TEXT NOT NULL DEFAULT '',
    provider TEXT NOT NULL DEFAULT '',
    endpoint TEXT,
    model_name TEXT NOT NULL DEFAULT '',
    api_key_ciphertext BLOB,
    temperature REAL NOT NULL DEFAULT 0.7 CHECK (temperature >= 0 AND temperature <= 2),
    max_response_len INTEGER NOT NULL DEFAULT 100 CHECK (max_response_len > 0),
    sensitive_filter_enabled BOOLEAN NOT NULL DEFAULT TRUE,
    banned_words_json TEXT NOT NULL DEFAULT '[]',
    danmaku_context_count INTEGER NOT NULL DEFAULT 20 CHECK (danmaku_context_count >= 0),
    asr_context_count INTEGER NOT NULL DEFAULT 0 CHECK (asr_context_count >= 0),
    manual_context_count INTEGER NOT NULL DEFAULT 0 CHECK (manual_context_count >= 0),
    updated_at INTEGER NOT NULL
);
INSERT INTO t_ai_config (
    id, identity_text, speaking_style, system_prompt, provider, endpoint, model_name,
    api_key_ciphertext, temperature, max_response_len, sensitive_filter_enabled,
    banned_words_json, danmaku_context_count, asr_context_count, manual_context_count, updated_at
)
SELECT 1, COALESCE(ai_identity, ''),
       CASE speak_style
           WHEN 'humorous' THEN 'humorous'
           WHEN 'formal' THEN 'formal'
           WHEN 'casual' THEN 'casual'
           ELSE 'short'
       END,
       '', provider, endpoint, model_name, NULL, temperature, max_response_len,
       negative_intercept, COALESCE(ban_words, '[]'), context_window, 0, 0,
       CAST(strftime('%s', COALESCE(update_time, CURRENT_TIMESTAMP)) AS INTEGER) * 1000
FROM t_ai_config_legacy
ORDER BY id DESC LIMIT 1;
DROP TABLE t_ai_config_legacy;

ALTER TABLE t_asr_config RENAME TO t_asr_config_legacy;
CREATE TABLE t_asr_config (
    id INTEGER PRIMARY KEY CHECK (id = 1),
    model_name TEXT,
    language TEXT NOT NULL DEFAULT 'zh',
    denoise BOOLEAN NOT NULL DEFAULT TRUE,
    cache_seconds INTEGER NOT NULL DEFAULT 30 CHECK (cache_seconds = 30),
    license_accepted BOOLEAN NOT NULL DEFAULT FALSE,
    updated_at INTEGER NOT NULL
);
INSERT INTO t_asr_config (id, model_name, language, denoise, cache_seconds, license_accepted, updated_at)
SELECT 1, NULL, COALESCE(language, 'zh'), COALESCE(denoise, TRUE), 30, FALSE,
       CAST(strftime('%s', COALESCE(update_time, CURRENT_TIMESTAMP)) AS INTEGER) * 1000
FROM t_asr_config_legacy
ORDER BY id DESC LIMIT 1;
DROP TABLE t_asr_config_legacy;

ALTER TABLE t_moderate_config RENAME TO t_moderate_config_legacy;
CREATE TABLE t_moderate_config (
    id INTEGER PRIMARY KEY CHECK (id = 1),
    auto_enable BOOLEAN NOT NULL DEFAULT FALSE,
    mute_duration TEXT NOT NULL DEFAULT '1h' CHECK (mute_duration IN ('10m', '1h', 'permanent')),
    frequency_window_seconds INTEGER NOT NULL DEFAULT 10 CHECK (frequency_window_seconds > 0),
    frequency_threshold INTEGER NOT NULL DEFAULT 5 CHECK (frequency_threshold > 0),
    repeat_window_seconds INTEGER NOT NULL DEFAULT 60 CHECK (repeat_window_seconds > 0),
    repeat_threshold INTEGER NOT NULL DEFAULT 3 CHECK (repeat_threshold > 0),
    similarity_window_seconds INTEGER NOT NULL DEFAULT 60 CHECK (similarity_window_seconds > 0),
    similarity_threshold INTEGER NOT NULL DEFAULT 5 CHECK (similarity_threshold > 0),
    similarity_ratio REAL NOT NULL DEFAULT 0.8 CHECK (similarity_ratio >= 0 AND similarity_ratio <= 1),
    risk_confirmed BOOLEAN NOT NULL DEFAULT FALSE,
    updated_at INTEGER NOT NULL
);
INSERT INTO t_moderate_config (
    id, auto_enable, mute_duration, frequency_window_seconds, frequency_threshold,
    repeat_window_seconds, repeat_threshold, similarity_window_seconds,
    similarity_threshold, similarity_ratio, risk_confirmed, updated_at
)
SELECT 1, COALESCE(auto_enable, FALSE),
       CASE mute_duration WHEN '10m' THEN '10m' WHEN 'permanent' THEN 'permanent' ELSE '1h' END,
       10, MAX(1, COALESCE(freq_threshold, 5)), 60, MAX(1, COALESCE(repeat_threshold, 3)),
       60, MAX(1, COALESCE(similar_threshold, 5)),
       MIN(1, MAX(0, COALESCE(similar_ratio, 0.8))), COALESCE(risk_confirmed, FALSE),
       CAST(strftime('%s', COALESCE(update_time, CURRENT_TIMESTAMP)) AS INTEGER) * 1000
FROM t_moderate_config_legacy
ORDER BY id DESC LIMIT 1;
DROP TABLE t_moderate_config_legacy;

CREATE TABLE t_moderation_alert (
    alert_id TEXT PRIMARY KEY NOT NULL,
    user_uid TEXT NOT NULL,
    username TEXT NOT NULL,
    violation_description TEXT NOT NULL,
    content_example TEXT NOT NULL,
    occurred_at INTEGER NOT NULL,
    status TEXT NOT NULL CHECK (status IN ('pending', 'executing', 'muted', 'ignored', 'failed')),
    error_code TEXT
);

CREATE TABLE t_moderation_log (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    alert_id TEXT,
    user_uid TEXT NOT NULL,
    username TEXT NOT NULL,
    action TEXT NOT NULL CHECK (action IN ('auto_mute', 'manual_mute', 'ignore', 'false_positive')),
    occurred_at INTEGER NOT NULL,
    is_false_positive BOOLEAN NOT NULL DEFAULT FALSE,
    FOREIGN KEY (alert_id) REFERENCES t_moderation_alert(alert_id) ON DELETE SET NULL
);

CREATE INDEX idx_moderation_alert_status_time ON t_moderation_alert(status, occurred_at DESC);
CREATE INDEX idx_moderation_log_user_time ON t_moderation_log(user_uid, occurred_at DESC);

INSERT INTO t_schema_state (state_key) VALUES ('schema_version_3');
