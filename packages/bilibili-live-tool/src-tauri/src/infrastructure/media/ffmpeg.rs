use std::{path::Path, process::Stdio};

use tokio::{
    io::{AsyncRead, AsyncReadExt},
    process::{Child, Command},
    task::JoinHandle,
};

use crate::application::media::{
    MediaIoError, RecorderExit, RecorderLaunch, RecorderProcess, RecorderProcessPort,
};

const MAX_CAPTURE_BYTES: usize = 16 * 1024;

#[derive(Default)]
pub struct LocalFfmpegProcessPort;

pub struct LocalFfmpegProcess {
    child: Option<Child>,
    stdout: Option<JoinHandle<Result<String, std::io::Error>>>,
    stderr: Option<JoinHandle<Result<String, std::io::Error>>>,
}

#[async_trait::async_trait]
impl RecorderProcessPort for LocalFfmpegProcessPort {
    async fn spawn(
        &self,
        launch: RecorderLaunch,
    ) -> Result<Box<dyn RecorderProcess>, MediaIoError> {
        validate_executable(&launch.executable)?;
        if let Some(directory) = &launch.working_directory {
            if !directory.is_dir() {
                return Err(MediaIoError::invalid(
                    "recorder working directory is invalid",
                ));
            }
        }
        let mut command = Command::new(&launch.executable);
        command
            .args(&launch.arguments)
            .stdin(Stdio::null())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .kill_on_drop(true);
        if let Some(directory) = launch.working_directory {
            command.current_dir(directory);
        }
        let mut child = command
            .spawn()
            .map_err(|_| MediaIoError::unavailable("configured FFmpeg could not be started"))?;
        let stdout = child
            .stdout
            .take()
            .map(|stream| tokio::spawn(capture(stream)));
        let stderr = child
            .stderr
            .take()
            .map(|stream| tokio::spawn(capture(stream)));
        Ok(Box::new(LocalFfmpegProcess {
            child: Some(child),
            stdout,
            stderr,
        }))
    }
}

#[async_trait::async_trait]
impl RecorderProcess for LocalFfmpegProcess {
    async fn stop(&mut self) -> Result<(), MediaIoError> {
        let child = self
            .child
            .as_mut()
            .ok_or_else(|| MediaIoError::invalid("recorder process already completed"))?;
        if child
            .try_wait()
            .map_err(|_| MediaIoError::operation("recorder status check failed", true))?
            .is_none()
        {
            child
                .kill()
                .await
                .map_err(|_| MediaIoError::operation("recorder stop failed", true))?;
        }
        Ok(())
    }

    async fn wait(&mut self) -> Result<RecorderExit, MediaIoError> {
        let mut child = self
            .child
            .take()
            .ok_or_else(|| MediaIoError::invalid("recorder process already completed"))?;
        let status = child
            .wait()
            .await
            .map_err(|_| MediaIoError::operation("recorder wait failed", true))?;
        let stdout = join_capture(self.stdout.take()).await?;
        let stderr = join_capture(self.stderr.take()).await?;
        Ok(RecorderExit {
            success: status.success(),
            code: status.code(),
            stdout,
            stderr,
        })
    }
}

pub struct UnavailableRecorderProcessPort;

#[async_trait::async_trait]
impl RecorderProcessPort for UnavailableRecorderProcessPort {
    async fn spawn(&self, _: RecorderLaunch) -> Result<Box<dyn RecorderProcess>, MediaIoError> {
        Err(MediaIoError::unavailable("bundled FFmpeg is not installed"))
    }
}

fn validate_executable(path: &Path) -> Result<(), MediaIoError> {
    let metadata = path
        .metadata()
        .map_err(|_| MediaIoError::unavailable("configured FFmpeg is unavailable"))?;
    if !metadata.is_file() {
        return Err(MediaIoError::invalid("configured FFmpeg is not a file"));
    }
    Ok(())
}

async fn capture(mut stream: impl AsyncRead + Unpin) -> Result<String, std::io::Error> {
    let mut captured = Vec::new();
    let mut buffer = [0_u8; 4096];
    loop {
        let read = stream.read(&mut buffer).await?;
        if read == 0 {
            break;
        }
        let remaining = MAX_CAPTURE_BYTES.saturating_sub(captured.len());
        captured.extend_from_slice(&buffer[..read.min(remaining)]);
    }
    Ok(String::from_utf8_lossy(&captured).into_owned())
}

async fn join_capture(
    task: Option<JoinHandle<Result<String, std::io::Error>>>,
) -> Result<String, MediaIoError> {
    match task {
        Some(task) => task
            .await
            .map_err(|_| MediaIoError::operation("recorder output task failed", true))?
            .map_err(|_| MediaIoError::operation("recorder output read failed", true)),
        None => Ok(String::new()),
    }
}
