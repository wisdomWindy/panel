use std::{collections::VecDeque, sync::Arc, time::Instant};

use tokio::sync::{Mutex, RwLock};

use crate::{
    application::media::{
        HotkeyPort, HotkeyRegistration, MediaIoError, RecorderLaunch, RecorderProcess,
        RecorderProcessPort,
    },
    domain::{
        live_data::LiveEvent,
        media::{
            highlight_window, is_danmaku_surge, HighlightWindow, RecordingConfig, RecordingState,
        },
    },
};

const START_ATTEMPTS: usize = 3;
const HEAT_WINDOW_MS: i64 = 10_000;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RecordingRuntimeSnapshot {
    pub state: RecordingState,
    pub room_id: Option<String>,
    pub elapsed_seconds: u64,
    pub disk_remaining_bytes: Option<u64>,
    pub highlights: Vec<HighlightWindow>,
    pub failure_code: Option<&'static str>,
}

struct RecordingRuntimeState {
    state: RecordingState,
    room_id: Option<String>,
    started_at: Option<Instant>,
    output_path: Option<std::path::PathBuf>,
    process: Option<Box<dyn RecorderProcess>>,
    disk_remaining_bytes: Option<u64>,
    danmaku_timestamps: VecDeque<i64>,
    highlights: Vec<HighlightWindow>,
    last_automatic_highlight_at: Option<i64>,
    failure_code: Option<&'static str>,
}

pub struct RecordingRuntime {
    config: RwLock<RecordingConfig>,
    process_port: Arc<dyn RecorderProcessPort>,
    operation: Mutex<()>,
    state: Mutex<RecordingRuntimeState>,
    hotkey: Mutex<Option<Box<dyn HotkeyRegistration>>>,
}

impl RecordingRuntime {
    pub fn new(config: RecordingConfig, process_port: Arc<dyn RecorderProcessPort>) -> Arc<Self> {        Arc::new(Self {
            config: RwLock::new(config),
            process_port,
            operation: Mutex::new(()),
            state: Mutex::new(RecordingRuntimeState {
                state: RecordingState::Idle,
                room_id: None,
                started_at: None,
                output_path: None,
                process: None,
                disk_remaining_bytes: None,
                danmaku_timestamps: VecDeque::new(),
                highlights: Vec::new(),
                last_automatic_highlight_at: None,
                failure_code: None,
            }),
            hotkey: Mutex::new(None),
        })
    }

    /// Runs a one-shot tool invocation (clip extraction, PRD 9.12).
    pub async fn run_tool(
        &self,
        executable: &std::path::Path,
        arguments: &[String],
    ) -> Result<(), MediaIoError> {
        self.process_port.run(executable, arguments).await
    }

    /// True when the recorder process is gone. Used by the reconnect watchdog.
    pub async fn has_exited(&self) -> bool {
        let mut state = self.state.lock().await;
        match state.process.as_mut() {
            Some(process) => process.has_exited(),
            None => true,
        }
    }

    /// Swaps in a freshly spawned recorder after a stream drop, keeping the
    /// session in `Recording` so highlights and duration keep accumulating.
    pub async fn replace_process(&self, launch: RecorderLaunch) -> Result<(), MediaIoError> {
        let process = self.process_port.spawn(launch).await?;
        let mut state = self.state.lock().await;
        if state.state != RecordingState::Recording {
            return Err(MediaIoError::conflict("recording is not active"));
        }
        state.process = Some(process);
        Ok(())
    }

    /// Marks the session as failed with a stable code (PRD 9.12: give up after
    /// the retry budget and tell the user).
    pub async fn fail(&self, code: &'static str) {
        let mut state = self.state.lock().await;
        state.state = RecordingState::Error;
        state.failure_code = Some(code);
        state.process = None;
    }

    pub async fn start(
        &self,
        room_id: impl Into<String>,
        launch: RecorderLaunch,
    ) -> Result<(), MediaIoError> {
        let _operation = self.operation.lock().await;
        let room_id = room_id.into();
        if room_id.trim().is_empty() {
            return Err(MediaIoError::invalid("recording room is required"));
        }
        {
            let mut state = self.state.lock().await;
            match state.state {
                RecordingState::Idle | RecordingState::Error => {}
                _ => return Err(MediaIoError::conflict("recording is already active")),
            }
            let config = self.config.read().await;
            if state
                .disk_remaining_bytes
                .is_some_and(|remaining| remaining < config.waterline_bytes)
            {
                state.state = RecordingState::Error;
                state.failure_code = Some("recording_disk_waterline");
                return Err(MediaIoError::operation(
                    "recording disk waterline has been reached",
                    false,
                ));
            }
            state.state = RecordingState::Starting;
            state.failure_code = None;
        }

        let mut last_error = None;
        for _ in 0..START_ATTEMPTS {
            match self.process_port.spawn(launch.clone()).await {
                Ok(process) => {
                    let mut state = self.state.lock().await;
                    state.state = RecordingState::Recording;
                    state.room_id = Some(room_id);
                    state.started_at = Some(Instant::now());
                    state.output_path = launch_output_path(&launch);
                    state.process = Some(process);
                    state.danmaku_timestamps.clear();
                    state.highlights.clear();
                    state.last_automatic_highlight_at = None;
                    state.failure_code = None;
                    return Ok(());
                }
                Err(error) => last_error = Some(error),
            }
        }

        let mut state = self.state.lock().await;
        state.state = RecordingState::Error;
        state.failure_code = Some("RECORD_START_FAILED");
        Err(last_error
            .unwrap_or_else(|| MediaIoError::operation("recording failed to start", true)))
    }

    pub async fn stop(&self) -> Result<(), MediaIoError> {
        let _operation = self.operation.lock().await;
        let mut process = {
            let mut state = self.state.lock().await;
            match state.state {
                RecordingState::Idle => return Ok(()),
                RecordingState::Starting => {
                    state.state = RecordingState::Error;
                    state.failure_code = Some("RECORD_START_INTERRUPTED");
                    return Err(MediaIoError::conflict("recording is still starting"));
                }
                RecordingState::Recording
                | RecordingState::Stopping
                | RecordingState::Finalizing
                | RecordingState::Error => {}
            }
            state.state = RecordingState::Stopping;
            state.process.take()
        };

        let result = if let Some(process) = process.as_mut() {
            match process.stop().await {
                Err(error) => Err(error),
                Ok(()) => {
                    self.state.lock().await.state = RecordingState::Finalizing;
                    match process.wait().await {
                        Ok(exit) if exit.success => Ok(()),
                        Ok(exit) => Err(MediaIoError::operation(
                            format!("recorder exited with code {:?}: {}", exit.code, exit.stderr),
                            true,
                        )),
                        Err(error) => Err(error),
                    }
                }
            }
        } else {
            Ok(())
        };

        let mut state = self.state.lock().await;
        state.process = None;
        state.started_at = None;
        state.room_id = None;
        state.danmaku_timestamps.clear();
        match &result {
            Ok(()) => {
                state.state = RecordingState::Idle;
                state.failure_code = None;
            }
            Err(_) => {
                state.state = RecordingState::Error;
                state.failure_code = Some("RECORD_FINALIZE_FAILED");
            }
        }
        result
    }

    pub async fn consume_event(&self, event: &LiveEvent) -> Option<HighlightWindow> {
        let LiveEvent::Danmu { base, .. } = event else {
            return None;
        };
        let config = self.config.read().await.clone();
        let mut state = self.state.lock().await;
        if state.state != RecordingState::Recording {
            return None;
        }
        let cutoff = base.occurred_at.saturating_sub(HEAT_WINDOW_MS);
        while state
            .danmaku_timestamps
            .front()
            .is_some_and(|timestamp| *timestamp < cutoff)
        {
            state.danmaku_timestamps.pop_front();
        }
        state.danmaku_timestamps.push_back(base.occurred_at);
        let cooled_down = state
            .last_automatic_highlight_at
            .is_none_or(|last| base.occurred_at.saturating_sub(last) >= HEAT_WINDOW_MS);
        let timestamps = state.danmaku_timestamps.make_contiguous();
        if !cooled_down || !is_danmaku_surge(timestamps, base.occurred_at, config.danmaku_threshold)
        {
            return None;
        }
        let position = elapsed_seconds(&state);
        let marker = highlight_window(position, None);
        state.highlights.push(marker);
        state.last_automatic_highlight_at = Some(base.occurred_at);
        Some(marker)
    }

    pub async fn mark_highlight(&self) -> Result<HighlightWindow, MediaIoError> {        let mut state = self.state.lock().await;
        if state.state != RecordingState::Recording {
            return Err(MediaIoError::conflict("recording is not active"));
        }
        let marker = highlight_window(elapsed_seconds(&state), None);
        state.highlights.push(marker);
        Ok(marker)
    }

    pub async fn update_disk_remaining(&self, remaining_bytes: u64) -> Result<bool, MediaIoError> {
        let waterline = self.config.read().await.waterline_bytes;
        let should_stop = {
            let mut state = self.state.lock().await;
            state.disk_remaining_bytes = Some(remaining_bytes);
            state.state == RecordingState::Recording && remaining_bytes < waterline
        };
        if should_stop {
            self.stop().await?;
        }
        Ok(should_stop)
    }

    pub async fn set_config(&self, config: RecordingConfig) {
        *self.config.write().await = config;
    }

    pub async fn config(&self) -> RecordingConfig {
        self.config.read().await.clone()
    }

    pub async fn register_hotkey(
        self: &Arc<Self>,
        hotkey_port: &dyn HotkeyPort,
    ) -> Result<(), MediaIoError> {
        let shortcut = self.config.read().await.shortcut.clone();
        let runtime = Arc::downgrade(self);
        let handle = tokio::runtime::Handle::current();
        let registration = hotkey_port.register(
            &shortcut,
            Arc::new(move || {
                if let Some(runtime) = runtime.upgrade() {
                    handle.spawn(async move {
                        let _ = runtime.mark_highlight().await;
                    });
                }
            }),
        )?;
        let mut current = self.hotkey.lock().await;
        if let Some(mut previous) = current.take() {
            let _ = previous.unregister();
        }
        current.replace(registration);
        Ok(())
    }

    /// Size of the file the recorder is writing right now.
    pub async fn output_size_bytes(&self) -> u64 {
        let path = self.state.lock().await.output_path.clone();
        path.and_then(|path| std::fs::metadata(path).ok())
            .map_or(0, |metadata| metadata.len())
    }

    /// Free space on the volume that receives the recording.
    pub async fn disk_free_bytes(&self) -> Option<u64> {
        let output_directory = {
            let state = self.state.lock().await;
            state
                .output_path
                .as_ref()
                .and_then(|path| path.parent().map(std::path::Path::to_path_buf))
        };
        if let Some(directory) = output_directory {
            return free_bytes_for(&directory);
        }
        let display = self.config.read().await.directory.clone();
        display
            .map(|grant| std::path::PathBuf::from(grant.display_path))
            .filter(|path| path.is_absolute())
            .and_then(|path| free_bytes_for(&path))
    }
    pub async fn snapshot(&self) -> RecordingRuntimeSnapshot {
        let state = self.state.lock().await;
        RecordingRuntimeSnapshot {
            state: state.state,
            room_id: state.room_id.clone(),
            elapsed_seconds: elapsed_seconds(&state),
            disk_remaining_bytes: state.disk_remaining_bytes,
            highlights: state.highlights.clone(),
            failure_code: state.failure_code,
        }
    }

    pub async fn shutdown(&self) {
        if let Some(mut registration) = self.hotkey.lock().await.take() {
            let _ = registration.unregister();
        }
        let _ = self.stop().await;
    }
}

fn elapsed_seconds(state: &RecordingRuntimeState) -> u64 {
    state
        .started_at
        .map_or(0, |started_at| started_at.elapsed().as_secs())
}

/// The recorder writes to the last positional argument (ffmpeg output path).
fn launch_output_path(launch: &RecorderLaunch) -> Option<std::path::PathBuf> {
    launch
        .arguments
        .iter()
        .rev()
        .find(|argument| !argument.starts_with('-') && !argument.starts_with('/') && !argument.contains('='))
        .map(std::path::PathBuf::from)
}

/// Free bytes on the volume that contains `directory`.
pub fn free_bytes_for(directory: &std::path::Path) -> Option<u64> {
    let disks = sysinfo::Disks::new_with_refreshed_list();
    disks
        .list()
        .iter()
        .filter(|disk| directory.starts_with(disk.mount_point()))
        .max_by_key(|disk| disk.mount_point().as_os_str().len())
        .map(sysinfo::Disk::available_space)
}