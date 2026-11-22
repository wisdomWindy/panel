//! On-demand FFmpeg installation into the application's own data directory.
//!
//! PRD 9.12: when the machine has no FFmpeg, the operator can install one with a
//! single click. The binary lands under the app's data directory, so nothing is
//! added to `PATH` and the user's environment stays untouched.

use std::{
    path::{Path, PathBuf},
    time::Duration,
};

use crate::application::media::MediaIoError;

/// LGPL static build, the same family the release pipeline references.
const WINDOWS_ARCHIVE: &str =
    "https://github.com/BtbN/FFmpeg-Builds/releases/download/latest/ffmpeg-master-latest-win64-lgpl.zip";
const DOWNLOAD_TIMEOUT_SECONDS: u64 = 600;

pub struct FfmpegInstaller {
    directory: PathBuf,
}

impl FfmpegInstaller {
    pub fn new(directory: PathBuf) -> Self {
        Self { directory }
    }

    /// Downloads and extracts FFmpeg, returning the installed executable.
    pub async fn install(&self) -> Result<PathBuf, MediaIoError> {
        if !cfg!(windows) {
            return Err(MediaIoError::coded(
                "RECORD_FFMPEG_UNSUPPORTED",
                "当前平台请通过系统包管理器安装 FFmpeg，例如 Homebrew 或 apt",
                false,
            ));
        }
        tokio::fs::create_dir_all(&self.directory)
            .await
            .map_err(|_| unavailable())?;

        let archive = self.directory.join("ffmpeg-download.zip");
        download(WINDOWS_ARCHIVE, &archive).await?;

        let staging = self.directory.join("staging");
        let _ = tokio::fs::remove_dir_all(&staging).await;
        tokio::fs::create_dir_all(&staging)
            .await
            .map_err(|_| unavailable())?;
        extract(&archive, &staging)?;

        let extracted = find_binary(&staging).ok_or_else(|| {
            MediaIoError::coded(
                "RECORD_FFMPEG_INSTALL_FAILED",
                "安装包已下载，但其中没有找到 ffmpeg 可执行文件",
                true,
            )
        })?;
        let target = self.directory.join(executable_name());
        if target.exists() {
            let _ = tokio::fs::remove_file(&target).await;
        }
        tokio::fs::copy(&extracted, &target)
            .await
            .map_err(|_| unavailable())?;

        let _ = tokio::fs::remove_dir_all(&staging).await;
        let _ = tokio::fs::remove_file(&archive).await;
        Ok(target)
    }
}

async fn download(url: &str, destination: &Path) -> Result<(), MediaIoError> {
    let client = reqwest::Client::builder()
        .timeout(Duration::from_secs(DOWNLOAD_TIMEOUT_SECONDS))
        .build()
        .map_err(|error| {
            log::warn!("ffmpeg http client failed: {error}");
            unavailable()
        })?;
    log::info!("downloading FFmpeg from {url}");
    let response = client
        .get(url)
        .send()
        .await
        .map_err(|error| {
            log::warn!("ffmpeg download request failed: {error}");
            MediaIoError::coded(
                "RECORD_FFMPEG_DOWNLOAD_FAILED",
                "无法下载 FFmpeg 安装包，请检查网络后重试",
                true,
            )
        })?
        .error_for_status()
        .map_err(|error| {
            log::warn!("ffmpeg download status failed: {error}");
            MediaIoError::coded(
                "RECORD_FFMPEG_DOWNLOAD_FAILED",
                "FFmpeg 下载地址返回错误，请稍后重试",
                true,
            )
        })?;
    let bytes = response.bytes().await.map_err(|error| {
        log::warn!("ffmpeg archive read failed: {error}");
        MediaIoError::coded(
            "RECORD_FFMPEG_DOWNLOAD_FAILED",
            "FFmpeg 安装包下载中断，请重试",
            true,
        )
    })?;
    log::info!("downloaded {} bytes of FFmpeg archive", bytes.len());
    tokio::fs::write(destination, &bytes)
        .await
        .map_err(|error| {
            log::warn!("ffmpeg archive write failed: {error}");
            unavailable()
        })
}

/// `bsdtar` ships with Windows 10+ and macOS and handles zip archives, so no
/// extra crate is needed; `unzip` is the fallback on Unix.
fn extract(archive: &Path, destination: &Path) -> Result<(), MediaIoError> {
    for (program, arguments) in [
        ("tar", vec!["-xf"]),
        ("bsdtar", vec!["-xf"]),
        ("unzip", vec!["-o", "-q"]),
    ] {
        let status = crate::infrastructure::process::quiet(program)
            .args(&arguments)
            .arg(archive)
            .args(["-C", &destination.display().to_string()])
            .status();
        if let Ok(status) = &status {
            if !status.success() {
                log::warn!("{program} failed to extract the FFmpeg archive with {status}");
            }
        }
        if matches!(status, Ok(status) if status.success()) {
            return Ok(());
        }
    }
    Err(MediaIoError::coded(
        "RECORD_FFMPEG_INSTALL_FAILED",
        "无法解压 FFmpeg 安装包，请确认系统自带 tar 或 unzip 可用",
        true,
    ))
}

fn find_binary(root: &Path) -> Option<PathBuf> {
    let wanted = executable_name();
    let mut stack = vec![root.to_path_buf()];
    while let Some(directory) = stack.pop() {
        let Ok(entries) = std::fs::read_dir(&directory) else {
            continue;
        };
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                stack.push(path);
            } else if path
                .file_name()
                .is_some_and(|name| name.eq_ignore_ascii_case(wanted))
            {
                return Some(path);
            }
        }
    }
    None
}

const fn executable_name() -> &'static str {
    if cfg!(windows) {
        "ffmpeg.exe"
    } else {
        "ffmpeg"
    }
}

fn unavailable() -> MediaIoError {
    MediaIoError::coded(
        "RECORD_FFMPEG_INSTALL_FAILED",
        "无法写入 FFmpeg 安装目录，请检查磁盘空间与权限",
        true,
    )
}

#[cfg(test)]
mod tests {
    use super::{executable_name, find_binary};

    #[test]
    fn finds_the_binary_nested_inside_the_archive() {
        let directory = tempfile::tempdir().unwrap();
        let bin = directory.path().join("ffmpeg-master").join("bin");
        std::fs::create_dir_all(&bin).unwrap();
        let binary = bin.join(executable_name());
        std::fs::write(&binary, b"stub").unwrap();

        assert_eq!(find_binary(directory.path()).unwrap(), binary);
    }

    #[test]
    fn reports_nothing_when_the_archive_has_no_binary() {
        let directory = tempfile::tempdir().unwrap();
        std::fs::write(directory.path().join("readme.txt"), b"x").unwrap();
        assert!(find_binary(directory.path()).is_none());
    }
}

/// Network test: `FFMPEG_INSTALL_E2E=1 cargo test --lib ffmpeg_install -- --ignored`.
#[tokio::test]
#[ignore = "downloads ~180 MB from the internet"]
async fn installs_ffmpeg_into_the_given_directory() {
    if std::env::var("FFMPEG_INSTALL_E2E").is_err() {
        return;
    }
    let directory = tempfile::tempdir().unwrap();
    let installer = super::FfmpegInstaller::new(directory.path().to_path_buf());
    let installed = installer.install().await.expect("install should succeed");
    let metadata = std::fs::metadata(&installed).unwrap();
    assert!(metadata.len() > 10 * 1024 * 1024, "binary looks truncated");
}