//! FFmpeg executable resolution.
//!
//! Two sources are supported: a bundled binary shipped next to the application,
//! and a local binary the operator picked (which arrives as an authorized
//! path-grant). The bundled lookup probes the usual Tauri resource layouts so a
//! packaged build works on all three release targets.

use std::path::{Path, PathBuf};

use crate::{
    application::media::{FfmpegLocator, MediaIoError},
    domain::media::FfmpegSource,
};

pub struct AppFfmpegLocator {
    bundled_roots: Vec<PathBuf>,
}

impl AppFfmpegLocator {
    /// Probes the directories a packaged Tauri build can place a sidecar in.
    pub fn for_current_app() -> Self {
        let mut bundled_roots = Vec::new();
        if let Ok(executable) = std::env::current_exe() {
            if let Some(directory) = executable.parent() {
                bundled_roots.push(directory.to_path_buf());
                bundled_roots.push(directory.join("resources"));
                bundled_roots.push(directory.join("bin"));
                if let Some(parent) = directory.parent() {
                    bundled_roots.push(parent.join("resources"));
                }
            }
        }
        Self { bundled_roots }
    }

    pub fn with_roots(bundled_roots: Vec<PathBuf>) -> Self {
        Self { bundled_roots }
    }
}

impl FfmpegLocator for AppFfmpegLocator {
    fn locate(
        &self,
        source: FfmpegSource,
        granted_executable: Option<&Path>,
    ) -> Result<PathBuf, MediaIoError> {
        match source {
            FfmpegSource::Bundled => self
                .bundled_roots
                .iter()
                .map(|root| root.join(binary_name()))
                .find(|candidate| candidate.is_file())
                .ok_or_else(|| {
                    MediaIoError::coded(
                        "RECORD_FFMPEG_MISSING",
                        "内置 FFmpeg 不可用，请在录制设置中选择本地 FFmpeg",
                        false,
                    )
                }),
            FfmpegSource::Local => {
                let executable = granted_executable.ok_or_else(|| {
                    MediaIoError::coded(
                        "RECORD_FFMPEG_MISSING",
                        "尚未授权本地 FFmpeg，请先在录制设置中选择",
                        false,
                    )
                })?;
                if !executable.is_file() {
                    return Err(MediaIoError::coded(
                        "RECORD_FFMPEG_MISSING",
                        "所选 FFmpeg 文件不存在或不可读，请重新选择",
                        false,
                    ));
                }
                Ok(executable.to_path_buf())
            }
        }
    }
}

const fn binary_name() -> &'static str {
    if cfg!(windows) {
        "ffmpeg.exe"
    } else {
        "ffmpeg"
    }
}

#[cfg(test)]
mod tests {
    use super::AppFfmpegLocator;
    use crate::application::media::FfmpegLocator;
    use crate::domain::media::FfmpegSource;
    use std::path::PathBuf;

    fn fake_binary(directory: &std::path::Path) -> PathBuf {
        let path = directory.join(super::binary_name());
        std::fs::write(&path, b"stub").unwrap();
        path
    }

    #[test]
    fn finds_a_bundled_binary_in_the_probe_order() {
        let directory = tempfile::tempdir().unwrap();
        let binary = fake_binary(directory.path());
        let locator = AppFfmpegLocator::with_roots(vec![
            directory.path().join("missing"),
            directory.path().to_path_buf(),
        ]);

        assert_eq!(
            locator.locate(FfmpegSource::Bundled, None).unwrap(),
            binary
        );
    }

    #[test]
    fn reports_a_missing_bundled_binary_with_actionable_text() {
        let directory = tempfile::tempdir().unwrap();
        let locator = AppFfmpegLocator::with_roots(vec![directory.path().to_path_buf()]);
        let error = locator.locate(FfmpegSource::Bundled, None).unwrap_err();

        assert_eq!(error.code, "RECORD_FFMPEG_MISSING");
        assert!(!error.retryable);
        assert!(error.message.contains("内置 FFmpeg"));
    }

    #[test]
    fn uses_the_authorized_local_binary() {
        let directory = tempfile::tempdir().unwrap();
        let binary = fake_binary(directory.path());
        let locator = AppFfmpegLocator::with_roots(Vec::new());

        assert_eq!(
            locator
                .locate(FfmpegSource::Local, Some(binary.as_path()))
                .unwrap(),
            binary
        );
    }

    #[test]
    fn rejects_a_local_source_without_authorization() {
        let locator = AppFfmpegLocator::with_roots(Vec::new());
        let error = locator.locate(FfmpegSource::Local, None).unwrap_err();
        assert_eq!(error.code, "RECORD_FFMPEG_MISSING");
    }

    #[test]
    fn rejects_an_authorized_path_that_no_longer_exists() {
        let locator = AppFfmpegLocator::with_roots(Vec::new());
        let error = locator
            .locate(FfmpegSource::Local, Some(std::path::Path::new("/nope/ffmpeg")))
            .unwrap_err();
        assert_eq!(error.code, "RECORD_FFMPEG_MISSING");
    }
}
