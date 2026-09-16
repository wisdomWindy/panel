use std::{
    collections::HashMap,
    path::{Component, Path, PathBuf},
    sync::RwLock,
};

pub use crate::application::paths::PathPurpose;
use crate::application::{
    paths::PathGrantView,
    ports::{PathGrantPort, PathPortError},
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PathGrant {
    id: String,
    display_path: String,
}

impl PathGrant {
    pub fn id(&self) -> &str {
        &self.id
    }

    pub fn display_path(&self) -> &str {
        &self.display_path
    }
}

#[derive(Clone, Debug)]
struct GrantedPath {
    root: PathBuf,
    purpose: PathPurpose,
}

#[derive(Debug, thiserror::Error)]
pub enum PathGrantError {
    #[error("selected path is unavailable")]
    Unavailable,
    #[error("selected path has the wrong kind")]
    WrongKind,
    #[error("path grant is unknown or revoked")]
    UnknownGrant,
    #[error("path grant purpose does not match")]
    WrongPurpose,
    #[error("path escapes its granted scope")]
    ScopeEscape,
    #[error("path grant store is unavailable")]
    StoreUnavailable,
}

#[derive(Default)]
pub struct PathGrantStore {
    grants: RwLock<HashMap<String, GrantedPath>>,
}

impl PathGrantStore {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn grant_selection(
        &self,
        selection: Option<PathBuf>,
        purpose: PathPurpose,
    ) -> Result<Option<PathGrant>, PathGrantError> {
        let Some(selection) = selection else {
            return Ok(None);
        };
        let root = if purpose.is_save_file() {
            canonicalize_save_target(&selection)?
        } else {
            let metadata = selection
                .metadata()
                .map_err(|_| PathGrantError::Unavailable)?;
            let is_expected_kind = if purpose.expects_directory() {
                metadata.is_dir()
            } else {
                metadata.is_file()
            };
            if !is_expected_kind {
                return Err(PathGrantError::WrongKind);
            }
            selection
                .canonicalize()
                .map_err(|_| PathGrantError::Unavailable)?
        };
        let id = uuid::Uuid::new_v4().to_string();
        self.grants
            .write()
            .map_err(|_| PathGrantError::StoreUnavailable)?
            .insert(id.clone(), GrantedPath { root, purpose });

        Ok(Some(PathGrant {
            id,
            display_path: selection.to_string_lossy().into_owned(),
        }))
    }

    pub fn resolve_root(
        &self,
        grant_id: &str,
        purpose: PathPurpose,
    ) -> Result<PathBuf, PathGrantError> {
        let granted = self.grant(grant_id, purpose)?;
        let current = if purpose.is_save_file() {
            canonicalize_save_target(&granted.root)?
        } else {
            granted
                .root
                .canonicalize()
                .map_err(|_| PathGrantError::Unavailable)?
        };
        if current != granted.root {
            return Err(PathGrantError::ScopeEscape);
        }
        Ok(current)
    }

    pub fn resolve_child(
        &self,
        grant_id: &str,
        purpose: PathPurpose,
        relative_path: &Path,
    ) -> Result<PathBuf, PathGrantError> {
        if !purpose.expects_directory()
            || relative_path.is_absolute()
            || relative_path.components().any(|component| {
                matches!(
                    component,
                    Component::ParentDir | Component::RootDir | Component::Prefix(_)
                )
            })
        {
            return Err(PathGrantError::ScopeEscape);
        }

        let root = self.resolve_root(grant_id, purpose)?;
        let candidate = root
            .join(relative_path)
            .canonicalize()
            .map_err(|_| PathGrantError::Unavailable)?;
        if !candidate.starts_with(&root) {
            return Err(PathGrantError::ScopeEscape);
        }
        Ok(candidate)
    }

    pub fn revoke(&self, grant_id: &str) -> Result<bool, PathGrantError> {
        self.grants
            .write()
            .map(|mut grants| grants.remove(grant_id).is_some())
            .map_err(|_| PathGrantError::StoreUnavailable)
    }

    fn grant(&self, grant_id: &str, purpose: PathPurpose) -> Result<GrantedPath, PathGrantError> {
        let grant = self
            .grants
            .read()
            .map_err(|_| PathGrantError::StoreUnavailable)?
            .get(grant_id)
            .cloned()
            .ok_or(PathGrantError::UnknownGrant)?;
        if grant.purpose != purpose {
            return Err(PathGrantError::WrongPurpose);
        }
        Ok(grant)
    }
}

fn canonicalize_save_target(path: &Path) -> Result<PathBuf, PathGrantError> {
    let file_name = path.file_name().ok_or(PathGrantError::WrongKind)?;
    let parent = path.parent().ok_or(PathGrantError::Unavailable)?;
    let parent = parent
        .canonicalize()
        .map_err(|_| PathGrantError::Unavailable)?;
    if !parent.is_dir() {
        return Err(PathGrantError::WrongKind);
    }
    Ok(parent.join(file_name))
}

impl PathGrantPort for PathGrantStore {
    fn grant_selection(
        &self,
        selection: Option<PathBuf>,
        purpose: PathPurpose,
    ) -> Result<Option<PathGrantView>, PathPortError> {
        PathGrantStore::grant_selection(self, selection, purpose)
            .map(|grant| {
                grant.map(|grant| PathGrantView {
                    grant_id: grant.id,
                    display_path: grant.display_path,
                })
            })
            .map_err(|_| PathPortError)
    }

    fn revoke(&self, grant_id: &str) -> Result<bool, PathPortError> {
        PathGrantStore::revoke(self, grant_id).map_err(|_| PathPortError)
    }

    fn resolve(&self, grant_id: &str, purpose: PathPurpose) -> Result<PathBuf, PathPortError> {
        self.resolve_root(grant_id, purpose)
            .map_err(|_| PathPortError)
    }
}
