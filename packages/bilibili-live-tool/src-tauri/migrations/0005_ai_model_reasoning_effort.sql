ALTER TABLE t_ai_model_profile
ADD COLUMN reasoning_effort TEXT NOT NULL DEFAULT 'medium'
CHECK (reasoning_effort IN ('none', 'minimal', 'low', 'medium', 'high', 'xhigh', 'max'));

INSERT INTO t_schema_state (state_key) VALUES ('schema_version_5');
