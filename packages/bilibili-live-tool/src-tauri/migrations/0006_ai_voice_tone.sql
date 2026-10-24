ALTER TABLE t_ai_voice_config
ADD COLUMN voice_id TEXT NOT NULL DEFAULT 'gentle-peach'
CHECK (length(trim(voice_id)) > 0);

INSERT INTO t_schema_state (state_key) VALUES ('schema_version_6');

