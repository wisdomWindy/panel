use std::{path::PathBuf, sync::Arc};

use serde::{Deserialize, Serialize};

use crate::application::ports::{PathGrantPort, PathPortError};

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum PathPurpose {
    RecordingDirectory,
    ExportDirectory,
    GiftSoundFile,
    GiftSoundImport,
    GiftSoundExport,
    HighlightExport,
    LocalFfmpegExecutable,
    SensitiveWordImport,
    SensitiveWordExport,
    BlacklistImport,
    BlacklistExport,
    StatExport,
    RankExport,
    ConfigImport,
    ConfigExport,
}

impl PathPurpose {
    pub const fn expects_directory(self) -> bool {
        matches!(self, Self::RecordingDirectory | Self::ExportDirectory)
    }

    pub const fn is_save_file(self) -> bool {
        matches!(
            self,
            Self::SensitiveWordExport
                | Self::BlacklistExport
                | Self::StatExport
                | Self::RankExport
                | Self::GiftSoundExport
                | Self::HighlightExport
                | Self::ConfigExport
        )
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PathGrantView {
    pub grant_id: String,
    pub display_path: String,
}

pub struct PathState {
    grants: Arc<dyn PathGrantPort>,
}

impl PathState {
    pub fn new(grants: impl PathGrantPort + 'static) -> Self {
        Self {
            grants: Arc::new(grants),
        }
    }

    pub fn from_arc(grants: Arc<dyn PathGrantPort>) -> Self {
        Self { grants }
    }

    pub fn select(
        &self,
        selection: Option<PathBuf>,
        purpose: PathPurpose,
    ) -> Result<Option<PathGrantView>, PathPortError> {
        self.grants.grant_selection(selection, purpose)
    }

    pub fn revoke(&self, grant_id: &str) -> Result<bool, PathPortError> {
        self.grants.revoke(grant_id)
    }

    pub fn resolve(&self, grant_id: &str, purpose: PathPurpose) -> Result<PathBuf, PathPortError> {
        self.grants.resolve(grant_id, purpose)
    }
}
