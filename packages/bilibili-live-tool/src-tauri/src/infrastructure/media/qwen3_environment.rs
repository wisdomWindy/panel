//! Prepares the Python environment the Qwen3-TTS service needs.
//!
//! PRD 9.11: enabling 语音播报 should end with a working speech engine, not with
//! a list of manual `pip install` commands. This module owns a virtualenv under
//! the app data directory, installs the documented requirements into it, and
//! reports progress through Tauri events so the UI can show real stages.

use std::{
    path::{Path, PathBuf},
    process::Stdio,
    time::Duration,
};

use crate::application::media::MediaIoError;

const INSTALL_TIMEOUT_SECONDS: u64 = 3600;
pub const ENVIRONMENT_PROGRESS_EVENT: &str = "tts_environment_progress";

/// Packages the service imports at runtime (see services/qwen3-tts/requirements.txt).
const REQUIRED_MODULES: &str = "torch, huggingface_hub, qwen_tts, soundfile, numpy";

pub struct Qwen3Environment {
    root: PathBuf,
    requirements: Option<PathBuf>,
    system_python: Option<PathBuf>,
    log_directory: PathBuf,
}

#[derive(Clone, Debug, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct EnvironmentStatus {
    /// True when the managed virtualenv can import every runtime dependency.
    pub ready: bool,
    /// Where the virtualenv lives; `None` when it has not been created yet.
    pub venv_python: Option<String>,
    /// Human readable hint describing what is missing.
    pub detail: String,
}

impl Qwen3Environment {
    pub fn for_current_app(root: PathBuf, log_directory: PathBuf) -> Self {
        Self {
            requirements: locate_requirements(),
            system_python: super::qwen3_service::locate_python(),
            root,
            log_directory,
        }
    }

    pub fn venv_python(&self) -> Option<PathBuf> {
        self.venv_python_path().filter(|path| path.is_file())
    }

    fn venv_python_path(&self) -> Option<PathBuf> {
        let venv = self.root.join("venv");
        let candidate = if cfg!(windows) {
            venv.join("Scripts").join("python.exe")
        } else {
            venv.join("bin").join("python")
        };
        Some(candidate)
    }

    /// Checks the managed environment without touching it.
    pub fn status(&self) -> EnvironmentStatus {
        let Some(python) = self.venv_python() else {
            return EnvironmentStatus {
                ready: false,
                venv_python: None,
                detail: "尚未创建语音运行环境".to_owned(),
            };
        };
        match probe_modules(&python) {
            Ok(()) => EnvironmentStatus {
                ready: true,
                venv_python: Some(python.display().to_string()),
                detail: "运行环境已就绪".to_owned(),
            },
            Err(detail) => EnvironmentStatus {
                ready: false,
                venv_python: Some(python.display().to_string()),
                detail,
            },
        }
    }

    /// Creates the virtualenv when needed and installs the runtime requirements.
    ///
    /// `progress` receives `(stage, message)` pairs so the caller can stream them.
    pub async fn prepare(
        &self,
        index_url: Option<&str>,
        progress: impl Fn(&str, &str) + Send + 'static,
    ) -> Result<EnvironmentStatus, MediaIoError> {
        if self.status().ready {
            progress("ready", "运行环境已就绪");
            return Ok(self.status());
        }
        let system_python = self.system_python.clone().ok_or_else(|| {
            MediaIoError::coded(
                "TTS_SERVICE_PYTHON_MISSING",
                "未检测到 Python 3，请先安装 Python 3.10 及以上版本",
                false,
            )
        })?;
        let venv_python = self.venv_python_path().ok_or_else(unavailable)?;

        if !venv_python.is_file() {
            progress("venv", "正在创建独立运行环境…");
            tokio::fs::create_dir_all(&self.root)
                .await
                .map_err(|_| unavailable())?;
            let venv_directory = self.root.join("venv");
            let log = self.log_file("venv.log");
            run(
                &system_python,
                &[
                    "-m",
                    "venv",
                    &venv_directory.display().to_string(),
                ],
                &self.root,
                log,
                600,
            )
            .await?;
        }
        if !venv_python.is_file() {
            return Err(unavailable());
        }

        progress("pip", "正在准备安装工具…");
        let log = self.log_file("pip.log");
        run(
            &venv_python,
            &["-m", "pip", "install", "--upgrade", "pip"],
            &self.root,
            log,
            INSTALL_TIMEOUT_SECONDS,
        )
        .await?;

        let requirements = self.requirements.as_deref().ok_or_else(|| {
            MediaIoError::coded(
                "TTS_SERVICE_SCRIPT_MISSING",
                "未找到依赖清单 requirements.txt，请重新安装应用",
                false,
            )
        })?;
        progress("deps", "正在安装语音运行依赖（torch 等，首次约 2–3 GB）…");
        let log = self.log_file("deps.log");
        let requirements_path = requirements.display().to_string();
        let mut arguments = vec![
            "-m",
            "pip",
            "install",
            "--disable-pip-version-check",
            "-r",
            requirements_path.as_str(),
        ];
        if let Some(index_url) = index_url {
            arguments.push("--index-url");
            arguments.push(index_url);
        }
        run(&venv_python, &arguments, &self.root, log, INSTALL_TIMEOUT_SECONDS).await?;

        progress("verify", "正在校验运行环境…");
        match probe_modules(&venv_python) {
            Ok(()) => {
                progress("ready", "运行环境已就绪");
                Ok(self.status())
            }
            Err(detail) => Err(MediaIoError::coded(
                "TTS_ENVIRONMENT_INCOMPLETE",
                &format!("依赖安装后仍无法导入：{detail}"),
                true,
            )),
        }
    }

    fn log_file(&self, name: &str) -> std::fs::File {
        let _ = std::fs::create_dir_all(&self.log_directory);
        std::fs::File::create(self.log_directory.join(name)).unwrap_or_else(|_| {
            std::fs::File::open(std::env::temp_dir()).unwrap_or_else(|_| {
                // `/dev/null` equivalent; a closed file is good enough here.
                std::fs::File::create(std::env::temp_dir().join("biliLiveHelper-null.log"))
                    .expect("temporary log file")
            })
        })
    }
}

fn unavailable() -> MediaIoError {
    MediaIoError::coded(
        "TTS_ENVIRONMENT_INCOMPLETE",
        "无法创建语音运行环境，请检查磁盘空间与权限",
        true,
    )
}

/// Runs a Python tool, appending its output to `log`, and maps failures to a
/// coded error that includes the log path so the operator can inspect it.
async fn run(
    program: &Path,
    arguments: &[&str],
    working_directory: &Path,
    log: std::fs::File,
    timeout_seconds: u64,
) -> Result<(), MediaIoError> {
    let mut command = tokio::process::Command::new(program);
    command
        .args(arguments)
        .current_dir(working_directory)
        .stdin(Stdio::null())
        .stdout(Stdio::from(log.try_clone().map_err(|_| unavailable())?))
        .stderr(Stdio::from(log));
    log::info!(
        "running {} {} in {}",
        program.display(),
        arguments.join(" "),
        working_directory.display()
    );
    let child = command.spawn().map_err(|error| {
        log::warn!("python helper spawn failed: {error}");
        MediaIoError::coded(
            "TTS_ENVIRONMENT_INCOMPLETE",
            "无法启动 Python，请确认安装完整",
            true,
        )
    })?;
    let status = tokio::time::timeout(
        Duration::from_secs(timeout_seconds),
        child.wait_with_output(),
    )
    .await
    .map_err(|_| {
        MediaIoError::coded(
            "TTS_ENVIRONMENT_INCOMPLETE",
            "依赖安装超时，请检查网络后重试",
            true,
        )
    })?
    .map_err(|_| unavailable())?;
    if status.status.success() {
        Ok(())
    } else {
        Err(MediaIoError::coded(
            "TTS_ENVIRONMENT_INCOMPLETE",
            &format!("依赖安装失败（退出码 {}），请查看日志", status.status),
            true,
        ))
    }
}

fn probe_modules(python: &Path) -> Result<(), String> {
    let output = std::process::Command::new(python)
        .args(["-c", &format!("import {REQUIRED_MODULES}")])
        .stdin(Stdio::null())
        .output()
        .map_err(|error| format!("无法执行 Python：{error}"))?;
    if output.status.success() {
        Ok(())
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        let missing = stderr
            .lines()
            .find(|line| line.contains("ModuleNotFoundError") || line.contains("ImportError"))
            .unwrap_or("缺少运行依赖")
            .trim()
            .to_owned();
        Err(missing)
    }
}

/// The requirements file ships beside `server.py`.
fn locate_requirements() -> Option<PathBuf> {
    if let Some(override_path) = std::env::var_os("QWEN3_TTS_REQUIREMENTS") {
        let path = PathBuf::from(override_path);
        if path.is_file() {
            return Some(path);
        }
    }
    let server = super::qwen3_service::locate_script()?;
    let path = server.parent()?.join("requirements.txt");
    path.is_file().then_some(path)
}

#[cfg(test)]
mod tests {
    use super::Qwen3Environment;

    #[test]
    fn reports_missing_environment_before_anything_is_installed() {
        let directory = tempfile::tempdir().unwrap();
        let environment = Qwen3Environment::for_current_app(
            directory.path().to_path_buf(),
            directory.path().to_path_buf(),
        );
        let status = environment.status();
        assert!(!status.ready);
        assert!(status.venv_python.is_none());
    }

    #[test]
    fn venv_python_path_follows_the_platform_layout() {
        let directory = tempfile::tempdir().unwrap();
        let environment = Qwen3Environment::for_current_app(
            directory.path().to_path_buf(),
            directory.path().to_path_buf(),
        );
        let path = environment.venv_python_path().unwrap();
        assert!(path.starts_with(directory.path().join("venv")));
        let expected = if cfg!(windows) { "python.exe" } else { "python" };
        assert_eq!(path.file_name().unwrap(), expected);
    }
}
