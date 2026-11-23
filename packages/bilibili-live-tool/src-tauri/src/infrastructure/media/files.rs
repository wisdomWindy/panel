use std::{
    collections::HashSet,
    future::Future,
    path::{Path, PathBuf},
    sync::Arc,
};

use serde::{Deserialize, Serialize};
use tokio::io::AsyncWriteExt;

use crate::application::{
    media::StagedMediaFile,
    paths::PathPurpose,
    ports::{PathGrantPort, PathPortError},
};

use super::GiftSoundBindingRecord;

const MAX_IMPORT_BYTES: u64 = 1024 * 1024;

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct PortableGiftSoundBinding {
    pub gift_id: String,
    pub gift_name: String,
    pub file_name: Option<String>,
    pub volume: f64,
    pub is_enabled: bool,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct GiftSoundDocument {
    pub version: u32,
    pub bindings: Vec<PortableGiftSoundBinding>,
}

impl GiftSoundDocument {
    pub const CURRENT_VERSION: u32 = 1;

    pub fn from_records(records: &[GiftSoundBindingRecord]) -> Result<Self, MediaFileError> {
        let document = Self {
            version: Self::CURRENT_VERSION,
            bindings: records
                .iter()
                .map(|record| PortableGiftSoundBinding {
                    gift_id: record.gift_id.clone(),
                    gift_name: record.gift_name.clone(),
                    file_name: record
                        .sound_path
                        .as_deref()
                        .and_then(Path::file_name)
                        .map(|name| name.to_string_lossy().into_owned()),
                    volume: record.volume,
                    is_enabled: record.is_enabled,
                })
                .collect(),
        };
        document.validate()?;
        Ok(document)
    }

    pub fn into_records(self) -> Result<Vec<GiftSoundBindingRecord>, MediaFileError> {
        self.validate()?;
        Ok(self
            .bindings
            .into_iter()
            .map(|binding| GiftSoundBindingRecord {
                gift_id: binding.gift_id,
                gift_name: binding.gift_name,
                sound_path: None,
                volume: binding.volume,
                is_enabled: binding.is_enabled,
            })
            .collect())
    }

    fn validate(&self) -> Result<(), MediaFileError> {
        if self.version != Self::CURRENT_VERSION || self.bindings.len() > 10_000 {
            return Err(MediaFileError::InvalidDocument);
        }
        let mut gift_ids = HashSet::with_capacity(self.bindings.len());
        for binding in &self.bindings {
            if binding.gift_id.trim().is_empty()
                || binding.gift_name.trim().is_empty()
                || !binding.volume.is_finite()
                || !(0.0..=1.0).contains(&binding.volume)
                || !gift_ids.insert(binding.gift_id.as_str())
                || binding.file_name.as_deref().is_some_and(|name| {
                    Path::new(name).file_name().and_then(|value| value.to_str()) != Some(name)
                        || !has_supported_sound_extension(Path::new(name))
                })
            {
                return Err(MediaFileError::InvalidDocument);
            }
        }
        Ok(())
    }
}

#[derive(Debug, thiserror::Error)]
pub enum MediaFileError {
    #[error("media file authorization failed")]
    Authorization,
    #[error("media import document is invalid")]
    InvalidDocument,
    #[error("media file operation failed: {0}")]
    Io(#[from] std::io::Error),
}

impl From<PathPortError> for MediaFileError {
    fn from(_: PathPortError) -> Self {
        Self::Authorization
    }
}

pub struct GiftSoundFileStore {
    grants: Arc<dyn PathGrantPort>,
}

impl GiftSoundFileStore {
    pub fn new(grants: Arc<dyn PathGrantPort>) -> Self {
        Self { grants }
    }

    pub fn resolve_sound(&self, grant_id: &str) -> Result<PathBuf, MediaFileError> {
        let path = self.grants.resolve(grant_id, PathPurpose::GiftSoundFile)?;
        if !has_supported_sound_extension(&path) {
            return Err(MediaFileError::InvalidDocument);
        }
        Ok(path)
    }

    pub async fn read_import(&self, grant_id: &str) -> Result<GiftSoundDocument, MediaFileError> {
        let path = self
            .grants
            .resolve(grant_id, PathPurpose::GiftSoundImport)?;
        if tokio::fs::metadata(&path).await?.len() > MAX_IMPORT_BYTES {
            return Err(MediaFileError::InvalidDocument);
        }
        let bytes = tokio::fs::read(path).await?;
        let document: GiftSoundDocument =
            serde_json::from_slice(&bytes).map_err(|_| MediaFileError::InvalidDocument)?;
        document.validate()?;
        Ok(document)
    }

    pub async fn write_export(
        &self,
        grant_id: &str,
        document: &GiftSoundDocument,
    ) -> Result<(), MediaFileError> {
        document.validate()?;
        let target = self
            .grants
            .resolve(grant_id, PathPurpose::GiftSoundExport)?;
        let bytes =
            serde_json::to_vec_pretty(document).map_err(|_| MediaFileError::InvalidDocument)?;
        atomic_write(&target, &bytes).await
    }
}

#[async_trait::async_trait]
impl crate::application::media::MediaFilePort for GiftSoundFileStore {
    fn resolve_gift_sound(
        &self,
        grant_id: &str,
    ) -> Result<PathBuf, crate::application::media::MediaIoError> {
        self.resolve_sound(grant_id)
            .map_err(|_| crate::application::media::MediaIoError::invalid("invalid sound grant"))
    }

    async fn import_gift_sounds(
        &self,
        grant_id: &str,
    ) -> Result<Vec<GiftSoundBindingRecord>, crate::application::media::MediaIoError> {
        self.read_import(grant_id)
            .await
            .and_then(GiftSoundDocument::into_records)
            .map_err(|_| {
                crate::application::media::MediaIoError::invalid("invalid gift sound import")
            })
    }

    async fn export_gift_sounds(
        &self,
        grant_id: &str,
        bindings: &[GiftSoundBindingRecord],
    ) -> Result<(), crate::application::media::MediaIoError> {
        let document = GiftSoundDocument::from_records(bindings).map_err(|_| {
            crate::application::media::MediaIoError::invalid("invalid gift sound export")
        })?;
        self.write_export(grant_id, &document).await.map_err(|_| {
            crate::application::media::MediaIoError::operation("gift sound export failed", true)
        })
    }

    async fn move_file(
        &self,
        source: &Path,
        target: &Path,
    ) -> Result<(), crate::application::media::MediaIoError> {
        if target.exists() {
            return Err(crate::application::media::MediaIoError::conflict(
                "media target already exists",
            ));
        }
        tokio::fs::rename(source, target).await.map_err(|_| {
            crate::application::media::MediaIoError::operation("media file move failed", true)
        })
    }

    async fn stage_delete(
        &self,
        paths: &[PathBuf],
    ) -> Result<Vec<StagedMediaFile>, crate::application::media::MediaIoError> {
        let mut staged = Vec::with_capacity(paths.len());
        for path in paths {
            if !path.is_file() {
                // The file is already gone (removed from the folder, or a clip
                // that never got written); deleting the row must still work.
                continue;
            }
            let target = sibling_path(path, "delete").map_err(|_| {
                crate::application::media::MediaIoError::operation(
                    "media file staging failed",
                    true,
                )
            })?;
            if tokio::fs::rename(path, &target).await.is_err() {
                self.restore_staged(&staged).await?;
                return Err(crate::application::media::MediaIoError::operation(
                    "media file staging failed",
                    true,
                ));
            }
            staged.push(StagedMediaFile {
                original: path.clone(),
                staged: target,
            });
        }
        Ok(staged)
    }

    async fn restore_staged(
        &self,
        files: &[StagedMediaFile],
    ) -> Result<(), crate::application::media::MediaIoError> {
        for file in files.iter().rev() {
            tokio::fs::rename(&file.staged, &file.original)
                .await
                .map_err(|_| {
                    crate::application::media::MediaIoError::operation(
                        "media file restoration failed",
                        false,
                    )
                })?;
        }
        Ok(())
    }

    async fn finalize_staged(
        &self,
        files: &[StagedMediaFile],
    ) -> Result<(), crate::application::media::MediaIoError> {
        for file in files {
            tokio::fs::remove_file(&file.staged).await.map_err(|_| {
                crate::application::media::MediaIoError::operation(
                    "media file deletion failed",
                    true,
                )
            })?;
        }
        Ok(())
    }

    async fn export_file(
        &self,
        source: &Path,
        target: &Path,
    ) -> Result<(), crate::application::media::MediaIoError> {
        if target.exists() {
            return Err(crate::application::media::MediaIoError::conflict(
                "media export target already exists",
            ));
        }
        tokio::fs::copy(source, target).await.map_err(|_| {
            crate::application::media::MediaIoError::operation("media file export failed", true)
        })?;
        Ok(())
    }

    async fn file_size(&self, path: &Path) -> Result<u64, crate::application::media::MediaIoError> {
        tokio::fs::metadata(path)
            .await
            .map(|metadata| metadata.len())
            .map_err(|_| {
                crate::application::media::MediaIoError::operation(
                    "media file metadata is unavailable",
                    true,
                )
            })
    }
}

#[derive(Debug, thiserror::Error)]
pub enum CompensatedDeleteError<E> {
    #[error("file staging failed: {0}")]
    File(#[source] std::io::Error),
    #[error("database action failed")]
    Action(E),
    #[error("database action failed and file restoration also failed: {restore}")]
    Restore { action: E, restore: std::io::Error },
}

#[derive(Debug, thiserror::Error)]
pub enum CompensatedMoveError<E> {
    #[error("file move failed: {0}")]
    File(#[source] std::io::Error),
    #[error("database action failed")]
    Action(E),
    #[error("database action failed and file restoration also failed: {restore}")]
    Restore { action: E, restore: std::io::Error },
}

pub async fn compensated_delete<F, Fut, T, E>(
    path: &Path,
    action: F,
) -> Result<T, CompensatedDeleteError<E>>
where
    F: FnOnce() -> Fut,
    Fut: Future<Output = Result<T, E>>,
{
    let staged = sibling_path(path, "delete").map_err(CompensatedDeleteError::File)?;
    tokio::fs::rename(path, &staged)
        .await
        .map_err(CompensatedDeleteError::File)?;
    match action().await {
        Ok(value) => {
            tokio::fs::remove_file(staged)
                .await
                .map_err(CompensatedDeleteError::File)?;
            Ok(value)
        }
        Err(action) => match tokio::fs::rename(&staged, path).await {
            Ok(()) => Err(CompensatedDeleteError::Action(action)),
            Err(restore) => Err(CompensatedDeleteError::Restore { action, restore }),
        },
    }
}

pub async fn compensated_move<F, Fut, T, E>(
    source: &Path,
    target: &Path,
    action: F,
) -> Result<T, CompensatedMoveError<E>>
where
    F: FnOnce() -> Fut,
    Fut: Future<Output = Result<T, E>>,
{
    if target.exists() {
        return Err(CompensatedMoveError::File(std::io::Error::new(
            std::io::ErrorKind::AlreadyExists,
            "move target already exists",
        )));
    }
    tokio::fs::rename(source, target)
        .await
        .map_err(CompensatedMoveError::File)?;
    match action().await {
        Ok(value) => Ok(value),
        Err(action) => match tokio::fs::rename(target, source).await {
            Ok(()) => Err(CompensatedMoveError::Action(action)),
            Err(restore) => Err(CompensatedMoveError::Restore { action, restore }),
        },
    }
}

async fn atomic_write(target: &Path, bytes: &[u8]) -> Result<(), MediaFileError> {
    let temporary = sibling_path(target, "write")?;
    let mut file = tokio::fs::OpenOptions::new()
        .create_new(true)
        .write(true)
        .open(&temporary)
        .await?;
    if let Err(error) = file.write_all(bytes).await {
        drop(file);
        let _ = tokio::fs::remove_file(&temporary).await;
        return Err(error.into());
    }
    file.flush().await?;
    drop(file);

    let backup = if target.exists() {
        let backup = sibling_path(target, "backup")?;
        tokio::fs::rename(target, &backup).await?;
        Some(backup)
    } else {
        None
    };
    if let Err(error) = tokio::fs::rename(&temporary, target).await {
        let _ = tokio::fs::remove_file(&temporary).await;
        if let Some(backup) = backup {
            let _ = tokio::fs::rename(backup, target).await;
        }
        return Err(error.into());
    }
    if let Some(backup) = backup {
        tokio::fs::remove_file(backup).await?;
    }
    Ok(())
}

fn sibling_path(path: &Path, operation: &str) -> Result<PathBuf, std::io::Error> {
    let file_name = path.file_name().ok_or_else(|| {
        std::io::Error::new(std::io::ErrorKind::InvalidInput, "path has no file name")
    })?;
    let staged_name = format!(
        ".{}.{}-{}",
        file_name.to_string_lossy(),
        operation,
        uuid::Uuid::new_v4()
    );
    Ok(path.with_file_name(staged_name))
}

fn has_supported_sound_extension(path: &Path) -> bool {
    path.extension()
        .and_then(|extension| extension.to_str())
        .is_some_and(|extension| {
            extension.eq_ignore_ascii_case("mp3") || extension.eq_ignore_ascii_case("wav")
        })
}
