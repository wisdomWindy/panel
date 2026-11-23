use std::{path::Path, process::Stdio};

use tokio::{
    io::{AsyncRead, AsyncReadExt},
    process::Child,
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
    /// Set when the runtime asked the recorder to stop; the resulting exit
    /// status is expected and must not be reported as a failure.
    stop_requested: bool,
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
        let mut command = crate::infrastructure::process::quiet_tokio(&launch.executable);
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
            stop_requested: false,
            stdout,
            stderr,
        }))
    }

    async fn run(&self, executable: &Path, arguments: &[String]) -> Result<(), MediaIoError> {
        validate_executable(executable)?;
        let output = crate::infrastructure::process::quiet_tokio(executable)
            .args(arguments)
            .stdin(Stdio::null())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .kill_on_drop(true)
            .output()
            .await
            .map_err(|_| MediaIoError::unavailable("configured FFmpeg could not be started"))?;
        if !output.status.success() {
            return Err(MediaIoError::operation(
                "FFmpeg clip extraction failed",
                false,
            ));
        }
        Ok(())
    }
}

#[async_trait::async_trait]
impl RecorderProcess for LocalFfmpegProcess {
    fn has_exited(&mut self) -> bool {
        self.child
            .as_mut()
            .and_then(|child| child.try_wait().ok())
            .flatten()
            .is_some()
    }

    async fn stop(&mut self) -> Result<(), MediaIoError> {
        // Stopping is requested by the runtime, so the non-zero exit that
        // TerminateProcess produces must not be reported as a failure later.
        self.stop_requested = true;
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
        let success = status.success() || self.stop_requested;
        Ok(RecorderExit {
            success,
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
    // FFmpeg writes GBK on a Chinese Windows host; decoding explicitly keeps
    // recorder failures readable instead of surfacing U+FFFD to the operator.
    Ok(crate::infrastructure::text::decode_external_text(&captured))
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
