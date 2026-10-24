ALTER TABLE t_ai_config
ADD COLUMN max_sentences INTEGER NOT NULL DEFAULT 2 CHECK (max_sentences BETWEEN 1 AND 3);

CREATE TABLE t_ai_model_profile (
    profile_id TEXT PRIMARY KEY NOT NULL,
    name TEXT NOT NULL,
    provider_kind TEXT NOT NULL CHECK (provider_kind IN ('cloud', 'aggregator', 'local', 'custom_openai')),
    base_url TEXT NOT NULL,
    model_name TEXT NOT NULL,
    temperature REAL NOT NULL CHECK (temperature >= 0 AND temperature <= 2),
    max_tokens INTEGER NOT NULL CHECK (max_tokens BETWEEN 1 AND 8192),
    timeout_seconds INTEGER NOT NULL CHECK (timeout_seconds BETWEEN 1 AND 120),
    proxy_url TEXT,
    is_enabled BOOLEAN NOT NULL DEFAULT TRUE,
    is_default BOOLEAN NOT NULL DEFAULT FALSE,
    connection_state TEXT NOT NULL DEFAULT 'untested' CHECK (connection_state IN ('untested', 'available', 'unavailable')),
    has_api_key BOOLEAN NOT NULL DEFAULT FALSE,
    updated_at INTEGER NOT NULL
);

CREATE UNIQUE INDEX idx_ai_model_default
ON t_ai_model_profile(is_default)
WHERE is_default = TRUE;

CREATE TABLE t_ai_voice_config (
    id INTEGER PRIMARY KEY CHECK (id = 1),
    is_enabled BOOLEAN NOT NULL DEFAULT FALSE,
    trigger_scope TEXT NOT NULL DEFAULT 'questions' CHECK (trigger_scope IN ('all', 'questions', 'captains', 'keywords', 'high_value')),
    keywords_json TEXT NOT NULL DEFAULT '[]',
    global_cooldown_seconds INTEGER NOT NULL DEFAULT 5 CHECK (global_cooldown_seconds BETWEEN 1 AND 60),
    user_cooldown_seconds INTEGER NOT NULL DEFAULT 10 CHECK (user_cooldown_seconds BETWEEN 5 AND 300),
    queue_capacity INTEGER NOT NULL DEFAULT 20 CHECK (queue_capacity BETWEEN 5 AND 100),
    duplicate_filter_enabled BOOLEAN NOT NULL DEFAULT TRUE,
    output_device_id TEXT,
    volume REAL NOT NULL DEFAULT 0.8 CHECK (volume >= 0 AND volume <= 1),
    audio_route_confirmed BOOLEAN NOT NULL DEFAULT FALSE,
    updated_at INTEGER NOT NULL
);

INSERT INTO t_ai_voice_config (id, updated_at)
VALUES (1, CAST(strftime('%s', CURRENT_TIMESTAMP) AS INTEGER) * 1000);

INSERT INTO t_schema_state (state_key) VALUES ('schema_version_4');
