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
    /// Extra directories searched when the operator already has FFmpeg on the
    /// machine; overridable so the probe order is testable.
    system_roots: Vec<PathBuf>,
    /// Tests disable PATH probing so the result only depends on injected roots.
    probe_path: bool,
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
        Self {
            bundled_roots,
            system_roots: default_system_roots(),
            probe_path: true,
        }
    }

    pub fn with_roots(bundled_roots: Vec<PathBuf>) -> Self {
        Self {
            bundled_roots,
            system_roots: default_system_roots(),
            probe_path: false,
        }
    }

    pub fn with_system_roots(mut self, system_roots: Vec<PathBuf>) -> Self {
        self.system_roots = system_roots;
        self
    }

    /// Every directory that may contain a system-wide FFmpeg, in probe order:
    /// `PATH` first (that is what a shell would use), then the usual installers.
    fn system_candidates(&self) -> Vec<PathBuf> {
        let mut roots = Vec::new();
        if self.probe_path {
            if let Some(path) = std::env::var_os("PATH") {
                roots.extend(std::env::split_paths(&path));
            }
        }
        roots.extend(self.system_roots.iter().cloned());
        roots
    }

    fn locate_system(&self) -> Option<PathBuf> {
        self.system_candidates()
            .into_iter()
            .map(|root| root.join(binary_name()))
            .find(|candidate| candidate.is_file())
    }
}

impl FfmpegLocator for AppFfmpegLocator {
    fn locate(
        &self,
        source: FfmpegSource,
        granted_executable: Option<&Path>,
    ) -> Result<PathBuf, MediaIoError> {
        match source {
            // PRD 9.12: prefer the shipped binary, then fall back to whatever
            // the operator already installed, so a missing sidecar is not fatal.
            FfmpegSource::Bundled => self
                .bundled_roots
                .iter()
                .map(|root| root.join(binary_name()))
                .find(|candidate| candidate.is_file())
                .or_else(|| self.locate_system())
                .ok_or_else(|| {
                    MediaIoError::coded(
                        "RECORD_FFMPEG_MISSING",
                        "未找到可用的 FFmpeg：内置二进制未提供，本机也未安装。请在录制设置中选择或安装 FFmpeg",
                        false,
                    )
                }),
            FfmpegSource::System => self.locate_system().ok_or_else(|| {
                MediaIoError::coded(
                    "RECORD_FFMPEG_MISSING",
                    "未检测到本机安装的 FFmpeg，请安装后重试或手动选择文件",
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

/// Locations the common Windows installers use; harmless elsewhere because the
/// directories simply do not exist.
fn default_system_roots() -> Vec<PathBuf> {
    let mut roots = Vec::new();
    if let Some(local) = std::env::var_os("LOCALAPPDATA") {
        let local = PathBuf::from(local);
        roots.push(local.join("Microsoft/WinGet/Links"));
        roots.push(local.join("ffmpeg/bin"));
    }
    if let Some(program_data) = std::env::var_os("ProgramData") {
        roots.push(PathBuf::from(program_data).join("chocolatey/bin"));
    }
    if let Some(profile) = std::env::var_os("USERPROFILE") {
        roots.push(PathBuf::from(profile).join("scoop/shims"));
    }
    if let Some(program_files) = std::env::var_os("ProgramFiles") {
        roots.push(PathBuf::from(program_files).join("ffmpeg/bin"));
    }
    roots.push(PathBuf::from("C:/ffmpeg/bin"));
    roots.push(PathBuf::from("/usr/bin"));
    roots.push(PathBuf::from("/usr/local/bin"));
    roots.push(PathBuf::from("/opt/homebrew/bin"));
    roots
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
        let locator = AppFfmpegLocator::with_roots(vec![directory.path().to_path_buf()])
            .with_system_roots(Vec::new());
        let error = locator.locate(FfmpegSource::Bundled, None).unwrap_err();

        assert_eq!(error.code, "RECORD_FFMPEG_MISSING");
        assert!(!error.retryable);
        assert!(error.message.contains("FFmpeg"));
    }

    #[test]
    fn falls_back_to_a_system_install_when_the_sidecar_is_missing() {
        let bundled = tempfile::tempdir().unwrap();
        let system = tempfile::tempdir().unwrap();
        let expected = fake_binary(system.path());
        let locator = AppFfmpegLocator::with_roots(vec![bundled.path().to_path_buf()])
            .with_system_roots(vec![system.path().to_path_buf()]);

        // The operator already has FFmpeg; a missing sidecar must not block them.
        assert_eq!(
            locator.locate(FfmpegSource::Bundled, None).unwrap(),
            expected
        );
    }

    #[test]
    fn the_system_source_only_uses_detected_installs() {
        let system = tempfile::tempdir().unwrap();
        let expected = fake_binary(system.path());
        let locator = AppFfmpegLocator::with_roots(Vec::new())
            .with_system_roots(vec![system.path().to_path_buf()]);
        assert_eq!(locator.locate(FfmpegSource::System, None).unwrap(), expected);

        let empty = AppFfmpegLocator::with_roots(Vec::new()).with_system_roots(Vec::new());
        assert_eq!(
            empty.locate(FfmpegSource::System, None).unwrap_err().code,
            "RECORD_FFMPEG_MISSING"
        );
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
