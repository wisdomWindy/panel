//! Starts the local Qwen3-TTS service on the operator's behalf.
//!
//! PRD 9.11/9.13: the speech feature must not ask the operator to open a
//! terminal. The service script only needs the Python standard library to boot
//! and report `/health`; the model stack is imported lazily, so a machine
//! without `torch` still gets a running service that explains what is missing.

use std::{
    path::{Path, PathBuf},
    process::{Command, Stdio},
    time::Duration,
};

use crate::application::media::MediaIoError;

const STARTUP_TIMEOUT_SECONDS: u64 = 40;
const POLL_INTERVAL_MILLIS: u64 = 500;

pub struct Qwen3ServiceLauncher {
    script: Option<PathBuf>,
    python: Option<PathBuf>,
    log_directory: PathBuf,
}

impl Qwen3ServiceLauncher {
    pub fn for_current_app(log_directory: PathBuf) -> Self {
        Self {
            script: locate_script(),
            python: locate_python(),
            log_directory,
        }
    }

    pub fn with_paths(script: Option<PathBuf>, python: Option<PathBuf>, log_directory: PathBuf) -> Self {
        Self {
            script,
            python,
            log_directory,
        }
    }

    pub fn script(&self) -> Option<&Path> {
        self.script.as_deref()
    }

    pub fn python(&self) -> Option<&Path> {
        self.python.as_deref()
    }

    /// Starts the service when it is not answering yet and reports the URL that
    /// actually ended up serving (the preferred port may be reserved by the OS).
    pub async fn ensure_running(
        &self,
        base_url: &str,
        preferred_port: u16,
    ) -> Result<ServiceStatus, MediaIoError> {
        if health_ok(base_url).await {
            return Ok(ServiceStatus {
                started: false,
                base_url: base_url.to_owned(),
            });
        }
        let script = self.script.as_deref().ok_or_else(|| {
            MediaIoError::coded(
                "TTS_SERVICE_SCRIPT_MISSING",
                "未找到本地语音服务脚本 server.py，请重新安装应用",
                false,
            )
        })?;
        let python = self.python.as_deref().ok_or_else(|| {
            MediaIoError::coded(
                "TTS_SERVICE_PYTHON_MISSING",
                "未检测到可用的 Python 3，请安装 Python 后重试",
                false,
            )
        })?;

        // Hyper-V and WinNAT reserve whole port ranges, so the configured port
        // is frequently unbindable; pick the first one the OS actually allows.
        let port = candidate_ports(preferred_port)
            .into_iter()
            .find(|port| port_is_free(*port))
            .ok_or_else(|| {
                MediaIoError::coded(
                    "TTS_SERVICE_START_FAILED",
                    "找不到可用于本地语音服务的端口，请检查系统保留端口设置",
                    true,
                )
            })?;
        let listening_url = format!("http://127.0.0.1:{port}");

        if let Err(error) = tokio::fs::create_dir_all(&self.log_directory).await {
            log::warn!("qwen3 service log directory is unavailable: {error}");
        }
        let stdout = std::fs::File::create(self.log_directory.join("qwen3-tts.out.log"));
        let stderr = std::fs::File::create(self.log_directory.join("qwen3-tts.err.log"));

        let mut command = Command::new(python);
        command
            .arg(script)
            .args(["--host", "127.0.0.1", "--port", &port.to_string()])
            .current_dir(script.parent().unwrap_or(Path::new(".")))
            .stdin(Stdio::null());
        match (stdout, stderr) {
            (Ok(out), Ok(err)) => {
                command.stdout(Stdio::from(out)).stderr(Stdio::from(err));
            }
            _ => {
                command.stdout(Stdio::null()).stderr(Stdio::null());
            }
        }
        log::info!(
            "starting the local Qwen3-TTS service: {} {} on port {port}",
            python.display(),
            script.display()
        );
        command.spawn().map_err(|error| {
            log::warn!("qwen3 service spawn failed: {error}");
            MediaIoError::coded(
                "TTS_SERVICE_START_FAILED",
                "无法启动本地语音服务，请确认 Python 可执行文件可用",
                true,
            )
        })?;

        let deadline = STARTUP_TIMEOUT_SECONDS * 1000 / POLL_INTERVAL_MILLIS;
        for _ in 0..deadline {
            tokio::time::sleep(Duration::from_millis(POLL_INTERVAL_MILLIS)).await;
            if health_ok(&listening_url).await {
                return Ok(ServiceStatus {
                    started: true,
                    base_url: listening_url,
                });
            }
        }
        Err(MediaIoError::coded(
            "TTS_SERVICE_START_FAILED",
            "本地语音服务启动超时，请查看日志后重试",
            true,
        ))
    }
}

/// What `ensure_running` achieved, including the URL that actually serves.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ServiceStatus {
    pub started: bool,
    pub base_url: String,
}

/// The configured port first, then the documented default, then ephemeral-ish
/// alternatives that are rarely reserved by virtualisation software.
fn candidate_ports(preferred: u16) -> Vec<u16> {
    let mut ports = vec![preferred, 8020];
    ports.extend(18020..18040);
    ports.dedup();
    ports
}

fn port_is_free(port: u16) -> bool {
    std::net::TcpListener::bind(("127.0.0.1", port)).is_ok()
}

/// Port taken from the configured base URL; the service default is 8020.
pub fn port_from_base_url(base_url: &str) -> u16 {
    base_url
        .rsplit(':')
        .next()
        .and_then(|value| value.trim_end_matches('/').parse::<u16>().ok())
        .filter(|port| *port > 0)
        .unwrap_or(8020)
}

async fn health_ok(base_url: &str) -> bool {
    let Ok(client) = reqwest::Client::builder()
        .timeout(Duration::from_millis(1500))
        .build()
    else {
        return false;
    };
    client
        .get(format!("{}/health", base_url.trim_end_matches('/')))
        .send()
        .await
        .map(|response| response.status().is_success())
        .unwrap_or(false)
}

/// `server.py` shipped beside the executable, as a bundled resource, in the app
/// data directory, or in the development checkout.
fn locate_script() -> Option<PathBuf> {
    if let Some(override_path) = std::env::var_os("QWEN3_TTS_SERVER") {
        let path = PathBuf::from(override_path);
        if path.is_file() {
            return Some(path);
        }
    }
    let relative = Path::new("services").join("qwen3-tts").join("server.py");
    let mut candidates = Vec::new();
    if let Ok(executable) = std::env::current_exe() {
        if let Some(directory) = executable.parent() {
            candidates.push(directory.join(&relative));
            candidates.push(directory.join("resources").join(&relative));
            if let Some(parent) = directory.parent() {
                candidates.push(parent.join("resources").join(&relative));
            }
        }
    }
    // Development checkout: src-tauri -> package -> packages -> repository root.
    candidates.push(
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("..")
            .join("..")
            .join("..")
            .join(&relative),
    );
    candidates.into_iter().find(|candidate| candidate.is_file())
}

fn locate_python() -> Option<PathBuf> {
    if let Some(override_path) = std::env::var_os("QWEN3_TTS_PYTHON") {
        let path = PathBuf::from(override_path);
        if path.is_file() {
            return Some(path);
        }
    }
    let names: &[&str] = if cfg!(windows) {
        &["python.exe", "python3.exe"]
    } else {
        &["python3", "python"]
    };
    if let Some(path) = std::env::var_os("PATH") {
        for directory in std::env::split_paths(&path) {
            for name in names {
                let candidate = directory.join(name);
                if candidate.is_file() {
                    return Some(candidate);
                }
            }
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::Qwen3ServiceLauncher;

    #[tokio::test]
    async fn reports_the_script_it_would_start() {
        let directory = tempfile::tempdir().unwrap();
        let script = directory.path().join("server.py");
        std::fs::write(&script, b"print('stub')").unwrap();
        let launcher = Qwen3ServiceLauncher::with_paths(
            Some(script.clone()),
            Some(directory.path().join("python.exe")),
            directory.path().to_path_buf(),
        );
        assert_eq!(launcher.script(), Some(script.as_path()));
    }

    #[tokio::test]
    async fn explains_a_missing_script_instead_of_failing_silently() {
        let directory = tempfile::tempdir().unwrap();
        let launcher = Qwen3ServiceLauncher::with_paths(
            None,
            None,
            directory.path().to_path_buf(),
        );
        // Nothing answers on this port, and there is no script to start.
        let error = launcher
            .ensure_running("http://127.0.0.1:9", 9)
            .await
            .unwrap_err();
        assert_eq!(error.code, "TTS_SERVICE_SCRIPT_MISSING");
    }
}
