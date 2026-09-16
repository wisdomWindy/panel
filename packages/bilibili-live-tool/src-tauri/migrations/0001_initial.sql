CREATE TABLE t_account (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    user_uid TEXT UNIQUE NOT NULL,
    username TEXT,
    avatar TEXT,
    access_token TEXT,
    refresh_token TEXT,
    expire_at DATETIME,
    create_time DATETIME DEFAULT CURRENT_TIMESTAMP,
    update_time DATETIME DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE t_membership (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    user_uid TEXT UNIQUE NOT NULL,
    status TEXT NOT NULL,
    plan_id TEXT,
    expire_at DATETIME,
    token TEXT,
    token_signature TEXT,
    last_verified_at DATETIME,
    create_time DATETIME DEFAULT CURRENT_TIMESTAMP,
    update_time DATETIME DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE t_order (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    order_id TEXT UNIQUE NOT NULL,
    user_uid TEXT NOT NULL,
    plan_id TEXT NOT NULL,
    amount REAL NOT NULL,
    status TEXT NOT NULL,
    pay_method TEXT,
    create_time DATETIME DEFAULT CURRENT_TIMESTAMP,
    pay_time DATETIME
);

CREATE TABLE t_module (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    module_name TEXT NOT NULL,
    version TEXT NOT NULL,
    install_path TEXT NOT NULL,
    hash TEXT,
    status TEXT NOT NULL,
    install_time DATETIME DEFAULT CURRENT_TIMESTAMP,
    update_time DATETIME DEFAULT CURRENT_TIMESTAMP,
    UNIQUE(module_name, version)
);

CREATE TABLE t_ai_config (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    ai_enable BOOLEAN DEFAULT FALSE,
    provider TEXT NOT NULL,
    api_key TEXT,
    endpoint TEXT,
    model_name TEXT NOT NULL,
    temperature REAL DEFAULT 0.7,
    max_tokens INTEGER DEFAULT 60,
    ai_identity TEXT,
    speak_style TEXT,
    negative_intercept BOOLEAN DEFAULT TRUE,
    ban_words TEXT,
    max_response_len INTEGER DEFAULT 100,
    context_window INTEGER DEFAULT 20,
    create_time DATETIME DEFAULT CURRENT_TIMESTAMP,
    update_time DATETIME DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE t_ai_log (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    operation_type TEXT NOT NULL,
    username TEXT,
    user_uid TEXT,
    original_content TEXT,
    ai_content TEXT,
    generation_status TEXT NOT NULL,
    reject_reason TEXT,
    room_id TEXT NOT NULL,
    create_time DATETIME DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE t_room_config (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    room_id TEXT UNIQUE NOT NULL,
    room_name TEXT,
    enable_danmu_notify BOOLEAN DEFAULT TRUE,
    enable_gift_notify BOOLEAN DEFAULT TRUE,
    enable_captain_notify BOOLEAN DEFAULT TRUE,
    enable_tts BOOLEAN DEFAULT TRUE,
    tts_volume REAL DEFAULT 0.8,
    record_auto_save BOOLEAN DEFAULT FALSE,
    record_format TEXT DEFAULT 'mp4',
    record_quality TEXT DEFAULT 'high',
    create_time DATETIME DEFAULT CURRENT_TIMESTAMP,
    update_time DATETIME DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE t_sensitive_word (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    word TEXT UNIQUE NOT NULL,
    word_type TEXT NOT NULL,
    match_type TEXT DEFAULT 'fuzzy',
    enabled BOOLEAN DEFAULT TRUE,
    create_time DATETIME DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE t_black_user (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    user_uid TEXT UNIQUE NOT NULL,
    username TEXT,
    reason TEXT,
    create_time DATETIME DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE t_gift_sound (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    gift_id TEXT NOT NULL,
    gift_name TEXT NOT NULL,
    sound_path TEXT,
    volume REAL DEFAULT 0.7,
    enable BOOLEAN DEFAULT TRUE,
    create_time DATETIME DEFAULT CURRENT_TIMESTAMP,
    update_time DATETIME DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE t_live_stat (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    room_id TEXT NOT NULL,
    live_date TEXT NOT NULL,
    total_danmu INTEGER DEFAULT 0,
    total_gift_value REAL DEFAULT 0,
    total_gift_num INTEGER DEFAULT 0,
    total_captain INTEGER DEFAULT 0,
    max_online INTEGER DEFAULT 0,
    create_time DATETIME DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE t_app_setting (
    key TEXT PRIMARY KEY,
    value TEXT,
    scope TEXT DEFAULT 'global',
    update_time DATETIME DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE t_event_log (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    room_id TEXT NOT NULL,
    event_type TEXT NOT NULL,
    user_uid TEXT,
    payload_json TEXT,
    filtered BOOLEAN DEFAULT FALSE,
    create_time DATETIME DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE t_recording (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    room_id TEXT NOT NULL,
    file_path TEXT NOT NULL,
    format TEXT,
    quality TEXT,
    start_time DATETIME,
    end_time DATETIME,
    size INTEGER,
    status TEXT,
    create_time DATETIME DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE t_highlight (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    recording_id INTEGER,
    room_id TEXT NOT NULL,
    trigger_type TEXT,
    start_time DATETIME,
    end_time DATETIME,
    file_path TEXT,
    create_time DATETIME DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE t_plugin (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL,
    app_version TEXT,
    enabled BOOLEAN DEFAULT FALSE,
    permissions_json TEXT,
    source TEXT,
    signature TEXT,
    create_time DATETIME DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE t_tts_config (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    engine TEXT,
    voice TEXT,
    volume REAL DEFAULT 0.8,
    rate REAL DEFAULT 1.0,
    identity_filter TEXT,
    update_time DATETIME DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE t_asr_config (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    model_path TEXT,
    language TEXT DEFAULT 'zh',
    denoise BOOLEAN DEFAULT TRUE,
    cache_seconds INTEGER DEFAULT 30,
    update_time DATETIME DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE t_moderate_config (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    auto_enable BOOLEAN DEFAULT FALSE,
    mute_duration TEXT DEFAULT '1h',
    freq_threshold INTEGER DEFAULT 5,
    repeat_threshold INTEGER DEFAULT 3,
    similar_threshold INTEGER DEFAULT 5,
    similar_ratio REAL DEFAULT 0.8,
    risk_confirmed BOOLEAN DEFAULT FALSE,
    update_time DATETIME DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE t_ranking_archive (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    room_id TEXT NOT NULL,
    rank_type TEXT NOT NULL,
    live_date TEXT NOT NULL,
    payload_json TEXT,
    create_time DATETIME DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE t_schema_state (
    state_key TEXT PRIMARY KEY,
    applied_at DATETIME DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX idx_event_room_time ON t_event_log(room_id, create_time);
CREATE INDEX idx_ai_log_room_time ON t_ai_log(room_id, create_time);
CREATE INDEX idx_live_stat_room_date ON t_live_stat(room_id, live_date);
CREATE INDEX idx_black_user_uid ON t_black_user(user_uid);
CREATE INDEX idx_sensitive_word_type ON t_sensitive_word(word_type);
CREATE INDEX idx_ranking_archive_room_date ON t_ranking_archive(room_id, live_date);
CREATE INDEX idx_order_user_time ON t_order(user_uid, create_time);

INSERT INTO t_schema_state (state_key) VALUES ('schema_version_1');
