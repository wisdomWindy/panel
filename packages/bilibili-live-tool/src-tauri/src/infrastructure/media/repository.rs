use std::path::PathBuf;

use serde::{de::DeserializeOwned, Serialize};
use sqlx::{Row, SqlitePool};

use crate::{
    application::media::{
        GiftSoundBindingRecord, HighlightRecord, HighlightTriggerType, RecordingItemState,
        RecordingRecord,
    },
    domain::media::{
        validate_captain_effect_config, validate_gift_notify_config, validate_gift_sound_policy,
        validate_recording_config, validate_tts_config, CaptainEffectConfig, GiftNotifyConfig,
        GiftSoundPolicy, RecordingConfig, RecordingFormat, RecordingQuality, TtsConfig,
        TtsSelectedEngine,
    },
};

const CONFIG_SCOPE: &str = "global";
const CAPTAIN_CONFIG_KEY: &str = "media.captain_effect";
const GIFT_NOTIFY_CONFIG_KEY: &str = "media.gift_notify";
const GIFT_SOUND_POLICY_KEY: &str = "media.gift_sound_policy";
const RECORDING_CONFIG_KEY: &str = "media.recording";

#[derive(Debug, thiserror::Error)]
pub enum MediaRepositoryError {
    #[error("invalid media persistence data: {0}")]
    InvalidData(&'static str),
    #[error("media repository query failed: {0}")]
    Query(#[from] sqlx::Error),
}

#[derive(Clone)]
pub struct SqliteMediaRepository {
    pool: SqlitePool,
}

impl SqliteMediaRepository {
    pub const fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }

    pub async fn load_captain_config(
        &self,
    ) -> Result<Option<CaptainEffectConfig>, MediaRepositoryError> {
        self.load_json_config(CAPTAIN_CONFIG_KEY, validate_captain_effect_config)
            .await
    }

    pub async fn save_captain_config(
        &self,
        config: &CaptainEffectConfig,
    ) -> Result<(), MediaRepositoryError> {
        validate_captain_effect_config(config).map_err(|_| invalid("captain config"))?;
        self.save_json_config(CAPTAIN_CONFIG_KEY, config).await
    }

    pub async fn load_gift_notify_config(
        &self,
    ) -> Result<Option<GiftNotifyConfig>, MediaRepositoryError> {
        self.load_json_config(GIFT_NOTIFY_CONFIG_KEY, validate_gift_notify_config)
            .await
    }

    pub async fn save_gift_notify_config(
        &self,
        config: &GiftNotifyConfig,
    ) -> Result<(), MediaRepositoryError> {
        validate_gift_notify_config(config).map_err(|_| invalid("gift notify config"))?;
        self.save_json_config(GIFT_NOTIFY_CONFIG_KEY, config).await
    }

    pub async fn load_gift_sound_policy(
        &self,
    ) -> Result<Option<GiftSoundPolicy>, MediaRepositoryError> {
        self.load_json_config(GIFT_SOUND_POLICY_KEY, validate_gift_sound_policy)
            .await
    }

    pub async fn save_gift_sound_policy(
        &self,
        config: &GiftSoundPolicy,
    ) -> Result<(), MediaRepositoryError> {
        validate_gift_sound_policy(config).map_err(|_| invalid("gift sound policy"))?;
        self.save_json_config(GIFT_SOUND_POLICY_KEY, config).await
    }

    pub async fn load_recording_config(
        &self,
    ) -> Result<Option<RecordingConfig>, MediaRepositoryError> {
        self.load_json_config(RECORDING_CONFIG_KEY, validate_recording_config)
            .await
    }

    pub async fn save_recording_config(
        &self,
        config: &RecordingConfig,
    ) -> Result<(), MediaRepositoryError> {
        validate_recording_config(config).map_err(|_| invalid("recording config"))?;
        self.save_json_config(RECORDING_CONFIG_KEY, config).await
    }

    pub async fn load_tts_config(&self) -> Result<Option<TtsConfig>, MediaRepositoryError> {
        let row = sqlx::query(
            "SELECT engine, voice, volume, rate, identity_filter, is_enabled, event_kinds_json, templates_json FROM t_tts_config WHERE id = 1",
        )
        .fetch_optional(&self.pool)
        .await?;
        let Some(row) = row else {
            return Ok(None);
        };
        let engine: String = row.try_get("engine")?;
        let config = TtsConfig {
            is_enabled: row.try_get("is_enabled")?,
            selected_engine: parse_tts_engine(&engine)?,
            selected_voice_id: row.try_get("voice")?,
            rate: row.try_get("rate")?,
            volume: row.try_get("volume")?,
            event_kinds: decode_json(row.try_get("event_kinds_json")?, "tts event kinds")?,
            identity_kinds: decode_json(row.try_get("identity_filter")?, "tts identities")?,
            templates: decode_json(row.try_get("templates_json")?, "tts templates")?,
        };
        validate_tts_config(&config).map_err(|_| invalid("tts config"))?;
        Ok(Some(config))
    }

    pub async fn save_tts_config(&self, config: &TtsConfig) -> Result<(), MediaRepositoryError> {
        validate_tts_config(config).map_err(|_| invalid("tts config"))?;
        let event_kinds = encode_json(&config.event_kinds, "tts event kinds")?;
        let identity_kinds = encode_json(&config.identity_kinds, "tts identities")?;
        let templates = encode_json(&config.templates, "tts templates")?;
        sqlx::query(
            "INSERT INTO t_tts_config (id, engine, voice, volume, rate, identity_filter, is_enabled, event_kinds_json, templates_json) VALUES (1, ?, ?, ?, ?, ?, ?, ?, ?) ON CONFLICT(id) DO UPDATE SET engine = excluded.engine, voice = excluded.voice, volume = excluded.volume, rate = excluded.rate, identity_filter = excluded.identity_filter, is_enabled = excluded.is_enabled, event_kinds_json = excluded.event_kinds_json, templates_json = excluded.templates_json, update_time = CURRENT_TIMESTAMP",
        )
        .bind(tts_engine_name(config.selected_engine))
        .bind(&config.selected_voice_id)
        .bind(config.volume)
        .bind(config.rate)
        .bind(identity_kinds)
        .bind(config.is_enabled)
        .bind(event_kinds)
        .bind(templates)
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    pub async fn list_gift_sounds(
        &self,
    ) -> Result<Vec<GiftSoundBindingRecord>, MediaRepositoryError> {
        sqlx::query(
            "SELECT gift_id, gift_name, sound_path, volume, enable FROM t_gift_sound ORDER BY id",
        )
        .fetch_all(&self.pool)
        .await?
        .into_iter()
        .map(map_gift_sound)
        .collect()
    }

    pub async fn upsert_gift_sound(
        &self,
        binding: &GiftSoundBindingRecord,
    ) -> Result<(), MediaRepositoryError> {
        validate_gift_sound(binding)?;
        sqlx::query(
            "INSERT INTO t_gift_sound (gift_id, gift_name, sound_path, volume, enable) VALUES (?, ?, ?, ?, ?) ON CONFLICT(gift_id) DO UPDATE SET gift_name = excluded.gift_name, sound_path = excluded.sound_path, volume = excluded.volume, enable = excluded.enable, update_time = CURRENT_TIMESTAMP",
        )
        .bind(&binding.gift_id)
        .bind(&binding.gift_name)
        .bind(path_to_string(binding.sound_path.as_ref()))
        .bind(binding.volume)
        .bind(binding.is_enabled)
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    pub async fn remove_gift_sound(&self, gift_id: &str) -> Result<bool, MediaRepositoryError> {
        Ok(sqlx::query("DELETE FROM t_gift_sound WHERE gift_id = ?")
            .bind(gift_id)
            .execute(&self.pool)
            .await?
            .rows_affected()
            == 1)
    }

    pub async fn replace_gift_sounds(
        &self,
        bindings: &[GiftSoundBindingRecord],
    ) -> Result<(), MediaRepositoryError> {
        for binding in bindings {
            validate_gift_sound(binding)?;
        }
        let mut transaction = self.pool.begin().await?;
        sqlx::query("DELETE FROM t_gift_sound")
            .execute(&mut *transaction)
            .await?;
        for binding in bindings {
            let result = sqlx::query(
                "INSERT INTO t_gift_sound (gift_id, gift_name, sound_path, volume, enable) VALUES (?, ?, ?, ?, ?)",
            )
            .bind(&binding.gift_id)
            .bind(&binding.gift_name)
            .bind(path_to_string(binding.sound_path.as_ref()))
            .bind(binding.volume)
            .bind(binding.is_enabled)
            .execute(&mut *transaction)
            .await;
            if let Err(error) = result {
                transaction.rollback().await?;
                return Err(error.into());
            }
        }
        transaction.commit().await?;
        Ok(())
    }

    pub async fn insert_recording(
        &self,
        recording: &RecordingRecord,
    ) -> Result<RecordingRecord, MediaRepositoryError> {
        validate_recording(recording)?;
        let result = sqlx::query(
            "INSERT INTO t_recording (room_id, file_path, format, quality, start_time, end_time, size, status, duration_seconds) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)",
        )
        .bind(&recording.room_id)
        .bind(recording.file_path.to_string_lossy().as_ref())
        .bind(recording_format_name(recording.format))
        .bind(recording_quality_name(recording.quality))
        .bind(recording.started_at)
        .bind(recording.ended_at)
        .bind(to_i64(recording.size_bytes, "recording size")?)
        .bind(recording_state_name(recording.status))
        .bind(to_i64(recording.duration_seconds, "recording duration")?)
        .execute(&self.pool)
        .await?;
        Ok(RecordingRecord {
            id: result.last_insert_rowid(),
            ..recording.clone()
        })
    }

    pub async fn update_recording(
        &self,
        recording: &RecordingRecord,
    ) -> Result<bool, MediaRepositoryError> {
        validate_recording(recording)?;
        Ok(sqlx::query(
            "UPDATE t_recording SET room_id = ?, file_path = ?, format = ?, quality = ?, start_time = ?, end_time = ?, size = ?, status = ?, duration_seconds = ? WHERE id = ?",
        )
        .bind(&recording.room_id)
        .bind(recording.file_path.to_string_lossy().as_ref())
        .bind(recording_format_name(recording.format))
        .bind(recording_quality_name(recording.quality))
        .bind(recording.started_at)
        .bind(recording.ended_at)
        .bind(to_i64(recording.size_bytes, "recording size")?)
        .bind(recording_state_name(recording.status))
        .bind(to_i64(recording.duration_seconds, "recording duration")?)
        .bind(recording.id)
        .execute(&self.pool)
        .await?
        .rows_affected()
            == 1)
    }

    pub async fn list_recordings(&self) -> Result<Vec<RecordingRecord>, MediaRepositoryError> {
        sqlx::query(
            "SELECT id, room_id, file_path, format, quality, CAST(start_time AS INTEGER) AS started_at, CAST(end_time AS INTEGER) AS ended_at, size, status, duration_seconds FROM t_recording ORDER BY id",
        )
        .fetch_all(&self.pool)
        .await?
        .into_iter()
        .map(map_recording)
        .collect()
    }

    pub async fn delete_recording(&self, id: i64) -> Result<bool, MediaRepositoryError> {
        let mut transaction = self.pool.begin().await?;
        sqlx::query("DELETE FROM t_highlight WHERE recording_id = ?")
            .bind(id)
            .execute(&mut *transaction)
            .await?;
        let deleted = sqlx::query("DELETE FROM t_recording WHERE id = ?")
            .bind(id)
            .execute(&mut *transaction)
            .await?
            .rows_affected()
            == 1;
        transaction.commit().await?;
        Ok(deleted)
    }

    pub async fn insert_highlight(
        &self,
        highlight: &HighlightRecord,
    ) -> Result<HighlightRecord, MediaRepositoryError> {
        validate_highlight(highlight)?;
        let result = sqlx::query(
            "INSERT INTO t_highlight (recording_id, room_id, trigger_type, file_path, create_time, position_seconds, clip_duration_seconds, is_clip_available) VALUES (?, ?, ?, ?, ?, ?, ?, ?)",
        )
        .bind(highlight.recording_id)
        .bind(&highlight.room_id)
        .bind(highlight_trigger_name(highlight.trigger_type))
        .bind(path_to_string(highlight.file_path.as_ref()))
        .bind(highlight.created_at)
        .bind(to_i64(highlight.position_seconds, "highlight position")?)
        .bind(to_i64(highlight.clip_duration_seconds, "highlight duration")?)
        .bind(highlight.is_clip_available)
        .execute(&self.pool)
        .await?;
        Ok(HighlightRecord {
            id: result.last_insert_rowid(),
            ..highlight.clone()
        })
    }

    pub async fn list_highlights(
        &self,
        recording_id: i64,
    ) -> Result<Vec<HighlightRecord>, MediaRepositoryError> {
        sqlx::query(
            "SELECT id, recording_id, room_id, trigger_type, file_path, CAST(create_time AS INTEGER) AS created_at, position_seconds, clip_duration_seconds, is_clip_available FROM t_highlight WHERE recording_id = ? ORDER BY id",
        )
        .bind(recording_id)
        .fetch_all(&self.pool)
        .await?
        .into_iter()
        .map(map_highlight)
        .collect()
    }

    pub async fn delete_highlight(&self, id: i64) -> Result<bool, MediaRepositoryError> {
        Ok(sqlx::query("DELETE FROM t_highlight WHERE id = ?")
            .bind(id)
            .execute(&self.pool)
            .await?
            .rows_affected()
            == 1)
    }

    async fn load_json_config<T>(
        &self,
        key: &'static str,
        validate: fn(&T) -> Result<(), crate::domain::media::MediaDomainError>,
    ) -> Result<Option<T>, MediaRepositoryError>
    where
        T: DeserializeOwned,
    {
        let value: Option<String> =
            sqlx::query_scalar("SELECT value FROM t_app_setting WHERE key = ? AND scope = ?")
                .bind(key)
                .bind(CONFIG_SCOPE)
                .fetch_optional(&self.pool)
                .await?;
        let Some(value) = value else {
            return Ok(None);
        };
        let config = decode_json(&value, "media config")?;
        validate(&config).map_err(|_| invalid("media config"))?;
        Ok(Some(config))
    }

    async fn save_json_config<T: Serialize>(
        &self,
        key: &'static str,
        config: &T,
    ) -> Result<(), MediaRepositoryError> {
        let value = encode_json(config, "media config")?;
        sqlx::query(
            "INSERT INTO t_app_setting (key, value, scope) VALUES (?, ?, ?) ON CONFLICT(key) DO UPDATE SET value = excluded.value, scope = excluded.scope, update_time = CURRENT_TIMESTAMP",
        )
        .bind(key)
        .bind(value)
        .bind(CONFIG_SCOPE)
        .execute(&self.pool)
        .await?;
        Ok(())
    }
}

impl From<MediaRepositoryError> for crate::application::media::MediaIoError {
    fn from(error: MediaRepositoryError) -> Self {
        match error {
            MediaRepositoryError::InvalidData(name) => {
                Self::invalid(format!("invalid persisted {name}"))
            }
            MediaRepositoryError::Query(_) => {
                Self::operation("media storage operation failed", true)
            }
        }
    }
}

#[async_trait::async_trait]
impl crate::application::media::MediaRepository for SqliteMediaRepository {
    async fn load_captain_config(
        &self,
    ) -> Result<Option<CaptainEffectConfig>, crate::application::media::MediaIoError> {
        SqliteMediaRepository::load_captain_config(self)
            .await
            .map_err(Into::into)
    }

    async fn save_captain_config(
        &self,
        config: &CaptainEffectConfig,
    ) -> Result<(), crate::application::media::MediaIoError> {
        SqliteMediaRepository::save_captain_config(self, config)
            .await
            .map_err(Into::into)
    }

    async fn load_gift_notify_config(
        &self,
    ) -> Result<Option<GiftNotifyConfig>, crate::application::media::MediaIoError> {
        SqliteMediaRepository::load_gift_notify_config(self)
            .await
            .map_err(Into::into)
    }

    async fn save_gift_notify_config(
        &self,
        config: &GiftNotifyConfig,
    ) -> Result<(), crate::application::media::MediaIoError> {
        SqliteMediaRepository::save_gift_notify_config(self, config)
            .await
            .map_err(Into::into)
    }

    async fn load_gift_sound_policy(
        &self,
    ) -> Result<Option<GiftSoundPolicy>, crate::application::media::MediaIoError> {
        SqliteMediaRepository::load_gift_sound_policy(self)
            .await
            .map_err(Into::into)
    }

    async fn save_gift_sound_policy(
        &self,
        config: &GiftSoundPolicy,
    ) -> Result<(), crate::application::media::MediaIoError> {
        SqliteMediaRepository::save_gift_sound_policy(self, config)
            .await
            .map_err(Into::into)
    }

    async fn load_recording_config(
        &self,
    ) -> Result<Option<RecordingConfig>, crate::application::media::MediaIoError> {
        SqliteMediaRepository::load_recording_config(self)
            .await
            .map_err(Into::into)
    }

    async fn save_recording_config(
        &self,
        config: &RecordingConfig,
    ) -> Result<(), crate::application::media::MediaIoError> {
        SqliteMediaRepository::save_recording_config(self, config)
            .await
            .map_err(Into::into)
    }

    async fn load_tts_config(
        &self,
    ) -> Result<Option<TtsConfig>, crate::application::media::MediaIoError> {
        SqliteMediaRepository::load_tts_config(self)
            .await
            .map_err(Into::into)
    }

    async fn save_tts_config(
        &self,
        config: &TtsConfig,
    ) -> Result<(), crate::application::media::MediaIoError> {
        SqliteMediaRepository::save_tts_config(self, config)
            .await
            .map_err(Into::into)
    }

    async fn list_gift_sounds(
        &self,
    ) -> Result<Vec<GiftSoundBindingRecord>, crate::application::media::MediaIoError> {
        SqliteMediaRepository::list_gift_sounds(self)
            .await
            .map_err(Into::into)
    }

    async fn upsert_gift_sound(
        &self,
        binding: &GiftSoundBindingRecord,
    ) -> Result<(), crate::application::media::MediaIoError> {
        SqliteMediaRepository::upsert_gift_sound(self, binding)
            .await
            .map_err(Into::into)
    }

    async fn remove_gift_sound(
        &self,
        gift_id: &str,
    ) -> Result<bool, crate::application::media::MediaIoError> {
        SqliteMediaRepository::remove_gift_sound(self, gift_id)
            .await
            .map_err(Into::into)
    }

    async fn replace_gift_sounds(
        &self,
        bindings: &[GiftSoundBindingRecord],
    ) -> Result<(), crate::application::media::MediaIoError> {
        SqliteMediaRepository::replace_gift_sounds(self, bindings)
            .await
            .map_err(Into::into)
    }

    async fn insert_recording(
        &self,
        recording: &RecordingRecord,
    ) -> Result<RecordingRecord, crate::application::media::MediaIoError> {
        SqliteMediaRepository::insert_recording(self, recording)
            .await
            .map_err(Into::into)
    }

    async fn update_recording(
        &self,
        recording: &RecordingRecord,
    ) -> Result<bool, crate::application::media::MediaIoError> {
        SqliteMediaRepository::update_recording(self, recording)
            .await
            .map_err(Into::into)
    }

    async fn list_recordings(
        &self,
    ) -> Result<Vec<RecordingRecord>, crate::application::media::MediaIoError> {
        SqliteMediaRepository::list_recordings(self)
            .await
            .map_err(Into::into)
    }

    async fn delete_recording(
        &self,
        id: i64,
    ) -> Result<bool, crate::application::media::MediaIoError> {
        SqliteMediaRepository::delete_recording(self, id)
            .await
            .map_err(Into::into)
    }

    async fn insert_highlight(
        &self,
        highlight: &HighlightRecord,
    ) -> Result<HighlightRecord, crate::application::media::MediaIoError> {
        SqliteMediaRepository::insert_highlight(self, highlight)
            .await
            .map_err(Into::into)
    }

    async fn list_highlights(
        &self,
        recording_id: i64,
    ) -> Result<Vec<HighlightRecord>, crate::application::media::MediaIoError> {
        SqliteMediaRepository::list_highlights(self, recording_id)
            .await
            .map_err(Into::into)
    }

    async fn delete_highlight(
        &self,
        id: i64,
    ) -> Result<bool, crate::application::media::MediaIoError> {
        SqliteMediaRepository::delete_highlight(self, id)
            .await
            .map_err(Into::into)
    }
}

fn encode_json<T: Serialize>(
    value: &T,
    name: &'static str,
) -> Result<String, MediaRepositoryError> {
    serde_json::to_string(value).map_err(|_| invalid(name))
}

fn decode_json<T: DeserializeOwned>(
    value: &str,
    name: &'static str,
) -> Result<T, MediaRepositoryError> {
    serde_json::from_str(value).map_err(|_| invalid(name))
}

fn invalid(name: &'static str) -> MediaRepositoryError {
    MediaRepositoryError::InvalidData(name)
}

fn validate_gift_sound(binding: &GiftSoundBindingRecord) -> Result<(), MediaRepositoryError> {
    if binding.gift_id.trim().is_empty()
        || binding.gift_name.trim().is_empty()
        || !binding.volume.is_finite()
        || !(0.0..=1.0).contains(&binding.volume)
    {
        return Err(invalid("gift sound"));
    }
    Ok(())
}

fn validate_recording(recording: &RecordingRecord) -> Result<(), MediaRepositoryError> {
    if recording.room_id.trim().is_empty()
        || recording.file_path.as_os_str().is_empty()
        || recording.started_at <= 0
        || recording
            .ended_at
            .is_some_and(|ended| ended < recording.started_at)
    {
        return Err(invalid("recording"));
    }
    Ok(())
}

fn validate_highlight(highlight: &HighlightRecord) -> Result<(), MediaRepositoryError> {
    if highlight.recording_id <= 0
        || highlight.room_id.trim().is_empty()
        || highlight.created_at <= 0
        || highlight.clip_duration_seconds == 0
        || (highlight.is_clip_available && highlight.file_path.is_none())
    {
        return Err(invalid("highlight"));
    }
    Ok(())
}

fn map_gift_sound(
    row: sqlx::sqlite::SqliteRow,
) -> Result<GiftSoundBindingRecord, MediaRepositoryError> {
    let path: Option<String> = row.try_get("sound_path")?;
    let binding = GiftSoundBindingRecord {
        gift_id: row.try_get("gift_id")?,
        gift_name: row.try_get("gift_name")?,
        sound_path: path.map(PathBuf::from),
        volume: row.try_get("volume")?,
        is_enabled: row.try_get("enable")?,
    };
    validate_gift_sound(&binding)?;
    Ok(binding)
}

fn map_recording(row: sqlx::sqlite::SqliteRow) -> Result<RecordingRecord, MediaRepositoryError> {
    let format: String = row.try_get("format")?;
    let quality: String = row.try_get("quality")?;
    let status: String = row.try_get("status")?;
    let recording = RecordingRecord {
        id: row.try_get("id")?,
        room_id: row.try_get("room_id")?,
        file_path: PathBuf::from(row.try_get::<String, _>("file_path")?),
        format: parse_recording_format(&format)?,
        quality: parse_recording_quality(&quality)?,
        started_at: row.try_get("started_at")?,
        ended_at: row.try_get("ended_at")?,
        size_bytes: to_u64(row.try_get("size")?, "recording size")?,
        duration_seconds: to_u64(row.try_get("duration_seconds")?, "recording duration")?,
        status: parse_recording_state(&status)?,
    };
    validate_recording(&recording)?;
    Ok(recording)
}

fn map_highlight(row: sqlx::sqlite::SqliteRow) -> Result<HighlightRecord, MediaRepositoryError> {
    let trigger: String = row.try_get("trigger_type")?;
    let path: Option<String> = row.try_get("file_path")?;
    let highlight = HighlightRecord {
        id: row.try_get("id")?,
        recording_id: row.try_get("recording_id")?,
        room_id: row.try_get("room_id")?,
        position_seconds: to_u64(row.try_get("position_seconds")?, "highlight position")?,
        trigger_type: parse_highlight_trigger(&trigger)?,
        clip_duration_seconds: to_u64(row.try_get("clip_duration_seconds")?, "highlight duration")?,
        file_path: path.map(PathBuf::from),
        created_at: row.try_get("created_at")?,
        is_clip_available: row.try_get("is_clip_available")?,
    };
    validate_highlight(&highlight)?;
    Ok(highlight)
}

fn path_to_string(path: Option<&PathBuf>) -> Option<String> {
    path.map(|path| path.to_string_lossy().into_owned())
}

fn to_i64(value: u64, name: &'static str) -> Result<i64, MediaRepositoryError> {
    i64::try_from(value).map_err(|_| invalid(name))
}

fn to_u64(value: i64, name: &'static str) -> Result<u64, MediaRepositoryError> {
    u64::try_from(value).map_err(|_| invalid(name))
}

const fn tts_engine_name(value: TtsSelectedEngine) -> &'static str {
    match value {
        TtsSelectedEngine::System => "system",
        TtsSelectedEngine::Cloud => "cloud",
    }
}

fn parse_tts_engine(value: &str) -> Result<TtsSelectedEngine, MediaRepositoryError> {
    match value {
        "system" => Ok(TtsSelectedEngine::System),
        "cloud" => Ok(TtsSelectedEngine::Cloud),
        _ => Err(invalid("tts engine")),
    }
}

const fn recording_format_name(value: RecordingFormat) -> &'static str {
    match value {
        RecordingFormat::Mp4 => "mp4",
        RecordingFormat::Flv => "flv",
    }
}

fn parse_recording_format(value: &str) -> Result<RecordingFormat, MediaRepositoryError> {
    match value {
        "mp4" => Ok(RecordingFormat::Mp4),
        "flv" => Ok(RecordingFormat::Flv),
        _ => Err(invalid("recording format")),
    }
}

const fn recording_quality_name(value: RecordingQuality) -> &'static str {
    match value {
        RecordingQuality::Original => "original",
        RecordingQuality::High => "high",
        RecordingQuality::Standard => "standard",
    }
}

fn parse_recording_quality(value: &str) -> Result<RecordingQuality, MediaRepositoryError> {
    match value {
        "original" => Ok(RecordingQuality::Original),
        "high" => Ok(RecordingQuality::High),
        "standard" => Ok(RecordingQuality::Standard),
        _ => Err(invalid("recording quality")),
    }
}

const fn recording_state_name(value: RecordingItemState) -> &'static str {
    match value {
        RecordingItemState::Recording => "recording",
        RecordingItemState::Completed => "completed",
        RecordingItemState::Partial => "partial",
        RecordingItemState::Failed => "failed",
    }
}

fn parse_recording_state(value: &str) -> Result<RecordingItemState, MediaRepositoryError> {
    match value {
        "recording" => Ok(RecordingItemState::Recording),
        "completed" => Ok(RecordingItemState::Completed),
        "partial" => Ok(RecordingItemState::Partial),
        "failed" => Ok(RecordingItemState::Failed),
        _ => Err(invalid("recording status")),
    }
}

const fn highlight_trigger_name(value: HighlightTriggerType) -> &'static str {
    match value {
        HighlightTriggerType::DanmakuSurge => "danmaku_surge",
        HighlightTriggerType::Manual => "manual",
    }
}

fn parse_highlight_trigger(value: &str) -> Result<HighlightTriggerType, MediaRepositoryError> {
    match value {
        "danmaku_surge" => Ok(HighlightTriggerType::DanmakuSurge),
        "manual" => Ok(HighlightTriggerType::Manual),
        _ => Err(invalid("highlight trigger")),
    }
}
