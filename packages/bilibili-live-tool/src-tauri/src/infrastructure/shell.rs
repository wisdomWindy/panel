use std::{
    path::Path,
    process::Command,
    sync::Mutex,
    time::{SystemTime, UNIX_EPOCH},
};

use async_trait::async_trait;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use sqlx::{FromRow, Sqlite, SqlitePool, Transaction};
use sysinfo::{Pid, ProcessRefreshKind, RefreshKind, System};

use crate::{
    application::shell::{
        ConfigArchiveFilePort, DirectoryOpenerPort, ShellError, ShellRepository, SystemLocalePort,
        SystemMetrics, SystemMetricsPort,
    },
    domain::shell::{
        ConfigArchiveSection, ConfigArchiveV1, LocalePreferences, ShellConfigScope, UiPreferences,
    },
};

const LOCALE_KEY: &str = "shell.locale";
const UI_KEY: &str = "shell.ui";
const LAST_BACKUP_KEY: &str = "shell.last_backup_at";
const RECORDING_KEY: &str = "media.recording";

#[derive(Clone)]
pub struct SqliteShellRepository {
    pool: SqlitePool,
}

#[derive(Clone, Debug, Deserialize, FromRow, Serialize)]
#[serde(rename_all = "camelCase")]
struct ConfigEntry {
    key: String,
    value: String,
    scope: String,
}

#[derive(Clone, Debug, Deserialize, FromRow, Serialize)]
#[serde(rename_all = "camelCase")]
struct RoomEntry {
    room_id: String,
    room_name: Option<String>,
    enable_danmu_notify: bool,
    enable_gift_notify: bool,
    enable_captain_notify: bool,
    enable_tts: bool,
    tts_volume: f64,
    record_auto_save: bool,
    record_format: String,
    record_quality: String,
}

#[derive(Clone, Debug, Deserialize, FromRow, Serialize)]
#[serde(rename_all = "camelCase")]
struct SensitiveWordEntry {
    word: String,
    word_type: String,
    match_type: String,
    enabled: bool,
}

#[derive(Clone, Debug, Deserialize, FromRow, Serialize)]
#[serde(rename_all = "camelCase")]
struct BlacklistEntry {
    user_uid: String,
    username: Option<String>,
    reason: Option<String>,
}

#[derive(Clone, Debug, Deserialize, FromRow, Serialize)]
#[serde(rename_all = "camelCase")]
struct GiftSoundEntry {
    gift_id: String,
    gift_name: String,
    sound_path: Option<String>,
    volume: f64,
    enable: bool,
}

#[derive(Clone, Debug, Deserialize, FromRow, Serialize)]
#[serde(rename_all = "camelCase")]
struct PluginEntry {
    name: String,
    app_version: Option<String>,
    enabled: bool,
    permissions_json: Option<String>,
}

enum PreparedSection {
    Room(Vec<RoomEntry>),
    SensitiveWords(Vec<SensitiveWordEntry>),
    Blacklist(Vec<BlacklistEntry>),
    GiftSounds(Vec<GiftSoundEntry>),
    InterfaceLayout(Vec<ConfigEntry>),
    Recording(Vec<ConfigEntry>),
    Plugins(Vec<PluginEntry>),
}

impl SqliteShellRepository {
    pub const fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }

    async fn load_json<T: for<'de> Deserialize<'de>>(
        &self,
        key: &str,
    ) -> Result<Option<T>, ShellError> {
        let value: Option<String> = sqlx::query_scalar(
            "SELECT value FROM t_app_setting WHERE key = ? AND scope = 'global'",
        )
        .bind(key)
        .fetch_optional(&self.pool)
        .await
        .map_err(query_error)?;
        value
            .map(|value| serde_json::from_str(&value).map_err(invalid_data))
            .transpose()
    }

    async fn save_json<T: Serialize>(&self, key: &str, value: &T) -> Result<String, ShellError> {
        let value = serde_json::to_string(value).map_err(invalid_data)?;
        sqlx::query(
            "INSERT INTO t_app_setting (key, value, scope) VALUES (?, ?, 'global') \
             ON CONFLICT(key) DO UPDATE SET value = excluded.value, scope = 'global', update_time = CURRENT_TIMESTAMP",
        )
        .bind(key)
        .bind(value)
        .execute(&self.pool)
        .await
        .map_err(query_error)?;
        Ok(uuid::Uuid::new_v4().to_string())
    }

    async fn export_section(
        &self,
        scope: ShellConfigScope,
    ) -> Result<ConfigArchiveSection, ShellError> {
        #[cfg(feature = "member")]
        if matches!(scope, ShellConfigScope::Ai | ShellConfigScope::Moderation) {
            return Err(unavailable_scope_error());
        }
        let entries = match scope {
            ShellConfigScope::Room => serde_json::to_value(sqlx::query_as::<_, RoomEntry>(
                "SELECT room_id, room_name, enable_danmu_notify, enable_gift_notify, enable_captain_notify, enable_tts, tts_volume, record_auto_save, record_format, record_quality FROM t_room_config ORDER BY room_id",
            ).fetch_all(&self.pool).await.map_err(query_error)?),
            ShellConfigScope::SensitiveWords => serde_json::to_value(sqlx::query_as::<_, SensitiveWordEntry>(
                "SELECT word, word_type, match_type, enabled FROM t_sensitive_word ORDER BY id",
            ).fetch_all(&self.pool).await.map_err(query_error)?),
            ShellConfigScope::Blacklist => serde_json::to_value(sqlx::query_as::<_, BlacklistEntry>(
                "SELECT user_uid, username, reason FROM t_black_user ORDER BY id",
            ).fetch_all(&self.pool).await.map_err(query_error)?),
            ShellConfigScope::GiftSounds => serde_json::to_value(sqlx::query_as::<_, GiftSoundEntry>(
                "SELECT gift_id, gift_name, sound_path, volume, enable FROM t_gift_sound ORDER BY id",
            ).fetch_all(&self.pool).await.map_err(query_error)?),
            ShellConfigScope::InterfaceLayout => serde_json::to_value(sqlx::query_as::<_, ConfigEntry>(
                "SELECT key, value, scope FROM t_app_setting WHERE key IN (?, ?) ORDER BY key",
            ).bind(LOCALE_KEY).bind(UI_KEY).fetch_all(&self.pool).await.map_err(query_error)?),
            ShellConfigScope::Recording => serde_json::to_value(sqlx::query_as::<_, ConfigEntry>(
                "SELECT key, value, scope FROM t_app_setting WHERE key = ? ORDER BY key",
            ).bind(RECORDING_KEY).fetch_all(&self.pool).await.map_err(query_error)?),
            ShellConfigScope::Plugins => serde_json::to_value(sqlx::query_as::<_, PluginEntry>(
                "SELECT name, app_version, enabled, permissions_json FROM t_plugin ORDER BY name",
            ).fetch_all(&self.pool).await.map_err(query_error)?),
            #[cfg(feature = "member")]
            ShellConfigScope::Ai | ShellConfigScope::Moderation => unreachable!(),
        }.map_err(invalid_data)?;
        Ok(ConfigArchiveSection { scope, entries })
    }

    fn prepare_section(section: &ConfigArchiveSection) -> Result<PreparedSection, ShellError> {
        match section.scope {
            ShellConfigScope::Room => decode_entries(&section.entries).map(PreparedSection::Room),
            ShellConfigScope::SensitiveWords => {
                decode_entries(&section.entries).map(PreparedSection::SensitiveWords)
            }
            ShellConfigScope::Blacklist => {
                decode_entries(&section.entries).map(PreparedSection::Blacklist)
            }
            ShellConfigScope::GiftSounds => {
                decode_entries(&section.entries).map(PreparedSection::GiftSounds)
            }
            ShellConfigScope::InterfaceLayout => {
                let entries: Vec<ConfigEntry> = decode_entries(&section.entries)?;
                if entries
                    .iter()
                    .any(|entry| !matches!(entry.key.as_str(), LOCALE_KEY | UI_KEY))
                {
                    return Err(ShellError::new(
                        "invalid_config_file",
                        "interface archive contains an unknown key",
                        false,
                    ));
                }
                Ok(PreparedSection::InterfaceLayout(entries))
            }
            ShellConfigScope::Recording => {
                let entries: Vec<ConfigEntry> = decode_entries(&section.entries)?;
                if entries.iter().any(|entry| entry.key != RECORDING_KEY) {
                    return Err(ShellError::new(
                        "invalid_config_file",
                        "recording archive contains an unknown key",
                        false,
                    ));
                }
                Ok(PreparedSection::Recording(entries))
            }
            ShellConfigScope::Plugins => {
                decode_entries(&section.entries).map(PreparedSection::Plugins)
            }
            #[cfg(feature = "member")]
            ShellConfigScope::Ai | ShellConfigScope::Moderation => Err(unavailable_scope_error()),
        }
    }

    async fn apply_section(
        transaction: &mut Transaction<'_, Sqlite>,
        section: PreparedSection,
    ) -> Result<(), ShellError> {
        match section {
            PreparedSection::Room(entries) => {
                sqlx::query("DELETE FROM t_room_config")
                    .execute(&mut **transaction)
                    .await
                    .map_err(query_error)?;
                for entry in entries {
                    sqlx::query("INSERT INTO t_room_config (room_id, room_name, enable_danmu_notify, enable_gift_notify, enable_captain_notify, enable_tts, tts_volume, record_auto_save, record_format, record_quality) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)")
                        .bind(entry.room_id).bind(entry.room_name).bind(entry.enable_danmu_notify).bind(entry.enable_gift_notify)
                        .bind(entry.enable_captain_notify).bind(entry.enable_tts).bind(entry.tts_volume).bind(entry.record_auto_save)
                        .bind(entry.record_format).bind(entry.record_quality).execute(&mut **transaction).await.map_err(query_error)?;
                }
            }
            PreparedSection::SensitiveWords(entries) => {
                sqlx::query("DELETE FROM t_sensitive_word")
                    .execute(&mut **transaction)
                    .await
                    .map_err(query_error)?;
                for entry in entries {
                    sqlx::query("INSERT INTO t_sensitive_word (word, word_type, match_type, enabled) VALUES (?, ?, ?, ?)")
                        .bind(entry.word).bind(entry.word_type).bind(entry.match_type).bind(entry.enabled).execute(&mut **transaction).await.map_err(query_error)?;
                }
            }
            PreparedSection::Blacklist(entries) => {
                sqlx::query("DELETE FROM t_black_user")
                    .execute(&mut **transaction)
                    .await
                    .map_err(query_error)?;
                for entry in entries {
                    sqlx::query(
                        "INSERT INTO t_black_user (user_uid, username, reason) VALUES (?, ?, ?)",
                    )
                    .bind(entry.user_uid)
                    .bind(entry.username)
                    .bind(entry.reason)
                    .execute(&mut **transaction)
                    .await
                    .map_err(query_error)?;
                }
            }
            PreparedSection::GiftSounds(entries) => {
                sqlx::query("DELETE FROM t_gift_sound")
                    .execute(&mut **transaction)
                    .await
                    .map_err(query_error)?;
                for entry in entries {
                    sqlx::query("INSERT INTO t_gift_sound (gift_id, gift_name, sound_path, volume, enable) VALUES (?, ?, ?, ?, ?)")
                        .bind(entry.gift_id).bind(entry.gift_name).bind(entry.sound_path).bind(entry.volume).bind(entry.enable).execute(&mut **transaction).await.map_err(query_error)?;
                }
            }
            PreparedSection::InterfaceLayout(entries) => {
                replace_config_entries(transaction, &[LOCALE_KEY, UI_KEY], entries).await?
            }
            PreparedSection::Recording(entries) => {
                replace_config_entries(transaction, &[RECORDING_KEY], entries).await?
            }
            PreparedSection::Plugins(entries) => {
                sqlx::query("DELETE FROM t_plugin")
                    .execute(&mut **transaction)
                    .await
                    .map_err(query_error)?;
                for entry in entries {
                    sqlx::query("INSERT INTO t_plugin (name, app_version, enabled, permissions_json) VALUES (?, ?, ?, ?)")
                        .bind(entry.name).bind(entry.app_version).bind(entry.enabled).bind(entry.permissions_json).execute(&mut **transaction).await.map_err(query_error)?;
                }
            }
        }
        Ok(())
    }
}

#[cfg(feature = "member")]
fn unavailable_scope_error() -> ShellError {
    ShellError::new(
        "invalid_config_scope",
        "configuration scope is not available",
        false,
    )
}

fn decode_entries<T: DeserializeOwned>(value: &serde_json::Value) -> Result<Vec<T>, ShellError> {
    serde_json::from_value(value.clone()).map_err(invalid_data)
}

#[async_trait]
impl ShellRepository for SqliteShellRepository {
    async fn load_locale(&self) -> Result<Option<LocalePreferences>, ShellError> {
        self.load_json(LOCALE_KEY).await
    }
    async fn save_locale(&self, value: &LocalePreferences) -> Result<String, ShellError> {
        self.save_json(LOCALE_KEY, value).await
    }
    async fn load_ui(&self) -> Result<Option<UiPreferences>, ShellError> {
        self.load_json(UI_KEY).await
    }
    async fn save_ui(&self, value: &UiPreferences) -> Result<String, ShellError> {
        self.save_json(UI_KEY, value).await
    }
    async fn last_backup_at(&self) -> Result<Option<i64>, ShellError> {
        self.load_json(LAST_BACKUP_KEY).await
    }
    async fn save_last_backup_at(&self, value: i64) -> Result<(), ShellError> {
        self.save_json(LAST_BACKUP_KEY, &value).await.map(|_| ())
    }
    async fn export_archive(
        &self,
        scopes: &[ShellConfigScope],
    ) -> Result<ConfigArchiveV1, ShellError> {
        let mut sections = Vec::with_capacity(scopes.len());
        for scope in scopes {
            sections.push(self.export_section(*scope).await?);
        }
        Ok(ConfigArchiveV1 {
            version: 1,
            sections,
        })
    }
    async fn import_archive(&self, archive: &ConfigArchiveV1) -> Result<String, ShellError> {
        let prepared = archive
            .sections
            .iter()
            .map(Self::prepare_section)
            .collect::<Result<Vec<_>, _>>()?;
        let mut transaction = self.pool.begin().await.map_err(query_error)?;
        for section in prepared {
            Self::apply_section(&mut transaction, section).await?;
        }
        transaction.commit().await.map_err(query_error)?;
        Ok(uuid::Uuid::new_v4().to_string())
    }
    async fn reset(&self) -> Result<String, ShellError> {
        let mut transaction = self.pool.begin().await.map_err(query_error)?;
        for table in [
            "t_room_config",
            "t_sensitive_word",
            "t_black_user",
            "t_gift_sound",
            "t_plugin",
        ] {
            sqlx::query(&format!("DELETE FROM {table}"))
                .execute(&mut *transaction)
                .await
                .map_err(query_error)?;
        }
        sqlx::query("DELETE FROM t_app_setting WHERE key IN (?, ?, ?)")
            .bind(LOCALE_KEY)
            .bind(UI_KEY)
            .bind(RECORDING_KEY)
            .execute(&mut *transaction)
            .await
            .map_err(query_error)?;
        transaction.commit().await.map_err(query_error)?;
        Ok(uuid::Uuid::new_v4().to_string())
    }
}

async fn replace_config_entries(
    transaction: &mut Transaction<'_, Sqlite>,
    keys: &[&str],
    entries: Vec<ConfigEntry>,
) -> Result<(), ShellError> {
    for key in keys {
        sqlx::query("DELETE FROM t_app_setting WHERE key = ?")
            .bind(key)
            .execute(&mut **transaction)
            .await
            .map_err(query_error)?;
    }
    for entry in entries {
        sqlx::query("INSERT INTO t_app_setting (key, value, scope) VALUES (?, ?, ?)")
            .bind(entry.key)
            .bind(entry.value)
            .bind(entry.scope)
            .execute(&mut **transaction)
            .await
            .map_err(query_error)?;
    }
    Ok(())
}

pub struct NativeConfigArchiveFiles;

#[async_trait]
impl ConfigArchiveFilePort for NativeConfigArchiveFiles {
    async fn read(&self, path: &Path) -> Result<Vec<u8>, ShellError> {
        tokio::fs::read(path)
            .await
            .map_err(|error| file_error("config_import_failed", error))
    }
    async fn write(&self, path: &Path, bytes: &[u8]) -> Result<(), ShellError> {
        tokio::fs::write(path, bytes)
            .await
            .map_err(|error| file_error("config_export_failed", error))
    }
}

pub struct NativeDirectoryOpener;

impl DirectoryOpenerPort for NativeDirectoryOpener {
    fn open(&self, path: &Path) -> Result<(), ShellError> {
        #[cfg(target_os = "windows")]
        let result = Command::new("explorer").arg(path).spawn();
        #[cfg(target_os = "macos")]
        let result = Command::new("open").arg(path).spawn();
        #[cfg(all(unix, not(target_os = "macos")))]
        let result = Command::new("xdg-open").arg(path).spawn();
        result.map(|_| ()).map_err(|error| {
            ShellError::new("config_directory_unavailable", error.to_string(), true)
        })
    }
}

pub struct SysinfoMetrics {
    system: Mutex<System>,
    pid: Pid,
}

pub struct NativeSystemLocale;

impl SystemLocalePort for NativeSystemLocale {
    fn current(&self) -> Option<String> {
        sys_locale::get_locale()
    }
}

impl SysinfoMetrics {
    pub fn current_process() -> Result<Self, ShellError> {
        let pid = sysinfo::get_current_pid().map_err(|error| {
            ShellError::new("system_metric_unavailable", error.to_string(), true)
        })?;
        let system = System::new_with_specifics(
            RefreshKind::new().with_processes(ProcessRefreshKind::new().with_memory().with_cpu()),
        );
        Ok(Self {
            system: Mutex::new(system),
            pid,
        })
    }
}

#[async_trait]
impl SystemMetricsPort for SysinfoMetrics {
    async fn sample(&self) -> Result<SystemMetrics, ShellError> {
        let mut system = self.system.lock().map_err(|_| {
            ShellError::new(
                "system_metric_unavailable",
                "metrics lock is poisoned",
                true,
            )
        })?;
        system.refresh_process_specifics(
            self.pid,
            ProcessRefreshKind::new().with_memory().with_cpu(),
        );
        let process = system.process(self.pid).ok_or_else(|| {
            ShellError::new(
                "system_metric_unavailable",
                "current process is unavailable",
                true,
            )
        })?;
        Ok(SystemMetrics {
            memory_bytes: Some(process.memory()),
            cpu_percent: Some(f64::from(process.cpu_usage()).clamp(0.0, 100.0)),
            observed_at: now_millis(),
        })
    }
}

fn query_error(error: sqlx::Error) -> ShellError {
    ShellError::new("config_transaction_failed", error.to_string(), true)
}

fn invalid_data(error: impl std::fmt::Display) -> ShellError {
    ShellError::new("invalid_config_file", error.to_string(), false)
}

fn file_error(code: &'static str, error: std::io::Error) -> ShellError {
    ShellError::new(
        code,
        error.to_string(),
        error.kind() != std::io::ErrorKind::PermissionDenied,
    )
}

fn now_millis() -> i64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis()
        .try_into()
        .unwrap_or(i64::MAX)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        application::shell::ShellRepository,
        domain::shell::{LocaleCode, ThemeMode},
        infrastructure::database::open_database,
    };

    #[tokio::test]
    async fn persists_shell_preferences_and_imports_only_after_full_validation() {
        let directory = tempfile::tempdir().unwrap();
        let pool = open_database(&directory.path().join("app.db"))
            .await
            .unwrap();
        let repository = SqliteShellRepository::new(pool);
        let locale = LocalePreferences {
            locale: LocaleCode::EnUs,
            follow_system: false,
        };
        let ui = UiPreferences {
            theme: ThemeMode::Light,
            ..UiPreferences::default()
        };
        repository.save_locale(&locale).await.unwrap();
        repository.save_ui(&ui).await.unwrap();
        assert_eq!(repository.load_locale().await.unwrap(), Some(locale));
        assert_eq!(repository.load_ui().await.unwrap(), Some(ui));

        let archive = repository
            .export_archive(&[ShellConfigScope::InterfaceLayout])
            .await
            .unwrap();
        repository.reset().await.unwrap();
        assert!(repository.load_ui().await.unwrap().is_none());
        repository.import_archive(&archive).await.unwrap();
        assert_eq!(
            repository.load_ui().await.unwrap().unwrap().theme,
            ThemeMode::Light
        );

        let damaged = ConfigArchiveV1 {
            version: 1,
            sections: vec![ConfigArchiveSection {
                scope: ShellConfigScope::InterfaceLayout,
                entries: serde_json::json!([{ "key": "unknown", "value": "x", "scope": "global" }]),
            }],
        };
        assert!(repository.import_archive(&damaged).await.is_err());
        assert_eq!(
            repository.load_ui().await.unwrap().unwrap().theme,
            ThemeMode::Light
        );
    }

    #[tokio::test]
    async fn round_trips_sensitive_words_and_blacklist_in_one_archive() {
        let directory = tempfile::tempdir().unwrap();
        let pool = open_database(&directory.path().join("app.db"))
            .await
            .unwrap();
        sqlx::query("INSERT INTO t_sensitive_word (word, word_type, match_type, enabled) VALUES ('blocked', 'text', 'exact', 1)").execute(&pool).await.unwrap();
        sqlx::query("INSERT INTO t_black_user (user_uid, username, reason) VALUES ('100', 'user', 'manual')").execute(&pool).await.unwrap();
        let repository = SqliteShellRepository::new(pool.clone());
        let archive = repository
            .export_archive(&[
                ShellConfigScope::SensitiveWords,
                ShellConfigScope::Blacklist,
            ])
            .await
            .unwrap();
        sqlx::query("DELETE FROM t_sensitive_word")
            .execute(&pool)
            .await
            .unwrap();
        sqlx::query("DELETE FROM t_black_user")
            .execute(&pool)
            .await
            .unwrap();
        repository.import_archive(&archive).await.unwrap();
        assert_eq!(
            sqlx::query_scalar::<_, i64>("SELECT COUNT(*) FROM t_sensitive_word")
                .fetch_one(&pool)
                .await
                .unwrap(),
            1
        );
        assert_eq!(
            sqlx::query_scalar::<_, i64>("SELECT COUNT(*) FROM t_black_user")
                .fetch_one(&pool)
                .await
                .unwrap(),
            1
        );
    }
}
