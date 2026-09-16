ALTER TABLE t_tts_config ADD COLUMN is_enabled BOOLEAN NOT NULL DEFAULT FALSE;
ALTER TABLE t_tts_config ADD COLUMN event_kinds_json TEXT NOT NULL DEFAULT '[]';
ALTER TABLE t_tts_config ADD COLUMN templates_json TEXT NOT NULL DEFAULT '{}';

ALTER TABLE t_recording ADD COLUMN duration_seconds INTEGER NOT NULL DEFAULT 0;

ALTER TABLE t_highlight ADD COLUMN position_seconds INTEGER NOT NULL DEFAULT 0;
ALTER TABLE t_highlight ADD COLUMN clip_duration_seconds INTEGER NOT NULL DEFAULT 0;
ALTER TABLE t_highlight ADD COLUMN is_clip_available BOOLEAN NOT NULL DEFAULT FALSE;

DELETE FROM t_gift_sound
WHERE id NOT IN (SELECT MAX(id) FROM t_gift_sound GROUP BY gift_id);

CREATE UNIQUE INDEX idx_gift_sound_gift_id ON t_gift_sound(gift_id);
CREATE INDEX idx_recording_start_time ON t_recording(start_time DESC);
CREATE INDEX idx_highlight_recording_id ON t_highlight(recording_id, create_time DESC);

INSERT INTO t_schema_state (state_key) VALUES ('schema_version_2');
